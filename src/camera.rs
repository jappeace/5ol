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

use conrod::Dimensions;
use crate::model::galaxy::{Au, BodyAddress};
use crate::geometry::{Position, Rectangle, Disk};

pub enum ZoomDirection {
    In,
    Out,
}
pub enum MoveDirection {
    Left,
    Right,
    Up,
    Down,
}
const zoom_factor: f64 = 2.0;
const move_step: f64 = 0.05;
pub const start_cam_width: Au = 2.0;
pub const start_cam_height: Au = 2.0;
pub struct Camera {
    pub position: Position, // position in world coordinates (AU)
    pub width: Au,          // in astromical units
    pub height: Au,
    pub track_body: Option<BodyAddress>,
}
impl Camera {
    pub fn new(position: Position, width: Au, height: Au) -> Camera {
        Camera {
            position: position,
            width: width,
            height: height,
            track_body: None,
        }
    }
    pub fn stop_tracking(&mut self) {
        self.track_body = None;
    }
    pub fn translate(&mut self, direction: MoveDirection) {
        let zero = 0.0;
        let movement = match direction {
            MoveDirection::Left => Position::new(move_step, zero),
            MoveDirection::Right => Position::new(-move_step, zero),
            MoveDirection::Up => Position::new(zero, -move_step),
            MoveDirection::Down => Position::new(zero, move_step),
        };
        let scaled_movement = movement * Position::new(self.width, self.height);
        self.position += scaled_movement;
        self.stop_tracking();
    }
    pub fn zoom(
        &mut self,
        screen_size: Dimensions,
        direction: ZoomDirection,
        mouse_position: Position,
    ) {
        let mouse = mouse_position - (Position::arr(screen_size) / Position::i(2));
        let desired_fixed_point = self.create_projection(screen_size).screen_to_world(mouse);

        match direction {
            ZoomDirection::In => {
                self.width /= zoom_factor;
                self.height /= zoom_factor;
            }
            ZoomDirection::Out => {
                self.width *= zoom_factor;
                self.height *= zoom_factor;
            }
        }

        let fixed_move = self.create_projection(screen_size).screen_to_world(mouse);
        let correction_movement = fixed_move - desired_fixed_point;

        println!(
            "move {}, mouse {}",
            correction_movement, desired_fixed_point
        );
        self.position += correction_movement * Position::new(-1.0, 1.0);
    }
    pub fn create_projection(&self, screen_size: Dimensions) -> Projection {
        let two = 2.0;
        Projection {
            view_port: Rectangle {
                one: Position {
                    x: self.position.x - self.width / two,
                    y: self.position.y - self.height / two,
                },
                two: Position {
                    x: self.position.x + self.width / two,
                    y: self.position.y + self.height / two,
                },
            },
            screen_size: screen_size,
        }
    }
}
pub struct Projection {
    pub view_port: Rectangle,
    screen_size: Dimensions,
}
impl Projection {
    pub fn get_screen_viewport_ratio(&self) -> Position {
        Position::new(self.screen_size[0], self.screen_size[1])
            / Position::new(self.view_port.width(), self.view_port.height())
    }
    pub fn screen_to_world(&self, position: Position) -> Position {
        let ratio = self.get_screen_viewport_ratio();
        (position) / ratio - self.view_port.center()
    }
    pub fn world_to_screen(&self, position: Position) -> Position {
        (position - self.view_port.center()) * self.get_screen_viewport_ratio()
    }
    pub fn is_visible(&self, disk: &Disk) -> bool {
        let (tl, tr, bl, br) = self.view_port.corners();
        self.is_pos_visible(&disk.position)
            || disk.contains([tl, tr])
            || disk.contains([tr, br])
            || disk.contains([br, bl])
            || disk.contains([bl, tl])
    }
    pub fn is_pos_visible(&self, pos: &Position) -> bool {
        let result = self.view_port.contains(pos);
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::camera::*;
    use chrono::Duration;

    #[test]
    fn projection_idompotency() {
        let mut cam = Camera::new(Position { x: 3.0, y: 59.3 }, 4.0, 2.1);
        let some_screensize = [100.0, 250.3]; // floating point may make epsilon differences
        let projection = cam.create_projection(some_screensize);
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
    #[test]
    fn ships_should_be_visible_in_start_location() {
        // there is an annoying filter bug where ships are only sometimes
        // visible on the lower zoom levels
        // its really hard to test ingame so this unit test does it
        // programaticially
        let gamemodel = {
            let ship = {
                let mut s = Ship::new(
                    0,
                    0,
                    BodyAddress {
                        system_id: 0,
                        planet_id: 0,
                    },
                );
                s.id = 0;
                s
            };
            let mut model = GameModel::new(vec![System::new(
                center,
                vec![StellarBody::new_earthlike("earth")],
            )]);
            model.ships = vec![ship];
            model
        };
        let mut camera = Camera::new(center, start_cam_width, start_cam_height);
        let projection = camera.create_projection([300.0, 600.0]);

        let some_visible_days = [45, 135, 215, 305, 345];
        for day in some_visible_days.iter() {
            let position = gamemodel.ships[0]
                .movement
                .calc_position(&Duration::days(*day), &gamemodel.galaxy);
            println!("position {} for day {}", position, day);
            assert!(projection.is_pos_visible(&position))
        }
    }
}
