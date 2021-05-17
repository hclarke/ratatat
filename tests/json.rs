use ratatat::*;
use std::rc::Rc;
use std::collections::HashMap;

enum JsonValue {
	Null,
	Object(HashMap<String, JsonValue>),
	Array(Vec<JsonValue>),
	String(String),
	Number(f64),
	Bool(bool),
}

impl<'a> Generator<'a> for JsonValue {
	type O = JsonValue;
	fn generate(_ctx: &Context<'a>) -> Rc<DynParser<'a, Self::O>> {

		let ws = [' ', '\n', '\r', '\t'];
		let ws0 = ws.many(..);
		let ws1 = ws.many(1..);

		let value = parser::<JsonValue>();

		let non_quotes = parser::<char>()
			.filter(|c| *c != '"')
			.many(..);

		let non_quotes = Recognize(non_quotes)
			.then(|bytes| String::from_utf8(bytes.to_vec()).ok());

		let string = ('"', non_quotes, '"').map(|x| x.1);

		let pair = ParserExt::map((
			ws0, 
			string, 
			ws0, 
			':', 
			ws0, 
			value, 
			ws0,
		), |x| (x.1, x.5));

		let object = (
			'{', 
			ParserExt::map(Sep(pair, ','), |kvs| {
				let mut m = HashMap::new();
				for (k,v) in kvs {
					m.insert(k,v);
				}
				m
			}),
			'}',
		).map(|x| x.1);

		let array = ('[', Sep(value, ',') , ']').map(|x| x.1);

		let boolean = parser::<bool>();

		let number = move |ctx: &Context<'a>, limit: usize, pos: &mut usize| -> Option<f64> {
			None
		};

		Rc::new(Alt((
			("null",).map(|_| JsonValue::Null),
			object.map(JsonValue::Object),
			array.map(JsonValue::Array),
			string.map(JsonValue::String),
			number.map(JsonValue::Number),
			boolean.map(JsonValue::Bool),
		)))
	}
}