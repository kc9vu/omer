use std::fmt;

#[cfg(feature = "serde")]
use serde::{ser, de};
#[cfg(feature = "serde")]
use serde_untagged::UntaggedEnumVisitor;

/// 表示单个或多个有组合, 用于序列化很方便
pub enum Omer<T> {
    One(T),
    More(Vec<T>),
}

impl<T> Clone for Omer<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Omer::One(t) => Omer::One(t.clone()),
            Omer::More(v) => Omer::More(v.clone()),
        }
    }
}

impl<T> Default for Omer<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::One(T::default())
    }
}

impl<T> fmt::Debug for Omer<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Omer::One(t) => t.fmt(f),
            Omer::More(v) => v.fmt(f),
        }
    }
}

#[cfg(feature = "serde")]
impl<T> ser::Serialize for Omer<T>
where
    T: ser::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match self {
            Omer::One(s) => s.serialize(serializer),
            Omer::More(m) => m.serialize(serializer),
        }
    }
}

#[cfg(feature = "serde")]
impl<'de, T> de::Deserialize<'de> for Omer<T>
where
    T: for<'d> de::Deserialize<'d>,
{
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        use de::SeqAccess as _;

        UntaggedEnumVisitor::new()
            .expecting("object or [object]")
            .seq(|mut seq| {
                let mut v = vec![];
                while let Some(ele) = seq.next_element::<T>()? {
                    v.push(ele);
                }
                Ok(Omer::More(v))
            })
            .bool(|b| {
                Ok(Omer::One(T::deserialize(
                    de::value::BoolDeserializer::new(b),
                )?))
            })
            .borrowed_str(|s| {
                Ok(Omer::One(T::deserialize(
                    de::value::StrDeserializer::new(s),
                )?))
            })
            .i32(|value| {
                Ok(Omer::One(T::deserialize(
                    de::value::I32Deserializer::new(value),
                )?))
            })
            .map(|map| {
                Ok(Omer::One(T::deserialize(
                    de::value::MapAccessDeserializer::new(map),
                )?))
            })
            .string(|s| {
                Ok(Omer::One(T::deserialize(
                    de::value::StringDeserializer::new(s.to_string()),
                )?))
            })
            .deserialize(d)
    }
}
