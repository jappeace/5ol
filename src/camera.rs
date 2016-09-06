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


// does the world to screen mapping (and also renders
// the bodies right now, this will probably change when rendering becomes more 
// complex)

use geometry::*;
use stellar_bodies::*;
use conrod::Dimensions;
use conrod;
use time::Duration;

pub struct Camera{
    pub position:Position, // position in world coordinates (AU)
    width:Au, // in astromical units
    height:Au,
}
impl Camera{
    fn worldToScreen(&self, screenSize:&Dimensions, position:Position) -> Position{
        let factor = Position::new(screenSize[0], screenSize[1]) / Position::new(self.width, self.height);
        (position + self.position) * factor
    }
    pub fn new(position:Position, width:Au, height:Au)->Camera{
        Camera{position:position, width:2.0, height:2.0}
    }
    pub fn update(&self, ui:&mut conrod::UiCell, screenSize:&Dimensions, systems:&Vec<System>, time:&Duration){
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
            let position = self.worldToScreen(screenSize, body.calcPosition(time));
            Circle::fill(5.0).x(position.x).y(position.y).color(conrod::color::WHITE).set(nextId, ui);
        }
    }
}
