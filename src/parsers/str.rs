use crate::*;

impl<'a,'b, I:?Sized> Parser<'a, I> for &'b str {
	type O = &'a str;
	fn parse(&self, ctx: &Context<'a, I>, limit: usize, pos: &mut usize) -> Option<Self::O> {
		let bytes = self.as_bytes();

		let src = &ctx.bytes[*pos..limit];
		if src.len() < self.len() {
			return None;
		}

		let prefix = &src[..bytes.len()];
		if prefix == bytes {
			*pos += self.len();

			// we know this is safe, because prefix matched a valid utf8 string
			let prefix = unsafe {
				std::str::from_utf8_unchecked(prefix)
			};

			return Some(prefix);
		}

		None
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn str_parser() {

		let ctx = &Context::from_str("hello world");
		let mut pos = 0;
		assert_eq!(Some("hello"), "hello".run(&ctx, ctx.bytes.len(), &mut pos));
		assert_eq!(5, pos);
		

	}
}