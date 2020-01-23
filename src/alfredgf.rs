use wgpu::{
    read_spirv, Adapter, BackendBit, BindGroup, BindGroupDescriptor, BindGroupLayout,
    BindGroupLayoutBinding, BindGroupLayoutDescriptor, Binding, BindingType, Buffer, BufferUsage,
    Device, DeviceDescriptor, Extensions, Limits, PipelineLayoutDescriptor, PowerPreference, Queue,
    RequestAdapterOptions, ShaderModule, ShaderStage, Surface, BufferDescriptor, BindingResource,
    PrimitiveTopology, FrontFace, CullMode, BlendDescriptor, IndexFormat, VertexBufferDescriptor,
};

use winit::{
    dpi::PhysicalSize,
    event_loop::EventLoop,
    window::{Icon, Window, WindowBuilder},
};

use std::collections::HashMap;
use std::ops::Range;

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
                    let t_w = Option::Some(AFWindow { window, event_loop });

                    CURRENT_WINDOW = t_w;
                }
                Some(afwindow) => {
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
                    })
                    .unwrap();

                    let (device, queue): (Device, Queue) =
                        adapter.request_device(&DeviceDescriptor {
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
    let buffer: Buffer = context.device.create_buffer(&BufferDescriptor{
        size: size as u64,
        usage,
    });

    return buffer;
}

static mut v_bs: Option<HashMap<u32, Buffer>> = None;

impl AFBindGroup {
    // TODO rewrite this so it takes in an AFBinding
    // TODO also update the specs later
    // TODO create a buffer in here and save it in a hashmap; let them be initialized?
    pub fn new(context: &AFContext, af_bindings: &[AFBinding]) -> Self {
        unsafe {
            v_bs = Option::Some(HashMap::new());
        }
        let mut shader_ids: Vec<u32> = Vec::new();
        let binding_layouts: Vec<BindGroupLayoutBinding> = af_bindings
            .iter()
            .map(|af_binding: &AFBinding| {
                shader_ids.push(af_binding.id);
                BindGroupLayoutBinding {
                    binding: af_binding.id,
                    visibility: af_binding.visibility,
                    ty: match af_binding.binding {
                        AFBindingType::Buffer {
                            dynamic, readonly, ..
                        } => BindingType::StorageBuffer { dynamic, readonly },
                    },
                }
            })
            .collect::<Vec<_>>();
        let binding_layouts = binding_layouts.as_slice();

        let layout: BindGroupLayout =
            context
                .device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    bindings: binding_layouts,
                });

        // TODO find out if bindings are actually necessary and when exactly
        let bindings: Vec<Binding> = af_bindings.iter().map(|af_binding: &AFBinding| {
            match af_binding.binding {
                AFBindingType::Buffer { size, dynamic, readonly } => {
                    unsafe {
                        v_bs.as_mut().unwrap().insert(af_binding.id,
                                             create_empty_buffer(context, size, BufferUsage::STORAGE));
                        // NOTE THAT THE BUFFER USAGE IS SPECIFIED WHEN MAKING A VERTEX BUFFER TO PASS, NOT HERE
                        Binding {
                            binding: 0,
                            resource: BindingResource::Buffer{
                                buffer: v_bs.as_mut().unwrap().get(&af_binding.id).unwrap(),
                                range: 0..size as u64,
                            },
                        }
                    }
                }
            }
        }).collect::<Vec<_>>();
        let bindings: &[Binding] = bindings.as_slice();

        let bind_group = context.device.create_bind_group(&BindGroupDescriptor {
            layout: &layout,
            bindings,
        });

        let mut n_m: HashMap<u32, Buffer> = HashMap::new();

        unsafe {
            for i in shader_ids.iter() {
                let b = v_bs.as_mut().unwrap().remove(&i).unwrap();
                n_m.insert(*i, b);
            }
        }

        return AFBindGroup {
            layout,
            bind_group,
            //index_buffer: None,
            vertex_buffers: n_m,
        };
    }
}

pub struct AFRenderPipelineConfig<'a> {

    pub bind_groups: &'a [BindGroupLayoutBinding],
    pub vertex_shader: &'a AFShaderModule<'a>,
    pub fragment_shader: &'a AFShaderModule<'a>,
    pub primitive_topology: PrimitiveTopology,
    pub front_face: FrontFace,
    pub cull_mode: CullMode,
    pub colour_blend: BlendDescriptor,
    pub alpha_blend: BlendDescriptor,
    pub index_format: IndexFormat,
    pub vertex_buffers: &'a [VertexBufferDescriptor<'a>]

}

pub struct AFRenderPipeline {

    //

}

impl AFRenderPipeline {
    fn new() -> Self {
        return AFRenderPipeline{
            //
        };
    }
}
