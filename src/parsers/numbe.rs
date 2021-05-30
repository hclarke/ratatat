use crate::*;

use num::*;
use num::bigint::Sign;

#[derive(Debug, Clone, Copy)]
pub struct Digit(pub u32);


impl<'a> Parser<'a> for Digit {
	type O = u32;
	fn impl_parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<Self::O> {
		let c = ctx.parse::<char>(limit, pos)?;
		c.to_digit(self.0)
	}
}

impl<'a> Generator<'a> for BigInt {
	type O = BigInt;
	fn generate(ctx: &Context<'a>) -> Rc<DynParser<'a,Self::O>> {
		let sign = Alt((
			'-'.map(|_| -1), 
			'+'.map(|_| 1),
			().map(|_| 1),
		));

		let radix = Alt((
			("0x",).map(|_| 16),
			("0o",).map(|_| 8),
			("0b",).map(|_| 2),
			().map(|_| 10),
		));

		let parser = (sign,radix).then(|(s,r)| {
			Digit(r)
				.many(1..)
				.map(move |digits| {
					let mut val = BigInt::zero();
					for d in digits {
						val *= r;
						val += d;
					}
					val * s
				})
		});

		Rc::new(parser)
	}
}

impl<'a> Generator<'a> for u8 {
	type O = u8;
	fn generate(ctx: &Context<'a>) -> Rc<DynParser<'a, Self::O>> {
		let parser = ctx.parser::<BigInt>().clone()
			.filter_map(|x| x.to_u8());
		Rc::new(parser)
	}
}

impl<'a> Generator<'a> for u16 {
	type O = u16;
	fn generate(ctx: &Context<'a>) -> Rc<DynParser<'a, Self::O>> {
		let parser = ctx.parser::<BigInt>().clone()
			.filter_map(|x| x.to_u16());
		Rc::new(parser)
	}
}

impl<'a> Generator<'a> for u32 {
	type O = u32;
	fn generate(ctx: &Context<'a>) -> Rc<DynParser<'a, Self::O>> {
		let parser = ctx.parser::<BigInt>().clone()
			.filter_map(|x| x.to_u32());
		Rc::new(parser)
	}
}

impl<'a> Generator<'a> for u64 {
	type O = u64;
	fn generate(ctx: &Context<'a>) -> Rc<DynParser<'a, Self::O>> {
		let parser = ctx.parser::<BigInt>().clone()
			.filter_map(|x| x.to_u64());
		Rc::new(parser)
	}
}

impl<'a> Generator<'a> for u128 {
	type O = u128;
	fn generate(ctx: &Context<'a>) -> Rc<DynParser<'a, Self::O>> {
		let parser = ctx.parser::<BigInt>().clone()
			.filter_map(|x| x.to_u128());
		Rc::new(parser)
	}
}

impl<'a> Generator<'a> for usize {
	type O = usize;
	fn generate(ctx: &Context<'a>) -> Rc<DynParser<'a, Self::O>> {
		let parser = ctx.parser::<BigInt>().clone()
			.filter_map(|x| x.to_usize());
		Rc::new(parser)
	}
}

impl<'a> Generator<'a> for i8 {
	type O = i8;
	fn generate(ctx: &Context<'a>) -> Rc<DynParser<'a, Self::O>> {
		let parser = ctx.parser::<BigInt>().clone()
			.filter_map(|x| x.to_i8());
		Rc::new(parser)
	}
}

impl<'a> Generator<'a> for i16 {
	type O = i16;
	fn generate(ctx: &Context<'a>) -> Rc<DynParser<'a, Self::O>> {
		let parser = ctx.parser::<BigInt>().clone()
			.filter_map(|x| x.to_i16());
		Rc::new(parser)
	}
}

impl<'a> Generator<'a> for i32 {
	type O = i32;
	fn generate(ctx: &Context<'a>) -> Rc<DynParser<'a, Self::O>> {
		let parser = ctx.parser::<BigInt>().clone()
			.filter_map(|x| x.to_i32());
		Rc::new(parser)
	}
}

impl<'a> Generator<'a> for i64 {
	type O = i64;
	fn generate(ctx: &Context<'a>) -> Rc<DynParser<'a, Self::O>> {
		let parser = ctx.parser::<BigInt>().clone()
			.filter_map(|x| x.to_i64());
		Rc::new(parser)
	}
}

impl<'a> Generator<'a> for i128 {
	type O = i128;
	fn generate(ctx: &Context<'a>) -> Rc<DynParser<'a, Self::O>> {
		let parser = ctx.parser::<BigInt>().clone()
			.filter_map(|x| x.to_i128());
		Rc::new(parser)
	}
}

impl<'a> Generator<'a> for isize {
	type O = isize;
	fn generate(ctx: &Context<'a>) -> Rc<DynParser<'a, Self::O>> {
		let parser = ctx.parser::<BigInt>().clone()
			.filter_map(|x| x.to_isize());
		Rc::new(parser)
	}
}


#[cfg(test)]
mod test {
    use super::*;
    use proptest::*;

    #[test]
    fn bigint_parser() {
        assert_parse!(
        	BigInt::from_radix_be(Sign::Plus, &[1], 10), 
        	parser::<BigInt>(), 
        	"1"
        );

        assert_parse!(
        	BigInt::from_radix_be(Sign::Plus, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 0], 10), 
        	parser::<BigInt>(), 
        	"12345678987654321000"
        );

        assert_parse!(
        	BigInt::from_radix_be(Sign::Plus, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, 0], 10), 
        	parser::<BigInt>(), 
        	"0012345678987654321000"
        );

        assert_parse!(
        	BigInt::from_radix_be(Sign::Plus, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0, 0, 0], 16), 
        	parser::<BigInt>(), 
        	"0x123456789AbCdEf000"
        );

         assert_parse!(
        	BigInt::from_radix_be(Sign::Plus, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0, 0, 0], 16), 
        	parser::<BigInt>(), 
        	"+0x123456789AbCdEf000"
        );

         assert_parse!(
        	BigInt::from_radix_be(Sign::Minus, &[1, 2, 3, 4, 5, 10, 11, 12], 16), 
        	parser::<BigInt>(), 
        	"-0x12345ABC"
        );

         assert_parse!(
        	BigInt::from_radix_be(Sign::Minus, &[1, 2, 3, 4, 5, 6, 7, 8], 10), 
        	parser::<BigInt>(), 
        	"-00012345678"
        );

         assert_parse!(
        	BigInt::from_radix_be(Sign::Minus, &[1, 1, 0, 0, 1, 0, 1], 2), 
        	parser::<BigInt>(), 
        	"-0b1100101"
        );
    }

    #[test]
    fn u8_parser() {
    	assert_parse!(Some(0u8), parser::<u8>(), "0");
    	assert_parse!(Some(1u8), parser::<u8>(), "1");

    	assert_parse!(Some(255u8), parser::<u8>(), "255");
    	assert_parse!(Some(255u8), parser::<u8>(), "0xFF");
    	assert_parse!(Some(255u8), parser::<u8>(), "0o377");
    	assert_parse!(Some(255u8), parser::<u8>(), "0b11111111");
    }

    proptest! {
    	#[test]
    	fn proptest_u8_dec(x in 0u8..=0xFFu8) {
    		let s = format!("{}", x);
    		assert_parse!(Some(x), parser::<u8>(), &s[..]);
    	}
    }

    proptest! {
    	#[test]
    	fn proptest_u8_hex(x in 0u8..=0xFFu8) {
    		let s = format!("{:#x}", x);
    		println!("{}", &s);
    		assert_parse!(Some(x), parser::<u8>(), &s[..]);
    	}
    }

    proptest! {
    	#[test]
    	fn proptest_u8_oct(x in 0u8..=0xFFu8) {
    		let s = format!("{:#o}", x);
    		println!("{}", &s);
    		assert_parse!(Some(x), parser::<u8>(), &s[..]);
    	}
    }

    proptest! {
    	#[test]
    	fn proptest_u8_bin(x in 0u8..=0xFFu8) {
    		let s = format!("{:#b}", x);
    		println!("{}", &s);
    		assert_parse!(Some(x), parser::<u8>(), &s[..]);
    	}
    }

    proptest! {
    	#[test]
    	fn proptest_i32_dec(x in std::i32::MIN..=std::i32::MAX) {
    		let s = format!("{}", x);
    		assert_parse!(Some(x), parser::<i32>(), &s[..]);
    	}
    }

    proptest! {
    	#[test]
    	fn proptest_usize_dec(x in std::usize::MIN..=std::usize::MAX) {
    		let s = format!("{}", x);
    		assert_parse!(Some(x), parser::<usize>(), &s[..]);
    	}
    }

    proptest! {
    	#[test]
    	fn proptest_isize_dec(x in std::isize::MIN..=std::isize::MAX) {
    		let s = format!("{}", x);
    		assert_parse!(Some(x), parser::<isize>(), &s[..]);
    	}
    }
}