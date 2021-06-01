use crate::*;

#[derive(Debug, Copy, Clone)]
pub struct Opt<P>(pub P);

impl<'a, P: Parser<'a>> Parser<'a> for Opt<P> {
    type O = Option<P::O>;
    fn impl_parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<Self::O> {
        Some(self.0.parse(ctx, limit, pos))
    }
}
