# llvm-amdgpu-types

Rust types for AMDGPU instructions and operands, generated from LLVM w/
Tablegen.
Currently targets GFX10, RDNA 2.

## Types

Registers and immediates carry the type an instruction uses to interpret them.
For example, `VectorRegister<U32>` is a vector register used as an unsigned
32-bit integer, and `Immediate<F32>` holds an `f32`.

The crate includes unsigned (`U16`, `U32`, `U64`), signed (`I16`, `I32`, `I64`),
floating-point (`F16`, `F32`, `F64`), and bit-pattern types such as `B32`.
Operands can be vector registers, scalar registers, special registers, or
immediates. Register constructors check bounds, width, and scalar alignment.

```rust
use llvm_amdgpu_types::{Immediate, ScalarRegister, U32, VectorRegister};

let vector = VectorRegister::<U32>::new(0).unwrap();
let scalar = ScalarRegister::<U32>::new(4).unwrap();
let immediate = Immediate::<U32>::new(42);

assert_eq!(vector.index(), 0);
assert_eq!(scalar.index(), 4);
assert_eq!(immediate.value(), 42);
```

## Usage

Instruction types specify their operands and encoding.

```rust
use llvm_amdgpu_types::{E32, F32, SourceOperand, VFloor, VectorRegister};

let instruction = VFloor::<F32, E32>::new(
    VectorRegister::new(0).unwrap(),
    SourceOperand::VectorRegister(VectorRegister::new(1).unwrap()),
);
```

Other families include `VAdd`, `VCvt`, and `VCmp`. Their types restrict which
operands and encodings you can use.

To decode assembly, use `parse` and match on `DecodedInstruction`:

```rust
use llvm_amdgpu_types::{DecodedInstruction, parse};

let instruction = parse("v_floor_f32_e32 v0, v1")?;
if let DecodedInstruction::VFloorF32E32(floor) = instruction {
    println!("destination: v{}", floor.destination().index());
}
# Ok::<(), llvm_amdgpu_types::DecodeError>(())
```

The decoder currently uses wave32 and handles common vector ALU instructions.
It isn't a complete assembler: memory instructions, branches, DPP, and SDWA
aren't supported yet. Parse errors include a reason and a byte range in the
input.

## Examples

```sh
cargo run --example typed
cargo run --example decode -- 'v_floor_f32_e32 v0, v1'
```

## Generate

To regenerate the types from an LLVM checkout:

```sh
cargo run --release --manifest-path tools/generate/Cargo.toml -- ~/llvm-project
```

This runs `llvm-tblgen` and writes `src/generated.rs`. Set to use a
specific executable, or pass an existing TableGen JSON export as a second
argument. Normal builds use the checked-in generated source.

Generated material derives from LLVM and is licensed under Apache-2.0 with LLVM
exceptions. See [LICENSE.txt](LICENSE.txt).
