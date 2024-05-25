use std::{collections::VecDeque, sync::Arc};

use grep::{
    matcher::{Match, Matcher},
    regex::RegexMatcherBuilder,
};
use tokio::{
    sync::{mpsc, RwLock},
    task::JoinHandle,
    time::{self, Duration},
};
use tokio_util::sync::CancellationToken;

use promkit::{
    crossterm::{self, event, style::ContentStyle},
    grapheme::StyledGraphemes,
    switch::ActiveKeySwitcher,
    text_editor, PaneFactory, PromptSignal,
};

mod keymap;
use crate::{stdin, terminal::Terminal};

fn matched(queries: &[&str], line: &str) -> anyhow::Result<Vec<Match>> {
    let mut matched = Vec::new();
    RegexMatcherBuilder::new()
        .build_many(queries)?
        .find_iter_at(line.as_bytes(), 0, |m| {
            if m.start() >= line.as_bytes().len() {
                return false;
            }
            matched.push(m);
            true
        })?;
    Ok(matched)
}

pub fn styled(query: &str, line: &str, highlight_style: ContentStyle) -> Option<StyledGraphemes> {
    let piped = &query
        .split('|')
        .filter(|s| !s.trim().is_empty())
        .collect::<Vec<&str>>();

    let mut styled = StyledGraphemes::from(line);

    if query.is_empty() {
        Some(styled)
    } else {
        match matched(piped, line) {
            Ok(matches) => {
                if matches.is_empty() {
                    None
                } else {
                    for m in matches {
                        for i in m.start()..m.end() {
                            styled = styled.apply_style_at(i, highlight_style);
                        }
                    }
                    Some(styled)
                }
            }
            _ => None,
        }
    }
}

pub async fn run(
    text_editor: text_editor::State,
    highlight_style: ContentStyle,
    retrieval_timeout: Duration,
    render_interval: Duration,
    queue_capacity: usize,
) -> anyhow::Result<VecDeque<String>> {
    let keymap = ActiveKeySwitcher::new("default", keymap::default);
    let size = crossterm::terminal::size()?;

    let pane = text_editor.create_pane(size.0, size.1);
    let mut term = Terminal::new(&pane)?;
    term.draw_pane(&pane)?;

    let shared_term = Arc::new(RwLock::new(term));
    let shared_text_editor = Arc::new(RwLock::new(text_editor));
    let readonly_term = Arc::clone(&shared_term);
    let readonly_text_editor = Arc::clone(&shared_text_editor);

    let (tx, mut rx) = mpsc::channel(1);
    let canceler = CancellationToken::new();

    let canceled = canceler.clone();
    let streaming =
        tokio::spawn(async move { stdin::streaming(tx, retrieval_timeout, canceled).await });

    let keeping: JoinHandle<anyhow::Result<VecDeque<String>>> = tokio::spawn(async move {
        let mut queue = VecDeque::with_capacity(queue_capacity);
        let interval = time::interval(render_interval);
        futures::pin_mut!(interval);

        loop {
            interval.tick().await;
            match rx.recv().await {
                Some(line) => {
                    let text_editor = readonly_text_editor.read().await;
                    let size = crossterm::terminal::size()?;

                    if queue.len() > queue_capacity {
                        queue.pop_front().unwrap();
                    }
                    queue.push_back(line.clone());

                    if let Some(styled) = styled(
                        &text_editor.texteditor.text_without_cursor().to_string(),
                        &line,
                        highlight_style,
                    ) {
                        let matrix = styled.matrixify(size.0 as usize, size.1 as usize, 0).0;
                        let term = readonly_term.read().await;
                        term.draw_stream_and_pane(
                            matrix,
                            &text_editor.create_pane(size.0, size.1),
                        )?;
                    }
                }
                None => break,
            }
        }
        Ok(queue)
    });

    loop {
        let event = event::read()?;
        let mut text_editor = shared_text_editor.write().await;
        let signal = keymap.get()(&event, &mut text_editor)?;
        if signal == PromptSignal::Quit {
            break;
        }

        let size = crossterm::terminal::size()?;
        let pane = text_editor.create_pane(size.0, size.1);
        let mut term = shared_term.write().await;
        term.draw_pane(&pane)?;
    }

    canceler.cancel();
    let _: anyhow::Result<(), anyhow::Error> = streaming.await?;

    keeping.await?
}
