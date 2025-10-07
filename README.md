# Windows Hook
A safe wrapper around the Windows Hook API for Rust.

This crate provides a safe interface to the Windows Hook API, allowing you to set and unset hooks for various events such as keyboard and mouse input. The `WindowsHook` struct wraps the underlying `winsafe::HHOOK` type and automatically unsets itself when it goes out of scope.

A builder pattern is also provided, as well as wrappers for the [`Keyboard_LL`](https://learn.microsoft.com/en-us/windows/win32/winmsg/about-hooks#wh_keyboard_ll) (`KeyboardLLHook`) and [`Mouse_LL`](https://learn.microsoft.com/en-us/windows/win32/winmsg/about-hooks#wh_mouse_ll) (`MouseLLHook`) hooks.

## Why not use Winsafe directly?

These wrapers automate ownership and drop of these hooks.

## Example Usage
```rust
use windows_hook::{WindowsHook, WH, Handle, HHOOK, Module, ThreadId};

extern "system" fn your_hook_fn(code: i32, wparam: usize, lparam: isize) -> isize {
    /// simple passthrough
    HHOOK::NULL.CallNextHookEx(unsafe { WH::from_raw(code) }, wparam, lparam)
}

let hook = WindowsHook::set_new(WH::KEYBOARD_LL, your_hook_fn, Module::NULL, ThreadId::NONE).unwrap();
assert!(hook.state().is_set());

// hook will unset when it drops out of scope, but it can also be unset manually.
hook.unset().unwrap();
```

Builder pattern:
```rust
use windows_hook::{WindowsHookBuilder, WH, Handle, HHOOK, Module, ThreadId};

extern "system" fn your_hook_fn(code: i32, wparam: usize, lparam: isize) -> isize {
    /// simple passthrough
    HHOOK::NULL.CallNextHookEx(unsafe { WH::from_raw(code) }, wparam, lparam)
}

let hook = WindowsHookBuilder::new(WH::KEYBOARD_LL, your_hook_fn)
    .with_module(winsafe::HINSTANCE::NULL)
    .with_thread_id(winsafe::ThreadId::NONE)
    .build_and_set().unwrap();

assert!(hook.state().is_set());
```

## Additional Features
- `tracing`: Enable tracing support - reports errors when the hook is dropped

## Additional Types

- `Module`: A module handle, similar to (and convertable to) the `winsafe::HMODULE`/`HINSTANCE` type.
- `ThreadId`: A thread id wrapper.

## ToDo
- Add additional support for hooks where it would be convienent to bundle the loaded library, and drop it with the hook.

## License
MIT