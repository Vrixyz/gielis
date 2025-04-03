use bevy_reflect_derive::Reflect;

#[derive(Reflect, Debug, Clone, Copy)]
pub struct Gielis {
    pub a: f32,
    pub b: f32,
    pub m: f32,
    pub n1: f32,
    pub n2: f32,
    pub n3: f32,
}

impl Gielis {
    pub fn rphi(&self, phi: f32) -> f32 {
        let xp = (((self.m * phi) / 4f32).cos() / self.a).abs().powf(self.n2);
        let yp = (((self.m * phi) / 4f32).sin() / self.b).abs().powf(self.n3);

        (xp + yp).powf(-1.0 / self.n1)
    }
}

pub fn xy_from_phi(rphi: f32, phi: f32) -> (f32, f32) {
    let x = rphi * phi.cos();
    let y = rphi * phi.sin();
    (x, y)
}

pub fn xyz_from_phi_and_theta(r1: f32, r2: f32, theta: f32, phi: f32) -> (f32, f32, f32) {
    let x = r1 * theta.cos() * r2 * phi.cos();
    let y = r1 * theta.sin() * r2 * phi.cos();
    let z = r2 * phi.sin();
    (x, y, z)
}
