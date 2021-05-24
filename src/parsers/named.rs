use crate::*;
use crate::tracing::TraceLevel;
use core::borrow::Borrow;

#[derive(Debug, Copy, Clone)]
pub struct Named<P,N>(pub P, pub N);

impl<'a, P:Parser<'a>, N:Borrow<str>+Debug> Parser<'a> for Named<P,N> {
	type O = P::O;
	fn impl_parse(&self, ctx: &Context<'a>, limit: usize, pos: &mut usize) -> Option<Self::O> {
		self.0.parse(ctx, limit, pos)
	}

	fn name(&self) -> String {
		self.1.borrow().to_owned()
	}

	fn trace_level(&self) -> TraceLevel {
		TraceLevel::Explicit
	}
}