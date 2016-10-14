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
use model::galaxy::{Au, BodyAddress};
use conrod::Dimensions;

pub enum Direction{
    In,
    Out
}
const zoom_factor:f64 = 2.0;
pub const move_step:f64 = 0.05;
pub struct Camera{
    last_screensize:Dimensions,
    mouse_position:Position, // world coordinates
    pub position:Position, // position in world coordinates (AU)
    pub width:Au, // in astromical units
    pub height:Au,
    pub track_body:Option<BodyAddress>
}
const init_dimensions:Dimensions = [0.0,0.0];
impl Camera{
    pub fn new(position:Position, width:Au, height:Au)->Camera{
        Camera{
            position:position,
            width:width,
            height:height,
            last_screensize:init_dimensions,
            mouse_position:center,
            track_body:None
        }
    }
    pub fn stop_tracking(&mut self){
        self.track_body = None;
    }
    pub fn record_mouse(&mut self, position:Dimensions){
        let screensize = self.last_screensize;
        let projection = self.create_projection(&screensize);

        self.mouse_position = projection.screen_to_world(Position::new(position[0] - screensize[0]/2.0, position[1] - screensize[1]/2.0));
    }
    pub fn zoom(&mut self, direction:Direction){
        let screen_size = self.last_screensize.clone();
        let desired_fixed_point = self.create_projection(&screen_size)
            .world_to_screen(self.mouse_position);
        match direction{
            Direction::In =>{
                self.width /= zoom_factor;
                self.height /= zoom_factor;
            }
            Direction::Out =>{
                self.width *= zoom_factor;
                self.height *= zoom_factor;
            }

        }
        let fixed_move = self.create_projection(&screen_size)
            .screen_to_world(desired_fixed_point);
        let movement = fixed_move - self.mouse_position;
        println!("move {}, mouse {}", movement, self.mouse_position);
        self.position += movement * Position::new(-1.0,1.0);
    }
    pub fn create_projection<'a>(&mut self, screen_size:&'a Dimensions) -> Projection<'a>{
        self.last_screensize = screen_size.clone();
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
    pub fn get_screen_viewport_ratio(&self) -> Position{
        Position::new(self.screen_size[0], self.screen_size[1]) /
            Position::new(self.view_port.width(), self.view_port.height())
    }
    pub fn screen_to_world(&self, position:Position)->Position{
        let ratio = self.get_screen_viewport_ratio();
        self.view_port.center() - position / ratio
    }
    pub fn world_to_screen(&self, position:Position)->Position{
        (position + self.view_port.center()) * self.get_screen_viewport_ratio()
    }
    pub fn is_visible(&self, disk:&Disk) -> bool{
        let (tl, tr, bl, br) = self.view_port.corners();
        self.is_pos_visible(&disk.position) ||
            disk.contains([tl, tr]) ||
            disk.contains([tr, br]) ||
            disk.contains([br, bl]) ||
            disk.contains([bl, tl])
    }
    pub fn is_pos_visible(&self, pos:&Position) -> bool{
        self.view_port.contains(&pos)
    }
}

#[cfg(test)]
mod tests{
    use camera::Camera;
    use geometry::*;
    #[test]
    fn projection_idompotency(){
        let mut cam = Camera::new(Position{x:3.0, y:59.3}, 4.0, 2.1);
        let some_screensize = [100.0, 250.3]; // floating point may make epsilon differences
        let projection = cam.create_projection(&some_screensize);
        let some_point = Position::new(49.0, -239.5);
        assert_eq!(
            projection.screen_to_world(projection.world_to_screen(some_point)),
            some_point
        );
        assert_eq!(
            projection.world_to_screen(projection.screen_to_world(some_point)),
            some_point
        )
    }
}
