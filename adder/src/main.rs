/// Compile a source program into a string of x86-64 assembly code.
fn compile(program: String) -> String {
    let num = program.trim().parse::<i32>().unwrap();
    return format!("mov rax, {}", num);
}

fn main() {
    let program = String::from("37");
    let assembly = compile(program);
    println!("{}", assembly);
}
