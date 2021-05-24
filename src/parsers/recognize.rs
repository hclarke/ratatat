use crate::*;

#[derive(Debug, Copy, Clone)]
pub struct Recognize<T>(pub T);

impl<'a, P: Parser<'a>> Parser<'a> for Recognize<P> {
    type O = &'a [u8];
    fn impl_parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<Self::O> {
        let start = *pos;
        let _ = self.0.parse(ctx, limit, pos)?;
        let end = *pos;

        Some(&ctx.bytes[start..end])
    }

    fn name(&self) -> String {
    	"Recognize".to_string()
    }
}

impl<'a, G: Generator<'a>> Generator<'a> for Recognize<G> {
    type O = &'a [u8];
    fn generate(ctx: &Context<'a>) -> Rc<DynParser<'a, Self::O>> {
        let parser = ctx.parser::<G>().clone();
        Rc::new(Recognize(parser))
    }
}
