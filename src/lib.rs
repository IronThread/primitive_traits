#![no_std]

//! Simple crate with the only purporse of export traits implemented by different kinds of
//! primitives in rust,like signed and unsigned integers,with all the shared trait implementations
//! and the capacity of downcasting with [`Any`].

use core::any::Any;
use core::default::Default;
use core::fmt::{Binary, Debug, Display, LowerExp, LowerHex, Octal, Pointer, UpperExp, UpperHex};
use core::hash::Hash;
use core::iter::{Product, Step, Sum};
use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};
use core::ptr::NonNull;
use core::str::FromStr;

/// Trait representing signed integers,like `i32`.
pub trait Integer:
    Any
    + Copy
    + Sized
    + Display
    + Binary
    + LowerHex
    + UpperHex
    + Octal
    + Product
    + Step
    + FromStr
    + Default
    + Debug
    + PartialEq
    + Eq
    + Hash
    + PartialOrd
    + Ord
    + Add
    + AddAssign
    + Sub
    + SubAssign
    + Mul
    + MulAssign
    + Div
    + DivAssign
    + Rem
    + RemAssign
    + Shr
    + ShrAssign
    + Shl
    + ShlAssign
    + Shr<i8>
    + ShrAssign<i8>
    + Shl<i8>
    + ShlAssign<i8>
    + Shr<u8>
    + ShrAssign<u8>
    + Shl<u8>
    + ShlAssign<i16>
    + Shr<i16>
    + ShrAssign<i16>
    + Shl<i16>
    + ShlAssign<i16>
    + Shr<u16>
    + ShrAssign<u16>
    + Shl<u16>
    + ShlAssign<u16>
    + Shr<i32>
    + ShrAssign<i32>
    + Shl<i32>
    + ShlAssign<i32>
    + Shr<u32>
    + ShrAssign<u32>
    + Shl<u32>
    + ShlAssign<u32>
    + Shr<i32>
    + ShrAssign<i32>
    + Shl<i32>
    + ShlAssign<i32>
    + Shr<i64>
    + ShrAssign<i64>
    + Shl<i64>
    + ShlAssign<i64>
    + Shr<u64>
    + ShrAssign<u64>
    + Shl<u64>
    + ShlAssign<u64>
    + Shr<i128>
    + ShrAssign<i128>
    + Shl<i128>
    + ShlAssign<i128>
    + Shr<u128>
    + ShrAssign<u128>
    + Shl<u128>
    + ShlAssign<u128>
    + Shr<usize>
    + ShrAssign<usize>
    + Shl<usize>
    + ShlAssign<usize>
    + Shr<isize>
    + ShrAssign<isize>
    + Shl<isize>
    + ShlAssign<isize>
    + Neg
    + Not
    + BitOr
    + BitOrAssign
    + BitXor
    + BitXorAssign
    + BitAnd
    + BitAndAssign
    + Sum
    + From<i8>
    + From<u8>
    + From<bool>
    + Into<i128>
{
    /// Identity method with the only purporse of make this safe as a trait object,althought this
    /// could already have been done by the supertraits.
    #[doc(hidden)]
    fn i(&self) -> &Self {
        self
    }
}

impl Integer for i8 {}
impl Integer for i16 {}
impl Integer for i32 {}
impl Integer for i64 {}
impl Integer for i128 {}

/// Trait representing unsigned integers,like `u32`.
pub trait Unsigned:
    Any
    + Copy
    + Sized
    + Display
    + Debug
    + Display
    + Binary
    + LowerHex
    + UpperHex
    + Octal
    + Product
    + Step
    + FromStr
    + Default
    + PartialEq
    + Eq
    + Hash
    + PartialOrd
    + Ord
    + Add
    + AddAssign
    + Sub
    + SubAssign
    + Mul
    + MulAssign
    + Div
    + DivAssign
    + Rem
    + RemAssign
    + Shr
    + ShrAssign
    + Shl
    + ShlAssign
    + Shr<i8>
    + ShrAssign<i8>
    + Shl<i8>
    + ShlAssign<i8>
    + Shr<u8>
    + ShrAssign<u8>
    + Shl<u8>
    + ShlAssign<i16>
    + Shr<i16>
    + ShrAssign<i16>
    + Shl<i16>
    + ShlAssign<i16>
    + Shr<u16>
    + ShrAssign<u16>
    + Shl<u16>
    + ShlAssign<u16>
    + Shr<i32>
    + ShrAssign<i32>
    + Shl<i32>
    + ShlAssign<i32>
    + Shr<u32>
    + ShrAssign<u32>
    + Shl<u32>
    + ShlAssign<u32>
    + Shr<i32>
    + ShrAssign<i32>
    + Shl<i32>
    + ShlAssign<i32>
    + Shr<i64>
    + ShrAssign<i64>
    + Shl<i64>
    + ShlAssign<i64>
    + Shr<u64>
    + ShrAssign<u64>
    + Shl<u64>
    + ShlAssign<u64>
    + Shr<i128>
    + ShrAssign<i128>
    + Shl<i128>
    + ShlAssign<i128>
    + Shr<u128>
    + ShrAssign<u128>
    + Shl<u128>
    + ShlAssign<u128>
    + Shr<usize>
    + ShrAssign<usize>
    + Shl<usize>
    + ShlAssign<usize>
    + Shr<isize>
    + ShrAssign<isize>
    + Shl<isize>
    + ShlAssign<isize>
    + Not
    + BitOr
    + BitOrAssign
    + BitXor
    + BitXorAssign
    + BitAnd
    + BitAndAssign
    + Sum
    + From<u8>
    + From<bool>
    + Into<u128>
{
    /// Identity method with the only purporse of make this safe as a trait object,althought this
    /// could already have been done by the supertraits.
    #[doc(hidden)]
    fn i(&self) -> &Self {
        self
    }
}

impl Unsigned for u8 {}
impl Unsigned for u16 {}
impl Unsigned for u32 {}
impl Unsigned for u64 {}
impl Unsigned for u128 {}

/// Type representing floating decimal comma integer types,like `f64`.
pub trait Float:
    Any
    + Copy
    + Sized
    + Display
    + Debug
    + Display
    + Product
    + FromStr
    + Default
    + PartialEq
    + PartialOrd
    + Add
    + AddAssign
    + Sub
    + SubAssign
    + Mul
    + MulAssign
    + Div
    + DivAssign
    + Rem
    + RemAssign
    + Sum
    + LowerExp
    + Neg
    + UpperExp
    + From<u8>
    + From<u16>
    + Into<u16>
    + Into<i16>
{
    /// Identity method with the only purporse of make this safe as a trait object,althought this
    /// could already have been done by the supertraits.
    #[doc(hidden)]
    fn i(&self) -> &Self {
        self
    }
}

impl Float for f32 {}
impl Float for f64 {}

pub trait RawPointer<T: ?Sized>:
    Any + Clone + Copy + PartialEq + Eq + PartialOrd + Ord + Pointer
{
    /// Identity method with the only purporse of make this safe as a trait object,althought this
    /// could already have been done by the supertraits.
    #[doc(hidden)]
    fn i(&self) -> &Self {
        self
    }
}

impl<T: ?Sized> RawPointer for *const T {}
impl<T: ?Sized> RawPointer for *mut T {}
impl<T: ?Sized> RawPointer for NonNull<T> {}
