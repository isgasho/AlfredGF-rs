use crate::util_structs::*;

#[derive(Copy, Clone)]
pub enum AFBackendLibrary {
    Vulkan,
    Metal,
    OpenGL,
    DX12,
    DX11,
}

#[derive(Copy, Clone)]
pub enum AFPowerPreference {
    LowPower,
    Default,
    HighPower,
}

#[derive(Copy, Clone)]
pub enum AFShaderStage {
    None,
    Vertex,
    Fragment,
    Compute,
}

#[derive(Copy, Clone)]
pub enum AFVertexStepMode {
    PerVertex,
    PerInstance,
}

#[derive(Copy, Clone)]
pub enum AFVertexFormat {
    Float,
    Float2,
    Float3,
    Float4,
    UnsignedInt,
    UnsignedInt2,
    UnsignedInt3,
    UnsignedInt4,
    Int,
    Int2,
    Int3,
    Int4,
}

#[derive(Copy, Clone)]
pub enum AFUniformType {
    Buffer,
    Sampler,
    Storage,
}

#[derive(Copy, Clone)]
pub enum AFIndexFormat {
    UnsignedInt16,
    UnsignedInt32,
}

#[derive(Copy, Clone)]
pub enum AFDrawablePrimitive {
    Points,
    Lines,
    LineStrip,
    Triangles,
    TriangleStrip,
}

#[derive(Copy, Clone)]
pub enum AFBlendOperation {
    Add,
    Subtract,
    ReverseSubtract,
    Min,
    Max,
}

#[derive(Copy, Clone)]
pub enum AFBlendFactor {
    Zero,
    One,
    SrcColour,
    OneMinusSrcColour,
    SrcAlpha,
    OneMinusSrcAlpha,
    DstColour,
    OneMinusDstColour,
    DstAlpha,
    OneMinusDstAlpha,
    SrcAlphaSaturated,
    BlendColour,
    OneMinusBlendColour,
}

#[derive(Copy, Clone)]
pub enum AFDirection {
    Clockwise,
    CounterClockwise,
}

#[derive(Copy, Clone)]
pub enum AFFace {
    Front,
    Back,
}

pub enum AFRenderCommandType {
    Empty,
    Vertex {
        vertex_data: Vec<Vec<u8>>,
    },
    Indices {
        vertex_data: Vec<Vec<u8>>,
        index_data: Vec<u8>,
    },
}

pub enum AFWindowCommand {
    UpdateSurface,
    ResizeWindow(AFSize2D<u32>),
    DestroyWindow,
}

#[derive(Copy, Clone)]
pub enum AFWindowSize {

    MonitorSize,
    Size(AFSize2D<u32>),

}
