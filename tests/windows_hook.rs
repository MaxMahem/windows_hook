use windows_hook::{Module, ThreadId, WH, WindowsHook};

// Dummy hook procedure for testing
extern "system" fn dummy_hook_proc(_code: i32, _wparam: usize, _lparam: isize) -> isize {
    0
}

#[test]
fn hook_creation_sets_state() {
    let hook = WindowsHook::set_new(
        WH::KEYBOARD_LL,
        dummy_hook_proc,
        Module::NULL,
        ThreadId::NONE,
    )
    .expect("Failed to set hook");
    assert!(hook.state().is_set());
}

#[test]
fn hook_unset_transitions_state() {
    let mut hook = WindowsHook::set_new(
        WH::KEYBOARD_LL,
        dummy_hook_proc,
        Module::NULL,
        ThreadId::NONE,
    )
    .expect("Failed to set hook");
    let previous = hook.unset().expect("Failed to unset hook");
    assert!(previous.is_set());
    assert!(hook.state().is_unset());
}

#[test]
fn unset_twice_is_safe() {
    let mut hook = WindowsHook::set_new(
        WH::KEYBOARD_LL,
        dummy_hook_proc,
        Module::NULL,
        ThreadId::NONE,
    )
    .expect("Failed to set hook");
    hook.unset().expect("First unset failed");
    let second = hook.unset().expect("Second unset failed");
    assert!(second.is_unset());
}

#[test]
fn default_hook_is_unset() {
    let hook = WindowsHook::default();
    assert!(hook.state().is_unset());
}
