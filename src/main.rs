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


// main file where we load the program and do the window loop
// (I'm not a bliever of a sparse main file)
#![allow(non_upper_case_globals)]
#[macro_use] extern crate conrod;
extern crate find_folder;
extern crate piston_window;
extern crate chrono;
extern crate petgraph;

use piston_window::{EventLoop, OpenGL, PistonWindow, WindowSettings};
use piston_window::Event::*;

mod state{
    pub mod state_machine;
    pub mod begin;
    pub mod conquest;
    pub mod planet;
}
mod geometry;
mod model{
    pub mod root;
    pub mod galaxy;
    pub mod colony;
    pub mod ship;
}
mod camera;
mod async{
    pub mod thread_status;
    pub mod logic_updater;
    pub mod model_access;
    pub mod pulser;
}
mod view{
    pub mod map_entities;
}

use state::state_machine::StateMachine;
use state::begin::BeginState;

const assetspath: &'static str = "assets";
const font: &'static str = "fonts/NotoSans/NotoSans-Regular.ttf";

const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;
    
    // Construct the window.
    let mut window: PistonWindow =
        WindowSettings::new(format!("{} - {}", NAME, VERSION), [WIDTH, HEIGHT])
            .opengl(opengl).exit_on_esc(true).vsync(true).build().unwrap();
    window.set_ups(60);
    window.set_max_fps(60);

    // construct our `Ui`.
    let mut ui = conrod::UiBuilder::new().build();

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = find_folder::Search::KidsThenParents(3, 5).
        for_folder(assetspath).expect("Couldn't find assets folder in root");
    let font_path = assets.join(font);
    ui.fonts.insert_from_file(font_path).expect(
        "Couldn't find the font" 
    );

    // Create a texture to use for efficiently caching text on the GPU.
    let mut text_texture_cache =
        conrod::backend::piston_window::GlyphCache::new(&mut window, WIDTH, HEIGHT);

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::new();

    let mut state_machine = StateMachine::new();
    state_machine.change_state(Box::new(BeginState::new(ui.widget_id_generator())));

    let mut should_update = true;
    while let Some(event) = window.next() {
        {
            use state::state_machine::StateEvent::*;
            match state_machine.poll_events(){
                Idle => {}
                WantsUpdate => should_update = true,
            }
        }
        if let Some(e) = conrod::backend::piston_window::convert_event(event.clone(), &window) {
            ui.handle_event(e);
        }
        match event.clone() {
            Idle (_ ) => std::thread::yield_now(),
            Render(_ ) => window.draw_2d(&event, |c, g| {
                    if let Some(primitives) = ui.draw_if_changed() {
                        fn texture_from_image<T>(img: &T) -> &T { img };
                        conrod::backend::piston_window::draw(c, g, primitives,
                                                             &mut text_texture_cache,
                                                             &image_map,
                                                             texture_from_image);
                    }
                }).unwrap(),
            AfterRender(_ ) => continue,
            Update(_ ) => if should_update {
                state_machine.update(&mut ui.set_widgets());
                should_update = false
            },
            Input(i) => {
                state_machine.input(i);
                should_update = true
            },
        };
    }

}
