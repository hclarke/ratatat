use crate::*;
use bstr::ByteSlice;

impl<'a> Parser<'a> for char {
    type O = char;
    fn impl_parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<Self::O> {
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

fn parse_char<'a>(ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<char> {
    let bytes = &ctx.bytes[*pos..limit];
    match bytes.char_indices().next() {
        Some((_, len, c)) => {
            *pos += len;
            Some(c)
        }
        None => None,
    }
}

impl<'a> Generator<'a> for char {
    type O = char;
    fn generate(_ctx: &Context<'a>) -> Rc<DynParser<'a, Self::O>> {
        Rc::new(FnParser(parse_char, Some("char")))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn char_parser() {
        assert_parse!(Some('a'), 'a', "a");
        assert_parse!(Some('∆'), '∆', "∆");
        assert_parse!(None, 'a', "ø");

        assert_parse!(Some('a'), parser::<char>(), "a");
        assert_parse!(Some('∆'), parser::<char>(), "∆");
        assert_parse!(None, parser::<char>(), "");
    }
}
