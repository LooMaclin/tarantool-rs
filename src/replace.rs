use iterator_type::IteratorType;
use rmpv::Value;
use tarantool::{header, request, serialize_keys, process_response};
use byteorder::BigEndian;
use request_type_key::RequestTypeKey;
use code::Code;
use serde::Serialize;

#[derive(Debug, Builder)]
pub struct Replace<'a> {
    id: u32,
    space: u16,
    keys: &'a Vec<Value>,
}

impl<'a> Replace<'a> {
    pub fn perform<I>(&self)
                      -> Result<Value, String>
        where I: Serialize
    {
        let mut keys_buffer = Vec::new();
        let wrapped_keys = Value::Array(keys);
        wrapped_keys.serialize(&mut Serializer::new(&mut keys_buffer)).unwrap();
        if keys_buffer.len() == 1 {
            keys_buffer = [&[0x91][..], &keys_buffer[..]].concat();
        }
        let request_id = self.get_id();
        let header = header(RequestTypeKey::Replace, request_id);
        let mut body = [&[0x82][..],
            &[Code::SpaceId as u8][..],
            &[0xCD, 0x0, 0x0][..],
            &[Code::Tuple as u8][..],
            &keys_buffer[..]]
            .concat();
        BigEndian::write_u16(&mut body[3..5], space);
        let response = request(&header, &body);
        process_response(&response)
    }
}

