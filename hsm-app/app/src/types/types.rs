#[derive(Clone, Debug)]
pub struct Bytes(pub Vec<u8>);

#[derive(Clone, Debug)]
pub struct Signature(pub Vec<u8>);

#[derive(Clone, Debug)]
pub enum Message {
	Sign(Bytes),
	Verify(Bytes, Signature),
}
