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


use conrod;
use piston_window::Input;

pub type StateChange = Option<Box<State>>;
pub trait State {
    fn enter(&mut self )-> StateChange{ None }
    fn update(&mut self, ui:&mut conrod::UiCell)-> StateChange{None}
    fn exit(&mut self,){}
    fn input(&mut self, input:Input) -> StateChange{None}
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
}
