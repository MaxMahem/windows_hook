use windows_hook::{HINSTANCE, Handle, Module, SysError};

#[test]
#[allow(non_snake_case)]
fn module_from_HINSTANCE() -> Result<(), SysError> {
    let module = Module::from(HINSTANCE::NULL);
    assert!(module.is_null());

    let module: Module = HINSTANCE::GetModuleHandle(None)?.into();
    assert!(!module.is_null());

    Ok(())
}

#[test]
fn module_from_ptr() -> Result<(), SysError> {
    let module = unsafe { Module::from_ptr(std::ptr::null_mut()) };
    assert!(module.is_null());

    let instance = HINSTANCE::GetModuleHandle(None)?;
    let module = unsafe { Module::from_ptr(instance.ptr()) };

    assert!(!module.is_null());
    assert_eq!(instance.ptr(), module.ptr());

    Ok(())
}

#[test]
fn is_invalid_checks() {
    assert!(Module::INVALID.is_invalid());
    assert!(!Module::NULL.is_invalid());
    assert!(
        !Module::current()
            .expect("Current module should be valid")
            .is_invalid()
    );
}

#[test]
fn module_debug() -> Result<(), SysError> {
    let module = Module::current()?;
    let hinstance = HINSTANCE::GetModuleHandle(None)?;

    // trim the names which are differnt
    let module_dbg = format!("{:?}", module);
    let Some((module_dbg, _)) = module_dbg.rsplit_once(' ') else {
        panic!("Failed to get module debug");
    };
    let hinstance_dbg = format!("{:?}", hinstance);
    let Some((hinstance_dbg, _)) = hinstance_dbg.rsplit_once(' ') else {
        panic!("Failed to get hinstance debug");
    };

    assert_eq!(module_dbg, hinstance_dbg);
    Ok(())
}

#[test]
fn get_by_name_valid_module() -> Result<(), SysError> {
    // kernel32.dll is always loaded in a Windows process
    let module = Module::get_by_name("kernel32.dll")?;
    assert!(!module.is_null());
    assert!(!module.is_invalid());
    Ok(())
}

#[test]
fn get_by_name_invalid_module() {
    // A module that doesn't exist should return an error
    let result = Module::get_by_name("nonexistent_module_12345.dll");
    assert!(result.is_err());
}
