use crate::*;

use array_init::try_array_init;

impl<'a, I: ?Sized, G: Generator<'a, I>, const N: usize> Generator<'a, I> for [G; N] {
    type O = [G::O; N];
    fn generate(ctx: &Context<'a, I>) -> Rc<DynParser<'a, I, Self::O>> {
        let parser = ctx.parser::<G>().clone();
        let f = move |ctx: &Context<'a, I>, limit: usize, pos: &mut usize| {
            try_array_init(|_| parser.parse(ctx, limit, pos).ok_or(())).ok()
        };
        Rc::new(f)
    }
}

impl<'a, I: ?Sized, P: Parser<'a, I>, const N: usize> Parser<'a, I> for [P; N] {
    type O = P::O;
    fn parse(&self, ctx: &Context<'a, I>, limit: usize, pos: &mut usize) -> Option<Self::O> {
        self[..].parse(ctx, limit, pos)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn char_parser() {
        assert_eq!(Some('b'), ['a', 'b', 'c'].parse_str("b"));
        assert_eq!(None, ['a', 'b', 'c'].parse_str("d"));

        assert_eq!(
            Some(['a', 'b', 'c']),
            parser::<[char; 3]>().parse_str("abc")
        );
        assert_eq!(None, parser::<[char; 3]>().parse_str("ab"));
    }
}
