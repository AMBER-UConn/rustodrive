
use glium::glutin;
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::{Display, Surface};
use imgui::{Context, FontConfig, FontGlyphRanges, FontSource, Ui};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::path::Path;
use std::time::Instant;

pub struct System {
    pub event_loop: EventLoop<()>,
    pub display: glium::Display,
    pub imgui: Context,
    pub platform: WinitPlatform,
    pub renderer: Renderer,
    pub font_size: f32,
}

pub fn init() -> System {
    let mut imgui_context = imgui::Context::create();

    let event_loop = glium::glutin::event_loop::EventLoop::new();
    let window_builder = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(1024f32, 768f32))
        .with_title("Hello World");
    let context_builder = glium::glutin::ContextBuilder::new();

    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

    let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui_context);
    {
        let gl_window = display.gl_window();
        let window = gl_window.window();

        platform.attach_window(
            imgui_context.io_mut(),
            window,
            imgui_winit_support::HiDpiMode::Default,
        );
    }

    let renderer = imgui_glium_renderer::Renderer::init(&mut imgui_context, &display).unwrap();

    // Fixed font size. Note imgui_winit_support uses "logical
    // pixels", which are physical pixels scaled by the devices
    // scaling factor. Meaning, 13.0 pixels should look the same size
    // on two different screens, and thus we do not need to scale this
    // value (as the scaling is handled by winit)
    let font_size = 13.0;

    // imgui_context.fonts().add_font(&[
    //     FontSource::TtfData {
    //         data: include_bytes!("../fonts/Roboto-Regular.ttf"),
    //         size_pixels: font_size,
    //         config: Some(FontConfig {
    //             // As imgui-glium-renderer isn't gamma-correct with
    //             // it's font rendering, we apply an arbitrary
    //             // multiplier to make the font a bit "heavier". With
    //             // default imgui-glow-renderer this is unnecessary.
    //             rasterizer_multiply: 1.5,
    //             // Oversampling font helps improve text rendering at
    //             // expense of larger font atlas texture.
    //             oversample_h: 4,
    //             oversample_v: 4,
    //             ..FontConfig::default()
    //         }),
    //     },
    // ]);

    System {
        event_loop,
        display,
        platform,
        renderer,
        font_size,
        imgui: imgui_context,
    }
}

impl System {
    pub fn main_loop<F: FnMut(&mut bool, &mut Ui) + 'static>(self, mut run_ui: F) {
        let mut last_frame = std::time::Instant::now();
        let System {
            event_loop,
            display,
            mut imgui,
            mut platform,
            mut renderer,
            ..
        } = self;

        event_loop.run(move |event, _, control_flow| match event {
            glium::glutin::event::Event::NewEvents(_) => {
                imgui
                    .io_mut()
                    .update_delta_time(last_frame.elapsed());
                last_frame = std::time::Instant::now();
            }

            glium::glutin::event::Event::MainEventsCleared => {
                let gl_window = display.gl_window();
                platform
                    .prepare_frame(imgui.io_mut(), gl_window.window())
                    .expect("Failed to prepare frame");
                gl_window.window().request_redraw();
            }

            glium::glutin::event::Event::RedrawRequested(_) => {
                let mut ui = imgui.frame();
                let mut run = true;
                run_ui(&mut run, &mut ui);
                if !run {
                    *control_flow = ControlFlow::Exit;
                }

                let gl_window = display.gl_window();
                let mut target = display.draw();
                target.clear_color_srgb(1.0, 1.0, 1.0, 1.0);
                platform.prepare_render(&ui, gl_window.window());
                let draw_data = ui.render();
                renderer
                    .render(&mut target, draw_data)
                    .expect("Rendering failed");
                target.finish().expect("Failed to swap buffers");
            }

            glium::glutin::event::Event::WindowEvent {
                event: glium::glutin::event::WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
            }

            event => {
                let gl_window = display.gl_window();
                platform.handle_event(imgui.io_mut(), gl_window.window(), &event);
            }
        });
    }
}
