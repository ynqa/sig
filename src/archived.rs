use rayon::prelude::*;

use promkit::{
    crossterm::{event::Event, style::ContentStyle},
    grapheme::StyledGraphemes,
    listbox,
    pane::Pane,
    snapshot::Snapshot,
    switch::ActiveKeySwitcher,
    text_editor, PaneFactory, Prompt, PromptSignal,
};

use crate::sig;

mod keymap;

struct Archived {
    keymap: ActiveKeySwitcher<keymap::Keymap>,
    text_editor_snapshot: Snapshot<text_editor::State>,
    lines: Snapshot<listbox::State>,
    highlight_style: ContentStyle,
    case_insensitive: bool,
    cmd: Option<String>,
}

impl promkit::Finalizer for Archived {
    type Return = ();

    fn finalize(&self) -> anyhow::Result<Self::Return> {
        Ok(())
    }
}

impl promkit::Renderer for Archived {
    fn create_panes(&self, width: u16, height: u16) -> Vec<Pane> {
        vec![
            self.lines.create_pane(width, height),
            self.text_editor_snapshot.create_pane(width, height),
        ]
    }

    fn evaluate(&mut self, event: &Event) -> anyhow::Result<PromptSignal> {
        let signal = self.keymap.get()(
            event,
            &mut self.text_editor_snapshot,
            &mut self.lines,
            self.cmd.clone(),
        );
        if self
            .text_editor_snapshot
            .after()
            .texteditor
            .text_without_cursor()
            != self
                .text_editor_snapshot
                .borrow_before()
                .texteditor
                .text_without_cursor()
        {
            let query = self
                .text_editor_snapshot
                .after()
                .texteditor
                .text_without_cursor()
                .to_string();

            let list: Vec<StyledGraphemes> = self
                .lines
                .init()
                .listbox
                .items()
                .par_iter()
                .filter_map(|line| {
                    sig::styled(
                        &query,
                        &line.to_string(),
                        self.highlight_style,
                        self.case_insensitive,
                    )
                })
                .collect();

            self.lines.after_mut().listbox = listbox::Listbox::from_iter(list);
        }
        signal
    }
}

pub fn run(
    text_editor: text_editor::State,
    lines: listbox::State,
    highlight_style: ContentStyle,
    case_insensitive: bool,
    cmd: Option<String>,
) -> anyhow::Result<()> {
    Prompt {
        renderer: Archived {
            keymap: ActiveKeySwitcher::new("default", keymap::default),
            text_editor_snapshot: Snapshot::new(text_editor),
            lines: Snapshot::new(lines),
            highlight_style,
            case_insensitive,
            cmd,
        },
    }
    .run()
}
