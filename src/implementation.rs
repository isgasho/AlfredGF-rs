use crate::config_structs::*;
use crate::enums::*;
use crate::constructors::*;

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

pub struct AFContext {

    window: Window,
    present_mode: PresentMode,
    device: Device,
    queue: Queue,
    adapter: Adapter,

}

// implementation

impl AFWindowConstructor for AFWindow {

    fn new(config: &AFWindowConfig) -> Self {
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

impl AFContextConstructor<AFWindow> for AFContext {

    // absorbs the window
    fn new(window: AFWindow, config: &AFContextConfig) -> Self {
        let adapter: Adapter = Adapter::request(&RequestAdapterOptions {
            power_preference: match config.power_preference {
                AFPowerPreference::LowPower => PowerPreference::LowPower,
                AFPowerPreference::Default => PowerPreference::Default,
                AFPowerPreference::HighPower => PowerPreference::HighPerformance,
            },
            backends: match config.backend_lib {
                AFBackendLibrary::Vulkan => BackendBit::VULKAN,
                AFBackendLibrary::Metal => BackendBit::METAL,
                AFBackendLibrary::OpenGL => BackendBit::GL,
                AFBackendLibrary::DX12 => BackendBit::DX12,
                AFBackendLibrary::DX11 => BackendBit::DX11,
            }
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
