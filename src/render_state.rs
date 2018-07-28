use super::format::BlendMode;
use super::format::DepthMode;
use super::format::StencilMode;

#[derive(Debug)]
pub(crate) struct RenderState {
    blend_mode : BlendMode,
    depth_mode : DepthMode,
    stencil_mode : StencilMode
}

impl RenderState {

}