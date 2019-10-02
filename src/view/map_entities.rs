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
 

// This file contains most boilerplate for drawing shapes on the map.
// The most important thing it does is connecting things from the game model
// to their respective conrod view_id's.

use petgraph::graph::NodeIndex;
use conrod::widget::primitive::shape::oval::Full;
use conrod::widget::{Oval};
use conrod::*;

use crate::model::*;
use crate::model::ship::ShipID;
use crate::model::galaxy::*;
use crate::geometry::Position;
use crate::camera::Projection;

pub trait View<T:Widget + Positionable> {
    fn get_view_id(&self)-> Option<NodeIndex<u32>>;
    fn set_view_id(&mut self, _:NodeIndex<u32>);
    fn get_world_position(&self, game_state:&GameModel) -> Position;
    fn get_widget(&self) -> T;
    fn is_visible(&self, projection:&Projection, game_state:&GameModel) -> bool{
        projection.is_pos_visible(&self.get_world_position(game_state))
    }
    fn render(
        &mut self,
        ui:&mut conrod::UiCell,
        projection:&Projection,
        game_state:&GameModel
    ) {
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
use std::marker::PhantomData;
pub struct ViewsMap<Key,Shape,ViewStruct>
    where Key:Sized+Eq+Hash+Clone,
          Shape:Widget + Positionable,
          ViewStruct:View<Shape> + Sized{
    pub map:HashMap<Key, ViewStruct>,
    create_view:fn(Key)->ViewStruct,
    shape_has_to_be_used:PhantomData<Shape>
}
impl<Key,Shape,ViewStruct> ViewsMap<Key,Shape,ViewStruct>
    where Key: Sized + Eq + Hash + Clone,
    Shape:Widget + Positionable,
    ViewStruct:View<Shape> + Sized{
    fn update_views<I>(&mut self, keys:I)
        where I: Iterator<Item=Key>{
        for k in keys{
            let view = (self.create_view)(k.clone());
            self.map.entry(k.clone()).or_insert(
                view
            );
        }
    }
    fn render(&mut self, ui:&mut conrod::UiCell, projection:&Projection, game_state:&GameModel){
        for value in self.map.values_mut().filter(|x| x.is_visible(projection, game_state)){
            value.render(ui,projection,game_state);
        }
    }
    fn new(create_function:fn(Key)->ViewStruct)->ViewsMap<Key,Shape,ViewStruct>{
        ViewsMap::<Key,Shape,ViewStruct>{
            map:HashMap::<Key,ViewStruct>::new(),
            create_view:create_function,
            shape_has_to_be_used:PhantomData
        }
    }
}

use conrod::widget::Rectangle;
pub struct MapRenderer{
    pub planets:ViewsMap<BodyAddress, Oval<Full>, PlanetView>,
    ships:ViewsMap<ShipID, Oval<Full>, ShipView>,
    selected:ViewsMap<usize,Rectangle,SelectionView>,
    player:PlayerID
}
impl MapRenderer{
    pub fn new() -> MapRenderer{
        MapRenderer{
            planets:ViewsMap::<BodyAddress, Oval<Full>, PlanetView>::new(PlanetView::new),
            ships:ViewsMap::<ShipID, Oval<Full>, ShipView>::new(ShipView::new),
            selected:ViewsMap::<usize,Rectangle,SelectionView>::new(SelectionView::new),
            player:0
        }
    }
    pub fn render(&mut self, ui:&mut conrod::UiCell, projection:&Projection, game_state:&GameModel){
        self.planets.update_views(game_state.galaxy.systems.iter().filter(
            |x| projection.is_visible(&x.used_space)
        ).flat_map(|x| x.bodies.iter().map(|y| y.address)));
        self.planets.render(ui,projection,game_state);
        self.ships.update_views(game_state.ships.iter().enumerate().map(|x| x.0));
        self.ships.render(ui,projection,game_state);
        self.selected.update_views(game_state.players[self.player].selected.iter().enumerate().map(|x| x.0));
        self.selected.render(ui,projection,game_state);
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
const black:Color = Color::Rgba(0.0,0.0,0.0,1.0);
impl View<Oval<Full>> for ShipView{
    fn get_view_id(&self)-> Option<NodeIndex<u32>>{
        self.view_id
    }
    fn set_view_id(&mut self, id:NodeIndex<u32>){
        self.view_id = Some(id);
    }
    fn get_world_position(&self, game_state:&GameModel) -> Position{
        game_state.ships[self.ship_id].movement.calc_position(&game_state.time, &game_state.galaxy)
    }
    fn get_widget(&self) -> Oval<Full>{
        Oval::fill([5.0,5.0]).color(black)
    }
}
pub struct PlanetView{
    view_id:Option<NodeIndex<u32>>,
    address:BodyAddress,
}
impl PlanetView{
    fn new(address:BodyAddress)->PlanetView{
        PlanetView{view_id:None,address:address}
    }
}
impl View<Oval<Full>> for PlanetView{
    fn get_view_id(&self)-> Option<NodeIndex<u32>>{
        self.view_id
    }
    fn set_view_id(&mut self, id:NodeIndex<u32>){
        self.view_id = Some(id);
    }
    fn get_world_position(&self, game_state:&GameModel) -> Position{
        game_state.galaxy[self.address].calc_position(&game_state.time)
    }
    fn get_widget(&self) -> Oval<Full>{
        Oval::fill([10.0,10.0])
    }
}

struct SelectionView{
    view_id:Option<NodeIndex<u32>>,
    selected_index:usize,
    player_id:PlayerID,
}
impl SelectionView{
    fn new(address:usize)->SelectionView{
        SelectionView{view_id:None,selected_index:address, player_id:0}
    }
}
impl View<Rectangle> for SelectionView{
    fn get_view_id(&self)-> Option<NodeIndex<u32>>{
        self.view_id
    }
    fn set_view_id(&mut self, id:NodeIndex<u32>){
        self.view_id = Some(id);
    }
    fn get_world_position(&self, game_state:&GameModel) -> Position{
        let ship_id = game_state.players[self.player_id].selected[self.selected_index];
        game_state.ships[ship_id].movement.calc_position(
            &game_state.time, &game_state.galaxy
        )
    }
    fn get_widget(&self) -> Rectangle{
        Rectangle::outline([10.0,10.0])
    }
}
