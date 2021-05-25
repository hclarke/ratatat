use crate::*;
use core::ops::Range;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum TraceLevel {
    None,
    Explicit,
    Normal,
    All,
}

#[derive(Debug)]
pub struct TraceConfig {
    pub depth: Option<usize>,
    pub level: TraceLevel,
}

#[derive(Debug)]
pub struct TraceElement {
    pub parent: isize,
    pub depth: usize,
    pub done: bool,
    pub name: String,
    pub pass: bool,
    pub span: Range<usize>,
    pub level: TraceLevel,
}

#[derive(Debug)]
pub struct Tracer {
    pub config: Option<TraceConfig>,
    pub traces: Vec<TraceElement>,
    current: isize,
    depth: usize,
}

#[derive(Debug)]
pub(crate) struct TraceId(usize);

impl Tracer {
    pub(crate) fn new() -> Self {
        Tracer {
            config: None,
            traces: Vec::new(),
            current: -1,
            depth: 0,
        }
    }

    pub(crate) fn enter<'a, P: Parser<'a> + ?Sized>(
        &mut self,
        parser: &P,
        pos: usize,
    ) -> Option<TraceId> {
        if self.config.is_none() {
            return None;
        }

        let config = self.config.as_ref().unwrap();

        let depth_limit = config.depth;
        if depth_limit.is_some() && self.depth >= depth_limit.unwrap() {
            return None;
        }

        let level = match parser.is_named() {
            true => TraceLevel::Explicit,
            false => TraceLevel::Normal,
        };

        if config.level < level {
            return None;
        }

        let id = self.traces.len();
        self.traces.push(TraceElement {
            parent: self.current,
            done: false,
            name: parser.name(),
            pass: false,
            span: pos..pos,
            depth: self.depth + 1,
            level,
        });
        self.current = id as isize;
        self.depth += 1;

        Some(TraceId(id))
    }

    pub(crate) fn exit<'a, P: Parser<'a> + ?Sized>(
        &mut self,
        _parser: &P,
        id: TraceId,
        pos: usize,
        result: &Option<P::O>,
    ) {
        let element = &mut self.traces[id.0];
        assert!(!element.done);
        element.span.end = pos;
        element.pass = result.is_some();
        element.done = true;
        self.current = element.parent;
        self.depth -= 1;
    }
}
