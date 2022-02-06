use std::fmt;
use std::marker;

use crate::reflect::runtime_types::RuntimeTypeEnumOrUnknown;
use crate::reflect::EnumDescriptor;
use crate::reflect::EnumValueDescriptor;
use crate::reflect::ProtobufValue;

/// Trait implemented by all protobuf enum types.
pub trait ProtobufEnum: Eq + Sized + Copy + 'static + ProtobufValue + fmt::Debug + Default {
    /// Get enum `i32` value.
    fn value(&self) -> i32;

    /// Try to create an enum from `i32` value.
    /// Return `None` if value is unknown.
    fn from_i32(v: i32) -> Option<Self>;

    /// Get all enum values for enum type.
    fn values() -> &'static [Self];

    /// Get enum value descriptor.
    fn descriptor(&self) -> EnumValueDescriptor {
        self.enum_descriptor()
            .get_value_by_number(self.value())
            .unwrap()
    }

    /// Get enum descriptor.
    fn enum_descriptor(&self) -> EnumDescriptor {
        Self::enum_descriptor_static()
    }

    /// Get enum descriptor by type.
    fn enum_descriptor_static() -> EnumDescriptor {
        panic!();
    }
}

/// Protobuf enums with possibly unknown values are preserved in this struct.
#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
#[repr(transparent)]
// TODO: specify <E: ProtobufEnum> when it no longer prevents using const fns
pub struct EnumOrUnknown<E> {
    value: i32,
    _marker: marker::PhantomData<E>,
}

// TODO: move into <E: ProtobufEnum> when no longer:
//  trait bounds other than `Sized` on const fn parameters are unstable
impl<E> EnumOrUnknown<E> {
    /// Construct from any `i32` value.
    ///
    /// Note passed value is not required to be a valid enum value.
    pub const fn from_i32(value: i32) -> EnumOrUnknown<E> {
        EnumOrUnknown {
            value,
            _marker: marker::PhantomData,
        }
    }
}

impl<E: ProtobufEnum> EnumOrUnknown<E> {
    /// Construct from typed enum
    pub fn new(e: E) -> EnumOrUnknown<E> {
        EnumOrUnknown::from_i32(e.value())
    }

    /// Get contained `i32` value of enum
    pub fn value(&self) -> i32 {
        self.value
    }

    /// Get `i32` value as typed enum. Return `None` is value is unknown.
    pub fn enum_value(&self) -> Result<E, i32> {
        E::from_i32(self.value).ok_or(self.value)
    }

    /// Get contained enum, panic if value is unknown.
    pub fn unwrap(&self) -> E {
        self.enum_value().unwrap()
    }

    /// Get `i32` value as typed enum.
    /// Return default enum value (first value) if value is unknown.
    pub fn enum_value_or_default(&self) -> E {
        self.enum_value().unwrap_or_default()
    }

    /// Get `i32` value as typed enum.
    /// Return given enum value if value is unknown.
    pub fn enum_value_or(&self, map_unknown: E) -> E {
        self.enum_value().unwrap_or(map_unknown)
    }

    /// Get enum descriptor by type.
    pub fn enum_descriptor_static() -> EnumDescriptor {
        E::enum_descriptor_static()
    }
}

impl<E: ProtobufEnum> From<E> for EnumOrUnknown<E> {
    fn from(e: E) -> Self {
        EnumOrUnknown::new(e)
    }
}

impl<E: ProtobufEnum> Default for EnumOrUnknown<E> {
    fn default() -> EnumOrUnknown<E> {
        EnumOrUnknown::new(E::default())
    }
}

impl<E: ProtobufEnum> fmt::Debug for EnumOrUnknown<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.enum_value() {
            Ok(e) => fmt::Debug::fmt(&e, f),
            Err(e) => fmt::Debug::fmt(&e, f),
        }
    }
}

impl<E: ProtobufEnum + ProtobufValue> ProtobufValue for EnumOrUnknown<E> {
    type RuntimeType = RuntimeTypeEnumOrUnknown<E>;
}
