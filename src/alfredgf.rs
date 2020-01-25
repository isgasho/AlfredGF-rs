use wgpu::{
    read_spirv, Adapter, BackendBit, BindGroup, BindGroupDescriptor, BindGroupLayout,
    BindGroupLayoutBinding, BindGroupLayoutDescriptor, Binding, BindingResource, BindingType,
    BlendDescriptor, Buffer, BufferDescriptor, BufferUsage, ColorStateDescriptor, ColorWrite,
    CullMode, Device, DeviceDescriptor, Extensions, FrontFace, IndexFormat, InputStepMode, Limits,
    PipelineLayout, PipelineLayoutDescriptor, PowerPreference, PresentMode, PrimitiveTopology,
    ProgrammableStageDescriptor, Queue, RasterizationStateDescriptor, RenderPipeline,
    RenderPipelineDescriptor, RequestAdapterOptions, ShaderModule, ShaderStage, Surface, SwapChain,
    SwapChainDescriptor, TextureFormat, TextureUsage, VertexAttributeDescriptor,
    VertexBufferDescriptor, VertexFormat,
};

use winit::{
    dpi::PhysicalSize,
    event_loop::{EventLoop, ControlFlow},
    window::{Icon, Window, WindowBuilder},
    event::{WindowEvent, Event, KeyboardInput},
};

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
    swap_chain: SwapChain,
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

                    let size = PhysicalSize::new(config.size[0], config.size[1]);
                    let swap_chain: SwapChain = device.create_swap_chain(
                        &surface,
                        &SwapChainDescriptor {
                            usage: TextureUsage::OUTPUT_ATTACHMENT,
                            format: TextureFormat::Bgra8UnormSrgb,
                            width: size.width as u32,
                            height: size.height as u32,
                            present_mode,
                        },
                    );

                    CURRENT_CONTEXT = Option::Some(AFContext {
                        size,
                        surface,
                        device,
                        queue,
                        swap_chain,
                    });
                }
                Some(afcontext) => {
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

// taken from the mainloop closure
pub struct AFMainloop {

    //

}

// sent to the mainloop closure each time called
// represents the state of the window, NOT events
// sent that update the state
pub struct AFMainloopState {

    resized: Option<PhysicalSize<u32>>,
    close_requested: bool,
    focused: bool,
    file_hovered: Option<PathBuf>,

}

pub fn mainloop<F>(context: &'static AFContext, window: AFWindow,
                   pipelines: &[AFRenderPipeline], mainloop_function: F)
    where F: Fn() -> AFMainloop {
    let event_loop = window.event_loop;
    let under_window = window.window;

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
                    WindowEvent::CloseRequested => {
                        //
                    }
                    WindowEvent::Resized(physical_size) => {
                        //
                    }
                    WindowEvent::Focused(focused) => {
                        //
                    }
                    WindowEvent::AxisMotion {device_id, axis, value} => {
                        //
                    }
                    WindowEvent::DroppedFile(path_buffer) => {
                        //
                    }
                    WindowEvent::HoveredFile(path_buffer) => {
                        //
                    }
                    WindowEvent::HoveredFileCancelled => {
                        //
                    }
                    WindowEvent::KeyboardInput {device_id, input, is_synthetic} => {
                        // is_synthetic is windows exclusive
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
            Event::RedrawRequested(window_id) => {
                // draw here
            }
            Event::LoopDestroyed => {
                //
            }
            _ => {}
        }
    });
}
