use std::panic::AssertUnwindSafe;
use anyhow::Result;
use wasmer::{imports, Module, Store, wat2wasm, Singlepass, Universal};

fn compile_and_compare(wasm: &[u8]) -> Result<()> {
    let store = Default::default();

    // compile for first time
    let module = Module::new(&store, wasm)?;
    let first = module.serialize()?;

    // compile for second time
    let module = Module::new(&store, wasm)?;
    let second = module.serialize()?;

    assert!(first == second);

    Ok(())
}

#[test]
fn deterministic_empty() -> Result<()> {
    let wasm_bytes = wat2wasm(
        br#"
    (module)
    "#,
    )?;

    compile_and_compare(&wasm_bytes)
}

#[test]
fn deterministic_table() -> Result<()> {
    let wasm_bytes = wat2wasm(
        br#"
(module
  (table 2 funcref)
  (func $f1)
  (func $f2)
  (elem (i32.const 0) $f1 $f2))
"#,
    )?;

    compile_and_compare(&wasm_bytes)
}

#[test]
fn should_not_panic_on_bad_init_param() -> Result<()> {
    let wasm_bytes = wat2wasm(br#"
(module
    (type $t0 (func (param i32)))
    (func $init (type $t0) (param $p0 i32))
    (memory $memory 2)
    (export "memory" (memory 0))
    (export "init" (func $init)))
"#,)?;

    let store = Store::new(&Universal::new(Singlepass::default()).engine());
    let module = Module::new(&store, wasm_bytes)?;

    match wasmer::Instance::new(&module, &imports! {}) {
        Ok(_) => panic!("Expected an error"),
        Err(e) => {
            assert_eq!(e.to_string(), "RuntimeError: invalid pointer copy");
        }
    }

    Ok(())
}
