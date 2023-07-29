use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};

bindgen!({
    world: "hello-world",
    // TODO: I don't understant very much why async is required (for wasi).
    // thread 'main' panicked at 'cannot use `func_wrap_async` without enabling async support in the config', /Users/xxchan/.cargo/git/checkouts/wasmtime-41807828cb3a7a7e/5b93cdb/crates/wasmtime/src/component/linker.rs:287:9
    async: true,
});

bindgen!({
    world: "byebye-world",
    // TODO: I don't understant very much why async is required (for wasi).
    // thread 'main' panicked at 'cannot use `func_wrap_async` without enabling async support in the config', /Users/xxchan/.cargo/git/checkouts/wasmtime-41807828cb3a7a7e/5b93cdb/crates/wasmtime/src/component/linker.rs:287:9
    async: true,
});

struct MyState {
    name: String,
    wasi_ctx: wasmtime_wasi::preview2::WasiCtx,
    table: wasmtime_wasi::preview2::Table,
}

impl wasmtime_wasi::preview2::WasiView for MyState {
    fn table(&self) -> &wasmtime_wasi::preview2::Table {
        &self.table
    }

    fn table_mut(&mut self) -> &mut wasmtime_wasi::preview2::Table {
        &mut self.table
    }

    fn ctx(&self) -> &wasmtime_wasi::preview2::WasiCtx {
        &self.wasi_ctx
    }

    fn ctx_mut(&mut self) -> &mut wasmtime_wasi::preview2::WasiCtx {
        &mut self.wasi_ctx
    }
}

// Imports into the world, like the `name` import for this world, are satisfied
// through traits.
#[async_trait::async_trait]
impl HelloWorldImports for MyState {
    async fn name(&mut self) -> wasmtime::Result<String> {
        Ok(self.name.clone())
    }
    // Note the `Result` return value here where `Ok` is returned back to
    // the component and `Err` will raise a trap.
}

#[tokio::main]
async fn main() -> wasmtime::Result<()> {
    // Configure an `Engine` and compile the `Component` that is being run for
    // the application.
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let engine = Engine::new(&config)?;
    let component = Component::from_file(&engine, "wasm_component.wasm")?;
    let component_wasi = Component::from_file(&engine, "wasm_component_wasi.wasm")?;

    // Instantiation of bindings always happens through a `Linker`.
    // Configuration of the linker is done through a generated `add_to_linker`
    // method on the bindings structure.
    //
    // Note that the closure provided here is a projection from `T` in
    // `Store<T>` to `&mut U` where `U` implements the `HelloWorldImports`
    // trait. In this case the `T`, `MyState`, is stored directly in the
    // structure so no projection is necessary here.
    let mut linker = Linker::new(&engine);
    HelloWorld::add_to_linker(&mut linker, |state: &mut MyState| state)?;

    // As with the core wasm API of Wasmtime instantiation occurs within a
    // `Store`. The bindings structure contains an `instantiate` method which
    // takes the store, component, and linker. This returns the `bindings`
    // structure which is an instance of `HelloWorld` and supports typed access
    // to the exports of the component.
    let mut table = wasmtime_wasi::preview2::Table::new();
    let wasi_ctx = wasmtime_wasi::preview2::WasiCtxBuilder::new()
        .inherit_stdio() // this is needed for println to work
        .build(&mut table)?;
    let mut store = Store::new(
        &engine,
        MyState {
            name: "smart🥵boy".to_string(),
            table,
            wasi_ctx,
        },
    );
    let (bindings, _) = HelloWorld::instantiate_async(&mut store, &component, &linker).await?;

    wasmtime_wasi::preview2::wasi::command::add_to_linker(&mut linker)?;
    let (bindings_wasi, _) =
        HelloWorld::instantiate_async(&mut store, &component_wasi, &linker).await?;

    // Here our `greet` function doesn't take any parameters for the component,
    // but in the Wasmtime embedding API the first argument is always a `Store`.
    let s = bindings.call_greet(&mut store).await?;
    println!("{}", s);

    let s = bindings_wasi.call_greet(&mut store).await?;
    println!("{}", s);

    // ======================
    // another component!

    let (bindings, _) = ByebyeWorld::instantiate_async(&mut store, &component, &linker).await?;
    let s = bindings.call_byebye(&mut store).await?;
    println!("{}", s);

    Ok(())
}
