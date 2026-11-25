use chrono::Local;
use iced::time;
use iced::widget::{column, container, mouse_area, row, text};
use iced::window;
use iced::{Alignment, Background, Color, Element, Length, Point, Size, Subscription, Theme};
use std::time::Duration;

pub fn main() -> iced::Result {
    iced::application(
        "Aurore-At a Glance",
        AtAGlance::update,
        AtAGlance::view,
    )
    .subscription(AtAGlance::subscription)
    .style(|_state, _theme| iced::application::Appearance {
        background_color: Color::TRANSPARENT,
        text_color: Color::WHITE,
    })
    .window(window::Settings {
        size: Size::new(460.0, 110.0),
        position: window::Position::Specific(Point::new(36.0, 48.0)), // 左上にPixel風に配置
        decorations: false,
        transparent: true,
        resizable: false,
        level: window::Level::AlwaysOnBottom, // 作業の邪魔をしないよう最背面に固定
        ..Default::default()
    })
    .run()
}

struct AtAGlance {
    now_date: String,
    now_time: String,
    show_media_demo: bool,
}

impl Default for AtAGlance {
    fn default() -> Self {
        let (date, time) = Self::current_datetime();
        Self {
            now_date: date,
            now_time: time,
            show_media_demo: false,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Tick,
    ToggleView,
}

impl AtAGlance {
    fn current_datetime() -> (String, String) {
        let now = Local::now();
        let weekday = match now.format("%u").to_string().as_str() {
            "1" => "月",
            "2" => "火",
            "3" => "水",
            "4" => "木",
            "5" => "金",
            "6" => "土",
            "7" => "日",
            _ => "",
        };
        let date = format!("{} ({})", now.format("%m月%d日"), weekday);
        let time = now.format("%H:%M").to_string();
        (date, time)
    }

    fn subscription(&self) -> Subscription<Message> {
        // 10秒ごとに時刻チェック（CPU負荷ほぼ0%）
        time::every(Duration::from_secs(10)).map(|_| Message::Tick)
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Tick => {
                let (date, time) = Self::current_datetime();
                self.now_date = date;
                self.now_time = time;
            }
            Message::ToggleView => {
                self.show_media_demo = !self.show_media_demo;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        // 1行目: Pixel風の日付 + 時刻
        let primary_line = text(format!("{}  {}", self.now_date, self.now_time))
            .size(24)
            .color(Color::from_rgb(0.98, 0.99, 1.0));

        // 2行目: コンテキスト情報（クリックでメディア表示テストに切り替え可能）
        let secondary_content = if self.show_media_demo {
            row![
                text("♪").size(15).color(Color::from_rgb(0.20, 0.83, 0.60)),
                text("Tranquility - SawanoHiroyuki[nZk]:Anly")
                    .size(15)
                    .color(Color::from_rgba(0.95, 0.97, 1.0, 0.92)),
            ]
            .spacing(8)
            .align_y(Alignment::Center)
        } else {
            row![
                text("☀").size(15).color(Color::from_rgb(0.98, 0.80, 0.25)),
                text("Norrsken Linux (A m2 T)")
                    .size(15)
                    .color(Color::from_rgba(0.95, 0.97, 1.0, 0.90)),
            ]
            .spacing(8)
            .align_y(Alignment::Center)
        };

        let content = column![primary_line, secondary_content]
            .spacing(6)
            .align_x(Alignment::Start);

        // 背景枠なし（完全透過）のコンテナ
        let body = container(content)
            .padding(12)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme: &Theme| container::Style {
                background: Some(Background::Color(Color::TRANSPARENT)),
                ..Default::default()
            });

        mouse_area(body).on_press(Message::ToggleView).into()
    }
}
