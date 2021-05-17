use crate::*;

#[derive(Copy,Clone)]
pub struct Map<P, F>(pub P, pub F);

impl<'a, O, P, F> Parser<'a> for Map<P, F>
where
    P: Parser<'a>,
    F: Fn(P::O) -> O,
{
    type O = O;
    fn parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<O> {
        self.0.parse(ctx, limit, pos).map(&self.1)
    }
}
