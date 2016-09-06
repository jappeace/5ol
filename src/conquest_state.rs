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
use piston_window::keyboard::Key;

use geometry::*;
use stellar_bodies::*;
use camera::Camera;
use std::thread;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{channel, Receiver};
pub struct ConquestState{
    ids:Ids,
    camera:Camera,
    systems:Vec<System>,
    event:Arc<RwLock<StateEvent>>,
    gameTime:Arc<RwLock<Duration>>,
}
impl State for ConquestState{
    fn enter(&mut self) -> StateChange{
        let eventref = self.event.clone();
        let gameTimeRef = self.gameTime.clone();
        thread::spawn(move|| {
            while true{
                // at this point we gave up on channels and let locks into our
                // hearts, it actually made things simpler, believe it or not.
                *gameTimeRef.write().expect("poisned") = (
                    *gameTimeRef.read().expect("poison time")
                )+ Duration::weeks(1);
                *eventref.write().expect(
                    "posining on set event") = StateEvent::WantsUpdate;
                use std::time;
                thread::sleep(time::Duration::from_millis(100));
            }
        });
        None
    }
    fn poll_event(&self) -> StateEvent{
        *self.event.read().expect("poisining on read event")
    }
    fn update(&mut self, ui:&mut conrod::UiCell) ->  StateChange{
        println!("update");
        use conrod::{color, widget, Colorable, Widget};
        let canvas = widget::Canvas::new();
        canvas
            .color(color::BLUE)
            .crop_kids()
            .set(self.ids.canvas_root, ui) ;
        let dimens = ui.window_dim();
        let time = *self.gameTime.read().expect("there is no time");
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
            ],
            event:Arc::new(RwLock::new(StateEvent::Idle)),
            gameTime:Arc::new(RwLock::new(Duration::zero())),
        }
    }
}

widget_ids! {
    Ids {
        canvas_root,
    }
}
