use std::collections::HashMap;

use crate::instruction::Inst;

pub struct BasicBlock {
    // preds: Vec<u32>,
    // succs: Vec<u32>,
    pub instructions: HashMap<u32, Inst>,
    pub first: u32,
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
pub struct Graph {
    pub blocks: HashMap<i8, BasicBlock>,
}

impl Graph {
    pub fn new() -> Graph {
        let mut graph = Graph {
            blocks: HashMap::new(),
        };
        graph.push(0, BasicBlock::new());
        graph.push(-1, BasicBlock::new());
        graph
    }

    pub fn push(&mut self, id: i8, bb: BasicBlock) {
        assert!(
            self.blocks.insert(id, bb).is_none(),
            "Basic block with id {} already exists",
            id
        );
    }

    fn contains_inst(&self, id: u32) -> i8 {
        let mut ret: i8 = -1;
        for (id_bb, block) in &self.blocks {
            if block.instructions.contains_key(&id) {
                ret = *id_bb
            }
        }
        ret
    }

    pub fn push_inst(&mut self, id_bb: i8, id_inst: u32, inst: Inst) {
        assert!(
            self.contains_inst(id_inst) == -1,
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
    }

    pub fn set_inputs(&mut self, id: u32, inputs: &[u32]) {
        let bb = self.contains_inst(id);
        assert!(
            bb != -1,
            "Instruction with id {} does not exist in this graph",
            id
        );

        for input in inputs {
            let input_bb = self.contains_inst(*input);
            assert!(
                input_bb != -1,
                "Input with id {} does not exist in this graph",
                *input
            );
            self.blocks
                .get_mut(&input_bb)
                .unwrap()
                .instructions
                .get_mut(input)
                .unwrap()
                .insert_user(id);
        }

        self.blocks
            .get_mut(&bb)
            .unwrap()
            .instructions
            .get_mut(&id)
            .unwrap()
            .set_inputs(inputs);
    }
}
