use crate::*;

#[derive(Copy, Clone)]
pub struct Map<P, F>(pub P, pub F);

impl<P: Debug, F> Debug for Map<P, F> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Map({:?},?)", self.0)
    }
}

impl<'a, O, P, F> Parser<'a> for Map<P, F>
where
    P: Parser<'a>,
    F: Fn(P::O) -> O,
    O: Debug,
{
    type O = O;
    fn impl_parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<O> {
        self.0.parse(ctx, limit, pos).map(&self.1)
    }

    fn name(&self) -> String {
        format!("Map")
    }
}
