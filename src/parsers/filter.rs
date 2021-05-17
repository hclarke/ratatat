use crate::*;

#[derive(Clone, Copy)]
pub struct Filter<P,F>(pub P, pub F);
#[derive(Clone, Copy)]
pub struct FilterMap<P,F>(pub P, pub F);

impl<'a, P:Parser<'a>, F:Fn(&P::O) -> bool> Parser<'a> for Filter<P,F> {
	type O = P::O;
	fn parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<Self::O> {
		let r = self.0.parse(ctx, limit, pos)?;
		if !self.1(&r) {
			return None;
		}

		Some(r)
	}
}

impl<'a, T, P:Parser<'a>, F:Fn(P::O) -> Option<T>> Parser<'a> for FilterMap<P,F> {
	type O = T;
	fn parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<Self::O> {
		let r = self.0.parse(ctx, limit, pos)?;
		self.1(r)
	}
}