mod basic_block;
mod dump;
mod instruction;
mod ir_constructor;

use crate::instruction::Inst;
use crate::ir_constructor::IrConstructor;
use basic_block::{BasicBlock, Graph};

fn main() {
    let line = "================================================================================";
    println!("{}", line);
    println!("Instruction dump format:");
    println!("<ID> <Type of instruction> <Mnemonic> <Value (if Constant)> <Inputs> -> <Users>");
    println!("{}", line);

    let mut graph = Graph::new();
    let mut z = IrConstructor::new(&mut graph);

    inst!(z, Parameter, 1).s32();
    inst!(z, Constant, 2).val(7).s32();
    basic_block!(z, 2);
    {
        inst!(z, Add, 3).s32().inputs(&[1, 2]);
        inst!(z, Add, 4).s32().inputs(&[1, 2]);
    }
    basic_block!(z, 3);
    {
        inst!(z, Add, 5).s32().inputs(&[1, 2]);
        inst!(z, Add, 6).s32().inputs(&[4, 5]);
    }

    z.graph.dump();
}
