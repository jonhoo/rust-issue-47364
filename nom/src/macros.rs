#[macro_export]
macro_rules! named (
    (pub $name:ident<$i:ty,$o:ty>, $submac:ident!( $($args:tt)* )) => (
        pub fn $name( i: $i ) -> $crate::IResult<$i, $o> {
            $submac!(i, $($args)*)
        }
    );
);

#[macro_export]
macro_rules! call (
  ($i:expr, $fun:expr) => ( $fun( $i ) );
  ($i:expr, $fun:expr, $($args:expr),* ) => ( $fun( $i, $($args),* ) );
);

#[macro_export]
macro_rules! complete (
  ($i:expr, $submac:ident!( $($args:tt)* )) => (
    {
      match $submac!($i, $($args)*) {
        $crate::IResult::Done(i, o)    => $crate::IResult::Done(i, o),
        $crate::IResult::Error(e)      => $crate::IResult::Error(e),
        $crate::IResult::Incomplete(_) =>  {
          $crate::IResult::Error($crate::Err::Position($crate::ErrorKind::Complete, $i))
        },
      }
    }
  );
  ($i:expr, $f:expr) => (
    complete!($i, call!($f));
  );
);

#[macro_export]
macro_rules! chain (
  ($i:expr, $($rest:tt)*) => (
    {
      chaining_parser!($i, 0usize, $($rest)*)
    }
  );
);

/// Internal parser, do not use directly
#[doc(hidden)]
#[macro_export]
macro_rules! chaining_parser (
  ($i:expr, $consumed:expr, $submac:ident!( $($args:tt)* ) ~ $($rest:tt)*) => (
    {
      match $submac!($i, $($args)*) {
        $crate::IResult::Error(e)      => $crate::IResult::Error(e),
        $crate::IResult::Incomplete($crate::Needed::Unknown) => $crate::IResult::Incomplete($crate::Needed::Unknown),
        $crate::IResult::Incomplete($crate::Needed::Size(i)) => $crate::IResult::Incomplete($crate::Needed::Size($consumed + i)),
        $crate::IResult::Done(i,_)     => {
          chaining_parser!(i, $consumed + ($crate::InputLength::input_len(&($i)) - $crate::InputLength::input_len(&i)), $($rest)*)
        }
      }
    }
);

  ($i:expr, $consumed:expr, $e:ident ? ~ $($rest:tt)*) => (
    chaining_parser!($i, $consumed, call!($e) ? ~ $($rest)*);
  );

  ($i:expr, $consumed:expr, $submac:ident!( $($args:tt)* ) ? ~ $($rest:tt)*) => (
    {
      let res = $submac!($i, $($args)*);
      if let $crate::IResult::Incomplete(inc) = res {
        match inc {
          $crate::Needed::Unknown => $crate::IResult::Incomplete($crate::Needed::Unknown),
          $crate::Needed::Size(i) => $crate::IResult::Incomplete($crate::Needed::Size($consumed + i)),
        }
      } else {
        let input = if let $crate::IResult::Done(i,_) = res {
          i
        } else {
          $i
        };
        chaining_parser!(input, $consumed + ($crate::InputLength::input_len(&($i)) - $crate::InputLength::input_len(&input)), $($rest)*)
      }
    }
  );

  ($i:expr, $consumed:expr, $field:ident : $submac:ident!( $($args:tt)* ) ~ $($rest:tt)*) => (
    {
      match  $submac!($i, $($args)*) {
        $crate::IResult::Error(e)      => $crate::IResult::Error(e),
        $crate::IResult::Incomplete($crate::Needed::Unknown) => $crate::IResult::Incomplete($crate::Needed::Unknown),
        $crate::IResult::Incomplete($crate::Needed::Size(i)) => $crate::IResult::Incomplete($crate::Needed::Size($consumed + i)),
        $crate::IResult::Done(i,o)     => {
          let $field = o;
          chaining_parser!(i, $consumed + ($crate::InputLength::input_len(&($i)) - $crate::InputLength::input_len(&i)), $($rest)*)
        }
      }
    }
  );

  ($i:expr, $consumed:expr, $submac:ident!( $($args:tt)* ), $assemble:expr) => (
    match $submac!($i, $($args)*) {
      $crate::IResult::Error(e)      => $crate::IResult::Error(e),
      $crate::IResult::Incomplete($crate::Needed::Unknown) => $crate::IResult::Incomplete($crate::Needed::Unknown),
      $crate::IResult::Incomplete($crate::Needed::Size(i)) => $crate::IResult::Incomplete($crate::Needed::Size($consumed + i)),
      $crate::IResult::Done(i,_)     => {
        $crate::IResult::Done(i, $assemble())
      }
    }
  );

  ($i:expr, $consumed:expr, $field:ident : $e:ident, $assemble:expr) => (
    chaining_parser!($i, $consumed, $field: call!($e), $assemble);
  );

  ($i:expr, $consumed:expr, $field:ident : $submac:ident!( $($args:tt)* ), $assemble:expr) => (
    match $submac!($i, $($args)*) {
      $crate::IResult::Error(e)      => $crate::IResult::Error(e),
      $crate::IResult::Incomplete($crate::Needed::Unknown) => $crate::IResult::Incomplete($crate::Needed::Unknown),
      $crate::IResult::Incomplete($crate::Needed::Size(i)) => $crate::IResult::Incomplete($crate::Needed::Size($consumed + i)),
      $crate::IResult::Done(i,o)     => {
        let $field = o;
        $crate::IResult::Done(i, $assemble())
      }
    }
  );

  ($i:expr, $consumed:expr, $assemble:expr) => (
    $crate::IResult::Done($i, $assemble())
  )
);

#[macro_export]
macro_rules! opt(
  ($i:expr, $submac:ident!( $($args:tt)* )) => (
    {
      match $submac!($i, $($args)*) {
        $crate::IResult::Done(i,o)     => $crate::IResult::Done(i, ::std::option::Option::Some(o)),
        $crate::IResult::Error(_)      => $crate::IResult::Done($i, ::std::option::Option::None),
        $crate::IResult::Incomplete(i) => $crate::IResult::Incomplete(i)
      }
    }
  );
  ($i:expr, $f:expr) => (
    opt!($i, call!($f));
  );
);
