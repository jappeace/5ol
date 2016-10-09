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
use std::sync::Arc;

use state::state_machine::{State, StateChange};
use model::galaxy::{BodyAddress, BodyClass};
use model::ship::Ship;
use async::model_access::{Change,ModelAccess};

pub struct PlanetState{
    ids:Ids,
    subject: BodyAddress,
    previous_state:Option<Box<State>>,
    model_access:ModelAccess
}
impl PlanetState{
    pub fn new(
        generator: conrod::widget::id::Generator,
        subject:BodyAddress,
        model_access:ModelAccess
    )->PlanetState{
        PlanetState{
            ids:Ids::new(generator),
            subject:subject,
            previous_state:None,
            model_access:model_access
        }
    }
}
impl State for PlanetState{
    fn enter(&mut self, previous:Box<State>) -> StateChange{
        self.previous_state = Some(previous);
        self.model_access.start();
        None
    }
    fn update(&mut self, ui:&mut conrod::UiCell) -> StateChange{
        // Construct our main `Canvas` tree.
        widget::Canvas::new().color(color::BLACK).set(self.ids.canvas_root, ui);
        let body = self.subject.get_body(&self.model_access.read_lock_model().galaxy).clone();
        let bodyinfo = match &body.class{
            &BodyClass::Rocky(ref habitat) =>{
                let head_count = if let Some(ref pop) = habitat.population {
                    pop.head_count  
                } else {0};
                ("rocky world", head_count)
            },
            &BodyClass::GasGiant => ("gass giant", 0),
            &BodyClass::Star => ("star", 0)
        };
        let text = format!("{} is a {} \n population {}", body.name, bodyinfo.0, bodyinfo.1);
        widget::Text::new(&text)
            .color(color::LIGHT_RED)
            .middle_of(self.ids.canvas_root)
            .align_text_left()
            .line_spacing(10.0)
            .set(self.ids.text_intro, ui);
        for _ in widget::Button::new()
            .w_h(200.0, 80.0)
            .label("Take me back")
            .color(color::DARK_CHARCOAL)
            .label_color(color::GRAY)
            .set(self.ids.button_begin, ui){
                let mut stored:Option<Box<State>> = None;
                use std::mem::swap;
                swap(&mut stored, &mut self.previous_state);
                return stored;
            }
        if let BodyClass::Rocky(habitat) = body.class{
            if let Some(owner)  = habitat.owner{
                for _ in widget::Button::new()
                    .w_h(200.0, 80.0)
                    .label("build ship")
                    .color(color::DARK_CHARCOAL)
                    .label_color(color::GRAY)
                    .set(self.ids.build_ship , ui){
                        println!("building for {}", owner);
                        self.model_access.enqueue(
                            Change::Construct(
                                Arc::new(
                                    Ship::new(0,1000,self.subject)
                                ),
                                self.subject
                            )
                        )
                    }
            }
        }
        None
    }
    fn exit(&mut self){
        self.model_access.stop();
    }
}

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    Ids {
        canvas_root,
        text_intro,
        button_begin,
        build_ship
    }
}
