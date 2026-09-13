//! Rule scoping and hierarchical specificity evaluation for constraint matching.

use serde::{Deserialize, Serialize};

/// Hierarchical scope where a design constraint applies.
///
/// Order of evaluation and specificity:
/// `Net` (highest, 4) > `NetClass` (3) > `Room` (2) > `Global` (lowest, 1).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RuleScope {
    /// Applies to every object across the entire PCB project.
    Global,
    /// Applies to objects physically positioned inside a designated geometric room / area.
    Room(String),
    /// Applies to all nets belonging to a specific net class (e.g., "Power", "HighSpeed_Diff").
    NetClass(String),
    /// Applies specifically to an individual named net (e.g., "VBUS", "+3V3", "USB_D+").
    Net(String),
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum TaggedRuleScope {
    Global,
    Room { name: String },
    NetClass { name: String },
    Net { name: String },
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ShorthandRuleScope {
    Room(String),
    NetClass(String),
    Net(String),
    Global,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum RuleScopeDeHelper {
    Tagged(TaggedRuleScope),
    Shorthand(ShorthandRuleScope),
    StringLiteral(String),
}

impl Serialize for RuleScope {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let tagged = match self {
            Self::Global => TaggedRuleScope::Global,
            Self::Room(name) => TaggedRuleScope::Room { name: name.clone() },
            Self::NetClass(name) => TaggedRuleScope::NetClass { name: name.clone() },
            Self::Net(name) => TaggedRuleScope::Net { name: name.clone() },
        };
        tagged.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RuleScope {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let helper = RuleScopeDeHelper::deserialize(deserializer)?;
        match helper {
            RuleScopeDeHelper::Tagged(t) => Ok(match t {
                TaggedRuleScope::Global => Self::Global,
                TaggedRuleScope::Room { name } => Self::Room(name),
                TaggedRuleScope::NetClass { name } => Self::NetClass(name),
                TaggedRuleScope::Net { name } => Self::Net(name),
            }),
            RuleScopeDeHelper::Shorthand(s) => Ok(match s {
                ShorthandRuleScope::Global => Self::Global,
                ShorthandRuleScope::Room(name) => Self::Room(name),
                ShorthandRuleScope::NetClass(name) => Self::NetClass(name),
                ShorthandRuleScope::Net(name) => Self::Net(name),
            }),
            RuleScopeDeHelper::StringLiteral(s) => {
                if s.eq_ignore_ascii_case("global") {
                    Ok(Self::Global)
                } else {
                    Err(serde::de::Error::custom(format!(
                        "Unknown scope string literal '{s}'. Expected 'global' or a scope object like {{ type = \"net\", name = \"...\" }}"
                    )))
                }
            }
        }
    }
}

impl RuleScope {
    /// Specificity priority rank. Higher numbers override lower numbers.
    #[inline]
    pub fn specificity(&self) -> u8 {
        match self {
            Self::Global => 1,
            Self::Room(_) => 2,
            Self::NetClass(_) => 3,
            Self::Net(_) => 4,
        }
    }

    /// Check if this scope matches the query parameters.
    pub fn matches(&self, net: &str, net_class: Option<&str>, room: Option<&str>) -> bool {
        match self {
            Self::Global => true,
            Self::Room(r) => room == Some(r.as_str()),
            Self::NetClass(nc) => net_class == Some(nc.as_str()),
            Self::Net(n) => net == n,
        }
    }
}
