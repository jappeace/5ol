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
use conrod::widget::{Oval};
use conrod;

use model::root::GameModel;
use model::ship::ShipID;
use geometry::Position;
use camera::Projection;

trait Renderer {
    fn render(
        &mut self,
        ui:&mut conrod::UiCell,
        projection:&Projection,
        game_state:&GameModel
    );
}
trait View<T:Widget + Positionable> : Renderer{
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
use std::collections::HashMap;
use std::hash::Hash;
trait ViewModelMap<'a, K, ViewStruct> 
    where K:'a+Sized+Eq+Hash+Clone, ViewStruct:'a+ Renderer + Sized{
    fn get_hashmap<'b>(&'b mut self) -> &'b mut HashMap<K, ViewStruct>;
    fn create_view(&self, with_key:K) -> ViewStruct;

    fn update_views(&mut self, keys:&Vec<K>){
        for k in keys{
            let view = self.create_view(k.clone());
            self.get_hashmap().entry(k.clone()).or_insert(
                view
            );
        }
    }
}
impl<'a, K, ViewStruct> Renderer for ViewModelMap<'a, K, ViewStruct>
    where K:'a+Sized+Eq+Hash+Clone, ViewStruct:'a+ Renderer + Sized{
    fn render(&mut self, ui:&mut conrod::UiCell, projection:&Projection, game_state:&GameModel){
        for value in self.get_hashmap().values_mut(){
            value.render(ui,projection,game_state);
        }
    }
}

struct ShipViews{
    map:HashMap<ShipID, ShipView>
}
impl<'a> ViewModelMap<'a, ShipID, ShipView> for ShipViews{
    fn get_hashmap<'b>(&'b mut self) -> &'b mut HashMap<ShipID, ShipView>{
        &mut self.map
    }
    fn create_view(&self, with_key:ShipID) -> ShipView{
        ShipView::new(with_key)
    }
}
struct ShipView{
    view_id:Option<NodeIndex<u32>>,
    ship_id:ShipID
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
    fn get_view_id(&self)-> Option<NodeIndex<u32>>{
        self.view_id
    }
    fn set_view_id(&mut self, id:NodeIndex<u32>){
        self.view_id = Some(id);
    }
    fn get_world_position(&self, game_state:&GameModel) -> Position{
        game_state.ships[self.ship_id].movement.calc_position(&game_state.time, &game_state.galaxy)
    }
    fn get_widget(&self) -> Oval{
        Oval::fill([5.0,5.0])
    }
}
impl Renderer for ShipView{
    fn render(
        &mut self,
        ui:&mut conrod::UiCell,
        projection:&Projection,
        game_state:&GameModel
    ){
        View::<Oval>::render(self,ui,projection,game_state)
    }
}
