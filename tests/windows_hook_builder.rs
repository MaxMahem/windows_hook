use fluent_result::bool::Expect;
use windows_hook::{Module, ThreadId, WH, WindowsHookBuilder};

extern "system" fn dummy_hook_proc(_code: i32, _wparam: usize, _lparam: isize) -> isize {
    0
}

#[test]
fn new_builder_defaults() {
    let builder = WindowsHookBuilder::new(WH::KEYBOARD_LL, dummy_hook_proc);

    assert_eq!(builder.id, WH::KEYBOARD_LL, "Hook id should match");
    assert_eq!(
        builder.proc as usize, dummy_hook_proc as usize,
        "Hook proc should match"
    );
    assert!(builder.module.is_null(), "Module should be null");
    assert!(builder.thread_id.is_none(), "Thread id should be none");
}

#[test]
fn with_module_sets_module() {
    let module = Module::NULL;
    let builder = WindowsHookBuilder::new(WH::MOUSE_LL, dummy_hook_proc).with_module(module);

    assert_eq!(builder.module, module, "Module should match");
}

#[test]
fn with_thread_id_sets_thread_id() {
    let builder = WindowsHookBuilder::new(WH::MOUSE_LL, dummy_hook_proc).with_thread_id(1234);

    assert_eq!(
        builder.thread_id,
        ThreadId::from(1234),
        "Thread id should match"
    );
}

#[test]
fn chained_builder_methods() {
    let module = Module::NULL;
    let builder = WindowsHookBuilder::new(WH::KEYBOARD_LL, dummy_hook_proc)
        .with_module(module)
        .with_thread_id(42);

    assert_eq!(builder.id, WH::KEYBOARD_LL, "Hook id should match");
    assert_eq!(builder.module, module, "Module should match");
    assert_eq!(
        builder.thread_id,
        ThreadId::from(42),
        "Thread id should match"
    );
}

#[test]
fn build_and_set_creates_hook() {
    WindowsHookBuilder::new(WH::KEYBOARD_LL, dummy_hook_proc)
        .build_and_set()
        .expect("Failed to build and set hook")
        .state()
        .is_set()
        .expect_true("Hook should be in set state");
}
