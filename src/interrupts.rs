use lazy_static::lazy_static;
use crate::gdt;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};


lazy_static! { // enables init of global static variables at runtime, rather than at compile time
    static ref IDT: InterruptDescriptorTable = {
        let mut idt: InterruptDescriptorTable = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler) // set double fault handler
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);    // set special stack for DFE
        }        
        
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}


extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

// Diverging function does not return from double fault error handler
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, _error_code: u64) -> ! 
{
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint_exception() {
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}
