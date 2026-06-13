use crate::net::NetMessageHeader;
use bytes::{Buf, BytesMut};
use std::fmt::Debug;
use std::io::Error as IoError;
use std::{any::type_name, io::Write};
use steam_vent_proto_common::{MsgKindEnum, RpcMessage, RpcMessageWithKind};
use tracing::trace;

/// A message which can be encoded and/or decoded
///
/// Applications can implement this trait on a struct to allow sending it using
/// [`raw_send_with_kind`](steam_vent::Connection::raw_send_with_kind). To use the higher level messages a struct also needs to implement
/// [`NetMessage`]
pub trait EncodableMessage: Sized + Debug + Send {
    fn read_body(_data: BytesMut, _header: &NetMessageHeader) -> Result<Self, IoError> {
        panic!("Reading not implemented for {}", type_name::<Self>())
    }

    fn write_body<W: Write>(&self, _writer: W) -> Result<(), IoError> {
        panic!("Writing not implemented for {}", type_name::<Self>())
    }

    fn encode_size(&self) -> usize {
        panic!("Writing not implemented for {}", type_name::<Self>())
    }

    fn process_header(&self, _header: &mut NetMessageHeader) {}
}

/// A message with associated kind
pub trait NetMessage: EncodableMessage {
    type KindEnum: MsgKindEnum;
    const KIND: Self::KindEnum;
    const IS_PROTOBUF: bool = false;
}

impl<ProtoMsg: RpcMessageWithKind + Send> EncodableMessage for ProtoMsg {
    fn read_body(data: BytesMut, _header: &NetMessageHeader) -> Result<Self, IoError> {
        trace!("reading body of protobuf message {:?}", Self::KIND);
        Self::parse(&mut data.reader()).map_err(IoError::from)
    }

    fn write_body<W: Write>(&self, mut writer: W) -> Result<(), IoError> {
        trace!("writing body of protobuf message {:?}", Self::KIND);
        self.write(&mut writer)
            .map_err(|_| IoError::from(std::io::ErrorKind::InvalidData))
    }

    fn encode_size(&self) -> usize {
        <Self as RpcMessage>::encode_size(self)
    }
}

impl<ProtoMsg: RpcMessageWithKind + Send> NetMessage for ProtoMsg {
    type KindEnum = ProtoMsg::KindEnum;
    const KIND: Self::KindEnum = <ProtoMsg as RpcMessageWithKind>::KIND;
    const IS_PROTOBUF: bool = true;
}
