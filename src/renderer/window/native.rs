use std::sync::mpsc::Receiver;

use glfw::{Action, Context, Key, WindowEvent, Glfw};

use crate::scene::core::*;
use crate::scene::scene::scene::*;

use super::super::webgl::*;
use super::super::renderer::forward::*;

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent)
{
    match event
    {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => 
        {
            window.set_should_close(true)
        }
        _ => {}
    }
}

pub struct Window
{
    glfw:Glfw,
    window:glfw::Window,
    renderer:ForwardRenderer,
    events:Receiver<(f64, WindowEvent)>,
}

impl Window
{
    pub fn new(tile:&str) -> Self
    {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        // glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
        // glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Compat));
        // glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        glfw.window_hint(glfw::WindowHint::Samples(Some(8)));

        let (mut window, events) = glfw.create_window(1376, 768, tile, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");
        
        window.make_current();

        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        glfw.set_swap_interval(glfw::SwapInterval::Adaptive);

        let size = window.get_size();
        let context = WebGLRenderingContext::new(|symbol| window.get_proc_address(symbol) as *const _);

        Self
        {
            glfw:glfw,
            window:window,
            renderer:ForwardRenderer::new(context, size.0 as u32, size.1 as u32),
            events:events,
        }
    }

    pub fn should_close(&self) -> bool
    {
        self.window.should_close()
    }

    pub fn update<T>(&mut self, mut callback:T) where T:'static + FnMut(&mut Canvas, f32)
    {
        while !self.should_close()
        {
            self.glfw.poll_events();

            for (_, event) in glfw::flush_messages(&self.events)
            {
                handle_window_event(&mut self.window, event);
            }

            let (w, h) = self.window.get_size();
            self.renderer.set_width(w as u32);
            self.renderer.set_height(h as u32);

            callback(&mut self.renderer, self.glfw.get_time() as f32);

            self.window.swap_buffers();
        }
    }
}

impl Canvas for Window
{
    fn width(&self) -> u32
    {
        self.window.get_size().0 as u32
    }

    fn height(&self) -> u32
    {
        self.window.get_size().1 as u32
    }

    fn render(&mut self, scene: &Scene)
    {
        self.renderer.render(scene);
    }
}