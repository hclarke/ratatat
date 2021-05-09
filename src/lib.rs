use elsa::FrozenMap;
use core::any::{Any, TypeId};
 
use std::rc::Rc;

mod parsers;
pub use parsers::*;

pub trait Parser<'a,I:?Sized> {
	type O;
	fn parse(&self, ctx: &Context<'a,I>, src: &mut &'a [u8]) -> Option<Self::O>;

	/// run is an alias for parse, to make it easier when the parser type has another 'parse' method
	fn run(&self, ctx: &Context<'a,I>, src: &mut &'a [u8]) -> Option<Self::O> {
		self.parse(ctx,src)
	}
}

impl<'a,I:?Sized,O,F> Parser<'a,I> for F where F:Fn(&Context<I>, &mut &[u8]) -> Option<O> {
	type O = O;
	fn parse(&self, ctx: &Context<'a, I>, src: &mut &'a [u8]) -> Option<Self::O> {
		self(ctx, src)
	}
}

pub trait Generator<'a,I:?Sized>:Any {
	type O;
	fn generate(ctx: &Context<'a,I>) -> Rc<dyn Parser<'a, I, O=Self::O>+'a>;
}




struct GeneratedParser<'a, I:?Sized, G:Generator<'a,I>>(Rc<dyn Parser<'a, I, O=G::O>+'a>);

trait Generated<'a,I:?Sized>:'a {
	fn generated_by(&self) -> TypeId;
}
impl<'a, I:?Sized+'a, G:Generator<'a, I>> Generated<'a,I> for GeneratedParser<'a, I, G> {
	fn generated_by(&self) -> TypeId {
		TypeId::of::<G>()
	}
}
impl<'a,I:?Sized+'a> dyn Generated<'a,I> {
	fn downcast<G:Generator<'a,I>>(&self) -> Option<&Rc<dyn Parser<'a, I, O=G::O>+'a>> {
		if TypeId::of::<G>() == self.generated_by() {
			return unsafe {
				let wrapper = &*(self as *const dyn Generated<'a, I> as *const GeneratedParser<'a,I,G>);
				Some(&wrapper.0)
			};
		}

		None
	}
}

pub struct Context<'a, I:?Sized> {
	pub source: &'a I,
	parsers: FrozenMap<TypeId, Box<dyn Generated<'a, I>+'a>>,
}

impl<'a,I:?Sized> Context<'a,I> {
	pub fn new(source: &'a I) -> Self {
		Context {
			source,
			parsers: FrozenMap::new(),
		}
	}

	pub fn init_parser<G:Generator<'a,I>>(&self, parser: Rc<dyn Parser<'a, I, O=G::O>+'a>) {
		let id = TypeId::of::<G>();

		assert!(self.parsers.get(&id).is_none());

		let parser = GeneratedParser::<'a,I,G>(parser);
		let parser = Box::new(parser);
		self.parsers.insert(id, parser);
	}

	pub fn parser<G:Generator<'a, I>>(&self) -> &Rc<dyn Parser<'a, I, O=G::O>+'a> {
		let id = TypeId::of::<G>();

		let parser = self.parsers.get(&id);

		//TODO: occurs check

		let parser = parser.or_else(|| {
			let parser = G::generate(self);
			self.init_parser::<G>(parser);
			self.parsers.get(&id)
		});

		let parser = parser.unwrap();

		parser.downcast::<G>().unwrap()
	}

	pub fn parse<G:Generator<'a, I>>(&self, input: &mut &'a [u8]) -> Option<G::O> {
		let parser = self.parser::<G>();
		parser.parse(self, input)
	}
}