# Windows Hook

[![Build](https://github.com/MaxMahem/windows_hook/actions/workflows/build.yml/badge.svg)](https://github.com/MaxMahem/windows_hook/actions/workflows/build.yml)
[![Docs](https://github.com/MaxMahem/windows_hook/actions/workflows/docs.yml/badge.svg)](https://MaxMahem.github.io/windows_hook/windows_hook/index.html)
[![dependency status](https://deps.rs/repo/github/MaxMahem/windows_hook/status.svg)](https://deps.rs/repo/github/MaxMahem/windows_hook)
[![codecov](https://codecov.io/github/MaxMahem/windows_hook/graph/badge.svg?token=EH5UONSSJA)](https://codecov.io/github/MaxMahem/windows_hook)
![GitHub License](https://img.shields.io/github/license/MaxMahem/windows_hook)

A safe wrapper around the Windows Hook API for Rust.

This crate provides a safe interface to the Windows Hook API, enabling you to set and unset hooks for various events, including keyboard and mouse input. The `WindowsHook` struct wraps the underlying `winsafe::HHOOK` type and automatically unsets itself when it goes out of scope.

A builder pattern is also provided, as well as wrappers for the [`Keyboard_LL`](https://learn.microsoft.com/en-us/windows/win32/winmsg/about-hooks#wh_keyboard_ll) (`KeyboardLLHook`) and [`Mouse_LL`](https://learn.microsoft.com/en-us/windows/win32/winmsg/about-hooks#wh_mouse_ll) (`MouseLLHook`) hooks.

## Why not use Winsafe directly?

These wrappers automate ownership and the drop of these hooks.

## Example Usage
```rust
use windows_hook::{WindowsHook, WH, Handle, HHOOK, Module, ThreadId};

extern "system" fn your_hook_fn(code: i32, wparam: usize, lparam: isize) -> isize {
    /// simple passthrough
    HHOOK::NULL.CallNextHookEx(unsafe { WH::from_raw(code) }, wparam, lparam)
}

let mut hook = WindowsHook::set_new(WH::KEYBOARD_LL, your_hook_fn, Module::NULL, ThreadId::NONE).unwrap();
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
    .with_module(Module::NULL)
    .with_thread_id(ThreadId::NONE)
    .build_and_set().unwrap();

assert!(hook.state().is_set());
```

## Additional Features
- `tracing`: Enable tracing support - reports errors when the hook is dropped

## Additional Types

- `Module`: A module handle, similar to (and convertable to) the `winsafe::HMODULE`/`HINSTANCE` type.
- `ThreadId`: A thread id wrapper.

## ToDo
[ ] Add additional support for hooks where it would be convienent to bundle the loaded library, and drop it with the hook.c
