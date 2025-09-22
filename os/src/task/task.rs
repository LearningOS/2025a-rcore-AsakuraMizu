//! Types related to task management

use alloc::collections::btree_map::BTreeMap;

use super::TaskContext;

/// The task control block (TCB) of a task.
#[derive(Default, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// The syscall counter
    pub syscall_counter: BTreeMap<usize, usize>,
}

/// The status of a task
#[derive(Default, Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    #[default]
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
