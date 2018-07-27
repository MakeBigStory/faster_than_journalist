enum TransformFeedbackBufferMode {}

enum TransformFeedbackPrimitiveMode {
    /// Points.
    Points = GL_POINTS,

    /// Lines.
    Lines = GL_LINES,

    /// Triangles.
    Triangles = GL_TRIANGLES,
}

struct TransformFeedback {}

impl TransformFeedback {
    pub fn begin(mode: TransformFeedbackPrimitiveMode) {}

    /// Pause transform feedback
    ///
    /// Pausing transform feedback makes it inactive, allowing to use
    /// different shader, or starting another transform feedback.
    pub fn pause() {}

    /// Resume transform feedback
    ///
    ///  Resumes transform feedback so the next captured data are appended to
    ///  already captured ones. The restrictions specified for @ref begin()
    ///  still apply after resuming. Only one transform feedback object can
    ///  be active at a time.
    pub fn resume() {}

    /// End transform feedback
    /// transform feedback so the captured data can be used.
    pub fn end() {}
}
