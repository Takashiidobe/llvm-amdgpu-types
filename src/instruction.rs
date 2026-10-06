use std::fmt::Debug;
use std::marker::PhantomData;

use crate::{
    ModifiedSource, OperandType, ScalarDestination, SourceOperand, Vcc, VectorRegister, WaveSize,
};

/// 32-bit instruction encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct E32;
/// 64-bit instruction encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct E64;
impl crate::private::Sealed for E32 {}
impl crate::private::Sealed for E64 {}

/// Operand types for a comparison encoding and wave size.
pub trait CompareEncoding<T: OperandType, W: WaveSize>:
    crate::private::Sealed + Copy + Debug + PartialEq
{
    type Destination: Copy + Debug + PartialEq;
    type Source0: Copy + Debug + PartialEq;
    type Source1: Copy + Debug + PartialEq;
}

impl<T: OperandType, W: WaveSize> CompareEncoding<T, W> for E32 {
    type Destination = Vcc<W>;
    type Source0 = SourceOperand<T>;
    type Source1 = VectorRegister<T>;
}

impl<T: OperandType, W: WaveSize> CompareEncoding<T, W> for E64 {
    type Destination = ScalarDestination<W>;
    type Source0 = ModifiedSource<T>;
    type Source1 = ModifiedSource<T>;
}

/// A supported combination of comparison predicate, input type, and encoding.
pub trait SupportedCompare<P, T, E>: crate::private::Compare<P, T, E> {}

/// Compares two inputs per lane and produces a lane mask.
///
/// `P` selects the predicate, `T` selects the input data type, and `E` selects
/// the encoding. `W` selects the mask width and defaults to [`crate::Wave32`].
///
/// With [`E32`], the destination is [`Vcc<W>`], the first source is
/// [`SourceOperand<T>`], and the second source is [`VectorRegister<T>`].
/// With [`E64`], the destination is [`ScalarDestination<W>`] and both sources
/// are [`ModifiedSource<T>`]. The text decoder currently uses wave32.
/// Supported predicates, input types, and encodings are enforced by
/// [`SupportedCompare`].
///
/// Assembly:
///
/// ```text
/// v_cmp_eq_u32_e32 vcc_lo, 42, v1
/// ```
///
/// Rust:
///
/// ```rust
/// use llvm_amdgpu_types::*;
///
/// let instruction = VCmp::<predicate::Eq, U32, E32>::new(
///     Vcc::default(),
///     SourceOperand::Immediate(Immediate::new(42)),
///     VectorRegister::new(1).unwrap(),
/// );
/// let decoded = parse("v_cmp_eq_u32_e32 vcc_lo, 42, v1").unwrap();
/// assert_eq!(decoded, DecodedInstruction::VCmpEqU32E32(instruction));
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VCmp<P, T, E, W = crate::Wave32>
where
    T: OperandType,
    W: WaveSize,
    E: CompareEncoding<T, W>,
    (): SupportedCompare<P, T, E>,
{
    destination: E::Destination,
    src0: E::Source0,
    src1: E::Source1,
    marker: PhantomData<(P, T, W)>,
}

impl<P, T, E, W> VCmp<P, T, E, W>
where
    T: OperandType,
    W: WaveSize,
    E: CompareEncoding<T, W>,
    (): SupportedCompare<P, T, E>,
{
    pub fn new(destination: E::Destination, src0: E::Source0, src1: E::Source1) -> Self {
        Self {
            destination,
            src0,
            src1,
            marker: PhantomData,
        }
    }

    pub fn destination(&self) -> E::Destination {
        self.destination
    }
    pub fn src0(&self) -> E::Source0 {
        self.src0
    }
    pub fn src1(&self) -> E::Source1 {
        self.src1
    }
}

/// An instruction encoding supported by the typed vector instructions.
pub trait Encoding: crate::private::Sealed + Copy + Debug + PartialEq {}
impl Encoding for E32 {}
impl Encoding for E64 {}

/// Source operand type for a supported unary instruction form.
pub trait SupportedUnary<F, D, S, E>: crate::private::Unary<F, D, S, E> {
    type Source: Copy + Debug + PartialEq;
}

/// Source operand types for a supported binary instruction form.
pub trait SupportedBinary<F, D, S0, S1, E>: crate::private::Binary<F, D, S0, S1, E> {
    type Source0: Copy + Debug + PartialEq;
    type Source1: Copy + Debug + PartialEq;
}

/// Source operand types for a supported ternary instruction form.
pub trait SupportedTernary<F, D, S0, S1, S2, E>:
    crate::private::Ternary<F, D, S0, S1, S2, E>
{
    type Source0: Copy + Debug + PartialEq;
    type Source1: Copy + Debug + PartialEq;
    type Source2: Copy + Debug + PartialEq;
}

/// A vector instruction with one source and a vector register destination.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VectorUnary<F, D, S, E>
where
    D: OperandType,
    S: OperandType,
    E: Encoding,
    (): SupportedUnary<F, D, S, E>,
{
    destination: VectorRegister<D>,
    src0: <() as SupportedUnary<F, D, S, E>>::Source,
    marker: PhantomData<(F, S, E)>,
}

impl<F, D, S, E> VectorUnary<F, D, S, E>
where
    D: OperandType,
    S: OperandType,
    E: Encoding,
    (): SupportedUnary<F, D, S, E>,
{
    pub fn new(
        destination: VectorRegister<D>,
        src0: <() as SupportedUnary<F, D, S, E>>::Source,
    ) -> Self {
        Self {
            destination,
            src0,
            marker: PhantomData,
        }
    }
    pub fn destination(&self) -> VectorRegister<D> {
        self.destination
    }
    pub fn src0(&self) -> <() as SupportedUnary<F, D, S, E>>::Source {
        self.src0
    }
}

/// A vector instruction with two sources and a vector register destination.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VectorBinary<F, D, S0, S1, E>
where
    D: OperandType,
    S0: OperandType,
    S1: OperandType,
    E: Encoding,
    (): SupportedBinary<F, D, S0, S1, E>,
{
    destination: VectorRegister<D>,
    src0: <() as SupportedBinary<F, D, S0, S1, E>>::Source0,
    src1: <() as SupportedBinary<F, D, S0, S1, E>>::Source1,
    marker: PhantomData<(F, S0, S1, E)>,
}

impl<F, D, S0, S1, E> VectorBinary<F, D, S0, S1, E>
where
    D: OperandType,
    S0: OperandType,
    S1: OperandType,
    E: Encoding,
    (): SupportedBinary<F, D, S0, S1, E>,
{
    pub fn new(
        destination: VectorRegister<D>,
        src0: <() as SupportedBinary<F, D, S0, S1, E>>::Source0,
        src1: <() as SupportedBinary<F, D, S0, S1, E>>::Source1,
    ) -> Self {
        Self {
            destination,
            src0,
            src1,
            marker: PhantomData,
        }
    }
    pub fn destination(&self) -> VectorRegister<D> {
        self.destination
    }
    pub fn src0(&self) -> <() as SupportedBinary<F, D, S0, S1, E>>::Source0 {
        self.src0
    }
    pub fn src1(&self) -> <() as SupportedBinary<F, D, S0, S1, E>>::Source1 {
        self.src1
    }
}

/// A vector instruction with three sources and a vector register destination.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VectorTernary<F, D, S0, S1, S2, E>
where
    D: OperandType,
    S0: OperandType,
    S1: OperandType,
    S2: OperandType,
    E: Encoding,
    (): SupportedTernary<F, D, S0, S1, S2, E>,
{
    destination: VectorRegister<D>,
    src0: <() as SupportedTernary<F, D, S0, S1, S2, E>>::Source0,
    src1: <() as SupportedTernary<F, D, S0, S1, S2, E>>::Source1,
    src2: <() as SupportedTernary<F, D, S0, S1, S2, E>>::Source2,
    marker: PhantomData<(F, S0, S1, S2, E)>,
}

impl<F, D, S0, S1, S2, E> VectorTernary<F, D, S0, S1, S2, E>
where
    D: OperandType,
    S0: OperandType,
    S1: OperandType,
    S2: OperandType,
    E: Encoding,
    (): SupportedTernary<F, D, S0, S1, S2, E>,
{
    pub fn new(
        destination: VectorRegister<D>,
        src0: <() as SupportedTernary<F, D, S0, S1, S2, E>>::Source0,
        src1: <() as SupportedTernary<F, D, S0, S1, S2, E>>::Source1,
        src2: <() as SupportedTernary<F, D, S0, S1, S2, E>>::Source2,
    ) -> Self {
        Self {
            destination,
            src0,
            src1,
            src2,
            marker: PhantomData,
        }
    }
    pub fn destination(&self) -> VectorRegister<D> {
        self.destination
    }
    pub fn src0(&self) -> <() as SupportedTernary<F, D, S0, S1, S2, E>>::Source0 {
        self.src0
    }
    pub fn src1(&self) -> <() as SupportedTernary<F, D, S0, S1, S2, E>>::Source1 {
        self.src1
    }
    pub fn src2(&self) -> <() as SupportedTernary<F, D, S0, S1, S2, E>>::Source2 {
        self.src2
    }
}
