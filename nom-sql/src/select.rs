use nom::{self, multispace};

pub enum ConditionExpression {
    Field(String),
    Placeholder,
}

pub fn condition_expr<'a>(i: &'a [u8]) -> nom::IResult<&[u8], ConditionExpression, u32> {
    nom::IResult::Done(i, ConditionExpression::Placeholder)
}

pub struct SelectStatement {
    pub where_clause: Option<ConditionExpression>,
}

/// Parse WHERE clause of a selection
named!(pub where_clause<&[u8], ConditionExpression>,
    complete!(chain!(
        multispace? ~
        cond: condition_expr,
        || { cond }
    ))
);

/// Parse rule for a SQL selection query.
named!(pub selection<&[u8], SelectStatement>,
    chain!(
        select: chain!(
            tag!("x") ~
            cond: opt!(where_clause) ~
            || {
                SelectStatement {
                    where_clause: cond,
                }
            }
        ) ~
        tag!(";"),
        || { select }
    )
);
