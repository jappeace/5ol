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


// this file describes some geometric constructs, the main unit being position.
// I wrote my own to use f64 instead of f32, simply because the entire milky
// way sortoff fits in that if we use AU as a unit
#[derive(Copy,Clone)]
pub struct Position{
    // left = 0
    pub x:f64,
    // top = 0
    pub y:f64
}
use std::ops::{Add, Sub, Neg, Div, Mul};
impl Position{
    pub fn new(x:f64,y:f64) -> Position{
        Position{
            x:x,
            y:y
        }
    }
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
use std::fmt;
impl fmt::Display for Position{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl Add for Position{
    type Output = Position;
    fn add(self, _rhs: Position) -> Position{   
        Position{x:_rhs.x+self.x, y:_rhs.y+self.y}
    }
}
impl Mul for Position{
    type Output = Position;
    fn mul(self, _rhs:Position) -> Position{
        Position::new(self.x*_rhs.x,self.y*_rhs.y)
    }
}
impl Div for Position{
    type Output = Position;
    fn div(self, _rhs:Position) -> Position{
        Position::new(self.x/_rhs.x,self.y/_rhs.y)
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

type Line = [Position;2];

pub struct Disk{
    pub position:Position,
    pub radius:f64
}
impl Disk{
    pub fn contains(&self, line:Line) -> bool{
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

// give me two points and I give you a rectangle
pub struct Rectangle {
    pub one:Position,
    pub two:Position
}
impl Rectangle{
    pub fn contains(&self, position:&Position) -> bool{
        let tl = self.tl();
        let br = self.br();
        tl.x < position.x && tl.y < position.y && br.x > position.x && br.y > position.y
    }
    pub fn tl(&self) -> Position{
        Position{
            x: if self.one.x < self.two.x {self.one.x} else {self.two.x},
            y: if self.one.y > self.two.y {self.one.y} else {self.two.y}
        }
    }
    pub fn tr(&self) -> Position{
        Position{
            x: if self.one.x > self.two.x {self.one.x} else {self.two.x},
            y: if self.one.y > self.two.y {self.one.y} else {self.two.y}
        }
    }
    pub fn br(&self) -> Position{
        Position{
            x: if self.one.x > self.two.x {self.one.x} else {self.two.x},
            y: if self.one.y < self.two.y {self.one.y} else {self.two.y}
        }
    }
    pub fn bl(&self) -> Position{
        Position{
            x: if self.one.x < self.two.x {self.one.x} else {self.two.x},
            y: if self.one.y < self.two.y {self.one.y} else {self.two.y}
        }
    }
}

pub const center:Position = Position{x:0.0,y:0.0};
