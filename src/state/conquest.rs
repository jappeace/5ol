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
use chrono::Duration;

use geometry::*;
use model::root::GameModel;
use model::colony::*;
use model::galaxy::*;
use camera::{Camera, Direction, move_step};
use async::pulser::Pulser;
use async::logic_updater::Updater;
use async::model_access::Change;
use async::thread_status::Status;
use std::sync::{Arc, RwLock};

pub struct ConquestState{
    ids:Ids,
    camera:Camera,
    updater:Updater,
    pulser:Pulser,
}
impl State for ConquestState{
    fn enter(&mut self, _:Box<State>) -> StateChange{
        self.updater.start();
        self.updater.controll.set_status(Status::Paused);
        self.pulser.start();
        None
    }
    fn poll_event(&self) -> StateEvent{
        self.pulser.get_event()
    }
    fn update(&mut self, ui:&mut conrod::UiCell) ->  StateChange{
        use conrod::{color, widget, Colorable, Widget, Positionable, Labelable, Sizeable};
        use conrod::widget::{Button, Oval};
        let model = self.updater.model_writer.copy_model();
        let canvas = widget::Canvas::new();
        canvas
            .color(color::BLUE)
            .crop_kids()
            .set(self.ids.canvas_root, ui) ;
        let dimens = ui.window_dim();
        let time = model.time;

        self.camera.position = self.camera.track_body.map_or(
            self.camera.position,
            |x| x.get_body(&model.galaxy).calc_position(&time) * Position::new(-1.0,-1.0)
        );

        let projection = self.camera.create_projection(&dimens);
        let visible = model.galaxy.iter()
            //.filter(|x| projection.is_visible(&x.used_space))
            .flat_map(|x| &x.bodies);


        use state::planet::PlanetState;
        for body in visible{
            let view_id = match body.view_id{
                None => {
                    let newid = ui.widget_id_generator().next();
                    self.updater.enqueue(Change::BodyViewID(body.address, Some(newid)));
                    newid
                }
                Some(x) => x
            };
            let body_position = body.calc_position(&time);
            let position = projection.world_to_screen(body_position);
            Oval::fill([10.0,10.0]).x(position.x).y(position.y).set(view_id, ui);
            let mut should_return = false;
            {
                let input = ui.widget_input(view_id);
                if let Some(ref mouse) = input.mouse(){
                    let buttons = mouse.buttons;
                    use conrod::input::state::mouse::ButtonPosition;
                    if let ButtonPosition::Down(_, _) = *buttons.left(){
                        should_return = true;
                    }
                    if let ButtonPosition::Down(_, _) = *buttons.right(){
                        self.camera.track_body = Some(body.address);
                    }
                }
            }
            if should_return{
                return Some(Box::new(PlanetState::new(
                    ui.widget_id_generator(),
                    body.address,
                    self.updater.model_writer.clone()
                )));
            }
        }


        use conrod::Color;
        for ship in model.ships
            .iter()
            .map(|x| (x, projection.world_to_screen(x.movement.calc_position(&model.time, &model.galaxy))))
            {
                println!("shippp! {}, ({},{})", ship.0.id, ship.1.x, ship.1.y);
                Oval::fill([5.0,5.0])
                    .x(ship.1.x).y(ship.1.y).color(Color::Rgba(0.0,0.0,0.0,1.0))
                    .set(ship.0.view.map_or_else(
                        || {
                            let result = ui.widget_id_generator().next();
                            self.updater.enqueue(
                                Change::ShipViewID(ship.0.id,Some(result))
                            );
                            result
                        },
                        |x| x),ui)
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

        let money = format!("money: {} \n time: {}", model.players[0].money, time.num_weeks());
        widget::Text::new(&money)
            .color(color::LIGHT_RED)
            .top_left_with_margin_on(self.ids.canvas_root, 10.0)
            .align_text_left()
            .line_spacing(10.0)
            .set(self.ids.text_money, ui);
        None
    }
    fn input(&mut self, input:Input) -> StateChange{
        use piston_window::Input::*;
        use piston_window::Button::*;
        use piston_window::keyboard::Key::*;
        use piston_window::Motion::{MouseScroll,MouseCursor};
        let size = self.camera.get_size();
        match input {
            Press(Keyboard(key)) => match key {
                W =>{
                    self.camera.position.y -= move_step * size[1];
                    self.camera.stop_tracking();
                },
                S => {
                    self.camera.position.y += move_step * size[1];
                    self.camera.stop_tracking();
                },
                D =>{
                    self.camera.position.x -= move_step * size[0];
                    self.camera.stop_tracking();
                },
                A => {
                    self.camera.position.x += move_step * size[0];
                    self.camera.stop_tracking();
                },
                Space => self.updater.controll.toggle_pause(),
                Return => {
                    self.camera.position = center;
                    self.camera.stop_tracking();
                },
                _ => {}
            },
            Move(MouseCursor(x, y)) => self.camera.record_mouse([x,y]),
            Move(MouseScroll(_, direction)) => self.camera.zoom(
                if direction == 1.0 {Direction::In}else{Direction::Out}
            ) ,
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
        let earth = StellarBody::new(
            BodyClass::Rocky(
                Colony::new_inhabited(
                    0,
                    1.0,
                    Population::new(
                        7456000000
                    )
                )
            ),
            "earth",
            Duration::days(365),
            1.0
        );
        ConquestState::new(generator, Camera::new(center,2.0,2.0), Arc::new(RwLock::new(GameModel::new(vec![
            System::new(
                center,
                vec![
                    StellarBody::create_single_star("sun"),
                    StellarBody::new(
                        BodyClass::Rocky(
                            Colony::new_empty(0.147)
                        ),
                        "mercury",
                        Duration::days(88),
                        0.387098
                    ),
                    StellarBody::new(
                        BodyClass::Rocky(
                            Colony::new_empty(0.902)
                        ),
                        "venus",
                        Duration::days(225),
                        0.723332
                    ),
                    earth,
                    StellarBody::new(
                        BodyClass::Rocky(
                            Colony::new_empty(0.284)
                        ),
                        "mars",
                        Duration::days(780),
                        1.523679,
                    ),
                    StellarBody::new(BodyClass::GasGiant, "jupiter", Duration::days(4333), 5.20260),
                    StellarBody::new(BodyClass::GasGiant, "saturn", Duration::days(10759), 9.554909),
                    StellarBody::new(BodyClass::GasGiant, "uranus", Duration::days(30688), 19.2184),
                    StellarBody::new(BodyClass::GasGiant, "neptune", Duration::days(60182), 30.110387),
                ]
            )
        ]
        ))))
    }
    pub fn new(generator: conrod::widget::id::Generator, start_cam:Camera, start_model:Arc<RwLock<GameModel>>)->ConquestState{
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
        text_money,
    }
}
