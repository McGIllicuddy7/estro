pub mod arm;
pub mod asm;
pub mod est;
pub mod rtils;
pub mod x86;
use std::collections::HashSet;
fn main() {
    let cmds = est::translate_file(
        std::fs::read_to_string("test.est").unwrap(),
        "test.est".to_string(),
        &mut HashSet::new(),
    )
    .unwrap();
    println!("{:#?}", cmds);
    est::tuci(&cmds, "test.c".to_string());
    let asm = asm::transpile(&cmds);
    arm::compile_arm(&asm, "test2.s".to_string());
    println!("{:#?}", &asm);
}
