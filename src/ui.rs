use color_eyre::owo_colors::OwoColorize;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, List, ListItem, Paragraph, Tabs, Widget},
};

use crate::app::{App, SelectedTab};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let main_layout = Layout::default()
            .constraints(vec![Constraint::Min(0), Constraint::Length(7)])
            .split(area);

        let selected_tab_index: usize = self.selected_tab.index();

        let tabs = Tabs::new(vec![
            "Title",
            "Required packages",
            "GUI programs",
            "CLI programs",
            "Useless programs",
            "Summary",
        ])
        .select(selected_tab_index);

        let block = Block::bordered().border_type(BorderType::Rounded);

        let inner = block.inner(main_layout[0]);

        block.render(main_layout[0], buf);

        let sub_layout = Layout::default()
            .constraints([Constraint::Length(1), Constraint::Min(0)])
            .split(inner);

        tabs.render(sub_layout[0], buf);
        self.render_main(
            Block::bordered().inner(sub_layout[1]),
            buf,
            &self.selected_tab,
        );

        self.render_help(main_layout[1], buf);
    }
}

impl App {
    fn render_help(&self, area: Rect, buf: &mut Buffer) {
        let paragraph = Paragraph::new(
            "Up ↑ Down ↓ or K ↑ J ↓ to choose package
Left ← Right → or H ← L → to change tab
Space ␣ to select/deselect package
Enter ↵ to submit (only on last tab)
Q to quit",
        )
        .block(Block::bordered().border_type(BorderType::Rounded));
        paragraph.render(area, buf);
    }

    fn render_main(&self, area: Rect, buf: &mut Buffer, selected_tab: &SelectedTab) {
        match selected_tab {
            SelectedTab::Tab1 => {
                self.render_first_tab(area, buf);
                return;
            }
            SelectedTab::Tab6 => {
                self.render_last_tab(area, buf);
                return;
            }
            _ => (),
        }

        let items: Vec<ListItem> = selected_tab
            .get_programs()
            .iter()
            .map(|program| {
                let (name, selected) = program;
                let symbol = if *selected { "[x]" } else { "[ ]" };
                ListItem::from(format!("{} {}", symbol, name))
            })
            .collect();

        let list = List::new(items);

        list.render(area, buf);
    }

    fn render_first_tab(&self, area: Rect, buf: &mut Buffer) {
        let main_layout =
            Layout::vertical(vec![Constraint::Length(3), Constraint::Min(0)]).split(area);

        let paragraph = Paragraph::new(
            "Hello!\nThis program will guide you through installing packages and stuff",
        );

        paragraph.render(main_layout[0], buf);
    }

    fn render_last_tab(&self, area: Rect, buf: &mut Buffer) {
        let main_layout =
            Layout::vertical(vec![Constraint::Length(1), Constraint::Min(0)]).split(area);

        let paragraph = Paragraph::new("These programs will be installed: ");

        let mut programs: Vec<String> = Vec::new();

        for t in SelectedTab::get_all_values() {
            for p in t.get_programs() {
                if p.1 {
                    programs.push(p.0.to_string());
                }
            }
        }

        let items: Vec<ListItem> = programs
            .iter()
            .map(|program| ListItem::from(program.to_string()))
            .collect();

        let list = List::new(items);

        paragraph.render(main_layout[0], buf);
        list.render(main_layout[1], buf);
    }
}
