use crate::*;

macro_rules! impl_tuple {
	($($i:tt:$P:tt),*) => {
		impl<'a, I:?Sized $(,$P:Parser<'a, I>)*> Parser<'a, I> for ($($P,)*) {
			type O = ($($P::O,)*);
			fn parse(&self, ctx: &Context<'a, I>, src: &mut &'a [u8]) -> Option<Self::O> {
				Some(($(self.$i.parse(ctx, src)?,)*))
			}
		}
	}
}

impl_tuple!(0:P0);
impl_tuple!(0:P0, 1:P1);
impl_tuple!(0:P0, 1:P1, 2:P2);
impl_tuple!(0:P0, 1:P1, 2:P2, 3:P3);
impl_tuple!(0:P0, 1:P1, 2:P2, 3:P3, 4:P4);
impl_tuple!(0:P0, 1:P1, 2:P2, 3:P3, 4:P4, 5:P5);
impl_tuple!(0:P0, 1:P1, 2:P2, 3:P3, 4:P4, 5:P5, 6:P6);
impl_tuple!(0:P0, 1:P1, 2:P2, 3:P3, 4:P4, 5:P5, 6:P6, 7:P7);
impl_tuple!(0:P0, 1:P1, 2:P2, 3:P3, 4:P4, 5:P5, 6:P6, 7:P7, 8:P8);


#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn parse_tuple() {
		let mut src = &b"hello world"[..];
		let ctx = Context::new(src);

		assert_eq!(Some(("hello", " ", "world")), ("hello", " ", "world").parse(&ctx, &mut src));
		assert_eq!(b"", src);
	}
}

