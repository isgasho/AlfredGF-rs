use crate::enums::*;
use crate::util_structs::{AFImage, AFSize2D};
use std::ops::Range;

pub struct AFWindowConfig<'a> {
    pub icon: Option<&'a AFImage<'a>>,
    pub start_size: AFWindowSize,
    pub min_size: AFWindowSize,
    pub max_size: AFWindowSize,
    pub decorated: bool,
    pub resizeable: bool,
    pub visible: bool,
    pub always_on_top: bool,
    pub transparent: bool,
    pub maximized: bool,
    pub fullscreen: bool,
    pub title: &'a str,
    pub monitor_chooser: Option<fn(Vec<AFMonitor>) -> usize>,
}

#[derive(Copy, Clone)]
pub struct AFContextConfig {
    pub vsync: bool,
    pub anisotropic_filtering: bool,
    pub backend_lib: AFBackendLibrary,
    pub power_preference: AFPowerPreference,
}

#[derive(Copy, Clone)]
pub struct AFShaderConfig<'a> {
    pub stage: AFShaderStage,
    pub bytecode: &'a [u8],
    pub entry_point: &'a str,
}

#[derive(Copy, Clone)]
pub struct AFVertexAttrib {
    pub location: u32,
    pub offset: u32,
    pub vertex_format: AFVertexFormat,
}

#[derive(Copy, Clone)]
pub struct AFVertexBuffer<'a> {
    pub stride: u64,
    pub step_mode: AFVertexStepMode,
    pub attribs: &'a [AFVertexAttrib],
}

#[derive(Copy, Clone)]
pub struct AFUniform {
    pub location: u32,
    pub stage: AFShaderStage,
    pub dynamic: bool,
    pub byte_size: u32,
    pub uniform_type: AFUniformType,
}

#[derive(Copy, Clone)]
pub struct AFUniformGroup<'a> {
    pub set: u32,
    pub uniforms: &'a [AFUniform],
}

#[derive(Copy, Clone)]
pub struct AFBlendDescriptor {
    pub src_factor: AFBlendFactor,
    pub dst_factor: AFBlendFactor,
    pub operation: AFBlendOperation,
}

#[derive(Copy, Clone)]
pub struct AFRenderPipelineConfig<'a> {
    pub uniforms: &'a [AFUniformGroup<'a>],
    pub vertex_buffer_slots: &'a [AFVertexBuffer<'a>],
    pub colour_blend: AFBlendDescriptor,
    pub alpha_blend: AFBlendDescriptor,
    pub primitive: AFDrawablePrimitive,
    pub front_face: AFDirection,
    pub cull_mode: Option<AFFace>,
    pub index_format: AFIndexFormat,
}

pub struct AFRenderCommandConfig {
    colour: [f64; 4],
    vertex_count: u32,
    enabled_uniform_groups: Range<u32>,
    calls: u32,
    command_type: AFRenderCommandType,
}

pub struct AFMonitor {

    pub size: AFSize2D<u32>,
    pub position: AFSize2D<i32>,
    pub name: Option<String>,
    pub scale_factor: f64,

}

#[derive(Copy, Clone)]
pub struct AFMainloopState {
    //
}

pub trait AFMainloop {
    fn mainloop<F: 'static, T: 'static>(context: Self, on_loop: F, on_finish: T)
    where
        F: Fn(AFMainloopState) -> (),
        T: Fn() -> ();
}
