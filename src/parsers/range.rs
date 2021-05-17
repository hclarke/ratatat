use crate::*;

impl<'a, T> Parser<'a> for std::ops::Range<T>
where
    T: Generator<'a, O = T> + PartialOrd,
{
    type O = T;
    fn parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<Self::O> {
        let val = ctx.parse::<T>(limit, pos);
        val.filter(|val| self.contains(val))
    }
}

impl<'a, T> Parser<'a> for std::ops::RangeFrom<T>
where
    T: Generator<'a, O = T> + PartialOrd,
{
    type O = T;
    fn parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<Self::O> {
        let val = ctx.parse::<T>(limit, pos);
        val.filter(|val| self.contains(val))
    }
}

impl<'a, T> Parser<'a> for std::ops::RangeTo<T>
where
    T: Generator<'a, O = T> + PartialOrd,
{
    type O = T;
    fn parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<Self::O> {
        let val = ctx.parse::<T>(limit, pos);
        val.filter(|val| self.contains(val))
    }
}

impl<'a, T> Parser<'a> for std::ops::RangeInclusive<T>
where
    T: Generator<'a, O = T> + PartialOrd,
{
    type O = T;
    fn parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<Self::O> {
        let val = ctx.parse::<T>(limit, pos);
        val.filter(|val| self.contains(val))
    }
}

impl<'a, T> Parser<'a> for std::ops::RangeToInclusive<T>
where
    T: Generator<'a, O = T> + PartialOrd,
{
    type O = T;
    fn parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<Self::O> {
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
