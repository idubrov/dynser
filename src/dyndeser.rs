#![allow(clippy::identity_conversion)]

// https://github.com/rust-lang/rust-clippy/issues/3944
use crate::reflection::{FieldMutReflection, List, Object, Primitive, PrimitiveValue};
use serde::de::{
    self, Deserialize, DeserializeSeed, Deserializer, Error, MapAccess, SeqAccess, Unexpected,
    Visitor,
};
use serde_json::de::{Deserializer as JsonDeserializer, StrRead};
use std::fmt;
use std::marker::PhantomData;

struct ObjectVisitor<'a> {
    object: &'a mut Object,
}

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
            let field = self
                .object
                .create(field_name)
                .map_err(|_| A::Error::unknown_field(field_name, &[]))?;
            match field {
                FieldMutReflection::Primitive(primitive) => {
                    map.next_value_seed(PrimitiveVisitor::new_seed(primitive))?;
                }
                FieldMutReflection::Object(object) => {
                    map.next_value_seed(ObjectVisitor::new_seed(object))?;
                }
                FieldMutReflection::List(list) => {
                    map.next_value_seed(ListVisitor::new_seed(list))?;
                }
                FieldMutReflection::Any(value) => {
                    *value = map.next_value::<serde_json::Value>()?;
                }
            }
        }
        Ok(())
    }
}

struct ListVisitor<'a> {
    list: &'a mut List,
}

impl<'a, 'de> Visitor<'de> for ListVisitor<'a> {
    type Value = ();

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "array")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<(), A::Error>
    where
        A: SeqAccess<'de>,
    {
        while let Some(()) = seq.next_element_seed(ListEntrySeed::new_seed(self.list))? {}

        Ok(())
    }
}

struct ListEntrySeed<'a> {
    list: &'a mut List,
}

impl<'a, 'de> DeserializeSeed<'de> for ListEntrySeed<'a> {
    type Value = ();
    fn deserialize<D>(self, deserializer: D) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        match self.list.push() {
            FieldMutReflection::Primitive(primitive) => {
                let seed = PrimitiveVisitor::new_seed(primitive);
                seed.deserialize(deserializer)?;
            }
            FieldMutReflection::Object(object) => {
                let seed = ObjectVisitor::new_seed(object);
                seed.deserialize(deserializer)?;
            }
            FieldMutReflection::List(list) => {
                let seed = ListVisitor::new_seed(list);
                seed.deserialize(deserializer)?;
            }
            FieldMutReflection::Any(value) => {
                *value = serde_json::Value::deserialize(deserializer)?;
            }
        }
        Ok(())
    }
}

struct PrimitiveVisitor<'a> {
    primitive: &'a mut Primitive,
}

impl<'a, 'de> Visitor<'de> for PrimitiveVisitor<'a> {
    type Value = ();

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.primitive.kind())
    }

    fn visit_bool<E>(self, v: bool) -> Result<(), E>
    where
        E: de::Error,
    {
        self.primitive
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
        self.primitive
            .set(PrimitiveValue::String(v))
            .map_err(|_| Error::invalid_type(Unexpected::Other("string"), &self))
    }
}

pub struct VisitorSeed<'de, V: Visitor<'de>>(V, PhantomData<&'de ()>);

impl<'de, V: Visitor<'de>> DeserializeSeed<'de> for VisitorSeed<'de, V> {
    type Value = <V as Visitor<'de>>::Value;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(self.0)
    }
}

impl ObjectVisitor<'_> {
    pub fn new_seed<'a, 'de: 'a>(
        object: &'a mut Object,
    ) -> impl DeserializeSeed<'de, Value = ()> + 'a {
        VisitorSeed(ObjectVisitor { object }, PhantomData)
    }
}

impl PrimitiveVisitor<'_> {
    pub fn new_seed<'a, 'de: 'a>(
        primitive: &'a mut Primitive,
    ) -> impl DeserializeSeed<'de, Value = ()> + 'a {
        VisitorSeed(PrimitiveVisitor { primitive }, PhantomData)
    }
}

impl ListVisitor<'_> {
    pub fn new_seed<'a, 'de: 'a>(list: &'a mut List) -> impl DeserializeSeed<'de, Value = ()> + 'a {
        VisitorSeed(ListVisitor { list }, PhantomData)
    }
}

impl<'a, 'de: 'a> ListEntrySeed<'de> {
    pub fn new_seed(list: &'a mut List) -> impl DeserializeSeed<'de, Value = ()> + 'a {
        ListEntrySeed { list }
    }
}

pub fn read_json(input: &str, object: &mut Object) -> Result<(), serde_json::error::Error> {
    let mut de = JsonDeserializer::new(StrRead::new(input));
    let seed = ObjectVisitor::new_seed(object);
    seed.deserialize(&mut de)?;
    Ok(())
}
