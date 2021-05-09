use crate::*;
use core::ops::Deref;

impl<'a,'b, I:?Sized> Parser<'a, I> for &'b str {
	type O = &'a str;
	fn parse(&self, _ctx: &Context<'a, I>, src: &mut &'a [u8]) -> Option<Self::O> {
		let bytes = self.as_bytes();

		if src.len() < self.len() {
			return None;
		}

		let prefix = &src[..bytes.len()];
		if prefix == bytes {
			*src = &src[bytes.len()..];

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
	use srcstr::SrcStr;

	#[test]
	fn str_parser() {
		let src = b"hello world";
		let mut src = &src[..];

		let ctx = &Context::new(src);
		assert_eq!(Some("hello"), "hello".run(&ctx, &mut src));
		assert_eq!(b" world", src);
		

	}
}