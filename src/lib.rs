use serde::de::SeqAccess;
use serde::de::Visitor;
use serde::ser::SerializeTuple;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use std::fmt;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::BitAnd;
use std::ops::BitAndAssign;
use std::ops::BitOr;
use std::ops::BitOrAssign;
use std::ops::BitXor;
use std::ops::BitXorAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Rem;
use std::ops::RemAssign;
use std::ops::Shl;
use std::ops::ShlAssign;
use std::ops::Shr;
use std::ops::ShrAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::usize;
#[macro_export]
macro_rules! vint {
    ($value:expr) => {{
        Vint::from($value as u128)
    }};
    ($value:expr, $size:expr) => {{
        Vint::<$size>::from($value as u128)
    }};
}
#[macro_export]
macro_rules! floor {
    ($value:expr, $size:expr) => {{
        u128::from(Vint::<$size>::from($value as u128))
    }};
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Vint<const A: usize>(pub [u8; A]);
// default
impl<const A: usize> Default for Vint<A> {
    fn default() -> Vint<A> {
        Vint([0; A])
    }
}
// display
impl<const A: usize> fmt::Display for Vint<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", u128::from(*self))
    }
}
// serde
impl<const A: usize> Serialize for Vint<A> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_tuple(A)?;
        for value in self.0.iter() {
            seq.serialize_element(value)?;
        }
        seq.end()
    }
}
impl<'de, const A: usize> Deserialize<'de> for Vint<A> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Vint<A>, D::Error> {
        struct VintVisitor<const A: usize>;
        impl<'de, const A: usize> Visitor<'de> for VintVisitor<A> {
            type Value = Vint<A>;
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
                Ok(Vint(array))
            }
        }
        deserializer.deserialize_tuple(A, VintVisitor)
    }
}
// from
impl<const A: usize> From<Vint<A>> for u128 {
    fn from(vint: Vint<A>) -> Self {
        let size = vint.0[A - 1] as usize & 0x0f;
        let mut bytes = [0; 16];
        for (i, v) in vint.0.iter().enumerate().take(A) {
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
}
impl<const A: usize> From<u128> for Vint<A> {
    fn from(u: u128) -> Self {
        let mut vint = Vint([0; A]);
        if u == 0 {
            return vint;
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
        for (j, v) in vint.0.iter_mut().enumerate().take(A) {
            let k = i + j;
            if k == 16 {
                break;
            }
            *v = bytes[k];
        }
        vint.0[A - 1] = (vint.0[A - 1] & 0xf0) | size as u8;
        vint
    }
}
// add
impl<const A: usize> Add<Vint<A>> for u128 {
    type Output = Vint<A>;
    fn add(self, rhs: Vint<A>) -> Vint<A> {
        vint![self + u128::from(rhs)]
    }
}
impl<const A: usize, T: Into<u128>> Add<T> for Vint<A> {
    type Output = Vint<A>;
    fn add(self, rhs: T) -> Vint<A> {
        vint![u128::from(self) + rhs.into()]
    }
}
impl<const A: usize> AddAssign<Vint<A>> for u128 {
    fn add_assign(&mut self, rhs: Vint<A>) {
        *self += u128::from(rhs);
    }
}
impl<const A: usize, T: Into<u128>> AddAssign<T> for Vint<A> {
    fn add_assign(&mut self, rhs: T) {
        *self = vint![u128::from(*self) + rhs.into()];
    }
}
// sub
impl<const A: usize> Sub<Vint<A>> for u128 {
    type Output = Vint<A>;
    fn sub(self, rhs: Vint<A>) -> Vint<A> {
        vint![self - u128::from(rhs)]
    }
}
impl<const A: usize, T: Into<u128>> Sub<T> for Vint<A> {
    type Output = Vint<A>;
    fn sub(self, rhs: T) -> Vint<A> {
        vint![u128::from(self) - rhs.into()]
    }
}
impl<const A: usize> SubAssign<Vint<A>> for u128 {
    fn sub_assign(&mut self, rhs: Vint<A>) {
        *self -= u128::from(rhs);
    }
}
impl<const A: usize, T: Into<u128>> SubAssign<T> for Vint<A> {
    fn sub_assign(&mut self, rhs: T) {
        *self = vint![u128::from(*self) - rhs.into()];
    }
}
// mul
impl<const A: usize> Mul<Vint<A>> for u128 {
    type Output = Vint<A>;
    fn mul(self, rhs: Vint<A>) -> Vint<A> {
        vint![self * u128::from(rhs)]
    }
}
impl<const A: usize, T: Into<u128>> Mul<T> for Vint<A> {
    type Output = Vint<A>;
    fn mul(self, rhs: T) -> Vint<A> {
        vint![u128::from(self) * rhs.into()]
    }
}
impl<const A: usize> MulAssign<Vint<A>> for u128 {
    fn mul_assign(&mut self, rhs: Vint<A>) {
        *self *= u128::from(rhs);
    }
}
impl<const A: usize, T: Into<u128>> MulAssign<T> for Vint<A> {
    fn mul_assign(&mut self, rhs: T) {
        *self = vint![u128::from(*self) * rhs.into()];
    }
}
// div
impl<const A: usize> Div<Vint<A>> for u128 {
    type Output = Vint<A>;
    fn div(self, rhs: Vint<A>) -> Vint<A> {
        vint![self / u128::from(rhs)]
    }
}
impl<const A: usize, T: Into<u128>> Div<T> for Vint<A> {
    type Output = Vint<A>;
    fn div(self, rhs: T) -> Vint<A> {
        vint![u128::from(self) / rhs.into()]
    }
}
impl<const A: usize> DivAssign<Vint<A>> for u128 {
    fn div_assign(&mut self, rhs: Vint<A>) {
        *self /= u128::from(rhs);
    }
}
impl<const A: usize, T: Into<u128>> DivAssign<T> for Vint<A> {
    fn div_assign(&mut self, rhs: T) {
        *self = vint![u128::from(*self) / rhs.into()];
    }
}
// rem
impl<const A: usize> Rem<Vint<A>> for u128 {
    type Output = Vint<A>;
    fn rem(self, rhs: Vint<A>) -> Vint<A> {
        vint![self % u128::from(rhs)]
    }
}
impl<const A: usize, T: Into<u128>> Rem<T> for Vint<A> {
    type Output = Vint<A>;
    fn rem(self, rhs: T) -> Vint<A> {
        vint![u128::from(self) % rhs.into()]
    }
}
impl<const A: usize> RemAssign<Vint<A>> for u128 {
    fn rem_assign(&mut self, rhs: Vint<A>) {
        *self %= u128::from(rhs);
    }
}
impl<const A: usize, T: Into<u128>> RemAssign<T> for Vint<A> {
    fn rem_assign(&mut self, rhs: T) {
        *self = vint![u128::from(*self) % rhs.into()];
    }
}
// bitand
impl<const A: usize> BitAnd<Vint<A>> for u128 {
    type Output = Vint<A>;
    fn bitand(self, rhs: Vint<A>) -> Vint<A> {
        vint![self & u128::from(rhs)]
    }
}
impl<const A: usize, T: Into<u128>> BitAnd<T> for Vint<A> {
    type Output = Vint<A>;
    fn bitand(self, rhs: T) -> Vint<A> {
        vint![u128::from(self) & rhs.into()]
    }
}
impl<const A: usize> BitAndAssign<Vint<A>> for u128 {
    fn bitand_assign(&mut self, rhs: Vint<A>) {
        *self &= u128::from(rhs);
    }
}
impl<const A: usize, T: Into<u128>> BitAndAssign<T> for Vint<A> {
    fn bitand_assign(&mut self, rhs: T) {
        *self = vint![u128::from(*self) & rhs.into()];
    }
}
// bitor
impl<const A: usize> BitOr<Vint<A>> for u128 {
    type Output = Vint<A>;
    fn bitor(self, rhs: Vint<A>) -> Vint<A> {
        vint![self | u128::from(rhs)]
    }
}
impl<const A: usize, T: Into<u128>> BitOr<T> for Vint<A> {
    type Output = Vint<A>;
    fn bitor(self, rhs: T) -> Vint<A> {
        vint![u128::from(self) | rhs.into()]
    }
}
impl<const A: usize> BitOrAssign<Vint<A>> for u128 {
    fn bitor_assign(&mut self, rhs: Vint<A>) {
        *self |= u128::from(rhs);
    }
}
impl<const A: usize, T: Into<u128>> BitOrAssign<T> for Vint<A> {
    fn bitor_assign(&mut self, rhs: T) {
        *self = vint![u128::from(*self) | rhs.into()];
    }
}
// bitxor
impl<const A: usize> BitXor<Vint<A>> for u128 {
    type Output = Vint<A>;
    fn bitxor(self, rhs: Vint<A>) -> Vint<A> {
        vint![self ^ u128::from(rhs)]
    }
}
impl<const A: usize, T: Into<u128>> BitXor<T> for Vint<A> {
    type Output = Vint<A>;
    fn bitxor(self, rhs: T) -> Vint<A> {
        vint![u128::from(self) ^ rhs.into()]
    }
}
impl<const A: usize> BitXorAssign<Vint<A>> for u128 {
    fn bitxor_assign(&mut self, rhs: Vint<A>) {
        *self ^= u128::from(rhs);
    }
}
impl<const A: usize, T: Into<u128>> BitXorAssign<T> for Vint<A> {
    fn bitxor_assign(&mut self, rhs: T) {
        *self = vint![u128::from(*self) ^ rhs.into()];
    }
}
// shl
impl<const A: usize> Shl<Vint<A>> for u128 {
    type Output = Vint<A>;
    fn shl(self, rhs: Vint<A>) -> Vint<A> {
        vint![self << u128::from(rhs)]
    }
}
impl<const A: usize, T: Into<u128>> Shl<T> for Vint<A> {
    type Output = Vint<A>;
    fn shl(self, rhs: T) -> Vint<A> {
        vint![u128::from(self) << rhs.into()]
    }
}
impl<const A: usize> ShlAssign<Vint<A>> for u128 {
    fn shl_assign(&mut self, rhs: Vint<A>) {
        *self <<= u128::from(rhs);
    }
}
impl<const A: usize, T: Into<u128>> ShlAssign<T> for Vint<A> {
    fn shl_assign(&mut self, rhs: T) {
        *self = vint![u128::from(*self) << rhs.into()];
    }
}
// shr
impl<const A: usize> Shr<Vint<A>> for u128 {
    type Output = Vint<A>;
    fn shr(self, rhs: Vint<A>) -> Vint<A> {
        vint![self >> u128::from(rhs)]
    }
}
impl<const A: usize, T: Into<u128>> Shr<T> for Vint<A> {
    type Output = Vint<A>;
    fn shr(self, rhs: T) -> Vint<A> {
        vint![u128::from(self) >> rhs.into()]
    }
}
impl<const A: usize> ShrAssign<Vint<A>> for u128 {
    fn shr_assign(&mut self, rhs: Vint<A>) {
        *self >>= u128::from(rhs);
    }
}
impl<const A: usize, T: Into<u128>> ShrAssign<T> for Vint<A> {
    fn shr_assign(&mut self, rhs: T) {
        *self = vint![u128::from(*self) >> rhs.into()];
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn macros() {
        assert_eq!(vint![0].0, [0]);
        assert_eq!(vint![0, 1].0, [0]);
        assert_eq!(vint![0], Vint::<1>::from(0));
        assert_eq!(vint![0, 1], Vint::from(0));
        assert_eq!(vint![0x00].0, [0x00]);
        assert_eq!(vint![0x01].0, [0x00]);
        assert_eq!(vint![0x10].0, [0x10]);
        assert_eq!(vint![0x11].0, [0x10]);
        assert_eq!(vint![0x10000000000000000000000000000000].0, [0x1f]);
        assert_eq!(vint![0x00].0, [0x00, 0x00]);
        assert_eq!(vint![0x01].0, [0x01, 0x00]);
        assert_eq!(vint![0x10].0, [0x10, 0x00]);
        assert_eq!(vint![0x11].0, [0x11, 0x00]);
        assert_eq!(vint![0x10000000000000000000000000000000].0, [0x10, 0x0f]);
        assert_eq!(floor![100, 1], 96);
    }
    #[test]
    fn bincode_serialize() {
        assert_eq!(bincode::serialize(&Vint([1])).unwrap(), [1]);
        assert_eq!(bincode::serialize(&Vint([1, 1])).unwrap(), [1, 1]);
    }
    #[test]
    fn bincode_deserialize() {
        assert_eq!(bincode::deserialize::<Vint<1>>(&[1]).unwrap(), Vint([1]));
        assert_eq!(
            bincode::deserialize::<Vint<2>>(&[1, 1]).unwrap(),
            Vint([1, 1])
        );
    }
    #[test]
    fn serde_json_to_string() {
        assert_eq!(serde_json::to_string(&Vint([1])).unwrap(), "[1]");
        assert_eq!(serde_json::to_string(&Vint([1, 1])).unwrap(), "[1,1]");
    }
    #[test]
    fn serde_json_from_str() {
        assert_eq!(serde_json::from_str::<Vint<1>>("[1]").unwrap(), Vint([1]));
        assert_eq!(
            serde_json::from_str::<Vint<2>>("[1,1]").unwrap(),
            Vint([1, 1])
        );
    }
    #[test]
    fn from() {
        assert_eq!(u128::from(Vint([0x00])), 0x00);
        assert_eq!(u128::from(Vint([0x01])), 0x00);
        assert_eq!(u128::from(Vint([0x10])), 0x10);
        assert_eq!(u128::from(Vint([0x11])), 0x1000);
        assert_eq!(u128::from(Vint([0x1f])), 0x10000000000000000000000000000000);
        assert_eq!(u128::from(Vint([0x00, 0x00])), 0x00);
        assert_eq!(u128::from(Vint([0x01, 0x00])), 0x01);
        assert_eq!(u128::from(Vint([0x10, 0x00])), 0x10);
        assert_eq!(u128::from(Vint([0x11, 0x00])), 0x11);
        assert_eq!(
            u128::from(Vint([0x10, 0x0f])),
            0x10000000000000000000000000000000
        );
    }
    #[test]
    fn add() {
        {
            let a = 1;
            let b = vint![1, 2];
            assert_eq!(a + b, vint![2]);
            assert_eq!(b + a, vint![2]);
            assert_eq!(b + b, vint![2]);
        }
        {
            let mut a = 1;
            a += vint![1, 2];
            assert_eq!(a, 2);
        }
        {
            let mut a = vint![1, 2];
            a += 1_u128;
            assert_eq!(u128::from(a), 2);
        }
    }
    #[test]
    fn sub() {
        {
            let a = 1;
            let b = vint![1, 2];
            assert_eq!(a - b, vint![0]);
            assert_eq!(b - a, vint![0]);
            assert_eq!(b - b, vint![0]);
        }
        {
            let mut a = 1;
            a -= vint![1, 2];
            assert_eq!(a, 0);
        }
        {
            let mut a = vint![1, 2];
            a -= 1_u128;
            assert_eq!(u128::from(a), 0);
        }
    }
    #[test]
    fn mul() {
        {
            let a = 2;
            let b = vint![2, 2];
            assert_eq!(a * b, vint![4]);
            assert_eq!(b * a, vint![4]);
            assert_eq!(b * b, vint![4]);
        }
        {
            let mut a = 2;
            a *= vint![2, 2];
            assert_eq!(a, 4);
        }
        {
            let mut a = vint![2, 2];
            a *= 2_u128;
            assert_eq!(u128::from(a), 4);
        }
    }
    #[test]
    fn div() {
        {
            let a = 2;
            let b = vint![2, 2];
            assert_eq!(a / b, vint![1]);
            assert_eq!(b / a, vint![1]);
            assert_eq!(b / b, vint![1]);
        }
        {
            let mut a = 2;
            a /= vint![2, 2];
            assert_eq!(a, 1);
        }
        {
            let mut a = vint![2, 2];
            a /= 2_u128;
            assert_eq!(u128::from(a), 1);
        }
    }
    #[test]
    fn rem() {
        {
            let a = 2;
            let b = vint![2, 2];
            assert_eq!(a % b, Vint::from(0));
            assert_eq!(b % a, Vint::from(0));
            assert_eq!(b % b, Vint::from(0));
        }
        {
            let mut a = 2;
            a %= vint![2, 2];
            assert_eq!(a, 0);
        }
        {
            let mut a = vint![2, 2];
            a %= 2_u128;
            assert_eq!(u128::from(a), 0);
        }
    }
    #[test]
    fn default() {
        assert_eq!(Vint::default(), Vint([]));
        assert_eq!(Vint::default(), Vint([0]));
        assert_eq!(Vint::default(), Vint([0, 0]));
    }
}
