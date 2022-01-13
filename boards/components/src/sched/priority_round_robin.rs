use core::mem::MaybeUninit;
use kernel::component::Component;
use kernel::process::Process;
use kernel::scheduler::priority_round_robin::{PRRProcessNode, PriorityRoundRobinSched};
use kernel::{debug, static_init_half};

#[macro_export]
macro_rules! prr_component_helper {
    ($N:expr $(,)?) => {{
        use core::mem::MaybeUninit;
        use kernel::scheduler::round_robin::RoundRobinProcessNode;
        use kernel::static_buf;
        const UNINIT: MaybeUninit<PRRProcessNode> = MaybeUninit::uninit();
        static mut BUF1: [MaybeUninit<PRRProcessNode>; $N] = [UNINIT; $N];
        static mut BUF2: MaybeUninit<PriorityRoundRobinSched<'static>> = MaybeUninit::uninit();
        (&mut BUF1, &mut BUF2)
    };};
}

pub struct PriorityRoundRobinComponent {
    processes: &'static [Option<&'static dyn Process>],
}

impl PriorityRoundRobinComponent {
    pub fn new(processes: &'static [Option<&'static dyn Process>]) -> PriorityRoundRobinComponent {
        PriorityRoundRobinComponent { processes }
    }
}

impl Component for PriorityRoundRobinComponent {
    type StaticInput = (
        &'static mut [MaybeUninit<PRRProcessNode>],
        &'static mut MaybeUninit<PriorityRoundRobinSched<'static>>,
    );
    type Output = &'static mut PriorityRoundRobinSched<'static>;

    unsafe fn finalize(self, buf: Self::StaticInput) -> Self::Output {
        let (nodes, sched) = buf;

        let scheduler = static_init_half!(
            sched,
            PriorityRoundRobinSched<'static>,
            PriorityRoundRobinSched::new()
        );

        for (i, node) in nodes.iter_mut().enumerate() {
            let init_node = static_init_half!(
                node,
                PRRProcessNode,
                PRRProcessNode::new(&self.processes[i], i as u32)
            );

            let _ = scheduler.processes.borrow_mut().push(init_node);
        }
        scheduler
    }
}
