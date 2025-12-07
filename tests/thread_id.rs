use std::fmt::Write;
use std::num::NonZeroU32;
use windows_hook::ThreadId;

#[test]
fn none_constant() {
    let none = ThreadId::NONE;
    assert!(none.is_none());
    assert_eq!(none.raw(), 0);
    assert_eq!(none.as_raw_option(), None);
    assert_eq!(*none, None);
}

#[test]
fn from_nonzero_value() {
    let tid = ThreadId::from(1);
    assert!(!tid.is_none());
    assert_eq!(tid.raw(), 1);
    assert_eq!(tid.as_raw_option(), Some(1));
    assert_eq!(*tid, Some(NonZeroU32::new(1).unwrap()));
}

#[test]
fn from_zero_value() {
    let tid = ThreadId::from(0);
    assert!(tid.is_none());
    assert_eq!(tid.raw(), 0);
    assert_eq!(tid.as_raw_option(), None);
    assert_eq!(*tid, None);
}

#[test]
fn display_trait() {
    let none = ThreadId::NONE;
    let some = ThreadId::from(42);

    let mut output = String::new();
    write!(&mut output, "{}", none).unwrap();
    assert_eq!(output, "0");

    output.clear();
    write!(&mut output, "{}", some).unwrap();
    assert_eq!(output, "42");
}

#[test]
fn equality_and_ordering() {
    let a = ThreadId::from(1);
    let b = ThreadId::from(2);
    let none = ThreadId::NONE;

    assert!(a < b);
    assert!(none < a);
    assert!(none < b);
    assert_eq!(a, ThreadId::from(1));
}

#[test]
fn current_thread_id() {
    let current = ThreadId::current();
    assert!(!current.is_none());
    assert!(current.raw() > 0);
}
