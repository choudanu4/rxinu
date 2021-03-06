use device::keyboard::ps2::PS2_KEYBOARD;
use device::pic_8259 as pic;
use device::uart_16550 as serial;
use task::scheduler::{global_sched, Scheduling};
use x86_64::structures::idt::ExceptionStackFrame;

pub extern "x86-interrupt" fn timer(_stack_frame: &mut ExceptionStackFrame) {
    pic::MASTER.lock().ack();
    global_sched().tick();
}

pub extern "x86-interrupt" fn keyboard(_stack_frame: &mut ExceptionStackFrame) {
    use device::ps2_controller_8042;
    use device::BufferedDevice;

    // Read a single scancode off our keyboard port.
    let code = ps2_controller_8042::key_read();

    // Pass scan code to ps2 driver buffer
    PS2_KEYBOARD.lock().buffer_mut().push_back(code);

    pic::MASTER.lock().ack();
}

#[allow(unused_variables)]
pub extern "x86-interrupt" fn cascade(_stack_frame: &mut ExceptionStackFrame) {
    pic::MASTER.lock().ack();
}

pub extern "x86-interrupt" fn com1(_stack_frame: &mut ExceptionStackFrame) {
    serial::COM1.lock().receive();
    pic::MASTER.lock().ack();
}

pub extern "x86-interrupt" fn com2(_stack_frame: &mut ExceptionStackFrame) {
    serial::COM2.lock().receive();
    pic::MASTER.lock().ack();
}
