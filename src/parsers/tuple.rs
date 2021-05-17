use crate::*;

pub struct Alt<T>(pub T);

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

		impl<'a, I:?Sized, O, $($P:Parser<'a, I, O=O>),*> Parser<'a, I> for Alt<($($P,)*)> {
			type O = O;
			fn parse(&self, ctx: &Context<'a, I>, limit: usize, pos: &mut usize) -> Option<Self::O> {
				let reset = *pos;
				$(
					if let Some(res) = self.0.$i.parse(ctx, limit, pos) {
						return Some(res);
					}

					*pos = reset;
				)*

				None
			}
		}

		impl<'a, I:?Sized, O:'a, $($P:Generator<'a, I, O=O>),*> Generator<'a, I> for Alt<($($P,)*)> {
			type O=O;
			fn generate(ctx: &Context<'a, I>) -> Rc<DynParser<'a, I, Self::O>> {
				Rc::new(Alt(($(
					ctx.parser::<$P>().clone(),
				)*)))
			}
		}
	}
}

impl_tuple!(0: P0);
impl_tuple!(0: P0, 1: P1);
impl_tuple!(0: P0, 1: P1, 2: P2);
impl_tuple!(0: P0, 1: P1, 2: P2, 3: P3);
impl_tuple!(0: P0, 1: P1, 2: P2, 3: P3, 4: P4);
impl_tuple!(0: P0, 1: P1, 2: P2, 3: P3, 4: P4, 5: P5);
impl_tuple!(0: P0, 1: P1, 2: P2, 3: P3, 4: P4, 5: P5, 6: P6);
impl_tuple!(0: P0, 1: P1, 2: P2, 3: P3, 4: P4, 5: P5, 6: P6, 7: P7);
impl_tuple!(
    0: P0,
    1: P1,
    2: P2,
    3: P3,
    4: P4,
    5: P5,
    6: P6,
    7: P7,
    8: P8
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_tuple() {
        assert_parse!(Some(("hello", " ", "world")), ("hello", " ", "world"), "hello world");
    }
}
