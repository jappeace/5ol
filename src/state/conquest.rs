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

use state::state_machine::{State, StateChange, StateEvent};
use piston_window::Input;
use conrod;
use time::Duration;

use geometry::*;
use model::*;
use camera::Camera;
use async::pulser::Pulser;
use async::logic_updater::Updater;
use async::model_access::Change;
use async::thread_status::Status;

pub struct ConquestState{
    ids:Ids,
    camera:Camera,
    updater:Updater,
    pulser:Pulser,
}
impl State for ConquestState{
    fn enter(&mut self, _:Box<State>) -> StateChange{
        self.updater.start();
        self.pulser.start();
        None
    }
    fn poll_event(&self) -> StateEvent{
        self.pulser.get_event()
    }
    fn update(&mut self, ui:&mut conrod::UiCell) ->  StateChange{
        use conrod::{color, widget, Colorable, Widget, Positionable, Labelable, Sizeable};
        use conrod::widget::Button;
        let model = self.updater.model_writer.copy_model();
        let canvas = widget::Canvas::new();
        canvas
            .color(color::BLUE)
            .crop_kids()
            .set(self.ids.canvas_root, ui) ;
        let dimens = ui.window_dim();
        let time = model.time;
        println!("update {:?}, {}", time, time.num_weeks());
        let projection = self.camera.create_projection(&dimens);
        let visible = model.galaxy.iter()
            .filter(|x| projection.is_visible(&x.used_space))
            .flat_map(|x| &x.bodies);

        use state::planet::PlanetState;
        for body in visible{
            let view_id = match body.view_id{
                None => {
                    let newid = ui.widget_id_generator().next();
                    self.updater.model_writer.enqueue(Change::BodyViewID(body.address, Some(newid)));
                    newid
                }
                Some(x) => x
            };
            let position = projection.world_to_screen(&body.calc_position(&time));
            for _ in Button::new().w_h(10.0,10.0).x(position.x).y(position.y).set(view_id, ui){
                return Some(Box::new(PlanetState::new(
                    ui.widget_id_generator(),
                    body.address,
                    self.updater.model_writer.clone()
                )));
            }
        }

        
        let pausedlabel = match self.updater.controll.get_status(){
            Status::Paused => ">",
            _ => "❚❚"
        };
        for _ in widget::Button::new()
            .w_h(30.0,30.0)
            .top_right_with_margin_on(self.ids.canvas_root, 10.0)
            .label(pausedlabel)
            .color(color::DARK_CHARCOAL)
            .label_color(color::GRAY)
            .set(self.ids.button_pause, ui){
                self.updater.controll.toggle_pause();
        }

        let mut previous = self.ids.button_pause;
        for &(label, speed, id) in [
            ("1>", 2000, self.ids.button_speed_one),
            ("2>", 500, self.ids.button_speed_two),
            ("3>", 200, self.ids.button_speed_three),
            ("4>", 50, self.ids.button_speed_four),
            ("5>", 0, self.ids.button_speed_five)
        ].iter(){
            for _ in widget::Button::new()
                .w_h(30.0,30.0)
                .left_from(previous,10.0)
                .label(label)
                .color(color::DARK_CHARCOAL)
                .label_color(color::GRAY)
                .set(id, ui){
                    self.updater.controll.set_pace(speed);
                }
            previous = id;
        }
        for _ in widget::Button::new()
            .w_h(30.0,30.0)
            .down_from(self.ids.button_pause, 10.0)
            .align_right_of(self.ids.button_pause)
            .label("w")
            .color(color::DARK_CHARCOAL)
            .label_color(color::GRAY)
            .set(self.ids.button_granu_weeks, ui){
                self.updater.set_granuality(Duration::weeks);
            }
        previous = self.ids.button_granu_weeks;
        let buttons:[(&'static str, fn(i64)->Duration, _); 5] = [
            ("d", Duration::days, self.ids.button_granu_days),
            ("h", Duration::hours, self.ids.button_granu_hours),
            ("m", Duration::minutes, self.ids.button_granu_minutes),
            ("s", Duration::seconds, self.ids.button_granu_seconds),
            ("ms", Duration::milliseconds, self.ids.button_granu_milliseconds),
        ];
        for &(label, function, id) in buttons.iter(){
            for _ in widget::Button::new()
                .w_h(30.0,30.0)
                .left_from(previous,10.0)
                .down_from(self.ids.button_pause, 10.0)
                .label(label)
                .color(color::DARK_CHARCOAL)
                .label_color(color::GRAY)
                .set(id, ui){
                    self.updater.set_granuality(function);
                }
            previous = id;
        }
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
                Space => self.updater.controll.toggle_pause(),
                _ => {}
            },
            _ => {}
        }
        None
        }
    fn exit(&mut self){
        // kill the threads
        println!("exiting conquest state");
        self.updater.stop();
        self.pulser.controll.stop();
    }
}

impl ConquestState{
    pub fn new_game(generator: conrod::widget::id::Generator) -> ConquestState{
        use std::sync::{Arc, RwLock};
        let mut earth = StellarBody::new("earth",Duration::days(365),1.0,1.0);
        earth.population = Some(Population{
            owner:0,
            head_count:7456000000
        });
        ConquestState::new(generator, Camera::new(center,2.0,2.0), Arc::new(RwLock::new(GameModel::new(vec![
            System::new(
                center,
                vec![
                    StellarBody::create_single_star("sun"),
                    StellarBody::new("mercury", Duration::days(88), 0.387098, 0.147),
                    StellarBody::new("venus", Duration::days(225),0.723332, 0.902),
                    earth,
                    StellarBody::new("mars",Duration::days(780),1.523679, 0.284),
                ]
            )
        ]
        ))))
    }
    pub fn new(generator: conrod::widget::id::Generator, start_cam:Camera, start_model:World)->ConquestState{
        ConquestState{
            ids:Ids::new(generator),
            camera:start_cam,
            updater:Updater::new(start_model, Duration::days),
            pulser:Pulser::new(StateEvent::Idle)
        }
    }
}

widget_ids! {
    Ids {
        canvas_root,
        button_pause,
        button_speed_one,
        button_speed_two,
        button_speed_three,
        button_speed_four,
        button_speed_five,
        button_granu_weeks,
        button_granu_days,
        button_granu_hours,
        button_granu_minutes,
        button_granu_seconds,
        button_granu_milliseconds,
    }
}
