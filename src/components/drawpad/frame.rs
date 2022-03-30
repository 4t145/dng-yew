use serde::{Serialize, Deserialize};
use super::figure::Instruction;

#[derive(Debug, Serialize, Deserialize)]
pub struct Frame {
    pub instructions: Vec<Instruction>
}

impl Frame {
    pub fn new()->Self{
        Self{
            instructions: Vec::new()
        }
    }
    pub fn push(&mut self, ins: Instruction) {
        self.instructions.push(ins);
    }


}



