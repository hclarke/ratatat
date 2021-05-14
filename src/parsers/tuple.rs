use crate::*;

macro_rules! impl_tuple {
	($($i:tt:$P:tt),*) => {
		impl<'a, I:?Sized $(,$P:Parser<'a, I>)*> Parser<'a, I> for ($($P,)*) {
			type O = ($($P::O,)*);
			fn parse(&self, ctx: &Context<'a, I>, limit: usize, pos: &mut usize) -> Option<Self::O> {
				Some(($(self.$i.parse(ctx, limit, pos)?,)*))
			}
		}

		impl<'a, I:?Sized $(,$P:Generator<'a, I>)*> Generator<'a, I> for ($($P,)*) {
			type O = ($($P::O,)*);
			fn generate(ctx: &Context<'a, I>) -> Rc<dyn Parser<'a, I, O=Self::O>+'a> {
				let rc = Rc::new(($(
					ctx.parser::<$P>().clone(),
				)*));

				rc as Rc<dyn Parser<'a, I, O=Self::O>+'a>
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
		let parser = ("hello", " ", "world");
		let result = parser.parse_str("hello world");
		assert_eq!(Some(("hello", " ", "world")), result);
	}
}

