use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use proc_macro2::{Ident, Span};
use quote::quote;
use roxmltree::Node;

fn ident(name: &str) -> Ident {
    Ident::new(name, Span::call_site())
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
        _ => return None,
    })
}

fn child<'a>(node: Node<'a, 'a>, name: &str) -> Option<Node<'a, 'a>> {
    node.children().find(|n| n.has_tag_name(name))
}

fn text<'a>(node: Node<'a, 'a>, name: &str) -> &'a str {
    child(node, name).and_then(|n| n.text()).unwrap_or("")
}

fn operand_type(operand: Node<'_, '_>, bit_type: Option<&str>) -> Option<Ident> {
    let raw = text(operand, "DataFormatName")
        .strip_prefix("FMT_NUM_")?
        .to_lowercase();
    let ty = value_type(&raw)?;
    let ty = bit_type
        .filter(|b| b.starts_with('B') && b[1..] == ty[1..])
        .unwrap_or(ty);
    Some(ident(ty))
}

fn source_type(operand: Node<'_, '_>, ty: &Ident, e64: bool) -> Option<proc_macro2::TokenStream> {
    Some(match text(operand, "OperandType") {
        "OPR_SRC" | "OPR_SRC_NOLDS" if e64 => quote! { crate::ModifiedSource<crate::#ty> },
        "OPR_SRC" | "OPR_SRC_NOLDS" => quote! { crate::SourceOperand<crate::#ty> },
        "OPR_VGPR" => quote! { crate::VectorRegister<crate::#ty> },
        "OPR_SREG" => quote! { crate::ScalarRegister<crate::#ty> },
        _ => return None,
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut args = env::args_os().skip(1);
    let xml = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("amd-isa/amdgpu_isa_rdna2.xml"));
    if args.next().is_some() {
        return Err("usage: generate-amdgpu-types [RDNA2_XML]".into());
    }
    let source = fs::read_to_string(xml)?;
    generate(&root, &source)
}

fn generate(root: &std::path::Path, source: &str) -> Result<(), Box<dyn Error>> {
    let document = roxmltree::Document::parse(source)?;
    let spec = document.root_element();
    let metadata = child(spec, "Document").ok_or("missing Document")?;
    let isa = child(spec, "ISA").ok_or("missing ISA")?;
    let architecture = child(isa, "Architecture").ok_or("missing Architecture")?;
    if text(architecture, "ArchitectureName") != "AMD RDNA 2" {
        return Err("only AMD RDNA 2 specifications are supported".into());
    }
    let release = text(metadata, "ReleaseDate");
    let schema = text(metadata, "SchemaVersion");
    if schema != "1.1.1" {
        return Err(format!("unsupported XML schema: {schema}").into());
    }
    let mut comparisons = BTreeSet::new();
    let mut predicates = BTreeSet::new();
    let mut forms = BTreeMap::<String, proc_macro2::TokenStream>::new();
    let mut families = BTreeMap::<String, (usize, bool)>::new();
    let mut family_docs = BTreeMap::<String, BTreeMap<String, String>>::new();
    let mut documentation = BTreeMap::<String, InstructionDoc>::new();
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
    let instructions = child(isa, "Instructions").ok_or("missing Instructions")?;
    for instruction in instructions
        .children()
        .filter(|n| n.has_tag_name("Instruction"))
    {
        let operation = text(instruction, "InstructionName").to_lowercase();
        if operation.is_empty() {
            return Err("instruction has no name".into());
        }
        inventory.insert(operation.clone());
        let description = text(instruction, "Description")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        let parts: Vec<_> = operation.split('_').collect();
        if parts.first() != Some(&"v") {
            continue;
        }
        let mut split = parts.len();
        while split > 0 && value_type(parts[split - 1]).is_some() {
            split -= 1;
        }
        if split == parts.len() {
            continue;
        }
        let bit_type = parts
            .last()
            .and_then(|p| value_type(p))
            .filter(|t| t.starts_with('B'));
        let Some(encodings) = child(instruction, "InstructionEncodings") else {
            continue;
        };
        for form in encodings
            .children()
            .filter(|n| n.has_tag_name("InstructionEncoding"))
        {
            if text(form, "EncodingCondition") != "default" {
                continue;
            }
            let (encoding_name, e64) = match text(form, "EncodingName") {
                "ENC_VOP1" | "ENC_VOP2" | "ENC_VOPC" => ("E32", false),
                "ENC_VOP3" => ("E64", true),
                _ => continue,
            };
            let encoding = ident(encoding_name);
            let mnemonic = format!("{operation}_{}", encoding_name.to_lowercase());
            let Some(operands) = child(form, "Operands") else {
                continue;
            };
            let mut operands: Vec<_> = operands
                .children()
                .filter(|n| n.has_tag_name("Operand"))
                .collect();
            operands.sort_by_key(|o| {
                o.attribute("Order")
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(usize::MAX)
            });
            if operands
                .iter()
                .any(|o| o.attribute("IsImplicit") != Some("false"))
            {
                continue;
            }
            if operands.len() < 2 || operands.len() > 4 {
                continue;
            }
            let output = operands[0];
            let sources = &operands[1..];
            if output.attribute("Output") != Some("true")
                || output.attribute("Input") != Some("false")
                || sources.iter().any(|o| {
                    o.attribute("Input") != Some("true") || o.attribute("Output") != Some("false")
                })
            {
                continue;
            }
            if sources.iter().enumerate().any(|(i, o)| {
                !matches!(text(*o, "FieldName"), "SRC0" | "SRC1" | "SRC2" | "VSRC1")
                    || (text(*o, "FieldName") != format!("SRC{i}")
                        && !(i == 1 && text(*o, "FieldName") == "VSRC1"))
            }) {
                continue;
            }
            let data: Option<Vec<_>> = sources.iter().map(|o| operand_type(*o, bit_type)).collect();
            let Some(source_data) = data else {
                continue;
            };
            if parts.starts_with(&["v", "cmp"]) && parts.len() == 4 {
                if sources.len() != 2 || source_data[0] != source_data[1] {
                    continue;
                }
                let expected = if e64 { "OPR_SREG" } else { "OPR_VCC" };
                if text(output, "OperandType") != expected
                    || text(output, "DataFormatName") != "FMT_NUM_M64"
                {
                    continue;
                }
                if !matches!(text(sources[0], "OperandType"), "OPR_SRC" | "OPR_SRC_NOLDS")
                    || text(sources[1], "OperandType")
                        != if e64 { "OPR_SRC_NOLDS" } else { "OPR_VGPR" }
                {
                    continue;
                }
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
                let ty = &source_data[0];
                predicates.insert(predicate.to_owned());
                comparisons.insert((
                    predicate.to_owned(),
                    ty.to_string(),
                    encoding_name.to_owned(),
                ));
                let p = ident(predicate);
                let instruction =
                    quote! { crate::VCmp<predicate::#p, crate::#ty, crate::#encoding> };
                let expression = quote! {
                    let fields = crate::decode::fields(text, 3)?;
                    <#instruction>::new(crate::decode::operand(&fields[0])?, crate::decode::operand(&fields[1])?, crate::decode::operand(&fields[2])?)
                };
                let variant = rust_name(&mnemonic);
                documentation.insert(
                    variant.clone(),
                    InstructionDoc {
                        mnemonic: mnemonic.clone(),
                        description: description.clone(),
                        family: None,
                        predicate: Some(predicate.to_owned()),
                        data_types: source_data.iter().map(ToString::to_string).collect(),
                        sources: sources
                            .iter()
                            .map(|o| text(*o, "OperandType").to_owned())
                            .collect(),
                        e64,
                    },
                );
                decoders.insert(variant, (mnemonic, instruction, expression, !e64));
                continue;
            }
            if text(output, "OperandType") != "OPR_VGPR" || text(output, "FieldName") != "VDST" {
                continue;
            }
            let Some(destination) = operand_type(output, bit_type) else {
                continue;
            };
            let mut data_types = vec![destination];
            data_types.extend(source_data);
            let operand_types: Option<Vec<_>> = sources
                .iter()
                .zip(&data_types[1..])
                .map(|(o, t)| source_type(*o, t, e64))
                .collect();
            let Some(operand_types) = operand_types else {
                continue;
            };
            let family_name = rust_name(&parts[..split].join("_"));
            let family = ident(&family_name);
            let arity = sources.len();
            let homogeneous = data_types.iter().all(|t| *t == data_types[0]);
            let entry = families
                .entry(family_name.clone())
                .or_insert((arity, homogeneous));
            if entry.0 != arity {
                return Err(format!("conflicting arities for {family_name}").into());
            }
            entry.1 &= homogeneous;
            family_docs
                .entry(family_name.clone())
                .or_default()
                .insert(operation.clone(), description.clone());
            let (private, support, container) = match arity {
                1 => (
                    ident("Unary"),
                    ident("SupportedUnary"),
                    ident("VectorUnary"),
                ),
                2 => (
                    ident("Binary"),
                    ident("SupportedBinary"),
                    ident("VectorBinary"),
                ),
                _ => (
                    ident("Ternary"),
                    ident("SupportedTernary"),
                    ident("VectorTernary"),
                ),
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
            if let Some(old) = forms.insert(key.clone(), tokens.clone()) {
                if old.to_string() != tokens.to_string() {
                    return Err(format!("conflicting operand schemas for {key}").into());
                }
            }
            let instruction = quote! { crate::#container<family::#family, #(crate::#data_types,)* crate::#encoding> };
            let indexes = 0..=arity;
            let count = arity + 1;
            let expression = quote! {
                let fields = crate::decode::fields(text, #count)?;
                <#instruction>::new(#(crate::decode::operand(&fields[#indexes])?,)*)
            };
            let variant = rust_name(&mnemonic);
            documentation.insert(
                variant.clone(),
                InstructionDoc {
                    mnemonic: mnemonic.clone(),
                    description: description.clone(),
                    family: Some(family_name.clone()),
                    predicate: None,
                    data_types: data_types.iter().map(ToString::to_string).collect(),
                    sources: sources
                        .iter()
                        .map(|o| text(*o, "OperandType").to_owned())
                        .collect(),
                    e64,
                },
            );
            decoders.insert(variant, (mnemonic, instruction, expression, !e64));
        }
    }
    if comparisons.is_empty() || forms.is_empty() {
        return Err("no supported RDNA 2 instruction forms found".into());
    }
    let temporary = root.join("src/generated.tmp.rs");
    let mut output = BufWriter::new(File::create(&temporary)?);
    write_doc(&mut output, "AMD ISA specification release date.")?;
    writeln!(
        output,
        "{}",
        quote! { pub const ISA_RELEASE_DATE: &str = #release; }
    )?;
    write_doc(&mut output, "AMD ISA XML schema version.")?;
    writeln!(
        output,
        "{}",
        quote! { pub const ISA_SCHEMA_VERSION: &str = #schema; }
    )?;
    write_doc(
        &mut output,
        "Architecture described by the generated instruction types.",
    )?;
    writeln!(
        output,
        "{}",
        quote! { pub const ISA_ARCHITECTURE: &str = "AMD RDNA 2"; }
    )?;
    write_doc(
        &mut output,
        "Comparison predicates used by [`crate::VCmp`].",
    )?;
    writeln!(output, "pub mod predicate {{")?;
    for name in &predicates {
        write_doc(&mut output, predicate_doc(name))?;
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
    write_doc(
        &mut output,
        "Instruction family markers used by the typed vector instructions.",
    )?;
    writeln!(output, "pub mod family {{")?;
    for name in families.keys() {
        write_doc(
            &mut output,
            &family_doc(&family_docs[name], name, &documentation, &families),
        )?;
        let name = ident(name);
        writeln!(
            output,
            "{}",
            quote! { #[derive(Debug, Clone, Copy, PartialEq, Eq)] pub struct #name; }
        )?;
    }
    writeln!(output, "}}")?;
    for (name, (arity, homogeneous)) in &families {
        write_doc(
            &mut output,
            &family_doc(&family_docs[name], name, &documentation, &families),
        )?;
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
    write_doc(
        &mut output,
        "A supported RDNA 2 instruction decoded from assembly using wave32 lane masks.",
    )?;
    writeln!(
        output,
        "#[derive(Debug, Clone, Copy, PartialEq)] pub enum DecodedInstruction {{"
    )?;
    for (variant, (_, ty, _, _)) in &decoders {
        write_doc(&mut output, &documentation[variant].render(&families))?;
        let variant = ident(variant);
        writeln!(output, "{}", quote! { #variant(#ty), })?;
    }
    writeln!(output, "}}")?;
    write_doc(
        &mut output,
        "Parses a supported RDNA 2 assembly instruction with wave32 lane masks.",
    )?;
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
    write_doc(
        &mut output,
        "All instruction mnemonics in the RDNA 2 specification, including forms without a typed decoder.",
    )?;
    writeln!(
        output,
        "{}",
        quote! { pub const KNOWN_MNEMONICS: &[&str] = &[#(#inventory,)*]; }
    )?;
    output.flush()?;
    let status = std::process::Command::new("rustfmt")
        .args(["--edition", "2024"])
        .arg(&temporary)
        .status()?;
    if !status.success() {
        return Err("rustfmt failed".into());
    }
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

struct InstructionDoc {
    mnemonic: String,
    description: String,
    family: Option<String>,
    predicate: Option<String>,
    data_types: Vec<String>,
    sources: Vec<String>,
    e64: bool,
}

impl InstructionDoc {
    fn render(&self, families: &BTreeMap<String, (usize, bool)>) -> String {
        let encoding = if self.e64 { "E64" } else { "E32" };
        let comparison = self.predicate.is_some();
        let destination_type = if comparison {
            if self.e64 {
                "ScalarDestination<Wave32>".to_owned()
            } else {
                "Vcc<Wave32>".to_owned()
            }
        } else {
            format!("VectorRegister<{}>", self.data_types[0])
        };
        let destination_description = if comparison {
            if self.e64 {
                "VCC or a scalar register containing a wave32 lane mask"
            } else {
                "VCC containing a wave32 lane mask"
            }
        } else {
            "vector register or register group"
        };
        let mut operands = vec![format!(
            "- destination: `{destination_type}` — {destination_description}."
        )];
        let mut assembly_operands = vec![if comparison {
            "vcc_lo".to_owned()
        } else {
            register_example(&self.data_types[0], 0)
        }];
        let mut rust_operands = vec![if comparison {
            if self.e64 {
                "ScalarDestination::Vcc(Vcc::default())".to_owned()
            } else {
                "Vcc::default()".to_owned()
            }
        } else {
            "VectorRegister::new(0).unwrap()".to_owned()
        }];
        let source_types = if comparison {
            &self.data_types[..]
        } else {
            &self.data_types[1..]
        };
        for (index, (ty, class)) in source_types.iter().zip(&self.sources).enumerate() {
            let (rust_type, accepts) = match class.as_str() {
                "OPR_VGPR" => ("VectorRegister", "vector register or register group"),
                "OPR_SREG" => ("ScalarRegister", "scalar register or register group"),
                _ if self.e64 => (
                    "ModifiedSource",
                    "vector register, scalar register, supported special register, or immediate; floating-point sources also support negate and absolute modifiers",
                ),
                _ => (
                    "SourceOperand",
                    "vector register, scalar register, supported special register, or immediate",
                ),
            };
            operands.push(format!("- src{index}: `{rust_type}<{ty}>` — {accepts}."));
            let register = 2 * (index + 1);
            let immediate = index == 0 && ty == "U32" && class.starts_with("OPR_SRC");
            assembly_operands.push(if immediate {
                "42".to_owned()
            } else if class == "OPR_SREG" {
                register_example(ty, register).replacen('v', "s", 1)
            } else {
                register_example(ty, register)
            });
            let source = if immediate {
                "SourceOperand::Immediate(Immediate::new(42))".to_owned()
            } else {
                let register_type = if class == "OPR_SREG" {
                    "ScalarRegister"
                } else {
                    "VectorRegister"
                };
                let register = format!("{register_type}::new({register}).unwrap()");
                if class.starts_with("OPR_SRC") {
                    format!("SourceOperand::VectorRegister({register})")
                } else {
                    register
                }
            };
            rust_operands.push(if class.starts_with("OPR_SRC") && self.e64 {
                format!("ModifiedSource::new({source})")
            } else {
                source
            });
        }
        let instruction_type = if let Some(predicate) = &self.predicate {
            format!(
                "VCmp::<predicate::{predicate}, {}, {encoding}>",
                self.data_types[0]
            )
        } else {
            let family = self.family.as_ref().unwrap();
            let types = if families[family].1 {
                vec![self.data_types[0].clone()]
            } else {
                self.data_types.clone()
            };
            format!("{family}::<{}, {encoding}>", types.join(", "))
        };
        let assembly = format!("{} {}", self.mnemonic, assembly_operands.join(", "));
        let variant = rust_name(&self.mnemonic);
        format!(
            "{}\n\nOperands:\n\n{}\n\nAssembly:\n\n```text\n{assembly}\n```\n\nRust:\n\n```rust\nuse llvm_amdgpu_types::*;\n\nlet instruction = {instruction_type}::new(\n    {},\n);\nlet decoded = parse(\"{assembly}\").unwrap();\nassert_eq!(decoded, DecodedInstruction::{variant}(instruction));\n```",
            self.description,
            operands.join("\n"),
            rust_operands.join(",\n    ")
        )
    }
}

fn register_example(ty: &str, index: usize) -> String {
    let bits: usize = ty[1..].parse().unwrap();
    let words = bits.div_ceil(32);
    if words == 1 {
        format!("v{index}")
    } else {
        format!("v[{index}:{}]", index + words - 1)
    }
}

fn family_doc(
    descriptions: &BTreeMap<String, String>,
    family: &str,
    documentation: &BTreeMap<String, InstructionDoc>,
    families: &BTreeMap<String, (usize, bool)>,
) -> String {
    let summary = descriptions
        .values()
        .next()
        .map(String::as_str)
        .unwrap_or("");
    let forms = documentation
        .values()
        .filter(|doc| doc.family.as_deref() == Some(family))
        .map(|doc| format!("### `{}`\n\n{}", doc.mnemonic, doc.render(families)))
        .collect::<Vec<_>>()
        .join("\n\n");
    format!("{summary}\n\n## Supported forms\n\n{forms}")
}

fn write_doc(output: &mut impl Write, documentation: &str) -> std::io::Result<()> {
    for line in documentation.lines() {
        if line.is_empty() {
            writeln!(output, "///")?;
        } else {
            writeln!(output, "/// {line}")?;
        }
    }
    Ok(())
}

fn predicate_doc(name: &str) -> &'static str {
    match name {
        "Always" => "Comparison predicate that is always true.",
        "Never" => "Comparison predicate that is always false.",
        "Eq" => "Equal comparison predicate.",
        "Ne" => "Not equal comparison predicate.",
        "Lt" => "Less than comparison predicate.",
        "Le" => "Less than or equal comparison predicate.",
        "Gt" => "Greater than comparison predicate.",
        "Ge" => "Greater than or equal comparison predicate.",
        "Lg" => "Ordered and not equal floating-point comparison predicate.",
        "Ordered" => "Floating-point comparison predicate that is true when neither input is NaN.",
        "Unordered" => "Floating-point comparison predicate that is true when either input is NaN.",
        "NotLt" => "Negation of the less than floating-point comparison predicate.",
        "NotLe" => "Negation of the less than or equal floating-point comparison predicate.",
        "NotGt" => "Negation of the greater than floating-point comparison predicate.",
        "NotGe" => "Negation of the greater than or equal floating-point comparison predicate.",
        "NotLg" => "Unordered or equal floating-point comparison predicate.",
        _ => unreachable!("unknown generated predicate"),
    }
}
