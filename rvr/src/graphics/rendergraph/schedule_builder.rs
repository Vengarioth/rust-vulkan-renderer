use crate::graphics::{
    *,
    rendergraph::*,
};

pub struct ScheduleBuilder {
    schedule: Vec<Instruction>,
}

impl ScheduleBuilder {
    pub fn new() -> Self {
        Self {
            schedule: Vec::new(),
        }
    }

    pub fn add_image_layout_barrier(&mut self, id: u32, from: ImageLayout, to: ImageLayout) {
        self.schedule.push(Instruction::ImageLayoutBarrier {
            id,
            from,
            to,
        });
    }

    pub fn build(self) -> Schedule {
        Schedule::new(self.schedule)
    }
}
