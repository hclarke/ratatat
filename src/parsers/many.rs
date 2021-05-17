use crate::*;
use core::ops::Bound;
use core::ops::RangeBounds;

#[derive(Clone)]
pub struct Many<P, R>(pub P, pub R);

impl<'a, P: Parser<'a>, R: RangeBounds<usize>> Parser<'a> for Many<P, R> {
    type O = Vec<P::O>;
    fn parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<Self::O> {
        let mut v = Vec::new();

        let min = match self.1.start_bound() {
            Bound::Included(n) => *n,
            Bound::Excluded(n) => *n + 1,
            Bound::Unbounded => 0,
        };

        for _ in 0..min {
            let res = self.0.parse(ctx, limit, pos)?;
            v.push(res);
        }

        let max = match self.1.end_bound() {
            Bound::Included(n) => *n + 1,
            Bound::Excluded(n) => *n,
            Bound::Unbounded => usize::MAX,
        };

        for _ in min..max {
            let rewind = *pos;
            let res = self.0.parse(ctx, limit, pos);
            match res {
                Some(val) => v.push(val),
                None => {
                    *pos = rewind;
                    return Some(v);
                }
            }
        }

        Some(v)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn char_parser() {
        assert_parse!(Some(vec!['a', 'a', 'a']), Many('a', ..), "aaab");
        assert_parse!(Some(vec!['a', 'a', 'a']), Many('a', ..3), "aaaa");

        assert_parse!(Some(vec!['a', 'a', 'a', 'a']), Many('a', 3..), "aaaa");
        assert_parse!(None, Many('a', 3..), "aa");
    }
}
