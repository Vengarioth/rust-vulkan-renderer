use crate::graphics::rendergraph::*;
use std::fmt::Debug;

pub trait Executor {
    fn execute(self: Box<Self>, context: &mut ExecuteContext);
}

pub struct FnOnceExecutor<T> {
    value: T,
    executor: Box<dyn FnOnce(T, &mut ExecuteContext)>,
}

impl<T> FnOnceExecutor<T> {
    pub fn new(value: T, executor: Box<dyn FnOnce(T, &mut ExecuteContext)>) -> Self {
        Self {
            value,
            executor,
        }
    }

    fn execute_internal(self, context: &mut ExecuteContext) {
        let executor = self.executor;
        executor(self.value, context);
    }
}

impl<T> Executor for FnOnceExecutor<T> {
    fn execute(self: Box<Self>, context: &mut ExecuteContext) {
        self.execute_internal(context);
    }
}

impl Debug for dyn Executor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Executor")
    }
}
