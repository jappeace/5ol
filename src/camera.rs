// does the world to screen mapping (and also renders
// the bodies right now, this will probably change when rendering becomes more 
// complex)

use geometry::*;
use stellar_bodies::*;
use conrod::Dimensions;
use conrod;
use time::Duration;

pub struct Camera{
    pub position:Position, // position in world coordinates (AU)
    width:Au, // in astromical units
    height:Au,
}
impl Camera{
    fn worldToScreen(&self, screenSize:&Dimensions, position:Position) -> Position{
        println!("screensize {:?}", screenSize);
        let factor = Position::new(screenSize[0], screenSize[1]) / Position::new(self.width, self.height);
        (position + self.position) * factor
    }
    pub fn new(position:Position, width:Au, height:Au)->Camera{
        Camera{position:position, width:2.0, height:2.0}
    }
    pub fn update(&self, ui:&mut conrod::UiCell, screenSize:&Dimensions, systems:&Vec<System>, time:&Duration){
        use conrod::widget::Circle;
        use conrod::{Positionable, Widget};
        use conrod::Colorable;
        let camrect = Rectangle{
            one: Position{
                x:self.position.x-self.width/2.0,
                y:self.position.y-self.height/2.0,
            },
            two: Position{
                x:self.position.x+self.width/2.0,
                y:self.position.y+self.height/2.0,
            }
        };
        // cull the ones outside view, (and ignoring their sub bodies)
        let visible = systems.iter().filter(|x| -> bool {
            let disk = Disk{
                position:x.position,
                radius:x.radius};
            camrect.contains(&x.position) ||
            disk.contains([camrect.tl(), camrect.tr()]) ||
            disk.contains([camrect.tr(), camrect.br()]) ||
            disk.contains([camrect.br(), camrect.bl()]) ||
            disk.contains([camrect.bl(), camrect.tl()])
        }
        ).flat_map(|x| &x.bodies);
        for body in visible{
            let nextId = {
                let mut generator = ui.widget_id_generator();
                generator.next()
            };
            let position = self.worldToScreen(screenSize, body.calcPosition(time));
            println!("{}", position);
            Circle::fill(5.0).x(position.x).y(position.y).color(conrod::color::WHITE).set(nextId, ui);
        }
    }
}
