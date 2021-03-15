use std::collections::HashSet;

pub enum Type {
    NoType,
    I32,
    /*
    I64,
    U32,
    U64,
    Void,
    Bool,
    */
}

impl Default for Type {
    fn default() -> Self {
        Type::NoType
    }
}

#[allow(dead_code)]
enum Cc {
    Eq_,
    Less,
    Greater,
}

pub struct InstBase {
    pub type_: Type,
    prev: u32,
    next: u32,
    pub users: HashSet<u32>,
}

impl Default for InstBase {
    fn default() -> Self {
        InstBase {
            type_: Default::default(),
            prev: u32::MAX,
            next: u32::MAX,
            users: HashSet::new(),
        }
    }
}

#[derive(Default)]
pub struct ConstantInst {
    pub inst_base: InstBase,
    pub value: i64,
}

pub struct BinaryOp {
    pub inst_base: InstBase,
    pub input1: u32,
    pub input2: u32,
}

impl Default for BinaryOp {
    fn default() -> Self {
        BinaryOp {
            inst_base: Default::default(),
            input1: u32::MAX,
            input2: u32::MAX,
        }
    }
}

pub enum Inst {
    Parameter(InstBase),
    Constant(ConstantInst),

    Add(BinaryOp),
    /*
    Sub(BinaryOp),
    Mul(BinaryOp),
    Div(BinaryOp),
    And(BinaryOp),
    Or(BinaryOp),
    Xor(BinaryOp),
    */
    // Cmp(),
    // If(),
}

impl Inst {
    pub fn get_next(&self) -> u32 {
        match self {
            Inst::Parameter(base) => base.next,
            Inst::Constant(con) => con.inst_base.next,
            Inst::Add(op) => op.inst_base.next,
        }
    }

    pub fn set_next(&mut self, next: u32) {
        match self {
            Inst::Parameter(base) => base.next = next,
            Inst::Constant(con) => con.inst_base.next = next,
            Inst::Add(op) => op.inst_base.next = next,
        }
    }

    pub fn set_prev(&mut self, prev: u32) {
        match self {
            Inst::Parameter(base) => base.prev = prev,
            Inst::Constant(con) => con.inst_base.prev = prev,
            Inst::Add(op) => op.inst_base.prev = prev,
        }
    }

    pub fn set_type(&mut self, type_: Type) {
        match self {
            Inst::Parameter(base) => base.type_ = type_,
            Inst::Constant(con) => con.inst_base.type_ = type_,
            Inst::Add(op) => op.inst_base.type_ = type_,
        }
    }

    pub fn set_value(&mut self, value: i64) {
        match self {
            Inst::Constant(con) => con.value = value,
            _ => panic!("Can not set value for non constant instruction"),
        }
    }

    pub fn set_inputs(&mut self, inputs: &[u32]) {
        match self {
            Inst::Add(op) => {
                op.input1 = inputs[0];
                op.input2 = inputs[1];
            }
            _ => panic!("Not implemented yet"),
        }
    }

    pub fn insert_user(&mut self, user: u32) {
        match self {
            Inst::Parameter(base) => base.users.insert(user),
            Inst::Constant(con) => con.inst_base.users.insert(user),
            Inst::Add(op) => op.inst_base.users.insert(user),
        };
    }
}
