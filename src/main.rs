// #![allow(unused)]

use std::{convert::Infallible, net::SocketAddr, sync::Arc};

use async_stream::try_stream;
use axum::{
    extract::State,
    response::{sse::Event, Sse},
    routing::get,
    Router,
};
use futures_util::{stream::Stream, StreamExt};
use sysinfo::{CpuExt, System, SystemExt};
use tokio::sync::watch::{Receiver, Sender};
use tokio_stream::wrappers::UnboundedReceiverStream;
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

        if tx.is_closed() {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }
}

async fn sse_handler(
    State(state): State<Broadcast>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state.rx;
    let tx = state.tx;

    tokio::spawn(async move {
        cpu_monitor(tx).await;
    });

    let (shutdown_sender, shutdown_receiver) = tokio::sync::mpsc::unbounded_channel::<()>();
    let shutdown_receiver = UnboundedReceiverStream::new(shutdown_receiver);

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        shutdown_sender.send(()).unwrap();
    });

    let mut shutdown_received = false;

    let stream = try_stream! {
        tokio::pin!(shutdown_receiver);

        loop {
            if shutdown_received {
                break;
            }

            tokio::select! {
                _ = shutdown_receiver.next() => {
                    println!("Received shutdown signal");
                    shutdown_received = true;
                    yield Event::default().event("shutdown");
                }
                _ = tokio::time::sleep(std::time::Duration::from_secs(1)) => {
                    let val = rx.borrow().clone();
                    yield Event::default().data(val);
                }
            }
        }
    };

    Sse::new(stream)
}
