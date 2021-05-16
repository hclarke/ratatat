use core::any::{Any, TypeId};
use elsa::FrozenMap;

use core::borrow::Borrow;
use std::rc::Rc;

mod parsers;
pub use parsers::*;

pub trait Parser<'a, I: ?Sized> {
    type O;
    fn parse(&self, ctx: &Context<'a, I>, limit: usize, pos: &mut usize) -> Option<Self::O>;
}

pub type DynParser<'a, I, O> = dyn Parser<'a, I, O = O> + 'a;

pub struct Context<'a, I: ?Sized> {
    pub source: &'a I,
    pub bytes: &'a [u8],
    parsers: FrozenMap<TypeId, Box<dyn Generated<'a, I> + 'a>>,
}

pub trait ParserExt<'a, I: ?Sized>: Parser<'a, I> {
    /// run is an alias for parse, to make it easier when the parser type has another 'parse' method
    fn run(&self, ctx: &Context<'a, I>, limit: usize, pos: &mut usize) -> Option<Self::O> {
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

    fn parse_bytes(&self, input: &'a I) -> Option<Self::O>
    where
        I: Borrow<[u8]>,
    {
        let ctx = Context::from_bytes(input);
        self.parse(&ctx, ctx.bytes.len(), &mut 0)
    }

    fn parse_str(&self, input: &'a I) -> Option<Self::O>
    where
        I: Borrow<str>,
    {
        let ctx = Context::from_str(input);
        self.parse(&ctx, ctx.bytes.len(), &mut 0)
    }
}

impl<'a, I: ?Sized, P: Parser<'a, I>> ParserExt<'a, I> for P {}

impl<'a, I: ?Sized, O, F> Parser<'a, I> for F
where
    F: Fn(&Context<'a, I>, usize, &mut usize) -> Option<O>,
{
    type O = O;
    fn parse(&self, ctx: &Context<'a, I>, limit: usize, pos: &mut usize) -> Option<Self::O> {
        self(ctx, limit, pos)
    }
}

pub trait Generator<'a, I: ?Sized>: Any {
    type O;
    fn generate(ctx: &Context<'a, I>) -> Rc<dyn Parser<'a, I, O = Self::O> + 'a>;
}

struct GeneratedParser<'a, I: ?Sized, G: Generator<'a, I>>(Rc<dyn Parser<'a, I, O = G::O> + 'a>);

trait Generated<'a, I: ?Sized>: 'a {
    fn generated_by(&self) -> TypeId;
}
impl<'a, I: ?Sized + 'a, G: Generator<'a, I>> Generated<'a, I> for GeneratedParser<'a, I, G> {
    fn generated_by(&self) -> TypeId {
        TypeId::of::<G>()
    }
}
impl<'a, I: ?Sized + 'a> dyn Generated<'a, I> {
    fn downcast<G: Generator<'a, I>>(&self) -> Option<&Rc<dyn Parser<'a, I, O = G::O> + 'a>> {
        if TypeId::of::<G>() == self.generated_by() {
            return unsafe {
                let wrapper =
                    &*(self as *const dyn Generated<'a, I> as *const GeneratedParser<'a, I, G>);
                Some(&wrapper.0)
            };
        }

        None
    }
}

impl<'a, I: ?Sized> Context<'a, I> {
    pub fn new(source: &'a I, bytes: &'a [u8]) -> Self {
        Context {
            source,
            bytes,
            parsers: FrozenMap::new(),
        }
    }

    pub fn from_str(input: &'a I) -> Self
    where
        I: Borrow<str>,
    {
        let src = input.borrow();
        let src = src.as_ref();
        Context::new(input, src)
    }

    pub fn from_bytes(input: &'a I) -> Self
    where
        I: Borrow<[u8]>,
    {
        let src = input.borrow();
        Context::new(input, src)
    }

    pub fn init_parser<G: Generator<'a, I>>(&self, parser: Rc<dyn Parser<'a, I, O = G::O> + 'a>) {
        let id = TypeId::of::<G>();

        assert!(self.parsers.get(&id).is_none());

        let parser = GeneratedParser::<'a, I, G>(parser);
        let parser = Box::new(parser);
        self.parsers.insert(id, parser);
    }

    pub fn parser<G: Generator<'a, I>>(&self) -> &Rc<dyn Parser<'a, I, O = G::O> + 'a> {
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

    pub fn parse<G: Generator<'a, I>>(&self, limit: usize, pos: &mut usize) -> Option<G::O> {
        let parser = self.parser::<G>();
        parser.parse(self, limit, pos)
    }
}
