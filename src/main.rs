use std::collections::HashMap;

enum Type {
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

#[allow(dead_code)]
enum Cc {
    Eq_,
    Less,
    Greater,
}

struct InstBase {
    id: u32,
    type_: Type,
    prev: u32,
    next: u32,
    users: Vec<u32>,
}

impl InstBase {
    pub fn new() -> InstBase {
        InstBase {
            id: 0,
            type_: Type::NoType,
            prev: 0,
            next: 0,
            users: Vec::new(),
        }
    }
}

#[allow(dead_code)]
struct ParameterInst {
    id: u32,
    type_: Type,
    users: Vec<u32>,
}

#[allow(dead_code)]
struct ConstantInst {
    id: u32,
    type_: Type,
    value: i64,
    users: Vec<u32>,
}

struct BinaryOp {
    inst_base: InstBase,
    input1: u32,
    input2: u32,
}

impl BinaryOp {
    pub fn new() -> BinaryOp {
        BinaryOp {
            inst_base: InstBase::new(),
            input1: 0,
            input2: 0,
        }
    }
}

enum Inst {
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
    pub fn get_id(&self) -> u32 {
        match self {
            Inst::Add(op) => op.inst_base.id,
        }
    }

    pub fn set_id(&mut self, id: u32) {
        match self {
            Inst::Add(op) => op.inst_base.id = id,
        }
    }

    pub fn dump(&self) {
        match self {
            Inst::Add(op) => {
                let type_ = match op.inst_base.type_ {
                    Type::NoType => "",
                    Type::I32 => "i32",
                };
                print!("{} Add    Inputs: {}, {}; ", type_, op.input1, op.input2);
                print!("Users:");
                for user in &op.inst_base.users {
                    print!(" {}", user);
                }
                println!("");
            }
        }
    }

    pub fn get_next(&self) -> u32 {
        match self {
            Inst::Add(op) => op.inst_base.next,
        }
    }

    pub fn set_next(&mut self, next: u32) {
        match self {
            Inst::Add(op) => op.inst_base.next = next,
        }
    }

    pub fn set_prev(&mut self, prev: u32) {
        match self {
            Inst::Add(op) => op.inst_base.prev = prev,
        }
    }

    pub fn set_type(&mut self, type_: Type) {
        match self {
            Inst::Add(op) => op.inst_base.type_ = type_,
        }
    }
}

struct BasicBlock {
    id: u32,
    // preds: Vec<u32>,
    // succs: Vec<u32>,
    instructions: HashMap<u32, Inst>,
    first: u32,
    last: u32,
}

impl BasicBlock {
    pub fn new(id: u32) -> BasicBlock {
        BasicBlock {
            id: id,
            instructions: HashMap::new(),
            first: u32::MAX,
            last: u32::MAX,
        }
    }

    pub fn dump(&self) {
        println!("BB{}:", self.id);

        let mut next = self.first;

        loop {
            if next == u32::MAX {
                break;
            }
            print!("  {} ", next);
            let inst = self.instructions.get(&next).unwrap();
            inst.dump();
            next = inst.get_next();
        }
    }

    // checks whether is basic block empty
    // sets id to next field of last instruction, resets last instruction to this,
    // sets next to -1, inserts instruction to instructions map
    pub fn push_back(&mut self, id: u32, mut inst: Inst) {
        assert!(
            !self.instructions.contains_key(&id),
            "Instruction with id {} already exists",
            id
        );

        if self.last == u32::MAX {
            self.first = id;
            inst.set_prev(u32::MAX);
        } else {
            (*self.instructions.get_mut(&self.last).unwrap()).set_next(id);
        }
        self.last = id;
        inst.set_next(u32::MAX);
        self.instructions.insert(id, inst);
    }
}

#[allow(dead_code)]
struct Graph {
    params: HashMap<u32, ParameterInst>,
    consts: HashMap<u32, ConstantInst>,
    basic_blocks: HashMap<u32, BasicBlock>,
}

struct IrConstructor<'a> {
    current_bb: Option<&'a mut BasicBlock>,
    current_inst: u32,
}

impl IrConstructor<'_> {
    pub fn new() -> IrConstructor<'static> {
        IrConstructor {
            current_bb: None,
            current_inst: u32::MAX,
        }
    }

    pub fn s32(&mut self) -> &mut Self {
        (*(*self.current_bb.as_mut().unwrap())
            .instructions
            .get_mut(&self.current_inst)
            .unwrap())
        .set_type(Type::I32);
        self
    }
}

macro_rules! basic_block {
    ( $ir_constructor:ident, $id: expr ) => {
        let mut bb = BasicBlock::new($id);

        $ir_constructor.current_bb = Some(&mut bb);
    };
}

macro_rules! inst_add {
    ( $ir_constructor:ident, $id:expr ) => {{
        let mut inst = Inst::Add(BinaryOp::new());
        inst.set_id($id);
        $ir_constructor
            .current_bb
            .as_mut()
            .unwrap()
            .push_back($id, inst);
        $ir_constructor.current_inst = $id;
        &mut $ir_constructor
    }};
}

fn main() {
    let mut z = IrConstructor::new();

    basic_block!(z, 2);
    {
        inst_add!(z, 1).s32();
        inst_add!(z, 2).s32();
    }

    (*z.current_bb.as_ref().unwrap()).dump();
}
