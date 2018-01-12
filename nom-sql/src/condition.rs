use nom::multispace;
use nom::{Err, ErrorKind, IResult, Needed};
use std::collections::{HashSet, VecDeque};
use std::str;
use std::fmt;

use column::Column;
use common::{binary_comparison_operator, column_identifier, integer_literal, string_literal,
             Literal, Operator};

#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
pub enum ConditionBase {
    Field(Column),
    Literal(Literal),
    Placeholder,
}

impl fmt::Display for ConditionBase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConditionBase::Field(ref col) => write!(f, "{}", col),
            ConditionBase::Literal(ref literal) => write!(f, "{}", literal.to_string()),
            ConditionBase::Placeholder => write!(f, "?"),
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct ConditionTree {
    pub operator: Operator,
    pub left: Box<ConditionExpression>,
    pub right: Box<ConditionExpression>,
}

impl<'a> ConditionTree {
    pub fn contained_columns(&'a self) -> HashSet<&'a Column> {
        let mut s = HashSet::new();
        let mut q = VecDeque::<&'a ConditionTree>::new();
        q.push_back(self);
        while let Some(ref ct) = q.pop_front() {
            match *ct.left.as_ref() {
                ConditionExpression::Base(ConditionBase::Field(ref c)) => {
                    s.insert(c);
                }
                ConditionExpression::LogicalOp(ref ct)
                | ConditionExpression::ComparisonOp(ref ct) => q.push_back(ct),
                _ => (),
            }
            match *ct.right.as_ref() {
                ConditionExpression::Base(ConditionBase::Field(ref c)) => {
                    s.insert(c);
                }
                ConditionExpression::LogicalOp(ref ct)
                | ConditionExpression::ComparisonOp(ref ct) => q.push_back(ct),
                _ => (),
            }
        }
        s
    }
}

impl fmt::Display for ConditionTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.left)?;
        write!(f, " {} ", self.operator)?;
        write!(f, "{}", self.right)
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
pub enum ConditionExpression {
    ComparisonOp(ConditionTree),
    LogicalOp(ConditionTree),
    NegationOp(Box<ConditionExpression>),
    Base(ConditionBase),
}

impl fmt::Display for ConditionExpression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConditionExpression::ComparisonOp(ref tree) => write!(f, "{}", tree),
            ConditionExpression::LogicalOp(ref tree) => write!(f, "{}", tree),
            ConditionExpression::NegationOp(ref expr) => write!(f, "NOT {}", expr),
            ConditionExpression::Base(ref base) => write!(f, "{}", base),
        }
    }
}

/// Parse a conditional expression into a condition tree structure
named!(pub condition_expr<&[u8], ConditionExpression>,
       alt_complete!(
           chain!(
               left: and_expr ~
               multispace? ~
               caseless_tag!("or") ~
               multispace ~
               right: condition_expr,
               || {
                   ConditionExpression::LogicalOp(
                       ConditionTree {
                           operator: Operator::Or,
                           left: Box::new(left),
                           right: Box::new(right),
                       }
                   )
               }
           )
       |   and_expr)
);

named!(pub and_expr<&[u8], ConditionExpression>,
       alt_complete!(
           chain!(
               left: parenthetical_expr ~
               multispace? ~
               caseless_tag!("and") ~
               multispace ~
               right: and_expr,
               || {
                   ConditionExpression::LogicalOp(
                       ConditionTree {
                           operator: Operator::And,
                           left: Box::new(left),
                           right: Box::new(right),
                       }
                   )
               }
           )
       |   parenthetical_expr)
);

named!(pub parenthetical_expr<&[u8], ConditionExpression>,
       alt_complete!(
           delimited!(tag!("("), condition_expr, chain!(tag!(")") ~ multispace?, ||{}))
       |   not_expr)
);

named!(pub not_expr<&[u8], ConditionExpression>,
       alt_complete!(
           chain!(
               caseless_tag!("not") ~
               multispace ~
               right: parenthetical_expr,
               || {
                   ConditionExpression::NegationOp(Box::new(right))
               }
           )
       |   boolean_primary)
);

named!(boolean_primary<&[u8], ConditionExpression>,
    chain!(
        left: predicate ~
        multispace? ~
        op: binary_comparison_operator ~
        multispace? ~
        right: predicate,
        || {
            ConditionExpression::ComparisonOp(
                ConditionTree {
                    operator: op,
                    left: Box::new(left),
                    right: Box::new(right),
                }
            )

        }
    )
);

named!(predicate<&[u8], ConditionExpression>,
    delimited!(
        opt!(multispace),
        alt_complete!(
                chain!(
                    tag!("?"),
                    || {
                        ConditionExpression::Base(
                            ConditionBase::Placeholder
                        )
                    }
                )
            |   chain!(
                    field: integer_literal,
                    || {
                        ConditionExpression::Base(ConditionBase::Literal(field))
                    }
                )
            |   chain!(
                    field: string_literal,
                    || {
                        ConditionExpression::Base(ConditionBase::Literal(field))
                    }
                )
            |   chain!(
                    field: column_identifier,
                    || {
                        ConditionExpression::Base(
                            ConditionBase::Field(field)
                        )
                    }
                )
        ),
        opt!(multispace)
    )
);
