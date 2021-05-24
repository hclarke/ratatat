use crate::*;

impl<'a, P: Parser<'a>> Parser<'a> for [P] {
    type O = P::O;
    fn impl_parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<Self::O> {
        let reset = *pos;
        for parser in self {
            match parser.parse(ctx, limit, pos) {
                Some(x) => return Some(x),
                None => {
                    *pos = reset;
                }
            }
        }
        None
    }

    fn name(&self) -> String {
        "[]".to_owned()
    }
}
