// This program is a 4x space game.
// Copyright (C) 2016 Jappie Klooster

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.If not, see <http://www.gnu.org/licenses/>.


// this file describes the first state of the game, showing a little welcome
// message.

use conrod::{widget_ids, color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};
use conrod;

use super::state_machine::{State, StateChange};
use super::conquest::ConquestState;

pub struct BeginState {
    ids: Ids,
}
impl BeginState {
    pub fn new(generator: conrod::widget::id::Generator) -> BeginState {
        BeginState { ids: Ids::new(generator) }
    }
}
impl State for BeginState {
    fn update(&mut self, ui: &mut conrod::UiCell) -> StateChange {

        const INTRO_TEXT: &'static str =
            "After a long power struggle over now several generations, 
    you Kim Ill Sung III \
             has finally crushed all opposition.
    With no-one else left daring to oppose you \
             mankind puts it faith in you,
    our dearest leader, and you look to the stars...
";
        // Construct our main `Canvas` tree.
        widget::Canvas::new().color(color::BLACK).set(self.ids.canvas_root, ui);
        widget::Text::new(INTRO_TEXT)
            .color(color::LIGHT_RED)
            .middle_of(self.ids.canvas_root)
            .left_justify()
            .line_spacing(10.0)
            .set(self.ids.text_intro, ui);
        for _ in widget::Button::new()
            .w_h(200.0, 80.0)
            .label("Begin your conquest")
            .color(color::DARK_CHARCOAL)
            .label_color(color::GRAY)
            .set(self.ids.button_begin, ui) {
            return Some(Box::new(ConquestState::new_game(ui.widget_id_generator())));
        }
        None
    }
}
// Generate a unique `WidgetId` for each widget.
widget_ids! {
    struct Ids {
        canvas_root,
        text_intro,
        button_begin
    }
}
