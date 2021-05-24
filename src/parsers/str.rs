use crate::*;

impl<'a, 'b> Parser<'a> for &'b str {
    type O = &'a str;
    fn impl_parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<Self::O> {
        let bytes = self.as_bytes();

        let src = &ctx.bytes[*pos..limit];
        if src.len() < self.len() {
            return None;
        }

        let prefix = &src[..bytes.len()];
        if prefix == bytes {
            *pos += self.len();

            // we know this is safe, because prefix matched a valid utf8 string
            let prefix = unsafe { std::str::from_utf8_unchecked(prefix) };

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
        let input = Shared::new(Rc::new(b"hello world"[..].to_owned()));
        let ctx = &Context::new(&input);
        let mut pos = 0;
        assert_eq!(Some("hello"), "hello".run(&ctx, ctx.bytes.len(), &mut pos));
        assert_eq!(5, pos);
    }
}
