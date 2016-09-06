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


// a simple construct implementing the statemachine pattern.
// where one api can have wildly different implementation.
// this is used for major GUI changes, note that thus a state isn't strictly
// static. This pattern doesn't prevent that (and isn't intended to)

use conrod;
use piston_window::Input;

pub type StateChange = Option<Box<State>>;
pub trait State {
    fn enter(&mut self )-> StateChange{ None }
    fn update(&mut self, &mut conrod::UiCell)-> StateChange{None}
    fn exit(&mut self,){}
    fn input(&mut self, Input) -> StateChange{None}
    fn poll_event(&self) -> StateEvent {StateEvent::Idle}
}

// do nothing state, for init
struct UnitState;
impl State for UnitState{}

pub struct StateMachine{
    state:Box<State>
}
impl StateMachine{
    pub fn change_state(&mut self, to:Box<State>) {
        self.state.exit();
        self.state = to;
        if let Some(statebox) = self.state.enter(){
            self.change_state(statebox);
        }
    }
    pub fn new() -> StateMachine{
        let mut result = StateMachine{state:Box::new(UnitState{})};
        if let Some(statebox) = result.state.enter(){
            result.change_state(statebox);
        }
        return result
    }
    pub fn update(&mut self, ui:&mut conrod::UiCell){
        if let Some(statebox) = self.state.update(ui){
            self.change_state(statebox);
        }
    }
    pub fn input(&mut self, input:Input) {
        if let Some(statebox) = self.state.input(input){
            self.change_state(statebox);
        }
    }
    // allows seperate treats managed by the state
    // to ask for simple stuff such as updates
    pub fn poll_events(&self) -> StateEvent{
        self.state.poll_event()
    }
}
#[derive(Clone,Copy)]
pub enum StateEvent{
    Idle,
    WantsUpdate
}
