use crate::syntax::namespace::Namespace;
use crate::syntax::Pair;
use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use std::fmt::{self, Display, Write};

// A mangled symbol consisting of segments separated by '$'.
// For example: cxxbridge05$string$new
pub struct Symbol(String);

impl Display for Symbol {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.0, formatter)
    }
}

impl ToTokens for Symbol {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        ToTokens::to_tokens(&self.0, tokens);
    }
}

impl Symbol {
    fn push(&mut self, segment: &dyn Display) {
        let len_before = self.0.len();
        if !self.0.is_empty() {
            self.0.push('$');
        }
        self.0.write_fmt(format_args!("{}", segment)).unwrap();
        assert!(self.0.len() > len_before);
    }

    pub fn from_idents<'a, T: Iterator<Item = &'a Ident>>(it: T) -> Self {
        let mut symbol = Symbol(String::new());
        for segment in it {
            segment.write(&mut symbol);
        }
        assert!(!symbol.0.is_empty());
        symbol
    }

    /// For example, for taking a symbol and then making a new symbol
    /// for a vec of that symbol.
    pub fn prefix_with(&self, prefix: &str) -> Symbol {
        Symbol(format!("{}{}", prefix, self.to_string()))
    }
}

pub trait Segment {
    fn write(&self, symbol: &mut Symbol);
}

impl Segment for str {
    fn write(&self, symbol: &mut Symbol) {
        symbol.push(&self);
    }
}
impl Segment for usize {
    fn write(&self, symbol: &mut Symbol) {
        symbol.push(&self);
    }
}
impl Segment for Ident {
    fn write(&self, symbol: &mut Symbol) {
        symbol.push(&self);
    }
}
impl Segment for Symbol {
    fn write(&self, symbol: &mut Symbol) {
        symbol.push(&self);
    }
}

impl Segment for Namespace {
    fn write(&self, symbol: &mut Symbol) {
        for segment in self {
            symbol.push(segment);
        }
    }
}

impl Segment for Pair {
    fn write(&self, symbol: &mut Symbol) {
        self.namespace.write(symbol);
        self.cxx.write(symbol);
    }
}

impl<T> Segment for &'_ T
where
    T: ?Sized + Segment + Display,
{
    fn write(&self, symbol: &mut Symbol) {
        (**self).write(symbol);
    }
}

pub fn join(segments: &[&dyn Segment]) -> Symbol {
    let mut symbol = Symbol(String::new());
    for segment in segments {
        segment.write(&mut symbol);
    }
    assert!(!symbol.0.is_empty());
    symbol
}
