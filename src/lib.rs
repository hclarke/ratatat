use core::any::{Any, TypeId};
use elsa::FrozenMap;

use core::borrow::Borrow;
use core::fmt::Debug;
use core::ops::RangeBounds;
use std::ops::Deref;
use std::rc::Rc;

#[macro_use]
mod test_macro {
    #[macro_export]
    macro_rules! assert_parse {
        ($exp: expr, $parser: expr, $input: expr) => {
            let input: &[u8] = $input.as_ref();
            let input = Rc::new(input.to_owned());
            let input = Shared::new(input);
            let ctx = Context::new(&input);
            let mut pos = 0;
            let res = $parser.parse(&ctx, ctx.bytes.len(), &mut pos);
            assert_eq!($exp, res);
        };
    }
}

mod parsers;
pub use parsers::*;

mod input;
pub use input::*;

#[cfg(feature = "traces")]
pub mod tracing;

pub trait Parser<'a>: Debug {
    type O: Debug;

    /// impl_parse should be implemented by parsers, but not called. call parse instead, which can do logging.
    fn impl_parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<Self::O>;

    fn name(&self) -> String {
        format!("{:?}", self)
    }

    fn is_named(&self) -> bool {
        false
    }
}

pub type DynParser<'a, O> = dyn Parser<'a, O = O> + 'a;

pub struct Context<'a> {
    pub source: &'a Shared<[u8]>,
    pub bytes: &'a [u8],
    parsers: FrozenMap<TypeId, Box<dyn Generated<'a> + 'a>>,

    #[cfg(feature = "traces")]
    tracer: core::cell::RefCell<tracing::Tracer>,
}

pub trait ParserExt<'a>: Parser<'a> {
    /// this is the parse method that should be called. it wraps impl_parse, and does logging/etc.
    fn parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<Self::O> {
        #[cfg(feature = "traces")]
        let trace_id = {
            let mut tracer = ctx.tracer.borrow_mut();
            tracer.enter(self, *pos)
        };

        let result = self.impl_parse(ctx, limit, pos);

        #[cfg(feature = "traces")]
        if let Some(trace_id) = trace_id {
            let mut tracer = ctx.tracer.borrow_mut();
            tracer.exit(self, trace_id, *pos, &result);
        }

        result
    }
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

    fn filter<F: Fn(&Self::O) -> bool>(self, f: F) -> Filter<Self, F>
    where
        Self: Sized,
    {
        Filter(self, f)
    }

    fn filter_map<T, F: Fn(Self::O) -> Option<T>>(self, f: F) -> FilterMap<Self, F>
    where
        Self: Sized,
    {
        FilterMap(self, f)
    }

    fn then<T: Parser<'a>, F: Fn(Self::O) -> T>(self, f: F) -> Then<Self, F>
    where
        Self: Sized,
    {
        Then(self, f)
    }

    fn recognize(self) -> Recognize<Self>
    where
        Self: Sized,
    {
        Recognize(self)
    }

    fn many<R: RangeBounds<usize>>(self, r: R) -> Many<Self>
    where
        Self: Sized,
    {
        Many::new(self, r)
    }

    fn named<N: Borrow<str> + Debug>(self, name: N) -> Named<Self, N>
    where
        Self: Sized,
    {
        Named(self, name)
    }

    fn opt(self) -> Opt<Self>
    where
        Self: Sized,
    {
        Opt(self)
    }

    fn not(self) -> Not<Self>
    where
        Self: Sized,
    {
        Not(self)
    }

    fn and<F:Parser<'a>>(self, f: F) -> And<Self, F> 
    where
        Self: Sized,
    {
        And(self, f)
    }

    fn and_not<F:Parser<'a>>(self, f:F) -> And<Self, Not<F>>  
    where
        Self: Sized,
    {
        self.and(f.not())
    }
}

impl<'a, P: Parser<'a> + ?Sized> ParserExt<'a> for P {}

pub trait Generator<'a>: Any {
    type O: Debug;
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
                let wrapper = &*(self as *const dyn Generated<'a> as *const GeneratedParser<'a, G>);
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

            #[cfg(feature = "traces")]
            tracer: core::cell::RefCell::new(tracing::Tracer::new()),
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

    #[cfg(feature = "traces")]
    pub fn trace_config(&mut self, config: Option<tracing::TraceConfig>) {
        self.tracer.get_mut().config = config;
    }

    #[cfg(feature = "traces")]
    pub fn trace(&self) -> core::cell::Ref<Vec<tracing::TraceElement>> {
        let tracer = self.tracer.borrow();
        core::cell::Ref::map(tracer, |t| &t.traces)
    }
}
