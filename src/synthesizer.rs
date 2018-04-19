use rand::{Rng, thread_rng};

use std::f64::consts::PI;

const TAU: f64 = PI * 2.0;

pub struct Synthesizer {
    voices: Vec<Voice>,
}

impl Synthesizer {
    pub fn new() -> Synthesizer {
        Synthesizer {
            voices: Vec::new(),
        }
    }

    pub fn voice(&mut self, voice: Voice) {
      self.voices.push(voice);
    }

    pub fn sample(&self, time: f64) -> f64 {
      self.voices.iter().map(|v| v.sample(time)).sum::<f64>()
    }
}

pub struct Voice {
    amplitude: f64,
    frequency: f64,
    kind:      VoiceKind,
}

pub enum VoiceKind {
    Noise,
    Saw,
    Sine,
    Square,
    Triangle,
}

impl Voice {
  pub fn new() -> Voice {
    Voice {
      amplitude: 0.0,
      frequency: 0.0,
      kind:      VoiceKind::Sine,
    }
  }

  pub fn amplitude(mut self, amplitude: f64) -> Voice {
    self.amplitude = amplitude;
    self
  }

  pub fn frequency(mut self, frequency: f64) -> Voice {
    self.frequency = frequency;
    self
  }

  pub fn kind(mut self, kind: VoiceKind) -> Voice {
    self.kind = kind;
    self
  }

  pub fn sample(&self, time: f64) -> f64 {
    let period = 1.0 / self.frequency;

    let t = time % period / period;

    use self::VoiceKind::*;

    let unscaled = match self.kind {
      Noise    => thread_rng().gen::<f64>() * 2.0 - 1.0,
      Saw      => if t < 0.5 { t * 2.0 } else { t * 2.0 - 2.0 },
      Sine     => (t * TAU).sin(),
      Square   => if t < 0.5 { 1.0 } else { -1.0 },
      Triangle => {
        let x = t * 4.0;
        if x < 1.0 {
          x
        } else if x < 3.0 {
          -x + 2.0
        } else {
          x - 4.0
        }
      }
    };

    unscaled * self.amplitude
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use std::f64::EPSILON;

  const DX: f64 = EPSILON * 2.0;

  // use float_cmp::{Ulps, ApproxEq};
  use float_cmp::ApproxEq;

  fn approx_eq(a: f64, b: f64) -> bool {
    a.approx_eq(&b, DX, 2)
  }

  #[test]
  fn triangle() {
    let v = Voice::new().amplitude(1.0).frequency(1.0).kind(VoiceKind::Triangle);
    assert!(approx_eq(v.sample(0.000),  0.0), "v.sample(0.000): {}", v.sample(0.000));
    assert!(approx_eq(v.sample(0.250),  1.0), "v.sample(0.250): {}", v.sample(0.250));
    assert!(approx_eq(v.sample(0.500),  0.0), "v.sample(0.500): {}", v.sample(0.500));
    assert!(approx_eq(v.sample(0.750), -1.0), "v.sample(0.750): {}", v.sample(0.750));
    assert!(approx_eq(v.sample(1.000),  0.0), "v.sample(1.000): {}", v.sample(1.000));
    assert!(approx_eq(v.sample(1.250),  1.0), "v.sample(1.250): {}", v.sample(1.250));
    assert!(approx_eq(v.sample(1.500),  0.0), "v.sample(1.500): {}", v.sample(1.500));
    assert!(approx_eq(v.sample(1.750), -1.0), "v.sample(1.750): {}", v.sample(1.750));
    assert!(approx_eq(v.sample(2.000),  0.0), "v.sample(2.000): {}", v.sample(2.000));
  }
}
