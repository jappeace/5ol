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


use state_machine::State;
use conrod;
pub struct ConquestState{
    ids:Ids
}
impl ConquestState{
    pub fn new(mut generator: conrod::widget::id::Generator)->ConquestState{
        ConquestState{ids:Ids::new(generator)}
    }
}
impl State for ConquestState{
    
    fn render(&mut self, ui:&mut conrod::UiCell) ->  Option<Box<State>>{
        use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget, Scalar};
        use conrod::widget::Line;
        widget::Canvas::new().color(color::BLUE).set(self.ids.canvas_root, ui);
        None
    }
}

widget_ids! {
    Ids {
        canvas_root,
    }
}
