use crate::*;

impl<'a, I:?Sized> Parser<'a, I> for bool {
	type O = bool;
	fn parse(&self, ctx: &Context<'a, I>, limit: usize, pos: &mut usize) -> Option<bool> {
		let parser = match *self {
			true => "true",
			false => "false",
		};

		let res = parser.run(ctx, limit, pos);
		res.map(|_| *self)
	}
}

impl<'a, I:?Sized> Generator<'a, I> for bool {
	type O=bool;
	fn generate(_ctx: &Context<'a, I>) -> Rc<DynParser<'a, I, Self::O>> {
		Rc::new([true,false])
	}
}


#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn bool_parser() {
		assert_eq!(Some(true), true.parse_str("true"));
		assert_eq!(Some(false), false.parse_str("false"));
		assert_eq!(None, true.parse_str("lol"));

		assert_eq!(Some(true), parser::<bool>().parse_str("true"));
		assert_eq!(Some(false), parser::<bool>().parse_str("false"));
		assert_eq!(None, parser::<bool>().parse_str("lol"));
		

	}
}