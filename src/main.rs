use iced::{
    executor, Command, Element, Button,
    Row, Column, Text, Settings, Application,
    Container, Length, TextInput,
    text_input, button,
};

#[derive(Debug, Clone)]
enum Message {
    UrlChanged(String),
    Send,
    ResponseReceived(Result<String, ()>)
}

#[derive(Debug, Default)]
struct App {
    url_input_state: text_input::State,
    url_value: String,

    send_button_state: button::State,
    response_result: Option<Result<String, ()>>
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (App, Command<Self::Message>) {
        let app = App::default();
        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("Happy")
    }

    fn view(&mut self) -> Element<Self::Message> {
        let col1 = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(
                Row::new().padding(10))
                .push(Text::new("Request").size(25))
            .push(
                Row::new().padding(10)
                    .push(Text::new("GET"))
                    .push(TextInput::new(
                            &mut self.url_input_state,
                            "URL",
                            &mut self.url_value,
                            Message::UrlChanged))
                    .push(Button::new(
                            &mut self.send_button_state,
                            Text::new("SEND")
                        ).on_press(Message::Send)
                        .style(style::Button::Primary)
                    )
            );

        let resp: String = match self.response_result {
            None => "Enter URL and send your first request.".to_string(),
            Some(ref res) => {
                match res {
                    Err(_) => "Error happened".to_string(),
                    Ok(body) => body.clone(),
                }
            }
        };

        let col2 = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(
                Row::new().padding(10))
                    .push(Text::new("Response").size(25))
            .push(
                Row::new().padding(10)
                    .push(Text::new(resp))
            );
        let row = Row::new()
            .push(col1)
            .push(col2);


        let container = Container::new(row)
            .height(Length::Fill)
            .width(Length::Fill);

        let el: Element<Self::Message> = container.into();
        // el.explain(iced::Color::BLACK)
        el
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::UrlChanged(val) => {
                self.url_value = val;
                Command::none()
            }
            Message::Send => {
                println!("Send GET {}", self.url_value);
                Command::perform(get_body(self.url_value.clone()), Message::ResponseReceived)
            },
            Message::ResponseReceived(res) => {
                self.response_result = Some(res);
                Command::none()
            }
        }
    }
}

async fn get_body(url: String) -> Result<String, ()> {
    let body = reqwest::get(&url)
        .await.map_err(|_| ())?
        .text()
        .await.map_err(|_| ())?;

    Ok(body)
}

fn main() {
    App::run(Settings::default());
}

mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Primary,
        Secondary,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                    Button::Secondary => Color::from_rgb(0.5, 0.5, 0.5),
                })),
                border_radius: 12.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
                ..button::Style::default()
            }
        }

        fn hovered(&self) -> button::Style {
            button::Style {
                text_color: Color::WHITE,
                shadow_offset: Vector::new(1.0, 2.0),
                ..self.active()
            }
        }
    }
}
