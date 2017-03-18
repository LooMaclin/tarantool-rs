use futures::{Future};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::{Encoder, Decoder, Framed};
use tokio_core::net::TcpStream;
use tokio_core::reactor::Handle;
use tokio_proto::{TcpClient};
use tokio_proto::multiplex::{RequestId, ServerProto, ClientProto, ClientService};
use tokio_service::{Service, NewService};
use bytes::{BytesMut, Buf, BufMut, BigEndian};
use std::{io};
use std::net::SocketAddr;
use tarantool;