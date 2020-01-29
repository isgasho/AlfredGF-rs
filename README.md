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
    
        Vertex,
        Instance
    
    }
    
    AFVertexAttrib {
        
            //
        
    }
    
    AFVertexBufferSlot<'a> {
    
        stride: u64,
        step_mode: AFVertexStepMode,
        attribs: &'a [AFVertexAttrib]
    
    }
    
    AFUniform {
    
        //
    
    }
    
    AFStorage {
    
        //
    
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
    
    AFRenderPipelineConfig {
    
        uniforms
        storage
        samplers
        vertex_buffer_slots
    
    }
