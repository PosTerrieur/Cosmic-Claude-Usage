use std::time::Duration;

use cosmic::app::{Core, Task};
use cosmic::iced::{Alignment, Length, Subscription};
use cosmic::widget::{autosize, button, Id};
use cosmic::{Element, surface};
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

const ID: &str = "com.github.user.ClaudeUsage";

static AUTOSIZE_MAIN_ID: LazyLock<Id> = LazyLock::new(|| Id::new("autosize-main"));

fn stats_path() -> std::path::PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
        .join("claude-usage")
        .join("stats.json")
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UsageStats {
    pub session_pct:   f64,
    pub hebdo_pct:     f64,
    pub reset_session: String,
    pub reset_hebdo:   String,
}

pub struct Window {
    core:  Core,
    stats: UsageStats,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    Surface(surface::Action),
}

impl cosmic::Application for Window {
    type Executor = cosmic::SingleThreadExecutor;
    type Flags   = ();
    type Message = Message;
    const APP_ID: &'static str = ID;

    fn core(&self)         -> &Core     { &self.core }
    fn core_mut(&mut self) -> &mut Core { &mut self.core }

    fn init(core: Core, _flags: ()) -> (Self, Task<Message>) {
        let mut app = Self { core, stats: UsageStats::default() };
        app.refresh_stats();
        (app, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => self.refresh_stats(),
            Message::Surface(a) => {
                return cosmic::task::message(cosmic::Action::Cosmic(
                    cosmic::app::Action::Surface(a),
                ));
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let label = format!(
            "Hebdo:{:.0}% Sess:{:.0}%",
            self.stats.hebdo_pct,
            self.stats.session_pct,
        );

        let tooltip = format!(
            "Reset hebdo   : {}\nReset session : {}",
            format_reset(&self.stats.reset_hebdo),
            format_reset(&self.stats.reset_session),
        );

        let content = cosmic::widget::row!(
            self.core.applet.text(label),
            cosmic::widget::space::vertical().height(Length::Fixed(
                (self.core.applet.suggested_size(true).1
                    + 2 * self.core.applet.suggested_padding(true).1)
                    as f32,
            ))
        )
        .align_y(Alignment::Center);

        let btn = button::custom(content)
            .padding([0, self.core.applet.suggested_padding(true).0])
            .class(cosmic::theme::Button::AppletIcon);

        let sized = autosize::autosize(btn, AUTOSIZE_MAIN_ID.clone());

        Element::from(self.core.applet.applet_tooltip::<Message>(
            sized,
            tooltip,
            false,
            |a| Message::Surface(a),
            None,
        ))
    }

    fn subscription(&self) -> Subscription<Message> {
        cosmic::iced::time::every(Duration::from_secs(30))
            .map(|_| Message::Tick)
    }

    fn style(&self) -> Option<cosmic::iced::theme::Style> {
        Some(cosmic::applet::style())
    }
}

impl Window {
    fn refresh_stats(&mut self) {
        if let Ok(content) = std::fs::read_to_string(stats_path()) {
            if let Ok(stats) = serde_json::from_str::<UsageStats>(&content) {
                self.stats = stats;
            }
        }
    }
}

/// Formate un timestamp ISO en heure locale lisible.
/// "2026-04-29T09:00:00+00:00" → "29/04 à 11:00"
fn format_reset(ts: &str) -> String {
    use chrono::{DateTime, Local};

    if ts.is_empty() {
        return "inconnu".to_string();
    }

    match DateTime::parse_from_rfc3339(ts) {
        Ok(dt) => {
            let local: chrono::DateTime<Local> = dt.into();
            local.format("%d/%m à %Hh%M").to_string()
        }
        Err(_) => ts.to_string(),
    }
}

fn main() -> cosmic::iced::Result {
    cosmic::applet::run::<Window>(())
}