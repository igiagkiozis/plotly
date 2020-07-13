use crate::common::color::{Color, ColorWrapper};
use serde::{Serialize, Serializer};

pub trait NumOrString {
    fn to_num_or_string(&self) -> NumOrStringWrapper;
}

pub fn owned_string_vector<S: AsRef<str>>(s: Vec<S>) -> Vec<String> {
    s.iter()
        .map(|x| x.as_ref().to_string())
        .collect::<Vec<String>>()
}

pub fn to_color_array<C: Color>(v: Vec<C>) -> Vec<ColorWrapper> {
    let mut sv: Vec<ColorWrapper> = Vec::with_capacity(v.len());
    for e in v.iter() {
        sv.push(e.to_color());
    }
    sv
}

pub fn to_num_or_string_wrapper<C: NumOrString>(v: Vec<C>) -> Vec<NumOrStringWrapper> {
    let mut sv: Vec<NumOrStringWrapper> = Vec::with_capacity(v.len());
    for e in v.iter() {
        sv.push(e.to_num_or_string());
    }
    sv
}

pub fn is_valid_color_array(a: &[ColorWrapper]) -> bool {
    let mut sv: Vec<String> = Vec::new();
    let mut fv: Vec<f64> = Vec::new();
    for e in a.iter() {
        match e {
            ColorWrapper::F(n) => fv.push(*n),
            ColorWrapper::S(s) => sv.push(s.clone()),
        }
    }

    !sv.is_empty() && !fv.is_empty()
}

impl NumOrString for String {
    fn to_num_or_string(&self) -> NumOrStringWrapper {
        NumOrStringWrapper::S(self.to_owned())
    }
}

impl NumOrString for str {
    fn to_num_or_string(&self) -> NumOrStringWrapper {
        NumOrStringWrapper::S(self.to_owned())
    }
}

impl NumOrString for &String {
    fn to_num_or_string(&self) -> NumOrStringWrapper {
        NumOrStringWrapper::S(String::from(*self))
    }
}

impl NumOrString for &str {
    fn to_num_or_string(&self) -> NumOrStringWrapper {
        NumOrStringWrapper::S(String::from(*self))
    }
}

impl NumOrString for f64 {
    fn to_num_or_string(&self) -> NumOrStringWrapper {
        NumOrStringWrapper::F(*self)
    }
}

impl NumOrString for usize {
    fn to_num_or_string(&self) -> NumOrStringWrapper {
        NumOrStringWrapper::U(*self as u64)
    }
}

impl NumOrString for i32 {
    fn to_num_or_string(&self) -> NumOrStringWrapper {
        NumOrStringWrapper::I(*self as i64)
    }
}

impl NumOrString for i64 {
    fn to_num_or_string(&self) -> NumOrStringWrapper {
        NumOrStringWrapper::I(*self)
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum NumOrStringWrapper {
    S(String),
    F(f64),
    I(i64),
    U(u64),
}

#[derive(Debug)]
pub struct TruthyEnum<E> {
    pub e: E,
}

impl<E> Serialize for TruthyEnum<E>
where
    E: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let s = serde_json::to_string(&self.e)
            .unwrap()
            .chars()
            .filter(|c| *c != '"')
            .collect::<String>();
        if s == "true" {
            return serializer.serialize_bool(true);
        }
        if s == "false" {
            return serializer.serialize_bool(false);
        }
        serializer.serialize_str(&s)
    }
}

pub fn copy_iterable_to_vec<'a, T, I>(iterable: I) -> Vec<T>
where
    I: IntoIterator<Item = T>,
{
    iterable.into_iter().collect::<Vec<T>>()
}
