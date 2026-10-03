use std::fmt::Debug;
use std::marker::PhantomData;

use crate::{
    ModifiedSource, OperandType, ScalarDestination, SourceOperand, Vcc, VectorRegister, WaveSize,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct E32;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct E64;
impl crate::private::Sealed for E32 {}
impl crate::private::Sealed for E64 {}

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

pub trait SupportedCompare<P, T, E>: crate::private::Compare<P, T, E> {}

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

pub trait Encoding: crate::private::Sealed + Copy + Debug + PartialEq {}
impl Encoding for E32 {}
impl Encoding for E64 {}

pub trait SupportedUnary<F, D, S, E>: crate::private::Unary<F, D, S, E> {
    type Source: Copy + Debug + PartialEq;
}

pub trait SupportedBinary<F, D, S0, S1, E>: crate::private::Binary<F, D, S0, S1, E> {
    type Source0: Copy + Debug + PartialEq;
    type Source1: Copy + Debug + PartialEq;
}

pub trait SupportedTernary<F, D, S0, S1, S2, E>:
    crate::private::Ternary<F, D, S0, S1, S2, E>
{
    type Source0: Copy + Debug + PartialEq;
    type Source1: Copy + Debug + PartialEq;
    type Source2: Copy + Debug + PartialEq;
}

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
