use crate::*;

#[derive(Copy,Clone)]
pub struct FnParser<F>(pub F, pub Option<&'static str>);
impl<F> Debug for FnParser<F> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f,"{}", self.1.unwrap_or("FnParser"))
    }
}

impl<'a, O:Debug, F:Fn(&Context<'a>, usize, &mut usize) -> Option<O>> Parser<'a> for FnParser<F> {
    type O = O;
    fn impl_parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<Self::O> {
        self.0(ctx, limit, pos)
    }
}