use serde_json::Value;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Pattern(pub Vec<String>);

impl FromStr for Pattern {
    type Err = clap::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Pattern(
            s.split("/").map(|i| i.to_string()).collect::<Vec<_>>(),
        ))
    }
}

#[derive(Debug, Clone)]
/// Enumerator of possible filter values.
enum ValueFilter {
    Int(u64),

    String(String),

    Float(f64),
}

#[derive(Debug, Clone)]
pub struct Filter {
    /// To reach the filter value.
    pattern: Pattern,

    /// Value used to match and filter out entries.
    value: ValueFilter,
}

impl FromStr for Filter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entries = s.split("/").map(|i| i.to_string()).collect::<Vec<_>>();

        // Convert value to [`ValueFilter`].
        // TODO: future might include some annotation to easier the transformation.
        // Currently assumes that this is first an int, then a float, then a string.
        let last = entries
            .last()
            .ok_or("Entries of 0 length".to_string())?
            .to_owned();

        let value = if let Ok(v) = u64::from_str(&last) {
            ValueFilter::Int(v)
        } else if let Ok(v) = f64::from_str(&last) {
            ValueFilter::Float(v)
        } else {
            ValueFilter::String(last)
        };

        Ok(Self {
            pattern: Pattern(entries[..entries.len() - 1].to_vec()),
            value,
        })
    }
}

impl Filter {
    /// Returns whether the [`serde_json::Value`] equals the filter.
    pub fn filter(&self, value: &Value) -> bool {
        self.pattern
            .0
            .iter()
            .try_fold(value, |v, p| convert_value_and_get(v, p))
            .map(|v| self.cmp_value(v))
            .unwrap_or(false)
    }

    /// Converts the [`serde_json::Value`] into the inner representation of [`Filter::value`] and compare them.
    fn cmp_value(&self, value: &Value) -> bool {
        match &self.value {
            ValueFilter::Int(v) => value.as_u64().map(|u| u == *v),
            ValueFilter::Float(v) => value.as_f64().map(|f| f == *v),
            ValueFilter::String(v) => value.as_str().map(|f| f == v),
        }
        .unwrap_or(false)
    }
}

/// Currently only returns the first matching occurence in the tree structure.
pub fn convert_value_and_get<'a>(v: &'a Value, p: &str) -> Option<&'a Value> {
    if let Some(m) = v.as_object() {
        m.get(p)
    } else if let Some(a) = v.as_array() {
        a.iter().filter_map(|i| convert_value_and_get(i, p)).next()
    } else {
        None
    }
}
