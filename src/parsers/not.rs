use crate::*;

#[derive(Debug, Copy, Clone)]
pub struct Not<P>(pub P);

impl<'a, P:Parser<'a>> Parser<'a> for Not<P> {
	type O = ();
	fn impl_parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<Self::O> {
		let rewind = *pos;
		let res = self.0.parse(ctx, limit, pos);
		*pos = rewind;

		match res {
			Some(_) => None,
			None => Some(()),
		}
	}
}
