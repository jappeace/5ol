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
use piston_window::Button;
use piston_window::ButtonArgs;
use piston_window::MouseButton;
use piston_window::Button::Keyboard;
use piston_window::keyboard::Key::*;
use piston_window::Motion::{MouseScroll, MouseCursor};
use conrod;
use conrod::Dimensions;
use chrono::Duration;
use std::sync::{Arc, RwLock};

use geometry::*;
use model::root::*;
use model::colony::*;
use model::galaxy::*;
use model::ship::*;
use camera::*;
use async::pulser::Pulser;
use async::logic_updater::Updater;
use async::model_access::Change;
use async::thread_status::Status;
use state::planet::PlanetState;
use view::map_entities::{MapRenderer, View};

pub struct ConquestState {
    ids: Ids,
    camera: Camera,
    updater: Updater,
    pulser: Pulser,
    player_id: PlayerID,
    map_renderer: MapRenderer,
    last_mouse_position: Position,
    drag_mouse_start: Option<Position>,
    last_screen_size: Dimensions,
}
impl State for ConquestState {
    fn enter(&mut self, _: Box<State>) -> StateChange {
        self.updater.start();
        self.updater.controll.set_status(Status::Paused);
        self.pulser.start();
        None
    }
    fn poll_event(&self) -> StateEvent {
        self.pulser.get_event()
    }
    fn update(&mut self, ui: &mut conrod::UiCell) -> StateChange {
        use conrod::{color, widget, Colorable, Widget, Positionable, Labelable, Sizeable};
        self.last_screen_size = ui.window_dim();
        let canvas = widget::Canvas::new();
        canvas.color(color::BLUE)
            .crop_kids()
            .set(self.ids.canvas_root, ui);

        let model = self.updater.model_writer.copy_model();
        let time = model.time;

        if let Some(rect) = self.ceate_dragtengle_maybe() {
            let corner = rect.center() - Position::arr(self.last_screen_size) / Position::i(2);
            conrod::widget::Rectangle::outline([rect.width(), rect.height()])
                .x(-corner.x)
                .y(corner.y)
                .set(self.ids.rect_select, ui);
        }

        self.camera.position = self.camera.track_body.map_or(self.camera.position, |x| {
            model.galaxy[x].calc_position(&time)
        });

        let projection = self.camera.create_projection(self.last_screen_size);

        self.map_renderer.render(ui, &projection, &model);
        for (body_address, view_id) in
            self.map_renderer
                .planets
                .map
                .iter()
                .filter_map(|kv| if let Some(view_id) = kv.1.get_view_id() {
                    Some((kv.0, view_id))
                } else {
                    None
                }) {
            let mut should_return = false;
            {
                let input = ui.widget_input(view_id);
                if let Some(ref mouse) = input.mouse() {
                    let buttons = mouse.buttons;
                    use conrod::input::state::mouse::ButtonPosition;
                    if let ButtonPosition::Down(_, _) = *buttons.left() {
                        should_return = true;
                    }
                    if let ButtonPosition::Down(_, _) = *buttons.right() {
                        self.camera.track_body = Some(body_address.clone());
                    }
                }
            }
            if should_return {
                return Some(Box::new(PlanetState::new(ui.widget_id_generator(),
                                                      body_address.clone(),
                                                      self.updater.model_writer.clone())));
            }
        }

        let pausedlabel = match self.updater.controll.get_status() {
            Status::Paused => ">",
            _ => "❚❚",
        };
        for _ in widget::Button::new()
            .w_h(30.0, 30.0)
            .top_right_with_margin_on(self.ids.canvas_root, 10.0)
            .label(pausedlabel)
            .color(color::DARK_CHARCOAL)
            .label_color(color::GRAY)
            .set(self.ids.button_pause, ui) {
            self.updater.controll.toggle_pause();
        }

        let mut previous = self.ids.button_pause;
        for &(label, speed, id) in
            [("1>", 2000, self.ids.button_speed_one),
             ("2>", 500, self.ids.button_speed_two),
             ("3>", 200, self.ids.button_speed_three),
             ("4>", 50, self.ids.button_speed_four),
             ("5>", 0, self.ids.button_speed_five)]
                .iter() {
            for _ in widget::Button::new()
                .w_h(30.0, 30.0)
                .left_from(previous, 10.0)
                .label(label)
                .color(color::DARK_CHARCOAL)
                .label_color(color::GRAY)
                .set(id, ui) {
                self.updater.controll.set_pace(speed);
            }
            previous = id;
        }
        for _ in widget::Button::new()
            .w_h(30.0, 30.0)
            .down_from(self.ids.button_pause, 10.0)
            .align_right_of(self.ids.button_pause)
            .label("w")
            .color(color::DARK_CHARCOAL)
            .label_color(color::GRAY)
            .set(self.ids.button_granu_weeks, ui) {
            self.updater.set_granuality(Duration::weeks);
        }
        previous = self.ids.button_granu_weeks;
        let buttons: [(&'static str, fn(i64) -> Duration, _); 5] =
            [("d", Duration::days, self.ids.button_granu_days),
             ("h", Duration::hours, self.ids.button_granu_hours),
             ("m", Duration::minutes, self.ids.button_granu_minutes),
             ("s", Duration::seconds, self.ids.button_granu_seconds),
             ("ms", Duration::milliseconds, self.ids.button_granu_milliseconds)];
        for &(label, function, id) in buttons.iter() {
            for _ in widget::Button::new()
                .w_h(30.0, 30.0)
                .left_from(previous, 10.0)
                .down_from(self.ids.button_pause, 10.0)
                .label(label)
                .color(color::DARK_CHARCOAL)
                .label_color(color::GRAY)
                .set(id, ui) {
                self.updater.set_granuality(function);
            }
            previous = id;
        }

        let money = format!("money: {} \n time: {}",
                            model.players[0].money,
                            time.num_weeks());
        widget::Text::new(&money)
            .color(color::LIGHT_RED)
            .top_left_with_margin_on(self.ids.canvas_root, 10.0)
            .left_justify()
            .line_spacing(10.0)
            .set(self.ids.text_money, ui);
        None
    }
    fn input(&mut self, input: Input) -> StateChange {
        match input {
            Input::Button(ButtonArgs{state: Press, button:Keyboard(key), ..}) => {
                match key {
                    W => self.camera.translate(MoveDirection::Up),
                    S => self.camera.translate(MoveDirection::Down),
                    A => self.camera.translate(MoveDirection::Left),
                    D => self.camera.translate(MoveDirection::Right),
                    Space => self.updater.controll.toggle_pause(),
                    Return => {
                        self.camera.width = start_cam_width;
                        self.camera.height = start_cam_height;
                        self.camera.position = center;
                        self.camera.stop_tracking();
                    }
                    _ => {}
                }
            }
            Input::Move(MouseCursor(x, y)) => self.last_mouse_position = Position::new(x, y),
            Input::Move(MouseScroll(_, direction)) => {
                self.camera.zoom(self.last_screen_size,
                                 if direction == 1.0 {
                                     ZoomDirection::In
                                 } else {
                                     ZoomDirection::Out
                                 },
                                 self.last_mouse_position)
            }
            Input::Button(ButtonArgs{state: Press, button:Button::Mouse(MouseButton::Left), ..}) => {
                self.drag_mouse_start = Some(self.last_mouse_position)
            }
            Input::Button(ButtonArgs{state: Release, button:Button::Mouse(MouseButton::Left), ..}) => {
                if let Some(rect) = self.ceate_dragtengle_maybe() {
                    let projection = self.camera.create_projection(self.last_screen_size);
                    let projected_rect = Rectangle {
                        one: projection.screen_to_world(rect.one),
                        two: projection.screen_to_world(rect.two),
                    };
                    let model_lock = self.updater.model_writer.read_lock_model();
                    let time = model_lock.time;
                    let selected: Vec<ShipID> = model_lock.ships
                        .iter()
                        .filter_map(|x| {
                            if x.owner != self.player_id {
                                return None;
                            }
                            let ship_pos = x.movement.calc_position(&time, &model_lock.galaxy);
                            if projected_rect.contains(&ship_pos) {
                                Some(x.id)
                            } else {
                                None
                            }
                        })
                        .collect();
                    self.updater.enqueue(Change::Select(self.player_id, selected));
                };
                self.drag_mouse_start = None
            }
            _ => {}
        }
        None
    }
    fn exit(&mut self) {
        // kill the threads
        println!("exiting conquest state");
        self.updater.stop();
        self.pulser.controll.stop();
    }
}

impl ConquestState {
    pub fn new_game(generator: conrod::widget::id::Generator) -> ConquestState {
        let earth = StellarBody::new_earthlike("earth");
        ConquestState::new(generator,
            Camera::new(
                center,
                start_cam_width,
                start_cam_height
            ),
            Arc::new(RwLock::new(GameModel::new(vec![
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
    pub fn new(generator: conrod::widget::id::Generator,
               start_cam: Camera,
               start_model: Arc<RwLock<GameModel>>)
               -> ConquestState {
        ConquestState {
            ids: Ids::new(generator),
            player_id: 0,
            camera: start_cam,
            updater: Updater::new(start_model, Duration::days),
            pulser: Pulser::new(StateEvent::Idle),
            map_renderer: MapRenderer::new(),
            last_mouse_position: center,
            drag_mouse_start: None,
            last_screen_size: init_dimensions,
        }
    }
    fn ceate_dragtengle_maybe(&self) -> Option<Rectangle> {
        if let Some(drag_start) = self.drag_mouse_start {
            Some(Rectangle {
                one: drag_start,
                two: self.last_mouse_position,
            })
        } else {
            None
        }
    }
}
const init_dimensions: Dimensions = [0.0, 0.0];

widget_ids! {
    struct Ids {
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
        rect_select,
    }
}
