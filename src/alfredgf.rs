use wgpu::{
    PowerPreference,
    Surface,
    Device,
    Queue,
    BackendBit,
    RequestAdapterOptions,
    Adapter,
    Limits,
    DeviceDescriptor,
    Extensions,
    ShaderModule,
    ShaderStage,
    read_spirv,
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

    pub fn new<'a>(config: &AFWindowConfig) -> &'a Self {
        return unsafe {
            match &CURRENT_WINDOW {
                None => {
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
                }
                Some(AFWindow) => {
                    // nothing... there's already a window
                }
            };

            CURRENT_WINDOW.as_ref().unwrap()
        };
    }

}

pub struct AFContextConfig {

    pub anisotropic_filtering: bool,
    pub power_preference: PowerPreference,

}

pub struct AFContext {

    size: PhysicalSize<u32>,
    surface: Surface,
    device: Device,
    queue: Queue,

}

static mut CURRENT_CONTEXT: Option<AFContext> = None;

impl AFContext {

    pub fn new<'a>(window: &AFWindow, config: &AFContextConfig) -> &'a Self {
        return unsafe {
            match &CURRENT_CONTEXT {
                None => {
                    let size: PhysicalSize<u32> = window.window.inner_size();
                    let surface: Surface = Surface::create(&window.window);
                    let adapter: Adapter = Adapter::request(&RequestAdapterOptions {
                        power_preference: config.power_preference,
                        backends: BackendBit::PRIMARY, // defaults to Vulkan / Metal
                    }).unwrap();

                    let (device, queue): (Device, Queue) = adapter.request_device(&DeviceDescriptor {
                        extensions: Extensions {
                            anisotropic_filtering: config.anisotropic_filtering,
                        },
                        limits: Limits::default(),
                    });

                    CURRENT_CONTEXT = Option::Some(AFContext {
                        size,
                        surface,
                        device,
                        queue,
                    });
                }
                Some(AFContext) => {
                    //
                }
            }

            CURRENT_CONTEXT.as_ref().unwrap()
        };
    }

}

pub struct AFShaderModule<'a> {

    module: ShaderModule,
    entry: &'a str,

}

impl<'a> AFShaderModule<'a> {

    pub fn new_with_bytes(context: &AFContext, data: &[u8], entry: &'a str) -> Self {
        let module: ShaderModule = context
            .device
            .create_shader_module(&read_spirv(std::io::Cursor::new(data)).unwrap());

        return AFShaderModule {
            module,
            entry: entry.clone(),
        };
    }

    pub fn new_with_path(context: &AFContext, path: &str, entry: &'a str) -> Self {
        let mut file = std::fs::File::open(path).unwrap();
        let mut words: Vec<u32> = read_spirv(&mut file).unwrap();
        let module: ShaderModule = context.device.create_shader_module(words.as_mut());

        return AFShaderModule {
            module,
            entry,
        };
    }

}
