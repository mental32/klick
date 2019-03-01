
#[macro_export]
macro_rules! hlt {
	() => (loop { x86_64::instructions::hlt() });
	($($arg:tt)*) => {
		print!($($arg)*);
		loop { x86_64::instructions::hlt() }
	}
}
