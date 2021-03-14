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

struct InstBase {
    type_: Type,
    prev: u32,
    next: u32,
    users: Vec<u32>,
}

impl Default for InstBase {
    fn default() -> Self {
        InstBase {
            type_: Default::default(),
            prev: u32::MAX,
            next: u32::MAX,
            users: Vec::new(),
        }
    }
}

#[allow(dead_code)]
#[derive(Default)]
struct ParameterInst {
    inst_base: InstBase,
}

#[allow(dead_code)]
#[derive(Default)]
struct ConstantInst {
    inst_base: InstBase,
    value: i64,
}

struct BinaryOp {
    inst_base: InstBase,
    input1: u32,
    input2: u32,
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
    // preds: Vec<u32>,
    // succs: Vec<u32>,
    instructions: HashMap<u32, Inst>,
    first: u32,
    last: u32,
}

impl BasicBlock {
    pub fn new() -> BasicBlock {
        BasicBlock {
            instructions: HashMap::new(),
            first: u32::MAX,
            last: u32::MAX,
        }
    }

    pub fn dump(&self) {
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

#[derive(Default)]
struct Graph {
    params: HashMap<u32, ParameterInst>,
    consts: HashMap<u32, ConstantInst>,
    blocks: HashMap<u32, BasicBlock>,
    insts_blocks: HashMap<u32, u32>,
}

impl Graph {
    fn push(&mut self, id: u32, bb: BasicBlock) {
        assert!(
            self.blocks.insert(id, bb).is_none(),
            "Basic block with id {} already exists",
            id
        );
    }

    fn push_inst(&mut self, id_bb: u32, id_inst: u32, inst: Inst) {
        assert!(
            self.insts_blocks.contains_key(&id_inst) == false,
            "Instruction with id {} already exists in this graph",
            id_inst
        );
        assert!(
            self.blocks.contains_key(&id_bb) == true,
            "Basic block with id {} does not exist in this graph",
            id_bb
        );

        self.blocks
            .get_mut(&id_bb)
            .unwrap()
            .push_back(id_inst, inst);
        self.insts_blocks.insert(id_inst, id_bb);
    }

    fn dump(&self) {
        for (id, block) in &self.blocks {
            println!("BB{}:", id);
            block.dump();
        }
    }
}

struct IrConstructor<'a> {
    graph: &'a mut Graph,
    current_bb: u32,
    current_inst: u32,
}

impl<'a> IrConstructor<'_> {
    pub fn new(graph: &'a mut Graph) -> IrConstructor<'a> {
        IrConstructor {
            graph: graph,
            current_bb: u32::MAX,
            current_inst: u32::MAX,
        }
    }

    pub fn get_mut_inst(&mut self) -> &mut Inst {
        (*self.graph.blocks.get_mut(&self.current_bb).unwrap())
            .instructions
            .get_mut(&self.current_inst)
            .unwrap()
    }

    pub fn s32(&mut self) -> &mut Self {
        self.get_mut_inst().set_type(Type::I32);
        self
    }
}

macro_rules! basic_block {
    ( $ir_constructor:ident, $id: expr ) => {
        let mut bb = BasicBlock::new();

        $ir_constructor.graph.push($id, bb);
        $ir_constructor.current_bb = $id;
    };
}

macro_rules! inst {
    ( $ir_constructor:ident, $opcode:ident, $id:expr ) => {{
        let mut inst = Inst::$opcode(Default::default());
        $ir_constructor
            .graph
            .push_inst($ir_constructor.current_bb, $id, inst);
        $ir_constructor.current_inst = $id;
        &mut $ir_constructor
    }};
}

fn main() {
    let mut graph: Graph = Default::default();
    let mut z = IrConstructor::new(&mut graph);

    basic_block!(z, 2);
    {
        inst!(z, Add, 1).s32();
        inst!(z, Add, 2).s32();
    }
    basic_block!(z, 3);
    {
        inst!(z, Add, 3).s32();
        inst!(z, Add, 4).s32();
    }

    z.graph.dump();
}
