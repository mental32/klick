
#[doc(hidden)]
#[inline(always)]
pub fn wrap<F>(f: F) where F: FnOnce() -> () {
	f();
}
