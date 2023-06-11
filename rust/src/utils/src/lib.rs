#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use std::cell::RefCell;

use anyhow::Result;
use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};
use sysinfo::System;
use sysinfo::SystemExt;

#[global_allocator]
pub static GLOBAL: &StatsAlloc<std::alloc::System> = &INSTRUMENTED_SYSTEM;

thread_local! {
    pub static SYSTEM: RefCell<System> = RefCell::new(System::new_all());
}

pub fn instrument<T>(title: &str, f: impl FnOnce() -> T) -> T {
    println!("{:=^50}", "");
    println!("|{:^48}|", title);
    println!("{:=^50}", "");

    let mem_old = SYSTEM.with(|s| {
        let mut s = s.borrow_mut();
        s.refresh_memory();
        s.used_memory()
    }) as i64;
    let region = Region::new(&GLOBAL);
    let t = std::time::Instant::now();
    // -------------
    let ret = f();
    // -------------
    let elapsed = t.elapsed();
    let mem = SYSTEM.with(|s| {
        let mut s = s.borrow_mut();
        s.refresh_memory();
        s.used_memory()
    }) as i64;

    // let mem_sysinfo = format!("used memory (sysinfo): {}\n", mem - mem_old);
    let mem_sysinfo = format!("");
    println!(
        "elapsed: {elapsed:?}
{mem_sysinfo}used memory: {}
",
        humansize::format_size(
            region.change().bytes_allocated - region.change().bytes_deallocated,
            humansize::BINARY
        ),
    );
    ret
}

pub fn instrument_res<T>(title: &str, f: impl FnOnce() -> Result<T, anyhow::Error>) -> T {
    instrument(title, f).unwrap()
}

pub const OKK: Result<(), anyhow::Error> = Ok(());

pub fn sep() {
    println!("\n\n{:-^80}\n\n", "");
}
