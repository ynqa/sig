use std::io::{self, Write};

use promkit::{
    crossterm::{self, cursor, style, terminal},
    grapheme::StyledGraphemes,
    pane::Pane,
};

pub struct Terminal {
    anchor_position: (u16, u16),
}

impl Terminal {
    pub fn new(pane: &Pane) -> anyhow::Result<Self> {
        let mut offset_from_bottom = terminal::size()?;
        offset_from_bottom.1 = offset_from_bottom
            .1
            .saturating_sub(1 + pane.visible_row_count() as u16);

        Ok(Self {
            anchor_position: (0, offset_from_bottom.1),
        })
    }

    pub fn draw_stream_and_pane(
        &self,
        items: Vec<StyledGraphemes>,
        pane: &Pane,
    ) -> anyhow::Result<()> {
        let coefficient = items.len().saturating_sub(1) as u16;
        crossterm::queue!(
            io::stdout(),
            cursor::MoveTo(
                self.anchor_position.0,
                self.anchor_position.1.saturating_sub(coefficient)
            ),
            terminal::ScrollUp(1 + coefficient),
            terminal::Clear(terminal::ClearType::FromCursorDown),
        )?;

        for item in items.iter() {
            crossterm::queue!(
                io::stdout(),
                style::Print(item.styled_display()),
                cursor::MoveToNextLine(1)
            )?;
        }

        io::stdout().flush()?;
        self.draw(pane)
    }

    pub fn draw_pane(&mut self, pane: &Pane) -> anyhow::Result<()> {
        let size = terminal::size()?;
        crossterm::queue!(
            io::stdout(),
            cursor::MoveTo(self.anchor_position.0, self.anchor_position.1 + 1),
            terminal::Clear(terminal::ClearType::FromCursorDown),
        )?;
        self.anchor_position.1 = size.1.saturating_sub(1 + pane.visible_row_count() as u16);
        self.draw(pane)
    }

    fn draw(&self, pane: &Pane) -> anyhow::Result<()> {
        crossterm::queue!(
            io::stdout(),
            cursor::MoveTo(self.anchor_position.0, self.anchor_position.1 + 1),
            terminal::Clear(terminal::ClearType::FromCursorDown),
        )?;

        for row in pane.extract(pane.visible_row_count()) {
            crossterm::queue!(io::stdout(), style::Print(row.styled_display()))?;
        }

        io::stdout().flush()?;
        Ok(())
    }
}
