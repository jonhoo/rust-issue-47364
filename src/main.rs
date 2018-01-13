#[macro_use]
extern crate nom;
use nom::multispace;

pub enum ConditionExpression {
    Field(String),
    Placeholder,
}

pub fn condition_expr<'a>(i: &'a [u8]) -> nom::IResult<&[u8], ConditionExpression, u32> {
    nom::IResult::Done(i, ConditionExpression::Placeholder)
}

named!(pub where_clause<&[u8], ConditionExpression>,
    complete!(chain!(
        multispace? ~
        cond: condition_expr,
        || { cond }
    ))
);

named!(pub selection<&[u8], Option<ConditionExpression>>,
    chain!(
        select: chain!(
            tag!("x") ~
            cond: opt!(where_clause) ~
            || { cond }
        ) ~
        tag!(";"),
        || { select }
    )
);

fn main() {
    selection("x ".as_bytes());
}
