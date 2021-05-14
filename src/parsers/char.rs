use crate::*;
use bstr::ByteSlice;

impl<'a, I: ?Sized> Parser<'a, I> for char {
    type O = char;
    fn parse(&self, ctx: &Context<'a, I>, limit: usize, pos: &mut usize) -> Option<Self::O> {
        let mut buf = [0; 4];
        let len = self.encode_utf8(&mut buf[..]).len();

        let bytes = &ctx.bytes[*pos..limit];
        if bytes.len() < len {
            return None;
        }

        if bytes[..len] == buf[..len] {
            *pos += len;
            return Some(*self);
        }

        None
    }
}

fn parse_char<'a, I: ?Sized>(ctx: &Context<'a, I>, limit: usize, pos: &mut usize) -> Option<char> {
    let bytes = &ctx.bytes[*pos..limit];
    match bytes.char_indices().next() {
        Some((_, len, c)) => {
            *pos += len;
            Some(c)
        }
        None => None,
    }
}

impl<'a, I: ?Sized> Generator<'a, I> for char {
    type O = char;
    fn generate(_ctx: &Context<'a, I>) -> Rc<DynParser<'a, I, Self::O>> {
        Rc::new(parse_char)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn char_parser() {
        assert_eq!(Some('a'), 'a'.parse_str("a"));
        assert_eq!(Some('∆'), '∆'.parse_str("∆"));
        assert_eq!(None, 'a'.parse_str("ø"));

        assert_eq!(Some('a'), parser::<char>().parse_str("a"));
        assert_eq!(Some('∆'), parser::<char>().parse_str("∆"));
        assert_eq!(None, parser::<char>().parse_str(""));
    }
}
