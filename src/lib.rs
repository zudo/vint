use serde::de::SeqAccess;
use serde::de::Visitor;
use serde::ser::SerializeTuple;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::fmt;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;
use std::ops::Sub;
use std::usize;
#[macro_export]
macro_rules! vint {
    ($value:expr) => {{
        Varint::new($value as u128)
    }};
    ($value:expr, $size:expr) => {{
        Varint::<$size>::new($value as u128)
    }};
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Varint<const A: usize>(pub [u8; A]);
impl<const A: usize> Varint<A> {
    pub fn new(u: u128) -> Varint<A> {
        let mut varint = Varint([0; A]);
        if u == 0 {
            return varint;
        }
        let bytes = u.to_be_bytes();
        let mut i = 0;
        for byte in bytes {
            if byte != 0 {
                break;
            }
            i += 1;
        }
        let size = 15 - i;
        for (j, v) in varint.0.iter_mut().enumerate().take(A) {
            let k = i + j;
            if k == 16 {
                break;
            }
            *v = bytes[k];
        }
        varint.0[A - 1] = (varint.0[A - 1] & 0xf0) | size as u8;
        varint
    }
    pub fn int(self) -> u128 {
        let size = self.0[A - 1] as usize & 0x0f;
        let mut bytes = [0; 16];
        for (i, v) in self.0.iter().enumerate().take(A) {
            let j = 15 - size + i;
            if j == 16 {
                break;
            }
            if i == A - 1 {
                bytes[j] = v & 0xf0;
                break;
            }
            bytes[j] = *v;
        }
        u128::from_be_bytes(bytes)
    }
    pub fn floor(u: u128) -> u128 {
        Varint::<A>::new(u).int()
    }
}
impl<const A: usize> From<u128> for Varint<A> {
    fn from(value: u128) -> Self {
        vint![value]
    }
}
impl<const A: usize> From<Varint<A>> for u128 {
    fn from(value: Varint<A>) -> Self {
        value.int()
    }
}
impl<const A: usize, T: Into<u128>> Add<T> for Varint<A> {
    type Output = Varint<A>;
    fn add(self, other: T) -> Varint<A> {
        vint![self.int() + other.into()]
    }
}
impl<const A: usize, T: Into<u128>> Sub<T> for Varint<A> {
    type Output = Varint<A>;
    fn sub(self, other: T) -> Varint<A> {
        vint![self.int() - other.into()]
    }
}
impl<const A: usize, T: Into<u128>> Mul<T> for Varint<A> {
    type Output = Varint<A>;
    fn mul(self, other: T) -> Varint<A> {
        vint![self.int() * other.into()]
    }
}
impl<const A: usize, T: Into<u128>> Div<T> for Varint<A> {
    type Output = Varint<A>;
    fn div(self, other: T) -> Varint<A> {
        vint![self.int() / other.into()]
    }
}
impl<const A: usize, T: Into<u128>> Rem<T> for Varint<A> {
    type Output = Varint<A>;
    fn rem(self, other: T) -> Varint<A> {
        vint![self.int() % other.into()]
    }
}
impl<const A: usize> fmt::Display for Varint<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.int())
    }
}
impl<const A: usize> Serialize for Varint<A> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_tuple(A)?;
        for value in self.0.iter() {
            seq.serialize_element(value)?;
        }
        seq.end()
    }
}
impl<'de, const A: usize> Deserialize<'de> for Varint<A> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Varint<A>, D::Error> {
        struct VarintVisitor<const A: usize>;
        impl<'de, const A: usize> Visitor<'de> for VarintVisitor<A> {
            type Value = Varint<A>;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_fmt(format_args!("a byte array of length {}", A))
            }
            fn visit_seq<S: SeqAccess<'de>>(self, mut seq: S) -> Result<Self::Value, S::Error> {
                let mut array = [0; A];
                for i in 0..A {
                    array[i] = seq
                        .next_element::<u8>()?
                        .ok_or_else(|| serde::de::Error::invalid_length(i, &"more elements"))?;
                }
                Ok(Varint(array))
            }
        }
        deserializer.deserialize_tuple(A, VarintVisitor)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vint() {
        assert_eq!(vint![0], Varint::<1>::new(0));
        assert_eq!(vint![0, 1], Varint::new(0));
    }
    #[test]
    fn new() {
        assert_eq!(Varint::new(0x00).0, [0x00]);
        assert_eq!(Varint::new(0x01).0, [0x00]);
        assert_eq!(Varint::new(0x10).0, [0x10]);
        assert_eq!(Varint::new(0x11).0, [0x10]);
        assert_eq!(Varint::new(0x10000000000000000000000000000000).0, [0x1f]);
        assert_eq!(Varint::new(0x00).0, [0x00, 0x00]);
        assert_eq!(Varint::new(0x01).0, [0x01, 0x00]);
        assert_eq!(Varint::new(0x10).0, [0x10, 0x00]);
        assert_eq!(Varint::new(0x11).0, [0x11, 0x00]);
        assert_eq!(
            Varint::new(0x10000000000000000000000000000000).0,
            [0x10, 0x0f]
        );
    }
    #[test]
    fn int() {
        assert_eq!(Varint([0x00]).int(), 0x00);
        assert_eq!(Varint([0x01]).int(), 0x00);
        assert_eq!(Varint([0x10]).int(), 0x10);
        assert_eq!(Varint([0x11]).int(), 0x1000);
        assert_eq!(Varint([0x1f]).int(), 0x10000000000000000000000000000000);
        assert_eq!(Varint([0x00, 0x00]).int(), 0x00);
        assert_eq!(Varint([0x01, 0x00]).int(), 0x01);
        assert_eq!(Varint([0x10, 0x00]).int(), 0x10);
        assert_eq!(Varint([0x11, 0x00]).int(), 0x11);
        assert_eq!(
            Varint([0x10, 0x0f]).int(),
            0x10000000000000000000000000000000
        );
    }
    #[test]
    fn bincode_serialize() {
        assert_eq!(bincode::serialize(&Varint([1])).unwrap(), [1]);
        assert_eq!(bincode::serialize(&Varint([1, 1])).unwrap(), [1, 1]);
    }
    #[test]
    fn bincode_deserialize() {
        assert_eq!(
            bincode::deserialize::<Varint<1>>(&vec![1]).unwrap(),
            Varint([1])
        );
        assert_eq!(
            bincode::deserialize::<Varint<2>>(&vec![1, 1]).unwrap(),
            Varint([1, 1])
        );
    }
    #[test]
    fn serde_json_to_string() {
        assert_eq!(serde_json::to_string(&Varint([1])).unwrap(), "[1]");
        assert_eq!(serde_json::to_string(&Varint([1, 1])).unwrap(), "[1,1]");
    }
    #[test]
    fn serde_json_from_str() {
        assert_eq!(
            serde_json::from_str::<Varint<1>>("[1]").unwrap(),
            Varint([1])
        );
        assert_eq!(
            serde_json::from_str::<Varint<2>>("[1,1]").unwrap(),
            Varint([1, 1])
        );
    }
    #[test]
    fn add() {
        let a = vint![1, 2];
        assert_eq!(a + a, vint![2]);
    }
    #[test]
    fn sub() {
        let a = vint![2, 2];
        assert_eq!(a - a, vint![0]);
    }
    #[test]
    fn mul() {
        let a = vint![2, 2];
        assert_eq!(a * a, vint![4]);
    }
    #[test]
    fn div() {
        let a = vint![4, 2];
        assert_eq!(a / a, vint![1]);
    }
    #[test]
    fn rem() {
        let a = vint![4, 2];
        assert_eq!(a % a, Varint::from(0));
    }
}
