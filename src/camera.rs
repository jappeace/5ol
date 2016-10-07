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


// does the world to screen mapping, ie determine where a world coordinate
// (au,au), should be rendered in (px,px)

use geometry::*;
use model::galaxy::Au;
use conrod::Dimensions;

pub struct Camera{
    pub position:Position, // position in world coordinates (AU)
    width:Au, // in astromical units
    height:Au,
}
impl Camera{
    pub fn new(position:Position, width:Au, height:Au)->Camera{
        Camera{position:position, width:width, height:height}
    }
    pub fn create_projection<'a>(&self, screen_size:&'a Dimensions) -> Projection<'a>{
        let two = 2.0;
        Projection{
            view_port:Rectangle{
                one: Position{
                    x:self.position.x-self.width/two,
                    y:self.position.y-self.height/two,
                },
                two: Position{
                    x:self.position.x+self.width/two,
                    y:self.position.y+self.height/two,
                }
            },
            screen_size:screen_size
        }
    }
}
pub struct Projection<'a>{
    view_port:Rectangle,
    screen_size:&'a Dimensions

}
impl<'a> Projection<'a>{
    pub fn world_to_screen(&self, position:&Position)->Position{
        let factor = Position::new(self.screen_size[0], self.screen_size[1]) /
            Position::new(self.view_port.width(), self.view_port.height());
        (position.clone() + self.view_port.center()) * factor
    }
    pub fn is_visible(&self, disk:&Disk) -> bool{
        let (tl, tr, bl, br) = self.view_port.corners();
        self.view_port.contains(&disk.position) ||
            disk.contains([tl, tr]) ||
            disk.contains([tr, br]) ||
            disk.contains([br, bl]) ||
            disk.contains([bl, tl])
    }
}
