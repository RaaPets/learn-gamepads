use ratatui::prelude::*;
use Constraint::*;
use ratatui::widgets::*;

use super::AppState;

mod game_widget;

//  //  //  //  //  //  //  //
pub fn view(app_state: &AppState, area: Rect, buf: &mut Buffer) {
    let [top_area, game_area, status_area] =
        Layout::vertical([Length(4), Min(35), Min(4)]).areas(area);

    if let AppState::Working(_, pos) = app_state {
        PlaygroundWidget(Some(pos)).render(game_area, buf);
    } else {
        PlaygroundWidget(None).render(game_area, buf);
    }

    if let AppState::Working(f, _) = app_state {
        StatusAreaWidget(*f).render(status_area, buf);
    }
}

//  //  //  //  //  //  //  //
struct StatusAreaWidget(bool);
impl Widget for StatusAreaWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let s = match self.0 {
            true => "gamepad connected",
            false => "no gamepads",
        };
        Paragraph::new(s).render(area, buf);
    }
}

struct PlaygroundWidget<'a>(Option<&'a super::app_state::TestPos>);
impl Widget for PlaygroundWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [left_bar, other] = Layout::horizontal([Length(3), Length(67)]).areas(area);
        let [top_bar, play_zone] = Layout::vertical([Length(1), Length(33)]).areas(other);

        LeftGameBarWidget().render(left_bar, buf);
        TopGameBarWidget().render(top_bar, buf);
        if let Some(ref pos) = self.0 {
            let mut cells = cells_world::CellsWorld::new(16,16);
            let ij = (pos.x as isize, pos.y as isize);
            cells[ij] = cells_world::CellState::Player;
            game_widget::GameWidget(Some(&cells)).render(play_zone, buf);
        }
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
