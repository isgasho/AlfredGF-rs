use crate::constructors::*;
use crate::enums::*;
use crate::generic::*;
use crate::util_structs::*;

use wgpu::{
    read_spirv, Adapter, BackendBit, BlendDescriptor, Device, DeviceDescriptor, Extensions, Limits,
    PowerPreference, PresentMode, Queue, RequestAdapterOptions, ShaderModule, ShaderStage,
};
use winit::{
    dpi::PhysicalSize,
    event_loop::{EventLoop, ControlFlow},
    event::{Event, WindowEvent},
    window::{Icon, Window, WindowBuilder, Fullscreen},
    monitor::{MonitorHandle,},
};

pub struct AFWindow {
    window: Window,
    event_loop: EventLoop<()>,
}

pub struct AFContext {
    window: Window,
    event_loop: EventLoop<()>,
    present_mode: PresentMode,
    device: Device,
    queue: Queue,
    adapter: Adapter,
}

pub struct AFShaderModule {
    module: ShaderModule,
    stage: ShaderStage,
    entry: String,
}

pub struct AFRenderPipeline {
    //
}

// implementation

impl AFWindowConstructor for AFWindow {
    fn new(config: &AFWindowConfig) -> Self {
        let event_loop: EventLoop<()> = EventLoop::new();
        let monitor = match config.monitor_chooser {
            None => {
                event_loop.primary_monitor()
            },
            Some(closure) => {
                let m_hs = event_loop.available_monitors().collect::<Vec<_>>();
                let monitors = m_hs.iter().map(|monitor_handle|{
                    AFMonitor {
                        size: AFSize2D {
                            width: monitor_handle.size().width,
                            height: monitor_handle.size().height,
                        },
                        position: AFSize2D {
                            width: monitor_handle.position().x,
                            height: monitor_handle.position().y,
                        },
                        name: monitor_handle.name(),
                        scale_factor: monitor_handle.scale_factor(),
                    }
                }).collect::<Vec<_>>();

                let m_n = closure(monitors);
                let m = event_loop.available_monitors().nth(m_n);

                m.expect("Could not find a suitable monitor.")
            },
        };

        let builder: WindowBuilder = WindowBuilder::new()
            .with_inner_size(match config.start_size {
                AFWindowSize::MonitorSize => {
                    monitor.size()
                },
                AFWindowSize::Size(size) => {
                    PhysicalSize::new(
                        size.width,
                        size.height,
                    )
                }
            })
            .with_max_inner_size(match config.max_size {
                AFWindowSize::MonitorSize => {
                    monitor.size()
                },
                AFWindowSize::Size(size) => {
                    PhysicalSize::new(
                        size.width,
                        size.height,
                    )
                }
            })
            .with_min_inner_size(match config.min_size {
                AFWindowSize::MonitorSize => {
                    monitor.size()
                },
                AFWindowSize::Size(size) => {
                    PhysicalSize::new(
                        size.width,
                        size.height,
                    )
                }
            })
            .with_transparent(config.transparent)
            .with_decorations(config.decorated)
            .with_window_icon(match config.icon {
                Some(icon) => {
                    Icon::from_rgba(icon.data.to_vec(), icon.size.width, icon.size.height).ok()
                }
                None => Icon::from_rgba(vec![], 0, 0).ok(),
            })
            .with_fullscreen(match config.fullscreen {
                true => {Some(Fullscreen::Borderless(monitor))},
                false => {None},
            })
            .with_title(config.title)
            .with_resizable(config.resizeable)
            .with_always_on_top(config.always_on_top)
            .with_maximized(config.maximized);

        let window = builder.build(&event_loop).unwrap();

        return AFWindow { window, event_loop };
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
            },
        })
        .unwrap();

        let (device, queue): (Device, Queue) = adapter.request_device(&DeviceDescriptor {
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
            event_loop: window.event_loop,
            present_mode,
            device,
            queue,
            adapter,
        };
    }
}

impl AFShaderConstructor<AFContext> for AFShaderModule {
    fn new(context: &AFContext, config: &AFShaderConfig) -> Self {
        let module: ShaderModule = context
            .device
            .create_shader_module(&read_spirv(std::io::Cursor::new(config.bytecode)).unwrap());

        return AFShaderModule {
            module,
            stage: match config.stage {
                AFShaderStage::None => ShaderStage::NONE,
                AFShaderStage::Vertex => ShaderStage::VERTEX,
                AFShaderStage::Fragment => ShaderStage::FRAGMENT,
                AFShaderStage::Compute => ShaderStage::COMPUTE,
            },
            entry: config.entry_point.to_string(),
        };
    }
}

impl AFRenderPipelineConstructor<AFContext> for AFRenderPipeline {
    fn new(context: &AFContext, config: &AFRenderPipelineConfig) -> Self {
        unimplemented!()
    }
}

impl AFMainloop for AFContext {
    fn mainloop<F: 'static, T: 'static>(context: AFContext, on_redraw: F, on_finish: T)
    where
        F: Fn(AFMainloopState) -> (),
        T: Fn() -> (),
    {
        let AFContext{event_loop, window, ..} = context;

        event_loop.run(move |event, _, control_flow|{
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent {window_id, event} => {
                    // do stuff with the window event
                },
                Event::MainEventsCleared => {
                    window.request_redraw();
                },
                Event::RedrawRequested(window_id) => {
                    on_redraw(AFMainloopState{});
                },
                Event::LoopDestroyed => {
                    on_finish();
                },
                _ => {},
            }
        });
    }
}
