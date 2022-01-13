use core::cell::RefMut;
use core::cmp::Ord;
use core::fmt::Binary;
use core::{cell::Cell, cell::RefCell};
use heapless::binary_heap::{BinaryHeap, Min};
use heapless::Vec;

use crate::debug;
use crate::{
    kernel::StoppedExecutingReason, platform::chip::Chip, process::Process,
    scheduler::SchedulingDecision, Scheduler,
};

/// A node in the linked list the scheduler uses to track processes
/// Each node holds a pointer to a slot in the processes array

pub struct PRRProcessNode {
    pub proc: &'static Option<&'static dyn Process>,
    priority: Cell<u32>,
}

impl PRRProcessNode {
    pub fn new(proc: &'static Option<&'static dyn Process>, priority: u32) -> PRRProcessNode {
        PRRProcessNode {
            proc,
            priority: Cell::new(priority),
        }
    }
}

impl Ord for PRRProcessNode {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.priority.get().cmp(&other.priority.get())
    }
}

impl Eq for PRRProcessNode {}

impl PartialOrd for PRRProcessNode {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PRRProcessNode {
    fn eq(&self, other: &Self) -> bool {
        self.priority.get() == other.priority.get()
    }
}

/// Priority Round Robin Scheduler
pub struct PriorityRoundRobinSched<'a> {
    time_remaining: Cell<u32>,
    pub processes: RefCell<BinaryHeap<&'a PRRProcessNode, Min, 8>>,
    pub done: RefCell<Vec<&'a PRRProcessNode, 8>>,
    last_rescheduled: Cell<bool>,
}

impl<'a> PriorityRoundRobinSched<'a> {
    /// How long a process can run before being pre-empted
    const DEFAULT_TIMESLICE_US: u32 = 10000;
    pub const fn new() -> PriorityRoundRobinSched<'a> {
        PriorityRoundRobinSched {
            time_remaining: Cell::new(Self::DEFAULT_TIMESLICE_US),
            processes: RefCell::new(BinaryHeap::new()),
            done: RefCell::new(Vec::new()),
            last_rescheduled: Cell::new(false),
        }
    }

    fn populate_with_new_priorities(
        &self,
        mut heap: RefMut<BinaryHeap<&'a PRRProcessNode, Min, 8>>,
    ) {
        if heap.is_empty() {
            let mut done = self.done.borrow_mut();
            while !done.is_empty() {
                let _ = heap.push(done.pop().unwrap());
            }
        }
    }
}

impl<'a, C: Chip> Scheduler<C> for PriorityRoundRobinSched<'a> {
    fn next(&self, kernel: &crate::Kernel) -> super::SchedulingDecision {
        if kernel.processes_blocked() {
            // No processes ready
            SchedulingDecision::TrySleep
        } else {
            let mut next = None; // This will be replaced, bc a process is guaranteed
                                 // to be ready if processes_blocked() is false

            let mut next_proc: Option<&dyn Process> = None;

            // Initial round robin scheduling not done yet
            // Find next ready process. Place any *empty* process slots, or not-ready
            // processes, at the back of the queue.
            'check: while !self.processes.borrow().is_empty() {
                debug!("Running top loop");
                let mut borrowed = self.processes.borrow_mut();
                while let Some(node) = borrowed.peek() {
                    match node.proc {
                        Some(proc) => {
                            if proc.ready() {
                                next = Some(proc.processid());
                                next_proc = Some(*proc);
                                debug!("Priority scheduled: {}", node.priority.get());
                                break 'check;
                            } else {
                                let _ = self.done.borrow_mut().push(borrowed.pop().unwrap());
                            }
                        }
                        None => {
                            node.priority.set(0);
                            let _ = self.done.borrow_mut().push(borrowed.pop().unwrap());
                        }
                    }
                }

                if next.is_none() {
                    self.populate_with_new_priorities(borrowed);
                }
            }

            let timeslice = if self.last_rescheduled.get() {
                self.time_remaining.get()
            } else {
                // grant a fresh timeslice
                self.time_remaining.set(
                    next_proc
                        .unwrap()
                        .get_timeslice()
                        .map(|x| x as u32)
                        .unwrap_or(Self::DEFAULT_TIMESLICE_US),
                );
                self.time_remaining.get()
            };
            assert!(timeslice != 0);

            return SchedulingDecision::RunProcess((next.unwrap(), Some(timeslice)));
        }
    }

    fn result(
        &self,
        result: crate::kernel::StoppedExecutingReason,
        execution_time_us: Option<u32>,
    ) {
        let used_time = execution_time_us.unwrap();
        let returned = self.processes.borrow_mut().pop().unwrap();

        let reschedule = match result {
            StoppedExecutingReason::KernelPreemption => {
                if self.time_remaining.get() > used_time {
                    self.time_remaining
                        .set(self.time_remaining.get() - used_time);
                    true
                } else {
                    false
                }
            }
            _ => false,
        };
        self.last_rescheduled.set(reschedule);

        if !reschedule {
            let index = returned.proc.unwrap().processid().index as u32;
            returned.priority.set(index * used_time);
            let _ = self.done.borrow_mut().push(returned);
        } else {
            self.processes.borrow_mut().push(returned);
        }

        let processes = self.processes.borrow_mut();

        if processes.is_empty() {
            self.populate_with_new_priorities(processes);
        }
    }
}
