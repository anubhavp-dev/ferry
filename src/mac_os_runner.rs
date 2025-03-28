use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <assembly_file.s>", args[0]);
        return;
    }

    let asm_file = &args[1];
    let output_file = asm_file.replace(".s", "");

    print!("The output file will be: {}.o\n", output_file);

    // Assemble
    println!("Assembling {}...", asm_file);
    let assemble_status = Command::new("riscv64-unknown-elf-as")
        .arg(asm_file)
        .arg("-o")
        .arg(format!("{}.o", output_file))
        .status()
        .expect("Failed to execute assembler");

    if !assemble_status.success() {
        println!("Assembly failed");
        return;
    }

    // Link - use the same toolchain for linking
    println!("Linking...");
    let link_status = Command::new("riscv64-unknown-elf-gcc")
        .arg(format!("{}.o", &output_file))
        .arg("-o")
        .arg(&output_file)
        .status()
        .expect("Failed to execute linker");

    if !link_status.success() {
        println!("Linking failed");
        return;
    }

    // Run with Spike (RISC-V reference simulator)
    println!("Running with Spike...");
    let run_status = Command::new("spike")
        .arg("pk")
        .arg(&output_file)
        .status()
        .expect("Failed to execute Spike");

    println!("Program exited with status: {}", run_status);
}
