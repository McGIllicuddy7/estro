use crate::asm::AsmUnit;

pub fn mangle_func(name: &str) -> String {
    format!("_est_{}", name)
}
pub fn compile_arm(trans: &AsmUnit, file: String) {
    let mut out = String::new();
    out += ".text\n";
    let mut has_main = false;
    for (i, f) in trans.functions.iter() {
        if i == "main" {
            has_main = true;
        }
        if !f.inline {
            out += &format!(".globl {}\n", mangle_func(i));
        }
    }
    for (i, f) in trans.functions.iter() {
        out += &format!("{}:\n", mangle_func(i));
        out += "sub sp, sp, #16\n";
        out += "str lr,[sp, #0]\n";
        out += "str fp,[sp, #8]\n";
        let mut sz = 16;
        for j in &f.variables {
            if j.is_byte() {
                sz += 1;
            } else {
                if sz % 8 != 0 {
                    sz += 8 - sz % 8;
                }
                sz += 8;
            }
        }
        if sz % 16 != 0 {
            sz += 16 - sz % 16;
        }
        out += &format!("\tmov fp, sp\n");
        out += &format!("\tsub sp,sp, #{}\n", sz);
        out += "mov sp, fp\n";
        out += "ldr lr,[sp, #0]\n";
        out += "ldr fp,[sp, #8]\n";
        out += "add sp, sp,#16\n";
        out += "br lr\n";
    }
    if has_main {
        out += ".globl _main\n";
        out += "_main:\n";
        out += "sub sp, sp, #16\n";
        out += "str lr,[sp, #0]\n";
        out += "str fp,[sp, #8]\n";
        out += "mov fp, sp\n";
        out += "bl _est_main\n";
        out += "mov sp, fp\n";
        out += "ldr lr,[sp, #0]\n";
        out += "ldr fp,[sp, #8]\n";
        out += "add sp, sp,#16\n";
        out += "br lr\n";
    }

    std::fs::write(file, out).unwrap();
}
