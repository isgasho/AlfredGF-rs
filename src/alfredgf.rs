use wgpu::{
    //
};

use winit::{
    window::{
        Window,
        Icon,
        WindowBuilder,
    },
    dpi::{
        PhysicalSize,
    },
    event_loop::{
        EventLoop,
    },
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

static mut CURRENT_WINDOW: Option<AFWindow> = None;

impl AFWindow {

    fn new<'a>(config: &AFWindowConfig) -> &'a Self {
        unsafe {
            let builder: WindowBuilder = WindowBuilder::new()
                .with_window_icon(Icon::from_rgba(
                    config.icon.data.to_vec(),
                    config.icon.width,
                    config.icon.height,
                ).ok())
                .with_min_inner_size(
                    PhysicalSize::new(config.min_size[0], config.min_size[1]))
                .with_max_inner_size(
                    PhysicalSize::new(config.max_size[0], config.max_size[1]))
                .with_inner_size(
                    PhysicalSize::new(config.start_size[0], config.start_size[1])
                )
                .with_title(config.title)
                .with_resizable(config.resizeable)
                .with_always_on_top(config.always_on_top)
                .with_maximized(config.maximized);

            let event_loop: EventLoop<()> = EventLoop::new();
            let window = builder.build(&event_loop).unwrap();
            let t_w = Option::Some(AFWindow {
                window,
                event_loop,
            });

            CURRENT_WINDOW = t_w;

            CURRENT_WINDOW.as_ref().unwrap()
        }
//        unsafe {
//            let mut w: Option<&AFWindow> = None;
//            let k = match &CURRENT_WINDOW {
//                None => {
//                    let builder: WindowBuilder = WindowBuilder::new()
//                        .with_window_icon(Icon::from_rgba(
//                            config.icon.data.to_vec(),
//                            config.icon.width,
//                            config.icon.height,
//                        ).ok())
//                        .with_min_inner_size(
//                            PhysicalSize::new(config.min_size[0], config.min_size[1]))
//                        .with_max_inner_size(
//                            PhysicalSize::new(config.max_size[0], config.max_size[1]))
//                        .with_inner_size(
//                            PhysicalSize::new(config.start_size[0], config.start_size[1])
//                        )
//                        .with_title(config.title)
//                        .with_resizable(config.resizeable)
//                        .with_always_on_top(config.always_on_top)
//                        .with_maximized(config.maximized);
//
//                    let event_loop: EventLoop<()> = EventLoop::new();
//                    let window = builder.build(&event_loop).unwrap();
//                    let aw: &AFWindow = &AFWindow {
//                        window,
//                        event_loop,
//                    };
//                    w = Option::Some(aw);
//
//                    w.unwrap()
//                }
//                Some(&AFWindow) => {
//                    w.unwrap()
//                }
//            };
//
//            return k;
//        }
    }

}
