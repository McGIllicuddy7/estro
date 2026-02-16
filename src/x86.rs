use crate::asm::AsmUnit;

pub fn mangle_func(name: &str) -> String {
    format!("est_{}", name)
}
pub fn compile_x86(trans: &AsmUnit, file: String) {
    let mut out = String::new();
    out += ".intel_syntax noprefix\n";
    out += ".text\n";
    let mut has_main = false;
    for (i, f) in trans.functions.iter() {
        if i == "main" {
            has_main = true;
        }
        if !f.inline && !f.external {
            out += &format!(".globl {}\n", mangle_func(i));
        } else if f.external {
            out += &format!(".extern {}\n", mangle_func(i));
        }
    }
    out += ".extern memcpy\n";
    out += ".extern memset\n";
    for (i, f) in trans.functions.iter() {
        if f.external {
            continue;
        }
        out += &format!("{}:\n", mangle_func(i));
        out += "\tpush rbp\n";
        out += "\tmov rbp,rsp\n";
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
        out += &format!("\tsub rsp,{}\n", sz);
        /*    out += &format!("\tmov x0,fp\n");
        out += &format!("\tmov x2,0\n");
        out += &format!("\tmov x3,{}\n", sz);
        out += &format!("\tbl _memset\n");*/
        for (name, blck) in &f.blocks {
            if name != "" {
                out += &format!("{}:\n", name);
            }
            for is in &blck.instructions {
                out += &format!("\t#{:#?}", is)
                    .lines()
                    .fold(String::new(), |mut x, y| {
                        x += y;
                        x
                    });
                out += "\n";
                match is {
                    crate::asm::AsmIn::StackLoad {
                        reg,
                        index,
                        is_byte,
                        offset,
                    } => {
                        if let Some(of) = offset.as_ref() {
                            if *is_byte {
                                out += &format!(
                                    "\tmov {}, [rbp-{}-{}]\n",
                                    reg.name_x86_byte(),
                                    index,
                                    of.get_x86_name()
                                );
                            } else {
                                out += &format!(
                                    "\tldr {}, [rbp -{}-{}]\n",
                                    reg.name_x86(),
                                    index,
                                    of.get_x86_name()
                                );
                            }
                        } else {
                            if *is_byte {
                                out +=
                                    &format!("\tmov {}, [rbp -{}]\n", reg.name_x86_byte(), index);
                            } else {
                                out += &format!("\tmov {}, [rbp -{}]\n", reg.name_x86(), index);
                            }
                        }
                    }
                    crate::asm::AsmIn::StackStore {
                        reg,
                        index,
                        is_byte,
                        offset,
                    } => {
                        if let Some(of) = offset.as_ref() {
                            if *is_byte {
                                out += &format!(
                                    "\tmov [rsp -{}-{}],{}\n",
                                    index,
                                    of.get_x86_name(),
                                    reg.name_x86_byte(),
                                );
                            } else {
                                out += &format!(
                                    "\tmov [rsp -{}-{}],{}\n",
                                    reg.name_x86(),
                                    index,
                                    of.get_x86_name()
                                );
                            }
                        } else {
                            if *is_byte {
                                out +=
                                    &format!("\tmov [rsp -{}], {}\n", index, reg.name_x86_byte(),);
                            } else {
                                out += &format!("\tmov [rsp-{}], {}\n", index, reg.name_x86());
                            }
                        }
                    }
                    crate::asm::AsmIn::Binop {
                        op,
                        kind,
                        left,
                        right,
                        output,
                    } => {
                        match op {
                            crate::est::BinopType::Float => {
                                out += &format!("\tfmov d1, {}\n", left.name_x86());
                                out += &format!("\tfmov d2, {}\n", right.name_x86());
                                match kind {
                                    crate::est::BinOpKind::Add => {
                                        out += &format!("\tfadd d0, d1, d2\n");
                                        out += &format!("\tfmov {}, d0", output.name_x86());
                                    }
                                    crate::est::BinOpKind::Sub => {
                                        out += &format!("\tfsub d0, d1, d2\n");
                                        out += &format!("\tfmov {}, d0", output.name_x86());
                                    }
                                    crate::est::BinOpKind::Div => {
                                        out += &format!("\tfdiv d0, d1, d2\n");
                                        out += &format!("\tfmov {}, d0", output.name_x86());
                                    }
                                    crate::est::BinOpKind::Mul => {
                                        out += &format!("\tfmul d0, d1, d2\n");
                                        out += &format!("\tfmov {}, d0", output.name_x86());
                                    }
                                    crate::est::BinOpKind::Less => {
                                        out += &format!("\tfcmp,d1, d2\n");
                                        out += &format!("\tcset {}, lt", output.name_x86());
                                    }
                                    crate::est::BinOpKind::LessEq => {
                                        out += &format!("\tfcmp,d1, d2\n");
                                        out += &format!("\tcset {}, le", output.name_x86());
                                    }
                                    crate::est::BinOpKind::Eq => {
                                        out += &format!("\tfcmp,d1, d2\n");
                                        out += &format!("\tcset {}, eq", output.name_x86());
                                    }
                                    crate::est::BinOpKind::Neq => {
                                        out += &format!("\tfcmp,d1, d2\n");
                                        out += &format!("\tcset {}, ne", output.name_x86());
                                    }
                                    crate::est::BinOpKind::GreaterEq => {
                                        out += &format!("\tfcmp,d1, d2\n");
                                        out += &format!("\tcset {}, ge", output.name_x86());
                                    }
                                    crate::est::BinOpKind::Greater => {
                                        out += &format!("\tfcmp,d1, d2\n");
                                        out += &format!("\tcset {}, gt", output.name_x86());
                                    }
                                    _ => {
                                        todo!()
                                    }
                                }
                            }
                            crate::est::BinopType::IByte
                            | crate::est::BinopType::Byte
                            | crate::est::BinopType::IWord
                            | crate::est::BinopType::Word => match kind {
                                crate::est::BinOpKind::Add => {
                                    out += &format!(
                                        "\tmov {}, {}\n\tadd {},{}\n",
                                        output.name_x86(),
                                        left.name_x86(),
                                        output.name_x86(),
                                        right.name_x86()
                                    );
                                }
                                crate::est::BinOpKind::Sub => {
                                    out += &format!(
                                        "\tmov {}, {}\n\tsub {},{}\n",
                                        output.name_x86(),
                                        left.name_x86(),
                                        output.name_x86(),
                                        right.name_x86()
                                    );
                                }
                                crate::est::BinOpKind::Div => {
                                    out += &format!(
                                        "\tmov {}, {}\n\tdiv {},{}\n",
                                        output.name_x86(),
                                        left.name_x86(),
                                        output.name_x86(),
                                        right.name_x86()
                                    );
                                }
                                crate::est::BinOpKind::Mul => {
                                    out += &format!(
                                        "\tmov {}, {}\n\tmul {},{}\n",
                                        output.name_x86(),
                                        left.name_x86(),
                                        output.name_x86(),
                                        right.name_x86()
                                    );
                                }
                                crate::est::BinOpKind::Rem => {
                                    todo!();
                                }
                                //https://stackoverflow.com/questions/27284895/how-to-compare-a-signed-value-and-an-unsigned-value-in-x86-assembly
                                crate::est::BinOpKind::Less => {
                                    out += &format!(
                                        "\tcmp {}, {}\n",
                                        left.name_x86(),
                                        right.name_x86(),
                                    );
                                    if op.is_signed() {
                                        out += &format!("\tsetb {}\n", output.name_x86_byte());
                                    } else {
                                        out += &format!("\tsetl {}\n", output.name_x86_byte());
                                    }
                                }
                                crate::est::BinOpKind::LessEq => {
                                    out += &format!(
                                        "\tcmp {}, {}\n",
                                        left.name_x86(),
                                        right.name_x86(),
                                    );
                                    if op.is_signed() {
                                        out += &format!("\tsetbe {}\n", output.name_x86_byte());
                                    } else {
                                        out += &format!("\tcmovle {}\n", output.name_x86_byte());
                                    }
                                }
                                crate::est::BinOpKind::Eq => {
                                    out += &format!(
                                        "\tcmp {}, {}\n",
                                        left.name_x86(),
                                        right.name_x86(),
                                    );
                                    out += &format!("\tseteq {}\n", output.name_x86_byte());
                                }
                                crate::est::BinOpKind::Neq => {
                                    out += &format!(
                                        "\tcmp {}, {}\n",
                                        left.name_x86(),
                                        right.name_x86(),
                                    );
                                    out += &format!("\tsetne {}\n", output.name_x86_byte());
                                }
                                crate::est::BinOpKind::GreaterEq => {
                                    out += &format!(
                                        "\tcmp {}, {}\n",
                                        left.name_x86(),
                                        right.name_x86(),
                                    );
                                    if op.is_signed() {
                                        out += &format!("\tsetnb {}\n", output.name_x86_byte());
                                    } else {
                                        out += &format!("\tsetge {}\n", output.name_x86_byte())
                                    }
                                }
                                crate::est::BinOpKind::Greater => {
                                    out += &format!(
                                        "\tcmp {}, {}\n",
                                        left.name_x86(),
                                        right.name_x86(),
                                    );
                                    if op.is_signed() {
                                        out += &format!("\tseta {}\n", output.name_x86_byte());
                                    } else {
                                        out += &format!("\tcmovg {}\n", output.name_x86_byte());
                                    }
                                }
                            },
                        };
                    }
                    crate::asm::AsmIn::MoveConst { to, value } => {
                        out += &format!("\tmov {}, {}\n", to.name_x86(), value);
                    }
                    crate::asm::AsmIn::Move { to, from } => {
                        out += &format!("\tmov {} {}\n", to.name_x86(), from.name_x86());
                    }
                    crate::asm::AsmIn::StaticLoad {
                        to,
                        name,
                        offset,
                        is_byte,
                    } => {
                        if let Some(of) = offset.as_ref() {
                            if *is_byte {
                                out += &format!(
                                    "\tmov {}, [{}+ {}]\n",
                                    to.name_x86_byte(),
                                    name,
                                    of.get_x86_name()
                                );
                            } else {
                                out += &format!(
                                    "\tmov {}, [{}+{}]\n",
                                    to.name_arm(),
                                    name,
                                    of.get_x86_name()
                                );
                            }
                        } else {
                            if *is_byte {
                                out += &format!("\tmov {}, [{}]\n", to.name_x86_byte(), name);
                            } else {
                                out += &format!("\tmov {}, [{}]\n", to.name_x86(), name);
                            }
                        }
                    }
                    crate::asm::AsmIn::StaticStore {
                        reg,
                        name,
                        offset,
                        is_byte,
                    } => {
                        if let Some(of) = offset.as_ref() {
                            if *is_byte {
                                out += &format!(
                                    "\tmov [{}+{}], {}\n",
                                    name,
                                    of.get_x86_name(),
                                    reg.name_x86_byte(),
                                );
                            } else {
                                out += &format!(
                                    "\tmov [{}+{}],{}\n",
                                    name,
                                    of.get_x86_name(),
                                    reg.name_x86(),
                                );
                            }
                        } else {
                            if *is_byte {
                                out += &format!("\tmov [{}], {}\n", name, reg.name_x86_byte());
                            } else {
                                out += &format!("\tmov [{}], {}\n", name, reg.name_x86());
                            }
                        }
                    }
                    crate::asm::AsmIn::Load {
                        to,
                        from,
                        offset,
                        is_byte,
                    } => {
                        if let Some(of) = offset {
                            if *is_byte {
                                out += &format!(
                                    "\tmov {}, [{}+{}]\n",
                                    to.name_x86_byte(),
                                    from.name_x86(),
                                    of.get_x86_name()
                                );
                            } else {
                                out += &format!(
                                    "\tmove {}, [{}+{}]\n",
                                    to.name_x86(),
                                    from.name_x86(),
                                    of.get_x86_name()
                                );
                            }
                        } else {
                            if *is_byte {
                                out += &format!(
                                    "\tmov {}, [{}]\n",
                                    to.name_x86_byte(),
                                    from.name_x86()
                                );
                            } else {
                                out += &format!("\tmov {}, [{}]\n", to.name_x86(), from.name_x86());
                            }
                        }
                    }
                    crate::asm::AsmIn::Store {
                        to,
                        from,
                        offset,
                        is_byte,
                    } => {
                        if let Some(of) = offset {
                            if *is_byte {
                                out += &format!(
                                    "\tmov [{}+{}], {}\n",
                                    to.name_arm(),
                                    of.get_arm_name(),
                                    from.name_arm_byte(),
                                );
                            } else {
                                out += &format!(
                                    "\tmov [{}+{}] {}\n",
                                    to.name_x86(),
                                    of.get_x86_name(),
                                    from.name_x86(),
                                );
                            }
                        } else {
                            if *is_byte {
                                out += &format!(
                                    "\tmov [{}], {}\n",
                                    to.name_x86(),
                                    from.name_x86_byte(),
                                );
                            } else {
                                out +=
                                    &format!("\tmov [{}], {}\n", to.name_x86(), from.name_x86(),);
                            }
                        }
                    }
                    crate::asm::AsmIn::LoadStaticAddress { to, name, offset } => {
                        out += &format!("\tlea {}, [{}]\n", to.name_x86(), name);
                        if let Some(of) = offset.as_ref() {
                            out += &format!("\tadd {},{}\n", to.name_arm(), of.get_arm_name());
                        }
                    }
                    crate::asm::AsmIn::LoadStackAddress { to, index, offset } => {
                        out += &format!(
                            "\tmov {},rbp\n\tsub {},{}\n",
                            to.name_x86(),
                            to.name_x86(),
                            index
                        );
                        if let Some(of) = offset.as_ref() {
                            out += &format!("\tadd {},{}\n", to.name_x86(), of.get_x86_name());
                        }
                    }
                    crate::asm::AsmIn::Call { to_call } => {
                        out += &format!("\tcall {}\n", mangle_func(to_call));
                    }
                }
            }
            match &blck.end {
                crate::asm::AsmBasicBlockEnd::Branch {
                    cond,
                    if_true,
                    if_false,
                } => {
                    out += &format!("\tcmp {}, 0\n", cond.name_x86());
                    out += &format!("\tje {}_bloc_{}\n", f.name, if_true);
                    out += &format!("\tjne {}_bloc_{}\n", f.name, if_false);
                }
                crate::asm::AsmBasicBlockEnd::Goto { to } => {
                    out += &format!("\tj {}_bloc_{}\n", f.name, to);
                }
                crate::asm::AsmBasicBlockEnd::Return => {
                    out += "\tmov rsp, rbp\n";
                    out += "\tpop rbp\n";
                    out += "\tret\n";
                }
                crate::asm::AsmBasicBlockEnd::Continue => {}
            }
        }
        out += "\tmov rsp, rbp\n";
        out += "\tpop rbp\n";
        out += "\tret\n";
    }
    if has_main {
        out += ".globl main\n";
        out += "main:\n";
        out += "\tpush rbp\n";
        out += "\tmov rbp,rsp\n";
        out += "\tcall est_main\n";
        out += "\tmov rsp, rbp\n";
        out += "\tpop rbp\n";
        out += "\tret\n";
    }
    std::fs::write(file, out).unwrap();
}
