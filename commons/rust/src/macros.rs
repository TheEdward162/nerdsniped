use std::str::FromStr;

pub struct FromStrToTryFromAdapter<T: FromStr>(pub T);
impl<'a, T: FromStr> TryFrom<&'a str> for FromStrToTryFromAdapter<T> {
	type Error = T::Err;

	fn try_from(value: &'a str) -> Result<Self, Self::Error> {
		let inner = value.parse::<T>()?;

		Ok(Self(inner))
	}
}

/// Match str tokens from the iterator.
/// 
/// Usage:
/// ```
/// # use aoc_commons::{macros::FromStrToTryFromAdapter, match_tokens};
/// #
/// let value = "move 5 from 0 to 17, 21";
/// let (count, from, to) = match_tokens!(value.split([' ', ',']).filter(|s| !s.is_empty()); "move", count: FromStrToTryFromAdapter<usize> {.0}, "from", from: &str, "to", ...to: Vec<FromStrToTryFromAdapter<usize>> {.0}).unwrap();
/// 
/// assert_eq!(count, 5usize);
/// assert_eq!(from, "0");
/// assert_eq!(to, vec![17, 21]);
/// ```
#[macro_export]
macro_rules! match_tokens {
	(
		$input: expr;
		$($rest: tt)+
	) => {
		{
			let mut iter = $input;
			$crate::match_tokens!(__internal iter; $($rest)+ ,)
		}
	};

	// literal
	(
		__internal $iter: expr $(, $partial: expr)*;
		$head: literal $(| $heads: literal )*, $($rest: tt)*
	) => {
		match $iter.next() {
			Some($head $(| $heads)*) => $crate::match_tokens!(__internal $iter $(, $partial)*; $($rest)*),
			res => Err($crate::anyhow::anyhow!(concat!("Expected literal \"", $head, "\" but found {:?}"), res))
		}
	};

	// ident-rest
	(
		__internal $iter: expr $(, $partial: expr)*;
		...$head: ident: Vec<$head_ty: ty> $({ $($accessor: tt)+ })? $(,)?
	) => {
		{
			let mut $head = Vec::new();
			let result = loop {
				match $iter.next() {
					None => break Ok(()),
					Some(val) => match <$head_ty>::try_from(val) {
						Ok(val) => { $head.push(val $($($accessor)+)?); },
						Err(err) => break Err($crate::anyhow::anyhow!(concat!("Failed to parse \"{}\" as ", stringify!($head_ty), ": {}"), val, err))
					}
				}
			};

			match result {
				Ok(()) => $crate::match_tokens!(__internal $iter $(, $partial)*, $head;),
				Err(err) => Err(err)
			}
		}
	};

	// ident
	(
		__internal $iter: expr $(, $partial: expr)*;
		$head: ident: $head_ty: ty $({ $($accessor: tt)+ })?, $($rest: tt)*
	) => {
		match $iter.next() {
			Some($head) => match <$head_ty>::try_from($head) {
				Ok($head) => $crate::match_tokens!(__internal $iter $(, $partial)*, $head $($($accessor)+)?; $($rest)*),
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