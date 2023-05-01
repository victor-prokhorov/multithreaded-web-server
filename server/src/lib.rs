mod router;

use parallel_task_executor::ThreadPool;
use router::Router;
use std::{
    fmt,
    io::prelude::*,
    net::{TcpListener, TcpStream, ToSocketAddrs},
};
use tracing::{error, info, instrument, trace};

pub struct Server<'server, A> {
    addr: &'server A,
    thread_pool_size: usize,
}

impl<'server, A> Server<'server, A>
where
    A: ToSocketAddrs + fmt::Debug,
{
    pub fn new(addr: &'server A, thread_pool_size: usize) -> Self {
        Self {
            addr,
            thread_pool_size,
        }
    }

    pub fn run(&self) {
        let pool = ThreadPool::new(self.thread_pool_size);
        let listener = TcpListener::bind(self.addr).unwrap();
        info!(addr = ?self.addr, "listening");
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            trace!("connection established");
            pool.execute(|| {
                handle_connection(stream);
            });
        }
    }
}

#[instrument(skip(stream))]
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();
    trace!(bytes_read = bytes_read);
    if bytes_read > buffer.len() {
        error!("`bytes_read` is larger than `buffer`");
    }
    let req = String::from_utf8(buffer[..bytes_read].to_vec())
        .unwrap()
        .into();
    Router::route(req, &mut stream);
}
