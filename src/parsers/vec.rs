use crate::*;

impl<'a, I:?Sized, G:Generator<'a, I>> Generator<'a, I> for Vec<G> {
	type O = Vec<G::O>;
	fn generate(ctx: &Context<'a, I>) -> Rc<DynParser<'a, I, Self::O>> {
		let parser = ctx.parser::<G>().clone();
		Rc::new(Many(parser, ..))
	}
}