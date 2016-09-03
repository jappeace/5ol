
#[macro_use] extern crate conrod;
extern crate find_folder;
extern crate piston_window;

use piston_window::{EventLoop, OpenGL, PistonWindow, UpdateEvent, WindowSettings, Event};
use piston_window::Event::*;

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

    // Instantiate the generated list of widget identifiers.
    let ids = &mut Ids::new(ui.widget_id_generator());
    // Poll events from the window.

    let mut handledInput = false;
    while let Some(event) = window.next() {
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
            Update(_ ) => if !handledInput {set_widgets(ui.set_widgets(), ids); handledInput = true },
            Input(I) => handledInput = false,
        };
    }

}

// Draw the Ui.
fn set_widgets(ref mut ui: conrod::UiCell, ids: &mut Ids) {
    use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget, Scalar};
    use conrod::widget::Line;


    const PAD: Scalar = 20.0;
    const DEMO_TEXT: &'static str = "After a long power struggle over now several generations, 
    you Kim Ill Sung III has finally crushed all opposition.
    With no-one else left daring to oppose you mankind puts it faith in you,
    our dearest leader, and you look to the stars...
";
    // Construct our main `Canvas` tree.
    widget::Canvas::new().color(color::BLACK).set(ids.root, ui);
    widget::Text::new(DEMO_TEXT)
        .color(color::LIGHT_RED)
        .padded_w_of(ids.root, PAD)
        .mid_top_with_margin_on(ids.root, PAD)
        .align_text_left()
        .line_spacing(10.0)
        .set(ids.intro_text, ui);
}


// Button matrix dimensions.
const ROWS: usize = 10;
const COLS: usize = 24;

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    Ids {
        root,
        intro_text
    }
}
