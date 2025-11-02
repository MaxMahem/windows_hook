use std::num::NonZeroU32;

use tap::Pipe;

/// A thread id. This is a wrapper around [`Option<NonZeroU32>`](NonZeroU32).
/// Where a [`None`] value represents no thread id, or `0`.
///
/// # Examples
///
/// ```rust
/// use windows_hook::ThreadId;
///
/// let none = ThreadId::NONE;
/// let some = ThreadId::from(1);
///
/// assert_eq!(None, *none);
/// assert_eq!(Some(1), some.as_raw_option());
///
/// assert_eq!(0, none.raw());
/// assert_eq!(1, some.raw());
/// ```
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, derive_more::Deref)]
pub struct ThreadId(Option<NonZeroU32>);

impl ThreadId {
    /// A thread id that represents no thread id, or `0`.
    pub const NONE: ThreadId = ThreadId(None);

    /// Returns the current thread id as a [ThreadId].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use windows_hook::ThreadId;
    ///
    /// assert!(ThreadId::NONE != ThreadId::current());
    /// ```
    pub fn current() -> Self {
        winsafe::GetCurrentThreadId().pipe(ThreadId::from)
    }

    /// Returns `true` if the thread id is none, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use windows_hook::ThreadId;
    ///
    /// assert_eq!(true, ThreadId::NONE.is_none());
    /// assert_eq!(false, ThreadId::from(1).is_none());
    /// ```
    pub fn is_none(&self) -> bool {
        self.0.is_none()
    }

    /// Returns the thread id as a raw value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use windows_hook::ThreadId;
    ///
    /// assert_eq!(0, ThreadId::NONE.raw());
    /// assert_eq!(1, ThreadId::from(1).raw());
    /// ```
    pub fn raw(&self) -> u32 {
        self.0.map(NonZeroU32::get).unwrap_or(0)
    }

    /// Returns the thread id as a [`Option<u32>`](u32).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use windows_hook::ThreadId;
    ///
    /// assert_eq!(None, ThreadId::NONE.as_raw_option());
    /// assert_eq!(Some(1), ThreadId::from(1).as_raw_option());
    /// ```
    pub fn as_raw_option(&self) -> Option<u32> {
        self.0.map(NonZeroU32::get)
    }
}

impl From<u32> for ThreadId {
    fn from(id: u32) -> Self {
        NonZeroU32::new(id).pipe(Self)
    }
}

impl std::fmt::Display for ThreadId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.raw().fmt(f)
    }
}
