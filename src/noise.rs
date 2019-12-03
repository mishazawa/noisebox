const Q: i32 = 15;
const C1: f32 = (1 << Q) as f32 - 1.;
const C2: f32 = (C1 as i32 / 3) as f32 + 1.;
const C3: f32 = 1. / C1;

#[derive(Debug)]
pub struct Noise {
    noise: i32,
    seed: i32,
    carry: i32,
}

impl Noise {
    pub fn new() -> Self {
        Noise {
            noise: 0,
            seed: 34,
            carry: 857,
        }
    }

    pub fn shshsh(&mut self) -> f32 {
        self.noise = self.seed;
        self.noise >>= 3;
        self.noise ^= self.seed;
        self.carry = self.noise & 1;
        self.noise >>= 1;
        self.seed >>= 1;
        self.seed |= self.carry << 30;
        self.noise &= 0xFF;

        let rand = ((self.noise << 16) | (self.noise << 8) | self.noise) as f32 / 10_000_000.;
        ((2. * ((rand * C2) + (rand * C2) + (rand * C2)) - 3. * (C2 - 1.)) * C3) * 0.1
    }
}
