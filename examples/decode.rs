fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    println!("{:#?}", llvm_amdgpu_types::parse(&text)?);
    Ok(())
}
