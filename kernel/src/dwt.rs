use tock_registers::interfaces::{Readable, Writeable};

use crate::debug;
use crate::utilities::registers::ReadWrite;
use crate::utilities::StaticRef;

struct DWTRegisters {
    ctrl: ReadWrite<u32>,
    cycnt: ReadWrite<u32>,
}
// keep track of last process,
// in case we want to measure switch from process 1 to 0 e.g
pub static mut LAST: usize = 500;
// used that one as a read write destination if i wanted to see something in the debugger
pub static mut KEEPER: usize = 0;

const DWT: StaticRef<DWTRegisters> = unsafe { StaticRef::new(0xE0001000 as *const _) };
const DEMCR: StaticRef<ReadWrite<u32>> = unsafe { StaticRef::new(0xE000EDFC as *const _) };

pub unsafe fn reset_timer() {
    DEMCR.set(DEMCR.get() | 0x01000000);
    DWT.cycnt.set(0); // reset counter
    DWT.ctrl.set(0); // disable counter
}

pub unsafe fn is_enabled() -> bool {
    DWT.ctrl.get() & 0x00000001 > 0
}

pub unsafe fn start_timer() {
    DWT.ctrl.set(1);
}

pub unsafe fn stop_timer() {
    DWT.ctrl.set(0);
}

pub unsafe fn get_time() -> u32 {
    let ticks = DWT.cycnt.get();
    return ticks;
}
