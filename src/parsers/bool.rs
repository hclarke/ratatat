use crate::*;

impl<'a> Parser<'a> for bool {
    type O = bool;
    fn impl_parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<bool> {
        let parser = match *self {
            true => "true",
            false => "false",
        };

        let res = parser.run(ctx, limit, pos);
        res.map(|_| *self)
    }
}

impl<'a> Generator<'a> for bool {
    type O = bool;
    fn generate(_ctx: &Context<'a>) -> Rc<DynParser<'a, Self::O>> {
        Rc::new([true, false])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bool_parser() {
        assert_parse!(Some(true), true, "true");
        assert_parse!(Some(false), false, "false");
        assert_parse!(None, true, "lol");

        assert_parse!(Some(true), parser::<bool>(), "true");
        assert_parse!(Some(false), parser::<bool>(), "false");
        assert_parse!(None, parser::<bool>(), "lol");
    }
}
