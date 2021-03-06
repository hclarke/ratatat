use crate::*;
use std::rc::Weak;

impl<'a, P: Parser<'a> + ?Sized> Parser<'a> for Rc<P> {
    type O = P::O;
    fn impl_parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<Self::O> {
        (&**self).parse(ctx, limit, pos)
    }

    fn name(&self) -> String {
        "Rc".to_owned()
    }
}

impl<'a, P: Parser<'a> + ?Sized> Parser<'a> for Weak<P> {
    type O = P::O;
    fn impl_parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<Self::O> {
        let rc = self.upgrade().unwrap();
        rc.parse(ctx, limit, pos)
    }

    fn name(&self) -> String {
        "Weak".to_owned()
    }
}
