pub use self::util::*;
pub use self::internal::*;
pub use self::macros::*;

#[macro_use]
mod util;
mod internal;
#[macro_use]
mod macros;

#[macro_export]
macro_rules! tag (
  ($i:expr, $inp: expr) => (
    {
      #[inline(always)]
      fn as_bytes<T: $crate::AsBytes>(b: &T) -> &[u8] {
        b.as_bytes()
      }

      let expected = $inp;
      let bytes    = as_bytes(&expected);

      tag_bytes!($i,bytes)
    }
  );
);

#[macro_export]
macro_rules! tag_bytes (
  ($i:expr, $bytes: expr) => (
    {
      let len = $i.len();
      let blen = $bytes.len();
      let m   = if len < blen { len } else { blen };
      let reduced = &$i[..m];
      let b       = &$bytes[..m];

      let res: $crate::IResult<_,_> = if reduced != b {
        $crate::IResult::Error($crate::Err::Position($crate::ErrorKind::Tag, $i))
      } else if m < blen {
        $crate::IResult::Incomplete($crate::Needed::Size(blen))
      } else {
        $crate::IResult::Done(&$i[blen..], reduced)
      };
      res
    }
  );
);

use std::ops::{Index, Range, RangeFrom};
use internal::IResult::*;
use internal::Err::*;
/// Recognizes spaces, tabs, carriage returns and line feeds
pub fn multispace<'a, T: ?Sized>(input: &'a T) -> IResult<&'a T, &'a T>
where
    T: Index<Range<usize>, Output = T> + Index<RangeFrom<usize>, Output = T>,
    &'a T: IterIndices + InputLength,
{
    let input_length = input.input_len();
    if input_length == 0 {
        return Error(Position(ErrorKind::MultiSpace, input));
    }

    for (idx, item) in input.iter_indices() {
        let chr = item.as_char();
        if !(chr == ' ' || chr == '\t' || chr == '\r' || chr == '\n') {
            if idx == 0 {
                return Error(Position(ErrorKind::MultiSpace, input));
            } else {
                return Done(&input[idx..], &input[0..idx]);
            }
        }
    }
    Done(&input[input_length..], input)
}
