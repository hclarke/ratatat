use crate::*;
use std::rc::Weak;
impl<'a, I:?Sized, P:Parser<'a, I>+?Sized> Parser<'a, I> for Rc<P> {
	type O = P::O;
	fn parse(&self, ctx: &Context<'a, I>, limit: usize, pos: &mut usize) -> Option<Self::O> {
		(&**self).parse(ctx, limit, pos)
	}
}

impl<'a, I:?Sized, P:Parser<'a, I>+?Sized> Parser<'a, I> for Weak<P> {
	type O = P::O;
	fn parse(&self, ctx: &Context<'a, I>, limit: usize, pos: &mut usize) -> Option<Self::O> {
		let rc = self.upgrade().unwrap();
		rc.parse(ctx, limit, pos)
	}
}