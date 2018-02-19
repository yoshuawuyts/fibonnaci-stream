extern crate fibonnaci_stream;
extern crate futures;

use fibonnaci_stream::{FibError, FibStream};
use futures::{future, Stream};

fn main() {
  FibStream::new().for_each(|n| {
    println!("{}", n);
    Ok(());
  });
}
