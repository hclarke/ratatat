use crate::*;
use core::marker::PhantomData;
use std::fmt;

pub struct Parse<G>(PhantomData<*const G>);
impl<G> Clone for Parse<G> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<G> Copy for Parse<G> {}

impl<G> Debug for Parse<G> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Parse<{}>", core::any::type_name::<G>())
	}
}

pub fn parser<G>() -> Parse<G> {
    Parse(PhantomData)
}

impl<'a, G: Generator<'a>> Parser<'a> for Parse<G> {
    type O = G::O;
    fn impl_parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<Self::O> {
        ctx.parse::<G>(limit, pos)
    }
}
