// #![allow(unused)]

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
use tokio::sync::{
    watch::{Receiver, Sender},
    Mutex,
};
use tower_http::services::ServeDir;

type SharedReceiver = Arc<Mutex<Receiver<String>>>;
type SharedSender = Arc<Mutex<Sender<String>>>;

#[allow(unused)]
#[derive(Debug, Clone)]
struct Broadcast {
    tx: SharedSender,
    rx: SharedReceiver,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = tokio::sync::watch::channel(String::from("initial"));
    let tx = Arc::new(Mutex::new(tx));
    let rx = Arc::new(Mutex::new(rx));

    let api_state = Broadcast { tx, rx };
    sever(api_state).await
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

        tx.lock().await.send(cpu_info).unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }
}

async fn sever(state: Broadcast) -> Result<(), Box<dyn std::error::Error>> {
    let router = Router::new()
        .nest_service("/", ServeDir::new("statics"))
        .route("/sse", get(sse_handler))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}

async fn sse_handler(
    State(state): State<Broadcast>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state.rx;
    let tx = state.tx;

    let _cpu_monitor_handle = tokio::spawn(async move {
        cpu_monitor(tx).await;
    });

    let stream = try_stream! {
        loop {
            let receiver = rx.lock().await;
            let value = receiver.borrow().to_string();
            yield Event::default().data(value);
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    };
    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::default())
}
