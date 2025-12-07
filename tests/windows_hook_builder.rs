use windows_hook::{WindowsHookBuilder, WH, Module, ThreadId};

extern "system" fn dummy_hook_proc(_code: i32, _wparam: usize, _lparam: isize) -> isize {
    0
}

#[test]
fn new_builder_defaults() {
    let builder = WindowsHookBuilder::new(WH::KEYBOARD_LL, dummy_hook_proc);

    assert_eq!(builder.id, WH::KEYBOARD_LL);
    assert_eq!(builder.proc as usize, dummy_hook_proc as usize);
    assert!(builder.module.is_null());
    assert!(builder.thread_id.is_none());
}

#[test]
fn with_module_sets_module() {
    let module = Module::NULL;
    let builder = WindowsHookBuilder::new(WH::MOUSE_LL, dummy_hook_proc).with_module(module);

    assert_eq!(builder.module, module);
}

#[test]
fn with_thread_id_sets_thread_id() {
    let builder = WindowsHookBuilder::new(WH::MOUSE_LL, dummy_hook_proc).with_thread_id(1234);

    assert_eq!(builder.thread_id, ThreadId::from(1234));
}

#[test]
fn chained_builder_methods() {
    let module = Module::NULL;
    let builder = WindowsHookBuilder::new(WH::KEYBOARD_LL, dummy_hook_proc)
        .with_module(module)
        .with_thread_id(42);

    assert_eq!(builder.id, WH::KEYBOARD_LL);
    assert_eq!(builder.module, module);
    assert_eq!(builder.thread_id, ThreadId::from(42));
}
