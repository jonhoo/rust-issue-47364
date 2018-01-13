pub trait InputLength {
    #[inline]
    fn input_len(&self) -> usize;
}

impl<'a, T> InputLength for &'a [T] {
    #[inline]
    fn input_len(&self) -> usize {
        self.len()
    }
}

impl<'a> InputLength for &'a str {
    #[inline]
    fn input_len(&self) -> usize {
        self.len()
    }
}

impl<'a> InputLength for (&'a [u8], usize) {
    #[inline]
    fn input_len(&self) -> usize {
        //println!("bit input length for ({:?}, {}):", self.0, self.1);
        let res = self.0.len() * 8 - self.1;
        //println!("-> {}", res);
        res
    }
}

use std::iter::Enumerate;
use std::str::CharIndices;

pub trait AsChar {
    #[inline]
    fn as_char(self) -> char;
    #[inline]
    fn is_alpha(self) -> bool;
    #[inline]
    fn is_alphanum(self) -> bool;
    #[inline]
    fn is_0_to_9(self) -> bool;
    #[inline]
    fn is_hex_digit(self) -> bool;
    #[inline]
    fn is_oct_digit(self) -> bool;
}

impl<'a> AsChar for &'a u8 {
    #[inline]
    fn as_char(self) -> char {
        *self as char
    }
    #[inline]
    fn is_alpha(self) -> bool {
        (*self >= 0x41 && *self <= 0x5A) || (*self >= 0x61 && *self <= 0x7A)
    }
    #[inline]
    fn is_alphanum(self) -> bool {
        self.is_alpha() || self.is_0_to_9()
    }
    #[inline]
    fn is_0_to_9(self) -> bool {
        *self >= 0x30 && *self <= 0x39
    }
    #[inline]
    fn is_hex_digit(self) -> bool {
        (*self >= 0x30 && *self <= 0x39) || (*self >= 0x41 && *self <= 0x46)
            || (*self >= 0x61 && *self <= 0x66)
    }
    #[inline]
    fn is_oct_digit(self) -> bool {
        *self >= 0x30 && *self <= 0x37
    }
}

impl AsChar for char {
    #[inline]
    fn as_char(self) -> char {
        self
    }
    #[inline]
    fn is_alpha(self) -> bool {
        self.is_alphabetic()
    }
    #[inline]
    fn is_alphanum(self) -> bool {
        self.is_alpha() || self.is_0_to_9()
    }
    #[inline]
    fn is_0_to_9(self) -> bool {
        self.is_digit(10)
    }
    #[inline]
    fn is_hex_digit(self) -> bool {
        self.is_digit(16)
    }
    #[inline]
    fn is_oct_digit(self) -> bool {
        self.is_digit(8)
    }
}

pub trait IterIndices {
    type Item: AsChar;
    type Iter: Iterator<Item = (usize, Self::Item)>;
    fn iter_indices(self) -> Self::Iter;
}

impl<'a> IterIndices for &'a [u8] {
    type Item = &'a u8;
    type Iter = Enumerate<::std::slice::Iter<'a, u8>>;
    #[inline]
    fn iter_indices(self) -> Enumerate<::std::slice::Iter<'a, u8>> {
        self.iter().enumerate()
    }
}

impl<'a> IterIndices for &'a str {
    type Item = char;
    type Iter = CharIndices<'a>;
    #[inline]
    fn iter_indices(self) -> CharIndices<'a> {
        self.char_indices()
    }
}

pub trait AsBytes {
    fn as_bytes(&self) -> &[u8];
}

impl<'a> AsBytes for &'a str {
    #[inline(always)]
    fn as_bytes(&self) -> &[u8] {
        str::as_bytes(self)
    }
}

impl AsBytes for str {
    #[inline(always)]
    fn as_bytes(&self) -> &[u8] {
        str::as_bytes(self)
    }
}

impl<'a> AsBytes for &'a [u8] {
    #[inline(always)]
    fn as_bytes(&self) -> &[u8] {
        *self
    }
}

impl AsBytes for [u8] {
    #[inline(always)]
    fn as_bytes(&self) -> &[u8] {
        self
    }
}

macro_rules! array_impls {
  ($($N:expr)+) => {
    $(
      impl<'a> AsBytes for &'a [u8; $N] {
        #[inline(always)]
        fn as_bytes(&self) -> &[u8] {
          *self
        }
      }

      impl AsBytes for [u8; $N] {
        #[inline(always)]
        fn as_bytes(&self) -> &[u8] {
          self
        }
      }
    )+
  };
}

array_impls! {
     0  1  2  3  4  5  6  7  8  9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32
}

/// indicates which parser returned an error
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ErrorKind<E = u32> {
    Custom(E),
    Tag,
    MultiSpace,
    Complete,
}

pub fn error_to_u32<E>(e: &ErrorKind<E>) -> u32 {
    match *e {
        ErrorKind::Custom(_) => 0,
        ErrorKind::Tag => 1,
        ErrorKind::MultiSpace => 21,
        ErrorKind::Complete => 48,
    }
}
