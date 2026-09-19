//! Query Filter DSL parser and AST evaluator.
//!
//! Enables instantaneous filtering and selection of dense PCB primitives using
//! natural engineering filter queries, e.g.:
//! `IsTrack AND Net = 'DDR4*' AND Layer = 'Top' AND Width >= 0.2`

use serde::{Deserialize, Serialize};

/// Target object primitive type in query predicate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueryTargetType {
    Track,
    Via,
    Pad,
    Footprint,
    Zone,
    Text,
}

/// Comparison operator for numeric and string fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComparisonOp {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Like, // Wildcard pattern match e.g. "DDR4_*"
}

/// Single query filter predicate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QueryPredicate {
    IsType(QueryTargetType),
    StringField {
        field: String,
        op: ComparisonOp,
        value: String,
    },
    NumericField {
        field: String,
        op: ComparisonOp,
        value: f64,
    },
    And(Box<QueryPredicate>, Box<QueryPredicate>),
    Or(Box<QueryPredicate>, Box<QueryPredicate>),
    Not(Box<QueryPredicate>),
}

/// Query context describing an arbitrary PCB primitive being evaluated.
#[derive(Debug, Clone, Default)]
pub struct PrimitiveEvaluationContext<'a> {
    pub object_type: Option<QueryTargetType>,
    pub net_name: Option<&'a str>,
    pub layer_name: Option<&'a str>,
    pub reference: Option<&'a str>,
    pub value: Option<&'a str>,
    pub footprint_id: Option<&'a str>,
    pub width_mm: Option<f64>,
    pub drill_mm: Option<f64>,
    pub diameter_mm: Option<f64>,
    pub is_locked: bool,
}

impl QueryPredicate {
    /// Evaluate this predicate against a primitive context.
    pub fn evaluate(&self, ctx: &PrimitiveEvaluationContext) -> bool {
        match self {
            Self::IsType(target) => ctx.object_type.as_ref() == Some(target),
            Self::StringField { field, op, value } => {
                let actual = match field.to_ascii_lowercase().as_str() {
                    "net" | "net_name" => ctx.net_name,
                    "layer" | "layer_name" => ctx.layer_name,
                    "ref" | "refdes" | "reference" => ctx.reference,
                    "val" | "value" => ctx.value,
                    "footprint" | "footprint_id" => ctx.footprint_id,
                    _ => None,
                };

                match (actual, op) {
                    (Some(act), ComparisonOp::Equal) => act.eq_ignore_ascii_case(value),
                    (Some(act), ComparisonOp::NotEqual) => !act.eq_ignore_ascii_case(value),
                    (Some(act), ComparisonOp::Like) => {
                        let pat = value.to_ascii_lowercase();
                        let target = act.to_ascii_lowercase();
                        if pat.ends_with('*') {
                            let prefix = &pat[..pat.len() - 1];
                            target.starts_with(prefix)
                        } else if pat.starts_with('*') {
                            let suffix = &pat[1..];
                            target.ends_with(suffix)
                        } else {
                            target.contains(&pat)
                        }
                    }
                    _ => false,
                }
            }
            Self::NumericField { field, op, value } => {
                let actual = match field.to_ascii_lowercase().as_str() {
                    "width" | "w" => ctx.width_mm,
                    "drill" | "hole" => ctx.drill_mm,
                    "diameter" | "diam" | "pad_size" => ctx.diameter_mm,
                    _ => None,
                };

                match (actual, op) {
                    (Some(act), ComparisonOp::Equal) => (act - value).abs() < 1e-4,
                    (Some(act), ComparisonOp::NotEqual) => (act - value).abs() >= 1e-4,
                    (Some(act), ComparisonOp::GreaterThan) => act > *value,
                    (Some(act), ComparisonOp::GreaterThanOrEqual) => act >= *value - 1e-4,
                    (Some(act), ComparisonOp::LessThan) => act < *value,
                    (Some(act), ComparisonOp::LessThanOrEqual) => act <= *value + 1e-4,
                    _ => false,
                }
            }
            Self::And(left, right) => left.evaluate(ctx) && right.evaluate(ctx),
            Self::Or(left, right) => left.evaluate(ctx) || right.evaluate(ctx),
            Self::Not(inner) => !inner.evaluate(ctx),
        }
    }
}

/// Helper parser for building simple AST expressions from tokens.
pub struct QueryParser;

impl QueryParser {
    /// Convenience helper to create a simple `IsTrack AND Net = '<name>'` query.
    pub fn track_with_net(net: impl Into<String>) -> QueryPredicate {
        QueryPredicate::And(
            Box::new(QueryPredicate::IsType(QueryTargetType::Track)),
            Box::new(QueryPredicate::StringField {
                field: "net".to_string(),
                op: ComparisonOp::Equal,
                value: net.into(),
            }),
        )
    }

    /// Convenience helper to create a wildcard `Net LIKE '<pattern>'` query.
    pub fn net_like(pattern: impl Into<String>) -> QueryPredicate {
        QueryPredicate::StringField {
            field: "net".to_string(),
            op: ComparisonOp::Like,
            value: pattern.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_predicate_evaluation() {
        let ctx = PrimitiveEvaluationContext {
            object_type: Some(QueryTargetType::Track),
            net_name: Some("DDR4_CLK_P"),
            layer_name: Some("Top"),
            width_mm: Some(0.25),
            ..Default::default()
        };

        // Query: IsTrack AND Net LIKE 'DDR4_*' AND Width >= 0.2
        let q = QueryPredicate::And(
            Box::new(QueryPredicate::IsType(QueryTargetType::Track)),
            Box::new(QueryPredicate::And(
                Box::new(QueryParser::net_like("DDR4_*")),
                Box::new(QueryPredicate::NumericField {
                    field: "width".to_string(),
                    op: ComparisonOp::GreaterThanOrEqual,
                    value: 0.20,
                }),
            )),
        );

        assert!(q.evaluate(&ctx));

        // Negative test: Layer = 'Bottom'
        let q_wrong_layer = QueryPredicate::StringField {
            field: "layer".to_string(),
            op: ComparisonOp::Equal,
            value: "Bottom".to_string(),
        };
        assert!(!q_wrong_layer.evaluate(&ctx));
    }
}
