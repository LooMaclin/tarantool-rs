#![feature(custom_derive)]
extern crate rmpv;
extern crate rmp_serde;
extern crate serde;

#[macro_use]
extern crate rmp;

extern crate base64;
extern crate sha1;
extern crate rmp_serialize;
extern crate rustc_serialize;
extern crate byteorder;
extern crate hex_slice;

#[macro_use]
extern crate log;

pub mod tarantool;
pub mod code;
pub mod greeting_packet;
pub mod greeting_packet_parameters;
pub mod upsert_operation;
pub mod integer_operation;
pub mod string_operation;
pub mod common_operation;
pub mod request_type_key;
pub mod iterator_type;
pub mod header;
pub mod response;
pub mod select;
pub mod insert;
pub mod upsert;
pub mod update_integer;
pub mod update_common;
pub mod update_string;
pub mod eval;
pub mod call;
pub mod delete;
pub mod action;
pub mod replace;
pub mod insertable;

pub use rmpv::Value;
pub use tarantool::Tarantool;
pub use select::Select;
pub use insert::Insert;
pub use update_common::UpdateCommon;
pub use update_string::UpdateString;
pub use update_integer::UpdateInteger;
pub use replace::Replace;
pub use eval::Eval;
pub use delete::Delete;
pub use call::Call;
pub use iterator_type::IteratorType;
pub use common_operation::CommonOperation;
pub use integer_operation::IntegerOperation;
pub use upsert::Upsert;
pub use upsert_operation::UpsertOperation;
pub use rmpv::{Utf8String, Integer};
pub use insertable::Insertable;

pub const CHAP_SHA_1: [u8; 10] = [0xA9, 0x63, 0x68, 0x61, 0x70, 0x2d, 0x73, 0x68, 0x61, 0x31];
pub const FIX_STR_PREFIX: u8 = 0xA1;
pub const TARANTOOL_SPACE_ID: u64 = 280;
pub const TARANTOOL_SPACE_ID_KEY_NUMBER: u64 = 2;
pub const TARANTOOL_INDEX_ID: u64 = 288;
pub const TARANTOOL_INDEX_ID_KEY_NUMBER: u64 = 2;
