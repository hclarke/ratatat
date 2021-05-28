# ratatat

a parser combinator library for rust

## goals

- asymptotically fast parsers
- practically fast for human-written files
- easy to debug
- syntax should look kinda like formal grammar
- quick to experiment with

## non-goals

- fast for large data files
- zero copy/allocation
- general over input types (it's `[u8]` only)

# concepts

steps for parsing a file:
- read the file
- convert to an `Input`
- create a `Context` from the input
- run a parser on the context
- throw away the context and the input
- use the results

## Input

an input is a `[u8]` backed by a reference counted vec. this allows parsers to hold on to spans of the input, rather than copy them.

## Context

a context has a reference to the input, and a cache of parsers. this allows parsers to precompute things for the entire input, memoize results, etc.

once parsing is complete, the context can be thrown away

## Parser

a parser has a parse method that takes a context, a mutable position, and returns an optional result.

there are many built-in parsers, and parser combinators. for example:
- `str` is a parser, which returns itself if the input contains it at the position
- `char` behaves similarly
- `Map(parser,func)` runs a parser, and on success, applies a function to its result
- `Filter(parser, func)` runs a parser, then filters results by a function

## Generator

a parser generator generates a parser for a type. this is the main way to make use of caching parsers in a `Context`

you can get a cached parser with `context.parser::<Generator>()`
or parse immediately with `context.parse::<Generator>(limit, &mut position)`

there are also many built-in generators:
- `char` generates a parser that parses any char
- `[Generator;N]` generates a parser that parses a fixed-size array of values
- `Memo<Generator>` generates a memoized parser from another generator

# library structure

Take a look at the examples to see how it works. the json parser is a good place to start. usually, you'll want to implement `Generator<YourType>` for the root of your AST.

the core traits and types are in `src/lib.rs`

and the built-in parsers and combinators are in `src/parsers/`
