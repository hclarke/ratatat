use crate::*;

impl<'a, I:?Sized, P:Parser<'a, I>> Parser<'a, I> for [P] {
	type O = P::O;
	fn parse(&self, ctx: &Context<'a, I>, src: &mut &'a [u8]) -> Option<Self::O> {
		let reset = *src;
		for parser in self {
			match parser.parse(ctx, src) {
				Some(x) => return Some(x),
				None => {
					*src = reset;
				}
			}
		}

		None
	}
}