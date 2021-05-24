use ratatat::*;

use indexmap::IndexMap;
use std::rc::Rc;

#[derive(Debug, serde::Serialize)]
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
        let ws = [' ', '\n', '\r', '\t'].many(..).named("ws");

        let value = parser::<JsonValue>();

        let non_quotes = parser::<char>().filter(|c| *c != '"').many(..);

        let non_quotes = Recognize(non_quotes).then(|bytes| String::from_utf8(bytes.to_vec()).ok());

        let string = ('"', non_quotes, '"').map(|x| x.1).named("string");

        let pair = ParserExt::map((ws, string, ws, ':', value), |x| (x.1, x.4)).named("pair");

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

        let array = ('[', Sep(value.named("element"), ','), ']').map(|x| x.1);

        let boolean = parser::<bool>();

        let number = FnParser(
        	move |_ctx: &Context<'a>, _limit: usize, _pos: &mut usize| -> Option<f64> { None },
        	Some("number")
        );

        let value = Alt((
            ("null",).map(|_| JsonValue::Null).named("null"),
            object.map(JsonValue::Object).named("object"),
            array.map(JsonValue::Array).named("array"),
            string.map(JsonValue::String),
            number.map(JsonValue::Number).named("number"),
            boolean.map(JsonValue::Bool).named("bool"),
        ));

        Rc::new((ws, value, ws).map(|x| x.1).named("value"))
    }
}

#[test]
fn parse_json() {
    insta::glob!("json_inputs/*.json", |path| {
        let file = std::fs::read(path).unwrap();
        let input = Shared::new(Rc::new(file));
        let mut ctx = Context::new(&input);
        let mut pos = 0;
        let val = ctx.parse::<JsonValue>(ctx.bytes.len(), &mut pos);
        insta::assert_ron_snapshot!(val);
    });
}
