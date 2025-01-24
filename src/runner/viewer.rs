use std::rc::Rc;
use ratatui::prelude::*;
use ratatui::widgets::*;
use Constraint::*;

use super::AppState;
use cells_world::CellsWorld;

mod game_widget;

//  //  //  //  //  //  //  //
pub fn view(app_state: &AppState, area: Rect, buf: &mut Buffer) {
    let [_top_area, game_area, status_area] =
        Layout::vertical([Length(4), Min(35), Min(4)]).areas(area);

    if let AppState::Working(working) = app_state {
        let cells = working.world.render_cells(15, 15);
        PlaygroundWidget(cells).render(game_area, buf);
    } else {
        PlaygroundWidget(None).render(game_area, buf);
    }

    if let AppState::Working(working) = app_state {
        StatusAreaWidget(working.is_gamepad_connected).render(status_area, buf);
    }
}

//  //  //  //  //  //  //  //
struct StatusAreaWidget(Option<bool>);
impl Widget for StatusAreaWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let s = match self.0 {
            Some(true) => "gamepad connected",
            Some(false) => "no gamepads",
            None => "",
        };
        Paragraph::new(s).render(area, buf);
    }
}

struct PlaygroundWidget(Option<Rc<CellsWorld>>);
impl Widget for PlaygroundWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [left_bar, other] = Layout::horizontal([Length(3), Length(67)]).areas(area);
        let [top_bar, play_zone] = Layout::vertical([Length(1), Length(33)]).areas(other);

        LeftGameBarWidget().render(left_bar, buf);
        TopGameBarWidget().render(top_bar, buf);
        game_widget::GameWidget(self.0).render(play_zone, buf);
    }
}

struct LeftGameBarWidget();
impl Widget for LeftGameBarWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut s = String::from("\n\n");
        for i in 0x0..0x10 {
            s += &format!("[{:X}]\n\n", i);
        }
        Paragraph::new(s).render(area, buf);
    }
}

struct TopGameBarWidget();
impl Widget for TopGameBarWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut s = String::from(" ");
        for i in 0x0..0x10 {
            s += &format!(" [{:X}]", i);
        }
        Paragraph::new(s).render(area, buf);
    }
}
