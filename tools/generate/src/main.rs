use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Write};
use std::path::PathBuf;
use std::process::Command;

use proc_macro2::{Ident, Span};
use quote::quote;
use serde::de::{Deserialize, Deserializer, MapAccess, Visitor};
use serde_json::Value;

struct Records(BTreeMap<String, Value>);

impl<'de> Deserialize<'de> for Records {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct RecordVisitor;
        impl<'de> Visitor<'de> for RecordVisitor {
            type Value = Records;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("TableGen record map")
            }
            fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Records, M::Error> {
                let mut records = BTreeMap::new();
                while let Some((name, mut record)) = map.next_entry::<String, Value>()? {
                    if (record["isPseudo"] == 0
                        && record["AsmString"].as_str().is_some_and(|s| !s.is_empty()))
                        || record.get("Pfl").is_some()
                        || record.get("DstVT").is_some()
                        || record.get("RegClass").is_some()
                    {
                        if let Some(fields) = record.as_object_mut() {
                            fields.retain(|key, _| {
                                matches!(
                                    key.as_str(),
                                    "AsmString"
                                        | "InOperandList"
                                        | "OutOperandList"
                                        | "Predicates"
                                        | "Uses"
                                        | "Defs"
                                        | "PseudoInstr"
                                        | "Pfl"
                                        | "DstVT"
                                        | "Src0VT"
                                        | "Src1VT"
                                        | "Src2VT"
                                        | "RegClass"
                                )
                            });
                        }
                        records.insert(name, record);
                    }
                }
                Ok(Records(records))
            }
        }
        deserializer.deserialize_map(RecordVisitor)
    }
}

fn ident(name: &str) -> Ident {
    Ident::new(name, Span::call_site())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args_os().skip(1);
    let llvm = PathBuf::from(
        args.next()
            .ok_or("usage: generate-amdgpu-types LLVM_CHECKOUT [TABLEGEN_JSON]")?,
    );
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let supplied_json = args.next().map(PathBuf::from);
    let json = supplied_json
        .clone()
        .unwrap_or_else(|| root.join("target/amdgpu-records.json"));
    fs::create_dir_all(root.join("target"))?;
    if supplied_json.is_none() {
        let tblgen = env::var_os("LLVM_TBLGEN").unwrap_or_else(|| "llvm-tblgen".into());
        let target = llvm.join("llvm/lib/Target/AMDGPU");
        let status = Command::new(tblgen)
            .arg("-dump-json")
            .arg("-I")
            .arg(llvm.join("llvm/include"))
            .arg("-I")
            .arg(&target)
            .arg(target.join("AMDGPU.td"))
            .arg("-o")
            .arg(&json)
            .status()?;
        if !status.success() {
            return Err("llvm-tblgen failed".into());
        }
    }
    eprintln!("reading TableGen records");
    let Records(records) = serde_json::from_reader(BufReader::new(File::open(&json)?))?;
    let revision = Command::new("git")
        .arg("-C")
        .arg(&llvm)
        .args(["rev-parse", "HEAD"])
        .output()?;
    if !revision.status.success() {
        return Err("could not read LLVM revision".into());
    }
    let revision = String::from_utf8(revision.stdout)?.trim().to_owned();
    generate(&root, &records, &revision)?;
    Ok(())
}

fn rust_name(name: &str) -> String {
    name.split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            chars.next().unwrap().to_uppercase().collect::<String>() + chars.as_str()
        })
        .collect()
}

fn value_type(name: &str) -> Option<&'static str> {
    Some(match name {
        "u8" => "U8",
        "u16" => "U16",
        "u32" => "U32",
        "u64" => "U64",
        "i8" => "I8",
        "i16" => "I16",
        "i32" => "I32",
        "i64" => "I64",
        "f16" => "F16",
        "f32" => "F32",
        "f64" => "F64",
        "b16" => "B16",
        "b32" => "B32",
        "b64" => "B64",
        "b128" => "B128",
        "b256" => "B256",
        "b512" => "B512",
        "b1024" => "B1024",
        _ => return None,
    })
}

fn source_type(
    records: &BTreeMap<String, Value>,
    class: &str,
    data: &Ident,
    modified: bool,
) -> Option<proc_macro2::TokenStream> {
    let reg_class = records
        .get(class)
        .and_then(|r| r["RegClass"]["def"].as_str())
        .unwrap_or(class);
    if class.starts_with("VSrc_") {
        Some(if modified {
            quote! { crate::ModifiedSource<crate::#data> }
        } else {
            quote! { crate::SourceOperand<crate::#data> }
        })
    } else if class.starts_with("SSrc_") {
        Some(quote! { crate::ScalarSourceOperand<crate::#data> })
    } else if reg_class.starts_with("VGPR_") || reg_class.starts_with("VReg_") {
        Some(quote! { crate::VectorRegister<crate::#data> })
    } else if reg_class.starts_with("SReg_") || reg_class.starts_with("SGPR_") {
        Some(quote! { crate::ScalarRegister<crate::#data> })
    } else {
        None
    }
}

fn generate(
    root: &std::path::Path,
    records: &BTreeMap<String, Value>,
    revision: &str,
) -> Result<(), Box<dyn Error>> {
    let mut comparisons = BTreeSet::new();
    let mut predicates = BTreeSet::new();
    let mut forms = BTreeMap::<String, proc_macro2::TokenStream>::new();
    let mut families = BTreeMap::<String, (usize, bool)>::new();
    let mut inventory = BTreeSet::new();
    let mut decoders = BTreeMap::<
        String,
        (
            String,
            proc_macro2::TokenStream,
            proc_macro2::TokenStream,
            bool,
        ),
    >::new();
    let profiles: BTreeMap<_, _> = records
        .values()
        .filter_map(|record| {
            Some((
                record["PseudoInstr"].as_str()?,
                record["Pfl"]["def"].as_str()?,
            ))
        })
        .collect();
    for (name, record) in records {
        let Some(assembly) = record["AsmString"].as_str() else {
            continue;
        };
        inventory.insert(
            assembly
                .split(|c: char| c.is_whitespace() || c == '$')
                .next()
                .unwrap_or("")
                .to_owned(),
        );
        if !name.ends_with("_gfx10") {
            continue;
        }
        let mnemonic = record["PseudoInstr"].as_str().unwrap_or("");
        let (operation, encoding) = if let Some(base) = mnemonic.strip_suffix("_e32") {
            (base, ident("E32"))
        } else if let Some(base) = mnemonic.strip_suffix("_e64") {
            (base, ident("E64"))
        } else {
            continue;
        };
        let parts: Vec<_> = operation.split('_').collect();
        let mut split = parts.len();
        while split > 0 && value_type(parts[split - 1]).is_some() {
            split -= 1;
        }
        if split == parts.len() {
            continue;
        }
        let types: Vec<_> = parts[split..]
            .iter()
            .filter_map(|ty| value_type(ty))
            .collect();
        let profile_name = record["Pfl"]["def"]
            .as_str()
            .or_else(|| profiles.get(mnemonic).copied());
        let Some(profile) = profile_name.and_then(|p| records.get(p)) else {
            continue;
        };
        if parts.starts_with(&["v", "cmp"]) && parts.len() == 4 {
            let predicate = match parts[2] {
                "f" => "Never",
                "t" => "Always",
                "eq" => "Eq",
                "ne" => "Ne",
                "lt" => "Lt",
                "le" => "Le",
                "gt" => "Gt",
                "ge" => "Ge",
                "lg" => "Lg",
                "o" => "Ordered",
                "u" => "Unordered",
                "nlt" => "NotLt",
                "nle" => "NotLe",
                "ngt" => "NotGt",
                "nge" => "NotGe",
                "nlg" => "NotLg",
                _ => continue,
            };
            let ty = types[0];
            predicates.insert(predicate.to_owned());
            comparisons.insert((predicate.to_owned(), ty.to_owned(), encoding.to_string()));
            let variant = rust_name(mnemonic);
            let (p, t) = (ident(predicate), ident(ty));
            let instruction = quote! { crate::VCmp<predicate::#p, crate::#t, crate::#encoding> };
            let expression = quote! {
                <#instruction>::new(crate::decode::operand(&fields[0])?, crate::decode::operand(&fields[1])?, crate::decode::operand(&fields[2])?)
            };
            decoders.insert(
                variant,
                (
                    mnemonic.to_owned(),
                    instruction,
                    quote! { let fields = crate::decode::fields(text, 3)?; #expression },
                    encoding == "E32",
                ),
            );
            continue;
        }
        if !parts.starts_with(&["v"]) {
            continue;
        }
        let Some(outputs) = record["OutOperandList"]["args"].as_array() else {
            continue;
        };
        if outputs.len() != 1 {
            continue;
        }
        let output_class = outputs[0][0]["def"].as_str().unwrap_or("");
        let Some(inputs) = record["InOperandList"]["args"].as_array() else {
            continue;
        };
        let sources: Vec<_> = inputs
            .iter()
            .filter(|input| matches!(input[1].as_str(), Some("src0" | "src1" | "src2")))
            .collect();
        if sources.is_empty() || sources.len() > 3 {
            continue;
        }
        if inputs.iter().any(|input| {
            !matches!(
                input[1].as_str(),
                Some(
                    "src0"
                        | "src1"
                        | "src2"
                        | "src0_modifiers"
                        | "src1_modifiers"
                        | "src2_modifiers"
                        | "clamp"
                        | "omod"
                )
            )
        }) {
            continue;
        }
        let mut data_types = Vec::new();
        for field in std::iter::once("DstVT").chain(
            ["Src0VT", "Src1VT", "Src2VT"]
                .into_iter()
                .take(sources.len()),
        ) {
            let Some(raw) = profile[field]["def"].as_str() else {
                break;
            };
            let typed = if raw.starts_with('i') {
                types
                    .iter()
                    .find(|ty| ty[1..] == raw[1..] && !ty.starts_with('F'))
                    .copied()
                    .or_else(|| value_type(raw))
            } else {
                value_type(raw)
            };
            let positional = types
                .get(data_types.len())
                .copied()
                .filter(|ty| types.len() == sources.len() + 1 && ty[1..] == raw[1..]);
            let typed = positional.or(typed);
            let Some(typed) = typed else { break };
            data_types.push(ident(typed));
        }
        if data_types.len() != sources.len() + 1 {
            continue;
        }
        if !source_type(records, output_class, &data_types[0], false)
            .is_some_and(|ty| ty.to_string().contains("VectorRegister"))
        {
            continue;
        }
        let family_name = rust_name(&parts[..split].join("_"));
        let family = ident(&family_name);
        let arity = sources.len();
        let homogeneous = data_types.iter().all(|ty| *ty == data_types[0]);
        let entry = families
            .entry(family_name.clone())
            .or_insert((arity, homogeneous));
        if entry.0 != arity {
            continue;
        }
        entry.1 &= homogeneous;
        let mut operand_types = Vec::new();
        for (index, source) in sources.iter().enumerate() {
            let Some(class) = source[0]["def"].as_str() else {
                break;
            };
            let modified = inputs
                .iter()
                .any(|input| input[1].as_str() == Some(&format!("src{index}_modifiers")));
            let Some(ty) = source_type(records, class, &data_types[index + 1], modified) else {
                break;
            };
            operand_types.push(ty);
        }
        if operand_types.len() != arity {
            continue;
        }
        let (private, support) = match arity {
            1 => (ident("Unary"), ident("SupportedUnary")),
            2 => (ident("Binary"), ident("SupportedBinary")),
            _ => (ident("Ternary"), ident("SupportedTernary")),
        };
        let associated = if arity == 1 {
            let source = &operand_types[0];
            quote! { type Source = #source; }
        } else {
            let names = (0..arity).map(|i| ident(&format!("Source{i}")));
            quote! { #(type #names = #operand_types;)* }
        };
        let tokens = quote! {
            impl crate::private::#private<family::#family, #(crate::#data_types,)* crate::#encoding> for () {}
            impl crate::#support<family::#family, #(crate::#data_types,)* crate::#encoding> for () { #associated }
        };
        let key = format!("{family_name}:{data_types:?}:{encoding}");
        if let Some(old) = forms.get(&key) {
            if old.to_string() != tokens.to_string() {
                return Err(format!("conflicting operand schemas for {key}").into());
            }
        } else {
            forms.insert(key, tokens);
        }
        let container = ident(match arity {
            1 => "VectorUnary",
            2 => "VectorBinary",
            _ => "VectorTernary",
        });
        let instruction =
            quote! { crate::#container<family::#family, #(crate::#data_types,)* crate::#encoding> };
        let indexes = 0..=arity;
        let count = arity + 1;
        let expression = quote! {
            let fields = crate::decode::fields(text, #count)?;
            <#instruction>::new(#(crate::decode::operand(&fields[#indexes])?,)*)
        };
        decoders.insert(
            rust_name(mnemonic),
            (
                mnemonic.to_owned(),
                instruction,
                expression,
                encoding == "E32",
            ),
        );
    }
    if comparisons.is_empty() || forms.is_empty() {
        return Err("no supported TableGen profiles found".into());
    }
    let temporary = root.join("src/generated.tmp.rs");
    let mut output = BufWriter::new(File::create(&temporary)?);
    writeln!(
        output,
        "{}",
        quote! { pub const LLVM_REVISION: &str = #revision; }
    )?;
    writeln!(output, "pub mod predicate {{")?;
    for name in &predicates {
        let name = ident(name);
        writeln!(
            output,
            "{}",
            quote! { #[derive(Debug, Clone, Copy, PartialEq, Eq)] pub struct #name; }
        )?;
    }
    writeln!(output, "}}")?;
    for (predicate, ty, encoding) in &comparisons {
        let (predicate, ty, encoding) = (ident(predicate), ident(ty), ident(encoding));
        writeln!(
            output,
            "{}",
            quote! {
                impl crate::private::Compare<predicate::#predicate, crate::#ty, crate::#encoding> for () {}
                impl crate::SupportedCompare<predicate::#predicate, crate::#ty, crate::#encoding> for () {}
            }
        )?;
    }
    writeln!(output, "pub mod family {{")?;
    for name in families.keys() {
        let name = ident(name);
        writeln!(
            output,
            "{}",
            quote! { #[derive(Debug, Clone, Copy, PartialEq, Eq)] pub struct #name; }
        )?;
    }
    writeln!(output, "}}")?;
    for (name, (arity, homogeneous)) in &families {
        let name = ident(name);
        let container = ident(match arity {
            1 => "VectorUnary",
            2 => "VectorBinary",
            _ => "VectorTernary",
        });
        if *homogeneous {
            let types = (0..=*arity).map(|_| ident("T"));
            writeln!(
                output,
                "{}",
                quote! { pub type #name<T, E> = crate::#container<family::#name, #(#types,)* E>; }
            )?;
        } else {
            let types: Vec<_> = std::iter::once(ident("D"))
                .chain((0..*arity).map(|i| ident(&format!("S{i}"))))
                .collect();
            writeln!(
                output,
                "{}",
                quote! { pub type #name<#(#types,)* E> = crate::#container<family::#name, #(#types,)* E>; }
            )?;
        }
    }
    for tokens in forms.values() {
        writeln!(output, "{tokens}")?;
    }
    writeln!(
        output,
        "#[derive(Debug, Clone, Copy, PartialEq)] pub enum DecodedInstruction {{"
    )?;
    for (variant, (_, ty, _, _)) in &decoders {
        let variant = ident(variant);
        writeln!(output, "{}", quote! { #variant(#ty), })?;
    }
    writeln!(output, "}}")?;
    writeln!(
        output,
        "pub fn parse(text: &str) -> Result<DecodedInstruction, crate::DecodeError> {{ match text.split_whitespace().next().unwrap_or(\"\") {{"
    )?;
    for (variant, (mnemonic, _, expression, e32)) in &decoders {
        let variant = ident(variant);
        let alias = if *e32 {
            let base = mnemonic.strip_suffix("_e32").unwrap();
            quote! { | #base }
        } else {
            quote! {}
        };
        writeln!(
            output,
            "{}",
            quote! { #mnemonic #alias => Ok(DecodedInstruction::#variant({ #expression })), }
        )?;
    }
    writeln!(
        output,
        "_ => Err(crate::DecodeError {{ range: 0..text.split_whitespace().next().unwrap_or(\"\").len(), reason: \"instruction has no generated decoder\".into() }}), }} }}"
    )?;
    let inventory: Vec<_> = inventory
        .into_iter()
        .filter(|name| !name.is_empty())
        .collect();
    writeln!(
        output,
        "{}",
        quote! { pub const KNOWN_MNEMONICS: &[&str] = &[#(#inventory,)*]; }
    )?;
    output.flush()?;
    fs::rename(temporary, root.join("src/generated.rs"))?;
    eprintln!(
        "generated {} comparison forms, {} vector families, {} vector forms, {} known mnemonics",
        comparisons.len(),
        families.len(),
        forms.len(),
        inventory.len()
    );
    Ok(())
}
