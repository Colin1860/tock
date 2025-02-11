//! Fixed Priority Scheduler for Tock
//!
//! This scheduler assigns priority to processes based on their order in the
//! `PROCESSES` array, and runs the highest priority process available at any
//! point in time. Kernel tasks (bottom half interrupt handling / deferred call
//! handling) always take priority over userspace processes.
//!
//! Notably, there is no need to enforce timeslices, as it is impossible for a
//! process running to not be the highest priority process at any point while it
//! is running. The only way for a process to longer be the highest priority is
//! for an interrupt to occur, which will cause the process to stop running.

use crate::dwt;
use crate::dynamic_deferred_call::DynamicDeferredCall;
use crate::kernel::{Kernel, StoppedExecutingReason};
use crate::platform::chip::Chip;
use crate::process::ProcessId;
use crate::scheduler::{Scheduler, SchedulingDecision};
use crate::utilities::cells::OptionalCell;

/// Priority scheduler based on the order of processes in the `PROCESSES` array.
pub struct PrioritySched {
    kernel: &'static Kernel,
    running: OptionalCell<ProcessId>,
}

impl PrioritySched {
    pub const fn new(kernel: &'static Kernel) -> Self {
        Self {
            kernel,
            running: OptionalCell::empty(),
        }
    }
}

impl<C: Chip> Scheduler<C> for PrioritySched {
    fn next(&self, kernel: &Kernel) -> SchedulingDecision {
        if kernel.processes_blocked() {
            // No processes ready
            SchedulingDecision::TrySleep
        } else {
            // Iterates in-order through the process array, always running the
            // first process it finds that is ready to run. This enforces the
            // priorities of all processes.
            let next = self
                .kernel
                .get_process_iter()
                .find(|&proc| proc.ready())
                .map_or(None, |proc| Some(proc.processid()));
            self.running.insert(next);
            // PREEMPTIVE_SWITCH_START

            SchedulingDecision::RunProcess((next.unwrap(), None))
        }
    }

    unsafe fn continue_process(&self, _: ProcessId, chip: &C) -> bool {
        // In addition to checking for interrupts, also checks if any higher
        // priority processes have become ready. This check is necessary because
        // a system call by this process could make another process ready, if
        // this app is communicating via IPC with a higher priority app.

        let has_pending = chip.has_pending_interrupts();
        let calls_pending = DynamicDeferredCall::global_instance_calls_pending().unwrap_or(false);
        let higher_process = self
            .kernel
            .get_process_iter()
            .find(|proc| proc.ready())
            .map_or(false, |ready_proc| {
                self.running.map_or(false, |running| {
                    ready_proc.processid().index < running.index
                })
            });

        let ret = !(has_pending || calls_pending || higher_process);

        ret
    }

    fn result(&self, _: StoppedExecutingReason, _: Option<u32>) {
        self.running.clear()
    }
}
