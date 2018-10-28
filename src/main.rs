extern crate cursive;
extern crate rand;

mod game;

use cursive::Cursive;
use cursive::views::{Button, Dialog, LinearLayout, SelectView, Panel};
use cursive::vec::Vec2;
use cursive::Printer;
use cursive::theme::{Color, BaseColor, ColorStyle};
use cursive::direction::Direction;
use cursive::event::{Event, EventResult};

fn main() {
    let mut siv = Cursive::default();
    siv.set_fps(10);

    siv.add_layer(
        Dialog::new()
            .title("Game of Life")
            .padding((2, 2, 1, 1))
            .content(
                LinearLayout::vertical()
                  .child(Button::new_raw("   New Game   ", play_game))
                  .child(Button::new_raw("     Quit     ", |s| s.quit())),
            ),
    );

    siv.run();
}

fn play_game(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("select size")
            .padding((2, 2, 1, 1))
            .content(
                SelectView::new()
                    .item(
                        "   small:          8x8   ",
                        game::Options{
                            size: Vec2::new(8, 8),
                            n_alive: 20,
                        }
                    )
                    .item(
                        "   medium:         16x16   ",
                        game::Options{
                            size: Vec2::new(16, 16),
                            n_alive: 80,
                        }
                    )
                    .item(
                        "   large:          32x32   ",
                        game::Options{
                            size: Vec2::new(32, 32),
                            n_alive: 160,
                        }
                    )
                    .item(
                        "   extra-large:    128x32   ",
                        game::Options{
                            size: Vec2::new(128, 32),
                            n_alive: 1400,
                        }
                    ).on_submit(|s, options| {
                        s.pop_layer();
                        new_game(s, options);
                    }
            )
    ).dismiss_button("back"));
}

struct BoardView {
    board: game::Board,
}

impl BoardView {
    fn new(options: game::Options) -> Self {
        let board = game::Board::new(options);
        BoardView{
            board: board,
        }
    }
}

impl cursive::view::View for BoardView {
    fn draw(&self, printer: &Printer) {
        for (i, cell) in self.board.cells.iter().enumerate() {
            let x = i % self.board.size.x;
            let y = i / self.board.size.x;

            let mut color = Color::Dark(BaseColor::White);
            let mut text = " ";
            if cell.alive {
                color = Color::Dark(BaseColor::Black);
                text = "#";
            }

            printer.with_color(
                ColorStyle::new(Color::Dark(BaseColor::Black), color),
                |printer| printer.print((x, y), text)
            );
        }
    }

    fn take_focus(&mut self, _: Direction) -> bool {
        true
    }

    fn layout(&mut self, _: Vec2) {
        self.board.update();
    }
    
    fn on_event(&mut self, _: Event) -> EventResult {
        EventResult::Ignored
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        self.board.size
    }
}

fn new_game(siv: &mut Cursive, options: &game::Options) {
    siv.add_layer(
        Dialog::new()
            .title("Game of Life")
            .content(
                LinearLayout::horizontal()
                    .child(Panel::new(BoardView::new(*options)))
            ).button("Quit Game", |s| {
                s.pop_layer();
            }),
    );
}
