use crate::*;
use core::ops::Bound;
use core::ops::RangeBounds;

#[derive(Clone)]
pub struct Many<P, R>(pub P, pub R);

impl<'a, I: ?Sized, P: Parser<'a, I>, R: RangeBounds<usize>> Parser<'a, I> for Many<P, R> {
    type O = Vec<P::O>;
    fn parse(&self, ctx: &Context<'a, I>, limit: usize, pos: &mut usize) -> Option<Self::O> {
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
        assert_eq!(Some(vec!['a', 'a', 'a']), Many('a', ..).parse_str("aaab"));
        assert_eq!(Some(vec!['a', 'a', 'a']), Many('a', ..3).parse_str("aaaa"));

        assert_eq!(
            Some(vec!['a', 'a', 'a', 'a']),
            Many('a', 3..).parse_str("aaaa")
        );
        assert_eq!(None, Many('a', 3..).parse_str("aa"));
    }
}
