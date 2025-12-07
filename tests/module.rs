use fluent_result::expect::ExpectNone;
use windows_hook::{HINSTANCE, Handle, Module, SysError};

#[test]
#[allow(non_snake_case)]
fn module_from_HINSTANCE() -> Result<(), SysError> {
    let module = Module::from(HINSTANCE::NULL);
    assert!(module.is_null(), "Null instance should return null");

    let module: Module = HINSTANCE::GetModuleHandle(None)?.into();
    assert!(!module.is_null(), "Valid instance should not return null");

    Ok(())
}

#[test]
fn module_from_ptr() -> Result<(), SysError> {
    let module = unsafe { Module::from_ptr(std::ptr::null_mut()) };
    assert!(module.is_null(), "Null pointer should return null");

    let instance = HINSTANCE::GetModuleHandle(None)?;
    let module = unsafe { Module::from_ptr(instance.ptr()) };

    assert!(!module.is_null(), "Valid pointer should not return null");
    assert_eq!(instance.ptr(), module.ptr(), "Pointer should match");

    Ok(())
}

#[test]
fn is_invalid_checks() -> Result<(), SysError> {
    assert!(
        Module::INVALID.is_invalid(),
        "Invalid module should be invalid"
    );
    assert!(
        !Module::NULL.is_invalid(),
        "Null module should not be invalid"
    );
    assert!(
        !Module::current()?.is_invalid(),
        "Current module should not be invalid"
    );
    Ok(())
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
    let module = Module::get_by_name("kernel32.dll")?;
    assert!(
        !module.is_null(),
        "Valid module name should not return null"
    );
    assert!(
        !module.is_invalid(),
        "Valid module name should not return invalid"
    );
    Ok(())
}

#[test]
fn get_by_name_invalid_module() {
    let result = Module::get_by_name("nonexistent_module_12345.dll");
    assert!(
        result.is_err(),
        "Invalid module name should return an error"
    );
}

#[test]
fn as_mut() -> Result<(), SysError> {
    let mut module = Module::current()?;
    let original_ptr = module.ptr();

    let ptr_ref = unsafe { module.as_mut() };
    assert_eq!(*ptr_ref, original_ptr, "Pointer should match");

    let null_ptr = std::ptr::null_mut();
    *ptr_ref = null_ptr;

    assert_eq!(module.ptr(), null_ptr, "Pointer should be null");
    assert!(module.is_null(), "Module should be null");

    Ok(())
}

#[test]
#[allow(non_snake_case)]
fn as_HINSTANCE() -> Result<(), SysError> {
    Module::NULL
        .as_HINSTANCE()
        .expect_none("Null module should return None");

    let current_module = Module::current()?;
    let hinstance = current_module.as_HINSTANCE();
    assert!(
        hinstance.is_some(),
        "Valid module should return Some(HINSTANCE)"
    );

    // Verify the HINSTANCE matches the original pointer
    let hinstance = hinstance.unwrap();
    assert_eq!(
        hinstance.ptr(),
        current_module.ptr(),
        "HINSTANCE pointer should match module pointer"
    );

    Ok(())
}
