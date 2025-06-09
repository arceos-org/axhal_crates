#![no_std]

#[macro_use]
extern crate log;
#[macro_use]
extern crate axplat;
#[macro_use]
extern crate memory_addr;

mod boot;
mod console;
mod init;
#[cfg(feature = "irq")]
mod irq;
mod mem;
mod power;
mod time;

mod config {
    axconfig_gen_macros::include_configs!("axconfig.toml");
}

unsafe extern "C" fn rust_entry(cpu_id: usize, dtb: usize) {
    axplat::call_main(cpu_id, dtb);
}

#[cfg(feature = "smp")]
unsafe extern "C" fn rust_entry_secondary(cpu_id: usize) {
    axplat::call_secondary_main(cpu_id);
}
