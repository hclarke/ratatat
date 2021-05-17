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
		let ws0 = Many(ws, ..);
		let ws1 = Many(ws, 1..);

		let value = parser::<JsonValue>();

		let non_quotes = Many(Filter(parser::<char>(), |c: &char| *c != '"'), ..);
		let non_quotes = FilterMap(Recognize(non_quotes), |bytes: &[u8]| String::from_utf8(bytes.to_vec()).ok());

		let string = ParserExt::map(('"', non_quotes, '"'), |x| x.1);
		let string = Rc::new(string);

		let pair = ParserExt::map((
			ws0.clone(), 
			string.clone(), 
			ws0.clone(), 
			':', 
			ws0.clone(), 
			value, 
			ws0.clone(),
		), |x| (x.1, x.5));

		let object = ParserExt::map((
			'{', 
			ParserExt::map(Sep(pair, ','), |kvs| {
				let mut m = HashMap::new();
				for (k,v) in kvs {
					m.insert(k,v);
				}
				m
			}),
			'}',
		), |x| x.1);

		let array = ParserExt::map(('[', Sep(value, ',') , ']'), |x| x.1);

		let boolean = parser::<bool>();

		let number = move |ctx: &Context<'a>, limit: usize, pos: &mut usize| -> Option<f64> {
			None
		};

		Rc::new(Alt((
			Map("null", |_| JsonValue::Null),
			Map(object, JsonValue::Object),
			Map(array, JsonValue::Array),
			Map(string, JsonValue::String),
			Map(number, JsonValue::Number),
			Map(boolean, JsonValue::Bool),
		)))
	}
}