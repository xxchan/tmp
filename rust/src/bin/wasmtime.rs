//! An example of how to interact with wasm memory.
//!
//! Here a small wasm module is used to show how memory is initialized, how to
//! read and write memory through the `Memory` object, and how wasm functions
//! can trap when dealing with out-of-bounds addresses.

// You can execute this example with `cargo run --example memory`

#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use std::{cell::RefCell, path::PathBuf};

use wasmtime::*;

use utils::*;

// error: the `#[global_allocator]` in this crate conflicts with global allocator in: utils
// #[global_allocator]
// pub static A: std::alloc::System = std::alloc::System;

struct MyState {
    my_secret_value: i32,
}

fn main() -> Result<()> {
    let wat = r#"
    (module
        (import "" "" (func $add (param i32 i32) (result i32)))
        (import "" "" (func $add_untyped (param i32 i32) (result i32)))
        (import "" "" (func $add_stateful (param i32 i32) (result i32)))



        (memory (export "memory") 2 3)
        (func (export "size") (result i32) (memory.size))
        (func (export "load") (param i32) (result i32)
          (i32.load8_s (local.get 0))
        )
        (func (export "store") (param i32 i32)
          (i32.store8 (local.get 0) (local.get 1))
        )
      
        (data (i32.const 0x1000) "\01\02\03\04")


        (func (export "call_add_twice") (param i32 i32) (result i32)
            local.get 0
            local.get 1
            call $add
            local.get 0
            local.get 1
            call $add
            i32.add)
    
        (func (export "call_add_twice_untyped") (param i32 i32) (result i32)
            local.get 0
            local.get 1
            call $add_untyped
            local.get 0
            local.get 1
            call $add_untyped
            i32.add)

        (func (export "call_add_twice_stateful") (param i32 i32) (result i32)
            local.get 0
            local.get 1
            call $add_stateful
            local.get 0
            local.get 1
            call $add_stateful
            i32.add)
        
      )
    "#;

    _ = instrument("create a stuplid vec![1,2,3]", || vec![1, 2, 3]);

    // Create our `store_fn` context and then compile a module and create an
    // instance from the compiled module all in one go.
    let mut store: Store<MyState> = instrument("create store", || {
        let mut config = Config::new();
        let config = config
            .debug_info(true)
            .consume_fuel(true)
            // .max_wasm_stack(65536 * 1000)
            // .async_stack_size(65536 * 1000)
            // below the size, the linear memory won't grow
            // TODO: This IS NOT "max memory usage"!! (Don't quite understand it yet)
            // It seems we should use `ResourceLimiter` via `store.limiter`
            // see also https://github.com/bytecodealliance/wasmtime/issues/2273
            .static_memory_maximum_size(65536 * 2048) 
            // .static_memory_forced(true);
            // .dynamic_memory_reserved_for_growth(1)
            ;
        let engine = Engine::new(&config).unwrap();

        Store::new(
            &engine,
            MyState {
                my_secret_value: 42,
            },
        )
    });
    _ = store.add_fuel(100);

    let module = instrument("create module", || Module::new(store.engine(), wat))?;

    let instance = instrument("create instance", || {
        // Create a custom `Func` which can execute arbitrary code inside of the
        // closure.
        let add = Func::wrap(&mut store, |a: i32, b: i32| -> i32 { a + b });
        let add_stateful = Func::wrap(
            &mut store,
            |mut caller: Caller<'_, MyState>, a: i32, b: i32| -> i32 {
                let fuel = caller.fuel_consumed();
                println!("fuel consumed: {:?}", fuel);
                let state = caller.data_mut();
                state.my_secret_value += 1;
                println!("wow! my_secret_value is now {}", state.my_secret_value);
                let ret = a + b + state.my_secret_value;
                let fuel = caller.fuel_consumed();
                println!("fuel consumed: {:?}", fuel);
                ret
            },
        );
        let add_untyped = Func::new(
            &mut store,
            FuncType::new([ValType::I32, ValType::I32], [ValType::I32]),
            move |mut caller, params, results| {
                let a = params[0].unwrap_i32();
                let b = params[1].unwrap_i32();
                let ret = a + b;

                results[0] = Val::I32(ret);
                Ok(())
            },
        );
        Instance::new(
            &mut store,
            &module,
            &[add.into(), add_untyped.into(), add_stateful.into()],
        )
    })?;

    let mut file = PathBuf::from(file!());
    file.pop();
    file.pop();
    let module_evil = Module::from_file(store.engine(), file.join("wasm-evil/out.wat")).unwrap();
    let instance_evil = Instance::new(&mut store, &module_evil, &[]).unwrap();

    sep();

    // play with memory
    {
        // load_fn up our exports from the instance
        let memory = instance
            .get_memory(&mut store, "memory")
            .ok_or(anyhow::format_err!("failed to find `memory` export"))?;
        let size = instance.get_typed_func::<(), i32>(&mut store, "size")?;
        let load_fn = instance.get_typed_func::<i32, i32>(&mut store, "load")?;
        let store_fn = instance.get_typed_func::<(i32, i32), ()>(&mut store, "store")?;

        instrument("Checking memory...", || {
            assert_eq!(memory.size(&store), 2);
            assert_eq!(memory.data_size(&store), 0x20000);
            assert_eq!(memory.data_mut(&mut store)[0], 0);
            assert_eq!(memory.data_mut(&mut store)[0x1000], 1);
            assert_eq!(memory.data_mut(&mut store)[0x1003], 4);

            assert_eq!(size.call(&mut store, ())?, 2);
            assert_eq!(load_fn.call(&mut store, 0)?, 0);
            assert_eq!(load_fn.call(&mut store, 0x1000)?, 1);
            assert_eq!(load_fn.call(&mut store, 0x1003)?, 4);
            assert_eq!(load_fn.call(&mut store, 0x1ffff)?, 0);
            assert!(load_fn.call(&mut store, 0x20000).is_err()); // out of bounds trap
            println!(
                "out of bounds trap: {}",
                load_fn.call(&mut store, 0x20000).unwrap_err()
            );
            OKK
        })?;

        instrument("Mutating memory...", || {
            memory.data_mut(&mut store)[0x1003] = 5;

            store_fn.call(&mut store, (0x1002, 6))?;
            assert!(store_fn.call(&mut store, (0x20000, 0)).is_err()); // out of bounds trap
            println!(
                "out of bounds trap: {}",
                load_fn.call(&mut store, 0x20000).unwrap_err()
            );

            assert_eq!(memory.data(&store)[0x1002], 6);
            assert_eq!(memory.data(&store)[0x1003], 5);
            assert_eq!(load_fn.call(&mut store, 0x1002)?, 6);
            assert_eq!(load_fn.call(&mut store, 0x1003)?, 5);
            OKK
        })?;

        instrument("Growing memory...", || {
            memory.grow(&mut store, 1)?;
            assert_eq!(memory.size(&store), 3);
            assert_eq!(memory.data_size(&store), 0x30000);

            assert_eq!(load_fn.call(&mut store, 0x20000)?, 0);
            store_fn.call(&mut store, (0x20000, 0))?;
            assert!(load_fn.call(&mut store, 0x30000).is_err());
            assert!(store_fn.call(&mut store, (0x30000, 0)).is_err());

            assert!(memory.grow(&mut store, 1).is_err());
            assert!(memory.grow(&mut store, 0).is_ok());
            OKK
        })?;

        instrument("Creating stand-alone memory...", || {
            let memorytype = MemoryType::new(5, Some(5));
            let memory2 = Memory::new(&mut store, memorytype)?;
            assert_eq!(memory2.size(&store), 5);
            assert!(memory2.grow(&mut store, 1).is_err());
            assert!(memory2.grow(&mut store, 0).is_ok());
            OKK
        })?;
    }

    sep();

    // play with function
    {
        {
            let untyped = instance.get_func(&mut store, "call_add_twice").unwrap();
            let err = match untyped.typed::<(i32, f32), i32>(&mut store) {
                Ok(_) => unreachable!(),
                Err(e) => e,
            };
            println!("type mismatch: {err}");
            let typed = untyped.typed::<(i32, i32), i32>(&mut store).unwrap();

            instrument_res("call_add_twice: call typed", || {
                assert_eq!(typed.call(&mut store, (4, 6))?, 20);
                OKK
            });
            instrument_res("call_add_twice: call untyped", || {
                let mut res = vec![Val::I32(114514)];
                untyped.call(&mut store, &[Val::I32(4), Val::I32(6)], &mut res)?;
                assert_eq!(res[0].unwrap_i32(), 20);
                OKK
            });
        }

        {
            let untyped = instance
                .get_func(&mut store, "call_add_twice_untyped")
                .unwrap();
            let err = match untyped.typed::<(i32, f32), i32>(&mut store) {
                Ok(_) => unreachable!(),
                Err(e) => e,
            };
            let typed = untyped.typed::<(i32, i32), i32>(&mut store).unwrap();

            instrument_res("call_add_twice_untyped: call typed", || {
                assert_eq!(typed.call(&mut store, (4, 6))?, 20);
                OKK
            });
            instrument_res("call_add_twice_untyped: call untyped", || {
                let mut res = vec![Val::I32(114514)];
                untyped.call(&mut store, &[Val::I32(4), Val::I32(6)], &mut res)?;
                assert_eq!(res[0].unwrap_i32(), 20);
                OKK
            });
        }

        {
            let stateful = instance
                .get_typed_func::<(i32, i32), i32>(&mut store, "call_add_twice_stateful")
                .unwrap();

            instrument_res("call_add_twice_stateful: call typed", || {
                assert_eq!(stateful.call(&mut store, (4, 6))?, 20 + 43 + 44);
                OKK
            });

            assert_eq!(store.data().my_secret_value, 44);
        }
    }

    {
        sep();

        let sleep = instance_evil.get_func(&mut store, "sleep").unwrap();
        match sleep.call(&mut store, &[], &mut []) {
            Ok(_) => todo!(),
            Err(e) => {
                assert!(e.downcast_ref::<Trap>().is_some());
                let root_cause = e.root_cause().downcast_ref::<Trap>().unwrap();
                println!(
                    "sleep error: {}\nroot_cause: {} ({:?})",
                    e, root_cause, root_cause
                );
            }
        }
        sep();
        _ = store.add_fuel(u64::MAX);

        let memory = instance_evil
            .get_memory(&mut store, "memory")
            .ok_or(anyhow::format_err!("failed to find `memory` export"))?;
        println!(
            "memory: {:?}, data_size: 65536*{}",
            memory.ty(&store),
            memory.size(&store)
        );

        let mut out = vec![Val::I32(0)];

        // let a_lot_of_stack_memory = instance_evil
        //     .get_func(&mut store, "a_lot_of_stack_memory")
        //     .unwrap();
        // match a_lot_of_stack_memory.call(&mut store, &[], &mut out) {
        //     Ok(_) => println!("a_lot_of_stack_memory: OK"),
        //     Err(e) => {
        //         assert!(e.downcast_ref::<Trap>().is_some());
        //         let root_cause = e.root_cause().downcast_ref::<Trap>().unwrap();
        //         println!(
        //             "a_lot_of_stack_memory error: {}\nroot_cause: {} ({:?})",
        //             e, root_cause, root_cause
        //         );
        //     }
        // }
        // println!(
        //     "memory: {:?}, data_size: {}",
        //     memory.ty(&store),
        //     memory.data_size(&store)
        // );
        // sep();

        let a_lot_of_heap_memory = instance_evil
            .get_func(&mut store, "a_lot_of_heap_memory")
            .unwrap();
        match a_lot_of_heap_memory.call(&mut store, &[], &mut out) {
            Ok(_) => println!("a_lot_of_heap_memory: OK"),
            Err(e) => {
                assert!(e.downcast_ref::<Trap>().is_some());
                let root_cause = e.root_cause().downcast_ref::<Trap>().unwrap();
                println!(
                    "a_lot_of_stack_memory error: {}\nroot_cause: {} ({:?})",
                    e, root_cause, root_cause
                );
            }
        }
        println!(
            "memory: {:?}, data_size: 65536* {}",
            memory.ty(&store),
            memory.size(&store)
        );
    }

    Ok(())
}
