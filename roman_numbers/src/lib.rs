use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
	Nulla,
	I,
	V,
	X,
	L,
	C,
	D,
	M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
	fn from(value: u32) -> Self {
		match value {
			0 => RomanDigit::Nulla,
			1 => RomanDigit::I,
			5 => RomanDigit::V,
			10 => RomanDigit::X,
			50 => RomanDigit::L,
			100 => RomanDigit::C,
			500 => RomanDigit::D,
			1000 => RomanDigit::M,
			_ => panic!("Unsupported digit for RomanDigit"),
		}
	}
}

impl From<u32> for RomanNumber {
	fn from(mut num: u32) -> Self {
		if num == 0 {
			return RomanNumber(vec![RomanDigit::Nulla]);
		}

		let mut result = Vec::new();

		let mapping: &[(u32, &[RomanDigit])] = &[
			(1000, &[M]),
			(900, &[C, M]),
			(500, &[D]),
			(400, &[C, D]),
			(100, &[C]),
			(90, &[X, C]),
			(50, &[L]),
			(40, &[X, L]),
			(10, &[X]),
			(9, &[I, X]),
			(5, &[V]),
			(4, &[I, V]),
			(1, &[I]),
		];

		for &(value, digits) in mapping {
			while num >= value {
				result.extend_from_slice(digits);
				num -= value;
			}
		}

		RomanNumber(result)
	}
}
