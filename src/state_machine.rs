use conrod;

pub trait State {
    fn enter(&mut self )-> Option<Box<State>>{ None }
    fn render(&mut self, ui:&mut conrod::UiCell)-> Option<Box<State>>{None}
    fn exit(&mut self,){}
}

// do nothing state, for init
struct UnitState;
impl State for UnitState{}

pub trait StateMachine{
    fn change_state(&mut self, to:Box<State>);
}

pub struct Machine{
    state:Box<State>
}
impl StateMachine for Machine{
    fn change_state(&mut self, to:Box<State>) {
        self.state.exit();
        self.state = to;
        if let Some(statebox) = self.state.enter(){
            self.change_state(statebox);
        }
    }
}
impl Machine{
    pub fn new() -> Machine{
        let mut result = Machine{state:Box::new(UnitState{})};
        if let Some(statebox) = result.state.enter(){
            result.change_state(statebox);
        }
        return result
    }
    pub fn render(&mut self, ui:&mut conrod::UiCell){
        if let Some(statebox) = self.state.render(ui){
            self.change_state(statebox);
        }
    }
}
