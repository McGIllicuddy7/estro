use std::collections::BTreeMap;

use crate::est::{
    BinOpKind, BinopType, Function, Literal, Operand, Static, TranslationUnit, Variable,
};

#[derive(Clone, Debug)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
}
impl Register {
    pub fn as_index(idx: usize) -> Option<Self> {
        match idx {
            0 => Some(Register::R0),
            1 => Some(Register::R1),
            2 => Some(Register::R2),
            3 => Some(Register::R3),
            4 => Some(Register::R3),
            5 => Some(Register::R5),
            6 => Some(Register::R6),
            7 => Some(Register::R7),
            8 => Some(Register::R8),
            9 => Some(Register::R9),
            10 => Some(Register::R10),
            11 => Some(Register::R11),
            _ => None,
        }
    }
    pub fn name_arm(&self) -> &'static str {
        match self {
            Self::R0 => "x0",
            Self::R1 => "x1",
            Self::R2 => "x2",
            Self::R3 => "x3",
            Self::R4 => "x4",
            Self::R5 => "x5",
            Self::R6 => "x6",
            Self::R7 => "x7",
            Self::R8 => "x8",
            Self::R9 => "x9",
            Self::R10 => "x10",
            Self::R11 => "x11",
        }
    }
    pub fn name_arm_byte(&self) -> &'static str {
        match self {
            Self::R0 => "w0",
            Self::R1 => "w1",
            Self::R2 => "w2",
            Self::R3 => "w3",
            Self::R4 => "w4",
            Self::R5 => "w5",
            Self::R6 => "w6",
            Self::R7 => "w7",
            Self::R8 => "w8",
            Self::R9 => "w9",
            Self::R10 => "w10",
            Self::R11 => "w11",
        }
    }
    pub fn name_x86(&self) -> &'static str {
        match self {
            Self::R0 => "rdi",
            Self::R1 => "rsi",
            Self::R2 => "rdx",
            Self::R3 => "rcx",
            Self::R4 => "r8",
            Self::R5 => "r9",
            Self::R6 => "r11",
            Self::R7 => "r12",
            Self::R8 => "r13",
            Self::R9 => "r14",
            Self::R10 => "rax",
            Self::R11 => "rbx",
        }
    }
    pub fn name_x86_byte(&self) -> &'static str {
        match self {
            Self::R0 => "dil",
            Self::R1 => "sil",
            Self::R2 => "dl",
            Self::R3 => "cl",
            Self::R4 => "r8b",
            Self::R5 => "r9b",
            Self::R6 => "r11b",
            Self::R7 => "r12b",
            Self::R8 => "r13b",
            Self::R9 => "r14b",
            Self::R10 => "al",
            Self::R11 => "bl",
        }
    }
}

#[derive(Clone, Debug)]
pub enum SignedOperand {
    Reg(Register),
    Op(i64),
}
impl SignedOperand {
    pub fn get_arm_name(&self) -> String {
        match self {
            Self::Op(x) => format!("{}", x),
            Self::Reg(x) => format!("{}", x.name_arm()),
        }
    }
    pub fn get_x86_name(&self) -> String {
        match self {
            Self::Op(x) => format!("{}", x),
            Self::Reg(x) => format!("{}", x.name_x86()),
        }
    }
}

#[derive(Clone, Debug)]
pub enum AsmIn {
    StackLoad {
        reg: Register,
        index: u64,
        is_byte: bool,
        offset: Option<SignedOperand>,
    },
    StackStore {
        reg: Register,
        index: u64,
        is_byte: bool,
        offset: Option<SignedOperand>,
    },
    Binop {
        op: BinopType,
        kind: BinOpKind,
        left: Register,
        right: Register,
        output: Register,
    },
    MoveConst {
        to: Register,
        value: u64,
    },
    Move {
        to: Register,
        from: Register,
    },
    StaticLoad {
        to: Register,
        name: String,
        offset: Option<SignedOperand>,
        is_byte: bool,
    },
    StaticStore {
        reg: Register,
        name: String,
        offset: Option<SignedOperand>,
        is_byte: bool,
    },
    //to = *from
    Load {
        to: Register,
        from: Register,
        offset: Option<SignedOperand>,
        is_byte: bool,
    },
    //*to = from */
    Store {
        to: Register,
        from: Register,
        offset: Option<SignedOperand>,
        is_byte: bool,
    },
    LoadStaticAddress {
        to: Register,
        name: String,
        offset: Option<SignedOperand>,
    },
    LoadStackAddress {
        to: Register,
        index: usize,
        offset: Option<SignedOperand>,
    },
    Call {
        to_call: String,
    },
}

#[derive(Clone, Debug)]
pub enum AsmBasicBlockEnd {
    Branch {
        cond: Register,
        if_true: String,
        if_false: String,
    },
    Goto {
        to: String,
    },
    Return,
    Continue,
}
#[derive(Clone, Debug)]
pub struct AsmBasicBlock {
    pub name: String,
    pub instructions: Vec<AsmIn>,
    pub end: AsmBasicBlockEnd,
}

#[derive(Clone, Debug)]
pub struct AsmFunction {
    pub name: String,
    pub variables: Vec<Variable>,
    pub inline: bool,
    pub external: bool,
    pub blocks: Vec<(String, AsmBasicBlock)>,
}

#[derive(Clone, Debug)]
pub struct AsmUnit {
    pub statics: BTreeMap<String, Static>,
    pub functions: BTreeMap<String, AsmFunction>,
}

pub fn transpile(unit: &TranslationUnit) -> AsmUnit {
    let mut statics = unit.statics.clone();
    let mut functions = BTreeMap::new();
    for (name, func) in unit.functions.iter() {
        functions.insert(name.clone(), transpile_func(name, func, &mut statics));
    }
    AsmUnit { statics, functions }
}

pub fn compile_var_load(
    register: Register,
    variable: Variable,
    offset: Option<SignedOperand>,
) -> AsmIn {
    if variable.count != 1 {
        return compile_var_addr_load(register, variable, offset);
    }
    if variable.is_static {
        AsmIn::StaticLoad {
            to: register,
            name: variable.name.clone(),
            offset: offset,
            is_byte: variable.is_byte(),
        }
    } else {
        AsmIn::StackLoad {
            reg: register,
            index: variable.stack_offset as u64,
            offset: offset,
            is_byte: variable.is_byte(),
        }
    }
}
pub fn compile_var_addr_load(
    register: Register,
    variable: Variable,
    offset: Option<SignedOperand>,
) -> AsmIn {
    if variable.is_static {
        AsmIn::LoadStaticAddress {
            to: register,
            name: variable.name.clone(),
            offset: offset,
        }
    } else {
        AsmIn::LoadStackAddress {
            to: register,
            index: variable.stack_offset,
            offset: offset,
        }
    }
}
pub fn compile_var_store(
    register: Register,
    variable: Variable,
    offset: Option<SignedOperand>,
) -> AsmIn {
    if variable.is_static {
        AsmIn::StaticStore {
            reg: register,
            name: variable.name.clone(),
            offset,
            is_byte: variable.is_byte(),
        }
    } else {
        AsmIn::StackStore {
            reg: register,
            index: variable.stack_offset as u64,
            offset,
            is_byte: variable.is_byte(),
        }
    }
}

pub fn compile_op_load(
    register: Register,
    op: Operand,
    static_table: &mut BTreeMap<String, Static>,
    offset: Option<SignedOperand>,
) -> AsmIn {
    if op.is_array() {
        let name = format!("_imm_const_{}", static_table.len());
        let stat = match op {
            Operand::Var(variable) => {
                return if variable.is_static {
                    AsmIn::LoadStaticAddress {
                        to: register,
                        name: variable.name.clone(),
                        offset,
                    }
                } else {
                    AsmIn::LoadStackAddress {
                        to: register,
                        index: variable.stack_offset as usize,
                        offset,
                    }
                };
            }
            Operand::Lit(x) => {
                let var = Variable {
                    is_static: true,
                    stack_offset: 0,
                    index: static_table.len(),
                    name: name.clone(),
                    declared_file: "_".to_string(),
                    declared_line: 0,
                    count: x.count(),
                    kind: x.kind(),
                };
                Static {
                    external: false,
                    inline: true,
                    variable: var,
                    literal: x,
                }
            }
        };
        static_table.insert(name.clone(), stat);
        return AsmIn::LoadStaticAddress {
            to: register,
            name,
            offset,
        };
    }
    match op {
        Operand::Var(variable) => {
            if variable.count != 1 {
                if variable.is_static {
                    AsmIn::LoadStaticAddress {
                        to: register,
                        name: variable.name.clone(),
                        offset,
                    }
                } else {
                    AsmIn::LoadStackAddress {
                        to: register,
                        index: variable.stack_offset as usize,
                        offset,
                    }
                }
            } else {
                if variable.is_static {
                    AsmIn::StaticLoad {
                        to: register,
                        name: variable.name.clone(),
                        offset,
                        is_byte: variable.is_byte(),
                    }
                } else {
                    AsmIn::StackLoad {
                        reg: register,
                        index: variable.stack_offset as u64,
                        offset,
                        is_byte: variable.is_byte(),
                    }
                }
            }
        }
        Operand::Lit(literal) => {
            let x = match literal {
                crate::est::Literal::String(_) => todo!(),
                crate::est::Literal::Int(x) => i64::cast_unsigned(x),
                crate::est::Literal::UInt(x) => x,
                crate::est::Literal::Float(x) => f64::to_bits(x),
                crate::est::Literal::Bool(x) => {
                    if x {
                        1
                    } else {
                        0
                    }
                }
                crate::est::Literal::Byte(x) => x as u64,
                crate::est::Literal::List(_) => todo!(),
                crate::est::Literal::ByteList(_) => todo!(),
            };
            AsmIn::MoveConst {
                to: register,
                value: x,
            }
        }
    }
}

pub fn compile_op_offset(
    instructions: &mut Vec<AsmIn>,
    op: &Option<Operand>,
    offset_reg: Register,
) -> Option<SignedOperand> {
    match op.clone()? {
        Operand::Var(variable) => {
            instructions.push(compile_var_load(offset_reg.clone(), variable, None));
            Some(SignedOperand::Reg(offset_reg))
        }
        Operand::Lit(literal) => match literal {
            crate::est::Literal::String(_) => todo!(),
            crate::est::Literal::Int(x) => Some(SignedOperand::Op(x)),
            crate::est::Literal::UInt(x) => Some(SignedOperand::Op(x as i64)),
            crate::est::Literal::Float(_) => todo!(),
            crate::est::Literal::Bool(x) => {
                if x {
                    Some(SignedOperand::Op(1))
                } else {
                    Some(SignedOperand::Op(0))
                }
            }
            crate::est::Literal::Byte(x) => Some(SignedOperand::Op(x as i64)),
            crate::est::Literal::List(_) => todo!(),
            crate::est::Literal::ByteList(_) => todo!(),
        },
    }
}

pub fn transpile_func(
    name: &str,
    func: &Function,
    statics: &mut BTreeMap<String, Static>,
) -> AsmFunction {
    let mut blocks = Vec::new();
    let names: Vec<String> = func.blocks.iter().map(|(i, _)| i.to_string()).collect();
    if !func.external {
        let mut ins = Vec::new();
        for i in 0..func.args.len() {
            ins.push(compile_var_store(
                Register::as_index(i).unwrap(),
                func.args[i].clone(),
                None,
            ));
        }
        ins.push(AsmIn::MoveConst {
            to: Register::R0,
            value: 0,
        });
        for i in func.args.len()..func.variables.len() {
            ins.push(compile_var_store(
                Register::as_index(0).unwrap(),
                func.variables[i].clone(),
                None,
            ));
        }
        blocks.push((
            "".to_string(),
            AsmBasicBlock {
                name: String::new(),
                instructions: ins,
                end: AsmBasicBlockEnd::Continue,
            },
        ));
    }

    let mut index = 0;
    for (name, bl) in &func.blocks {
        let mut ins = Vec::new();
        let end;
        for j in &bl.instructions {
            match &j.ins {
                crate::est::EstIn::Move { to, from } => {
                    ins.push(compile_op_load(Register::R0, from.clone(), statics, None));
                    ins.push(compile_var_store(Register::R0, to.clone(), None));
                }
                crate::est::EstIn::Store { to, from, offset } => {
                    let op = compile_op_offset(&mut ins, offset, Register::R1);
                    ins.push(compile_var_load(Register::R2, to.clone(), op));
                    ins.push(compile_op_load(Register::R0, from.clone(), statics, None));
                    ins.push(AsmIn::Store {
                        to: Register::R2,
                        from: Register::R0,
                        offset: None,
                        is_byte: false,
                    });
                }
                crate::est::EstIn::Load { to, from, offset } => {
                    let op = compile_op_offset(&mut ins, offset, Register::R1);
                    ins.push(compile_var_addr_load(Register::R2, from.clone(), op));
                    ins.push(AsmIn::Load {
                        to: Register::R0,
                        from: Register::R2,
                        offset: None,
                        is_byte: false,
                    });
                    ins.push(compile_var_store(Register::R0, to.clone(), None));
                }
                crate::est::EstIn::StoreByte { to, from, offset } => {
                    let op = compile_op_offset(&mut ins, offset, Register::R1);
                    ins.push(compile_var_load(Register::R0, to.clone(), op));
                    ins.push(compile_op_load(Register::R2, from.clone(), statics, None));
                    ins.push(AsmIn::Store {
                        to: Register::R0,
                        from: Register::R2,
                        offset: None,
                        is_byte: true,
                    });
                }
                crate::est::EstIn::LoadByte { to, from, offset } => {
                    let op = compile_op_offset(&mut ins, offset, Register::R1);
                    ins.push(compile_var_load(Register::R0, to.clone(), None));
                    ins.push(compile_var_load(Register::R2, from.clone(), op));
                    ins.push(AsmIn::Load {
                        to: Register::R0,
                        from: Register::R2,
                        offset: None,
                        is_byte: true,
                    });
                }

                crate::est::EstIn::LoadAddress { to, from, offset } => {
                    let op = compile_op_offset(&mut ins, offset, Register::R1);
                    if to.is_static {
                        ins.push(AsmIn::LoadStaticAddress {
                            to: Register::R2,
                            name: from.name.clone(),
                            offset: op,
                        });
                    } else {
                        ins.push(AsmIn::LoadStackAddress {
                            to: Register::R2,
                            index: from.index,
                            offset: op,
                        });
                    }
                    ins.push(AsmIn::StackStore {
                        reg: Register::R0,
                        is_byte: false,
                        index: to.index as u64,
                        offset: None,
                    });
                }
                crate::est::EstIn::BinOp {
                    kind,
                    types,
                    left,
                    right,
                    output,
                } => {
                    ins.push(compile_op_load(Register::R1, left.clone(), statics, None));
                    ins.push(compile_op_load(Register::R2, right.clone(), statics, None));
                    ins.push(AsmIn::Binop {
                        op: types.clone(),
                        kind: kind.clone(),
                        left: Register::R1,
                        right: Register::R2,
                        output: Register::R0,
                    });
                    ins.push(compile_var_store(Register::R0, output.clone(), None))
                }
                crate::est::EstIn::Call {
                    to_call,
                    arguments,
                    output,
                } => {
                    let mut idx = 0;
                    for i in arguments {
                        let rg = Register::as_index(idx).unwrap();
                        ins.push(compile_op_load(rg, i.clone(), statics, None));
                        idx += 1;
                    }
                    ins.push(AsmIn::Call {
                        to_call: to_call.clone(),
                    });
                    if let Some(op) = output {
                        ins.push(compile_var_store(Register::R0, op.clone(), None));
                    }
                }
            }
        }
        match &bl.end {
            crate::est::BasicBlocEnd::Branch {
                cond,
                if_true,
                if_false,
            } => {
                ins.push(compile_op_load(Register::R0, cond.clone(), statics, None));
                end = AsmBasicBlockEnd::Branch {
                    cond: Register::R0,
                    if_true: if_true.clone(),
                    if_false: if_false.clone(),
                };
            }
            crate::est::BasicBlocEnd::Goto { to } => {
                if let Some(nxt) = names.get(index + 1) {
                    if nxt == to {
                        end = AsmBasicBlockEnd::Continue;
                    } else {
                        end = AsmBasicBlockEnd::Goto { to: to.clone() };
                    }
                } else {
                    end = AsmBasicBlockEnd::Goto { to: to.clone() };
                }
            }
            crate::est::BasicBlocEnd::Return { to_return } => {
                if let Some(rt) = to_return {
                    ins.push(compile_op_load(Register::R0, rt.clone(), statics, None));
                }
                end = AsmBasicBlockEnd::Return;
            }
        }
        blocks.push((
            name.into(),
            (AsmBasicBlock {
                name: name.into(),
                instructions: ins,
                end,
            }),
        ));
        index += 1;
    }
    AsmFunction {
        name: name.into(),
        variables: func.variables.clone(),
        inline: func.inline,
        external: func.external,
        blocks,
    }
}
