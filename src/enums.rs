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
