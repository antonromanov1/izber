use crate::basic_block::{BasicBlock, Graph};
use crate::instruction::Inst;
use crate::instruction::Type;

macro_rules! dump_users {
    ( $users:expr ) => {
        let mut users = Vec::new();
        for user in &$users {
            users.push(user);
            // print!("{}, ", user);
        }
        users.sort();

        for user in users {
            print!("{}, ", user);
        }
        println!("");
    };
}

macro_rules! type_str {
    ( $type:expr ) => {{
        match $type {
            Type::NoType => "",
            Type::I32 => "i32",
        }
    }};
}

impl Inst {
    pub fn dump(&self) {
        match self {
            Inst::Parameter(base) => {
                print!("{} Parameter          -> ", type_str!(base.type_));
                dump_users!(base.users);
            }
            Inst::Constant(con) => {
                print!(
                    "{} Constant {}        -> ",
                    type_str!(con.inst_base.type_),
                    con.value
                );
                dump_users!(con.inst_base.users);
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
}

impl BasicBlock {
    pub fn dump(&self) {
        let mut next = self.first;

        print!("preds: ");
        for pred in &self.get_preds() {
            print!("{}, ", pred);
        }
        print!("succs: ");
        for succ in &self.get_succs() {
            print!("{}, ", succ);
        }
        println!("");

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
}

impl Graph {
    pub fn dump(&self) {
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
            println!("");
        }
        println!("BB-1");
    }
}
