use chumsky::error::Simple;
use chumsky::prelude::{end, just, recursive};
use chumsky::text::TextParser;
use chumsky::{text, Parser};
use std::cmp::Ordering;
use std::slice;

#[derive(Clone, Debug, Eq)]
pub enum Expr {
    Num(u8),
    List(Vec<Self>),
}

impl Ord for Expr {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Num(l), Self::Num(r)) => l.cmp(r),
            (num @ Self::Num(_l), Self::List(r)) => slice::from_ref(num).cmp(r.as_slice()),
            (Self::List(l), num @ Self::Num(_r)) => l.as_slice().cmp(slice::from_ref(num)),
            (Self::List(l), Self::List(r)) => l.cmp(r),
        }
    }
}

impl PartialOrd for Expr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

pub fn parser() -> impl Parser<char, Expr, Error = Simple<char>> {
    let rec = recursive(|value| {
        let int = text::int(10)
            .map(|s: String| Expr::Num(s.parse().unwrap()))
            .labelled("int");

        let list = value
            .separated_by(just(','))
            .delimited_by(just('['), just(']'))
            .map(Expr::List)
            .labelled("list");

        int.or(list)
    });

    rec.padded().then_ignore(end())
}
