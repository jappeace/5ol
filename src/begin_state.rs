use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget, Scalar};
use conrod::widget::Line;
use conrod;

use state_machine::State;
use conquest_state::ConquestState;

pub struct BeginState{
    ids:Ids
}
impl BeginState{
    pub fn new(mut generator: conrod::widget::id::Generator)->BeginState{
        BeginState{ids:Ids::new(generator)}
    }
}
impl State for BeginState {
    fn render(&mut self, ui:&mut conrod::UiCell) -> Option<Box<State>> {

        const PAD: Scalar = 20.0;
        const INTRO_TEXT: &'static str = "After a long power struggle over now several generations, 
    you Kim Ill Sung III has finally crushed all opposition.
    With no-one else left daring to oppose you mankind puts it faith in you,
    our dearest leader, and you look to the stars...
";
        // Construct our main `Canvas` tree.
        widget::Canvas::new().color(color::BLACK).set(self.ids.canvas_root, ui);
        widget::Text::new(INTRO_TEXT)
            .color(color::LIGHT_RED)
            .middle_of(self.ids.canvas_root)
            .align_text_left()
            .line_spacing(10.0)
            .set(self.ids.text_intro, ui);
        for click in widget::Button::new()
            .w_h(200.0, 80.0)
            .label("Begin your conquest")
            .color(color::DARK_CHARCOAL)
            .label_color(color::GRAY)
            .set(self.ids.button_begin, ui){
                return Some(Box::new(ConquestState::new(ui.widget_id_generator())))
            }
        None
    }
}

// Button matrix dimensions.
const ROWS: usize = 10;
const COLS: usize = 24;

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    Ids {
        canvas_root,
        text_intro,
        button_begin
    }
}
