use ratatat::*;
use std::collections::HashMap;
use std::rc::Rc;
use indexmap::IndexMap;

#[derive(serde::Serialize)]
enum JsonValue {
    Null,
    Object(IndexMap<String, JsonValue>), // uses indexmap for consistent yaml ordering
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
        let _ws1 = ws.many(1..);

        let value = parser::<JsonValue>();

        let non_quotes = parser::<char>().filter(|c| *c != '"').many(..);

        let non_quotes = Recognize(non_quotes).then(|bytes| String::from_utf8(bytes.to_vec()).ok());

        let string = ('"', non_quotes, '"').map(|x| x.1);

        let pair = ParserExt::map((ws0, string, ws0, ':', ws0, value, ws0), |x| (x.1, x.5));

        let object = (
            '{',
            ParserExt::map(Sep(pair, ','), |kvs| {
                let mut m = IndexMap::new();
                for (k, v) in kvs {
                    m.insert(k, v);
                }
                m
            }),
            '}',
        )
            .map(|x| x.1);

        let array = ('[', Sep(value, ','), ']').map(|x| x.1);

        let boolean = parser::<bool>();

        let number =
            move |_ctx: &Context<'a>, _limit: usize, _pos: &mut usize| -> Option<f64> { None };

        let value = Alt((
            ("null",).map(|_| JsonValue::Null),
            object.map(JsonValue::Object),
            array.map(JsonValue::Array),
            string.map(JsonValue::String),
            number.map(JsonValue::Number),
            boolean.map(JsonValue::Bool),
        ));


        Rc::new((ws0, value, ws0).map(|x| x.1))
    }
}

#[test]
fn parse_json() {
	insta::glob!("json_inputs/*.json", |path|{
		let file = std::fs::read(path).unwrap();
		let input = Shared::new(Rc::new(file));
		let ctx = Context::new(&input);
		let mut pos = 0;
		let val = ctx.parse::<JsonValue>(ctx.bytes.len(), &mut pos);
		insta::assert_ron_snapshot!(val);
	});

}
