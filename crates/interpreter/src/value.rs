use crate::datetime;
pub use crate::datetime::{DatePrecision, DateTimePrecision, TimeInterval, TimePrecision};
use crate::error::InterpreterError;
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta};
use parser::TypeSpecifier;
use std::collections::HashMap;
use std::rc::Rc;

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

pub const MAX_DECIMAL_PRECISION: u8 = 8;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Boolean(bool),
    String(String),
    Number(f64, u8),
    Date(NaiveDate, DatePrecision),
    DateTime(NaiveDateTime, DateTimePrecision, Option<FixedOffset>),
    Time(NaiveTime, TimePrecision),
    Quantity(f64, u8, String, Option<QuantityType>),
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

    #[allow(clippy::cast_possible_truncation, clippy::manual_range_contains)]
    pub fn precision(v: f64) -> u8 {
        let mut n = v.abs();
        let mut p: u8 = 0;
        while p < MAX_DECIMAL_PRECISION {
            let frac = n.fract();
            if frac < 1e-10 || frac > 1.0 - 1e-10 {
                break;
            }
            n *= 10.0;
            p += 1;
        }
        p
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
            Value::Number(n, _) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Date(..) | Value::DateTime(..) | Value::Time(..) => true,
            Value::Quantity(v, ..) => *v != 0.0,
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
            (Value::Boolean(_), Some("Boolean"))
            | (Value::String(_), Some("String"))
            | (Value::Number(..), Some("Decimal"))
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
            (Value::String(s), Some("Integer")) => s.trim().parse::<f64>().ok().map_or_else(
                || Value::collection(vec![]),
                |n| Value::Number(n.trunc(), 0),
            ),
            (Value::String(s), Some("Decimal")) => s
                .trim()
                .parse::<f64>()
                .ok()
                .map_or_else(|| Value::collection(vec![]), |n| Value::Number(n, 0)),
            (Value::String(s), Some("Boolean")) => match s.to_lowercase().as_str() {
                "true" | "t" | "yes" | "y" | "1" | "1.0" => Value::Boolean(true),
                "false" | "f" | "no" | "n" | "0" | "0.0" => Value::Boolean(false),
                _ => Value::collection(vec![]),
            },
            (Value::Number(n, _), Some("String")) => Value::String(n.to_string()),
            (Value::Number(n, _), Some("Integer")) => Value::Number(n.trunc(), 0),
            (Value::Number(n, _), Some("Boolean")) => {
                if *n == 1.0 {
                    Value::Boolean(true)
                } else if *n == 0.0 {
                    Value::Boolean(false)
                } else {
                    Value::collection(vec![])
                }
            }
            (Value::Boolean(b), Some("String")) => Value::String(b.to_string()),
            (Value::Boolean(b), Some("Integer" | "Decimal")) => {
                Value::Number(if *b { 1.0 } else { 0.0 }, 0)
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
            Value::Number(n, _) => Some(*n),
            Value::String(s) => s.parse().ok(),
            Value::Boolean(true) => Some(1.0),
            Value::Boolean(false) => Some(0.0),
            _ => None,
        }
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn to_usize(&self) -> Option<usize> {
        self.to_f64().map(|n| n.trunc() as usize)
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn to_i32(&self) -> Option<i32> {
        self.to_f64().map(|n| n.trunc() as i32)
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

    pub fn as_quantity(&self) -> Option<Value> {
        let mut val = self;
        loop {
            match val {
                Value::Quantity(..) => return Some(val.clone()),
                Value::Collection(items) if items.len() == 1 => val = &items[0],
                Value::Object(obj) => {
                    let v = match obj.get("value") {
                        Some(Value::Number(n, _)) => *n,
                        _ => return None,
                    };
                    let code = match obj.get("code") {
                        Some(Value::String(s)) => s.clone(),
                        _ => return None,
                    };
                    let qt = obj.get("resourceType").and_then(|rt| match rt {
                        Value::String(s) => QuantityType::from_suffix(s),
                        _ => None,
                    });
                    return Some(Value::Quantity(v, 0, code, qt));
                }
                _ => return None,
            }
        }
    }

    pub fn to_vec(&self) -> Vec<Value> {
        match self {
            Value::Collection(items) => (***items).clone(),
            Value::Null => vec![],
            other => vec![other.clone()],
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn to_time_interval(&self) -> Option<TimeInterval> {
        let mut current = self;
        while let Value::Collection(items) = current {
            if items.len() == 1 {
                current = &items[0];
            } else {
                return None;
            }
        }
        match current {
            Value::Quantity(value, _, unit, _) => {
                let unit_lower = unit.to_lowercase();
                match unit_lower.as_str() {
                    "year" | "years" => i32::try_from((*value * 12.0).trunc() as i64)
                        .ok()
                        .map(TimeInterval::Months),
                    "month" | "months" => i32::try_from(value.trunc() as i64)
                        .ok()
                        .map(TimeInterval::Months),
                    "week" | "weeks" | "wk" => {
                        TimeDelta::try_weeks(value.trunc() as i64).map(TimeInterval::Duration)
                    }
                    "day" | "days" | "d" => {
                        TimeDelta::try_days(value.trunc() as i64).map(TimeInterval::Duration)
                    }
                    "hour" | "hours" | "h" => {
                        TimeDelta::try_hours(value.trunc() as i64).map(TimeInterval::Duration)
                    }
                    "minute" | "minutes" | "min" => {
                        TimeDelta::try_minutes(value.trunc() as i64).map(TimeInterval::Duration)
                    }
                    "second" | "seconds" | "s" =>
                    {
                        #[allow(clippy::cast_possible_truncation)]
                        TimeDelta::try_milliseconds((*value * 1000.0).round() as i64)
                            .map(TimeInterval::Duration)
                    }
                    "millisecond" | "milliseconds" | "ms" => {
                        TimeDelta::try_milliseconds(value.trunc() as i64)
                            .map(TimeInterval::Duration)
                    }
                    _ => None,
                }
            }
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

    pub fn equals(&self, other: &Value) -> bool {
        let mut stack: Vec<(&Value, &Value)> = vec![(self, other)];
        while let Some((a, b)) = stack.pop() {
            if !match (a, b) {
                (Value::Null, Value::Null) => true,
                (Value::Boolean(ba), Value::Boolean(bb)) => ba == bb,
                (Value::String(sa), Value::String(sb)) => sa == sb,
                (Value::Number(na, _), Value::Number(nb, _)) => (na - nb).abs() < f64::EPSILON,
                (Value::Date(da, pa), Value::Date(db, pb)) => pa == pb && da == db,
                (Value::DateTime(da, pa, tza), Value::DateTime(db, pb, tzb)) => {
                    pa.comparable_to(*pb)
                        && match (tza, tzb) {
                            (Some(oa), Some(ob)) => {
                                datetime::to_utc_naive(*da, oa) == datetime::to_utc_naive(*db, ob)
                            }
                            (None, None) => da == db,
                            _ => false,
                        }
                }
                (Value::Time(ta, pa), Value::Time(tb, pb)) => pa.comparable_to(*pb) && ta == tb,
                (Value::Quantity(v1, _, u1, qt1), Value::Quantity(v2, _, u2, qt2)) => {
                    (v1 - v2).abs() < f64::EPSILON && u1 == u2 && qt1 == qt2
                }
                (Value::Collection(ca), Value::Collection(cb)) => {
                    if ca.len() != cb.len() {
                        return false;
                    }
                    for (x, y) in ca.iter().zip(cb.iter()) {
                        stack.push((x, y));
                    }
                    continue;
                }
                (Value::Object(oa), Value::Object(ob)) => {
                    if oa.len() != ob.len() {
                        return false;
                    }
                    for (k, v) in oa.iter() {
                        match ob.get(k) {
                            Some(bv) => stack.push((v, bv)),
                            None => return false,
                        }
                    }
                    continue;
                }
                _ => false,
            } {
                return false;
            }
        }
        true
    }

    pub fn equivalent(&self, other: &Value) -> bool {
        let mut stack: Vec<(&Value, &Value)> = vec![(self, other)];
        while let Some((a, b)) = stack.pop() {
            if !match (a, b) {
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
                        return false;
                    }
                    let sort_key = |x: &&Value, y: &&Value| {
                        x.compare_equivalent(y)
                            .unwrap_or_else(|| x.discriminant().cmp(&y.discriminant()))
                    };
                    flat_a.sort_by(sort_key);
                    flat_b.sort_by(sort_key);
                    for (x, y) in flat_a.into_iter().zip(flat_b) {
                        stack.push((x, y));
                    }
                    continue;
                }
                (Value::Object(oa), Value::Object(ob)) => {
                    if oa.len() != ob.len() {
                        return false;
                    }
                    for (k, v) in oa.iter() {
                        match ob.get(k) {
                            Some(bv) => stack.push((v, bv)),
                            None => return false,
                        }
                    }
                    continue;
                }
                (Value::Number(na, pa), Value::Number(nb, pb)) => {
                    let min_prec = (*pa).min(*pb).min(17);
                    let factor = 10_f64.powi(i32::from(min_prec));
                    (na * factor).round().total_cmp(&(nb * factor).round())
                        == std::cmp::Ordering::Equal
                }
                (Value::Quantity(v1, p1, u1, _), Value::Quantity(v2, p2, u2, _)) => {
                    u1.to_lowercase() == u2.to_lowercase() && {
                        let min_prec = (*p1).min(*p2).min(17);
                        let factor = 10_f64.powi(i32::from(min_prec));
                        (v1 * factor).round().total_cmp(&(v2 * factor).round())
                            == std::cmp::Ordering::Equal
                    }
                }
                _ => a.compare_equivalent(b) == Some(std::cmp::Ordering::Equal),
            } {
                return false;
            }
        }
        true
    }

    fn compare_equivalent(&self, other: &Value) -> Option<std::cmp::Ordering> {
        let mut stack: Vec<(&Value, &Value)> = vec![(self, other)];
        while let Some((a, b)) = stack.pop() {
            let ord = match (a, b) {
                (Value::Null, Value::Null) => std::cmp::Ordering::Equal,
                (Value::Boolean(ba), Value::Boolean(bb)) => ba.cmp(bb),
                (Value::Number(na, _), Value::Number(nb, _)) => na.total_cmp(nb),
                (Value::String(sa), Value::String(sb)) => sa.to_lowercase().cmp(&sb.to_lowercase()),
                (Value::Date(da, _), Value::Date(db, _)) => da.cmp(db),
                (Value::DateTime(da, _, tza), Value::DateTime(db, _, tzb)) => tza
                    .map_or(*da, |o| datetime::to_utc_naive(*da, &o))
                    .cmp(&tzb.map_or(*db, |o| datetime::to_utc_naive(*db, &o))),
                (Value::Time(ta, _), Value::Time(tb, _)) => ta.cmp(tb),
                (Value::Quantity(v1, _, u1, qt1), Value::Quantity(v2, _, u2, qt2)) => v1
                    .total_cmp(v2)
                    .then_with(|| u1.to_lowercase().cmp(&u2.to_lowercase()))
                    .then_with(|| qt1.cmp(qt2)),
                (Value::Object(oa), Value::Object(ob)) => {
                    match oa.len().cmp(&ob.len()) {
                        std::cmp::Ordering::Equal => {}
                        ord => return Some(ord),
                    }
                    let mut ka: Vec<&String> = oa.keys().collect();
                    let mut kb: Vec<&String> = ob.keys().collect();
                    ka.sort();
                    kb.sort();
                    match ka.cmp(&kb) {
                        std::cmp::Ordering::Equal => {}
                        ord => return Some(ord),
                    }
                    for key in ka.into_iter().rev() {
                        stack.push((&oa[key], &ob[key]));
                    }
                    continue;
                }
                _ => return None,
            };
            if ord != std::cmp::Ordering::Equal {
                return Some(ord);
            }
        }
        Some(std::cmp::Ordering::Equal)
    }

    pub fn compare_equal(&self, other: &Value) -> Option<std::cmp::Ordering> {
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
            (Value::Number(a, _), Value::Number(b, _)) => a.partial_cmp(b),
            (Value::Date(a, pa), Value::Date(b, pb)) if pa == pb => Some(a.cmp(b)),
            (Value::DateTime(a, pa, tza), Value::DateTime(b, pb, tzb)) if pa.comparable_to(*pb) => {
                match (tza, tzb) {
                    (Some(oa), Some(ob)) => {
                        Some(datetime::to_utc_naive(*a, oa).cmp(&datetime::to_utc_naive(*b, ob)))
                    }
                    (None, None) => Some(a.cmp(b)),
                    _ => None,
                }
            }
            (Value::Time(a, pa), Value::Time(b, pb)) if pa.comparable_to(*pb) => Some(a.cmp(b)),
            (Value::String(a), Value::String(b)) => Some(a.cmp(b)),
            (Value::DateTime(dt, _, _), Value::Date(d, _)) => {
                let cmp = dt.date().cmp(d);
                if cmp == std::cmp::Ordering::Equal {
                    None
                } else {
                    Some(cmp)
                }
            }
            (Value::Date(d, _), Value::DateTime(dt, _, _)) => {
                let cmp = d.cmp(&dt.date());
                if cmp == std::cmp::Ordering::Equal {
                    None
                } else {
                    Some(cmp)
                }
            }
            _ => None,
        }
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
            Value::Number(n, _) => write!(f, "{}", n),
            Value::Date(d, p) => write!(f, "@{}", datetime::format_date(*d, *p)),
            Value::DateTime(dt, p, tz) => {
                write!(f, "@{}", datetime::format_datetime(*dt, *p, tz))
            }
            Value::Time(t, p) => write!(f, "@T{}", datetime::format_time(*t, *p)),
            Value::Quantity(v, _, u, _) => write!(f, "{} '{}'", v, u),
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
