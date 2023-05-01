use argh::FromArgs;
use server::Server;
use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;

fn default_server_addr() -> SocketAddr {
    SocketAddr::from(([127, 0, 0, 1], 3000))
}

fn default_server_thread_pool_size() -> usize {
    4
}

fn default_tracing_level() -> String {
    "info".to_owned()
}

#[derive(FromArgs)]
#[argh(description = "multithreaded web server")]
pub struct Args {
    #[argh(
        option,
        default = "default_server_addr()",
        description = "server address"
    )]
    server_addr: SocketAddr,
    #[argh(
        option,
        default = "default_server_thread_pool_size()",
        description = "server thread pool size"
    )]
    server_thread_pool_size: usize,
    #[argh(
        option,
        default = "default_tracing_level()",
        description = "tracing level"
    )]
    tracing_level: String,
}

fn main() {
    let args: Args = argh::from_env();
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive(
                    format!("{}={}", module_path!(), args.tracing_level)
                        .parse()
                        .unwrap(),
                )
                .add_directive(format!("server={}", args.tracing_level).parse().unwrap())
                .add_directive(
                    format!("parallel_task_executor={}", args.tracing_level)
                        .parse()
                        .unwrap(),
                )
                .add_directive(format!("http={}", args.tracing_level).parse().unwrap()),
        )
        .init();
    let server = Server::new(&args.server_addr, args.server_thread_pool_size);
    server.run();
}
