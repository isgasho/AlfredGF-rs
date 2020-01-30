use crate::config_structs::*;
use crate::implementation::*;
use crate::enums::*;

pub trait AFWindowConstructor {

    fn new(config: &AFWindowConfig) -> Self;

}

pub trait AFContextConstructor<Window> {

    fn new(window: Window, config: &AFContextConfig) -> Self;

}

pub trait AFShaderConstructor<Context> {

    fn new(context: &Context, config: &AFShaderConfig) -> Self;

}

pub trait AFRenderPipelineConstructor<Context> {

    fn new(context: &Context, config: &AFRenderPipelineConfig) -> Self;

}

pub trait AFRenderCommandConstructor<Pipeline> {

    fn new(pipeline: &Pipeline, config: &AFRenderCommandConfig) -> Self;

}
