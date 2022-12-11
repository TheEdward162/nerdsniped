use std::str::FromStr;

pub struct FromStrToTryFromAdapter<T: FromStr>(pub T);
impl<'a, T: FromStr> TryFrom<&'a str> for FromStrToTryFromAdapter<T> {
	type Error = T::Err;

	fn try_from(value: &'a str) -> Result<Self, Self::Error> {
		let inner = value.parse::<T>()?;

		Ok(Self(inner))
	}
}

#[macro_export]
macro_rules! split_match_tokens {
	(
		$input: expr, $pattern: expr;
		$($rest: tt)+
	) => {
		{
			let mut iter = $input.split($pattern);
			$crate::split_match_tokens!(__internal iter; $($rest)+ ,)
		}
	};

	// literal
	(
		__internal $iter: expr $(, $partial: expr)*;
		$head: literal, $($rest: tt)*
	) => {
		match $iter.next() {
			Some($head) => $crate::split_match_tokens!(__internal $iter $(, $partial)*; $($rest)*),
			res => Err($crate::anyhow::anyhow!(concat!("Expected literal \"", $head, "\" but found {:?}"), res))
		}
	};

	// ident
	(
		__internal $iter: expr $(, $partial: expr)*;
		$head: ident: $head_ty: ty, $($rest: tt)*
	) => {
		match $iter.next() {
			Some($head) => match <$head_ty>::try_from($head) {
				Ok($head) => $crate::split_match_tokens!(__internal $iter $(, $partial)*, $head; $($rest)*),
				Err(err) => Err($crate::anyhow::anyhow!(concat!("Failed to parse \"{}\" as ", stringify!($head_ty), ": {}"), $head, err))
			},
			res => Err($crate::anyhow::anyhow!(concat!("Expected ", stringify!($head), " but found {:?}"), res))
		}
	};

	// terminal
	(
		__internal $iter: expr $(, $partial: expr)+;
		$(,)*
	) => {
		Ok(($($partial),+))
	};
}