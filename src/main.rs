use iced::executor;
use iced::font::Font;
use iced::theme;
use iced::widget::{checkbox, column, container, row};
use iced::{Application, Command, Element, Length, Settings, Theme};
use std::borrow::Cow;

pub fn main() -> iced::Result {
    let mut setting = Settings::<()>::default();
    let default_font = include_bytes!("../fonts/HarmonyOS_Sans_SC_Regular.ttf").as_slice();
    let default_font: Cow<[u8]> = Cow::Borrowed(&default_font[..]);
    setting.fonts.push(default_font);
    setting.default_font = Font::with_name("HarmonyOS Sans SC");
    Example::run(setting)
}

#[derive(Default)]
struct Example {
    default: bool,
    styled: bool,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    DefaultToggled(bool),
    StyledToggled(bool),
}

impl Application for Example {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Checkbox - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::DefaultToggled(default) => {
                self.default = default;
            }
            Message::StyledToggled(styled) => {
                self.styled = styled;
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let default_checkbox = checkbox("缺省值", self.default).on_toggle(Message::DefaultToggled);

        let styled_checkbox = |label, style| {
            checkbox(label, self.styled)
                .on_toggle_maybe(self.default.then_some(Message::StyledToggled))
                .style(style)
        };

        let checkboxes = row![
            styled_checkbox("主要的", theme::Checkbox::Primary),
            styled_checkbox("にほんご", theme::Checkbox::Secondary),
            styled_checkbox("Success", theme::Checkbox::Success),
            styled_checkbox("Danger", theme::Checkbox::Danger),
        ]
        .spacing(20);

        let content = column![default_checkbox, checkboxes].spacing(30);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
