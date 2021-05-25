use crate::*;

#[derive(Debug, Copy, Clone)]
pub struct Sep<P, S>(pub P, pub S);

impl<'a, P: Parser<'a>, S: Parser<'a>> Parser<'a> for Sep<P, S> {
    type O = Vec<P::O>;
    fn impl_parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<Self::O> {
        let mut v = Vec::new();

        let mut reset = *pos;
        match self.0.parse(ctx, limit, pos) {
            Some(val) => v.push(val),
            None => {
                // goto end
                *pos = reset;
                return Some(v);
            }
        }

        loop {
            reset = *pos;
            if self.1.parse(ctx, limit, pos).is_none() {
                break;
            }
            match self.0.parse(ctx, limit, pos) {
                Some(val) => v.push(val),
                None => break,
            }
        }

        *pos = reset;
        Some(v)
    }

    fn name(&self) -> String {
        "Sep".to_owned()
    }
}

impl<'a, G: Generator<'a>, S: Generator<'a>> Generator<'a> for Sep<G, S> {
    type O = Vec<G::O>;
    fn generate(ctx: &Context<'a>) -> Rc<DynParser<'a, Self::O>> {
        let parser = ctx.parser::<G>().clone();
        let sep = ctx.parser::<S>().clone();
        Rc::new(Sep(parser, sep))
    }
}
