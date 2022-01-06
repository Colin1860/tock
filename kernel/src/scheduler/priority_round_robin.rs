use core::cell::Cell;

use crate::{
    collections::list::{List, ListLink, ListNode},
    debug,
    kernel::StoppedExecutingReason,
    platform::chip::Chip,
    process::Process,
    scheduler::SchedulingDecision,
    Scheduler,
};

/// A node in the linked list the scheduler uses to track processes
/// Each node holds a pointer to a slot in the processes array

pub struct PRRProcessNode<'a> {
    proc: &'static Option<&'static dyn Process>,
    priority: Cell<u32>,
    next: ListLink<'a, PRRProcessNode<'a>>,
}

impl<'a> PRRProcessNode<'a> {
    pub fn new(proc: &'static Option<&'static dyn Process>, priority: u32) -> PRRProcessNode<'a> {
        PRRProcessNode {
            proc,
            priority: Cell::new(priority),
            next: ListLink::empty(),
        }
    }

    fn set_prio(&'a self, prio: u32) {
        self.priority.set(prio);
    }
}

impl<'a> ListNode<'a, PRRProcessNode<'a>> for PRRProcessNode<'a> {
    fn next(&'a self) -> &'a ListLink<'a, PRRProcessNode> {
        &self.next
    }

    fn prio(&'a self) -> Option<u32> {
        Some(self.priority.get())
    }
}

/// Priority Round Robin Scheduler
pub struct PriorityRoundRobinSched<'a> {
    time_remaining: Cell<u32>,
    pub processes: List<'a, PRRProcessNode<'a>>,
    pub done: List<'a, PRRProcessNode<'a>>,
    last_rescheduled: Cell<bool>,
}

impl<'a> PriorityRoundRobinSched<'a> {
    /// How long a process can run before being pre-empted
    const DEFAULT_TIMESLICE_US: u32 = 10000;
    pub const fn new() -> PriorityRoundRobinSched<'a> {
        PriorityRoundRobinSched {
            time_remaining: Cell::new(Self::DEFAULT_TIMESLICE_US),
            processes: List::new(),
            done: List::new(),
            last_rescheduled: Cell::new(false),
        }
    }

    fn populate_with_new_priorities(&self) {
        if self.processes.head().is_none() {
            // new calculated priorities put into list
            while self.done.head().is_some() {
                self.processes
                    .insert_with_prio(self.done.pop_head().unwrap());
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
            for node in self.processes.iter() {
                match node.proc {
                    Some(proc) => {
                        if proc.ready() {
                            next = Some(proc.processid());
                            next_proc = Some(*proc);
                            break;
                        } else {
                            self.processes.push_tail(self.processes.pop_head().unwrap());
                        }
                    }
                    None => {
                        node.set_prio(0);
                        self.done.push_tail(self.processes.pop_head().unwrap());
                    }
                }
            }

            if self.processes.head().is_none() && next.is_none() {
                self.populate_with_new_priorities();

                debug!(
                    "Priority: {}",
                    self.processes.head().unwrap().priority.get()
                );

                for node in self.processes.iter() {
                    match node.proc {
                        Some(proc) => {
                            if proc.ready() {
                                next = Some(proc.processid());
                                next_proc = Some(*proc);
                                break;
                            } else {
                                self.processes.push_tail(self.processes.pop_head().unwrap());
                            }
                        }
                        None => {
                            node.set_prio(0);
                            self.done.push_tail(self.processes.pop_head().unwrap());
                        }
                    }
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
        let returned = self.processes.pop_head().unwrap();

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
            returned.set_prio(index * used_time);
            self.done.push_head(returned);
        }

        self.populate_with_new_priorities();
    }
}
