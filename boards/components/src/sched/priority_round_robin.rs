use core::borrow::Borrow;
use core::mem::MaybeUninit;
use heapless::binary_heap::Min;
use heapless::BinaryHeap;
use heapless::Vec;
use kernel::component::Component;
use kernel::debug;
use kernel::process::Process;
use kernel::scheduler::priority_round_robin::{PRRProcessNode, PriorityRoundRobinSched};
use kernel::{static_init, static_init_half};

#[macro_export]
macro_rules! prr_component_helper {
    ($N:expr $(,)?) => {{
        use core::mem::MaybeUninit;
        use kernel::scheduler::round_robin::RoundRobinProcessNode;
        use kernel::static_buf;
        const UNINIT: MaybeUninit<PRRProcessNode> = MaybeUninit::uninit();
        static mut BUF1: [MaybeUninit<PRRProcessNode>; $N] = [UNINIT; $N];
        &mut BUF1
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
    type StaticInput = &'static mut [MaybeUninit<PRRProcessNode>];
    type Output = &'static mut PriorityRoundRobinSched<'static>;

    unsafe fn finalize(self, buf: Self::StaticInput) -> Self::Output {
        let scheduler = static_init!(
            PriorityRoundRobinSched<'static>,
            PriorityRoundRobinSched::new()
        );

        for (i, node) in buf.iter_mut().enumerate() {
            let init_node = static_init_half!(
                node,
                PRRProcessNode,
                PRRProcessNode::new(&self.processes[i], i as u32)
            );
            scheduler
                .processes
                .borrow_mut()
                .push(init_node)
                .map_err(|_| "Knallt beim ersten befüllen")
                .unwrap();
        }
        scheduler
    }
}
