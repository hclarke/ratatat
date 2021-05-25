use crate::*;

#[derive(Clone, Copy)]
pub struct Filter<P, F>(pub P, pub F);
#[derive(Clone, Copy)]
pub struct FilterMap<P, F>(pub P, pub F);

impl<P: Debug, F> Debug for Filter<P, F> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Filter({:?},?)", self.0)
    }
}
impl<P: Debug, F> Debug for FilterMap<P, F> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "FilterMap({:?},?)", self.0)
    }
}

impl<'a, P: Parser<'a>, F: Fn(&P::O) -> bool> Parser<'a> for Filter<P, F> {
    type O = P::O;
    fn impl_parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<Self::O> {
        let r = self.0.parse(ctx, limit, pos)?;
        if !self.1(&r) {
            return None;
        }

        Some(r)
    }

    fn name(&self) -> String {
        format!("Filter")
    }
}

impl<'a, T: Debug, P: Parser<'a>, F: Fn(P::O) -> Option<T>> Parser<'a> for FilterMap<P, F> {
    type O = T;
    fn impl_parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<Self::O> {
        let r = self.0.parse(ctx, limit, pos)?;
        self.1(r)
    }

    fn name(&self) -> String {
        format!("Then")
    }
}
