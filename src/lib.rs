//! A lazily evaluated [futures stream](https://docs.rs/futures/*/futures/stream/trait.Stream.html)
//! of Fibonnaci numbers.
//!
//! ## Usage
//!
//! ```rust,ignore
//! extern crate fibonnaci_stream;
//!
//! use fibonnaci_stream::{FibError, Stream};
//!
//! fn main () {
//!   let mut stream = FibStream::new();
//!   stream.poll().and_then(|val| {
//!     assert_eq!(val, Async::Ready(Some(1)));
//!
//!     stream.poll().and_then(|val| {
//!       assert_eq!(val, Async::Ready(Some(2)));
//!       Ok(())
//!     });
//!
//!     Ok(())
//!   });
//! }
//! ```

extern crate futures;

pub use futures::Stream;
use futures::{Async, Future, Poll};
use std::error::Error;

/// A lazily evaluated [futures stream] of Fibonnaci numbers.
///
/// ## Example
/// ```rust
/// extern crate fibonnaci_stream;
/// extern crate futures;
///
/// use futures::{Async, Stream}; // Stream must be in scope here for trait to work
/// use fibonnaci_stream::FibStream;
///
/// fn main () {
///   let mut stream = FibStream::new();
///   stream.poll().and_then(|val| {
///     assert_eq!(val, Async::Ready(Some(1)));
///     stream.poll().and_then(|val| {
///       assert_eq!(val, Async::Ready(Some(2)));
///       Ok(())
///     });
///     Ok(())
///   });
/// }
/// ```
/// [futures stream]: (https://docs.rs/futures/*/futures/stream/trait.Stream.html)
#[derive(Debug)]
pub struct FibStream {
  prev: u64,
  num: u64,
}

impl FibStream {
  /// Creates a new Fibonnaci stream.
  ///
  /// This method initializes the stream to start at 1 and continues the
  /// sequence from there.
  pub fn new() -> FibStream {
    FibStream { prev: 0, num: 1 }
  }
}

impl Stream for FibStream {
  type Item = u64;
  type Error = Box<Error>;

  fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
    let res = self.prev + self.num;
    self.prev = self.num;
    self.num = res;
    Ok(Async::Ready(Some(res)))
  }
}

impl Future for FibStream {
  type Item = u64;
  type Error = Box<Error>;

  fn poll(&mut self) -> Result<Async<u64>, Box<Error>> {
    let res = self.prev + self.num;
    self.prev = self.num;
    self.num = res;
    Ok(Async::Ready(res))
  }
}
