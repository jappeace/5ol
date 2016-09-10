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

use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};
use conrod;

use state_machine::{State, StateChange};
use conquest_state::ConquestState;
use stellar_bodies::*;

pub struct PlanetState<'a>{
    ids:Ids,
    subject:&'a StellarBody,
    go_back_to:Option<Box<State>>,
}
impl<'a> PlanetState<'a>{
    pub fn new(generator: conrod::widget::id::Generator, subject:&'a StellarBody)->PlanetState<'a>{
        PlanetState{
            ids:Ids::new(generator),
            subject:subject,
            go_back_to:None
        }
    }
}
impl<'a> State for PlanetState<'a>{
    fn enter(&mut self, previous_state:Box<State>) -> StateChange{
        self.go_back_to = Some(previous_state);
        None
    }
    fn update(&mut self, ui:&mut conrod::UiCell) -> StateChange{
        // Construct our main `Canvas` tree.
        widget::Canvas::new().color(color::BLACK).set(self.ids.canvas_root, ui);
        widget::Text::new(self.subject.name)
            .color(color::LIGHT_RED)
            .top_of(self.ids.canvas_root)
            .align_text_left()
            .line_spacing(10.0)
            .set(self.ids.text_intro, ui);
        for _ in widget::Button::new()
            .w_h(200.0, 80.0)
            .label("Take me back")
            .color(color::DARK_CHARCOAL)
            .label_color(color::GRAY)
            .set(self.ids.button_begin, ui){
                return Some(self.go_back_to.expect("nothing to go back to"))
            }
        None
    }
}

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    Ids {
        canvas_root,
        text_intro,
        button_begin
    }
}
