use crate::reflection::{FieldMutReflection, List, Object, Primitive, PrimitiveValue};
use serde::de::{
    self, Deserialize, DeserializeSeed, Deserializer, Error, MapAccess, SeqAccess, Unexpected,
    Visitor,
};
use serde_json::de::{Deserializer as JsonDeserializer, StrRead};
use std::fmt;

struct ObjectVisitor<'a>(&'a mut Object);

impl<'a, 'de> Visitor<'de> for ObjectVisitor<'a> {
    type Value = ();

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "object")
    }

    fn visit_map<A>(self, mut map: A) -> Result<(), A::Error>
    where
        A: MapAccess<'de>,
    {
        while let Some(field_name) = map.next_key::<&str>()? {
            let field = self.0.create(field_name);
            match field {
                Ok(FieldMutReflection::Primitive(primitive)) => {
                    map.next_value_seed(VisitorSeed(PrimitiveVisitor(primitive)))?;
                }
                Ok(FieldMutReflection::Object(object)) => {
                    map.next_value_seed(VisitorSeed(ObjectVisitor(object)))?;
                }
                Ok(FieldMutReflection::List(list)) => {
                    map.next_value_seed(VisitorSeed(ListVisitor(list)))?;
                }
                Ok(FieldMutReflection::Any(value)) => {
                    *value = map.next_value::<serde_json::Value>()?;
                }
                Err(_) => {
                    // Ignoring unknown fields
                    map.next_value::<serde::de::IgnoredAny>()?;

                    // Alternatively, raise an error
                    // return Err(A::Error::unknown_field(field_name, &[]))
                }
            }
        }
        Ok(())
    }
}

struct ListVisitor<'a>(&'a mut List);

impl<'a, 'de> Visitor<'de> for ListVisitor<'a> {
    type Value = ();

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "array")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<(), A::Error>
    where
        A: SeqAccess<'de>,
    {
        while let Some(()) = seq.next_element_seed(ListEntrySeed(self.0))? {}

        Ok(())
    }
}

struct ListEntrySeed<'a>(&'a mut List);

impl<'a, 'de> DeserializeSeed<'de> for ListEntrySeed<'a> {
    type Value = ();
    fn deserialize<D>(self, deserializer: D) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        match self.0.push() {
            FieldMutReflection::Primitive(primitive) => {
                deserializer.deserialize_any(PrimitiveVisitor(primitive))?;
            }
            FieldMutReflection::Object(object) => {
                deserializer.deserialize_any(ObjectVisitor(object))?;
            }
            FieldMutReflection::List(list) => {
                deserializer.deserialize_any(ListVisitor(list))?;
            }
            FieldMutReflection::Any(value) => {
                *value = serde_json::Value::deserialize(deserializer)?;
            }
        }
        Ok(())
    }
}

struct PrimitiveVisitor<'a>(&'a mut Primitive);

impl<'a, 'de> Visitor<'de> for PrimitiveVisitor<'a> {
    type Value = ();

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0.kind())
    }

    fn visit_bool<E>(self, v: bool) -> Result<(), E>
    where
        E: de::Error,
    {
        self.0
            .set(PrimitiveValue::Bool(v))
            .map_err(|_| Error::invalid_type(Unexpected::Bool(v), &self))
    }

    fn visit_str<E>(self, v: &str) -> Result<(), E>
    where
        E: de::Error,
    {
        self.visit_string(v.to_owned())
    }

    fn visit_string<E>(self, v: String) -> Result<(), E>
    where
        E: de::Error,
    {
        self.0
            .set(PrimitiveValue::String(v))
            .map_err(|_| Error::invalid_type(Unexpected::Other("string"), &self))
    }
}

struct VisitorSeed<V: for<'de> Visitor<'de>>(V);

impl<'de, V: for<'a> Visitor<'a>> DeserializeSeed<'de> for VisitorSeed<V> {
    type Value = <V as Visitor<'de>>::Value;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(self.0)
    }
}

pub fn read_json(input: &str, object: &mut Object) -> Result<(), serde_json::error::Error> {
    let mut de = JsonDeserializer::new(StrRead::new(input));
    de.deserialize_any(ObjectVisitor(object))?;
    Ok(())
}
