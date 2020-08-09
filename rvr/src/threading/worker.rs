use crate::Error;
use crossbeam_channel::{unbounded, Sender, Receiver};
use std::thread;

pub trait Worker {
    type Task;
    type TaskResult;

    fn execute(&mut self, task: Self::Task) -> Result<Self::TaskResult, Error>;
}

#[derive(Debug)]
pub struct WorkerThread<Task, TaskResult> {
    task_sender: Sender<Task>,
    result_receiver: Receiver<Result<TaskResult, Error>>,
    handle: thread::JoinHandle<()>,
}

impl<Task: 'static + Send + Sync, TaskResult: 'static + Send + Sync> WorkerThread<Task, TaskResult> {
    pub fn new(mut worker: Box<dyn Worker<Task = Task, TaskResult = TaskResult> + Send>) -> Self {
        let (task_sender, task_receiver) = unbounded();
        let (result_sender, result_receiver) = unbounded();

        let handle = thread::spawn(move || {
            loop {
                if let Ok(task) = task_receiver.recv() {
                    let result = worker.execute(task);
                    result_sender.send(result);
                }
            }
        });

        Self {
            task_sender,
            result_receiver,
            handle,
        }
    }

    pub fn enqueue(&mut self, task: Task) -> Result<(), Error> {
        self.task_sender.send(task)?;
        Ok(())
    }

    pub fn poll(&mut self) -> Option<Result<TaskResult, Error>> {
        if let Ok(result) = self.result_receiver.try_recv() {
            Some(result)
        } else {
            None
        }
    }
}
