#[derive(Debug, Clone)]
pub struct Color {
    pub R: f32,
    pub G: f32,
    pub B: f32,
    pub A: f32,
}

impl  Color {
    pub fn new(R: f32, G: f32, B: f32, A: f32) -> Color{
        Color {
            R,
            G,
            B,
            A,
        }
    }

    pub fn new_from_slice(RGBA: &[f32]) -> Color {
        if RGBA.len() < 4 {
            panic!("Color: new_from_slice slice length don't equal to 4 ");
        }

        Color{
            R: RGBA[0],
            B: RGBA[1],
            G: RGBA[2],
            A: RGBA[3],
        }
    }

    pub fn setColor(&mut self, R: f32, G: f32, B: f32, A: f32) {
        self.R = R;
        self.G = G;
        self.B = B;
        self.A = A;
    }
}