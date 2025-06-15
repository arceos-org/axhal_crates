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
    axconfig_macros::include_configs!(path_env = "AX_CONFIG_PATH", fallback = "axconfig.toml");
}
const_assert!(const_str::compare!(==, env!("CARGO_PKG_NAME"), config::PACKAGE));
