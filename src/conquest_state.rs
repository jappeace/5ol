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
use conrod::Dimensions;
use time::Duration;
use piston_window::keyboard::Key;

use geometry::*;
use stellar_bodies::*;

pub struct ConquestState{
    ids:Ids,
    camera:Camera,
    systems:Vec<System>
}
struct Camera{
    position:Position, // position in world coordinates (AU)
    width:Au, // in astromical units
    height:Au,
}
impl Camera{
    fn worldToScreen(&self, screenSize:&Dimensions, position:Position) -> Position{
        println!("screensize {:?}", screenSize);
        let factor = Position::new(screenSize[0], screenSize[1]) / Position::new(self.width, self.height);
        (position + self.position) * factor
    }
    fn update(&self, ui:&mut conrod::UiCell, screenSize:&Dimensions, systems:&Vec<System>){
        use conrod::widget::Circle;
        use conrod::{Positionable, Widget};
        use conrod::Colorable;
        let camrect = Rectangle{
            one: Position{
                x:self.position.x-self.width/2.0,
                y:self.position.y-self.height/2.0,
            },
            two: Position{
                x:self.position.x+self.width/2.0,
                y:self.position.y+self.height/2.0,
            }
        };
        // cull the ones outside view, (and ignoring their sub bodies)
        let visible = systems.iter().filter(|x| -> bool {
            let disk = Disk{
                position:x.position,
                radius:x.radius};
            camrect.contains(&x.position) ||
            disk.contains([camrect.tl(), camrect.tr()]) ||
            disk.contains([camrect.tr(), camrect.br()]) ||
            disk.contains([camrect.br(), camrect.bl()]) ||
            disk.contains([camrect.bl(), camrect.tl()])
        }
        ).flat_map(|x| &x.bodies);
        for body in visible{
            let nextId = {
                let mut generator = ui.widget_id_generator();
                generator.next()
            };
            let position = self.worldToScreen(screenSize, body.calcPosition(Duration::zero()));
            println!("{}", position);
            Circle::fill(5.0).x(position.x).y(position.y).color(conrod::color::WHITE).set(nextId, ui);
        }
    }
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
            camera:Camera{position:center, width:2.0, height:2.0},
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
