
#[macro_export]
macro_rules! kflag {
	($canary:expr, $error:expr) => ({
		use $crate::utils::WriteOnceBitField;
		if let Err(_) = (*$crate::KFLAGS.lock()).set($canary) { panic!($error) }
	})
}

#[macro_export]
macro_rules! kflagcanary {
	() => ({
		use $crate::utils::WriteOnceBitField;
		(*$crate::KFLAGS.lock()).canary().unwrap()
	})
}
