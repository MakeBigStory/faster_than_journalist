enum Wrap {
    /// Repeat texture
    Repeat,

    /// Repeat mirrored texture. **Unavailable on rectangle textures.**
    MirroredRepeat,

    ///
    /// Clamp to edge. Coordinates out of the range will be clamped to
    /// first / last column / row in given direction.
    ClampToEdge,

    /// Clamp to border color. Coordinates out of range will be clamped
    ClampToBorder,

    /// Mirror the texture once in negative coordinates and clamp to
    MirrorClampToEdge,
}
