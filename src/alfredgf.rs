use wgpu::{
    read_spirv, Adapter, BackendBit, BindGroup, BindGroupDescriptor, BindGroupLayout,
    BindGroupLayoutBinding, BindGroupLayoutDescriptor, Binding, BindingResource, BindingType,
    BlendDescriptor, Buffer, BufferDescriptor, BufferUsage, ColorStateDescriptor, ColorWrite,
    CullMode, Device, DeviceDescriptor, Extensions, FrontFace, IndexFormat, InputStepMode, Limits,
    PipelineLayout, PipelineLayoutDescriptor, PowerPreference, PresentMode, PrimitiveTopology,
    ProgrammableStageDescriptor, Queue, RasterizationStateDescriptor, RenderPipeline,
    RenderPipelineDescriptor, RequestAdapterOptions, ShaderModule, ShaderStage, Surface, SwapChain,
    SwapChainDescriptor, TextureFormat, TextureUsage, VertexAttributeDescriptor,
    VertexBufferDescriptor, VertexFormat, CommandEncoder, CommandEncoderDescriptor, CommandBuffer,
    RenderPassDescriptor, RenderPass, RenderPassColorAttachmentDescriptor,
};

use winit::{
    dpi::PhysicalSize,
    event_loop::{EventLoop, ControlFlow},
    window::{Icon, Window, WindowBuilder},
    event::{WindowEvent, Event, KeyboardInput, ElementState, VirtualKeyCode},
};

use std::time::SystemTime;
use std::collections::HashMap;
use std::ops::Range;
use std::path::PathBuf;

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
    pub fn new<'a>(config: &AFWindowConfig) -> Self {
        let builder: WindowBuilder = WindowBuilder::new()
            .with_window_icon(
                Icon::from_rgba(
                    config.icon.data.to_vec(),
                    config.icon.width,
                    config.icon.height,
                )
                    .ok(),
            )
            .with_min_inner_size(PhysicalSize::new(
                config.min_size[0],
                config.min_size[1],
            ))
            .with_max_inner_size(PhysicalSize::new(
                config.max_size[0],
                config.max_size[1],
            ))
            .with_inner_size(PhysicalSize::new(
                config.start_size[0],
                config.start_size[1],
            ))
            .with_title(config.title)
            .with_resizable(config.resizeable)
            .with_always_on_top(config.always_on_top)
            .with_maximized(config.maximized);

        let event_loop: EventLoop<()> = EventLoop::new();
        let window = builder.build(&event_loop).unwrap();

        return AFWindow{event_loop, window};
    }
}

pub struct AFContextConfig {
    pub anisotropic_filtering: bool,
    pub power_preference: PowerPreference,
    pub vsync: bool,
    pub size: [u32; 2],
}

pub struct AFContext {
    size: PhysicalSize<u32>,
    surface: Surface,
    device: Device,
    queue: Queue,
    present_mode: PresentMode,
}

static mut CURRENT_CONTEXT: Option<AFContext> = None;

impl AFContext {
    pub fn new<'a>(window: &AFWindow, config: &AFContextConfig) -> &'a mut Self {
        return unsafe {
            match &CURRENT_CONTEXT {
                None => {
                    let size: PhysicalSize<u32> = window.window.inner_size();
                    let surface: Surface = Surface::create(&window.window);
                    let adapter: Adapter = Adapter::request(&RequestAdapterOptions {
                        power_preference: config.power_preference,
                        backends: BackendBit::PRIMARY, // defaults to Vulkan / Metal
                    })
                    .unwrap();

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

                    CURRENT_CONTEXT = Option::Some(AFContext {
                        size,
                        surface,
                        device,
                        queue,
                        present_mode,
                    });
                }
                Some(afcontext) => {
                    //
                }
            }

            CURRENT_CONTEXT.as_mut().unwrap()
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

        return AFShaderModule { module, entry };
    }
}

pub struct AFBindGroup {
    layout: BindGroupLayout,
    bind_group: BindGroup,
    //index_buffer: Option<Buffer>,
    vertex_buffers: HashMap<u32, Buffer>,
    // TODO use the below command to copy data to here and other
    // wgpu::CommandEncoder.copy_buffer_to_buffer(&src, 0, &dst, 0, len_of_src); validate len of source first
}

pub enum AFBindingType {
    Buffer {
        size: usize,
        dynamic: bool,
        readonly: bool,
    },
    // TODO add more bindings here
}

pub struct AFBinding {
    pub id: u32,
    pub binding: AFBindingType,
    pub visibility: ShaderStage,
}

fn create_buffer(context: &AFContext, usage: BufferUsage, data: &[u8]) -> Buffer {
    let buffer: Buffer = context
        .device
        .create_buffer_mapped(data.len(), usage)
        .fill_from_slice(&data);

    return buffer;
}

fn create_empty_buffer(context: &AFContext, size: usize, usage: BufferUsage) -> Buffer {
    let buffer: Buffer = context.device.create_buffer(&BufferDescriptor {
        size: size as u64,
        usage,
    });

    return buffer;
}

// make note of these functions:

// wgpu::CommandEncoder.copy_buffer_to_buffer
// wgpu::CommandEncoder.copy_buffer_to_texture
// wgpu::CommandEncoder.copy_texture_to_buffer

pub struct AFUniform {
    // TODO add thing that can upload to uniforms
    pub id: u32,
    pub stage: ShaderStage,
    pub dynamic: bool,
    pub byte_size: u64,
}

pub struct AFVertexBufferSlot<'a> {
    pub stride: u64,
    pub step_mode: InputStepMode,
    pub attribs: &'a [&'a AFVertexAttrib],
}

pub struct AFVertexAttrib {
    pub location: u32,
    pub offset: u64,
    pub format: VertexFormat,
}

pub struct AFRenderPipelineConfig<'a> {
    // with bindgrouplayoutbinding, bindgrouplayout, bindingtype, and binding:

    // uniforms (TODO make modifiable later if dynamic and initializable with vals)
    // TODO samplers
    // TODO sampled texture (is multisampled?)
    // TODO storage texture
    // TODO storage stuff (modify later if dynamic)
    pub uniforms: &'a [&'a AFUniform],

    // with vertexbufferdescriptor and vertexattributedescriptor:

    // vertex buffers (format, offset, stride, step_mode)
    // NOT the index buffer
    // NOT the actual buffers; will be given in mainloop
    pub vertex_buffer_slots: &'a [&'a AFVertexBufferSlot<'a>],

    // other schtuff specified below
    pub vertex_shader: &'a AFShaderModule<'a>,
    pub fragment_shader: &'a AFShaderModule<'a>,
    pub index_format: IndexFormat,
    pub primitive_topology: PrimitiveTopology,
    pub front_face: FrontFace,
    pub cull_mode: CullMode,
    pub colour_blend: BlendDescriptor,
    pub alpha_blend: BlendDescriptor,
}

pub struct AFRenderPipeline {
    uniform_buffers: HashMap<u32, Buffer>,
}

// contains a hashmap of uniforms;
// cleared at the end of every pipeline call with
// the data copied into a new hashmap
// for the purpose of circumventing issues with
// reference lifetimes
static mut TEMP_UNIFORM_MAP: Option<HashMap<u32, Buffer>> = None;
static mut TEMP_VERTEX_ATTRIBS: Option<Vec<VertexAttributeDescriptor>> = None;

impl AFRenderPipeline {
    pub fn new(context: &AFContext, config: &AFRenderPipelineConfig) -> Self {
        // uniforms, samplers, and storages

        // this is JUST for uniforms
        let uniform_binding_layouts: Vec<BindGroupLayoutBinding> = config
            .uniforms
            .iter()
            .map(|uniform| BindGroupLayoutBinding {
                binding: uniform.id,
                visibility: uniform.stage,
                ty: BindingType::UniformBuffer {
                    dynamic: uniform.dynamic,
                },
            })
            .collect::<Vec<_>>();

        // compiled from uniforms, samplers, and storages
        let binding_layouts: &[BindGroupLayoutBinding] = uniform_binding_layouts.as_slice();

        let bind_group_layout =
            context
                .device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    bindings: binding_layouts,
                });

        // this is JUST for uniforms
        let mut real_uniform_map: HashMap<u32, Buffer> = HashMap::new();
        unsafe {
            match TEMP_UNIFORM_MAP {
                None => {
                    TEMP_UNIFORM_MAP = Some(HashMap::new());
                }
                Some(..) => {
                    //
                }
            }

            let uniform_bindings: Vec<Binding> = config
                .uniforms
                .iter()
                .map(|uniform| {
                    TEMP_UNIFORM_MAP.as_mut().unwrap().insert(
                        uniform.id,
                        create_empty_buffer(
                            context,
                            uniform.byte_size as usize,
                            BufferUsage::UNIFORM,
                        ),
                    );
                    Binding {
                        binding: uniform.id,
                        resource: BindingResource::Buffer {
                            buffer: TEMP_UNIFORM_MAP
                                .as_ref()
                                .unwrap()
                                .get(&uniform.id.clone())
                                .unwrap(),
                            range: 0..uniform.byte_size,
                        },
                    }
                })
                .collect::<Vec<_>>();

            for uniform in config.uniforms {
                real_uniform_map.insert(
                    uniform.id,
                    TEMP_UNIFORM_MAP
                        .as_mut()
                        .unwrap()
                        .remove(&uniform.id)
                        .unwrap(),
                );
            }
        }

        // vertex buffers

        // creating the actual pipeline

        let pipeline_layout_desc: PipelineLayoutDescriptor = PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
        }; // TODO allow for multiple bind group layouts
        let pipeline_layout: PipelineLayout =
            context.device.create_pipeline_layout(&pipeline_layout_desc);

        let colour_blend: BlendDescriptor = BlendDescriptor {
            src_factor: config.colour_blend.src_factor,
            dst_factor: config.colour_blend.dst_factor,
            operation: config.colour_blend.operation,
        };
        let alpha_blend: BlendDescriptor = BlendDescriptor {
            src_factor: config.alpha_blend.src_factor,
            dst_factor: config.alpha_blend.dst_factor,
            operation: config.alpha_blend.operation,
        };

        let vertex_buffer_descriptors: Vec<VertexBufferDescriptor> = config
            .vertex_buffer_slots
            .iter()
            .map(|slot| unsafe {
                TEMP_VERTEX_ATTRIBS = Option::Some(
                    slot.attribs
                        .iter()
                        .map(|attrib| VertexAttributeDescriptor {
                            offset: attrib.offset,
                            format: attrib.format,
                            shader_location: attrib.location,
                        })
                        .collect::<Vec<_>>(),
                );

                VertexBufferDescriptor {
                    stride: slot.stride,
                    step_mode: slot.step_mode,
                    attributes: TEMP_VERTEX_ATTRIBS.as_mut().unwrap().as_slice(),
                }
            })
            .collect::<Vec<_>>();
        let vertex_buffer_descriptors: &[VertexBufferDescriptor] =
            vertex_buffer_descriptors.as_slice();

        let render_pipeline: RenderPipeline =
            context
                .device
                .create_render_pipeline(&RenderPipelineDescriptor {
                    layout: &pipeline_layout,
                    vertex_stage: ProgrammableStageDescriptor {
                        module: &config.vertex_shader.module,
                        entry_point: config.vertex_shader.entry,
                    },
                    fragment_stage: Option::Some(ProgrammableStageDescriptor {
                        module: &config.fragment_shader.module,
                        entry_point: config.fragment_shader.entry,
                    }),
                    rasterization_state: Option::Some(RasterizationStateDescriptor {
                        front_face: config.front_face,
                        cull_mode: config.cull_mode,
                        depth_bias: 0,
                        depth_bias_slope_scale: 0.0,
                        depth_bias_clamp: 0.0,
                    }),
                    primitive_topology: config.primitive_topology,
                    color_states: &[ColorStateDescriptor {
                        format: TextureFormat::Bgra8UnormSrgb,
                        color_blend: colour_blend,
                        alpha_blend,
                        write_mask: ColorWrite::ALL,
                    }],
                    depth_stencil_state: None,
                    index_format: config.index_format,
                    vertex_buffers: vertex_buffer_descriptors,
                    sample_count: 1,
                    sample_mask: !0,
                    alpha_to_coverage_enabled: false,
                });

        // pipeline creation
        return AFRenderPipeline {
            uniform_buffers: real_uniform_map,
        };
    }
}

pub struct AFMainloopComputeCommand {

    // a stub, TODO implement with compute pipelines later

}

pub enum AFMainloopRenderType<'a> {

    Empty {vertices: u32, calls: u32},
    Vertex {vertices: u32, calls: u32, vertex_buffers: &'a [u8]},
    Index {indices: u32, calls: u32, vertex_buffers: &'a [u8], index_buffer: &'a [u8]},

}

pub struct AFMainloopRenderCommand {

    pub pipeline_index: usize,
    pub enabled_bind_groups: Range<usize>,
    pub clear_colour: [f64; 4],

}

// returned by the mainloop closure
pub struct AFMainloop {

    pub destroy: bool,
    pub update_surface: bool,

}

// sent to the mainloop closure each time called
// represents the state of the window, NOT events
// sent that update the state
pub struct AFMainloopState<'a> {

    pub size: PhysicalSize<u32>,
    pub close_requested: bool,
    pub focused: bool,
    pub file_hovered: &'a Vec<PathBuf>,
    pub was_resized: bool,
    pub events: Vec<AFMainloopInputEvent>,
    pub pressed: &'a Vec<VirtualKeyCode>,
    pub clicked: &'a Vec<VirtualKeyCode>,

}

// sent to the mainloop closure in
// a vector of others - represent
// single events, NOT the general
// state of the window
pub enum AFMainloopInputEvent {

    FileDropped(PathBuf),
    KeyClicked(), // TODO add whatever represents the key into here

}

// mainloop state here

static mut SIZE: Option<PhysicalSize<u32>> = None;
static mut FOCUSED: bool = false;
static mut CLOSE_REQUESTED: bool = false;
static mut WAS_RESIZED: bool = false;
static mut FILE_HOVERED: Option<Vec<PathBuf>> = None;
static mut DROPPED_FILE: Option<Vec<PathBuf>> = None;
static mut DOWN_KEYS: Option<Vec<VirtualKeyCode>> = None;
static mut CLICKED_KEYS: Option<Vec<VirtualKeyCode>> = None;

// mainloop function
pub fn mainloop<F: 'static, K: 'static>(context: &'static mut AFContext, window: AFWindow,
                   pipelines: &[AFRenderPipeline], mainloop_function: F, on_destroy: K)
    where F: Fn(AFMainloopState) -> AFMainloop, K: Fn() -> () {
    let event_loop = window.event_loop;
    let window = window.window;

    let mut surface = wgpu::Surface::create(&window);
    let mut swap_chain: SwapChain =
        context.device.create_swap_chain(&surface, &SwapChainDescriptor{
            usage: TextureUsage::OUTPUT_ATTACHMENT,
            format: TextureFormat::Bgra8UnormSrgb,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: context.present_mode,
        });

    unsafe {
        SIZE = Option::Some(window.inner_size());
        FILE_HOVERED = Option::Some(Vec::new());
        DROPPED_FILE = Option::Some(Vec::new());
        DOWN_KEYS = Option::Some(Vec::new());
        CLICKED_KEYS = Option::Some(Vec::new());
    }

    event_loop.run(move |event, _, control_flow|{
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {window_id, event} => {
                match event {
                    // some of these events are cached to maintain
                    // a state of the window that is passed
                    // to the mainloop closure;

                    // others of these events are sent
                    // as events to the mainloop closure because
                    // they need to be noticed only once
                    WindowEvent::CloseRequested => unsafe {
                        CLOSE_REQUESTED = true;
                    }
                    WindowEvent::Resized(physical_size) => unsafe {
                        SIZE = Option::Some(physical_size);
                        WAS_RESIZED = true;
                    }
                    WindowEvent::Focused(focused) => unsafe {
                        FOCUSED = focused;
                    }
                    WindowEvent::AxisMotion {device_id, axis, value} => {
                        //
                    }
                    WindowEvent::DroppedFile(path_buffer) => unsafe {
                        FILE_HOVERED = Option::Some(Vec::new());
                        DROPPED_FILE.as_mut().unwrap().push(path_buffer);
                    }
                    WindowEvent::HoveredFile(path_buffer) => unsafe {
                        FILE_HOVERED.as_mut().unwrap().push(path_buffer);
                    }
                    WindowEvent::HoveredFileCancelled => unsafe {
                        FILE_HOVERED = Option::Some(Vec::new());
                    }
                    WindowEvent::KeyboardInput {device_id, input, is_synthetic} => {
                        // is_synthetic is windows exclusive
                        match input.state {
                            ElementState::Pressed => unsafe {
                                let key: VirtualKeyCode = input.virtual_keycode.unwrap();
                                DOWN_KEYS.as_mut().unwrap().push(key);
                                CLICKED_KEYS.as_mut().unwrap().push(key);
                            }
                            ElementState::Released => unsafe {
                                let key: VirtualKeyCode = input.virtual_keycode.unwrap();
                                let index = DOWN_KEYS.as_ref().unwrap().iter()
                                    .position(|x| *x == key).unwrap();
                                DOWN_KEYS.as_mut().unwrap().remove(index);
                            }
                        }
                    }
                    WindowEvent::MouseInput {device_id, state, button, modifiers} => {
                        //
                    }
                    WindowEvent::Touch(touch) => {
                        //
                    }
                    WindowEvent::TouchpadPressure {device_id, pressure, stage} => {
                        //
                    }
                    WindowEvent::MouseWheel {device_id, delta, phase, modifiers} => {
                        //
                    }
                    WindowEvent::Moved(physical_position) => {
                        //
                    }
                    WindowEvent::ThemeChanged(theme) => { // windows exclusive
                        //
                    }
                    WindowEvent::ScaleFactorChanged {scale_factor, new_inner_size} => {
                        //
                    }
                    WindowEvent::CursorEntered {device_id} => {
                        //
                    }
                    WindowEvent::CursorMoved {device_id, position, modifiers} => {
                        //
                    }
                    WindowEvent::CursorLeft {device_id} => {
                        //
                    }
                    _ => {}
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(window_id) => {
                // something in here is freezing the window after a bit
                let dropped_files: Vec<AFMainloopInputEvent> =
                    unsafe {
                        DROPPED_FILE.as_mut().unwrap().clone().iter().map(|path_buf|{
                            AFMainloopInputEvent::FileDropped(path_buf.clone())
                        }).collect::<Vec<_>>()
                    };

                let mainloop_data: AFMainloop = mainloop_function(AFMainloopState{
                    size: unsafe {SIZE.unwrap()},
                    close_requested: unsafe {CLOSE_REQUESTED},
                    focused: unsafe {FOCUSED},
                    file_hovered: unsafe {FILE_HOVERED.as_ref().unwrap()},
                    events: dropped_files,
                    was_resized: unsafe {WAS_RESIZED},
                    pressed: unsafe {DOWN_KEYS.as_ref().unwrap()},
                    clicked: unsafe {CLICKED_KEYS.as_ref().unwrap()},
                });

                unsafe {
                    DROPPED_FILE = Option::Some(Vec::new());
                    WAS_RESIZED = false;
                    CLICKED_KEYS = Option::Some(Vec::new());
                };

                match mainloop_data.destroy {
                    true => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                }

                match mainloop_data.update_surface {
                    true => {
                        surface = wgpu::Surface::create(&window);
                        swap_chain = context.device.create_swap_chain(&surface, &SwapChainDescriptor{
                            usage: TextureUsage::OUTPUT_ATTACHMENT,
                            format: TextureFormat::Bgra8UnormSrgb,
                            width: window.inner_size().width,
                            height: window.inner_size().height,
                            present_mode: context.present_mode,
                        });
                    }
                    _ => {}
                }

                // draw here

                let mut command_encoder: CommandEncoder = context.device.create_command_encoder(
                    &CommandEncoderDescriptor {
                        todo: 0,
                    });

                {
//                    let mut render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor{
//                        color_attachments: &[RenderPassColorAttachmentDescriptor{
//                            attachment: (),
//                            resolve_target: None,
//                            load_op: (),
//                            store_op: (),
//                            clear_color: (),
//                        }],
//                        depth_stencil_attachment: None,
//                    });
                }

                let command_buffer: CommandBuffer = command_encoder.finish();
                context.queue.submit(&[command_buffer]);
            }
            Event::LoopDestroyed => {
                on_destroy();
            }
            _ => {}
        }
    });
}
