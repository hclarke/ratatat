use crate::*;

pub struct Recognize<T>(pub T);

impl<'a, I:?Sized, P:Parser<'a, I>> Parser<'a, I> for Recognize<P> {
	type O = &'a [u8];
	fn parse(&self, ctx: &Context<'a, I>, limit: usize, pos: &mut usize) -> Option<Self::O> {
		let start = *pos;
		let _ = self.0.parse(ctx, limit, pos)?;
		let end = *pos;

		Some(&ctx.bytes[start..end])
	}
}

impl<'a, I:?Sized, G:Generator<'a, I>> Generator<'a, I> for Recognize<G> {
	type O = &'a [u8];
	fn generate(ctx: &Context<'a, I>) -> Rc<DynParser<'a, I, Self::O>> {
		let parser = ctx.parser::<G>().clone();
		Rc::new(Recognize(parser))
	}
}