use std::ops::Range;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodeError {
    pub range: Range<usize>,
    pub reason: String,
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.reason)
    }
}

impl std::error::Error for DecodeError {}

pub(crate) struct Field<'a> {
    text: &'a str,
    range: Range<usize>,
}

pub(crate) fn fields(text: &str, count: usize) -> Result<Vec<Field<'_>>, DecodeError> {
    let start = text.find(char::is_whitespace).unwrap_or(text.len());
    let mut offset = start;
    let mut fields = Vec::new();
    for raw in text[start..].split(',') {
        let field = raw.trim();
        let position = offset + raw.len() - raw.trim_start().len();
        fields.push(Field {
            text: field,
            range: position..position + field.len(),
        });
        offset += raw.len() + 1;
    }
    if fields.len() != count || fields.iter().any(|field| field.text.is_empty()) {
        return Err(DecodeError {
            range: start..text.len(),
            reason: format!("expected {count} operands"),
        });
    }
    Ok(fields)
}

pub(crate) fn operand<T: ParseOperand>(field: &Field<'_>) -> Result<T, DecodeError> {
    T::parse(field.text).ok_or_else(|| DecodeError {
        range: field.range.clone(),
        reason: format!("expected {}, found {:?}", T::DESCRIPTION, field.text),
    })
}

pub(crate) trait ParseOperand: Sized {
    const DESCRIPTION: &'static str;
    fn parse(text: &str) -> Option<Self>;
}

pub(crate) trait Literal: OperandType {
    fn parse_literal(text: &str) -> Option<Self::Value>;
}

fn integer(text: &str) -> Option<i128> {
    let (negative, digits) = text
        .strip_prefix('-')
        .map_or((false, text), |digits| (true, digits));
    let value = if let Some(hex) = digits.strip_prefix("0x") {
        i128::from_str_radix(hex, 16).ok()?
    } else {
        digits.parse().ok()?
    };
    Some(if negative { -value } else { value })
}

macro_rules! integer_literals {
    ($($marker:ty => $value:ty, $bits:literal);* $(;)?) => {
        $(impl Literal for $marker {
            fn parse_literal(text: &str) -> Option<Self::Value> {
                let value = integer(text)?;
                (-(1i128 << ($bits - 1))..(1i128 << $bits)).contains(&value).then_some(value as $value)
            }
        })*
    };
}

integer_literals! {
    U8 => u8, 8; I8 => i8, 8;
    U16 => u16, 16; I16 => i16, 16; B16 => u16, 16;
    U32 => u32, 32; I32 => i32, 32; B32 => u32, 32;
    U64 => u64, 64; I64 => i64, 64; B64 => u64, 64;
}

impl Literal for F16 {
    fn parse_literal(text: &str) -> Option<F16Bits> {
        integer(text)
            .and_then(|value| u16::try_from(value).ok())
            .map(F16Bits)
    }
}

impl Literal for F32 {
    fn parse_literal(text: &str) -> Option<f32> {
        if text.contains('.') || (!text.starts_with("0x") && text.contains(['e', 'E'])) {
            text.parse().ok()
        } else {
            <B32 as Literal>::parse_literal(text).map(f32::from_bits)
        }
    }
}

impl Literal for F64 {
    fn parse_literal(text: &str) -> Option<f64> {
        if text.contains('.') || (!text.starts_with("0x") && text.contains(['e', 'E'])) {
            text.parse().ok()
        } else {
            <B64 as Literal>::parse_literal(text).map(f64::from_bits)
        }
    }
}

fn register_index<T: OperandType>(text: &str, prefix: char) -> Option<u16> {
    let digits = text.strip_prefix(prefix)?;
    let words = T::BITS.div_ceil(32);
    if let Some(group) = digits.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
        let (first, last) = group.split_once(':')?;
        let first: u16 = first.parse().ok()?;
        let last: u16 = last.parse().ok()?;
        (last.checked_sub(first)?.checked_add(1)? == words).then_some(first)
    } else {
        (words == 1 && !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()))
            .then(|| digits.parse().ok())
            .flatten()
    }
}

impl<T: OperandType> ParseOperand for VectorRegister<T> {
    const DESCRIPTION: &'static str = "vector register or register group";
    fn parse(text: &str) -> Option<Self> {
        Self::new(register_index::<T>(text, 'v')?)
    }
}

impl<T: OperandType> ParseOperand for ScalarRegister<T> {
    const DESCRIPTION: &'static str = "scalar register or register group";
    fn parse(text: &str) -> Option<Self> {
        Self::new(register_index::<T>(text, 's')?)
    }
}

impl<T: OperandType> ParseOperand for SpecialRegister<T> {
    const DESCRIPTION: &'static str = "special register of the appropriate width";
    fn parse(text: &str) -> Option<Self> {
        let name = match text {
            "vcc_lo" => SpecialRegisterName::VccLo,
            "vcc_hi" => SpecialRegisterName::VccHi,
            "vcc" => SpecialRegisterName::Vcc,
            "exec_lo" => SpecialRegisterName::ExecLo,
            "exec_hi" => SpecialRegisterName::ExecHi,
            "exec" => SpecialRegisterName::Exec,
            "m0" => SpecialRegisterName::M0,
            "scc" => SpecialRegisterName::Scc,
            _ => return None,
        };
        Self::new(name)
    }
}

impl<T: Literal> ParseOperand for SourceOperand<T> {
    const DESCRIPTION: &'static str = "register or numeric literal of the appropriate type";
    fn parse(text: &str) -> Option<Self> {
        VectorRegister::parse(text)
            .map(Self::VectorRegister)
            .or_else(|| ScalarRegister::parse(text).map(Self::ScalarRegister))
            .or_else(|| SpecialRegister::parse(text).map(Self::SpecialRegister))
            .or_else(|| T::parse_literal(text).map(|value| Self::Immediate(Immediate::new(value))))
    }
}

impl<T: Literal> ParseOperand for ScalarSourceOperand<T> {
    const DESCRIPTION: &'static str = "scalar register or numeric literal of the appropriate type";
    fn parse(text: &str) -> Option<Self> {
        ScalarRegister::parse(text)
            .map(Self::Register)
            .or_else(|| SpecialRegister::parse(text).map(Self::SpecialRegister))
            .or_else(|| T::parse_literal(text).map(|value| Self::Immediate(Immediate::new(value))))
    }
}

impl<T: Literal> ParseOperand for ModifiedSource<T> {
    const DESCRIPTION: &'static str = "source operand with supported modifiers";
    fn parse(text: &str) -> Option<Self> {
        if let Some(source) = SourceOperand::parse(text) {
            return Some(Self::new(source));
        }
        let (negate, text) = text
            .strip_prefix('-')
            .map_or((false, text), |text| (true, text));
        let (absolute, text) = text
            .strip_prefix('|')
            .and_then(|text| text.strip_suffix('|'))
            .or_else(|| {
                text.strip_prefix("abs(")
                    .and_then(|text| text.strip_suffix(')'))
            })
            .map_or((false, text), |text| (true, text));
        Self::with_modifiers(SourceOperand::parse(text)?, negate, absolute)
    }
}

impl ParseOperand for Vcc<Wave32> {
    const DESCRIPTION: &'static str = "wave32 destination vcc_lo";
    fn parse(text: &str) -> Option<Self> {
        (text == "vcc_lo").then(Self::default)
    }
}

impl ParseOperand for ScalarDestination<Wave32> {
    const DESCRIPTION: &'static str = "wave32 mask destination";
    fn parse(text: &str) -> Option<Self> {
        Vcc::parse(text)
            .map(Self::Vcc)
            .or_else(|| ScalarRegister::parse(text).map(Self::Register))
    }
}
