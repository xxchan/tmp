use std::alloc::Layout;

#[no_mangle]
pub fn sleep() {
    loop {}
}

// TODO: It seems this isn't rejected by `max_wasm_stack(1024 * 512)`
// What is `__stack_pointer`?
#[no_mangle]
pub fn a_lot_of_stack_memory() -> usize {
    // one page is 64KiB (65536) 
    // let v = [0u8; 65536 * 100];
    1
}

#[no_mangle]
pub fn a_lot_of_heap_memory() -> usize {
    let mut v = vec![];
    for i in 0..65536 * 100 {
        v.push(i);
    }
    let mut ret = 0;
    for i in v {
        ret += i;
    }
    ret
}
