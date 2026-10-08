use crate::datetime;
pub use crate::datetime::{DatePrecision, DateTimePrecision, TimeInterval, TimePrecision};
use crate::decimal::Decimal;
use crate::error::InterpreterError;
use crate::units::UCUM_SYSTEM;
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta};
use parser::TypeSpecifier;
use std::collections::HashMap;
use std::rc::Rc;

/// Result of comparing two `Value`s. Distinguishes the three cases the FHIRPath
/// equality/comparison operators need:
///
/// - `Equal`/`Less`/`Greater` — determinate result with ordering (when types support it)
/// - `Unequal` — definitely not equal, but no meaningful ordering (e.g. cross-type `1` vs `'a'`,
///   Boolean `true` vs `false`). Equality operators derive `false`; ordering operators error.
/// - `Uncomparable` — empty-propagation result (empty operand, incompatible quantity units,
///   mixed date/time precision). All operators derive empty.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Less,
    Greater,
    Unequal,
    Uncomparable,
}

impl Comparison {
    pub fn is_equal(self) -> bool {
        matches!(self, Comparison::Equal)
    }

    pub fn as_ordering(self) -> Option<std::cmp::Ordering> {
        match self {
            Comparison::Equal => Some(std::cmp::Ordering::Equal),
            Comparison::Less => Some(std::cmp::Ordering::Less),
            Comparison::Greater => Some(std::cmp::Ordering::Greater),
            Comparison::Unequal | Comparison::Uncomparable => None,
        }
    }
}

impl From<std::cmp::Ordering> for Comparison {
    fn from(ord: std::cmp::Ordering) -> Self {
        match ord {
            std::cmp::Ordering::Equal => Comparison::Equal,
            std::cmp::Ordering::Less => Comparison::Less,
            std::cmp::Ordering::Greater => Comparison::Greater,
        }
    }
}

impl From<Option<std::cmp::Ordering>> for Comparison {
    fn from(ord: Option<std::cmp::Ordering>) -> Self {
        ord.map_or(Comparison::Uncomparable, Comparison::from)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QuantityType {
    Age,
    Count,
    Distance,
    Duration,
    Money,
    SimpleQuantity,
}

impl QuantityType {
    pub fn from_suffix(suffix: &str) -> Option<Self> {
        match suffix {
            "Age" => Some(Self::Age),
            "Count" => Some(Self::Count),
            "Distance" => Some(Self::Distance),
            "Duration" => Some(Self::Duration),
            "Money" => Some(Self::Money),
            "SimpleQuantity" => Some(Self::SimpleQuantity),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Age => "Age",
            Self::Count => "Count",
            Self::Distance => "Distance",
            Self::Duration => "Duration",
            Self::Money => "Money",
            Self::SimpleQuantity => "SimpleQuantity",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuantityView<'a> {
    pub value: Decimal,
    pub precision: u8,
    pub code: Option<&'a str>,
    pub system: Option<&'a str>,
    pub unit: Option<&'a str>,
    pub quantity_type: Option<QuantityType>,
}

impl<'a> QuantityView<'a> {
    pub fn ucum_code(&self) -> Option<&'a str> {
        self.code.filter(|_| self.system == Some(UCUM_SYSTEM))
    }

    pub fn same_unit_as(&self, other: &QuantityView<'_>) -> bool {
        self.unit.is_some() && self.unit == other.unit
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Boolean(bool),
    String(String),
    Number(Decimal, u8),
    Date(NaiveDate, DatePrecision),
    DateTime(NaiveDateTime, DateTimePrecision, Option<FixedOffset>),
    Time(NaiveTime, TimePrecision),
    Quantity(Decimal, u8, String, Option<QuantityType>),
    Collection(std::mem::ManuallyDrop<Rc<Vec<Value>>>),
    Object(std::mem::ManuallyDrop<Rc<HashMap<String, Value>>>),
}

impl Drop for Value {
    fn drop(&mut self) {
        let mut stack: Vec<Value> = match self {
            Value::Collection(md) => {
                let rc = unsafe { std::mem::ManuallyDrop::take(md) };
                match Rc::try_unwrap(rc) {
                    Ok(items) => items,
                    Err(_) => return,
                }
            }
            Value::Object(md) => {
                let rc = unsafe { std::mem::ManuallyDrop::take(md) };
                match Rc::try_unwrap(rc) {
                    Ok(map) => map.into_values().collect(),
                    Err(_) => return,
                }
            }
            _ => return,
        };
        while let Some(mut val) = stack.pop() {
            match &mut val {
                Value::Collection(md) => {
                    let old_md =
                        std::mem::replace(md, std::mem::ManuallyDrop::new(Rc::new(Vec::new())));
                    let rc = std::mem::ManuallyDrop::into_inner(old_md);
                    if let Ok(items) = Rc::try_unwrap(rc) {
                        stack.extend(items);
                    }
                }
                Value::Object(md) => {
                    let old_md =
                        std::mem::replace(md, std::mem::ManuallyDrop::new(Rc::new(HashMap::new())));
                    let rc = std::mem::ManuallyDrop::into_inner(old_md);
                    if let Ok(map) = Rc::try_unwrap(rc) {
                        stack.extend(map.into_values());
                    }
                }
                _ => {}
            }
        }
    }
}

impl Value {
    pub fn collection(items: Vec<Value>) -> Self {
        Value::Collection(std::mem::ManuallyDrop::new(Rc::new(items)))
    }

    pub fn object(map: HashMap<String, Value>) -> Self {
        Value::Object(std::mem::ManuallyDrop::new(Rc::new(map)))
    }

    pub fn number(value: f64, precision: u8) -> Self {
        Decimal::from_f64(value).map_or_else(
            || Value::collection(vec![]),
            |d| Value::Number(d, precision.min(Decimal::MAX_PRECISION)),
        )
    }

    pub fn from_f64(value: f64) -> Self {
        Value::decimal_or_empty(Decimal::from_f64(value))
    }

    pub fn from_f64_result(value: f64) -> Self {
        Value::decimal_or_empty(Decimal::from_f64_result(value))
    }

    pub fn decimal(value: Decimal) -> Self {
        Value::Number(value, value.precision())
    }

    pub fn decimal_or_empty(value: Option<Decimal>) -> Self {
        value.map_or_else(|| Value::collection(vec![]), Value::decimal)
    }

    pub fn quantity_or_empty(
        value: Option<Decimal>,
        unit: &str,
        quantity_type: Option<QuantityType>,
    ) -> Self {
        value.map_or_else(
            || Value::collection(vec![]),
            |v| Value::Quantity(v, v.precision(), unit.to_string(), quantity_type),
        )
    }

    pub fn quantity(
        value: f64,
        precision: u8,
        unit: String,
        quantity_type: Option<QuantityType>,
    ) -> Self {
        Decimal::from_f64(value).map_or_else(
            || Value::collection(vec![]),
            |d| {
                Value::Quantity(
                    d,
                    precision.min(Decimal::MAX_PRECISION),
                    unit,
                    quantity_type,
                )
            },
        )
    }

    fn discriminant(&self) -> u8 {
        match self {
            Value::Null => 0,
            Value::Boolean(_) => 1,
            Value::Number(..) => 2,
            Value::String(_) => 3,
            Value::Date(..) => 4,
            Value::DateTime(..) => 5,
            Value::Time(..) => 6,
            Value::Quantity(..) => 7,
            Value::Collection(_) => 8,
            Value::Object(_) => 9,
        }
    }

    pub fn unwrap_singleton(&self) -> Value {
        if let Value::Collection(items) = self
            && items.len() == 1
        {
            return items[0].clone();
        }
        self.clone()
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Null => false,
            Value::Boolean(b) => *b,
            Value::Number(n, _) => !n.is_zero(),
            Value::String(s) => !s.is_empty(),
            Value::Date(..) | Value::DateTime(..) | Value::Time(..) => true,
            Value::Quantity(v, ..) => !v.is_zero(),
            Value::Collection(v) => !v.is_empty(),
            Value::Object(o) => !o.is_empty(),
        }
    }

    pub fn is_null_or_empty(&self) -> bool {
        match self {
            Value::Null => true,
            Value::Collection(c) => c.is_empty(),
            _ => false,
        }
    }

    pub fn is_multi_item_collection(&self) -> bool {
        matches!(self, Value::Collection(c) if c.len() > 1)
    }

    #[allow(clippy::match_same_arms)]
    pub fn is(&self, type_spec: &TypeSpecifier) -> bool {
        let TypeSpecifier::QualifiedIdentifier(parts) = type_spec;
        let type_name = parts.last().map(|s| s.as_str());
        let namespace = if parts.len() > 1 {
            parts.first().map(|s| s.as_str())
        } else {
            None
        };
        match (self, type_name) {
            (Value::Null, _) => false,
            (Value::Number(_, p), Some("Integer")) => {
                (namespace.is_none() || namespace == Some("System")) && *p == 0
            }
            (Value::Number(_, p), Some("Decimal")) => {
                (namespace.is_none() || namespace == Some("System")) && *p > 0
            }
            (Value::Boolean(_), Some("Boolean"))
            | (Value::String(_), Some("String"))
            | (Value::Date(..), Some("Date"))
            | (Value::DateTime(..), Some("DateTime"))
            | (Value::Time(..), Some("Time"))
            | (Value::Quantity(..), Some("Quantity")) => {
                namespace.is_none() || namespace == Some("System")
            }
            (Value::Quantity(_, _, _, Some(qt)), Some(expected)) => {
                (namespace.is_none() || namespace == Some("FHIR")) && qt.as_str() == expected
            }
            (Value::Boolean(_), Some("boolean")) => {
                namespace.is_none() || namespace == Some("FHIR")
            }
            (
                Value::String(_),
                Some(
                    "string" | "uri" | "url" | "uuid" | "code" | "id" | "oid" | "markdown"
                    | "base64Binary" | "canonical" | "xhtml",
                ),
            ) => namespace.is_none() || namespace == Some("FHIR"),
            (Value::Number(_, p), Some("integer" | "positiveInt" | "unsignedInt")) => {
                (namespace.is_none() || namespace == Some("FHIR")) && *p == 0
            }
            (Value::Number(..), Some("decimal")) => {
                namespace.is_none() || namespace == Some("FHIR")
            }
            (Value::Date(..), Some("date")) => namespace.is_none() || namespace == Some("FHIR"),
            (Value::DateTime(..), Some("dateTime" | "instant")) => {
                namespace.is_none() || namespace == Some("FHIR")
            }
            (Value::Time(..), Some("time")) => namespace.is_none() || namespace == Some("FHIR"),
            (Value::Object(obj), Some(expected_type)) => {
                if (namespace.is_none() || namespace == Some("FHIR"))
                    && let Some(Value::String(resource_type)) = obj.get("resourceType")
                {
                    return resource_type == expected_type;
                }
                false
            }
            _ => false,
        }
    }

    pub fn type_name(&self) -> Option<&str> {
        match self {
            Value::Null | Value::Collection(_) => None,
            Value::Boolean(_) => Some("Boolean"),
            Value::String(_) => Some("String"),
            Value::Number(_, p) if *p == 0 => Some("Integer"),
            Value::Number(..) => Some("Decimal"),
            Value::Date(..) => Some("Date"),
            Value::DateTime(..) => Some("DateTime"),
            Value::Time(..) => Some("Time"),
            Value::Quantity(_, _, _, t) => Some(t.map_or("Quantity", QuantityType::as_str)),
            Value::Object(obj) => match obj.get("resourceType") {
                Some(Value::String(rt)) => Some(rt),
                _ => None,
            },
        }
    }

    #[allow(clippy::match_same_arms)]
    pub fn as_type(&self, type_spec: &TypeSpecifier) -> Value {
        if self.is(type_spec) {
            return self.clone();
        }

        let TypeSpecifier::QualifiedIdentifier(parts) = type_spec;
        match (self, parts.last().map(|s| s.as_str())) {
            (Value::Null, _) => Value::collection(vec![]),
            (Value::String(s), Some("Integer")) => Decimal::parse(s).map_or_else(
                || Value::collection(vec![]),
                |n| Value::Number(n.trunc(), 0),
            ),
            (Value::String(s), Some("Decimal")) => {
                Decimal::parse(s).map_or_else(|| Value::collection(vec![]), Value::decimal)
            }
            (Value::String(s), Some("Boolean")) => match s.to_lowercase().as_str() {
                "true" | "t" | "yes" | "y" | "1" | "1.0" => Value::Boolean(true),
                "false" | "f" | "no" | "n" | "0" | "0.0" => Value::Boolean(false),
                _ => Value::collection(vec![]),
            },
            (Value::Number(n, _), Some("String")) => Value::String(n.to_string()),
            (Value::Number(n, _), Some("Integer")) => Value::Number(n.trunc(), 0),
            (Value::Number(n, _), Some("Boolean")) => {
                if *n == Decimal::ONE {
                    Value::Boolean(true)
                } else if n.is_zero() {
                    Value::Boolean(false)
                } else {
                    Value::collection(vec![])
                }
            }
            (Value::Boolean(b), Some("String")) => Value::String(b.to_string()),
            (Value::Boolean(b), Some("Integer" | "Decimal")) => {
                Value::Number(if *b { Decimal::ONE } else { Decimal::ZERO }, 0)
            }
            (Value::Date(d, p), Some("String")) => Value::String(datetime::format_date(*d, *p)),
            (Value::DateTime(dt, p, tz), Some("String")) => {
                Value::String(datetime::format_datetime(*dt, *p, tz))
            }
            (Value::Time(t, p), Some("String")) => Value::String(datetime::format_time(*t, *p)),
            (Value::Quantity(v, _, u, _), Some("String")) => {
                Value::String(format!("{} '{}'", v, u))
            }
            (Value::Quantity(v, p, ..), Some("Decimal")) => Value::Number(*v, *p),
            (Value::Quantity(v, ..), Some("Integer")) => Value::Number(v.trunc(), 0),
            _ => Value::collection(vec![]),
        }
    }

    pub fn to_bool(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            Value::Null => None,
            Value::Collection(c) if c.is_empty() => None,
            other => Some(other.is_truthy()),
        }
    }

    pub fn to_f64(&self) -> Option<f64> {
        let mut current = self;
        while let Value::Collection(items) = current {
            if items.len() == 1 {
                current = &items[0];
            } else {
                return None;
            }
        }
        match current {
            Value::Number(n, _) => Some(n.to_f64()),
            Value::String(s) => Decimal::parse(s).map(Decimal::to_f64),
            Value::Boolean(true) => Some(1.0),
            Value::Boolean(false) => Some(0.0),
            _ => None,
        }
    }

    pub fn to_decimal(&self) -> Option<Decimal> {
        let mut current = self;
        while let Value::Collection(items) = current {
            if items.len() == 1 {
                current = &items[0];
            } else {
                return None;
            }
        }
        match current {
            Value::Number(n, _) => Some(*n),
            Value::String(s) => Decimal::parse(s),
            Value::Boolean(true) => Some(Decimal::ONE),
            Value::Boolean(false) => Some(Decimal::ZERO),
            _ => None,
        }
    }

    pub fn to_usize(&self) -> Option<usize> {
        self.to_decimal()
            .and_then(|n| usize::try_from(n.trunc_to_i128()).ok())
    }

    pub fn to_i32(&self) -> Option<i32> {
        self.to_decimal()
            .and_then(|n| i32::try_from(n.trunc_to_i128()).ok())
    }

    pub fn to_str(&self) -> Result<String, InterpreterError> {
        let mut current = self;
        while let Value::Collection(items) = current {
            if items.len() == 1 {
                current = &items[0];
            } else {
                break;
            }
        }
        match current {
            Value::String(s) => Ok(s.clone()),
            Value::Null => Err(InterpreterError::TypeMismatch(
                "Expected string, got null".to_string(),
            )),
            other => Ok(other.to_string()),
        }
    }

    pub fn as_string(&self) -> Option<String> {
        let mut current = self;
        while let Value::Collection(items) = current {
            if items.len() == 1 {
                current = &items[0];
            } else {
                return None;
            }
        }
        match current {
            Value::String(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn quantity_view(&self) -> Option<QuantityView<'_>> {
        match self {
            Value::Quantity(value, precision, unit, quantity_type) => Some(QuantityView {
                value: *value,
                precision: *precision,
                code: Some(unit.as_str()),
                system: Some(UCUM_SYSTEM),
                unit: Some(unit.as_str()),
                quantity_type: *quantity_type,
            }),
            Value::Number(value, precision) => Some(QuantityView {
                value: *value,
                precision: *precision,
                code: Some("1"),
                system: Some(UCUM_SYSTEM),
                unit: Some("1"),
                quantity_type: None,
            }),
            Value::Object(obj) => {
                let (value, precision) = match obj.get("value") {
                    Some(Value::Number(n, p)) => (*n, *p),
                    _ => return None,
                };
                let text = |key: &str| match obj.get(key) {
                    Some(Value::String(s)) => Some(s.as_str()),
                    _ => None,
                };
                let (code, unit) = (text("code"), text("unit"));
                if code.is_none() && unit.is_none() {
                    return None;
                }
                Some(QuantityView {
                    value,
                    precision,
                    code,
                    system: text("system"),
                    unit,
                    quantity_type: text("resourceType").and_then(QuantityType::from_suffix),
                })
            }
            _ => None,
        }
    }

    pub fn to_vec(&self) -> Vec<Value> {
        match self {
            Value::Collection(items) => (***items).clone(),
            Value::Null => vec![],
            other => vec![other.clone()],
        }
    }

    pub fn to_time_interval(&self) -> Option<TimeInterval> {
        let mut current = self;
        while let Value::Collection(items) = current {
            if items.len() == 1 {
                current = &items[0];
            } else {
                return None;
            }
        }
        let Value::Quantity(value, _, unit, _) = current else {
            return None;
        };
        let whole = i64::try_from(value.trunc_to_i128()).ok();
        match unit.to_lowercase().as_str() {
            "year" | "years" => value
                .checked_mul(Decimal::from(12))
                .and_then(|months| i32::try_from(months.trunc_to_i128()).ok())
                .map(TimeInterval::Months),
            "month" | "months" => i32::try_from(value.trunc_to_i128())
                .ok()
                .map(TimeInterval::Months),
            "week" | "weeks" | "wk" => whole
                .and_then(TimeDelta::try_weeks)
                .map(TimeInterval::Duration),
            "day" | "days" | "d" => whole
                .and_then(TimeDelta::try_days)
                .map(TimeInterval::Duration),
            "hour" | "hours" | "h" => whole
                .and_then(TimeDelta::try_hours)
                .map(TimeInterval::Duration),
            "minute" | "minutes" | "min" => whole
                .and_then(TimeDelta::try_minutes)
                .map(TimeInterval::Duration),
            "second" | "seconds" | "s" => value
                .checked_mul(Decimal::from(1000))
                .and_then(|millis| millis.round_dp(0))
                .and_then(|millis| i64::try_from(millis.trunc_to_i128()).ok())
                .and_then(TimeDelta::try_milliseconds)
                .map(TimeInterval::Duration),
            "millisecond" | "milliseconds" | "ms" => whole
                .and_then(TimeDelta::try_milliseconds)
                .map(TimeInterval::Duration),
            _ => None,
        }
    }

    pub fn from_date_str(s: &str) -> Option<Value> {
        if let Ok(d) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            return Some(Value::Date(d, DatePrecision::Day));
        }
        if let Ok(d) = NaiveDate::parse_from_str(&format!("{}-01", s), "%Y-%m-%d") {
            return Some(Value::Date(d, DatePrecision::Month));
        }
        if let Ok(d) = NaiveDate::parse_from_str(&format!("{}-01-01", s), "%Y-%m-%d") {
            return Some(Value::Date(d, DatePrecision::Year));
        }
        None
    }

    pub fn from_datetime_str(s: &str) -> Option<Value> {
        let precision = datetime::detect_datetime_precision(s);

        for fmt in &[
            "%Y-%m-%dT%H:%M:%S%:z",
            "%Y-%m-%dT%H:%M:%S%.f%:z",
            "%Y-%m-%dT%H:%M%:z",
        ] {
            if let Ok(dt) = DateTime::parse_from_str(s, fmt) {
                return Some(Value::DateTime(
                    dt.naive_local(),
                    precision,
                    Some(*dt.offset()),
                ));
            }
        }

        let (base_str, tz) = if s.ends_with('Z') {
            (s.trim_end_matches('Z'), FixedOffset::east_opt(0))
        } else {
            (s, None)
        };

        let (date_str, time_str) = match base_str.split_once('T') {
            Some((d, t)) => (d, t),
            None => (base_str, ""),
        };

        let date_parts: Vec<&str> = date_str.split('-').collect();
        let padded_date = match date_parts.len() {
            1 => format!("{}-01-01", date_parts[0]),
            2 => format!("{}-{}-01", date_parts[0], date_parts[1]),
            _ => date_str.to_string(),
        };

        let (padded_time, fmt) = if time_str.is_empty() {
            ("00:00:00".to_string(), "%Y-%m-%dT%H:%M:%S")
        } else if time_str.contains('.') {
            let time_parts: Vec<&str> = time_str.splitn(2, '.').collect();
            let base_parts: Vec<&str> = time_parts[0].split(':').collect();
            let padded_base = match base_parts.len() {
                1 => format!("{}:00:00", base_parts[0]),
                2 => format!("{}:{}:00", base_parts[0], base_parts[1]),
                _ => base_parts.join(":"),
            };
            (
                format!("{}.{}", padded_base, time_parts[1]),
                "%Y-%m-%dT%H:%M:%S%.f",
            )
        } else {
            let time_parts: Vec<&str> = time_str.split(':').collect();
            let padded = match time_parts.len() {
                1 => format!("{}:00:00", time_parts[0]),
                2 => format!("{}:{}:00", time_parts[0], time_parts[1]),
                _ => time_parts.join(":"),
            };
            (padded, "%Y-%m-%dT%H:%M:%S")
        };

        NaiveDateTime::parse_from_str(&format!("{}T{}", padded_date, padded_time), fmt)
            .ok()
            .map(|dt| Value::DateTime(dt, precision, tz))
    }

    pub fn from_time_str(s: &str) -> Option<Value> {
        let precision = datetime::detect_time_precision(s);

        let (padded, fmt) = if s.contains('.') {
            let parts: Vec<&str> = s.splitn(2, '.').collect();
            let base_parts: Vec<&str> = parts[0].split(':').collect();
            let padded_base = match base_parts.len() {
                1 => format!("{}:00:00", base_parts[0]),
                2 => format!("{}:{}:00", base_parts[0], base_parts[1]),
                _ => base_parts.join(":"),
            };
            (format!("{}.{}", padded_base, parts[1]), "%H:%M:%S%.f")
        } else {
            let parts: Vec<&str> = s.split(':').collect();
            let padded = match parts.len() {
                1 => format!("{}:00:00", parts[0]),
                2 => format!("{}:{}:00", parts[0], parts[1]),
                _ => parts.join(":"),
            };
            (padded, "%H:%M:%S")
        };

        NaiveTime::parse_from_str(&padded, fmt)
            .ok()
            .map(|t| Value::Time(t, precision))
    }

    pub fn compare_equivalent(&self, other: &Value) -> Comparison {
        use std::cmp::Ordering;
        let mut stack: Vec<(&Value, &Value)> = vec![(self, other)];
        let mut result: Option<Comparison> = None;
        while let Some((a, b)) = stack.pop() {
            let pair: Comparison = match (a, b) {
                (Value::Null, Value::Null) => Comparison::Equal,
                (Value::Boolean(ba), Value::Boolean(bb)) => ba.cmp(bb).into(),
                (Value::Number(na, pa), Value::Number(nb, pb)) => {
                    let places = i32::from((*pa).min(*pb));
                    match (na.round_dp(places), nb.round_dp(places)) {
                        (Some(ra), Some(rb)) => ra.cmp(&rb).into(),
                        _ => Comparison::Uncomparable,
                    }
                }
                (Value::String(sa), Value::String(sb)) => {
                    sa.to_lowercase().cmp(&sb.to_lowercase()).into()
                }
                (Value::Date(da, _), Value::Date(db, _)) => da.cmp(db).into(),
                (Value::DateTime(da, _, tza), Value::DateTime(db, _, tzb)) => {
                    let utc_a = tza.map_or(*da, |o| datetime::to_utc_naive(*da, &o));
                    let utc_b = tzb.map_or(*db, |o| datetime::to_utc_naive(*db, &o));
                    utc_a.cmp(&utc_b).into()
                }
                (Value::Time(ta, _), Value::Time(tb, _)) => ta.cmp(tb).into(),
                _ if a.quantity_view().is_some() && b.quantity_view().is_some() => {
                    if crate::units::quantity_equivalent(a, b) {
                        Comparison::Equal
                    } else {
                        Comparison::Unequal
                    }
                }
                (Value::Collection(ca), Value::Collection(cb)) => {
                    let mut flat_a: Vec<&Value> = Vec::new();
                    let mut flat_b: Vec<&Value> = Vec::new();
                    let mut expand: Vec<&Value> = ca.iter().collect();
                    while let Some(val) = expand.pop() {
                        if let Value::Collection(inner) = val {
                            expand.extend(inner.iter());
                        } else {
                            flat_a.push(val);
                        }
                    }
                    expand.extend(cb.iter());
                    while let Some(val) = expand.pop() {
                        if let Value::Collection(inner) = val {
                            expand.extend(inner.iter());
                        } else {
                            flat_b.push(val);
                        }
                    }
                    if flat_a.len() != flat_b.len() {
                        Comparison::Unequal
                    } else {
                        let sort_key = |x: &&Value, y: &&Value| {
                            x.compare_equivalent(y)
                                .as_ordering()
                                .unwrap_or_else(|| x.discriminant().cmp(&y.discriminant()))
                        };
                        crate::sort::sort_by(&mut flat_a, sort_key);
                        crate::sort::sort_by(&mut flat_b, sort_key);
                        for (x, y) in flat_a.into_iter().zip(flat_b) {
                            stack.push((x, y));
                        }
                        Comparison::Equal
                    }
                }
                (Value::Object(oa), Value::Object(ob)) => match oa.len().cmp(&ob.len()) {
                    Ordering::Equal => {
                        let mut ka: Vec<&String> = oa.keys().collect();
                        let mut kb: Vec<&String> = ob.keys().collect();
                        ka.sort();
                        kb.sort();
                        match ka.cmp(&kb) {
                            Ordering::Equal => {
                                for key in ka.into_iter().rev() {
                                    stack.push((&oa[key], &ob[key]));
                                }
                                Comparison::Equal
                            }
                            ord => ord.into(),
                        }
                    }
                    ord => ord.into(),
                },
                _ => Comparison::Unequal,
            };
            match pair {
                Comparison::Equal | Comparison::Uncomparable => {}
                definite => {
                    if result.is_none() {
                        result = Some(definite);
                    }
                }
            }
        }
        result.unwrap_or(Comparison::Equal)
    }

    pub fn compare_equal(&self, other: &Value) -> Comparison {
        use std::cmp::Ordering;
        let mut left = self;
        while let Value::Collection(items) = left {
            if items.len() == 1 {
                left = &items[0];
            } else {
                break;
            }
        }
        let mut right = other;
        while let Value::Collection(items) = right {
            if items.len() == 1 {
                right = &items[0];
            } else {
                break;
            }
        }
        let mut stack: Vec<(&Value, &Value)> = vec![(left, right)];
        let mut uncomparable = false;
        let mut result: Option<Comparison> = None;
        while let Some((a, b)) = stack.pop() {
            let pair: Comparison = match (a, b) {
                _ if a.quantity_view().is_some() && b.quantity_view().is_some() => {
                    crate::units::quantity_cmp(a, b)
                }
                (Value::Null, Value::Null) => Comparison::Equal,
                (Value::Boolean(ba), Value::Boolean(bb)) => {
                    if ba == bb {
                        Comparison::Equal
                    } else {
                        Comparison::Unequal
                    }
                }
                (Value::String(sa), Value::String(sb)) => sa.cmp(sb).into(),
                (Value::Date(da, pa), Value::Date(db, pb)) => {
                    if pa == pb {
                        da.cmp(db).into()
                    } else {
                        Comparison::Uncomparable
                    }
                }
                (Value::DateTime(da, pa, tza), Value::DateTime(db, pb, tzb)) => {
                    if pa.comparable_to(*pb) {
                        match (tza, tzb) {
                            (Some(oa), Some(ob)) => datetime::to_utc_naive(*da, oa)
                                .cmp(&datetime::to_utc_naive(*db, ob))
                                .into(),
                            (None, None) => da.cmp(db).into(),
                            _ => Comparison::Uncomparable,
                        }
                    } else {
                        Comparison::Uncomparable
                    }
                }
                (Value::Time(ta, pa), Value::Time(tb, pb)) => {
                    if pa.comparable_to(*pb) {
                        ta.cmp(tb).into()
                    } else {
                        Comparison::Uncomparable
                    }
                }
                (Value::DateTime(dt, _, _), Value::Date(d, _)) => match dt.date().cmp(d) {
                    Ordering::Equal => Comparison::Uncomparable,
                    ord => ord.into(),
                },
                (Value::Date(d, _), Value::DateTime(dt, _, _)) => match d.cmp(&dt.date()) {
                    Ordering::Equal => Comparison::Uncomparable,
                    ord => ord.into(),
                },
                (Value::Collection(ca), Value::Collection(cb)) => {
                    if ca.len() != cb.len() {
                        Comparison::Unequal
                    } else {
                        for (x, y) in ca.iter().zip(cb.iter()) {
                            stack.push((x, y));
                        }
                        Comparison::Equal
                    }
                }
                (Value::Object(oa), Value::Object(ob)) => {
                    if oa.len() != ob.len() {
                        Comparison::Unequal
                    } else {
                        let mut matched = true;
                        for (k, v) in oa.iter() {
                            match ob.get(k) {
                                Some(bv) => stack.push((v, bv)),
                                None => {
                                    matched = false;
                                    break;
                                }
                            }
                        }
                        if matched {
                            Comparison::Equal
                        } else {
                            Comparison::Unequal
                        }
                    }
                }
                _ => Comparison::Unequal,
            };
            match pair {
                Comparison::Equal => {}
                Comparison::Uncomparable => uncomparable = true,
                definite => {
                    if result.is_none() {
                        result = Some(definite);
                    }
                }
            }
        }
        if let Some(r) = result {
            r
        } else if uncomparable {
            Comparison::Uncomparable
        } else {
            Comparison::Equal
        }
    }

    pub fn order(
        &self,
        other: &Value,
        descending: bool,
    ) -> Result<std::cmp::Ordering, InterpreterError> {
        let ord = match (self.is_null_or_empty(), other.is_null_or_empty()) {
            (true, true) => std::cmp::Ordering::Equal,
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            (false, false) => self.compare_equal(other).as_ordering().ok_or_else(|| {
                InterpreterError::InvalidOperation(
                    "sort() cannot order values of incompatible types".to_string(),
                )
            })?,
        };
        Ok(if descending { ord.reverse() } else { ord })
    }

    pub fn compare_precision(&self, other: &Value) -> Option<std::cmp::Ordering> {
        let mut left = self;
        while let Value::Collection(items) = left {
            if items.len() == 1 {
                left = &items[0];
            } else {
                return None;
            }
        }
        let mut right = other;
        while let Value::Collection(items) = right {
            if items.len() == 1 {
                right = &items[0];
            } else {
                return None;
            }
        }
        match (left, right) {
            (Value::Date(_, pa), Value::Date(_, pb)) => Some(pa.cmp(pb)),
            (Value::DateTime(_, pa, _), Value::DateTime(_, pb, _)) => {
                if pa.comparable_to(*pb) {
                    Some(std::cmp::Ordering::Equal)
                } else {
                    Some(pa.cmp(pb))
                }
            }
            (Value::Time(_, pa), Value::Time(_, pb)) => {
                if pa.comparable_to(*pb) {
                    Some(std::cmp::Ordering::Equal)
                } else {
                    Some(pa.cmp(pb))
                }
            }
            (Value::Date(..), Value::DateTime(..)) | (Value::DateTime(..), Value::Date(..)) => {
                Some(std::cmp::Ordering::Less)
            }
            _ => None,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "{{}}"),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::String(s) => write!(f, "{}", s),
            Value::Number(n, p) => f.write_str(&n.format(*p)),
            Value::Date(d, p) => write!(f, "@{}", datetime::format_date(*d, *p)),
            Value::DateTime(dt, p, tz) => {
                write!(f, "@{}", datetime::format_datetime(*dt, *p, tz))
            }
            Value::Time(t, p) => write!(f, "@T{}", datetime::format_time(*t, *p)),
            Value::Quantity(v, p, u, _) => {
                if crate::units::is_calendar_unit(u) {
                    write!(f, "{} {}", v.format(*p), u)
                } else {
                    write!(f, "{} '{}'", v.format(*p), u)
                }
            }
            Value::Collection(items) => {
                write!(
                    f,
                    "[{}]",
                    items
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Value::Object(_) => write!(f, "{{object}}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_string_value_returns_string() {
        assert_eq!(
            Value::String("hello".into()).as_string(),
            Some("hello".into())
        );
    }

    #[test]
    fn as_string_value_unwraps_singleton() {
        let val = Value::collection(vec![Value::String("x".into())]);
        assert_eq!(val.as_string(), Some("x".into()));
    }

    #[test]
    fn as_string_value_returns_none_for_non_string() {
        assert_eq!(Value::number(42.0, 0).as_string(), None);
        assert_eq!(Value::Boolean(true).as_string(), None);
        assert_eq!(Value::Null.as_string(), None);
    }

    #[test]
    fn as_string_value_returns_none_for_multi_item() {
        let val = Value::collection(vec![Value::String("a".into()), Value::String("b".into())]);
        assert_eq!(val.as_string(), None);
    }

    #[test]
    fn is_multi_item_collection_true() {
        let val = Value::collection(vec![Value::number(1.0, 0), Value::number(2.0, 0)]);
        assert!(val.is_multi_item_collection());
    }

    #[test]
    fn is_multi_item_collection_false_for_singleton() {
        let val = Value::collection(vec![Value::number(1.0, 0)]);
        assert!(!val.is_multi_item_collection());
    }

    #[test]
    fn is_multi_item_collection_false_for_non_collection() {
        assert!(!Value::number(1.0, 0).is_multi_item_collection());
        assert!(!Value::Null.is_multi_item_collection());
    }

    #[test]
    fn compare_for_sort_orders_values() {
        let one = Value::number(1.0, 0);
        let two = Value::number(2.0, 0);
        assert_eq!(one.order(&two, false).ok(), Some(std::cmp::Ordering::Less));
        assert_eq!(
            one.order(&two, true).ok(),
            Some(std::cmp::Ordering::Greater)
        );
    }

    #[test]
    fn compare_for_sort_empty_is_lowest_and_reverses() {
        let empty = Value::collection(vec![]);
        let one = Value::number(1.0, 0);
        assert_eq!(
            empty.order(&one, false).ok(),
            Some(std::cmp::Ordering::Less)
        );
        assert_eq!(
            empty.order(&one, true).ok(),
            Some(std::cmp::Ordering::Greater)
        );
        assert_eq!(
            empty.order(&Value::Null, false).ok(),
            Some(std::cmp::Ordering::Equal)
        );
    }

    #[test]
    fn compare_for_sort_incompatible_types_error() {
        let one = Value::number(1.0, 0);
        assert!(one.order(&Value::Boolean(true), false).is_err());
        assert!(
            Value::Boolean(true)
                .order(&Value::Boolean(false), false)
                .is_err()
        );
    }

    #[test]
    fn order_number_and_dimensionless_quantity() {
        let two = Value::number(2.0, 0);
        let one = Value::quantity(1.0, 0, "1".to_string(), None);
        assert_eq!(
            two.order(&one, false).ok(),
            Some(std::cmp::Ordering::Greater)
        );
        assert_eq!(one.order(&two, false).ok(), Some(std::cmp::Ordering::Less));
        assert!(
            two.order(&Value::quantity(1.0, 0, "mg".to_string(), None), false)
                .is_err()
        );
        assert!(
            two.order(
                &Value::quantity(2.0, 0, "1".to_string(), Some(QuantityType::Count)),
                false
            )
            .is_err()
        );
    }

    #[test]
    fn constructors_keep_declared_precision_within_max() {
        assert!(matches!(Value::number(1.5, 1), Value::Number(d, 1) if d.to_string() == "1.5"));
        assert_eq!(Value::number(1.5, 20), Value::number(1.5, 8));
        assert_eq!(Value::from_f64(0.1 + 0.2), Value::number(0.3, 1));
        assert_eq!(Value::from_f64(1.0 / 3.0), Value::number(0.33333333, 8));
        assert_eq!(Value::from_f64(f64::NAN), Value::collection(vec![]));
        assert_eq!(Value::number(1e301, 0), Value::collection(vec![]));
        assert_eq!(Value::decimal_or_empty(None), Value::collection(vec![]));
        assert_eq!(
            Value::quantity_or_empty(Decimal::parse("2.50"), "mg", None),
            Value::quantity(2.5, 1, "mg".to_string(), None)
        );
        assert_eq!(
            Value::quantity_or_empty(None, "mg", None),
            Value::collection(vec![])
        );
    }

    #[test]
    fn compare_equal_is_exact() {
        assert!(
            Value::number(1.0, 0)
                .compare_equal(&Value::number(1.0, 8))
                .is_equal()
        );
        assert!(
            !Value::number(1.00000001, 8)
                .compare_equal(&Value::number(1.0, 0))
                .is_equal()
        );
        assert_eq!(
            Decimal::parse("1000000000.00000001")
                .zip(Decimal::parse("1000000000.00000002"))
                .map(|(a, b)| Value::Number(a, 8).compare_equal(&Value::Number(b, 8))),
            Some(Comparison::Less)
        );
    }

    #[test]
    fn quantity_view_reads_fhir_quantity_objects() {
        let obj = Value::object(HashMap::from([
            ("value".to_string(), Value::number(1.5, 1)),
            ("code".to_string(), Value::String("mg".to_string())),
            ("unit".to_string(), Value::String("milligram".to_string())),
            ("system".to_string(), Value::String(UCUM_SYSTEM.to_string())),
            ("resourceType".to_string(), Value::String("Age".to_string())),
        ]));
        assert_eq!(
            obj.quantity_view().map(|v| (
                v.value.to_string(),
                v.precision,
                v.code,
                v.unit,
                v.ucum_code(),
                v.quantity_type
            )),
            Some((
                "1.5".to_string(),
                1,
                Some("mg"),
                Some("milligram"),
                Some("mg"),
                Some(QuantityType::Age)
            ))
        );

        assert_eq!(
            Value::number(2.0, 0)
                .quantity_view()
                .map(|v| (v.code, v.ucum_code())),
            Some((Some("1"), Some("1")))
        );
        assert_eq!(Value::String("mg".to_string()).quantity_view(), None);

        let no_unit = Value::object(HashMap::from([(
            "value".to_string(),
            Value::number(1.0, 0),
        )]));
        assert_eq!(no_unit.quantity_view(), None);

        let unit_only = Value::object(HashMap::from([
            ("value".to_string(), Value::number(1.0, 0)),
            ("unit".to_string(), Value::String("mg".to_string())),
        ]));
        assert_eq!(
            unit_only.quantity_view().map(|v| (v.unit, v.ucum_code())),
            Some((Some("mg"), None))
        );
        let literal = Value::quantity(1.0, 0, "mg".to_string(), None);
        assert_eq!(
            unit_only
                .quantity_view()
                .zip(literal.quantity_view())
                .map(|(a, b)| a.same_unit_as(&b)),
            Some(true)
        );

        let foreign = Value::object(HashMap::from([
            ("value".to_string(), Value::number(1.0, 0)),
            ("code".to_string(), Value::String("kg".to_string())),
            (
                "system".to_string(),
                Value::String("http://example.org/not-ucum".to_string()),
            ),
        ]));
        assert_eq!(foreign.quantity_view().map(|v| v.ucum_code()), Some(None));
    }

    #[test]
    fn compare_equal_coerces_fhir_quantity_objects() {
        let obj = Value::object(HashMap::from([
            ("value".to_string(), Value::number(1.0, 0)),
            ("code".to_string(), Value::String("1".to_string())),
            ("system".to_string(), Value::String(UCUM_SYSTEM.to_string())),
        ]));
        assert!(obj.compare_equal(&Value::number(1.0, 0)).is_equal());
        assert_eq!(
            Value::number(2.0, 0).compare_equal(&obj),
            Comparison::Greater
        );
        assert!(obj.compare_equivalent(&Value::number(1.0, 0)).is_equal());
        let mg = Value::object(HashMap::from([
            ("value".to_string(), Value::number(1000.0, 0)),
            ("code".to_string(), Value::String("mg".to_string())),
            ("system".to_string(), Value::String(UCUM_SYSTEM.to_string())),
        ]));
        assert!(
            mg.compare_equal(&Value::quantity(1.0, 0, "g".to_string(), None))
                .is_equal()
        );
        assert_eq!(obj.compare_equal(&mg), Comparison::Uncomparable);
        let unit_only = Value::object(HashMap::from([
            ("value".to_string(), Value::number(1.0, 0)),
            ("unit".to_string(), Value::String("mg".to_string())),
        ]));
        assert!(
            unit_only
                .compare_equal(&Value::quantity(1.0, 0, "mg".to_string(), None))
                .is_equal()
        );
    }
}
