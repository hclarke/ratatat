use crate::*;
use core::ops::Bound;
use core::ops::RangeBounds;

#[derive(Debug, Clone, Copy)]
pub struct Many<P>(pub P, pub usize, pub usize);

impl<P> Many<P> {
    pub fn new<R: RangeBounds<usize>>(p: P, r: R) -> Many<P> {
        let min = match r.start_bound() {
            Bound::Included(n) => *n,
            Bound::Excluded(n) => *n + 1,
            Bound::Unbounded => 0,
        };

        let max = match r.end_bound() {
            Bound::Included(n) => *n + 1,
            Bound::Excluded(n) => *n,
            Bound::Unbounded => usize::MAX,
        };

        Many(p, min, max)
    }
}

impl<'a, P: Parser<'a>> Parser<'a> for Many<P> {
    type O = Vec<P::O>;
    fn impl_parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<Self::O> {
        let mut v = Vec::new();

        for _ in 0..self.1 {
            let res = self.0.parse(ctx, limit, pos)?;
            v.push(res);
        }

        for _ in self.1..self.2 {
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

    fn name(&self) -> String {
        match (self.1 == 0, self.2 == std::usize::MAX) {
            (true, true) => format!("Many({:?})", ..),
            (true, false) => format!("Many({:?})", ..self.2),
            (false, true) => format!("Many({:?})", self.1..),
            (false, false) => format!("Many({:?})", self.1..self.2),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn char_parser() {
        assert_parse!(Some(vec!['a', 'a', 'a']), Many::new('a', ..), "aaab");
        assert_parse!(Some(vec!['a', 'a', 'a']), Many::new('a', ..3), "aaaa");

        assert_parse!(Some(vec!['a', 'a', 'a', 'a']), Many::new('a', 3..), "aaaa");
        assert_parse!(None, Many::new('a', 3..), "aa");
    }
}
