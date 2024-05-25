use tokio::{
    io::{self, AsyncBufReadExt, BufReader},
    sync::mpsc,
    time::{timeout, Duration},
};

use tokio_util::sync::CancellationToken;

pub async fn streaming(
    tx: mpsc::Sender<String>,
    retrieval_timeout: Duration,
    canceled: CancellationToken,
) -> Result<(), anyhow::Error> {
    let mut reader = BufReader::new(io::stdin()).lines();

    while !canceled.is_cancelled() {
        // Set a timeout to ensure non-blocking behavior,
        // especially responsive to user inputs like ctrl+c.
        // Continuously retry until cancellation to prevent loss of logs.
        let ret = timeout(retrieval_timeout, reader.next_line()).await;
        if ret.is_err() {
            continue;
        }

        let ret = ret?;

        match ret {
            Ok(Some(line)) => {
                let escaped = strip_ansi_escapes::strip_str(line.replace(['\n', '\t'], " "));
                tx.send(escaped).await?;
            }
            _ => break,
        }
    }
    Ok(())
}
