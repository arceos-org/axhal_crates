use riscv::interrupt::supervisor::{Exception as E, Interrupt as I};
use riscv::interrupt::Trap;
use riscv::register::{scause, stval};

use super::TrapFrame;
use crate::trap::PageFaultFlags;

core::arch::global_asm!(
    include_asm_marcos!(),
    include_str!("trap.S"),
    trapframe_size = const core::mem::size_of::<TrapFrame>(),
);

fn handle_breakpoint(sepc: &mut usize) {
    debug!("Exception(Breakpoint) @ {:#x} ", sepc);
    *sepc += 2
}

fn handle_page_fault(tf: &TrapFrame, mut access_flags: PageFaultFlags, is_user: bool) {
    if is_user {
        access_flags |= PageFaultFlags::USER;
    }
    let vaddr = va!(stval::read());
    if !handle_trap!(PAGE_FAULT, vaddr, access_flags, is_user) {
        panic!(
            "Unhandled {} Page Fault @ {:#x}, fault_vaddr={:#x} ({:?}):\n{:#x?}",
            if is_user { "User" } else { "Supervisor" },
            tf.sepc,
            vaddr,
            access_flags,
            tf,
        );
    }
}

#[unsafe(no_mangle)]
fn riscv_trap_handler(tf: &mut TrapFrame, from_user: bool) {
    let scause = scause::read();
    if let Ok(cause) = scause.cause().try_into::<I, E>() {
        match cause {
            #[cfg(feature = "uspace")]
            Trap::Exception(E::UserEnvCall) => {
                tf.regs.a0 = crate::trap::handle_syscall(tf, tf.regs.a7) as usize;
                tf.sepc += 4;
            }
            Trap::Exception(E::LoadPageFault) => {
                handle_page_fault(tf, PageFaultFlags::READ, from_user)
            }
            Trap::Exception(E::StorePageFault) => {
                handle_page_fault(tf, PageFaultFlags::WRITE, from_user)
            }
            Trap::Exception(E::InstructionPageFault) => {
                handle_page_fault(tf, PageFaultFlags::EXECUTE, from_user)
            }
            Trap::Exception(E::Breakpoint) => handle_breakpoint(&mut tf.sepc),
            Trap::Interrupt(int) => {
                handle_trap!(IRQ, int as usize);
            }
            _ => {
                panic!("Unhandled trap {:?} @ {:#x}:\n{:#x?}", cause, tf.sepc, tf);
            }
        }
    } else {
        panic!(
            "Unknown trap {:#x?} @ {:#x}:\n{:#x?}",
            scause.cause(),
            tf.sepc,
            tf
        );
    }
}
