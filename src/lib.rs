use elsa::FrozenMap;
use srcstr::SrcStr;
use core::any::{Any, TypeId};
 
use core::ops::Range;


pub type Ctx<'a> = &'a Context<'a>;
pub type DynParser<'a,O> = &'a (dyn Parser<'a, O=O>+'a);

pub trait Parser<'a> {
	type O;
	fn parse(&self, ctx: Ctx<'a>, src: &mut &'a str) -> Option<Self::O>;

	/// run is an alias for parse, to make it easier when the parser type has another 'parse' method
	fn run(&self, ctx: Ctx<'a>, src: &mut &'a str) -> Option<Self::O> {
		self.parse(ctx,src)
	}
}

impl<'a,O,F> Parser<'a> for F where F:Fn(&Context, &mut &str) -> Option<O> {
	type O = O;
	fn parse(&self, ctx: Ctx<'a>, src: &mut &'a str) -> Option<Self::O> {
		self(ctx, src)
	}
}

pub trait Generator<'a>:Any {
	type O;
	fn generate(ctx: Ctx<'a>) -> Box<dyn Parser<'a, O=Self::O>>;
}




struct GeneratedParser<'a, G:Generator<'a>>(Box<dyn Parser<'a, O=G::O>+'a>);

trait Generated<'a>:'a {
	fn generated_by(&self) -> TypeId;
}
impl<'a, G:Generator<'a>> Generated<'a> for GeneratedParser<'a, G> {
	fn generated_by(&self) -> TypeId {
		TypeId::of::<G>()
	}
}
impl<'a> dyn Generated<'a> {
	fn downcast<G:Generator<'a>>(&'a self) -> Option<&'a dyn Parser<'a, O=G::O>> {
		if TypeId::of::<G>() == self.generated_by() {
			return unsafe {
				let wrapper = &*(self as *const dyn Generated<'a> as *const GeneratedParser<'a,G>);
				Some(&*wrapper.0)
			};
		}

		None
	}
}

pub struct Context<'a> {
	source: &'a SrcStr,
	parsers: FrozenMap<TypeId, Box<dyn Generated<'a>+'a>>,
}

impl<'a> Context<'a> {
	pub fn new(source: &'a SrcStr) -> Self {

		Context {
			source,
			parsers: FrozenMap::new(),
		}
	}

	pub fn init_parser<G:Generator<'a>>(&'a self, parser: Box<dyn Parser<'a, O=G::O> +'a>) {
		let id = TypeId::of::<G>();

		assert!(self.parsers.get(&id).is_none());

		let parser = GeneratedParser::<'a,G>(parser);
		let parser = Box::new(parser);
		self.parsers.insert(id, parser);
	}

	pub fn parser<G:Generator<'a>>(&'a self) -> DynParser<'a, G::O> {
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

	pub fn parse<G:Generator<'a>>(&'a self, input: &mut &'a str) -> Option<G::O> {
		let parser = self.parser::<G>();
		parser.parse(self, input)
	}
}