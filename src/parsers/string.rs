use crate::parsers::numbe::Digit;
use crate::*;
use std::collections::HashMap;

pub struct EscapeChar;

impl<'a> Generator<'a> for EscapeChar {
    type O = char;
    fn generate(ctx: &Context<'a>) -> Rc<DynParser<'a, Self::O>> {
        let ch = ctx.parser::<char>().clone();

        let mut esc_map = HashMap::new();
        esc_map.insert('\'', '\'');
        esc_map.insert('"', '"');
        esc_map.insert('\\', '\\');
        esc_map.insert('n', '\n');
        esc_map.insert('r', '\r');
        esc_map.insert('t', '\t');
        esc_map.insert('0', '\0');

        let esc_simple = ch.clone().filter_map(move |c| esc_map.get(&c).map(|v| *v));

        let esc_hex = ('x', Digit(16).many(2..=2)).filter_map(|(_, digits)| {
            let val = (digits[0] << 4) | digits[1];
            core::char::from_u32(val as u32)
        });

        let esc_unicode = ("u{", Digit(16).many(1..=6), '}').filter_map(|(_, digits, _)| {
            let mut val = 0;
            for d in digits {
                val *= 16;
                val += d as u32;
            }

            core::char::from_u32(val)
        });

        let esc = ('\\', Alt((esc_simple, esc_hex, esc_unicode))).map(|(_, c)| c);

        Rc::new(esc)
    }
}

impl<'a> Generator<'a> for String {
    type O = String;
    fn generate(ctx: &Context<'a>) -> Rc<DynParser<'a, Self::O>> {
        let esc = ctx.parser::<EscapeChar>().clone();
        let ch = ctx.parser::<char>().clone();

        let str_char = Alt((esc, ch.filter(|c| *c != '"' && *c != '\\')));

        let parser = ('"', str_char.many(..), '"').map(|(_, chars, _)| {
            let mut s = String::new();
            for c in chars {
                s.push(c);
            }
            s
        });

        Rc::new(parser)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn string_parser() {
        assert_parse!(
            Some("hello".to_string()),
            parser::<String>(),
            r##""hello""##
        );
        assert_parse!(
            Some("hey\nworld".to_string()),
            parser::<String>(),
            r##""hey\nworld""##
        );
        assert_parse!(None, parser::<String>(), r##""unclosed\""##);
    }
}
