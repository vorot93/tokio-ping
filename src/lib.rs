//! tokio-ping is an asynchronous ICMP pinging library.
//!
//! The repository is located at <https://github.com/knsd/tokio-ping/>.
//!
//! # Usage example
//!
//! Note, sending and receiving ICMP packets requires privileges.
//!
//! ```rust,no_run
//! #![feature(async_await)]
//!
//! use futures::{compat::*, prelude::*};
//! use futures01::{Future, Stream};
//!
//! fn main() {
//!     tokio::run(async {
//!         let addr = std::env::args().nth(1).unwrap().parse().unwrap();
//!
//!         let pinger = tokio_ping::Pinger::new().await?;
//!         let mut stream = pinger.chain(addr).stream();
//!
//!         for _ in 0..3 {
//!             if let Some(mb_time) = stream.try_next().await? {
//!                 match mb_time {
//!                     Some(time) => println!("time={}", time),
//!                     None => println!("timeout"),
//!                 }
//!             }
//!         }
//!         Ok(())
//!     }.map_err(|err: failure::Error| {
//!         eprintln!("Error: {}", err)
//!     }).boxed().compat())
//! }
//! ```
#![feature(async_await)]

mod errors;
mod packet;
mod ping;
mod socket;

pub use self::errors::Error;
pub use self::ping::{PingChain, PingChainStream, PingFuture, Pinger};
