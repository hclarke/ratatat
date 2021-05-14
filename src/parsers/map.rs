use crate::*;

pub struct Map<P, F>(pub P, pub F);

impl<'a, I, O, P, F> Parser<'a, I> for Map<P, F>
where
    I: ?Sized,
    P: Parser<'a, I>,
    F: Fn(P::O) -> O,
{
    type O = O;
    fn parse(&self, ctx: &Context<'a, I>, limit: usize, pos: &mut usize) -> Option<O> {
        self.0.parse(ctx, limit, pos).map(&self.1)
    }
}
