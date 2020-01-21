use wgpu::{
    //
};

use winit::{
    window::{
        Window,
        Icon,
        WindowBuilder,
    },
    event_loop::EventLoop,
};

pub struct AFImage<'a> {
    pub data: &'a [u8],
    pub width: u32,
    pub height: u32,
}

pub struct AFWindowConfig<'a> {

    pub icon: &'a AFImage<'a>,
    pub min_size: [u16; 2],
    pub max_size: [u16; 2],
    pub start_size: [u16; 2],
    pub title: &'a str,
    pub resizeable: bool,
    pub visible: bool,
    pub always_on_top: bool,
    pub maximized: bool,

}

pub struct AFWindow {

    event_loop: EventLoop<()>,
    window: Window,

}

impl AFWindow {

    fn new(config: &AFWindowConfig) -> &Self {
        let builder: WindowBuilder = WindowBuilder::new()
            .with_window_icon(Icon::from_rgba(
                config.icon.data.to_vec(),
                config.icon.width,
                config.icon.height,
            ).ok())
            .with_min_inner_size(config.min_size)
            .with_max_inner_size(config.max_size)
            .with_inner_size(config.start_size)
            .with_title(config.title)
            .with_resizable(config.resizeable)
            .with_always_on_top(config.always_on_top)
            .with_maximized(config.maximized);

        let event_loop: EventLoop<()> = EventLoop::new();
        let window = builder.build(&event_loop).unwrap();

        return &AFWindow {
            window,
            event_loop,
        };
    }

}
