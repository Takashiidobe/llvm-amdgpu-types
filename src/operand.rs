use std::fmt::Debug;
use std::marker::PhantomData;

/// The data interpretation, storage type, and width of an operand.
pub trait OperandType: crate::private::Sealed + Copy + Debug + PartialEq {
    type Value: Copy + Debug + PartialEq;
    const BITS: u16;
    const IS_FLOAT: bool = false;
}

macro_rules! operand_types {
    ($( $(#[$meta:meta])* $name:ident: $value:ty = $bits:literal),* $(,)?) => {
        $(
            $(#[$meta])*
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub struct $name;
            impl crate::private::Sealed for $name {}
            impl OperandType for $name {
                type Value = $value;
                const BITS: u16 = $bits;
                const IS_FLOAT: bool = matches!(stringify!($name).as_bytes()[0], b'F');
            }
        )*
    };
}

operand_types! {
    /// 8-bit unsigned integer operand type.
    U8: u8 = 8,
    /// 16-bit unsigned integer operand type.
    U16: u16 = 16,
    /// 32-bit unsigned integer operand type.
    U32: u32 = 32,
    /// 64-bit unsigned integer operand type.
    U64: u64 = 64,
    /// 8-bit signed integer operand type.
    I8: i8 = 8,
    /// 16-bit signed integer operand type.
    I16: i16 = 16,
    /// 32-bit signed integer operand type.
    I32: i32 = 32,
    /// 64-bit signed integer operand type.
    I64: i64 = 64,
    /// 16-bit floating-point value operand type.
    F16: F16Bits = 16,
    /// 32-bit floating-point value operand type.
    F32: f32 = 32,
    /// 64-bit floating-point value operand type.
    F64: f64 = 64,
    /// 16-bit bit pattern operand type.
    B16: u16 = 16,
    /// 32-bit bit pattern operand type.
    B32: u32 = 32,
    /// 64-bit bit pattern operand type.
    B64: u64 = 64,
    /// 128-bit bit pattern operand type.
    B128: [u32; 4] = 128,
    /// 256-bit bit pattern operand type.
    B256: [u32; 8] = 256,
    /// 512-bit bit pattern operand type.
    B512: [u32; 16] = 512,
    /// 1024-bit bit pattern operand type.
    B1024: [u32; 32] = 1024,
}

/// The raw bits of a 16-bit floating-point immediate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct F16Bits(pub u16);

/// An immediate value interpreted as operand type `T`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Immediate<T: OperandType>(T::Value);

impl<T: OperandType> Immediate<T> {
    pub fn new(value: T::Value) -> Self {
        Self(value)
    }

    pub fn value(self) -> T::Value {
        self.0
    }
}

/// A vector register or consecutive register group interpreted as `T`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VectorRegister<T: OperandType> {
    index: u16,
    marker: PhantomData<T>,
}

impl<T: OperandType> VectorRegister<T> {
    pub fn new(index: u16) -> Option<Self> {
        let words = T::BITS.div_ceil(32);
        (index < 256 && words <= 256 - index).then_some(Self {
            index,
            marker: PhantomData,
        })
    }

    pub fn index(self) -> u16 {
        self.index
    }
}

/// An aligned scalar register or consecutive register group interpreted as `T`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScalarRegister<T: OperandType> {
    index: u16,
    marker: PhantomData<T>,
}

impl<T: OperandType> ScalarRegister<T> {
    pub fn new(index: u16) -> Option<Self> {
        let words = T::BITS.div_ceil(32);
        (index < 106 && words <= 106 - index && index.is_multiple_of(words)).then_some(Self {
            index,
            marker: PhantomData,
        })
    }

    pub fn index(self) -> u16 {
        self.index
    }
}

/// Names of the special registers supported by this crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecialRegisterName {
    VccLo,
    VccHi,
    Vcc,
    ExecLo,
    ExecHi,
    Exec,
    M0,
    Scc,
}

/// A special register interpreted as operand type `T`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpecialRegister<T: OperandType> {
    name: SpecialRegisterName,
    marker: PhantomData<T>,
}

impl<T: OperandType> SpecialRegister<T> {
    pub fn new(name: SpecialRegisterName) -> Option<Self> {
        let bits = match name {
            SpecialRegisterName::Vcc | SpecialRegisterName::Exec => 64,
            _ => 32,
        };
        (T::BITS == bits || (bits == 32 && T::BITS < 32)).then_some(Self {
            name,
            marker: PhantomData,
        })
    }

    pub fn name(self) -> SpecialRegisterName {
        self.name
    }
}

/// A vector, scalar, or special register, or an immediate source value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SourceOperand<T: OperandType> {
    VectorRegister(VectorRegister<T>),
    ScalarRegister(ScalarRegister<T>),
    SpecialRegister(SpecialRegister<T>),
    Immediate(Immediate<T>),
}

/// The lane mask type associated with a wave size.
pub trait WaveSize: crate::private::Sealed + Copy + Debug + PartialEq {
    type Mask: OperandType;
}

/// A wave of 32 lanes, with a 32-bit lane mask.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Wave32;
/// A wave of 64 lanes, with a 64-bit lane mask.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Wave64;
impl crate::private::Sealed for Wave32 {}
impl crate::private::Sealed for Wave64 {}
impl WaveSize for Wave32 {
    type Mask = U32;
}
impl WaveSize for Wave64 {
    type Mask = U64;
}

/// The vector condition-code register for wave size `W`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vcc<W: WaveSize>(PhantomData<W>);

impl<W: WaveSize> Default for Vcc<W> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

/// VCC or a scalar register used as a lane mask destination.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScalarDestination<W: WaveSize> {
    Vcc(Vcc<W>),
    Register(ScalarRegister<W::Mask>),
}

/// A floating-point operand type supporting negate and absolute modifiers.
pub trait FloatType: OperandType {}
impl FloatType for F16 {}
impl FloatType for F32 {}
impl FloatType for F64 {}

/// A source operand with optional floating-point negate and absolute modifiers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModifiedSource<T: OperandType> {
    source: SourceOperand<T>,
    negate: bool,
    absolute: bool,
}

impl<T: OperandType> ModifiedSource<T> {
    pub fn new(source: SourceOperand<T>) -> Self {
        Self {
            source,
            negate: false,
            absolute: false,
        }
    }

    pub(crate) fn with_modifiers(
        source: SourceOperand<T>,
        negate: bool,
        absolute: bool,
    ) -> Option<Self> {
        (T::IS_FLOAT || (!negate && !absolute)).then_some(Self {
            source,
            negate,
            absolute,
        })
    }

    pub fn source(self) -> SourceOperand<T> {
        self.source
    }
    pub fn is_negated(self) -> bool {
        self.negate
    }
    pub fn is_absolute(self) -> bool {
        self.absolute
    }
}

impl<T: FloatType> ModifiedSource<T> {
    pub fn negate(mut self) -> Self {
        self.negate = !self.negate;
        self
    }
    pub fn absolute(mut self) -> Self {
        self.absolute = true;
        self
    }
}

/// A scalar register, special register, or immediate source value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScalarSourceOperand<T: OperandType> {
    Register(ScalarRegister<T>),
    SpecialRegister(SpecialRegister<T>),
    Immediate(Immediate<T>),
}
