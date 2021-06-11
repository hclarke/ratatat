use ratatat::*;
use std::rc::Rc;
use indexmap::IndexMap;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct KdlNode {
	name: String,
	args: Vec<KdlValue>,
	vals: IndexMap<String, KdlValue>,
	body: Vec<KdlNode>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum KdlValue {
	Null,
	String(String),
	Number(f64),
	Bool(bool),
}


#[derive(Debug)]
pub struct KdlString;

#[derive(Debug)]
pub struct KdlIdentifier;

#[derive(Debug)]
pub struct KdlNodes;

#[derive(Debug, Clone, PartialEq)]
pub enum KdlArg {
	Arg(KdlValue),
	Val(String, KdlValue),
}

impl<'a> Generator<'a> for KdlString {
	type O = String;
	fn generate(ctx: &Context<'a>) -> Rc<DynParser<'a, Self::O>> {

		let ch = ctx.parser::<char>().clone();

		let raw_string = ('r', '#'.many(..), '"').then(move |(_,hashes,_)| {
			let len = hashes.len();
			let end = ('"', '#'.many(len..=len));

			let not_end = (Not(end), ch.clone()).map(|(_, c)| c);

			(not_end.many(..).recognize(), end).filter_map(|(chars, _)| {
				core::str::from_utf8(chars).ok().map(|s| s.to_string())
			})
		});
		

		let parser = Alt((
			ctx.parser::<String>().clone(),
			raw_string,
		));

		Rc::new(parser)
	}
}

impl<'a> Generator<'a> for KdlIdentifier {
	type O = String;
	fn generate(ctx: &Context<'a>) -> Rc<DynParser<'a, Self::O>> {

		let non_id_char = Alt((
			..='\x20',
			['\\', '<', '>', '{', '}', ';', '[', ']', '=', ',', '"'],
		));

		let non_initial_char = Alt((
			'0'..='9',
			non_id_char.clone(),
		));

		let ch = ctx.parser::<char>();

		let bare_identifier = ( 
			ch.clone().and(non_initial_char.not()), 
			ch.clone().and_not(non_id_char).many(..),
		).recognize().filter_map(|chars| {
			core::str::from_utf8(chars).ok().map(|s| s.to_string())
		});

		let parser = Alt((
			ctx.parser::<KdlString>().clone(),
			bare_identifier,
		));

		Rc::new(parser)
	}
}

impl<'a> Generator<'a> for KdlValue {
	type O = KdlValue;
	fn generate(ctx: &Context<'a>) -> Rc<DynParser<'a, Self::O>> {
		let parser = Alt((
			("null",).map(|_| KdlValue::Null),
			ctx.parser::<bool>().clone().map(KdlValue::Bool),
			ctx.parser::<f64>().clone().map(KdlValue::Number),
			ctx.parser::<KdlString>().clone().map(KdlValue::String),
		));

		Rc::new(parser)
	}
}

impl<'a> Generator<'a> for KdlArg {
	type O = KdlArg;
	fn generate(ctx: &Context<'a>) -> Rc<DynParser<'a, Self::O>> {

		let value = ctx.parser::<KdlValue>();

		let val = (parser::<KdlIdentifier>(), '=', value.clone())
			.map(move |(k,_,v)| KdlArg::Val(k,v));

		let arg = value.clone().map(KdlArg::Arg);

		let parser = Alt((arg, val));

		Rc::new(parser)
	}
}

impl<'a> Generator<'a> for KdlNodes {
	type O = Vec<KdlNode>;
	fn generate(ctx: &Context<'a>) -> Rc<DynParser<'a, Self::O>> {
		let node = ctx.parser::<KdlNode>().clone();

		let ws = Alt((
			[' ', '\t'],
			//TODO: comments
			//TODO: more unicode whitespace
		)).recognize();

		let newline = ["\r\n", "\n", "\r"].recognize();

		let linespace = Alt((ws, newline));

		let ws = linespace.many(..).recognize();

		let parser = (ws, node, ws).map(|(_, node, _)| node).many(..);

		Rc::new(parser)
	}
}

impl<'a> Generator<'a> for KdlNode {
	type O = KdlNode;
	fn generate(ctx: &Context<'a>) -> Rc<DynParser<'a, Self::O>> {

		let name = ctx.parser::<KdlIdentifier>().clone();

		let arg = ctx.parser::<KdlArg>().clone();


		let ws = Alt((
			[' ', '\t'],
			//TODO: comments
			//TODO: more unicode whitespace
		)).recognize();

		let newline = ["\r\n", "\n", "\r"];

		let escline = ("\\", ws.many(..), newline).recognize();

		let node_space = Alt((
			(ws.many(..), escline, ws.many(..)).recognize(),
			ws.many(1..).recognize(),
		));

		let node_terminator = Alt((";", newline));

		let args = (node_space, arg).map(|(_, arg)| arg).many(..);
		let body = (node_space.many(..), '{',  parser::<KdlNodes>(), '}')
			.map(|(_, _, children, _)| children)
			.opt();


		let parser = (name, args, body, node_terminator)
			.map(move |(name, argv, body, _)| {

				let mut args = Vec::new();
				let mut vals = IndexMap::new();

				for arg in argv {
					match arg {
						KdlArg::Arg(a) => { args.push(a); },
						KdlArg::Val(k,v) => { vals.insert(k,v); },
					}
				}

				let body = body.unwrap_or_else(|| Vec::new());

				KdlNode {
					name,
					args,
					vals,
					body,
				}
			});

		Rc::new(parser)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use insta::*;

    #[test]
    fn kdl_parse_string() {
        assert_parse!(Some("hello".to_string()), parser::<KdlString>(), r#""hello""#);
        assert_parse!(Some("hello\nworld".to_string()), parser::<KdlString>(), r#""hello\nworld""#);
        assert_parse!(Some(r#""hello""#.to_string()), parser::<KdlString>(), r###"r#""hello""#"###);
    }

    #[test]
    fn kdl_parse_identifier() {
        assert_parse!(Some("hello".to_string()), parser::<KdlIdentifier>(), r#""hello""#);
        assert_parse!(Some("hello\nworld".to_string()), parser::<KdlIdentifier>(), r#""hello\nworld""#);
        assert_parse!(Some(r#""hello""#.to_string()), parser::<KdlIdentifier>(), r###"r#""hello""#"###);
        assert_parse!(Some("_hello_".to_string()), parser::<KdlIdentifier>(), "_hello_");
    }

    #[test]
    fn kdl_parse_value() {
        assert_parse!(Some(KdlValue::Null), parser::<KdlValue>(), "null");
        assert_parse!(Some(KdlValue::Number(2.0)), parser::<KdlValue>(), "2");
        assert_parse!(Some(KdlValue::String("hello".to_string())), parser::<KdlValue>(), r#""hello""#);
        assert_parse!(Some(KdlValue::Bool(true)), parser::<KdlValue>(), r#"true"#);

    }

    #[test]
    fn kdl_snapshot() {
    	insta::glob!("kdl_input/*.kdl", |path| {
        let file = std::fs::read(path).unwrap();
        let input = Shared::new(Rc::new(file));
        let ctx = Context::new(&input);
        let mut pos = 0;
        let vals = ctx.parse::<KdlNodes>(ctx.bytes.len(), &mut pos);
        insta::assert_ron_snapshot!(vals);
    });
    }
}
