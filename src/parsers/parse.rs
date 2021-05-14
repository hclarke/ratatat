use crate::*;
use core::marker::PhantomData;

pub struct Parse<G>(PhantomData<G>);

pub fn parser<G>() -> Parse<G> {
    Parse(PhantomData)
}

impl<'a, I: ?Sized, G: Generator<'a, I>> Parser<'a, I> for Parse<G> {
    type O = G::O;
    fn parse(&self, ctx: &Context<'a, I>, limit: usize, pos: &mut usize) -> Option<Self::O> {
        ctx.parse::<G>(limit, pos)
    }
}
