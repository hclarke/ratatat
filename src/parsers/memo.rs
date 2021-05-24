use crate::*;

use elsa::FrozenMap;

pub struct Memo<P>(*const P, Never);
enum Never {}

impl<'a, G: Generator<'a>> Generator<'a> for Memo<G>
where
    G::O: Clone,
{
    type O = G::O;
    fn generate(ctx: &Context<'a>) -> Rc<DynParser<'a, Self::O>> {
        struct MemoEntry<T> {
            value: Option<T>,
            pos: usize,
        }

        let parser = ctx.parser::<G>().clone();

        let memo: FrozenMap<(usize, usize), Box<MemoEntry<Self::O>>> = FrozenMap::new();

        let f = move |ctx: &Context<'a>, limit: usize, pos: &mut usize| {
            let key = (*pos, limit);
            if let Some(entry) = memo.get(&key) {
                *pos = entry.pos;
                return entry.value.clone();
            }

            let res = parser.parse(ctx, limit, pos);
            memo.insert(
                key,
                Box::new(MemoEntry {
                    value: res.clone(),
                    pos: *pos,
                }),
            );

            res
        };

        Rc::new(FnParser(f, Some("Memo")))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn memo_parser() {
        struct Counter;
        impl<'a> Generator<'a> for Counter {
            type O = usize;
            fn generate(_ctx: &Context) -> Rc<DynParser<'a, Self::O>> {
                let cell = std::cell::RefCell::new(0);
                let counter = move |_ctx: &Context, _limit: usize, _pos: &mut usize| {
                    let mut val = cell.borrow_mut();
                    *val += 1;
                    Some(*val)
                };
                Rc::new(FnParser(counter, None))
            }
        }

        let input = Shared::new(Rc::new(b""[..].to_owned()));
        let ctx = Context::new(&input);

        // make sure the ocunter works
        assert_eq!(Some(1), ctx.parse::<Counter>(0, &mut 0));
        assert_eq!(Some(2), ctx.parse::<Counter>(0, &mut 0));

        //make sure memo doesn't change it
        assert_eq!(Some(3), ctx.parse::<Memo<Counter>>(0, &mut 0));
        assert_eq!(Some(3), ctx.parse::<Memo<Counter>>(0, &mut 0));

        assert_eq!(Some(4), ctx.parse::<Counter>(0, &mut 0));
        assert_eq!(Some(3), ctx.parse::<Memo<Counter>>(0, &mut 0));

        // make sure memo calls parser again with a new pos
        assert_eq!(Some(5), ctx.parse::<Memo<Counter>>(0, &mut 1));
        assert_eq!(Some(5), ctx.parse::<Memo<Counter>>(0, &mut 1));
    }
}
