use crate::*;

use array_init::try_array_init;


struct Array<P,const N:usize>(P);
impl<'a, I:?Sized, P:Parser<'a,I>, const N: usize> Parser<'a, I> for Array<P,N> {
	type O = [P::O; N];
	fn parse(&self, ctx: &Context<'a, I>, limit: usize, pos: &mut usize) -> Option<Self::O> {
		try_array_init(|_| {
			self.0.parse(ctx, limit, pos).ok_or(())
		}).ok()
	}
}

impl<'a, I:?Sized, G:Generator<'a, I>, const N: usize> Generator<'a, I> for [G;N] {
	type O = [G::O; N];
	fn generate(ctx: &Context<'a, I>) -> Rc<DynParser<'a, I, Self::O>> {
		let parser = ctx.parser::<G>().clone();
		Rc::new(Array::<_, N>(parser))
	}
}

impl<'a, I:?Sized, P:Parser<'a,I>, const N: usize> Parser<'a, I> for [P;N] {
	type O = P::O;
	fn parse(&self, ctx: &Context<'a, I>, limit: usize, pos: &mut usize) -> Option<Self::O> {
		self[..].parse(ctx, limit, pos)
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn char_parser() {
		assert_eq!(Some('b'), ['a','b','c'].parse_str("b"));
		assert_eq!(None, ['a','b','c'].parse_str("d"));

		assert_eq!(Some(['a', 'b', 'c']), parser::<[char;3]>().parse_str("abc"));
		assert_eq!(None, parser::<[char;3]>().parse_str("ab"));
	}
}