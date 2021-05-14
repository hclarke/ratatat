use crate::*;

impl<'a, I:?Sized, P:Parser<'a, I>> Parser<'a, I> for [P] {
	type O = P::O;
	fn parse(&self, ctx: &Context<'a, I>, limit: usize, pos: &mut usize) -> Option<Self::O> {
		let reset = *pos;
		for parser in self {
			match parser.parse(ctx, limit, pos) {
				Some(x) => return Some(x),
				None => {
					*pos = reset;
				}
			}
		}
		None
	}
}