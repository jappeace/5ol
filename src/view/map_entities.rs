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

use petgraph::graph::NodeIndex;
use conrod::widget::Widget;
use conrod::Positionable;

trait View<T:Widget + Positionable>{
    fn get_view_id(&self)-> Option<NodeIndex<u32>>;
    fn set_view_id(&mut self, NodeIndex<u32>);
    fn get_world_position(&self, game_state:&GameModel) -> Position;
    fn get_widget(&self) -> T;
    fn is_visible(&self, projection:&Projection, game_state:&GameModel) -> bool{
        projection.is_pos_visible(&self.get_world_position(game_state))
    }
    fn render(&mut self, ui:&mut conrod::UiCell, projection:&Projection, game_state:&GameModel){
        let position = projection.world_to_screen(
            self.get_world_position(game_state)
        );
        let widget = self.get_widget();
        let view_id = self.get_view_id().unwrap_or_else(|| {
            let id = ui.widget_id_generator().next();
            self.set_view_id(id);
            id
        });
        widget.x(position.x).y(position.y).set(
            view_id,
            ui
        );
    }
}
struct ShipView{
    view_id:Option<NodeIndex<u32>>;
    ship_id:ShipID;
}
impl ShipView{
    fn new(id:ShipID) -> ShipView{
        ShipView{
            view_id:None,
            ship_id:id
        }
    }
}
impl View<Oval> for ShipView{
    
}
