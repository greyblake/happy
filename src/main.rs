use iced::{
    executor, Command, Element, Button,
    Row, Column, Text, Settings, Application,
    Container, Length, TextInput,
    text_input, button, Subscription,
    keyboard,
    pick_list, PickList,
    Rule,
    scrollable, Scrollable,
};
use iced_native::{subscription, Event};
use derive_more::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
enum Method {
    GET,
    POST,
    PATCH,
    DELETE
}

impl Method {
    const ALL: [Method; 4] = [
        Method::GET,
        Method::POST,
        Method::PATCH,
        Method::DELETE
    ];
}

impl Default for Method {
    fn default() -> Self {
        Method::GET
    }
}

#[derive(Debug)]
enum ResponseState {
    New,
    Loading,
    Error(()),
    Received(String)
}

impl Default for ResponseState {
    fn default() -> Self {
        ResponseState::New
    }
}


#[derive(Debug, Clone)]
enum Message {
    UrlChanged(String),
    Send,
    ResponseReceived(Result<String, ()>),
    MethodPicked(Method),
}


#[derive(Debug, Default)]
struct App {
    url_input_state: text_input::State,
    url: String,

    send_button_state: button::State,
    response_state: ResponseState,

    method_state: pick_list::State<Method>,
    method: Method,

    scrollable_state: scrollable::State,
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (App, Command<Self::Message>) {
        let mut app = App::default();
        app.url = "https://httpbin.org/delay/1".to_string();
        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("Happy")
    }

    fn view(&mut self) -> Element<Self::Message> {
        let method_picker = PickList::new(
            &mut self.method_state,
            &Method::ALL[..],
            Some(self.method),
            Message::MethodPicked
        );

        let col1 = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(
                Row::new().padding(10))
                .push(Text::new("Request").size(25))
            .push(
                Row::new().padding(10)
                    //.push(Text::new("GET"))
                    .push(method_picker)
                    .push(
                        TextInput::new(
                            &mut self.url_input_state,
                            "URL",
                            &mut self.url,
                            Message::UrlChanged
                        ).padding(10).size(16)
                    )
                    .push(Button::new(
                            &mut self.send_button_state,
                            Text::new("Send")
                        ).on_press(Message::Send)
                        .style(style::Button::Primary)
                    )
            );


        let resp_text = match self.response_state {
            ResponseState::New => "Enter URL and send your first request.".to_string(),
            ResponseState::Loading => "...LOADING...".to_string(),
            ResponseState::Error(_) => "Error happened".to_string(),
            ResponseState::Received(ref body) => body.clone()
        };

        let col2 = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .push(
                Row::new().padding(10))
                    .push(Text::new("Response").size(25))
            .push(
                Row::new().padding(10)
                    .push(
                        Scrollable::new(&mut self.scrollable_state)
                            .push(Text::new(resp_text))
                    )
            );
        let row = Row::new()
            .push(col1)
            .push(Rule::vertical(30))
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
                self.url = val;
                Command::none()
            }
            Message::Send => {
                self.response_state = ResponseState::Loading;
                Command::perform(get_body(self.url.clone()), Message::ResponseReceived)
            },
            Message::ResponseReceived(result) => {
                self.response_state = match result {
                    Ok(resp) => ResponseState::Received(resp),
                    Err(_) => ResponseState::Error(())
                };
                Command::none()
            }
            Message::MethodPicked(method) => {
                self.method = method;
                Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        use keyboard::Event::KeyPressed;
        use keyboard::KeyCode;

        subscription::events_with(|event, _status| {
            match event {
                Event::Keyboard(KeyPressed { key_code: KeyCode::Enter, .. }) => Some(Message::Send),
                _ => None
            }
        })
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
    App::run(Settings::default()).unwrap()
}

mod style {
    use iced::{button, Background, Color, Vector};

    pub enum Button {
        Primary,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                })),
                border_radius: 3.0,
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
