use std::rc::Rc;
use std::cell::RefCell;

use world::core::*;
use world::scene::scene::*;

use super::super::renderer::forward::*;

use stdweb::unstable::TryInto;
use stdweb::web::{ IEventTarget, IHtmlElement, IParentNode, document, window };
use stdweb::web::event::{ ResizeEvent };
use stdweb::web::html_element::CanvasElement;

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

struct Context
{
    pub canvas:CanvasElement,
    pub renderer:ForwardRenderer,
}

impl Context 
{
    fn step(&mut self)
    {
        self.renderer.set_width(self.canvas.width());
        self.renderer.set_height(self.canvas.height());        
    }

    fn animate<T>(&mut self, rc: Rc<RefCell<Self>>, mut dispatch:T, time:f32) where T:'static + FnMut(&mut Canvas, f32)
    {
        self.step();
        dispatch(&mut self.renderer, time);

        window().request_animation_frame(
            move |time| 
            {
                rc.borrow_mut().animate(rc.clone(), dispatch, (time as f32) * 0.001);
            }
        );
    }
}

pub struct Window
{
    context:Rc<RefCell<Context>>,
}

impl Window
{
    pub fn new(_tile:&str) -> Self
    {
        stdweb::initialize();

        let canvas: CanvasElement = document().query_selector( "#canvas" ).unwrap().unwrap().try_into().unwrap();
            canvas.set_width(canvas.offset_width() as u32);
            canvas.set_height(canvas.offset_height() as u32);
    
        window().add_event_listener(enclose!( (canvas) move |_: ResizeEvent| {
            canvas.set_width(canvas.offset_width() as u32);
            canvas.set_height(canvas.offset_height() as u32);
        }));

        Self
        {
            context : Rc::new(RefCell::new(Context 
            {
                renderer:ForwardRenderer::new(canvas.get_context().unwrap(), canvas.width(), canvas.height()),
                canvas:canvas,
            }))
        }
    }

    pub fn should_close(&self) -> bool
    {
        false
    }

    pub fn update<T>(&mut self, callback:T) where T:'static + FnMut(&mut Canvas, f32)
    {
        self.context.borrow_mut().animate(self.context.clone(), callback, 0.0);
    }
}

impl Canvas for Window
{
    fn width(&self) -> u32
    {
        self.context.borrow_mut().canvas.width()
    }

    fn height(&self) -> u32
    {
        self.context.borrow_mut().canvas.height()
    }

    fn render(&mut self, scene: &Scene)
    {
        self.context.borrow_mut().renderer.render(scene);
    }
}