use crate::event::{AppEvent, Event, EventHandler};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};

/// Application.
#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub events: EventHandler,
    pub selected_tab: SelectedTab,
}

#[derive(Debug)]
pub enum SelectedTab {
    Tab1,
    Tab2,
    Tab3,
    Tab4,
    Tab5,
    Tab6,
}

impl std::fmt::Display for SelectedTab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            SelectedTab::Tab1 => "Intro",
            SelectedTab::Tab2 => "Required packages",
            SelectedTab::Tab3 => "GUI Programs",
            SelectedTab::Tab4 => "CLI Programs",
            SelectedTab::Tab5 => "Useless programs",
            SelectedTab::Tab6 => "Summary",
        };
        write!(f, "{}", value)
    }
}

impl SelectedTab {
    pub fn next(&self) -> SelectedTab {
        match self {
            SelectedTab::Tab1 => Self::Tab2,
            SelectedTab::Tab2 => Self::Tab3,
            SelectedTab::Tab3 => Self::Tab4,
            SelectedTab::Tab4 => Self::Tab5,
            SelectedTab::Tab5 => Self::Tab6,
            SelectedTab::Tab6 => Self::Tab6,
        }
    }

    pub fn previous(&self) -> SelectedTab {
        match self {
            SelectedTab::Tab1 => Self::Tab1,
            SelectedTab::Tab2 => Self::Tab1,
            SelectedTab::Tab3 => Self::Tab2,
            SelectedTab::Tab4 => Self::Tab3,
            SelectedTab::Tab5 => Self::Tab4,
            SelectedTab::Tab6 => Self::Tab5,
        }
    }

    pub fn index(&self) -> usize {
        match self {
            SelectedTab::Tab1 => 0,
            SelectedTab::Tab2 => 1,
            SelectedTab::Tab3 => 2,
            SelectedTab::Tab4 => 3,
            SelectedTab::Tab5 => 4,
            SelectedTab::Tab6 => 5,
        }
    }

    pub fn get_programs(&self) -> Vec<(&str, bool)> {
        // TODO - make this serializable or something

        match self {
            SelectedTab::Tab1 => vec![("", false)],
            SelectedTab::Tab2 => vec![("cargo", true), ("neovim", true), ("wget", true)],
            SelectedTab::Tab3 => vec![
                ("arduino-ide", false),
                ("discord", false),
                ("ghostty", false),
                ("spotify", false),
            ],
            SelectedTab::Tab4 => vec![
                ("btop", false),
                ("duf", false),
                ("dust", false),
                ("fastfetch", false),
            ],
            SelectedTab::Tab5 => vec![("asciiquarium", false)],
            SelectedTab::Tab6 => vec![("", false)],
        }
    }

    pub fn get_all_values() -> Vec<SelectedTab> {
        vec![
            SelectedTab::Tab1,
            SelectedTab::Tab2,
            SelectedTab::Tab3,
            SelectedTab::Tab4,
            SelectedTab::Tab5,
            SelectedTab::Tab6,
        ]
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            events: EventHandler::new(),
            selected_tab: SelectedTab::Tab1,
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event)
                        if key_event.kind == crossterm::event::KeyEventKind::Press =>
                    {
                        self.handle_key_events(key_event)?
                    }
                    _ => {}
                },
                Event::App(app_event) => match app_event {
                    AppEvent::Quit => self.quit(),
                    AppEvent::NextTab => self.selected_tab = self.selected_tab.next(),
                    AppEvent::PreviousTab => self.selected_tab = self.selected_tab.previous(),
                    AppEvent::TrySubmit => self.try_submit(),
                },
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }

            KeyCode::Right | KeyCode::Char('l' | 'L') => self.events.send(AppEvent::NextTab),
            KeyCode::Left | KeyCode::Char('h' | 'H') => self.events.send(AppEvent::PreviousTab),

            KeyCode::Enter => self.events.send(AppEvent::TrySubmit),
            _ => {}
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn try_submit(&mut self) {
        if let SelectedTab::Tab6 = self.selected_tab {
            self.events.send(AppEvent::Quit);
        }
    }
}
