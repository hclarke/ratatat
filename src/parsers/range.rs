use crate::*;

impl<'a, I: ?Sized, T> Parser<'a, I> for std::ops::Range<T>
where
    T: Generator<'a, I, O = T> + PartialOrd,
{
    type O = T;
    fn parse(&self, ctx: &Context<'a, I>, limit: usize, pos: &mut usize) -> Option<Self::O> {
        let val = ctx.parse::<T>(limit, pos);
        val.filter(|val| self.contains(val))
    }
}

impl<'a, I: ?Sized, T> Parser<'a, I> for std::ops::RangeFrom<T>
where
    T: Generator<'a, I, O = T> + PartialOrd,
{
    type O = T;
    fn parse(&self, ctx: &Context<'a, I>, limit: usize, pos: &mut usize) -> Option<Self::O> {
        let val = ctx.parse::<T>(limit, pos);
        val.filter(|val| self.contains(val))
    }
}

impl<'a, I: ?Sized, T> Parser<'a, I> for std::ops::RangeTo<T>
where
    T: Generator<'a, I, O = T> + PartialOrd,
{
    type O = T;
    fn parse(&self, ctx: &Context<'a, I>, limit: usize, pos: &mut usize) -> Option<Self::O> {
        let val = ctx.parse::<T>(limit, pos);
        val.filter(|val| self.contains(val))
    }
}

impl<'a, I: ?Sized, T> Parser<'a, I> for std::ops::RangeInclusive<T>
where
    T: Generator<'a, I, O = T> + PartialOrd,
{
    type O = T;
    fn parse(&self, ctx: &Context<'a, I>, limit: usize, pos: &mut usize) -> Option<Self::O> {
        let val = ctx.parse::<T>(limit, pos);
        val.filter(|val| self.contains(val))
    }
}

impl<'a, I: ?Sized, T> Parser<'a, I> for std::ops::RangeToInclusive<T>
where
    T: Generator<'a, I, O = T> + PartialOrd,
{
    type O = T;
    fn parse(&self, ctx: &Context<'a, I>, limit: usize, pos: &mut usize) -> Option<Self::O> {
        let val = ctx.parse::<T>(limit, pos);
        val.filter(|val| self.contains(val))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn range_parser() {
        assert_parse!(Some('d'), ('a'..'z'), "d");
        assert_parse!(None, ('a'..'z'), "D");
    }
}
