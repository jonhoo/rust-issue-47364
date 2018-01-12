use nom::multispace;
use nom::{Err, ErrorKind, IResult, Needed};
use std::str;

use common::FieldExpression;
use common::{field_definition_expr, field_list, statement_terminator, table_list, table_reference};
use condition::{condition_expr, ConditionExpression};
use join::{join_operator, JoinConstraint, JoinOperator, JoinRightSide};
use table::Table;

#[derive(Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
pub struct JoinClause {
    pub operator: JoinOperator,
    pub right: JoinRightSide,
    pub constraint: JoinConstraint,
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Serialize, Deserialize)]
pub struct SelectStatement {
    pub tables: Vec<Table>,
    pub fields: Vec<FieldExpression>,
    pub join: Vec<JoinClause>,
    pub where_clause: Option<ConditionExpression>,
}

/// Parse JOIN clause
named!(join_clause<&[u8], JoinClause>,
    complete!(chain!(
        multispace? ~
        _natural: opt!(caseless_tag!("natural")) ~
        multispace? ~
        op: join_operator ~
        multispace ~
        right: join_rhs ~
        multispace ~
        constraint: alt_complete!(
              chain!(
                  caseless_tag!("using") ~
                  multispace ~
                  fields: delimited!(tag!("("), field_list, tag!(")")),
                  || {
                      JoinConstraint::Using(fields)
                  }
              )
            | chain!(
                  caseless_tag!("on") ~
                  multispace ~
                  cond: alt_complete!(delimited!(tag!("("), condition_expr, tag!(")"))
                                      | condition_expr),
                  || {
                      JoinConstraint::On(cond)
                  }
              )
        ),
    || {
        JoinClause {
            operator: op,
            right: right,
            constraint: constraint,
        }
    }))
);

/// Different options for the right hand side of the join operator in a `join_clause`
named!(join_rhs<&[u8], JoinRightSide>,
    alt_complete!(
        complete!(chain!(
              table: table_reference,
              || {
                  JoinRightSide::Table(table)
              }
          ))
        | complete!(chain!(
              tables: delimited!(tag!("("), table_list, tag!(")")),
              || {
                  JoinRightSide::Tables(tables)
              }
          ))
    )
);

/// Parse WHERE clause of a selection
named!(pub where_clause<&[u8], ConditionExpression>,
    complete!(chain!(
        multispace? ~
        caseless_tag!("where") ~
        cond: condition_expr,
        || { cond }
    ))
);

/// Parse rule for a SQL selection query.
named!(pub selection<&[u8], SelectStatement>,
    chain!(
        select: nested_selection ~
        statement_terminator,
        || { select }
    )
);

named!(pub nested_selection<&[u8], SelectStatement>,
    chain!(
        caseless_tag!("select") ~
        multispace ~
        fields: field_definition_expr ~
        delimited!(opt!(multispace), caseless_tag!("from"), opt!(multispace)) ~
        tables: table_list ~
        join: many0!(join_clause) ~
        cond: opt!(where_clause) ~
        || {
            SelectStatement {
                tables: tables,
                fields: fields,
                join: join,
                where_clause: cond,
            }
        }
    )
);
