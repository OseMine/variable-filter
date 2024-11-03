pub struct Le13700Filter {
    cutoff: f32,
    resonance: f32,
    vca1: f32,
    vca2: f32,
    cap1: f32,
    cap2: f32,
}

impl Le13700Filter {
    pub fn new() -> Self {
        Self {
            cutoff: 1000.0,
            resonance: 0.0,
            vca1: 0.0,
            vca2: 0.0,
            cap1: 0.0,
            cap2: 0.0,
        }
    }

    pub fn set_params(&mut self, cutoff: f32, resonance: f32) {
        self.cutoff = cutoff.clamp(20.0, 20000.0);
        self.resonance = resonance.clamp(0.0, 1.0);
    }

    pub fn process(&mut self, input: f32, sample_rate: f32) -> f32 {
        let dt = 1.0 / sample_rate;
        let fc = self.cutoff / sample_rate;
        let k = 2.0 * std::f32::consts::PI * fc;
        
        let ota_gain = 1.0 - (-k * dt).exp();
        let feedback = self.resonance * 4.0;

        self.vca1 += (input - self.cap1 - feedback * self.cap2) * ota_gain;
        self.vca2 += (self.cap1 - self.cap2) * ota_gain;

        self.cap1 += self.vca1 * dt;
        self.cap2 += self.vca2 * dt;

        self.cap2
    }
}
