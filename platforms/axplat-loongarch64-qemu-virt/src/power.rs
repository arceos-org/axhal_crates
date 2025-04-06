use axhal_plat::power::PowerIf;

struct PowerImpl;

#[impl_plat_interface]
impl PowerIf for PowerImpl {
    /// Bootstraps the given CPU core with the given initial stack (in physical
    /// address).
    ///
    /// Where `cpu_id` is the logical CPU ID (0, 1, ..., N-1, N is the number of
    /// CPU cores on the platform).
    fn cpu_boot(_cpu_id: usize, _stack_top_paddr: usize) {
        #[cfg(feature = "smp")]
        crate::mp::start_secondary_cpu(_cpu_id, pa!(_stack_top_paddr));
    }

    /// Shutdown the whole system.
    fn system_off() -> ! {
        const HALT_ADDR: *mut u8 =
            crate::mem::phys_to_virt(pa!(crate::config::devices::GED_PADDR)).as_mut_ptr();

        info!("Shutting down...");
        unsafe { HALT_ADDR.write_volatile(0x34) };
        axhal_cpu::halt();
        warn!("It should shutdown!");
        loop {
            axhal_cpu::halt();
        }
    }
}
