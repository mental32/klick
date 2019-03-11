
#[macro_export]
macro_rules! kflag {
	() => ({
		use core::sync::atomic::Ordering;
		$crate::KFLAGS.load(Ordering::SeqCst)
	});

	($canary:expr, $error:expr) => ({
		use core::sync::atomic::Ordering;
		if $canary < $crate::KFLAGS.load(Ordering::SeqCst) { panic!($error) }
		$crate::KFLAGS.fetch_add(1, Ordering::SeqCst);
	})
}
