use crate::*;

#[derive(Copy, Clone)]
pub struct Then<P, F>(pub P, pub F);

impl<P: Debug, F> Debug for Then<P, F> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Then({:?},?)", self.0)
    }
}

impl<'a, P, F, O> Parser<'a> for Then<P, F>
where
    P: Parser<'a>,
    F: Fn(<P as Parser<'a>>::O) -> O,
    O: Parser<'a>,
{
    type O = <O as Parser<'a>>::O;
    fn impl_parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<Self::O> {
        let v = self.0.parse(ctx, limit, pos)?;
        let p = self.1(v);
        p.parse(ctx, limit, pos)
    }
}
