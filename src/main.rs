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

#[macro_use]
use piston_window::{
    Size, Event, EventLoop, OpenGL, PistonWindow, WindowSettings, TextureSettings, G2dTexture,
};
use conrod::Ui;
use piston_window::context::Context;
use input::Loop::*;
use piston_window::Event::*;
use conrod::image::Map;

pub mod camera;
pub mod geometry;
pub mod logic;
pub mod model;
pub mod state;
pub mod view;

use state::begin::BeginState;
use state::state_machine::StateMachine;

use piston_window::texture::UpdateTexture;
use piston_window::{G2d, Window};
use conrod::text::GlyphCache;

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
    let mut window: PistonWindow = PistonWindow::new(
        opengl,
        0,
        WindowSettings::new(format!("{} - {}", NAME, VERSION), [WIDTH, HEIGHT])
            .samples(4)
            .exit_on_esc(true)
            .srgb(false)
            .vsync(true)
            .build()
            .expect("Could not get gl context"),
    );
    window.set_ups(60);
    window.set_max_fps(60);

    // construct our `Ui`.
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = find_folder::Search::KidsThenParents(3, 5)
        .for_folder(assetspath)
        .expect("Couldn't find assets folder in root");
    let font_path = assets.join(font);
    ui.fonts
        .insert_from_file(font_path)
        .expect("Couldn't find the font");

    // Create a texture to use for efficiently caching text on the GPU.
    let mut text_vertex_data = Vec::new();

    let (mut glyph_cache, mut text_texture_cache) = {
        const SCALE_TOLERANCE: f32 = 0.1;
        const POSITION_TOLERANCE: f32 = 0.1;
        let cache =
            GlyphCache::new(WIDTH, HEIGHT, SCALE_TOLERANCE, POSITION_TOLERANCE);
        let buffer_len = WIDTH as usize * HEIGHT as usize;
        let init = vec![128; buffer_len];
        let settings = TextureSettings::new();
        let factory = &mut window.factory;
        let texture =
            G2dTexture::from_memory_alpha(factory, &init, WIDTH, HEIGHT, &settings).unwrap();
        (cache, texture)
    };


    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = Map::new();

    let mut state_machine = StateMachine::new();
    state_machine.change_state(Box::new(BeginState::new(ui.widget_id_generator())));

    let mut should_update = true;
    while let Some(event) = window.next() {
        {
            use state::state_machine::StateEvent::*;
            match state_machine.poll_events() {
                Idle => {}
                WantsUpdate => should_update = true,
            }
        }
        handle_piston_events(&mut ui, &event, window.size());

        match event.clone() {
            Loop(loop_event) => match loop_event {
                Idle(_) => std::thread::yield_now(),
                Render(_) => {
                    window.draw_2d(&event, |context, mut graphics| {
                        render(&mut ui,
                           &mut text_vertex_data,
                           &mut text_texture_cache
                           ,&mut glyph_cache
                           ,& image_map
                           , context
                           , &mut graphics)
                        }).unwrap();
                }
                AfterRender(_) => continue,
                Update(_) => {
                    if should_update {
                        state_machine.update(&mut ui.set_widgets());
                        should_update = false
                    }
                }
            },
            Input(i) => {
                state_machine.input(i);
                should_update = true
            }
            Custom(..) => {
                println!("Custom event! not sure what to do with these");
            }
        };
    }
}

fn render ( ui: &mut Ui
          , text_vertex_data: &mut Vec<u8>
          , mut text_texture_cache: &mut G2dTexture
          , mut glyph_cache: &mut GlyphCache
          , image_map: &Map<G2dTexture> // not used atm
          , context:  Context
          , graphics: &mut G2d
          ) {
            if let Some(primitives) = ui.draw_if_changed() {
                // A function used for caching glyphs to the texture cache.
                let cache_queued_glyphs = |graphics: &mut G2d,
                                           cache: &mut G2dTexture,
                                           rect: conrod::text::rt::Rect<u32>,
                                           data: &[u8]| {
                    let offset = [rect.min.x, rect.min.y];
                    let size = [rect.width(), rect.height()];
                    let format = piston_window::texture::Format::Rgba8;
                    let encoder = &mut graphics.encoder;
                    text_vertex_data.clear();
                    text_vertex_data.extend(data.iter().flat_map(|&b| vec![255, 255, 255, b]));
                    UpdateTexture::update(
                        cache,
                        encoder,
                        format,
                        &text_vertex_data[..],
                        offset,
                        size,
                    )
                    .expect("failed to update texture")
                };
                conrod::backend::piston::draw::primitives(
                    primitives,
                    context,
                    graphics,
                    &mut text_texture_cache,
                    &mut glyph_cache,
                    &image_map,
                    cache_queued_glyphs,
                    texture_from_image,
                );
            }
}
fn texture_from_image<T>(img: &T) -> &T {
                                     img
}

fn handle_piston_events(ui: &mut Ui, event: &Event, size: Size) {
    let (win_w, win_h) = (size.width as conrod::Scalar, size.height as conrod::Scalar);
    if let Some(e) = conrod::backend::piston::event::convert(event.clone(), win_w, win_h) {
        ui.handle_event(e);
    }
}
