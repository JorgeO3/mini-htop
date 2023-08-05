use std::{convert::Infallible, net::SocketAddr, sync::Arc, time::Duration};

use async_stream::try_stream;
use axum::{
    extract::State,
    response::sse::{Event, Sse},
    routing::get,
    Router,
};
use futures_util::stream::Stream;
use sysinfo::{CpuExt, System, SystemExt};
use tokio::{
    sync::watch::{self, Receiver, Sender},
    task::JoinHandle,
    time::{interval, sleep, Interval},
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
impl Broadcast {
    fn new(initial_message: impl Into<String>) -> Self {
        let (tx, rx) = watch::channel(initial_message.into());
        Self {
            tx: Arc::new(tx),
            rx: Arc::new(rx),
        }
    }
}

struct ChannelGuard(JoinHandle<()>);
impl Drop for ChannelGuard {
    fn drop(&mut self) {
        self.0.abort();
    }
}

#[tokio::main]
async fn main() {
    if let Err(e) = server(Broadcast::new("none")).await {
        eprintln!("{}", e)
    };
}

async fn server(state: Broadcast) -> Result<(), Box<dyn std::error::Error>> {
    let router = Router::new()
        .nest_service("/", ServeDir::new("statics"))
        .route("/sse", get(sse_handler))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .map_err(Into::into)
}

async fn monitor_cpu(tx: SharedSender, mut interval: Interval) {
    let mut sys = System::new();
    sys.refresh_cpu();
    sleep(Duration::from_millis(200)).await;

    loop {
        sys.refresh_cpu();
        let cpu_info = sys
            .cpus()
            .iter()
            .enumerate()
            .map(|(i, cpu)| format!("Cpu {}: {:.2} \n", i, cpu.cpu_usage()))
            .collect::<String>();

        if let Err(e) = tx.send(cpu_info) {
            println!("{}", e);
            break;
        }

        interval.tick().await;
    }
}

async fn sse_handler(
    State(state): State<Broadcast>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state.rx;
    let tx = state.tx;

    let monitor_cpu_handle = tokio::spawn(async move {
        monitor_cpu(tx, interval(Duration::from_millis(500))).await;
    });

    let stream = try_stream! {
        let mut interval = interval(Duration::from_millis(1000));
        let _guard = ChannelGuard(monitor_cpu_handle);

        loop {
            let message = rx.borrow().to_string();
            yield Event::default().data(message);
            interval.tick().await;
        }
    };
    Sse::new(stream)
}
