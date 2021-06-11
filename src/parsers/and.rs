use crate::*;

#[derive(Debug, Copy, Clone)]
pub struct And<P,F>(pub P, pub F);

impl<'a,P:Parser<'a>, F:Parser<'a>> Parser<'a> for And<P,F> {
	type O = P::O;
	fn impl_parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<Self::O> {
		let mut start = *pos;
		let res = self.0.parse(ctx, limit, pos);
		if res.is_none() {
			return None;
		}
		self.1.parse(ctx, limit, &mut start)?;

		res
	}
}