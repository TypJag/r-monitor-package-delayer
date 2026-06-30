mod network;
mod protocol;

use std::time::Duration;

use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, column, container, row, text};
use iced::{Element, Length, Subscription, Task};
use tokio::sync::mpsc;

use protocol::{format_pixel_message, parse_lap_field};

// ---------------------------------------------------------------------------
// PixelSender — newtype so UnboundedSender can sit in a derived-Debug enum
// ---------------------------------------------------------------------------

#[derive(Clone)]
pub struct PixelSender(pub mpsc::UnboundedSender<String>);

impl std::fmt::Debug for PixelSender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PixelSender").finish_non_exhaustive()
    }
}

// ---------------------------------------------------------------------------
// Messages
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum Message {
    // 1-second heartbeat
    Tick,
    // Timer controls (mirror the web UI buttons)
    IncreaseTime,   // +5 s
    DecreaseTime,   // -5 s
    IncreaseDefault, // +1 s on default
    DecreaseDefault, // -1 s on default
    // Action buttons
    SendNow,  // force timer to 0, trigger finish on next Tick
    Halt,
    Unhalt,
    Reset,
    PlusOne,  // send remainingLaps as-is (sendToPixel2 equivalent)
    // Network events
    ScoritData(String),
    ScoritConnected,
    ScoritDisconnected,
    PixelConnected(PixelSender),
    PixelDisconnected,
}

// ---------------------------------------------------------------------------
// Application state
// ---------------------------------------------------------------------------

struct App {
    default_time: u32,
    time_left: u32,
    halt: bool,
    remaining_laps: i32,
    is_finished: bool,
    scorit_connected: bool,
    pixel_sender: Option<PixelSender>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            default_time: 30,
            time_left: 30,
            halt: false,
            remaining_laps: 10,
            is_finished: false,
            scorit_connected: false,
            pixel_sender: None,
        }
    }
}

impl App {
    // ------------------------------------------------------------------
    // Helper: write a pixel message (fires-and-forgets via mpsc)
    // ------------------------------------------------------------------

    fn send_to_pixel(&self, laps: i32) {
        if let Some(ps) = &self.pixel_sender {
            ps.0.send(format_pixel_message(laps)).ok();
        }
    }

    // ------------------------------------------------------------------
    // Update
    // ------------------------------------------------------------------

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            // ----- Timer tick -----
            Message::Tick => {
                if !self.halt {
                    self.time_left = self.time_left.saturating_sub(1);
                    if self.time_left == 0 && !self.is_finished {
                        self.is_finished = true;
                        // Mirror sendToPixel: send remainingLaps - 1
                        self.send_to_pixel(self.remaining_laps - 1);
                    }
                }
            }

            // ----- Manual timer controls -----
            Message::IncreaseTime => {
                self.time_left = self.time_left.saturating_add(5);
            }
            Message::DecreaseTime => {
                self.time_left = self.time_left.saturating_sub(5);
            }
            Message::IncreaseDefault => {
                self.default_time = self.default_time.saturating_add(1);
            }
            Message::DecreaseDefault => {
                self.default_time = self.default_time.saturating_sub(1);
            }

            // ----- Action buttons -----
            Message::SendNow => {
                // Mirror Python on_end: set timeLeft=0, isFinished=False.
                // The finish/send fires on the next Tick.
                self.time_left = 0;
                self.is_finished = false;
            }
            Message::Halt => {
                self.halt = true;
            }
            Message::Unhalt => {
                self.halt = false;
            }
            Message::Reset => {
                self.time_left = self.default_time;
                self.is_finished = false;
            }
            Message::PlusOne => {
                // Mirror sendToPixel2: send remainingLaps as-is
                self.send_to_pixel(self.remaining_laps);
            }

            // ----- Scorit TCP events -----
            Message::ScoritData(line) => {
                if let Some(lap_field) = parse_lap_field(&line) {
                    let current_str = self.remaining_laps.to_string();
                    // Only act when the lap field differs from the stored value
                    if lap_field != current_str {
                        let new_laps = if lap_field == "9999" {
                            // Leader crossed the finish line
                            if self.remaining_laps == 1 {
                                // Already on last lap — ignore (matches Python behaviour)
                                return Task::none();
                            }
                            1
                        } else {
                            match lap_field.parse::<i32>() {
                                Ok(n) => n,
                                Err(_) => return Task::none(),
                            }
                        };
                        // Leader passed — reset the countdown and update the lap counter
                        self.time_left = self.default_time;
                        self.is_finished = false;
                        self.remaining_laps = new_laps;
                    }
                }
            }
            Message::ScoritConnected => {
                self.scorit_connected = true;
            }
            Message::ScoritDisconnected => {
                self.scorit_connected = false;
            }

            // ----- PixelCom TCP events -----
            Message::PixelConnected(ps) => {
                self.pixel_sender = Some(ps);
            }
            Message::PixelDisconnected => {
                self.pixel_sender = None;
            }
        }

        Task::none()
    }

    // ------------------------------------------------------------------
    // View
    // ------------------------------------------------------------------

    fn view(&self) -> Element<'_, Message> {
        // Large countdown display
        let timer_value = text(self.time_left.to_string()).size(80.0);

        // Laps counter: display remainingLaps - 1 (mirrors the web UI `Laps:` field)
        let laps_display = text(format!(
            "Laps: {}",
            (self.remaining_laps - 1).max(0)
        ))
        .size(28.0);

        // Default time display
        let default_display =
            text(format!("Default Time: {}s", self.default_time)).size(22.0);

        // Timer ±5 s row
        let time_row = row![
            button(text("-5")).on_press(Message::DecreaseTime),
            button(text("+5")).on_press(Message::IncreaseTime),
        ]
        .spacing(10.0)
        .align_y(Vertical::Center);

        // Default ±1 s row
        let default_row = row![
            button(text("-")).on_press(Message::DecreaseDefault),
            default_display,
            button(text("+")).on_press(Message::IncreaseDefault),
        ]
        .spacing(10.0)
        .align_y(Vertical::Center);

        // Action buttons
        let action_row = row![
            button(text("Send now!")).on_press(Message::SendNow),
            button(text("Halt")).on_press(Message::Halt),
            button(text("Unhalt")).on_press(Message::Unhalt),
            button(text("Reset")).on_press(Message::Reset),
            button(text("Send Laps Plus One!")).on_press(Message::PlusOne),
        ]
        .spacing(10.0);

        // HALT indicator (blank when running so layout stays stable)
        let halt_label = if self.halt {
            text("[ HALTED ]").size(22.0)
        } else {
            text("").size(22.0)
        };

        // Connection status row
        let scorit_label = text(if self.scorit_connected {
            "Scorit: Connected"
        } else {
            "Scorit: Disconnected"
        })
        .size(16.0);

        let pixel_label = text(if self.pixel_sender.is_some() {
            "Pixel: Connected"
        } else {
            "Pixel: Disconnected"
        })
        .size(16.0);

        let status_row = row![scorit_label, pixel_label].spacing(30.0);

        let content = column![
            timer_value,
            laps_display,
            time_row,
            default_row,
            action_row,
            halt_label,
            status_row,
        ]
        .spacing(16.0)
        .align_x(Horizontal::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .padding(24.0)
            .into()
    }

    // ------------------------------------------------------------------
    // Subscriptions
    // ------------------------------------------------------------------

    fn subscription(&self) -> Subscription<Message> {
        let tick = iced::time::every(Duration::from_secs(1)).map(|_| Message::Tick);
        let scorit = network::scorit_subscription();
        let pixel = network::pixel_subscription();
        Subscription::batch(vec![tick, scorit, pixel])
    }
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn main() -> iced::Result {
    iced::application("Race Monitor", App::update, App::view)
        .subscription(App::subscription)
        .run()
}
