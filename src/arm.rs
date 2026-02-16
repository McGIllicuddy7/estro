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
        if !f.inline && !f.external {
            out += &format!(".globl {}\n", mangle_func(i));
        } else if f.external {
            out += &format!(".extern {}\n", mangle_func(i));
        }
    }
    out += ".extern _memcpy\n";
    out += ".extern _memset\n";
    for (i, f) in trans.functions.iter() {
        if f.external {
            continue;
        }
        out += &format!("{}:\n", mangle_func(i));
        out += "\tsub sp, sp, #32\n";
        out += "\tstr lr,[sp, #8]\n";
        out += "\tstr fp,[sp, #16]\n";
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
        /*    out += &format!("\tmov x0,fp\n");
        out += &format!("\tmov x2,0\n");
        out += &format!("\tmov x3,{}\n", sz);
        out += &format!("\tbl _memset\n");*/
        for (name, blck) in &f.blocks {
            if name != "" {
                out += &format!("{}:\n", name);
            }
            for is in &blck.instructions {
                out += &format!("\t;{:#?}", is)
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
                                    "\tldrb {}, [fp, -{}, {}]\n",
                                    reg.name_arm_byte(),
                                    index,
                                    of.get_arm_name()
                                );
                            } else {
                                out += &format!(
                                    "\tldr {}, [fp, -{}, {}]\n",
                                    reg.name_arm(),
                                    index,
                                    of.get_arm_name()
                                );
                            }
                        } else {
                            if *is_byte {
                                out +=
                                    &format!("\tldrb {}, [fp, -{}]\n", reg.name_arm_byte(), index);
                            } else {
                                out += &format!("\tldr {}, [fp, -{}]\n", reg.name_arm(), index);
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
                                    "\tstrb {}, [fp, -{}, {}]\n",
                                    reg.name_arm_byte(),
                                    index,
                                    of.get_arm_name()
                                );
                            } else {
                                out += &format!(
                                    "\tstr {}, [fp, -{},{}]\n",
                                    reg.name_arm(),
                                    index,
                                    of.get_arm_name()
                                );
                            }
                        } else {
                            if *is_byte {
                                out +=
                                    &format!("\tstrb {}, [fp, -{}]\n", reg.name_arm_byte(), index);
                            } else {
                                out += &format!("\tstr {}, [fp, -{}]\n", reg.name_arm(), index);
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
                                out += &format!("\tfmov d1, {}\n", left.name_arm());
                                out += &format!("\tfmov d2, {}\n", right.name_arm());
                                match kind {
                                    crate::est::BinOpKind::Add => {
                                        out += &format!("\tfadd d0, d1, d2\n");
                                        out += &format!("\tfmov {}, d0", output.name_arm());
                                    }
                                    crate::est::BinOpKind::Sub => {
                                        out += &format!("\tfsub d0, d1, d2\n");
                                        out += &format!("\tfmov {}, d0", output.name_arm());
                                    }
                                    crate::est::BinOpKind::Div => {
                                        out += &format!("\tfdiv d0, d1, d2\n");
                                        out += &format!("\tfmov {}, d0", output.name_arm());
                                    }
                                    crate::est::BinOpKind::Mul => {
                                        out += &format!("\tfmul d0, d1, d2\n");
                                        out += &format!("\tfmov {}, d0", output.name_arm());
                                    }
                                    crate::est::BinOpKind::Less => {
                                        out += &format!("\tfcmp,d1, d2\n");
                                        out += &format!("\tcset {}, lt", output.name_arm());
                                    }
                                    crate::est::BinOpKind::LessEq => {
                                        out += &format!("\tfcmp,d1, d2\n");
                                        out += &format!("\tcset {}, le", output.name_arm());
                                    }
                                    crate::est::BinOpKind::Eq => {
                                        out += &format!("\tfcmp,d1, d2\n");
                                        out += &format!("\tcset {}, eq", output.name_arm());
                                    }
                                    crate::est::BinOpKind::Neq => {
                                        out += &format!("\tfcmp,d1, d2\n");
                                        out += &format!("\tcset {}, ne", output.name_arm());
                                    }
                                    crate::est::BinOpKind::GreaterEq => {
                                        out += &format!("\tfcmp,d1, d2\n");
                                        out += &format!("\tcset {}, ge", output.name_arm());
                                    }
                                    crate::est::BinOpKind::Greater => {
                                        out += &format!("\tfcmp,d1, d2\n");
                                        out += &format!("\tcset {}, gt", output.name_arm());
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
                                        "\tadd {},{},{}\n",
                                        output.name_arm(),
                                        left.name_arm(),
                                        right.name_arm()
                                    );
                                }
                                crate::est::BinOpKind::Sub => {
                                    out += &format!(
                                        "\tsub {},{},{}\n",
                                        output.name_arm(),
                                        left.name_arm(),
                                        right.name_arm()
                                    );
                                }
                                crate::est::BinOpKind::Div => {
                                    out += &format!(
                                        "\tdiv {},{},{}\n",
                                        output.name_arm(),
                                        left.name_arm(),
                                        right.name_arm()
                                    );
                                }
                                crate::est::BinOpKind::Mul => {
                                    out += &format!(
                                        "\tmul {},{},{}\n",
                                        output.name_arm(),
                                        left.name_arm(),
                                        right.name_arm()
                                    );
                                }
                                crate::est::BinOpKind::Rem => {
                                    todo!();
                                }
                                crate::est::BinOpKind::Less => {
                                    out += &format!(
                                        "\tcmp {}, {}\n",
                                        left.name_arm(),
                                        right.name_arm()
                                    );
                                    if op.is_signed() {
                                        out += &format!("\tcset {},lt\n", output.name_arm());
                                    } else {
                                        out += &format!("\tcset {},li\n", output.name_arm());
                                    }
                                }
                                crate::est::BinOpKind::LessEq => {
                                    out += &format!(
                                        "\tcmp {}, {}\n",
                                        left.name_arm(),
                                        right.name_arm()
                                    );
                                    if op.is_signed() {
                                        out += &format!("\tcset {},le\n", output.name_arm());
                                    } else {
                                        out += &format!("\tcset {}, ls\n", output.name_arm());
                                    }
                                }
                                crate::est::BinOpKind::Eq => {
                                    out += &format!(
                                        "\tcmp {}, {}\n",
                                        left.name_arm(),
                                        right.name_arm()
                                    );
                                    out += &format!("\tcset {}, eq\n", output.name_arm());
                                }
                                crate::est::BinOpKind::Neq => {
                                    out += &format!(
                                        "\tcmp {}, {}\n",
                                        left.name_arm(),
                                        right.name_arm()
                                    );
                                    out += &format!("\tcset {}, ne\n", output.name_arm());
                                }
                                crate::est::BinOpKind::GreaterEq => {
                                    out += &format!(
                                        "\tcmp {}, {}\n",
                                        left.name_arm(),
                                        right.name_arm()
                                    );
                                    if op.is_signed() {
                                        out += &format!("\tcset {}, ge\n", output.name_arm());
                                    } else {
                                        out += &format!("\tcset {}, hs\n", output.name_arm());
                                    }
                                }
                                crate::est::BinOpKind::Greater => {
                                    out += &format!(
                                        "\tcmp {}, {}\n",
                                        left.name_arm(),
                                        right.name_arm()
                                    );
                                    if op.is_signed() {
                                        out += &format!("\tcset {}, gt\n", output.name_arm());
                                    } else {
                                        out += &format!("\tcset {}, hi\n", output.name_arm());
                                    }
                                }
                            },
                        };
                    }
                    crate::asm::AsmIn::MoveConst { to, value } => {
                        out += &format!("\tmov {}, #{}\n", to.name_arm(), value);
                    }
                    crate::asm::AsmIn::Move { to, from } => {
                        out += &format!("\tmov {} {}\n", to.name_arm(), from.name_arm());
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
                                    "\tldrb {}, [{}, {}]\n",
                                    to.name_arm_byte(),
                                    name,
                                    of.get_arm_name()
                                );
                            } else {
                                out += &format!(
                                    "\tldr {}, [{},{}]\n",
                                    to.name_arm(),
                                    name,
                                    of.get_arm_name()
                                );
                            }
                        } else {
                            if *is_byte {
                                out += &format!("\tldrb {}, [{}]\n", to.name_arm_byte(), name);
                            } else {
                                out += &format!("\tldr {}, [{}]\n", to.name_arm(), name);
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
                                    "\tstrb {}, [{}, {}]\n",
                                    reg.name_arm_byte(),
                                    name,
                                    of.get_arm_name()
                                );
                            } else {
                                out += &format!(
                                    "\tstr {}, [{},{}]\n",
                                    reg.name_arm(),
                                    name,
                                    of.get_arm_name()
                                );
                            }
                        } else {
                            if *is_byte {
                                out += &format!("\tstrb {}, [{}]\n", reg.name_arm_byte(), name);
                            } else {
                                out += &format!("\tstr {}, [{}]\n", reg.name_arm(), name);
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
                                    "\tldrb {}, [{},{}]\n",
                                    to.name_arm_byte(),
                                    from.name_arm(),
                                    of.get_arm_name()
                                );
                            } else {
                                out += &format!(
                                    "\tldr {}, [{},{}]\n",
                                    to.name_arm(),
                                    from.name_arm(),
                                    of.get_arm_name()
                                );
                            }
                        } else {
                            if *is_byte {
                                out += &format!(
                                    "\tldrb {}, [{}]\n",
                                    to.name_arm_byte(),
                                    from.name_arm()
                                );
                            } else {
                                out += &format!("\tldr {}, [{}]\n", to.name_arm(), from.name_arm());
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
                                    "\tstrb {}, [{},{}]\n",
                                    from.name_arm_byte(),
                                    to.name_arm(),
                                    of.get_arm_name()
                                );
                            } else {
                                out += &format!(
                                    "\tstr {}, [{},{}]\n",
                                    from.name_arm(),
                                    to.name_arm(),
                                    of.get_arm_name()
                                );
                            }
                        } else {
                            if *is_byte {
                                out += &format!(
                                    "\tstrb {}, [{}]\n",
                                    from.name_arm_byte(),
                                    to.name_arm(),
                                );
                            } else {
                                out +=
                                    &format!("\tstr {}, [{}]\n", from.name_arm(), to.name_arm(),);
                            }
                        }
                    }
                    crate::asm::AsmIn::LoadStaticAddress { to, name, offset } => {
                        out += &format!("\tldr {}, {}\n", to.name_arm(), name);
                        if let Some(of) = offset.as_ref() {
                            out += &format!(
                                "\tadd {},{},{}\n",
                                to.name_arm(),
                                to.name_arm(),
                                of.get_arm_name()
                            );
                        }
                    }
                    crate::asm::AsmIn::LoadStackAddress { to, index, offset } => {
                        out += &format!("\tsub {},fp,{}\n", to.name_arm(), index);
                        if let Some(of) = offset.as_ref() {
                            out += &format!(
                                "\tadd {},{},{}\n",
                                to.name_arm(),
                                to.name_arm(),
                                of.get_arm_name()
                            );
                        }
                    }
                    crate::asm::AsmIn::Call { to_call } => {
                        out += &format!("\tbl {}\n", mangle_func(to_call));
                    }
                }
            }
            match &blck.end {
                crate::asm::AsmBasicBlockEnd::Branch {
                    cond,
                    if_true,
                    if_false,
                } => {
                    out += &format!("\tcmp {}, #0\n", cond.name_arm());
                    out += &format!("\tbne {}_bloc_{}\n", f.name, if_true);
                    out += &format!("\tbeq {}_bloc_{}\n", f.name, if_false);
                }
                crate::asm::AsmBasicBlockEnd::Goto { to } => {
                    out += &format!("\tb {}_bloc_{}\n", f.name, to);
                }
                crate::asm::AsmBasicBlockEnd::Return => {
                    out += "\tmov sp, fp\n";
                    out += "\tldr lr,[sp, #8]\n";
                    out += "\tldr fp,[sp, #16]\n";
                    out += "\tadd sp, sp,#32\n";
                    out += "\tbr lr\n";
                }
                crate::asm::AsmBasicBlockEnd::Continue => {}
            }
        }
        out += "\tmov sp, fp\n";
        out += "\tldr lr,[sp, #8]\n";
        out += "\tldr fp,[sp, #16]\n";
        out += "\tadd sp, sp,#32\n";
        out += "\tbr lr\n";
    }
    if has_main {
        out += ".globl _main\n";
        out += "_main:\n";
        out += "\tsub sp, sp, #32\n";
        out += "\tstr lr,[sp, #8]\n";
        out += "\tstr fp,[sp, #16]\n";
        out += "\tmov fp, sp\n";
        out += "\tbl _est_main\n";
        out += "\tmov sp, fp\n";
        out += "\tldr lr,[fp, #8]\n";
        out += "\tldr fp,[fp, #16]\n";
        out += "\tadd sp, sp,#32\n";
        out += "\tbr lr\n";
    }
    std::fs::write(file, out).unwrap();
}
