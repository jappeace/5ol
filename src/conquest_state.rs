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

// this file describes the main game where you stare at a map of the galaxy

use state_machine::{State, StateChange};
use piston_window::Input;
use conrod;
use time::Duration;
use piston_window::keyboard::Key;

use geometry::*;
use stellar_bodies::*;
use camera::Camera;

pub struct ConquestState{
    ids:Ids,
    camera:Camera,
    systems:Vec<System>
}
impl State for ConquestState{
    fn update(&mut self, ui:&mut conrod::UiCell) ->  StateChange{
        println!("update");
        use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget, Scalar};
        use conrod::widget::Line;
        let canvas = widget::Canvas::new();
        canvas
            .color(color::BLUE)
            .crop_kids()
            .set(self.ids.canvas_root, ui) ;
        let dimens = ui.window_dim();
        self.camera.update(ui, &dimens, &self.systems);
        None
    }
    fn input(&mut self, input:Input) -> StateChange{
        use piston_window::Input::*;
        use piston_window::Button::*;
        use piston_window::keyboard::Key::*;
        match input {
            Press(Keyboard(key)) => match key {
                W => self.camera.position.y -= 0.1,
                S => self.camera.position.y += 0.1,
                D => self.camera.position.x -= 0.1,
                A => self.camera.position.x += 0.1,
                _ => {}
            },
            _ => {}
        }
        None
    }
}

impl ConquestState{
    pub fn new(mut generator: conrod::widget::id::Generator)->ConquestState{
        ConquestState{
            ids:Ids::new(generator),
            camera:Camera::new(center,2.0,2.0),
            systems:vec![
                System::new(
                    center,
                    vec![
                        create_single_star("sun"),
                        StellarBody{
                            name:"mercury",
                            orbitTime:Duration::days(88),
                            distance:0.387098 
                        },
                        StellarBody{
                            name:"venus",
                            orbitTime:Duration::days(225),
                            distance:0.723332
                        },
                        StellarBody{
                            name:"earth",
                            orbitTime:Duration::days(365),
                            distance:1.0
                        },
                        StellarBody{
                            name:"mars",
                            orbitTime:Duration::days(780),
                            distance:1.523679
                        },
                    ]
                )
            ]
        }
    }
}

widget_ids! {
    Ids {
        canvas_root,
    }
}
