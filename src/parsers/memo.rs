use crate::*;

use elsa::FrozenMap;

pub struct Memo<P>(*const P, Never);
enum Never{}



impl<'a, I:?Sized, G:Generator<'a, I>> Generator<'a, I> for Memo<G> 
where
	G::O: Clone
{
	type O = G::O;
	fn generate(ctx: &Context<'a, I>) -> Rc<DynParser<'a, I, Self::O>> {


		struct MemoEntry<T> {
			value: Option<T>,
			pos: usize,
		}

		let parser = ctx.parser::<G>().clone();

		let memo : FrozenMap::<(usize,usize), Box<MemoEntry<Self::O>>> = FrozenMap::new();

		Rc::new(move |ctx: &Context<'a, I>, limit: usize, pos: &mut usize| {
			let key = (*pos, limit);
			if let Some(entry) = memo.get(&key) {
				*pos = entry.pos;
				return entry.value.clone();
			}

			let res = parser.parse(ctx, limit, pos);
			memo.insert(key, Box::new(MemoEntry {
				value: res.clone(),
				pos: *pos,
			}));

			res
		})
	}
}



#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn memo_parser() {

		struct Counter;
		impl<'a,I:?Sized> Generator<'a,I> for Counter {
			type O = usize;
			fn generate(_ctx: &Context<I>) -> Rc<DynParser<'a, I, Self::O>> {
				let cell = std::cell::RefCell::new(0);
				let counter = move |_ctx: &Context<I>, _limit: usize, _pos: &mut usize| {
					let mut val = cell.borrow_mut();
					*val += 1;
					Some(*val)
				};
				Rc::new(counter)
			}
		}

		let ctx = Context::from_str("");

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