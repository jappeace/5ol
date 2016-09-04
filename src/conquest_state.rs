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

use state_machine::{State, StateChange};
use piston_window::Input;
use conrod;
use time::Duration;
pub struct ConquestState{
    ids:Ids,
    camera:Camera,
    systems:Vec<System>
}
struct Position{
    x:f64,
    y:f64
}
use std::ops::{Add, Sub, Neg};
impl Position{
    fn dot(&self, with:Position) -> f64{
        self.length() * with.length() * self.angleRad(with).cos()
    }
    fn lengthSq(&self)->f64{
        self.x*self.x+self.y*self.y
    }
    fn length(&self)->f64{
        self.lengthSq().sqrt()
    }
    fn angleRad(&self, other:Position) -> f64{
        (self.y - other.y).atan2(self.x - other.x)
    }
}
impl Add for Position{
    type Output = Position;
    fn add(self, _rhs: Position) -> Position{   
        Position{x:_rhs.x+self.x, y:_rhs.y+self.y}
    }
}
impl Sub for Position{
    type Output = Position;
    fn sub(self, _rhs: Position) -> Position{   
        Position{x:_rhs.x-self.x, y:_rhs.y-self.y}
    }
}
impl Neg for Position{
    type Output = Position;
    fn neg(self) -> Position{   
        Position{x:-self.x, y:-self.y}
    }
}
const center:Position = Position{x:0.0,y:0.0};
type Line = [Position;2];
struct Disk{
    position:Position,
    radius:f64
}
impl Disk{
    fn contains(&self, line:Line) -> bool{
        let d = line[0] - line[1];
        let f = line[1] - self.position;
        let r = self.radius;
        let a = d.dot( d ) ;
        let b = 2.0*f.dot( d ) ;
        let c = f.dot( f ) - r*r ;

        let discriminant = b*b-4.0*a*c;
        discriminant >= 0.0
    }
}
struct Rectangle {
    one:Position,
    two:Position
}
impl Rectangle{
    fn contains(&self, position:&Position) -> bool{
        let tl = Position{
            x: if self.one.x < self.two.x {self.one.x} else {self.two.x},
            y: if self.one.y > self.two.y {self.one.y} else {self.two.y}
        };
        let br = Position{
            x: if self.one.x > self.two.x {self.one.x} else {self.two.x},
            y: if self.one.y < self.two.y {self.one.y} else {self.two.y}
        };
        tl.x < position.x && tl.y < position.y && br.x > position.x && br.y > position.y
    }
}
type Au = f64;
trait StellarBody{
    fn calcPosition(&self, sinceStartOfSimulation:Duration) -> Position{
        let orbitTime:i64 = self.getOrbitTime().num_seconds();
        if orbitTime == 0 {
            // prevents division by 0
            return center;
        }
        let cycleProgress:i64 = sinceStartOfSimulation.num_seconds() % orbitTime;
        let progressFraction:f64 = (cycleProgress as f64)/(orbitTime as f64);
        Position{
            x: progressFraction.sin() * self.getOrbitDistance(),
            y: progressFraction.cos() * self.getOrbitDistance()
        }
    }
    fn getOrbitTime(&self) -> Duration;
    fn getOrbitDistance(&self) -> Au;
}
// a single star is the center of a system
struct SingleStar;
// something that has an orbit other than 0
struct Orbital{
    orbitTime:Duration,
    distance:Au
}
impl StellarBody for SingleStar{
    fn getOrbitTime(&self) -> Duration{Duration::zero()}
    fn getOrbitDistance(&self) -> Au{0.0}
}
impl StellarBody for Orbital{
    fn getOrbitTime(&self) -> Duration{self.orbitTime}
    fn getOrbitDistance(&self) -> Au {self.distance}
}
struct System{
    position:Position,
    radius:Au, // allows quick filtering
    bodies:Vec<Box<StellarBody>>,
}
struct Camera{
    position:Position,
    width:Au, // in astromical units
    height:Au,
}
use piston_window::keyboard::Key;
impl Camera{
    fn update(&self, ui:&mut conrod::UiCell, systems:&Vec<System>){

        visible = systems.iter().filter(|x| x.);
    }
}
impl ConquestState{
    pub fn new(mut generator: conrod::widget::id::Generator)->ConquestState{
        ConquestState{
            ids:Ids::new(generator),
            camera:Camera{position:center, width:2.0, height:2.0},
            systems:vec![
                System{
                    position:center,
                    bodies:vec![
                        Box::new(SingleStar),
                        Box::new(Orbital{
                            orbitTime:Duration::days(365),
                            distance:1.0
                        })
                    ]
                }
            ]
        }
    }
}
impl State for ConquestState{
    
    fn update(&mut self, ui:&mut conrod::UiCell) ->  StateChange{
        use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget, Scalar};
        use conrod::widget::Line;
        widget::Canvas::new().color(color::BLUE).set(self.ids.canvas_root, ui);
        self.camera.update(ui, self.systems);
        None
    }
    fn input(&mut self, input:Input) -> StateChange{
        use piston_window::Input::*;
        use piston_window::Button::*;
        use piston_window::keyboard::Key::*;
        match input {
            Press(Keyboard(key)) => match key {
                W => self.camera.position.y += 0.1,
                S => self.camera.position.y -= 0.1,
                D => self.camera.position.x += 0.1,
                A => self.camera.position.x -= 0.1,
                _ => {}
            },
            _ => {}
        }
        None
    }
}

widget_ids! {
    Ids {
        canvas_root,
    }
}
