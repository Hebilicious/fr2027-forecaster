//! Seeded pseudo-random numbers and the few distributions the model draws from.
//!
//! The generator and samplers live here rather than in a dependency so that a seeded run
//! reproduces exactly across dependency upgrades: xoshiro256++ seeded through SplitMix64,
//! Marsaglia's polar method for normals and Marsaglia–Tsang for gamma variates.

/// A xoshiro256++ generator.
#[derive(Clone, Debug)]
pub struct Rng {
    state: [u64; 4],
    spare_normal: Option<f64>,
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        let mut splitmix = seed;
        let mut next = || {
            splitmix = splitmix.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = splitmix;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        };
        Self {
            state: [next(), next(), next(), next()],
            spare_normal: None,
        }
    }

    /// An independent stream for one stage of a run, so adding draws to one stage never shifts
    /// the numbers another stage sees.
    pub fn derive(seed: u64, label: &str) -> Self {
        // FNV-1a over the label, mixed into the seed.
        let mut hash: u64 = 0xCBF2_9CE4_8422_2325;
        for byte in label.bytes() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
        }
        Self::new(seed ^ hash.rotate_left(17))
    }

    pub fn next_u64(&mut self) -> u64 {
        let s = &mut self.state;
        let result = s[0].wrapping_add(s[3]).rotate_left(23).wrapping_add(s[0]);
        let t = s[1] << 17;
        s[2] ^= s[0];
        s[3] ^= s[1];
        s[1] ^= s[2];
        s[0] ^= s[3];
        s[2] ^= t;
        s[3] = s[3].rotate_left(45);
        result
    }

    /// Uniform on [0, 1).
    pub fn uniform(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }

    /// Standard normal.
    pub fn normal(&mut self) -> f64 {
        if let Some(z) = self.spare_normal.take() {
            return z;
        }
        loop {
            let u = 2.0 * self.uniform() - 1.0;
            let v = 2.0 * self.uniform() - 1.0;
            let s = u * u + v * v;
            if s > 0.0 && s < 1.0 {
                let m = (-2.0 * s.ln() / s).sqrt();
                self.spare_normal = Some(v * m);
                return u * m;
            }
        }
    }

    /// Gamma with the given shape and unit scale.
    pub fn gamma(&mut self, shape: f64) -> f64 {
        assert!(shape > 0.0, "gamma shape must be positive");
        if shape < 1.0 {
            let u = self.uniform();
            return self.gamma(shape + 1.0) * u.powf(1.0 / shape);
        }
        let d = shape - 1.0 / 3.0;
        let c = 1.0 / (9.0 * d).sqrt();
        loop {
            let x = self.normal();
            let v = 1.0 + c * x;
            if v <= 0.0 {
                continue;
            }
            let v = v * v * v;
            let u = self.uniform();
            if u < 1.0 - 0.0331 * x.powi(4) {
                return d * v;
            }
            if u > 0.0 && u.ln() < 0.5 * x * x + d * (1.0 - v + v.ln()) {
                return d * v;
            }
        }
    }

    /// Student-t with `df` degrees of freedom, rescaled to unit standard deviation (`df > 2`).
    pub fn student_t_unit(&mut self, df: f64) -> f64 {
        assert!(
            df > 2.0,
            "a unit-variance Student-t needs more than 2 degrees of freedom"
        );
        let z = self.normal();
        let chi2 = 2.0 * self.gamma(df / 2.0);
        z / (chi2 / df).sqrt() * ((df - 2.0) / df).sqrt()
    }

    /// Index drawn with the given probabilities, or `None` with the remaining probability
    /// `1 - sum(probabilities)`.
    pub fn categorical(&mut self, probabilities: &[f64]) -> Option<usize> {
        let u = self.uniform();
        let mut cumulative = 0.0;
        for (index, p) in probabilities.iter().enumerate() {
            cumulative += p;
            if u < cumulative {
                return Some(index);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::Rng;

    fn moments(draws: &[f64]) -> (f64, f64) {
        let n = draws.len() as f64;
        let mean = draws.iter().sum::<f64>() / n;
        let var = draws.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1.0);
        (mean, var)
    }

    #[test]
    fn same_seed_same_stream() {
        let mut a = Rng::new(42);
        let mut b = Rng::new(42);
        for _ in 0..1000 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
        assert_ne!(Rng::new(1).next_u64(), Rng::new(2).next_u64());
    }

    #[test]
    fn derived_streams_differ_by_label() {
        assert_ne!(Rng::derive(7, "field").next_u64(), Rng::derive(7, "error").next_u64());
        assert_eq!(Rng::derive(7, "field").next_u64(), Rng::derive(7, "field").next_u64());
    }

    #[test]
    fn uniform_in_unit_interval() {
        let mut rng = Rng::new(3);
        let draws: Vec<f64> = (0..100_000).map(|_| rng.uniform()).collect();
        assert!(draws.iter().all(|u| (0.0..1.0).contains(u)));
        let (mean, var) = moments(&draws);
        assert!((mean - 0.5).abs() < 0.005);
        assert!((var - 1.0 / 12.0).abs() < 0.002);
    }

    #[test]
    fn normal_has_unit_moments() {
        let mut rng = Rng::new(11);
        let draws: Vec<f64> = (0..200_000).map(|_| rng.normal()).collect();
        let (mean, var) = moments(&draws);
        assert!(mean.abs() < 0.01, "mean {mean}");
        assert!((var - 1.0).abs() < 0.015, "var {var}");
    }

    #[test]
    fn gamma_mean_matches_shape() {
        let mut rng = Rng::new(5);
        for shape in [0.5, 1.0, 2.5, 10.0] {
            let draws: Vec<f64> = (0..100_000).map(|_| rng.gamma(shape)).collect();
            let (mean, var) = moments(&draws);
            assert!(
                (mean - shape).abs() < 0.03 * shape.max(1.0),
                "shape {shape} mean {mean}"
            );
            assert!((var - shape).abs() < 0.08 * shape.max(1.0), "shape {shape} var {var}");
        }
    }

    #[test]
    fn student_t_unit_has_unit_variance() {
        let mut rng = Rng::new(9);
        let draws: Vec<f64> = (0..400_000).map(|_| rng.student_t_unit(7.0)).collect();
        let (mean, var) = moments(&draws);
        assert!(mean.abs() < 0.01, "mean {mean}");
        assert!((var - 1.0).abs() < 0.05, "var {var}");
    }

    #[test]
    fn categorical_respects_remainder() {
        let mut rng = Rng::new(1);
        let n = 100_000;
        let mut counts = [0usize; 3];
        for _ in 0..n {
            match rng.categorical(&[0.2, 0.5]) {
                Some(i) => counts[i] += 1,
                None => counts[2] += 1,
            }
        }
        let freq: Vec<f64> = counts.iter().map(|c| *c as f64 / n as f64).collect();
        assert!((freq[0] - 0.2).abs() < 0.01);
        assert!((freq[1] - 0.5).abs() < 0.01);
        assert!((freq[2] - 0.3).abs() < 0.01);
    }
}
