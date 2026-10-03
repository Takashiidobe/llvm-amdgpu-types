use llvm_amdgpu_types::{
    E32, F32, Immediate, SourceOperand, U32, VCmp, VFloor, Vcc, VectorRegister, predicate,
};

fn main() {
    let rhs = VectorRegister::<U32>::new(1).unwrap();
    let comparison = VCmp::<predicate::Eq, U32, E32>::new(
        Vcc::default(),
        SourceOperand::Immediate(Immediate::new(42)),
        rhs,
    );
    let floor = VFloor::<F32, E32>::new(
        VectorRegister::new(0).unwrap(),
        SourceOperand::VectorRegister(VectorRegister::new(1).unwrap()),
    );
    println!("{comparison:?}\n{floor:?}");
}
