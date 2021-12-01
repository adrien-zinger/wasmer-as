# Wasmer and AssemblyScript

This project is a fork from the original [wasmer-as](https://github.com/onsails/wasmer-as) ⭐ project

*Read and write from rust to webassembly*

```rust
use std::error::Error;
use wasmer::{Store, Module, Instance, imports, Function};
use wasmer_as::{Read, Write, StringPtr, Env, abort};

fn main() -> Result<(), Box<dyn Error>> {
    let wasm_bytes = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/yourfile.wasm"
    ));

    // Creation of the instance
    let store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;
    let import_object = imports! {
        "env" => {
            "abort" => Function::new_native_with_env(&store, Env::default(), abort),
        },
    };
    let instance = Instance::new(&module, &import_object)?;
    let memory = instance.exports.get_memory("memory").expect("get memory");

    // Clone manually the environment with a new (this is automatic in the import object but ot here)
    let env = Env::new(memory.clone(), match instance.exports.get_function("__new") {
        Ok(func) => Some(func.clone()),
        _ => None
    });

    // Get a string (require wasmer_as::Read)
    let get_string = instance
        .exports
        .get_native_function::<(), StringPtr>("getString")?;
    let str_ptr = get_string.call()?;
    let string = str_ptr.read(memory)?;
    assert_eq!(string, "hello test");

    // Create a new string (require wasmer_as::Write)
    let str_ptr_2 = StringPtr::alloc("hello return", &env)?;
    // Check
    let string = str_ptr_2.read(memory)?;
    assert_eq!(string, "hello return");

    Ok(())
}
```
