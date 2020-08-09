/// ByteRotator directs any type implementing it to have a rotate_byte method which must modify
/// the value of the input byte in some way and return the new modified value.
pub trait ByteRotator {
  fn rotate_byte(&self, byte: u8) -> u8;
}
