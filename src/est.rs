use crate::rtils::rtils_useful::{Throw, Throws};
pub use crate::rtils::rtils_useful::{Token, TokenStream, tokenize};
pub use std::collections::{BTreeMap, HashSet};
#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Ord, Eq)]
pub enum Type {
    Word,
    Byte,
}
impl Type {
    pub fn c_name(&self) -> &'static str {
        match self {
            Self::Byte => "EstroByte",
            Self::Word => "EstroWord",
        }
    }
}
#[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Ord, Eq)]
pub struct Variable {
    pub is_static: bool,
    pub stack_offset: usize,
    pub index: usize,
    pub name: String,
    pub declared_file: String,
    pub declared_line: usize,
    pub kind: Type,
    pub count: usize,
}
impl Variable {
    pub fn is_byte(&self) -> bool {
        match self.kind {
            Type::Byte => true,
            Type::Word => false,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Literal {
    String(String),
    Int(i64),
    UInt(u64),
    Float(f64),
    Bool(bool),
    Byte(u8),
    List(Vec<Literal>),
    ByteList(Vec<Literal>),
}

impl Literal {
    pub fn c_fmt(&self) -> String {
        match self {
            Literal::Bool(f) => {
                if *f {
                    "1".to_string()
                } else {
                    "0".to_string()
                }
            }
            Literal::Float(x) => format!("(EstroWord){{.db = {}}}", f64::to_bits(*x)),
            Literal::Int(x) => format!("(EstroWord){{.sn = {}}}", *x),
            Literal::String(x) => format!("(EstroWord){{.str = (EstroByte*){}}}", x),
            Literal::UInt(x) => format!("(EstroWord){{.un = {}}}", x),
            Literal::Byte(x) => format!("(EstroWord){{.un = {}}}", x),
            Literal::List(list) => {
                let mut out = format!("(EstroWord[]){{");
                for i in 0..list.len() {
                    out += &list[i].c_fmt();
                    if i != list.len() - 1 {
                        out += ","
                    }
                }
                out = format!("(EstroWord){{.ptr = &{}}}", out);
                out
            }
            Literal::ByteList(list) => {
                let mut out = format!("(EstroByte[]){{");
                for i in 0..list.len() {
                    out += &list[i].c_fmt();
                    if i != list.len() - 1 {
                        out += ","
                    }
                }
                out = format!("(EstroWord){{.byte_ptr= &{}}}", out);
                out
            }
        }
    }
    pub fn count(&self) -> usize {
        match self {
            Literal::String(x) => x.len(),
            Literal::Int(_) => 1,
            Literal::UInt(_) => 1,
            Literal::Float(_) => 1,
            Literal::Bool(_) => 1,
            Literal::Byte(_) => 1,
            Literal::List(literals) => literals.len(),
            Literal::ByteList(literals) => literals.len(),
        }
    }
    pub fn kind(&self) -> Type {
        match self {
            Literal::String(_) => Type::Byte,
            Literal::Int(_) => Type::Word,
            Literal::UInt(_) => Type::Word,
            Literal::Float(_) => Type::Word,
            Literal::Bool(_) => Type::Byte,
            Literal::Byte(_) => Type::Byte,
            Literal::List(_) => Type::Word,
            Literal::ByteList(_) => Type::Byte,
        }
    }
}
#[derive(Clone, Debug)]
pub enum Operand {
    Var(Variable),
    Lit(Literal),
}

impl Operand {
    pub fn can_be_byte(&self) -> bool {
        match self {
            Self::Lit(x) => match x {
                Literal::Bool(_) => true,
                Literal::Float(_) => false,
                Literal::Int(x) => i64::cast_unsigned(*x) < 256,
                Literal::String(_) => false,
                Literal::UInt(x) => *x < 256,
                Literal::Byte(_) => true,
                Literal::List(_) => false,
                Literal::ByteList(_) => false,
            },
            Self::Var(v) => match v.kind {
                Type::Word => false,
                Type::Byte => true,
            },
        }
    }

    pub fn c_fmt(&self) -> String {
        match self {
            Self::Lit(x) => match x {
                Literal::Bool(f) => {
                    if *f {
                        "1".to_string()
                    } else {
                        "0".to_string()
                    }
                }
                Literal::Float(x) => format!("(EstroWord){{.db = {}}}", x),
                Literal::Int(x) => format!("(EstroWord){{.sn = {}}}", *x),
                Literal::String(x) => format!("(EstroWord){{.str = (EstroByte*){}}}", x),
                Literal::UInt(x) => format!("(EstroWord){{.un = {}}}", x),
                Literal::Byte(x) => format!("(EstroWord){{.un = {}}}", x),
                Literal::List(list) => {
                    let mut out = format!("(EstroWord[]){{");
                    for i in 0..list.len() {
                        out += &list[i].c_fmt();
                        if i != list.len() - 1 {
                            out += ",";
                        }
                    }
                    out = format!("(EstroWord){{.ptr = &{}}}", out);
                    out
                }
                Literal::ByteList(list) => {
                    let mut out = format!("(EstroByte[]){{");
                    for i in 0..list.len() {
                        out += &list[i].c_fmt();
                        if i != list.len() - 1 {
                            out += ","
                        }
                    }
                    out = format!("(EstroWord){{.byte_ptr= &{}}}", out);
                    out
                }
            },
            Self::Var(v) => {
                format!("{}", v.name)
            }
        }
    }
    pub fn is_lit(&self) -> bool {
        match self {
            Self::Lit(_) => true,
            Self::Var(_) => false,
        }
    }
    pub fn is_array(&self) -> bool {
        match self {
            Operand::Var(x) => x.count != 1,
            Operand::Lit(literal) => match literal {
                Literal::String(_) => true,
                Literal::Int(_) => false,
                Literal::UInt(_) => false,
                Literal::Float(_) => false,
                Literal::Bool(_) => false,
                Literal::List(_) => true,
                Literal::Byte(_) => false,
                Literal::ByteList(_) => true,
            },
        }
    }
}

#[derive(Clone, Debug)]
pub enum BinOpKind {
    Add,
    Sub,
    Div,
    Mul,
    Rem,
    Less,
    LessEq,
    Eq,
    Neq,
    GreaterEq,
    Greater,
}
#[derive(Clone, Debug)]
pub enum BinopType {
    Float,
    IByte,
    Byte,
    IWord,
    Word,
}
impl BinopType {
    pub fn is_signed(&self) -> bool {
        match self {
            BinopType::Float => true,
            BinopType::IByte => true,
            BinopType::Byte => false,
            BinopType::IWord => true,
            BinopType::Word => false,
        }
    }
}

#[derive(Clone, Debug)]
pub enum EstIn {
    Move {
        to: Variable,
        from: Operand,
    },
    Store {
        to: Variable,
        from: Operand,
        offset: Option<Operand>,
    },
    StoreByte {
        to: Variable,
        from: Operand,
        offset: Option<Operand>,
    },

    Load {
        to: Variable,
        from: Variable,
        offset: Option<Operand>,
    },
    LoadByte {
        to: Variable,
        from: Variable,
        offset: Option<Operand>,
    },

    LoadAddress {
        to: Variable,
        from: Variable,
        offset: Option<Operand>,
    },
    BinOp {
        kind: BinOpKind,
        types: BinopType,
        left: Operand,
        right: Operand,
        output: Variable,
    },
    Call {
        to_call: String,
        arguments: Vec<Operand>,
        output: Option<Variable>,
    },
}

#[derive(Clone, Debug)]
pub struct EstInstr {
    pub ins: EstIn,
    pub line: usize,
    pub file: String,
}

#[derive(Clone, Debug)]
pub enum BasicBlocEnd {
    Branch {
        cond: Operand,
        if_true: String,
        if_false: String,
    },
    Goto {
        to: String,
    },
    Return {
        to_return: Option<Operand>,
    },
}

#[derive(Clone, Debug)]
pub struct BasicBloc {
    pub instructions: Vec<EstInstr>,
    pub end: BasicBlocEnd,
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub args: Vec<Variable>,
    pub variables: Vec<Variable>,
    pub returns: Option<Type>,
    pub blocks: Vec<(String, BasicBloc)>,
    pub inline: bool,
    pub external: bool,
}

#[derive(Clone, Debug)]
pub struct Static {
    pub variable: Variable,
    pub literal: Literal,
    pub inline: bool,
    pub external: bool,
}

#[derive(Clone, Debug)]
pub struct TranslationUnit {
    pub statics: BTreeMap<String, Static>,
    pub functions: BTreeMap<String, Function>,
}

pub fn parse_static_literal(stream: &mut TokenStream) -> Throws<Literal> {
    if let Some(t) = stream.peek() {
        if t.text == "(" {
            let mut bytes = false;
            let mut list = Vec::new();
            let mut x = extract_between_parens(stream)?;
            if let Some(t) = x.peek() {
                if t.text == "byte" {
                    bytes = true;
                }
                _ = x.next();
            }
            while let Some(_) = x.peek() {
                list.push(parse_static_literal(&mut x)?);
            }
            if bytes {
                return Ok(Literal::ByteList(list));
            } else {
                return Ok(Literal::List(list));
            }
        }
    }
    let mut minus = 1;
    let mut t = stream.next().throw()?;
    if t.as_ref() == "-" {
        minus = -1;
        t = stream.next().throw()?;
    }
    if t.as_ref().starts_with('"') {
        let s = t.as_ref().strip_prefix('"').throw()?;
        let s2 = s.strip_suffix('"').throw()?;
        Ok(Literal::String(s2.into()))
    } else if let Ok(x) = t.as_ref().parse::<i64>() {
        Ok(Literal::Int(x * minus))
    } else if let Ok(x) = t.as_ref().parse::<u64>() {
        Ok(Literal::UInt(x))
    } else if let Ok(x) = t.as_ref().parse::<f64>() {
        Ok(Literal::Float(x * minus as f64))
    } else if let Ok(x) = t.as_ref().parse::<bool>() {
        Ok(Literal::Bool(x))
    } else {
        println!("{:#?}", t);
        todo!()
    }
}

pub fn parse_static(
    is_inline: bool,
    is_external: bool,
    stream: &mut TokenStream,
) -> Throws<(String, Static)> {
    let var = parse_var_dec(&[], true, stream)?;
    let name = var.name.clone();
    if is_external {
        return Ok((
            name,
            Static {
                variable: var,
                literal: Literal::UInt(0),
                inline: is_inline,
                external: is_external,
            },
        ));
    }
    let lit = parse_op(&[], &BTreeMap::new(), stream)?;
    match lit {
        Operand::Var(_) => todo!(),
        Operand::Lit(x) => {
            return Ok((
                name,
                Static {
                    variable: var,
                    literal: x,
                    inline: is_inline,
                    external: is_external,
                },
            ));
        }
    }
}

pub fn extract_between_parens(stream: &mut TokenStream) -> Throws<TokenStream> {
    let mut out = TokenStream {
        tokens: Vec::new(),
        index: 0,
    };
    let mut paren_count = 1;
    let p = stream.next().throw()?;
    if p.as_ref() != "(" {
        println!("{}", p.as_ref());
        todo!()
    }
    for n in stream.by_ref() {
        if n.as_ref() == "(" {
            paren_count += 1;
        } else if n.as_ref() == ")" {
            paren_count -= 1;
            if paren_count == 0 {
                break;
            }
        }
        out.tokens.push(n);
    }
    if paren_count != 0 {
        todo!()
    }
    Ok(out)
}

pub fn parse_var_dec(
    current: &[Variable],
    is_static: bool,
    stream: &mut TokenStream,
) -> Throws<Variable> {
    let ty = match stream.next().throw()?.as_ref() {
        "byte" => Type::Byte,
        "word" => Type::Word,
        _ => {
            todo!()
        }
    };
    let mut name = stream.next().throw()?;
    if is_static {
        name.text = "est_".to_string() + &name.text;
    }
    let mut count = 1;
    let trys = name.text.strip_prefix("[");
    if let Some(t) = trys {
        let t2 = t.strip_suffix("]").throw()?.to_owned();
        name = stream.next().throw()?;
        count = t2.parse()?;
    }
    let mut offset = 0;
    if let Some(l) = current.last() {
        let sz = match ty {
            Type::Word => 8,
            Type::Byte => 1,
        };
        offset = l.stack_offset + sz * l.count;
        if offset % sz != 0 {
            offset += sz - offset % sz;
        }
    }
    Ok(Variable {
        is_static,
        stack_offset: offset,
        index: current.len(),
        name: name.text,
        declared_file: name.file,
        declared_line: name.line,
        kind: ty,
        count,
    })
}

pub fn parse_function(
    is_inline: bool,
    is_extern: bool,
    stream: &mut TokenStream,
    globals: &BTreeMap<String, Static>,
) -> Throws<(String, Function)> {
    let mut args = Vec::new();
    let return_type_s = stream.next().throw()?;
    let rt = match return_type_s.as_ref() {
        "byte" => Some(Type::Byte),
        "word" => Some(Type::Word),
        "none" => None,
        _ => {
            todo!()
        }
    };
    let name = stream.next().throw()?;
    let mut argx = extract_between_parens(stream)?;
    while argx.peek().is_some() {
        let ag = parse_var_dec(&args, false, &mut argx)?;
        args.push(ag);
    }
    if is_extern {
        return Ok((
            name.text.to_string(),
            Function {
                name: name.text.to_string(),
                args: args,
                external: true,
                returns: rt,
                blocks: Vec::new(),
                inline: is_inline,
                variables: Vec::new(),
            },
        ));
    }
    let mut bloc_tokens = extract_between_parens(stream)?;
    let mut blocs = Vec::new();
    let mut variables = args.clone();
    while let Some(x) = bloc_tokens.peek() {
        let bloc_name = format!("{}_bloc_{}", name.as_ref(), x.as_ref());
        let _ = bloc_tokens.next();
        let mut blc_tokens = extract_between_parens(&mut bloc_tokens)?;
        let block = parse_bloc(
            x.as_ref().to_string(),
            &mut variables,
            globals,
            &mut blc_tokens,
        )?;
        blocs.push((bloc_name, block));
    }
    let vars = variables[args.len()..].to_vec();
    Ok((
        name.text.to_string(),
        Function {
            name: name.text.to_string(),
            args,
            external: false,
            returns: rt,
            blocks: blocs,
            inline: is_inline,
            variables: vars,
        },
    ))
}

pub fn parse_var(
    vars: &[Variable],
    globals: &BTreeMap<String, Static>,
    stream: &mut TokenStream,
) -> Throws<Variable> {
    let t = stream.next().throw()?;
    for i in vars {
        if i.name == t.as_ref() {
            return Ok(i.clone());
        }
    }
    if let Some(var) = globals.get(t.as_ref()) {
        return Ok(var.variable.clone());
    }
    println!("use of undeclared var:{:#?}", t);
    todo!()
}

pub fn parse_op(
    vars: &[Variable],
    globals: &BTreeMap<String, Static>,
    stream: &mut TokenStream,
) -> Throws<Operand> {
    let mut minus = 1;
    let mut t = stream.next().throw()?;
    if t.as_ref() == "-" {
        minus = -1;
        t = stream.next().throw()?;
    }
    for i in vars {
        if i.name == t.as_ref() {
            return Ok(Operand::Var(i.clone()));
        }
    }
    if let Some(var) = globals.get(t.as_ref()) {
        return Ok(Operand::Var(var.variable.clone()));
    }
    if t.as_ref().starts_with('"') {
        let s = t.as_ref().strip_prefix('"').throw()?;
        let s2 = s.strip_suffix('"').throw()?;
        Ok(Operand::Lit(Literal::String(s2.into())))
    } else if let Ok(x) = t.as_ref().parse::<i64>() {
        Ok(Operand::Lit(Literal::Int(x * minus)))
    } else if let Ok(x) = t.as_ref().parse::<u64>() {
        Ok(Operand::Lit(Literal::UInt(x)))
    } else if let Ok(x) = t.as_ref().parse::<f64>() {
        Ok(Operand::Lit(Literal::Float(x * minus as f64)))
    } else if let Ok(x) = t.as_ref().parse::<bool>() {
        Ok(Operand::Lit(Literal::Bool(x)))
    } else {
        println!("{:#?}", t);
        todo!()
    }
}

pub fn parse_bloc(
    name: String,
    variables: &mut Vec<Variable>,
    globals: &BTreeMap<String, Static>,
    tokens: &mut TokenStream,
) -> Throws<BasicBloc> {
    let mut ins = Vec::new();
    let mut end = BasicBlocEnd::Goto { to: name };
    while tokens.peek().is_some() {
        let mut cmd_toks = extract_between_parens(tokens)?;
        let continues = tokens.peek().is_some();
        if continues {
            let cmd = cmd_toks.next().throw()?;
            match cmd.as_ref() {
                "+" | "-" | "*" | "/" | "%" | "eq" | "ne" | "lt" | "le" | "ge" | "gt" => {
                    let tk = cmd_toks.peek().throw()?;
                    let binop = if cmd.as_ref() == "+" {
                        BinOpKind::Add
                    } else if cmd.as_ref() == "-" {
                        BinOpKind::Sub
                    } else if cmd.as_ref() == "*" {
                        BinOpKind::Mul
                    } else if cmd.as_ref() == "/" {
                        BinOpKind::Div
                    } else if cmd.as_ref() == "%" {
                        BinOpKind::Rem
                    } else if cmd.as_ref() == "eq" {
                        BinOpKind::Eq
                    } else if cmd.as_ref() == "ne" {
                        BinOpKind::Neq
                    } else if cmd.as_ref() == "lt" {
                        BinOpKind::Less
                    } else if cmd.as_ref() == "le" {
                        BinOpKind::LessEq
                    } else if cmd.as_ref() == "ge" {
                        BinOpKind::GreaterEq
                    } else if cmd.as_ref() == "gt" {
                        BinOpKind::Greater
                    } else {
                        todo!()
                    };
                    let out = parse_var(variables, globals, &mut cmd_toks)?;
                    let left = parse_op(variables, globals, &mut cmd_toks)?;
                    let right = parse_op(variables, globals, &mut cmd_toks)?;
                    if out.count != 1 || left.is_array() || right.is_array() {
                        println!(
                            "error file:{}, line:{}, cannot assign arrays",
                            tk.file, tk.line
                        );
                        todo!()
                    }
                    let typs = match out.kind {
                        Type::Byte => BinopType::IByte,
                        Type::Word => BinopType::IWord,
                    };
                    ins.push(EstInstr {
                        ins: EstIn::BinOp {
                            kind: binop,
                            types: typs,
                            left,
                            right,
                            output: out,
                        },
                        line: cmd.line,
                        file: cmd.file,
                    });
                }
                "uadd" | "usub" | "umul" | "udiv" | "urem" | "ueq" | "une" | "ult" | "ule"
                | "uge" | "ugt" => {
                    let tk = cmd_toks.peek().throw()?;
                    let binop = if cmd.as_ref() == "uadd" {
                        BinOpKind::Add
                    } else if cmd.as_ref() == "usub" {
                        BinOpKind::Sub
                    } else if cmd.as_ref() == "umul" {
                        BinOpKind::Mul
                    } else if cmd.as_ref() == "udiv" {
                        BinOpKind::Div
                    } else if cmd.as_ref() == "urem" {
                        BinOpKind::Rem
                    } else if cmd.as_ref() == "ueq" {
                        BinOpKind::Eq
                    } else if cmd.as_ref() == "une" {
                        BinOpKind::Neq
                    } else if cmd.as_ref() == "ult" {
                        BinOpKind::Less
                    } else if cmd.as_ref() == "ule" {
                        BinOpKind::LessEq
                    } else if cmd.as_ref() == "uge" {
                        BinOpKind::GreaterEq
                    } else if cmd.as_ref() == "ugt" {
                        BinOpKind::Greater
                    } else {
                        todo!()
                    };
                    let out = parse_var(variables, globals, &mut cmd_toks)?;
                    let left = parse_op(variables, globals, &mut cmd_toks)?;
                    let right = parse_op(variables, globals, &mut cmd_toks)?;
                    if out.count != 1 || left.is_array() || right.is_array() {
                        println!(
                            "error file:{}, line:{}, cannot assign arrays",
                            tk.file, tk.line
                        );
                        todo!()
                    }
                    let typs = match out.kind {
                        Type::Byte => BinopType::Byte,
                        Type::Word => BinopType::Word,
                    };
                    ins.push(EstInstr {
                        ins: EstIn::BinOp {
                            kind: binop,
                            types: typs,
                            left,
                            right,
                            output: out,
                        },
                        line: cmd.line,
                        file: cmd.file,
                    });
                }
                "fadd" | "fsub" | "fmul" | "fdiv" | "feq" | "fne" | "flt" | "fle" | "fge"
                | "fgt =>" => {
                    let tk = cmd_toks.peek().throw()?;
                    let binop = if cmd.as_ref() == "fadd" {
                        BinOpKind::Add
                    } else if cmd.as_ref() == "fsub" {
                        BinOpKind::Sub
                    } else if cmd.as_ref() == "fmul" {
                        BinOpKind::Mul
                    } else if cmd.as_ref() == "fdiv" {
                        BinOpKind::Div
                    } else if cmd.as_ref() == "feq" {
                        BinOpKind::Eq
                    } else if cmd.as_ref() == "fne" {
                        BinOpKind::Neq
                    } else if cmd.as_ref() == "flt" {
                        BinOpKind::Less
                    } else if cmd.as_ref() == "fle" {
                        BinOpKind::LessEq
                    } else if cmd.as_ref() == "fge" {
                        BinOpKind::GreaterEq
                    } else if cmd.as_ref() == "fgt" {
                        BinOpKind::Greater
                    } else {
                        todo!()
                    };
                    let out = parse_var(variables, globals, &mut cmd_toks)?;
                    let left = parse_op(variables, globals, &mut cmd_toks)?;
                    let right = parse_op(variables, globals, &mut cmd_toks)?;
                    if out.count != 1 || left.is_array() || right.is_array() {
                        println!(
                            "error file:{}, line:{}, cannot assign arrays",
                            tk.file, tk.line
                        );
                        todo!()
                    }
                    let typs = match out.kind {
                        Type::Byte => {
                            todo!()
                        }
                        Type::Word => BinopType::Float,
                    };
                    ins.push(EstInstr {
                        ins: EstIn::BinOp {
                            kind: binop,
                            types: typs,
                            left,
                            right,
                            output: out,
                        },
                        line: cmd.line,
                        file: cmd.file,
                    });
                }
                "let" => {
                    let v = parse_var_dec(variables, false, &mut cmd_toks)?;
                    if cmd_toks.next().is_some() {
                        todo!()
                    }
                    variables.push(v);
                }
                "=" => {
                    let tk = cmd_toks.peek().throw()?;
                    let left = parse_var(variables, globals, &mut cmd_toks)?;
                    let right = parse_op(variables, globals, &mut cmd_toks)?;
                    if left.count != 1 || right.is_array() {
                        println!(
                            "error file:{}, line:{}, cannot assign arrays",
                            tk.file, tk.line
                        );
                        todo!()
                    }
                    ins.push(EstInstr {
                        ins: EstIn::Move {
                            to: left,
                            from: right,
                        },
                        line: cmd.line,
                        file: cmd.file,
                    });
                }
                "store" => {
                    let left = parse_var(variables, globals, &mut cmd_toks)?;
                    let right = parse_op(variables, globals, &mut cmd_toks)?;
                    let off = if let Some(_op) = cmd_toks.peek() {
                        Some(parse_op(variables, globals, &mut cmd_toks)?)
                    } else {
                        None
                    };
                    ins.push(EstInstr {
                        ins: EstIn::Store {
                            to: left,
                            from: right,
                            offset: off,
                        },
                        line: cmd.line,
                        file: cmd.file,
                    });
                }
                "storeb" => {
                    let left = parse_var(variables, globals, &mut cmd_toks)?;
                    let right = parse_op(variables, globals, &mut cmd_toks)?;
                    let off = if let Some(_op) = cmd_toks.peek() {
                        Some(parse_op(variables, globals, &mut cmd_toks)?)
                    } else {
                        None
                    };
                    ins.push(EstInstr {
                        ins: EstIn::StoreByte {
                            to: left,
                            from: right,
                            offset: off,
                        },
                        line: cmd.line,
                        file: cmd.file,
                    });
                }

                "load" => {
                    let left = parse_var(variables, globals, &mut cmd_toks)?;
                    let right = parse_var(variables, globals, &mut cmd_toks)?;
                    let off = if cmd_toks.peek().is_some() {
                        Some(parse_op(variables, globals, &mut cmd_toks)?)
                    } else {
                        None
                    };
                    ins.push(EstInstr {
                        ins: EstIn::Load {
                            to: left,
                            from: right,
                            offset: off,
                        },
                        line: cmd.line,
                        file: cmd.file,
                    });
                }
                "loadb" => {
                    let left = parse_var(variables, globals, &mut cmd_toks)?;
                    let right = parse_var(variables, globals, &mut cmd_toks)?;
                    let off = if cmd_toks.peek().is_some() {
                        Some(parse_op(variables, globals, &mut cmd_toks)?)
                    } else {
                        None
                    };
                    ins.push(EstInstr {
                        ins: EstIn::LoadByte {
                            to: left,
                            from: right,
                            offset: off,
                        },
                        line: cmd.line,
                        file: cmd.file,
                    });
                }

                "load&" => {
                    let left = parse_var(variables, globals, &mut cmd_toks)?;
                    let right = parse_var(variables, globals, &mut cmd_toks)?;
                    let off = if cmd_toks.peek().is_some() {
                        Some(parse_op(variables, globals, &mut cmd_toks)?)
                    } else {
                        None
                    };
                    ins.push(EstInstr {
                        ins: EstIn::LoadAddress {
                            to: left,
                            from: right,
                            offset: off,
                        },
                        line: cmd.line,
                        file: cmd.file,
                    });
                }
                "call" => {
                    let to_call = cmd_toks.next().throw()?;
                    let mut args = Vec::new();
                    while cmd_toks.peek().is_some() {
                        let op = parse_op(variables, globals, &mut cmd_toks)?;
                        args.push(op);
                    }
                    ins.push(EstInstr {
                        ins: EstIn::Call {
                            to_call: to_call.text.to_string(),
                            arguments: args,
                            output: None,
                        },
                        line: cmd.line,
                        file: cmd.file,
                    });
                }
                "=call" => {
                    let var = parse_var(variables, globals, &mut cmd_toks)?;
                    let to_call = cmd_toks.next().throw()?;
                    let mut args = Vec::new();
                    while cmd_toks.peek().is_some() {
                        let op = parse_op(variables, globals, &mut cmd_toks)?;
                        args.push(op);
                    }
                    if var.count != 1 {
                        println!(
                            "error cannot assign to array file:{}, line:{}",
                            to_call.file, to_call.line
                        );
                        todo!()
                    }
                    ins.push(EstInstr {
                        ins: EstIn::Call {
                            to_call: to_call.text.to_string(),
                            arguments: args,
                            output: Some(var),
                        },
                        line: cmd.line,
                        file: cmd.file,
                    });
                }
                _ => {
                    println!("{:#?}", cmd);
                    todo!()
                }
            }
        } else {
            let cmd = cmd_toks.next().throw()?;
            match cmd.as_ref() {
                "goto" => {
                    let to = cmd_toks.next().throw()?.as_ref().to_string();
                    end = BasicBlocEnd::Goto { to };
                }
                "if" => {
                    let cond = parse_op(variables, globals, &mut cmd_toks)?;
                    let gt = cmd_toks.next().throw()?;
                    if cond.is_array() {
                        println!("cannot branch on an array:{},line:{}", gt.file, gt.line);
                        todo!()
                    }
                    if gt.as_ref() != "goto" {
                        println!("{:#?}", gt);
                        todo!()
                    }
                    let if_true = cmd_toks.next().throw()?.as_ref().to_string();
                    let el = cmd_toks.next().throw()?;

                    if el.as_ref() != "else" {
                        println!("{:#?}", el);
                        todo!()
                    }
                    let gt = cmd_toks.next().throw()?;
                    if gt.as_ref() != "goto" {
                        println!("{:#?}", gt);
                        todo!()
                    }
                    let if_false = cmd_toks.next().throw()?.as_ref().to_string();
                    end = BasicBlocEnd::Branch {
                        cond,
                        if_true,
                        if_false,
                    }
                }
                "return" => {
                    if !cmd_toks.peek().is_some() {
                        end = BasicBlocEnd::Return { to_return: None };
                    } else {
                        let tok = cmd_toks.peek().throw()?;
                        let to_return = parse_op(variables, globals, &mut cmd_toks)?;
                        if to_return.is_array() {
                            println!("cannot return an array file:{},line:{}", tok.file, tok.line);
                            todo!()
                        }
                        end = BasicBlocEnd::Return {
                            to_return: Some(to_return),
                        }
                    }
                }
                _ => {
                    println!("undefined cmd:{:#?}", cmd);
                    todo!()
                }
            }
        }
    }
    Ok(BasicBloc {
        instructions: ins,
        end,
    })
}

pub fn translate_file(
    text: String,
    file: String,
    reached: &mut HashSet<String>,
) -> Throws<TranslationUnit> {
    reached.insert(file.clone());
    let mut stream = TokenStream::from_string(text, file);
    let mut functions = BTreeMap::new();
    let mut statics = BTreeMap::new();
    while let Some(mut tok) = stream.next() {
        let extrn = if tok.as_ref() == "extern" {
            tok = stream.next().throw()?;
            true
        } else {
            false
        };

        let inline = if tok.as_ref() == "inline" {
            tok = stream.next().throw()?;
            true
        } else {
            false
        };
        if tok.as_ref() == "fn" {
            let (name, func) = parse_function(inline, extrn, &mut stream, &statics)?;
            if functions.contains_key(&name) {
                todo!();
            }
            functions.insert(name, func);
        } else if tok.as_ref() == "static" {
            let (name, stat) = parse_static(inline, extrn, &mut stream)?;
            if statics.contains_key(&name) {
                todo!();
            }
            statics.insert(name, stat);
        } else if tok.as_ref() == "import" {
            let file = stream.next().throw()?;
            let s1 = file.text.strip_prefix("\"").throw()?;
            let name = s1.strip_suffix("\"").throw()?;
            if reached.contains(name) {
                continue;
            }
            let f = std::fs::read_to_string(name)?;
            let tc = translate_file(f, name.to_string(), reached)?;
            for (name, mut func) in tc.functions {
                if functions.contains_key(&name) {
                    continue;
                }
                func.external = true;
                func.blocks.clear();
                func.variables.clear();
                functions.insert(name, func);
            }
            for (name, mut s) in tc.statics {
                if statics.contains_key(&name) {
                    continue;
                } else {
                    s.external = true;
                    s.literal = Literal::UInt(0);
                }
                statics.insert(name, s);
            }
        } else {
            println!("{}", tok.as_ref());
            todo!()
        }
    }
    Ok(TranslationUnit { statics, functions })
}

pub fn tuci(trans: &TranslationUnit, file: String) {
    let mut out = String::new();
    out += "#include <stdint.h>\n";
    out += "typedef union EstroByte{
    char sn;
    unsigned char un;
} EstroByte;\n";
    out += "typedef union EstroWord{
    int64_t sn;
    uint64_t un;
    double db;
    union EstroWord * ptr;
    union EstroByte * byte_ptr;
    union EstroByte * str;
} EstroWord;\n";

    for (name, func) in &trans.functions {
        if func.external {
            out += "extern ";
        }
        if let Some(rt) = func.returns.clone() {
            match rt {
                Type::Word => {
                    out += "EstroWord ";
                }
                Type::Byte => {
                    out += "EstroByte ";
                }
            }
        } else {
            out += "void ";
        };
        out += "est_";
        out += &name;
        out += "(";
        for i in 0..func.args.len() {
            if i == func.args.len() - 1 {
                out += &format!("{} {}", func.args[i].kind.c_name(), func.args[i].name);
            } else {
                out += &format!("{} {},", func.args[i].kind.c_name(), func.args[i].name);
            }
        }
        out += ");\n";
    }
    out += "\n";
    let functions = trans.functions.clone();
    for (name, func) in &trans.functions {
        if func.external {
            continue;
        }
        if let Some(rt) = func.returns.clone() {
            match rt {
                Type::Word => {
                    out += "EstroWord ";
                }
                Type::Byte => {
                    out += "EstroByte ";
                }
            }
        } else {
            out += "void ";
        };
        out += "est_";
        out += &name;
        out += "(";
        for i in 0..func.args.len() {
            if i == func.args.len() - 1 {
                out += &format!("{} {}", func.args[i].kind.c_name(), func.args[i].name);
            } else {
                out += &format!("{} {},", func.args[i].kind.c_name(), func.args[i].name);
            }
        }
        out += "){\n";
        for i in &func.variables {
            if i.count == 1 {
                match i.kind {
                    Type::Word => out += &format!("\tEstroWord {} = {{}};\n", i.name),
                    Type::Byte => out += &format!("\tEstroByte {} = {{}};\n", i.name),
                }
            } else {
                match i.kind {
                    Type::Word => out += &format!("\tEstroWord {} [{}] = {{}};\n", i.name, i.count),
                    Type::Byte => {
                        out += &format!("\tEstroByte {} [{}] = {{}};\n", i.name, i.count);
                    }
                }
            }
        }
        let names: Vec<String> = func.blocks.iter().map(|(i, _)| i.to_string()).collect();
        let mut index = 0;
        for (name, vs) in &func.blocks {
            out += &format!("\t{}:\n", name);
            for i in &vs.instructions {
                match &i.ins {
                    EstIn::Move { to, from } => match from {
                        Operand::Var(variable) => {
                            if variable.is_byte() != to.is_byte() {
                                out += &format!("\t{}.un = {}.un;\n", to.name, variable.name);
                            } else {
                                out += &format!("\t{}.un= {}.un;\n", to.name, variable.name);
                            }
                        }
                        Operand::Lit(literal) => match literal {
                            Literal::String(x) => {
                                out += &format!("\t{}.str= {};\n", to.name, x);
                            }
                            Literal::Int(x) => {
                                out += &format!("\t{}.sn= {};\n", to.name, x);
                            }
                            Literal::UInt(x) => {
                                out += &format!("\t{}.un= {};\n", to.name, x);
                            }
                            Literal::Float(x) => {
                                out += &format!("\t{}.db= {};\n", to.name, x);
                            }
                            Literal::Bool(x) => {
                                out += &format!("\t{}.sn= {};\n", to.name, x);
                            }
                            Literal::Byte(x) => {
                                out += &format!("\t{}.un= {};\n", to.name, x);
                            }
                            Literal::List(_) => {
                                out += &format!("\t{}  = {};\n", to.name, literal.c_fmt());
                            }
                            Literal::ByteList(_) => {
                                out += &format!("\t{}  = {};\n", to.name, literal.c_fmt());
                            }
                        },
                    },
                    EstIn::Store { to, from, offset } => {
                        let tn = if to.count != 0 {
                            if let Some(of) = offset {
                                format!("*(EstroWord*)((char*){}+ {}.un)", to.name, of.c_fmt())
                            } else {
                                format!("{}[0]", to.name)
                            }
                        } else {
                            if let Some(of) = offset {
                                format!("*(EstroWord*)((char*)({}.ptr)+{}.un)", to.name, of.c_fmt())
                            } else {
                                format!("*({}.ptr)", to.name)
                            }
                        };
                        match from {
                            Operand::Var(variable) => {
                                out += &format!("\t({}).un= {}.un;\n", tn, variable.name);
                            }
                            Operand::Lit(literal) => match literal {
                                Literal::String(x) => {
                                    out += &format!("\t({}).str= {};\n", tn, x);
                                }
                                Literal::Int(x) => {
                                    out += &format!("\t({}).sn= {};\n", tn, x);
                                }
                                Literal::UInt(x) => {
                                    out += &format!("\t({}).un= {};\n", tn, x);
                                }
                                Literal::Float(x) => {
                                    out += &format!("\t({}).db= {};\n", tn, x);
                                }
                                Literal::Bool(x) => {
                                    out += &format!("\t({}).sn= {};\n", tn, x);
                                }
                                Literal::Byte(x) => {
                                    out += &format!("\t({}).un= {};\n", to.name, x);
                                }
                                Literal::List(_) => {
                                    out +=
                                        &format!("\t({}).ptr  = {};\n", to.name, literal.c_fmt());
                                }
                                Literal::ByteList(_) => {
                                    out += &format!("\t({}).ptr = {};\n", to.name, literal.c_fmt());
                                }
                            },
                        }
                    }
                    EstIn::StoreByte { to, from, offset } => {
                        let tn = if to.count != 0 {
                            if let Some(of) = offset {
                                format!("{}[{}.un]", to.name, of.c_fmt())
                            } else {
                                format!("{}[0]", to.name)
                            }
                        } else {
                            if let Some(of) = offset {
                                format!("*({}.byte_ptr+({}))", to.name, of.c_fmt())
                            } else {
                                format!("*({}.byte_ptr)", to.name)
                            }
                        };
                        match from {
                            Operand::Var(variable) => {
                                out += &format!("\t({}).un= {}.un;\n", tn, variable.name);
                            }
                            Operand::Lit(literal) => match literal {
                                Literal::String(x) => {
                                    out += &format!("\t({}).str= {};\n", tn, x);
                                }
                                Literal::Int(x) => {
                                    out += &format!("\t({}).sn= {};\n", tn, x);
                                }
                                Literal::UInt(x) => {
                                    out += &format!("\t({}).un= {};\n", tn, x);
                                }
                                Literal::Float(x) => {
                                    out += &format!("\t({}).db= {};\n", tn, x);
                                }
                                Literal::Bool(x) => {
                                    out += &format!("\t({}).sn= {};\n", tn, x);
                                }
                                Literal::Byte(x) => {
                                    out += &format!("\t({}).un= {};\n", to.name, x);
                                }
                                Literal::List(_) => {
                                    out += &format!(
                                        "\t({}).byte_ptr  = {};\n",
                                        to.name,
                                        literal.c_fmt()
                                    );
                                }
                                Literal::ByteList(_) => {
                                    out += &format!(
                                        "\t({}).byte_ptr = {};\n",
                                        to.name,
                                        literal.c_fmt()
                                    );
                                }
                            },
                        }
                    }

                    EstIn::Load { to, from, offset } => {
                        let tn = if from.count != 0 {
                            if let Some(of) = offset {
                                format!("*(EstroWord*)((char*){}+{}.sn)", from.name, of.c_fmt())
                            } else {
                                format!("{}[0]", from.name)
                            }
                        } else {
                            if let Some(of) = offset {
                                format!(
                                    "*(EstroWord*)((char*){}.ptr+({}).sn)",
                                    from.name,
                                    of.c_fmt()
                                )
                            } else {
                                format!("*({}.ptr)", from.name)
                            }
                        };
                        out += &format!("\t({}).un= ({}).un;\n", to.name, tn);
                    }

                    EstIn::LoadByte { to, from, offset } => {
                        let tn = if from.count != 1 {
                            if let Some(of) = offset {
                                format!("{}[({}).un]", from.name, of.c_fmt())
                            } else {
                                format!("&{}", from.name)
                            }
                        } else {
                            if let Some(of) = offset {
                                format!("{}.byte_ptr+({})", from.name, of.c_fmt())
                            } else {
                                format!("{}.byte_ptr", from.name)
                            }
                        };
                        out += &format!("\t{}.un = (unsigned char)({}).un;\n", to.name, tn);
                    }

                    EstIn::LoadAddress { to, from, offset } => {
                        let tf = if let Some(of) = offset {
                            format!("(&({})+{}))", from.name, of.c_fmt())
                        } else {
                            format!("&({})", from.name)
                        };
                        out += &format!("\t{}.ptr = &({});\n", to.name, tf);
                    }
                    EstIn::BinOp {
                        kind,
                        types,
                        left,
                        right,
                        output,
                    } => {
                        let op = match kind {
                            BinOpKind::Add => "+",
                            BinOpKind::Sub => "-",
                            BinOpKind::Div => "/",
                            BinOpKind::Mul => "*",
                            BinOpKind::Rem => "%",
                            BinOpKind::Less => "<",
                            BinOpKind::LessEq => "<=",
                            BinOpKind::Eq => "==",
                            BinOpKind::Neq => "!=",
                            BinOpKind::GreaterEq => ">=",
                            BinOpKind::Greater => ">",
                        };
                        let sz = match types {
                            BinopType::Float => "db",
                            BinopType::IByte => "sn",
                            BinopType::Byte => "un",
                            BinopType::IWord => "sn",
                            BinopType::Word => "un",
                        };
                        out += &format!(
                            "\t{}.{} = {}.{} {} {}.{};\n",
                            output.name,
                            sz,
                            left.c_fmt(),
                            sz,
                            op,
                            right.c_fmt(),
                            sz
                        );
                    }
                    EstIn::Call {
                        to_call,
                        arguments,
                        output,
                    } => {
                        let Some(tc) = functions.get(to_call) else {
                            todo!();
                        };
                        if tc.args.len() > arguments.len() {
                            todo!();
                        }
                        if arguments.len() > 12 {
                            todo!()
                        }
                        if output.is_some() && tc.returns.is_none() {
                            todo!();
                        }
                        let mut arg_str = "(".to_string();
                        for i in 0..arguments.len() {
                            match tc.args[i].kind {
                                Type::Byte => {
                                    if arguments[i].is_array() {
                                        arg_str += &format!(
                                            "(EstroWord){{.byte_ptr = {}}}",
                                            &arguments[i].c_fmt()
                                        );
                                    } else {
                                        arg_str += &format!(
                                            "(EstroByte){{.un = {}.un}}",
                                            &arguments[i].c_fmt()
                                        );
                                    }
                                }
                                Type::Word => {
                                    if arguments[i].is_array() {
                                        if arguments[i].can_be_byte() {
                                            arg_str += &format!(
                                                "(EstroWord){{.byte_ptr = {}}}",
                                                &arguments[i].c_fmt()
                                            );
                                        } else {
                                            arg_str += &format!(
                                                "(EstroWord){{.ptr = {}}}",
                                                &arguments[i].c_fmt()
                                            );
                                        }
                                    } else {
                                        arg_str += &format!(
                                            "(EstroWord){{.un = {}.un}}",
                                            &arguments[i].c_fmt()
                                        );
                                    }
                                }
                            }

                            if i != arguments.len() - 1 {
                                arg_str += ",";
                            }
                        }
                        arg_str += ")";
                        if let Some(op) = output {
                            out += &format!("\t{}.un = est_{}{}.un;\n", op.name, to_call, arg_str);
                        } else {
                            out += &format!("\test_{}{};\n", to_call, arg_str)
                        }
                    }
                }
            }
            match &vs.end {
                BasicBlocEnd::Branch {
                    cond,
                    if_true,
                    if_false,
                } => {
                    out += &format!(
                        "\tif ({}.un) goto {}_bloc_{};else goto {}_bloc_{};\n",
                        cond.c_fmt(),
                        func.name,
                        if_true,
                        func.name,
                        if_false
                    );
                }
                BasicBlocEnd::Goto { to } => {
                    if index != names.len() - 1 {
                        let fmt = format!("{}_bloc_{}", func.name, to);
                        if names[index + 1] == fmt {
                            index += 1;
                            continue;
                        }
                    }
                    out += &format!("\tgoto {}_bloc_{};\n", func.name, to);
                }
                BasicBlocEnd::Return { to_return } => {
                    if let Some(op) = to_return {
                        if let Some(rt) = func.returns.clone() {
                            match rt {
                                Type::Byte => {
                                    out += &format!(
                                        "\treturn (EstroByte){{.un = {}.un}};\n",
                                        op.c_fmt()
                                    );
                                }
                                Type::Word => {
                                    out += &format!(
                                        "\treturn (EstroWord){{.un = {}.un}};\n",
                                        op.c_fmt()
                                    );
                                }
                            }
                        } else {
                            todo!();
                        }
                    } else {
                        out += "\treturn;\n";
                    }
                }
            }
            index += 1;
        }
        out += "}\n\n";
    }
    out += "\nint main(int argc, const char ** argv){{
    EstroWord out = est_main((EstroWord){.sn = argc},(EstroWord){.ptr = (EstroWord*)argv});
    return out.sn;
}}";
    std::fs::write(file, out).unwrap();
}
