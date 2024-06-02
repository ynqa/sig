use std::process::Stdio;

use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
    sync::mpsc,
    time::{timeout, Duration},
};
use tokio_util::sync::CancellationToken;

pub async fn execute(
    cmdstr: &str,
    tx: mpsc::Sender<String>,
    retrieval_timeout: Duration,
    canceled: CancellationToken,
) -> anyhow::Result<()> {
    let args: Vec<&str> = cmdstr.split_whitespace().collect();
    let mut child = Command::new(args[0])
        .args(&args[1..])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| anyhow::anyhow!("stdout is not available"))?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| anyhow::anyhow!("stderr is not available"))?;
    let mut stdout_reader = BufReader::new(stdout).lines();
    let mut stderr_reader = BufReader::new(stderr).lines();

    while !canceled.is_cancelled() {
        tokio::select! {
            stdout_res = timeout(retrieval_timeout, stdout_reader.next_line()) => {
                if let Ok(Ok(Some(line))) = stdout_res {
                    let escaped = strip_ansi_escapes::strip_str(line.replace(['\n', '\t'], " "));
                    tx.send(escaped).await?;
                }
            },
            stderr_res = timeout(retrieval_timeout, stderr_reader.next_line()) => {
                if let Ok(Ok(Some(line))) = stderr_res {
                    let escaped = strip_ansi_escapes::strip_str(line.replace(['\n', '\t'], " "));
                    tx.send(escaped).await?;
                }
            }
        }
    }

    child.kill().await?;
    Ok(())
}
