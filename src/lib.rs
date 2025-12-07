#![doc = include_str!("../README.md")]

#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]

#[cfg(not(windows))]
compile_error!("This crate only supports Windows targets.");

mod module;
mod thread_id;
mod windows_hook;

pub use module::Module;
pub use thread_id::ThreadId;
pub use windows_hook::{HookState, KeyboardLLHook, MouseLLHook, WindowsHook, WindowsHookBuilder};

pub use winsafe::co::ERROR as SysError;
pub use winsafe::co::WH;
pub use winsafe::prelude::Handle;
pub use winsafe::{HHOOK, HINSTANCE, HOOKPROC};
