use iced::Length::Units;
use iced::{button, slider, Align, Button, Column, Element, Sandbox, Settings, Slider, Text};

pub fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.size = (300, 300);
    settings.window.resizable = false;
    Frontend::run(settings) //Starts the application
}

#[derive(Default)]
struct Frontend {
    voltage: i32,
    current: i32,
    ok_button: button::State,
    slider1: slider::State,
    slider2: slider::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ButtonPressed,
    Slider1Changed(i32),
    Slider2Changed(i32),
}

impl Sandbox for Frontend {
    type Message = Message;

    // Constructor
    fn new() -> Self {
        let Frontend = Self::default();
        return Frontend;
    }

    // Set Title
    fn title(&self) -> String {
        String::from("Power Source")
    }

    // Message Loop - handles state transition
    fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed => {
                self.voltage = 0;
                self.current = 0;
            }
            Message::Slider1Changed(v) => {
                self.voltage = v;
            }
            Message::Slider2Changed(v) => {
                self.current = v;
            }
        }
    }
    // Build Application
    fn view(&mut self) -> Element<Message> {
        let column: Element<Message> = Column::new() //Column Layout
            .padding(20)
            .push(Slider::new(
                &mut self.slider1,
                0..=100,
                self.voltage,
                Message::Slider1Changed,
            ))
            .push(
                Text::new({
                    let mut string = self.voltage.to_string();
                    string.push_str(" V");
                    string
                })
                .size(50),
            )
            .push(Slider::new(
                &mut self.slider2,
                0..=100,
                self.current,
                Message::Slider2Changed,
            ))
            .push(
                Text::new({
                    let mut string = self.current.to_string();
                    string.push_str(" mA");
                    string
                })
                .size(50),
            )
            .push(
                Button::new(&mut self.ok_button, Text::new("OK")).on_press(Message::ButtonPressed),
            )
            .into();
        return column;
    }
}
