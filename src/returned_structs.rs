use crate::config_structs::{
    AFWindowConfig,
    AFContextConfig,
};
use winit::{
    window::{
        Window,
        WindowBuilder,
        Icon,
    },
    dpi::{
        PhysicalSize,
    },
    event_loop::{
        EventLoop,
    },
};
use wgpu::{
    Adapter,
    RequestAdapterOptions,
    PresentMode,
    Limits,
    Device,
    DeviceDescriptor,
    Queue,
    Extensions,
    PowerPreference,
    BackendBit,
};

pub struct AFWindow {

    window: Window,
    event_loop: EventLoop<()>,

}

impl AFWindow {

    pub fn new(config: &AFWindowConfig) -> Self {
        let builder: WindowBuilder = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(config.start_size.width, config.start_size.height))
            .with_max_inner_size(PhysicalSize::new(config.max_size.width, config.max_size.height))
            .with_min_inner_size(PhysicalSize::new(config.min_size.width, config.min_size.height))
            .with_window_icon(match config.icon {
                Some(icon) => Icon::from_rgba(icon.data.to_vec(),
                                         icon.size.width,
                                         icon.size.height).ok(),
                None => Icon::from_rgba(vec![], 0, 0).ok(),
            })
            .with_title(config.title)
            .with_resizable(config.resizeable)
            .with_always_on_top(config.always_on_top)
            .with_maximized(config.maximized);

        let event_loop: EventLoop<()> = EventLoop::new();
        let window = builder.build(&event_loop).unwrap();

        return AFWindow {
            window,
            event_loop,
        };
    }

}

pub struct AFContext {

    window: Window,
    present_mode: PresentMode,
    device: Device,
    queue: Queue,
    adapter: Adapter,

}

impl AFContext {

    // absorbs the window
    pub fn new(window: AFWindow, config: &AFContextConfig) -> Self {
        let adapter: Adapter = Adapter::request(&RequestAdapterOptions {
            power_preference: PowerPreference::Default, // TODO make configurable
            backends: BackendBit::PRIMARY, // TODO make configurable
        }).unwrap();

        let (device, queue): (Device, Queue) =
            adapter.request_device(&DeviceDescriptor {
                extensions: Extensions {
                    anisotropic_filtering: config.anisotropic_filtering,
                },
                limits: Limits::default(),
            });

        let present_mode: PresentMode = match config.vsync {
            true => PresentMode::Vsync,
            false => PresentMode::NoVsync,
        };

        return AFContext {
            window: window.window,
            present_mode,
            device,
            queue,
            adapter,
        };
    }

}
