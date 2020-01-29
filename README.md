AlfredGF-rs
=
A functional API for low level rendering.

New spec:

    AFSize2D {
    
        width: u32
        height: u32
    
    }
    
    AFImage<'a> {
    
        size: AFSize2D
        data: &'a [u8]
    
    }
    
    AFWindowConfig<'a> {
    
        icon: &'a AFImage<'a>,
        start_size: AFSize2D
        max_size: AFSize2D
        min_size: AFSize2D
        resizeable: bool,
        visible: bool,
        always_on_top: bool,
        maximized: bool,
    
    }
    
    AFWindow {
    
        fn new(config: &AFWindowConfig) -> AFWindow
    
    }
    
    // bitwise combineable with bit or
    // ie. Metal | Vulkan
    enum AFBackendLibrary {
    
        Metal
        OpenGL
        Vulkan
        DX12
        DX11
    
    }
    
    AFContextConfig {
    
        vsync: bool,
        anisotropic_filtering: bool,
        backend_lib: AFBackendLibrary
    
    }
    
    AFContext {
    
        // absorbs the window
        fn new(window: AFWindow, config: &AFContextConfig) -> AFContext
        
    }
    
    enum AFShaderStage {
        
            None,
            Vertex,
            Fragment,
            Compute,
        
        }
    
    AFShaderModule {
    
        fn new(context: &AFContext, stage: AFShaderStage, spv_bytecode: &[u8], entry: &str) -> AFShaderModule
    
    }
    
    enum AFVertexStepMode {
    
        Vertex
        Instance
    
    }
    
    enum AFVertexFormat {
    
        Float
        Float2
        Float3
        Float4
        UnsignedInt
        UnsignedInt2
        UnsignedInt3
        UnsignedInt4
        Int
        Int2
        Int3
        Int4
    
    }
    
    AFVertexAttrib {
        
        location: u32
        offset: u32
        vertex_format: AFVertexFormat
        
    }
    
    AFVertexBufferSlot<'a> {
    
        stride: u64,
        step_mode: AFVertexStepMode,
        attribs: &'a [AFVertexAttrib]
    
    }
    
    enum AFUniformType {
    
        Sampler,
        Buffer,
        Storage,
    
    }
    
    AFUniform {
    
        location: u32
        stage: AFShaderStage
        dynamic: bool
        byte_size: u32
    
    }
    
    enum AFIndexFormat {
    
        UnsignedInt16,
        UnsignedInt32,
    
    }
    
    enum AFDrawablePrimitive {
    
        Points,
        Lines,
        LineStrip,
        Triangles,
        TriangleStrip
    
    }
    
    enum AFBlendOperation {
    
        Add
        Subtract
        ReverseSubtract
        Min
        Max
    
    }
    
    enum AFBlendFactor {
    
        Zero
        One
        SrcColour
        OneMinusSrcColour
        SrcAlpha
        OneMinusSrcAlpha
        DstColour
        OneMinusDstColour
        DstAlpha
        OneMinusDstAlpha
        SrcAlphaSaturated
        BlendColour
        OneMinusBlendColour
    
    }
    
    AFBlendOperation {
    
        src_factor: AFBlendFactor
        dst_factor: AFBlendFactor
        operation: AFBlendOperation
    
    }
    
    AFFace {
    
        Clockwise
        CounterClockwise
    
    }
    
    AFCullMode {
    
        None
        Front
        Back
    
    }
        
    AFRenderPipelineConfig<'a> {
    
        uniforms: &'a [AFUniform]
        vertex_buffer_slots: &'a [VertexBufferSlot]
        colour_blend: AFBlendOperation
        alpha_blend: AFBlendOperation
        primitive: AFDrawablePrimitive
        front_face: AFFace
        cull_mode: AFCullMode
        index_format: AFIndexFormat
    
    }
