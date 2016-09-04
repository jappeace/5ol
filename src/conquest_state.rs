use state_machine::State;
use conrod;
pub struct ConquestState{
    ids:Ids
}
impl ConquestState{
    pub fn new(mut generator: conrod::widget::id::Generator)->ConquestState{
        ConquestState{ids:Ids::new(generator)}
    }
}
impl State for ConquestState{
    
    fn render(&mut self, ui:&mut conrod::UiCell) ->  Option<Box<State>>{
        use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget, Scalar};
        use conrod::widget::Line;
        widget::Canvas::new().color(color::BLUE).set(self.ids.canvas_root, ui);
        None
    }
}

widget_ids! {
    Ids {
        canvas_root,
    }
}
