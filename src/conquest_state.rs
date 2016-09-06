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

use state_machine::{State, StateChange, StateEvent};
use piston_window::Input;
use conrod;
use time::Duration;

use geometry::*;
use stellar_bodies::*;
use camera::Camera;
use update_thread::Updater;
pub struct ConquestState{
    ids:Ids,
    camera:Camera,
    systems:Vec<System>,
    updater:Updater,
}
impl State for ConquestState{
    fn enter(&mut self) -> StateChange{
        self.updater.start();
        None
    }
    fn poll_event(&self) -> StateEvent{
        let mut shared = self.updater.event.lock().expect("poisining on read event");
        let result = *shared;
        *shared = StateEvent::Idle;
        result
    }
    fn update(&mut self, ui:&mut conrod::UiCell) ->  StateChange{
        use conrod::{color, widget, Colorable, Widget};
        let canvas = widget::Canvas::new();
        canvas
            .color(color::BLUE)
            .crop_kids()
            .set(self.ids.canvas_root, ui) ;
        let dimens = ui.window_dim();
        let time = *self.updater.game_time.read().expect("there is no time");
        println!("update {:?}, {}", time, time.num_weeks());
        self.camera.update(ui, &dimens, &self.systems, &time);
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
    pub fn new(generator: conrod::widget::id::Generator)->ConquestState{
        ConquestState{
            ids:Ids::new(generator),
            camera:Camera::new(center,2.0,2.0),
            systems:vec![
                System::new(
                    center,
                    vec![
                        StellarBody::create_single_star("sun"),
                        StellarBody::new("mercury", Duration::days(88), 0.387098),
                        StellarBody::new("venus", Duration::days(225),0.723332),
                        StellarBody::new("earth",Duration::days(365),1.0),
                        StellarBody::new("mars",Duration::days(780),1.523679),
                    ]
                )
            ],
            updater:Updater::new(StateEvent::Idle, Duration::zero())
        }
    }
}

widget_ids! {
    Ids {
        canvas_root,
    }
}
