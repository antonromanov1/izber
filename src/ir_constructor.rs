use crate::basic_block::{BasicBlock, Graph};
use crate::instruction::{Inst, Type};

pub struct IrConstructor<'a> {
    pub graph: &'a mut Graph,
    pub current_bb: i8,
    pub current_inst: u32,
}

impl<'a> IrConstructor<'_> {
    pub fn new(graph: &'a mut Graph) -> IrConstructor<'a> {
        IrConstructor {
            graph: graph,
            current_bb: 0,
            current_inst: u32::MAX,
        }
    }

    pub fn basic_block(&mut self, id: i8, _succs: &[i8]) {
        let bb = BasicBlock::new();

        self.graph.push(id, bb);
        self.current_bb = id;
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

    pub fn inputs(&mut self, inputs: &[u32]) -> &mut Self {
        self.graph.set_inputs(self.current_inst, inputs);
        self
    }
}

#[macro_export]
macro_rules! inst {
    ( $ir_constructor:ident, $opcode:ident, $id:expr ) => {{
        let inst = Inst::$opcode(Default::default());
        $ir_constructor
            .graph
            .push_inst($ir_constructor.current_bb, $id, inst);
        $ir_constructor.current_inst = $id;
        &mut $ir_constructor
    }};
}

#[macro_export]
macro_rules! parameter {
    ( $ir_constructor:ident, $id:expr ) => {{
        inst!($ir_constructor, Parameter, $id)
    }};
}

#[macro_export]
macro_rules! constant {
    ( $ir_constructor:ident, $id:expr ) => {{
        inst!($ir_constructor, Constant, $id)
    }};
}
