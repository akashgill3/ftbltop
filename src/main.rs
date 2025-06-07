// use std::time::Duration;

// use ratatui::{
//     Frame,
//     crossterm::event::{self, Event, KeyCode},
//     layout::{Alignment, Constraint, Direction, Layout, Rect},
//     widgets::{Block, Paragraph},
// };

// #[derive(Debug, Default)]
// struct Model {
//     counter: i32,
//     running_state: RunningState,
// }

// #[derive(Debug, Default, PartialEq, Eq)]
// enum RunningState {
//     #[default]
//     Running,
//     Done,
// }

// #[derive(PartialEq)]
// enum Message {
//     Increment,
//     Decrement,
//     Reset,
//     Quit,
// }

// fn main() -> color_eyre::Result<()> {
//     tui::install_panic_hook();
//     let mut terminal = tui::init_terminal()?;
//     let mut model = Model::default();

//     while model.running_state != RunningState::Done {
//         // Render the current view
//         terminal.draw(|f| view(&mut model, f))?;

//         // Handle events and map to a Message
//         let mut current_msg = handle_event(&model)?;

//         // Process updates as long as they return a non-None message
//         while current_msg.is_some() {
//             current_msg = update(&mut model, current_msg.unwrap());
//         }
//     }

//     tui::restore_terminal()?;
//     Ok(())
// }

// fn view(model: &mut Model, frame: &mut Frame) {
//     let area = frame.area();

//     let center_area = centered_rect(50, 50, area); // 60% width, 40% height

//     frame.render_widget(
//         Paragraph::new(format!("Counter: {}", model.counter)).block(
//             Block::bordered()
//                 .title("ftbltop")
//                 .title_alignment(Alignment::Center),
//         ),
//         center_area,
//     );
//     frame.render_widget(
//         Paragraph::new(format!("Counter: {}", model.counter)).block(
//             Block::bordered()
//                 .title("ftbltop")
//                 .title_alignment(Alignment::Center),
//         ),
//         center_area,
//     );
// }

// fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
//     let popup_layout = Layout::default()
//         .direction(Direction::Vertical)
//         .constraints([
//             Constraint::Percentage((100 - percent_y) / 2),
//             Constraint::Percentage(percent_y),
//             Constraint::Percentage((100 - percent_y) / 2),
//         ])
//         .split(r);

//     Layout::default()
//         .direction(Direction::Horizontal)
//         .constraints([
//             Constraint::Percentage((100 - percent_x) / 2),
//             Constraint::Percentage(percent_x),
//             Constraint::Percentage((100 - percent_x) / 2),
//         ])
//         .split(popup_layout[1])[1]
// }

// /// Convert Event to Message
// ///
// /// We don't need to pass in a `model` to this function in this example
// /// but you might need it as your project evolves
// fn handle_event(_: &Model) -> color_eyre::Result<Option<Message>> {
//     if event::poll(Duration::from_millis(250))? {
//         if let Event::Key(key) = event::read()? {
//             if key.kind == event::KeyEventKind::Press {
//                 return Ok(handle_key(key));
//             }
//         }
//     }
//     Ok(None)
// }

// fn handle_key(key: event::KeyEvent) -> Option<Message> {
//     match key.code {
//         KeyCode::Char('j') => Some(Message::Increment),
//         KeyCode::Char('k') => Some(Message::Decrement),
//         KeyCode::Char('q') => Some(Message::Quit),
//         _ => None,
//     }
// }

// fn update(model: &mut Model, msg: Message) -> Option<Message> {
//     match msg {
//         Message::Increment => {
//             model.counter += 1;
//             if model.counter > 50 {
//                 return Some(Message::Reset);
//             }
//         }
//         Message::Decrement => {
//             model.counter -= 1;
//             if model.counter < -50 {
//                 return Some(Message::Reset);
//             }
//         }
//         Message::Reset => model.counter = 0,
//         Message::Quit => {
//             // You can handle cleanup and exit here
//             model.running_state = RunningState::Done;
//         }
//     };
//     None
// }

// mod tui {
//     use ratatui::{
//         Terminal,
//         backend::{Backend, CrosstermBackend},
//         crossterm::{
//             ExecutableCommand,
//             terminal::{
//                 EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
//             },
//         },
//     };
//     use std::{io::stdout, panic};

//     pub fn init_terminal() -> color_eyre::Result<Terminal<impl Backend>> {
//         enable_raw_mode()?;
//         stdout().execute(EnterAlternateScreen)?;
//         let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
//         Ok(terminal)
//     }

//     pub fn restore_terminal() -> color_eyre::Result<()> {
//         stdout().execute(LeaveAlternateScreen)?;
//         disable_raw_mode()?;
//         Ok(())
//     }

//     pub fn install_panic_hook() {
//         let original_hook = panic::take_hook();
//         panic::set_hook(Box::new(move |panic_info| {
//             stdout().execute(LeaveAlternateScreen).unwrap();
//             disable_raw_mode().unwrap();
//             original_hook(panic_info);
//         }));
//     }
// }
//
use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    symbols,
    widgets::{Block, Borders},
};

/// This example shows how to use custom borders to collapse borders between widgets.
/// See https://ratatui.rs/how-to/layout/collapse-borders for more info
fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(draw)?;
        if key_pressed()? {
            return Ok(());
        }
    }
}

fn key_pressed() -> Result<bool> {
    Ok(event::poll(Duration::from_millis(16))? && matches!(event::read()?, Event::Key(_)))
}

fn draw(frame: &mut Frame) {
    use Constraint::{Fill, Length, Min};

    let vertical = Layout::vertical([Length(1), Min(20), Length(1)]);
    let [title_area, main_area, status_area] = vertical.areas(frame.area());
    let horizontal = Layout::horizontal([Fill(1); 2]);
    let [left_area, right_area] = horizontal.areas(main_area);

    frame.render_widget(Block::bordered().title("Title Bar"), title_area);
    frame.render_widget(Block::bordered().title("Status Bar"), status_area);
    frame.render_widget(Block::bordered().title("Left"), left_area);
    frame.render_widget(Block::bordered().title("Right"), right_area);
}

fn draw_blocks(frame: &mut Frame) {
    // create a layout that splits the screen into 2 equal columns and the right column
    // into 2 equal rows

    // use a 49/51 split instead of 50/50 to ensure that any extra space is on the right
    // side of the screen. This is important because the right side of the screen is
    // where the borders are collapsed.
    let [left, right] =
        Layout::horizontal([Constraint::Percentage(49), Constraint::Percentage(51)])
            .areas(frame.area());
    // use a 49/51 split to ensure that any extra space is on the bottom
    let [top_right, bottom_right] =
        Layout::vertical([Constraint::Percentage(49), Constraint::Percentage(51)]).areas(right);

    let left_block = Block::new()
        // don't render the right border because it will be rendered by the right block
        .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM)
        .title("Left Block");

    // top right block must render the top left border to join with the left block
    let top_right_border_set = symbols::border::Set {
        top_left: symbols::line::NORMAL.horizontal_down,
        ..symbols::border::PLAIN
    };
    let top_right_block = Block::new()
        .border_set(top_right_border_set)
        // don't render the bottom border because it will be rendered by the bottom block
        .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
        .title("Top Right Block");

    // bottom right block must render:
    // - top left border to join with the left block and top right block
    // - top right border to join with the top right block
    // - bottom left border to join with the left block
    let collapsed_top_and_left_border_set = symbols::border::Set {
        top_left: symbols::line::NORMAL.vertical_right,
        top_right: symbols::line::NORMAL.vertical_left,
        bottom_left: symbols::line::NORMAL.horizontal_up,
        ..symbols::border::PLAIN
    };
    let bottom_right_block = Block::new()
        .border_set(collapsed_top_and_left_border_set)
        .borders(Borders::ALL)
        .title("Bottom Right Block");

    frame.render_widget(left_block, left);
    frame.render_widget(top_right_block, top_right);
    frame.render_widget(bottom_right_block, bottom_right);
}
