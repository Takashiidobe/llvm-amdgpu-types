use std::fmt::Debug;
use std::marker::PhantomData;

pub trait OperandType: crate::private::Sealed + Copy + Debug + PartialEq {
    type Value: Copy + Debug + PartialEq;
    const BITS: u16;
    const IS_FLOAT: bool = false;
}

macro_rules! operand_types {
    ($($name:ident: $value:ty = $bits:literal),* $(,)?) => {
        $(
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
    U8: u8 = 8, U16: u16 = 16, U32: u32 = 32, U64: u64 = 64,
    I8: i8 = 8, I16: i16 = 16, I32: i32 = 32, I64: i64 = 64,
    F16: F16Bits = 16, F32: f32 = 32, F64: f64 = 64,
    B16: u16 = 16, B32: u32 = 32, B64: u64 = 64,
    B128: [u32; 4] = 128, B256: [u32; 8] = 256,
    B512: [u32; 16] = 512, B1024: [u32; 32] = 1024,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct F16Bits(pub u16);

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SourceOperand<T: OperandType> {
    VectorRegister(VectorRegister<T>),
    ScalarRegister(ScalarRegister<T>),
    SpecialRegister(SpecialRegister<T>),
    Immediate(Immediate<T>),
}

pub trait WaveSize: crate::private::Sealed + Copy + Debug + PartialEq {
    type Mask: OperandType;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Wave32;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vcc<W: WaveSize>(PhantomData<W>);

impl<W: WaveSize> Default for Vcc<W> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScalarDestination<W: WaveSize> {
    Vcc(Vcc<W>),
    Register(ScalarRegister<W::Mask>),
}

pub trait FloatType: OperandType {}
impl FloatType for F16 {}
impl FloatType for F32 {}
impl FloatType for F64 {}

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScalarSourceOperand<T: OperandType> {
    Register(ScalarRegister<T>),
    SpecialRegister(SpecialRegister<T>),
    Immediate(Immediate<T>),
}
