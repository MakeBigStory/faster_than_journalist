use buffer_state::BufferState;
use debug_state::DebugState;
use framebuffer_state::FramebufferState;
use query_state::QueryState;
use shader_program_state::ShaderProgramState;
use shader_state::ShaderState;
use transform_feedback_state::TransformFeedbackState;

/// Initializes context-based functionality
pub struct State {
    buffer: BufferState,
    // todo: context是否放到device类
//    context: ContextState,

    // todo: WebGL不支持debug
    debug: DebugState,
    framebuffer: FramebufferState,
    // todo： 后面再支持mesh
    //    mesh: MeshState,
    // todo: WebGL不支持debug
    query: QueryState,

    // todo: 是否放到device类
    //    renderer: RendererState,
    shader: ShaderState,
    shader_program: ShaderProgramState,
    texture: TextureState,
    // todo: es 2.0不支持
    transform_feedback: TransformFeedbackState,
}

impl State {}
