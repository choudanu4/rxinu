use arch::x86::memory::MemoryController;
use x86;

#[cfg(target_arch = "x86")]
use x86::bits32::task::TaskStateSegment;
#[cfg(target_arch = "x86_64")]
use x86::bits64::task::TaskStateSegment;

mod exception;
mod gdt;
mod idt;
mod irq;

const DOUBLE_FAULT_IST_INDEX: usize = 0;

#[cfg(target_arch = "x86")]
pub fn init(_memory_controller: &mut MemoryController) {
    let tss = TaskStateSegment::new();

    unsafe {
        gdt::init(&tss);
    }
    unsafe {
        idt::init();
    }
}

#[cfg(target_arch = "x86_64")]
/// Initialize double fault stack and load gdt and idt
pub fn init(memory_controller: &mut MemoryController) {
    let mut tss = TaskStateSegment::new();

    let double_fault_stack = memory_controller
        .alloc_stack(1)
        .expect("could not allocate double fault stack");
    tss.ist[DOUBLE_FAULT_IST_INDEX] = double_fault_stack.top() as u64;

    unsafe {
        gdt::init(&tss);
    }
    unsafe {
        idt::init();
    }
}

/// Disable interrupts, execute code uninterrupted, re-enable interrupts
pub fn disable_interrupts_then<T>(uninterrupted_fn: T) -> ()
where
    T: FnOnce() -> (),
{
    unsafe {
        x86::shared::irq::disable();
    }
    uninterrupted_fn();
    unsafe {
        x86::shared::irq::enable();
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn breakpoint_exception() {
        unsafe {
            asm!("int3");
        }
    }
}
