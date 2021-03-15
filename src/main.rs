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

macro_rules! type_str {
    ( $type:expr ) => {{
        match $type {
            Type::NoType => "",
            Type::I32 => "i32",
        }
    }};
}

macro_rules! dump_users {
    ( $users:expr ) => {
        for user in &$users {
            print!("{}, ", user);
        }
        println!("");
    };
}

impl Inst {
    pub fn dump(&self) {
        match self {
            Inst::Parameter(base) => {
                println!("{} Parameter          -> ", type_str!(base.type_));
            }
            Inst::Constant(con) => {
                println!(
                    "{} Constant {}        -> ",
                    type_str!(con.inst_base.type_),
                    con.value
                );
            }
            Inst::Add(op) => {
                print!(
                    "{} Add            {}, {} -> ",
                    type_str!(op.inst_base.type_),
                    op.input1,
                    op.input2
                );
                dump_users!(op.inst_base.users);
            }
        }
    }

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

// Basic block with id 0 is called "start block" and is reserved for parameters
// and constants, with id -1 is called "end block"
struct Graph {
    blocks: HashMap<i32, BasicBlock>,
    // insts_blocks: HashMap<u32, i32>,
}

impl Graph {
    fn new() -> Graph {
        let mut graph = Graph {
            blocks: HashMap::new(),
        };
        graph.push(0, BasicBlock::new());
        graph.push(-1, BasicBlock::new());
        graph
    }

    fn push(&mut self, id: i32, bb: BasicBlock) {
        assert!(
            self.blocks.insert(id, bb).is_none(),
            "Basic block with id {} already exists",
            id
        );
    }

    fn contains_inst(&self, id: u32) -> bool {
        let mut ret = false;
        for (_, block) in &self.blocks {
            ret = block.instructions.contains_key(&id);
        }
        ret
    }

    fn push_inst(&mut self, id_bb: i32, id_inst: u32, inst: Inst) {
        assert!(
            self.contains_inst(id_inst) == false,
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
        // self.insts_blocks.insert(id_inst, id_bb);
    }

    fn dump(&self) {
        let mut keys = Vec::new();
        for (id, _) in &self.blocks {
            keys.push(id);
        }
        keys.sort();

        for x in &keys {
            if **x == -1 {
                continue;
            }
            println!("BB{}:", x);
            self.blocks.get(x).unwrap().dump();
        }
        println!("BB-1");
    }
}

struct IrConstructor<'a> {
    graph: &'a mut Graph,
    current_bb: i32,
    current_inst: u32,
}

impl<'a> IrConstructor<'_> {
    pub fn new(graph: &'a mut Graph) -> IrConstructor<'a> {
        IrConstructor {
            graph: graph,
            current_bb: 0,
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

    pub fn val(&mut self, value: i64) -> &mut Self {
        self.get_mut_inst().set_value(value);
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
    let mut graph = Graph::new();
    let mut z = IrConstructor::new(&mut graph);

    inst!(z, Parameter, 1).s32();
    inst!(z, Constant, 2).val(7).s32();
    basic_block!(z, 2);
    {
        inst!(z, Add, 3).s32();
        inst!(z, Add, 4).s32();
    }
    basic_block!(z, 3);
    {
        inst!(z, Add, 5).s32();
        inst!(z, Add, 6).s32();
    }

    z.graph.dump();
}
