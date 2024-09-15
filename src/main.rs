use iced::executor;
use iced::font::Font;
use iced::theme;
use iced::widget::{checkbox, column, container, row, text};
use iced::{Application, Command, Element, Length, Settings, Theme};
use std::borrow::Cow;

const ICON_FONT: Font = Font::with_name("icons");
const D_FONT: Font = Font::with_name("HarmonyOS Sans SC");
pub fn main() -> iced::Result {
    let mut setting = Settings::<()>::default();
    let icon_font = include_bytes!("../fonts/icons.ttf").as_slice();
    let icon_font: Cow<[u8]> = Cow::Borrowed(&icon_font[..]);
    let default_font = include_bytes!("../fonts/HarmonyOS_Sans_SC_Regular.ttf").as_slice();
    let default_font: Cow<[u8]> = Cow::Borrowed(&default_font[..]);
    setting.fonts.push(icon_font);
    setting.fonts.push(default_font);
    setting.default_font = D_FONT;
    Example::run(setting)
}

#[derive(Default)]
struct Example {
    default: bool,
    styled: bool,
    custom: bool,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    DefaultToggled(bool),
    CustomToggled(bool),
    StyledToggled(bool),
}

impl Application for Example {
    type Message = Message;
    type Flags = ();
    type Executor = executor::Default;
    type Theme = Theme;

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
            Message::CustomToggled(custom) => {
                self.custom = custom;
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

        let custom_checkbox = checkbox("Custom", self.custom)
            .on_toggle(Message::CustomToggled)
            .icon(checkbox::Icon {
                font: ICON_FONT,
                code_point: '\u{e901}',
                size: None,
                line_height: text::LineHeight::Relative(1.0),
                shaping: text::Shaping::Basic,
            });

        let content = column![default_checkbox, checkboxes, custom_checkbox].spacing(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
