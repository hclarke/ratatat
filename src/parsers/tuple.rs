use crate::*;

#[derive(Debug, Copy, Clone)]
pub struct Alt<T>(pub T);

macro_rules! impl_tuple {
	($($i:tt:$P:tt),*) => {
		impl<'a $(,$P:Parser<'a>)*> Parser<'a> for ($($P,)*) {
			type O = ($($P::O,)*);
			fn impl_parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<Self::O> {
				Some(($(self.$i.parse(ctx, limit, pos)?,)*))
			}

			fn name(&self) -> String {
				"()".to_owned()
			}
		}

		impl<'a $(,$P:Generator<'a>)*> Generator<'a> for ($($P,)*) {
			type O = ($($P::O,)*);
			fn generate(ctx: &Context<'a>) -> Rc<dyn Parser<'a, O=Self::O>+'a> {
				let rc = Rc::new(($(
					ctx.parser::<$P>().clone(),
				)*));

				rc as Rc<dyn Parser<'a, O=Self::O>+'a>
			}
		}

		impl<'a, O:Debug, $($P:Parser<'a, O=O>),*> Parser<'a> for Alt<($($P,)*)> {
			type O = O;
			fn impl_parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<Self::O> {
				let reset = *pos;
				$(
					if let Some(res) = self.0.$i.parse(ctx, limit, pos) {
						return Some(res);
					}

					*pos = reset;
				)*

				None
			}

			fn name(&self) -> String {
				"Alt".to_owned()
			}
		}

		impl<'a, O:Debug+'a, $($P:Generator<'a, O=O>),*> Generator<'a> for Alt<($($P,)*)> {
			type O=O;
			fn generate(ctx: &Context<'a>) -> Rc<DynParser<'a, Self::O>> {
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

impl<'a> Parser<'a> for () {
	type O = ();
	fn impl_parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<Self::O> {
		Some(())
	}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_tuple() {
        assert_parse!(
            Some(("hello", " ", "world")),
            ("hello", " ", "world"),
            "hello world"
        );
    }
}
