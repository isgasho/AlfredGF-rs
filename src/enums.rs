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
