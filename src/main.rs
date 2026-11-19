use crate::config::load::{ConfigInterface, ImplConfigInterface};
use crate::handlers::service::endpoints;
use custom_logger as log;
use hyper::service::service_fn;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto::Builder;
use mimalloc::MiMalloc;
use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;

mod config;
mod handlers;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    // read and parse config
    // for documents 'config/application-config.json'
    let impl_config = ImplConfigInterface {};
    let res_params = impl_config.read("config/application-config.json".to_string());
    let parameters = match res_params {
        Ok(params) => params,
        Err(e) => {
            eprintln!("{:?}", e.to_string());
            std::process::exit(1);
        }
    };

    let level = match parameters.log_level.as_str() {
        "warn" => log::LevelFilter::Warn,
        "debug" => log::LevelFilter::Debug,
        "trace" => log::LevelFilter::Trace,
        &_ => log::LevelFilter::Trace,
    };

    // setup logging
    if let Err(e) = log::Logging::new().with_level(level).init() {
        // log is broken so use eprintln!
        eprintln!("[main] error {}", e);
        std::process::exit(1);
    }

    log::info!("application : {}", env!("CARGO_PKG_NAME"));
    log::info!("author      : {}", env!("CARGO_PKG_AUTHORS"));
    log::info!("version     : {}", env!("CARGO_PKG_VERSION"));
    log::info!("port        : {}", parameters.server_port);

    let result = run_server(parameters.server_port);
    match result {
        Ok(_) => log::info!("[main] server shutdown successfully"),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}

#[tokio::main]
pub async fn run_server(port: usize) -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::new(Ipv4Addr::new(0, 0, 0, 0).into(), port as u16);
    log::info!("[run_server] starting to serve on http://{}", addr);
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        tokio::task::spawn(async move {
            if let Err(err) = Builder::new(TokioExecutor::new())
                .serve_connection(io, service_fn(endpoints))
                .await
            {
                log::error!("[run_server] error serving connection: {:?}", err);
            }
        });
    }
}
