use windows_hook::{MouseLLHook, HHOOK, Handle, WH};

// A dummy hook procedure that simply calls the next hook in the chain.
extern "system" fn dummy_hook_proc(code: i32, wparam: usize, lparam: isize) -> isize {
    HHOOK::NULL.CallNextHookEx(unsafe { WH::from_raw(code) }, wparam, lparam)
}

#[test]
fn mouse_ll_hook_set_and_state() {
    let hook =
        MouseLLHook::set_new(dummy_hook_proc).expect("Failed to set mouse low-level hook");

    assert!(hook.state().is_set());
}
