use iced::widget::{button, column as iced_column, text, Column};

pub fn run_app() -> iced::Result {
    iced::application("Forme", State::update, State::view).run()
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

#[derive(Default)]
struct State {
    count: u64,
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.count += 1,
        }
    }

    fn view(&self) -> Column<Message> {
        Column::with_children(vec![
            text(self.count).into(),
            button("+").on_press(Message::Increment).into(),
        ])
    }
}
