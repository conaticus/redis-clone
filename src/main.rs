#[macro_use]
extern crate lazy_static;

mod commands;
mod resp;
mod tcp;
mod util;

use std::io;

use tcp::listen;

#[tokio::main]
async fn main() -> io::Result<()> {
    listen(6379).await
}
