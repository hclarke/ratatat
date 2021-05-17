use core::any::{Any, TypeId};
use elsa::FrozenMap;

use core::borrow::Borrow;
use std::rc::Rc;
use std::ops::Deref;
use core::ops::RangeBounds;
use core::ops::Bound;

#[cfg(test)]
#[macro_use]
mod test_macro {
    macro_rules! assert_parse {
        ($exp: expr, $parser: expr, $input: expr) => {
            let input : &[u8] = $input.as_ref();
            let input = Rc::new(input.to_owned());
            let input = Shared::new(input);
            let ctx = Context::new(&input);
            let mut pos = 0;
            let res = $parser.parse(&ctx, ctx.bytes.len(), &mut pos);
            assert_eq!($exp, res);
        }
    }
}
#[cfg(test)]
use test_macro::*;

mod parsers;
pub use parsers::*;

mod input;
pub use input::*;

pub trait Parser<'a> {
    type O;
    fn parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<Self::O>;
}

pub type DynParser<'a, O> = dyn Parser<'a, O = O> + 'a;

pub struct Context<'a> {
    pub source: &'a Shared<[u8]>,
    pub bytes: &'a [u8],
    parsers: FrozenMap<TypeId, Box<dyn Generated<'a> + 'a>>,
}

pub trait ParserExt<'a>: Parser<'a> {
    /// run is an alias for parse, to make it easier when the parser type has another 'parse' method
    fn run(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<Self::O> {
        self.parse(ctx, limit, pos)
    }

    fn map<T, F: Fn(Self::O) -> T>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
    {
        Map(self, f)
    }

    fn filter<F: Fn(&Self::O) -> bool>(self, f:F) -> Filter<Self,F> 
    where
        Self: Sized,
    {
        Filter(self, f)
    }

    fn then<T, F: Fn(Self::O) -> Option<T>>(self, f:F) -> FilterMap<Self,F> 
    where
        Self: Sized,
    {
        FilterMap(self, f)
    }

    fn recognize(self) -> Recognize<Self> 
    where
        Self:Sized,
    {
        Recognize(self)
    }

    fn many<R:RangeBounds<usize>>(self, r: R) -> Many<Self> 
    where
        Self:Sized,
    {
        Many::new(self, r)
    }
}

impl<'a, P: Parser<'a>> ParserExt<'a> for P {}

impl<'a, O, F> Parser<'a> for F
where
    F: Fn(&Context<'a>, usize, &mut usize) -> Option<O>,
{
    type O = O;
    fn parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<Self::O> {
        self(ctx, limit, pos)
    }
}

pub trait Generator<'a>: Any {
    type O;
    fn generate(ctx: &Context<'a>) -> Rc<dyn Parser<'a, O = Self::O> + 'a>;
}

struct GeneratedParser<'a, G: Generator<'a>>(Rc<dyn Parser<'a, O = G::O> + 'a>);

trait Generated<'a>: 'a {
    fn generated_by(&self) -> TypeId;
}
impl<'a, G: Generator<'a>> Generated<'a> for GeneratedParser<'a, G> {
    fn generated_by(&self) -> TypeId {
        TypeId::of::<G>()
    }
}
impl<'a> dyn Generated<'a> {
    fn downcast<G: Generator<'a>>(&self) -> Option<&Rc<dyn Parser<'a, O = G::O> + 'a>> {
        if TypeId::of::<G>() == self.generated_by() {
            return unsafe {
                let wrapper =
                    &*(self as *const dyn Generated<'a> as *const GeneratedParser<'a, G>);
                Some(&wrapper.0)
            };
        }

        None
    }
}

impl<'a> Context<'a> {
    pub fn new(source: &'a Shared<[u8]>) -> Self {
        let bytes = source.deref();
        Context {
            source,
            bytes,
            parsers: FrozenMap::new(),
        }
    }

    pub fn init_parser<G: Generator<'a>>(&self, parser: Rc<dyn Parser<'a, O = G::O> + 'a>) {
        let id = TypeId::of::<G>();

        assert!(self.parsers.get(&id).is_none());

        let parser = GeneratedParser::<'a, G>(parser);
        let parser = Box::new(parser);
        self.parsers.insert(id, parser);
    }

    pub fn parser<G: Generator<'a>>(&self) -> &Rc<dyn Parser<'a, O = G::O> + 'a> {
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

    pub fn parse<G: Generator<'a>>(&self, limit: usize, pos: &mut usize) -> Option<G::O> {
        let parser = self.parser::<G>();
        parser.parse(self, limit, pos)
    }
}
