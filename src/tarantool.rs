use std::borrow::Cow;
use std::net::TcpStream;
use std::io::Read;
use std::ops::BitXor;
use std::collections::BTreeMap;
use std::io::Write;
use std::io::Cursor;

use base64::{decode};
use sha1::{Sha1};
use rustc_serialize::{Encodable, Decodable};
use msgpack::{Encoder, Decoder};
use rmp::decode::{read_map_size};
use byteorder::{BigEndian, ByteOrder};
use rmp::encode::{write_u32, write_u8};
use hex_slice::AsHex;

use greeting_packet::GreetingPacket;
use code::Code;
use request_type_key::RequestTypeKey;
use protocol_parts::ProtocolParts;

#[derive(Debug)]
pub struct Tarantool<'a> {
    address: Cow<'a, str>,
    user: Cow<'a, str>,
    password: Cow<'a, str>,
    greeting_packet: GreetingPacket<'a>,
    request_id: u32,
    socket: TcpStream,
}

#[derive(Debug)]
pub struct Response<'a> {
    len: u32,
    header: Header,
    body: Optional<Vec<u8>>,
}

#[derive(Debug)]
pub struct Header {
    code: u32,
    sync: u64,
    schema_id:  u32,
}

impl<'a> Tarantool<'a> {
    pub fn new<S>(address: S, user: S, password: S) -> Tarantool<'a>
        where S: Into<Cow<'a, str>> {
        let mut stream = TcpStream::connect("127.0.0.1:3301").unwrap();
        let mut buf = [0; 128];
        stream.read(&mut buf);
        Tarantool {
            address: address.into(),
            user: user.into(),
            password: password.into(),
            greeting_packet: GreetingPacket::new(
                String::from_utf8(buf[64..108].to_vec()).unwrap(),
                String::from_utf8(buf[..64].to_vec()).unwrap(),
            ),
            request_id: 0,
            socket: stream,
        }
    }

    pub fn get_id(&mut self) -> u32 {
        self.request_id+=1;
        self.request_id
    }

    pub fn header(&self, command: RequestTypeKey, request_id: u32) -> Vec<u8> {
        let mut encoded_header = [
            &[0x82][..],
            &[Code::RequestType as u8][..],
            &[command as u8][..],
            &[Code::Sync as u8][..],
            &[0, 0, 0, 0, 0]].concat();
        write_u32(&mut &mut encoded_header[4..], request_id).ok().unwrap();
        encoded_header
    }

    pub fn request(&mut self, header: &[u8], body: &[u8]) {
        let mut encoded_request_length = [0x00, 0x00, 0x00, 0x00, 0x00];
        write_u32(&mut &mut encoded_request_length[..],
                  (header.len() + body.len()) as u32).ok().unwrap();
        let request = [&encoded_request_length[..],&header[..],&body[..]].concat();
        self.socket.write(&request);
        let response_length = self.read_length();
        let payload = self.read_payload(response_length);
        println!("Greeting: {:?}", &self.greeting_packet);
        println!("request(size: {}): {:#X}", &request.len(), &request.as_hex());
        println!("length(size: {}): {:#X}", &encoded_request_length.len(), &encoded_request_length.as_hex());
        println!("header(size: {}): {:#X}", &header.len(), &header.as_hex());
        println!("body(size: {}): {:#X}", &body.len(), &body.as_hex());
        println!("payload(size: {}): {:#X}", &payload.len(), &payload.as_hex());
        println!("payload(as text): {}", String::from_utf8_lossy(&payload));
    }

    pub fn read_length(&mut self) -> u32 {
        let mut packet_length = [0x00, 0x00, 0x00, 0x00, 0x00];
        self.socket.read(&mut packet_length);
        let mut decoder = Decoder::new(&packet_length[..]);
        let mut length = Decodable::decode(&mut decoder).unwrap();
        length
    }

    pub fn read_payload(&mut self, length: u32) -> Vec<u8> {
        let mut payload = vec![0u8; length as usize];
        self.socket.read(&mut payload);
        payload
    }

    pub fn scramble<S>(salt: S, password: S) -> Vec<u8>
        where S: Into<Cow<'a, str>> {
        let decoded_salt = &decode(&salt.into()).unwrap()[..];
        let mut step_1 = Sha1::new();
        step_1.update(&(password.into()[..]).as_bytes());
        let mut step_2 = Sha1::new();
        step_2.update(&step_1.digest().bytes());
        let mut step_3 = Sha1::new();
        step_3.update(&[&decoded_salt[..20], &step_2.digest().bytes()].concat());
        let digest_1 = step_1.digest().bytes();
        let digest_3 = step_3.digest().bytes();
        (0..20).into_iter()
            .map(|n| {
                digest_1[n] ^ digest_3[n]
            })
            .collect::<Vec<u8>>()
    }

    pub fn auth(&mut self) {
        let scramble = Tarantool::scramble(&*self.greeting_packet.salt, &*self.password);
        println!("scramble (size: {}): {:#X}",&scramble.len(), &scramble.as_hex());
        let id = self.get_id();
        let header = self.header(RequestTypeKey::Auth, id);
        let mut chap_sha1_encoded = Vec::new();
        "chap-sha1".encode(&mut Encoder::new(&mut &mut chap_sha1_encoded[..]));
        let username = self.user.clone().into_owned().into_bytes();
        println!("scramble (as text): {}", String::from_utf8_lossy(&scramble[..]));
        println!("chap-sha1 size bytes: {}", &"chap-sha1".as_bytes().len());
        println!("user name: {:?}", &username);
        let body = [
            &[0x82][..],
            &[Code::UserName as u8][..],
            //TODO: Set the username string size dynamically. A4 - harcoded value for `test`
            &[0xA4][..],
            &username[..],
            &[Code::Tuple as u8, 0x92, 0xA9][..],
            &"chap-sha1".as_bytes(),
            &[0xC4, 0x14][..],
            &scramble[..]
        ].concat();
        self.request(&header, &body);
    }
}

#[cfg(test)]
mod test {

    use super::Tarantool;
    use hex_slice::AsHex;

    #[test]
    fn scramble_result() {
        let scramble = Tarantool::scramble("WPE4wY2+RTBuFvElfHawAheh37sa58XKR/ZEOvgRsa8=", "123");
        assert_eq!([0xAC, 0x3F, 0xAD, 0x90, 0x6F, 0xFE, 0x80, 0x28, 0x92, 0x79, 0xCE, 0xC3, 0xFC,
                   0xDA, 0x0B, 0x86, 0xBD, 0x06, 0x2A, 0x69], &scramble[..]);
    }
}