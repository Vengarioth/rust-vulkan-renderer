use crate::graphics::*;

#[derive(Debug)]
pub enum Instruction {
    CreateImage,
    ReleaseImage,
    ExecutePass,
    Present,

    ImageLayoutBarrier {
        id: u32,
        from: ImageLayout,
        to: ImageLayout,
    },
}

#[derive(Debug)]
pub struct Schedule {
    instructions: Vec<Instruction>,
}

impl Schedule {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
        }
    }

    pub fn get_instructions(&self) -> &[Instruction] {
        &self.instructions
    }
}
