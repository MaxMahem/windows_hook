#![doc = include_str!("../README.md")]

mod module;
mod thread_id;
mod windows_hook;
mod windows_hook_builder;

pub use module::*;
pub use thread_id::*;
pub use windows_hook::*;
pub use windows_hook_builder::*;

pub use winsafe::co::ERROR as SysError;
pub use winsafe::co::WH;
pub use winsafe::prelude::Handle;
pub use winsafe::{HHOOK, HINSTANCE, HOOKPROC};
