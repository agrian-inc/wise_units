use super::{Component, Error, Finishable, Term, Visit};
use crate::{invert::Invert, parser::terms::term_parser::Rule};
use pest::iterators::Pair;

pub(super) struct AstTerm {
    pub(super) component: Component,
    pub(super) terms: Vec<Term>,
}

impl AstTerm {
    pub(super) fn visit(pair: Pair<'_, Rule>) -> Result<Self, Error> {
        let mut pairs = pair.into_inner();

        let component = match pairs.next() {
            Some(first) => match first.as_rule() {
                Rule::component => Component::visit(first)?,
                _ => unreachable!(),
            },
            None => unreachable!(),
        };

        let op = match pairs.next() {
            Some(second) => match second.as_rule() {
                Rule::dot => SecondToken::Dot,
                Rule::slash => SecondToken::Slash,
                _ => unreachable!(),
            },
            None => {
                return Ok(Self {
                    component,
                    terms: Vec::with_capacity(0),
                });
            }
        };

        match pairs.next() {
            Some(third) => match third.as_rule() {
                Rule::term => {
                    let mut new_terms: Vec<Term> = Self::visit(third)?.finish();

                    match op {
                        SecondToken::Dot => (),
                        SecondToken::Slash => {
                            new_terms.invert();
                        }
                    }

                    Ok(Self {
                        component,
                        terms: new_terms,
                    })
                }
                _ => unreachable!(),
            },
            None => unreachable!(),
        }
    }
}

enum SecondToken {
    Dot,
    Slash,
}

impl Finishable for AstTerm {
    fn finish(self) -> Vec<Term> {
        let mut component_terms = self.component.finish();
        component_terms.extend(self.terms.into_iter());

        component_terms
    }
}
