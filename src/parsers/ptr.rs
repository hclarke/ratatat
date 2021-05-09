use crate::*;
use std::rc::Weak;
impl<'a, I:?Sized, P:Parser<'a, I>> Parser<'a, I> for Rc<P> {
	type O = P::O;
	fn parse(&self, ctx: &Context<'a, I>, src: &mut &'a [u8]) -> Option<Self::O> {
		(&**self).parse(ctx, src)
	}
}

impl<'a, I:?Sized, P:Parser<'a, I>> Parser<'a, I> for Weak<P> {
	type O = P::O;
	fn parse(&self, ctx: &Context<'a, I>, src: &mut &'a [u8]) -> Option<Self::O> {
		let rc = self.upgrade().unwrap();
		rc.parse(ctx, src)
	}
}