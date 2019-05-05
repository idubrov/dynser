use failure::Fail;
use std::collections::HashMap;

#[derive(Debug, Fail)]
pub enum ReflectionError {
    #[fail(
        display = "Primitive value type mismatch, expected '{:?}', got '{:?}'",
        expected, actual
    )]
    ValueMismatch {
        expected: PrimitiveValueKind,
        actual: PrimitiveValueKind,
    },
    #[fail(display = "Field '{}' does not exist", name)]
    InvalidField { name: String },
}

// Primitive values

#[derive(Debug, Clone, Copy)]
pub enum PrimitiveValueKind {
    String,
    Bool,
}

pub enum PrimitiveValue {
    String(String),
    Bool(bool),
}

impl PrimitiveValue {
    fn kind(&self) -> PrimitiveValueKind {
        match self {
            PrimitiveValue::String(_) => PrimitiveValueKind::String,
            PrimitiveValue::Bool(_) => PrimitiveValueKind::Bool,
        }
    }
}

// Reflection API

pub enum FieldMutReflection<'a> {
    Object(&'a mut dyn Object),
    List(&'a mut dyn List),
    Primitive(&'a mut dyn Primitive),
    Any(&'a mut serde_json::Value),
}

pub trait Object {
    fn create(&mut self, field_name: &str) -> Result<FieldMutReflection, ReflectionError>;
}

pub trait List {
    fn push(&mut self) -> FieldMutReflection;
}

pub trait Primitive {
    fn kind(&self) -> PrimitiveValueKind;
    fn set(&mut self, value: PrimitiveValue) -> Result<(), ReflectionError>;
}

// Blanket implementations

impl<T> Object for HashMap<String, T>
where
    T: Object + Default,
{
    fn create(&mut self, field_name: &str) -> Result<FieldMutReflection, ReflectionError> {
        let child = self.entry(field_name.to_owned()).or_insert_with(T::default);
        Ok(FieldMutReflection::Object(child))
    }
}

impl<T> Object for HashMap<String, Vec<T>>
where
    T: Object + Default,
{
    fn create(&mut self, field_name: &str) -> Result<FieldMutReflection, ReflectionError> {
        let list = self.entry(field_name.to_owned()).or_insert_with(Vec::new);
        Ok(FieldMutReflection::List(list))
    }
}

impl<T> List for Vec<T>
where
    T: Object + Default,
{
    fn push(&mut self) -> FieldMutReflection {
        self.push(T::default());
        FieldMutReflection::Object(self.last_mut().unwrap())
    }
}

// Any (serde_json::Value) support

// serde_json::Value

impl Object for HashMap<String, serde_json::Value> {
    fn create(&mut self, field_name: &str) -> Result<FieldMutReflection, ReflectionError> {
        let child = self
            .entry(field_name.to_owned())
            .or_insert(serde_json::Value::Null);
        Ok(FieldMutReflection::Any(child))
    }
}

impl Object for HashMap<String, Vec<serde_json::Value>> {
    fn create(&mut self, field_name: &str) -> Result<FieldMutReflection, ReflectionError> {
        let list = self.entry(field_name.to_owned()).or_insert_with(Vec::new);
        Ok(FieldMutReflection::List(list))
    }
}

impl List for Vec<serde_json::Value> {
    fn push(&mut self) -> FieldMutReflection {
        self.push(serde_json::Value::Null);
        FieldMutReflection::Any(self.last_mut().unwrap())
    }
}

// Primitives support

macro_rules! primitive {
    ($typ:ty => $kind:ident) => {
        impl Primitive for $typ {
            fn kind(&self) -> PrimitiveValueKind {
                PrimitiveValueKind::$kind
            }

            fn set(&mut self, value: PrimitiveValue) -> Result<(), ReflectionError> {
                match value {
                    PrimitiveValue::$kind(s) => {
                        *self = s;
                        Ok(())
                    }
                    v => Err(ReflectionError::ValueMismatch {
                        expected: PrimitiveValueKind::$kind,
                        actual: v.kind(),
                    }),
                }
            }
        }

        impl Object for HashMap<String, $typ> {
            fn create(&mut self, field_name: &str) -> Result<FieldMutReflection, ReflectionError> {
                let primitive = self
                    .entry(field_name.to_owned())
                    .or_insert_with(Default::default);
                Ok(FieldMutReflection::Primitive(primitive))
            }
        }

        impl Object for HashMap<String, Vec<$typ>> {
            fn create(&mut self, field_name: &str) -> Result<FieldMutReflection, ReflectionError> {
                let list = self.entry(field_name.to_owned()).or_insert_with(Vec::new);
                Ok(FieldMutReflection::List(list))
            }
        }

        impl List for Vec<$typ> {
            fn push(&mut self) -> FieldMutReflection {
                self.push(<$typ>::default());
                FieldMutReflection::Primitive(self.last_mut().unwrap())
            }
        }
    };
}

primitive!(String => String);
primitive!(bool => Bool);
