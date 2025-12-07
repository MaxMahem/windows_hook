use windows_hook::{HHOOK, Handle, KeyboardLLHook, WH};

// A dummy hook procedure that simply calls the next hook in the chain.
extern "system" fn dummy_hook_proc(code: i32, wparam: usize, lparam: isize) -> isize {
    HHOOK::NULL.CallNextHookEx(unsafe { WH::from_raw(code) }, wparam, lparam)
}

#[test]
fn keyboard_ll_hook_set_and_state() {
    let hook =
        KeyboardLLHook::set_new(dummy_hook_proc).expect("Failed to set keyboard low-level hook");

    assert!(hook.state().is_set());
}
