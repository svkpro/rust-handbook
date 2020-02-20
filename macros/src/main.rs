/*
Captures are written as a dollar ($) followed by an identifier, a colon (:), and finally the kind of capture, which must be one of the following:

item: an item, like a function, struct, module, etc.
block: a block (i.e. a block of statements and/or an expression, surrounded by braces)
stmt: a statement
pat: a pattern
expr: an expression
ty: a type
ident: an identifier
path: a path (e.g. foo, ::std::mem::replace, transmute::<_, int>, …)
meta: a meta item; the things that go inside #[...] and #![...] attributes
tt: a single token tree
*/

macro_rules! multiple {
    ($a:tt, $b:tt) => {
        $a * $b
    };
}

fn main() {
    println!("{}", multiple!(10, 5))
}
