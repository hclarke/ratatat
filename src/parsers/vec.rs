use crate::*;

impl<'a, G:Generator<'a>> Generator<'a> for Vec<G> {
	type O = Vec<G::O>;
	fn generate(ctx: &Context<'a>) -> Rc<DynParser<'a, Self::O>> {
		let parser = ctx.parser::<G>().clone();
		Rc::new(Many::new(parser, ..))
	}
}