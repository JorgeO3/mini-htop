use std::{convert::Infallible, net::SocketAddr, sync::Arc};

use async_stream::try_stream;
use axum::{
    extract::State,
    response::{sse::Event, Sse},
    routing::get,
    Router,
};
use futures_util::stream::Stream;
use sysinfo::{CpuExt, System, SystemExt};
use tokio::{
    sync::watch::{Receiver, Sender},
    task::JoinHandle,
};
use tower_http::services::ServeDir;

type SharedReceiver = Arc<Receiver<String>>;
type SharedSender = Arc<Sender<String>>;

#[allow(unused)]
#[derive(Debug, Clone)]
struct Broadcast {
    tx: SharedSender,
    rx: SharedReceiver,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = tokio::sync::watch::channel(String::from("initial"));
    let tx = Arc::new(tx);
    let rx = Arc::new(rx);

    sever(Broadcast { tx, rx }).await;
    Ok(())
}

async fn sever(state: Broadcast) {
    let router = Router::new()
        .nest_service("/", ServeDir::new("statics"))
        .route("/sse", get(sse_handler))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn cpu_monitor(tx: SharedSender) {
    let mut sys = System::new();
    sys.refresh_cpu();
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    loop {
        sys.refresh_cpu();
        let cpu_info = sys
            .cpus()
            .iter()
            .enumerate()
            .map(|(i, cpu)| format!("Cpu {}: {:.2} \n", i, cpu.cpu_usage()))
            .collect::<String>();

        if let Err(e) = tx.send(cpu_info) {
            eprintln!("{}", e);
            break;
        }

        println!("dentro del cpu");
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }
}

struct ChannelGuard(JoinHandle<()>);
impl Drop for ChannelGuard {
    fn drop(&mut self) {
        self.0.abort();
    }
}

async fn sse_handler(
    State(state): State<Broadcast>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state.rx;
    let tx = state.tx;

    let handle = tokio::spawn(async move {
        cpu_monitor(tx).await;
    });

    let stream = try_stream! {
        let mut interval = tokio::time::interval(std::time::Duration::from_millis(1000));
        let _guard = ChannelGuard(handle);

        loop {
            let cpu_info = rx.borrow().to_string();
            yield Event::default().data(cpu_info);
            interval.tick().await;
        }
    };
    Sse::new(stream)
}
