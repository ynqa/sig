use std::{collections::VecDeque, io};

use clap::Parser;
use tokio::{
    sync::mpsc,
    time::{timeout, Duration},
};
use tokio_util::sync::CancellationToken;

use promkit::{
    crossterm::{
        self, cursor, execute,
        style::Color,
        terminal::{disable_raw_mode, enable_raw_mode},
    },
    listbox,
    style::StyleBuilder,
    text_editor,
};

mod archived;
mod cmd;
mod sig;
mod stdin;
mod terminal;

#[derive(Eq, PartialEq)]
pub enum Signal {
    Continue,
    GotoArchived,
    GotoStreaming,
}

/// Interactive grep (for streaming)
#[derive(Parser)]
#[command(name = "sig", version)]
#[command(
    name = "sig",
    version,
    help_template = "
{about}

Usage: {usage}

Examples:

$ stern --context kind-kind etcd |& sig
Or the method to retry command by pressing ctrl+r:
$ sig --cmd \"stern --context kind-kind etcd\"

Archived mode:
$ cat README.md |& sig -a
Or
$ sig -a --cmd \"cat README.md\"

Options:
{options}
"
)]
pub struct Args {
    #[arg(
        long = "retrieval-timeout",
        default_value = "10",
        help = "Timeout to read a next line from the stream in milliseconds."
    )]
    pub retrieval_timeout_millis: u64,

    #[arg(
        long = "render-interval",
        default_value = "10",
        help = "Interval to render a line in milliseconds.",
        long_help = "Adjust this value to prevent screen flickering
        when a large volume of lines is rendered in a short period."
    )]
    pub render_interval_millis: u64,

    #[arg(
        short = 'q',
        long = "queue-capacity",
        default_value = "1000",
        help = "Queue capacity to store lines.",
        long_help = "Queue capacity for storing lines.
        This value is used for temporary storage of lines
        and should be adjusted based on the system's memory capacity.
        Increasing this value allows for more lines to be stored temporarily,
        which can be beneficial when digging deeper into lines with the digger."
    )]
    pub queue_capacity: usize,

    #[arg(
        short = 'a',
        long = "archived",
        default_value = "false",
        help = "Archived mode to grep through static data."
    )]
    pub archived: bool,

    #[arg(
        short = 'i',
        long = "ignore-case",
        default_value = "false",
        help = "Case insensitive search."
    )]
    pub case_insensitive: bool,

    #[arg(
        long = "cmd",
        help = "Command to execute on initial and retries.",
        long_help = "This command is invoked initially and
        whenever a retry is triggered according to key mappings."
    )]
    pub cmd: Option<String>,
}

impl Drop for Args {
    fn drop(&mut self) {
        disable_raw_mode().ok();
        execute!(io::stdout(), cursor::Show).ok();
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    enable_raw_mode()?;
    execute!(io::stdout(), cursor::Hide)?;

    let highlight_style = StyleBuilder::new().fgc(Color::Red).build();

    if args.archived {
        let (tx, mut rx) = mpsc::channel(1);

        if let Some(cmd) = args.cmd.clone() {
            tokio::spawn(async move {
                cmd::execute(
                    &cmd,
                    tx,
                    Duration::from_millis(args.retrieval_timeout_millis),
                    CancellationToken::new(),
                )
                .await
            });
        } else {
            tokio::spawn(async move {
                stdin::streaming(
                    tx,
                    Duration::from_millis(args.retrieval_timeout_millis),
                    CancellationToken::new(),
                )
                .await
            });
        }

        let mut queue = VecDeque::with_capacity(args.queue_capacity);
        loop {
            match timeout(
                Duration::from_millis(args.retrieval_timeout_millis),
                rx.recv(),
            )
            .await
            {
                Ok(Some(line)) => {
                    if queue.len() > args.queue_capacity {
                        queue.pop_front().unwrap();
                    }
                    queue.push_back(line.clone());
                }
                Ok(None) => break,
                Err(_) => break,
            }
        }

        crossterm::execute!(
            io::stdout(),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::Purge),
            cursor::MoveTo(0, 0),
        )?;

        archived::run(
            text_editor::State {
                texteditor: Default::default(),
                history: Default::default(),
                prefix: String::from("❯❯❯ "),
                mask: Default::default(),
                prefix_style: StyleBuilder::new().fgc(Color::DarkBlue).build(),
                active_char_style: StyleBuilder::new().bgc(Color::DarkCyan).build(),
                inactive_char_style: StyleBuilder::new().build(),
                edit_mode: Default::default(),
                word_break_chars: Default::default(),
                lines: Default::default(),
            },
            listbox::State {
                listbox: listbox::Listbox::from_displayable(queue),
                cursor: String::from("❯ "),
                active_item_style: None,
                inactive_item_style: None,
                lines: Default::default(),
            },
            highlight_style,
            args.case_insensitive,
            // In archived mode, command for retry is meaningless.
            None,
        )?;
    } else {
        while let Ok((signal, queue)) = sig::run(
            text_editor::State {
                texteditor: Default::default(),
                history: Default::default(),
                prefix: String::from("❯❯ "),
                mask: Default::default(),
                prefix_style: StyleBuilder::new().fgc(Color::DarkGreen).build(),
                active_char_style: StyleBuilder::new().bgc(Color::DarkCyan).build(),
                inactive_char_style: StyleBuilder::new().build(),
                edit_mode: Default::default(),
                word_break_chars: Default::default(),
                lines: Default::default(),
            },
            highlight_style,
            Duration::from_millis(args.retrieval_timeout_millis),
            Duration::from_millis(args.render_interval_millis),
            args.queue_capacity,
            args.case_insensitive,
            args.cmd.clone(),
        )
        .await
        {
            crossterm::execute!(
                io::stdout(),
                crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
                crossterm::terminal::Clear(crossterm::terminal::ClearType::Purge),
                cursor::MoveTo(0, 0),
            )?;

            match signal {
                Signal::GotoArchived => {
                    archived::run(
                        text_editor::State {
                            texteditor: Default::default(),
                            history: Default::default(),
                            prefix: String::from("❯❯❯ "),
                            mask: Default::default(),
                            prefix_style: StyleBuilder::new().fgc(Color::DarkBlue).build(),
                            active_char_style: StyleBuilder::new().bgc(Color::DarkCyan).build(),
                            inactive_char_style: StyleBuilder::new().build(),
                            edit_mode: Default::default(),
                            word_break_chars: Default::default(),
                            lines: Default::default(),
                        },
                        listbox::State {
                            listbox: listbox::Listbox::from_displayable(queue),
                            cursor: String::from("❯ "),
                            active_item_style: None,
                            inactive_item_style: None,
                            lines: Default::default(),
                        },
                        highlight_style,
                        args.case_insensitive,
                        args.cmd.clone(),
                    )?;

                    // Re-enable raw mode and hide the cursor again here
                    // because they are disabled and shown, respectively, by promkit.
                    enable_raw_mode()?;
                    execute!(io::stdout(), cursor::Hide)?;

                    crossterm::execute!(
                        io::stdout(),
                        crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
                        crossterm::terminal::Clear(crossterm::terminal::ClearType::Purge),
                        cursor::MoveTo(0, 0),
                    )?;
                }
                Signal::GotoStreaming => {
                    continue;
                }
                _ => {}
            }
        }
    }

    Ok(())
}
