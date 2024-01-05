//! # Functions and types used by generated protobuf code
//!
//! These are not considered to be public API of rust-protobuf,
//! so they can be changed any time (provided compatibility with
//! previously generated code is preserved).

pub(crate) mod map;
mod message;
pub(crate) mod packed;
pub(crate) mod repeated;
pub(crate) mod unknown_or_group;

pub use map::compute_map_size;
pub use map::read_map_into;
pub use map::write_map_with_cached_sizes;
pub use message::read_singular_message_into_field;
pub use message::write_message_field_with_cached_size;
pub use packed::vec_packed_bool_size;
pub use packed::vec_packed_double_size;
pub use packed::vec_packed_enum_or_unknown_size;
pub use packed::vec_packed_fixed32_size;
pub use packed::vec_packed_fixed64_size;
pub use packed::vec_packed_float_size;
pub use packed::vec_packed_int32_size;
pub use packed::vec_packed_int64_size;
pub use packed::vec_packed_sfixed32_size;
pub use packed::vec_packed_sfixed64_size;
pub use packed::vec_packed_sint32_size;
pub use packed::vec_packed_sint64_size;
pub use packed::vec_packed_uint32_size;
pub use packed::vec_packed_uint64_size;
pub use repeated::read_repeated_packed_enum_or_unknown_into;
pub use unknown_or_group::read_unknown_or_skip_group;
pub use unknown_or_group::unknown_fields_size;

pub use crate::cached_size::CachedSize;
pub use crate::lazy::Lazy;
use crate::varint::encode::encoded_varint64_len;
pub use crate::wire_format::WireType;
use crate::zigzag::ProtobufVarintZigzag;

/// Given `u64` value compute varint encoded length.
pub fn compute_raw_varint64_size(value: u64) -> u64 {
    encoded_varint64_len(value) as u64
}

/// Given `u32` value compute varint encoded length.
pub(crate) fn compute_raw_varint32_size(value: u32) -> u64 {
    compute_raw_varint64_size(value as u64)
}

/// Helper trait implemented by integer types which could be encoded as varint.
pub trait ProtobufVarint {
    /// Size of self when encoded as varint.
    fn len_varint(&self) -> u64;
}

impl ProtobufVarint for u64 {
    fn len_varint(&self) -> u64 {
        compute_raw_varint64_size(*self)
    }
}

impl ProtobufVarint for u32 {
    fn len_varint(&self) -> u64 {
        (*self as u64).len_varint()
    }
}

impl ProtobufVarint for i64 {
    fn len_varint(&self) -> u64 {
        // same as length of u64
        (*self as u64).len_varint()
    }
}

impl ProtobufVarint for i32 {
    fn len_varint(&self) -> u64 {
        // sign-extend and then compute
        (*self as i64).len_varint()
    }
}

impl ProtobufVarint for bool {
    fn len_varint(&self) -> u64 {
        1
    }
}

/// Compute tag size. Size of tag does not depend on wire type.
#[inline]
pub fn tag_size(field_number: u32) -> u64 {
    encoded_varint64_len((field_number as u64) << 3) as u64
}

/// Integer value size when encoded.
#[inline]
fn varint_size<T: ProtobufVarint>(field_number: u32, value: T) -> u64 {
    tag_size(field_number) + value.len_varint()
}

/// Encoded `int32` size.
#[inline]
pub fn int32_size(field_number: u32, value: i32) -> u64 {
    varint_size(field_number, value)
}

/// Encoded `int64` size.
#[inline]
pub fn int64_size(field_number: u32, value: i64) -> u64 {
    varint_size(field_number, value)
}

/// Encoded `uint32` size.
#[inline]
pub fn uint32_size(field_number: u32, value: u32) -> u64 {
    varint_size(field_number, value)
}

/// Encoded `uint64` size.
#[inline]
pub fn uint64_size(field_number: u32, value: u64) -> u64 {
    varint_size(field_number, value)
}

/// Integer value size when encoded as specified wire type.
pub(crate) fn value_varint_zigzag_size_no_tag<T: ProtobufVarintZigzag>(value: T) -> u64 {
    value.len_varint_zigzag()
}

/// Length of value when encoding with zigzag encoding with tag
#[inline]
fn value_varint_zigzag_size<T: ProtobufVarintZigzag>(field_number: u32, value: T) -> u64 {
    tag_size(field_number) + value_varint_zigzag_size_no_tag(value)
}

/// Size of serialized `sint32` field.
#[inline]
pub fn sint32_size(field_number: u32, value: i32) -> u64 {
    value_varint_zigzag_size(field_number, value)
}

/// Size of serialized `sint64` field.
#[inline]
pub fn sint64_size(field_number: u32, value: i64) -> u64 {
    value_varint_zigzag_size(field_number, value)
}

/// Size of encoded bytes field.
pub(crate) fn bytes_size_no_tag(bytes: &[u8]) -> u64 {
    compute_raw_varint64_size(bytes.len() as u64) + bytes.len() as u64
}

/// Size of encoded bytes field.
#[inline]
pub fn bytes_size(field_number: u32, bytes: &[u8]) -> u64 {
    tag_size(field_number) + bytes_size_no_tag(bytes)
}

/// Size of encoded string field.
pub(crate) fn string_size_no_tag(s: &str) -> u64 {
    bytes_size_no_tag(s.as_bytes())
}

/// Size of encoded string field.
#[inline]
pub fn string_size(field_number: u32, s: &str) -> u64 {
    tag_size(field_number) + string_size_no_tag(s)
}
