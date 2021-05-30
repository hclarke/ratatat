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


#[cfg(test)]
mod test {
    use super::*;

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
}