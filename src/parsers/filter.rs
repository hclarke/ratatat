use crate::*;

#[derive(Clone)]
pub struct Filter<P,F>(pub P, pub F);
#[derive(Clone)]
pub struct FilterMap<P,F>(pub P, pub F);

impl<'a, I:?Sized, P:Parser<'a, I>, F:Fn(&P::O) -> bool> Parser<'a, I> for Filter<P,F> {
	type O = P::O;
	fn parse(&self, ctx: &Context<'a, I>, limit: usize, pos: &mut usize) -> Option<Self::O> {
		let r = self.0.parse(ctx, limit, pos)?;
		if !self.1(&r) {
			return None;
		}

		Some(r)
	}
}

impl<'a, I:?Sized, T, P:Parser<'a, I>, F:Fn(P::O) -> Option<T>> Parser<'a, I> for FilterMap<P,F> {
	type O = T;
	fn parse(&self, ctx: &Context<'a, I>, limit: usize, pos: &mut usize) -> Option<Self::O> {
		let r = self.0.parse(ctx, limit, pos)?;
		self.1(r)
	}
}