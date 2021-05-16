use crate::*;

pub struct Sep<P,S>(pub P, pub S);

impl<'a, I:?Sized, P:Parser<'a,I>, S:Parser<'a,I>> Parser<'a,I> for Sep<P,S> {
	type O = Vec<P::O>;
	fn parse(&self, ctx: &Context<'a,I>, limit: usize, pos: &mut usize) -> Option<Self::O> {
		let mut v = Vec::new();

		let mut reset = *pos;
		match self.0.parse(ctx, limit, pos) {
			Some(val) => v.push(val),
			None => {
				// goto end
				*pos = reset;
				return Some(v);
			}
		}

		loop {
			reset = *pos;
			if let None = self.1.parse(ctx, limit, pos) {
				break;
			}
			match self.0.parse(ctx, limit, pos) {
				Some(val) => v.push(val),
				None => break,
			}
		}

		*pos = reset;
		Some(v)
	}
}

impl<'a, I:?Sized, G:Generator<'a, I>, S:Generator<'a,I>> Generator<'a, I> for Sep<G,S> {
	type O = Vec<G::O>;
	fn generate(ctx: &Context<'a,I>) -> Rc<DynParser<'a,I,Self::O>> {
		let parser = ctx.parser::<G>().clone();
		let sep = ctx.parser::<S>().clone();
		Rc::new(Sep(parser, sep))
	}
}

