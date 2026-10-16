//! A linear-Gaussian state with the two Kalman operations the aggregation needs: random-walk
//! prediction and a linear observation update that also returns the observation's marginal
//! log-likelihood, used to choose hyperparameters.

use nalgebra::{DMatrix, DVector};

use crate::{ModelError, rng::Rng};

#[derive(Clone, Debug)]
pub struct Gaussian {
    pub mean: DVector<f64>,
    pub cov: DMatrix<f64>,
}

impl Gaussian {
    pub fn new(mean: DVector<f64>, cov: DMatrix<f64>) -> Self {
        assert_eq!(mean.len(), cov.nrows());
        assert_eq!(cov.nrows(), cov.ncols());
        Self { mean, cov }
    }

    pub fn dim(&self) -> usize {
        self.mean.len()
    }

    /// Random-walk prediction: each listed state's variance grows by `daily_variance * days`.
    pub fn predict_random_walk(&mut self, daily_variance: &[f64], days: f64) {
        assert_eq!(daily_variance.len(), self.dim());
        if days <= 0.0 {
            return;
        }
        for (i, variance) in daily_variance.iter().enumerate() {
            self.cov[(i, i)] += variance * days;
        }
    }

    /// Conditions on `y = H x + e`, `e ~ N(0, R)`, and returns `log p(y)` under the prior.
    pub fn update(&mut self, h: &DMatrix<f64>, y: &DVector<f64>, r: &DMatrix<f64>) -> Result<f64, ModelError> {
        let m = y.len();
        let innovation = y - h * &self.mean;
        let hp = h * &self.cov;
        let s = &hp * h.transpose() + r;
        let chol = s
            .cholesky()
            .ok_or(ModelError::NotPositiveDefinite("innovation covariance"))?;
        let gain_t = chol.solve(&hp);
        let weighted = chol.solve(&innovation);
        self.mean += hp.transpose() * &weighted;
        self.cov -= hp.transpose() * gain_t;
        symmetrize(&mut self.cov);
        let ln_det: f64 = (0..m).map(|i| chol.l_dirty()[(i, i)].ln()).sum::<f64>() * 2.0;
        Ok(-0.5 * (m as f64 * (2.0 * std::f64::consts::PI).ln() + ln_det + innovation.dot(&weighted)))
    }

    /// The marginal distribution of the leading `n` coordinates.
    pub fn leading(&self, n: usize) -> Gaussian {
        Gaussian::new(
            self.mean.rows(0, n).into_owned(),
            self.cov.view((0, 0), (n, n)).into_owned(),
        )
    }

    pub fn sampler(&self) -> Result<Sampler, ModelError> {
        let n = self.dim();
        let mut jitter = 0.0;
        for _ in 0..8 {
            let mut cov = self.cov.clone();
            for i in 0..n {
                cov[(i, i)] += jitter;
            }
            if let Some(chol) = cov.cholesky() {
                return Ok(Sampler {
                    mean: self.mean.clone(),
                    lower: chol.unpack(),
                });
            }
            jitter = if jitter == 0.0 { 1e-12 } else { jitter * 100.0 };
        }
        Err(ModelError::NotPositiveDefinite("state covariance"))
    }
}

fn symmetrize(matrix: &mut DMatrix<f64>) {
    let n = matrix.nrows();
    for i in 0..n {
        for j in (i + 1)..n {
            let average = 0.5 * (matrix[(i, j)] + matrix[(j, i)]);
            matrix[(i, j)] = average;
            matrix[(j, i)] = average;
        }
    }
}

/// Draws from a fixed multivariate normal through its Cholesky factor.
#[derive(Clone, Debug)]
pub struct Sampler {
    mean: DVector<f64>,
    lower: DMatrix<f64>,
}

impl Sampler {
    pub fn dim(&self) -> usize {
        self.mean.len()
    }

    /// Writes one draw into `out`, using `scratch` for the standard normals.
    pub fn draw(&self, rng: &mut Rng, scratch: &mut [f64], out: &mut [f64]) {
        let n = self.dim();
        for z in scratch.iter_mut().take(n) {
            *z = rng.normal();
        }
        for (i, value) in out.iter_mut().enumerate().take(n) {
            let row = self.lower.row(i);
            *value = self.mean[i] + (0..=i).map(|j| row[j] * scratch[j]).sum::<f64>();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scalar_update_matches_closed_form() {
        // Prior N(0, 4), observation y = 1 with variance 1: posterior mean 0.8, variance 0.8.
        let mut g = Gaussian::new(DVector::from_element(1, 0.0), DMatrix::from_element(1, 1, 4.0));
        let ll = g
            .update(
                &DMatrix::from_element(1, 1, 1.0),
                &DVector::from_element(1, 1.0),
                &DMatrix::from_element(1, 1, 1.0),
            )
            .unwrap();
        assert!((g.mean[0] - 0.8).abs() < 1e-12);
        assert!((g.cov[(0, 0)] - 0.8).abs() < 1e-12);
        // Marginal of y is N(0, 5).
        let expected = -0.5 * ((2.0 * std::f64::consts::PI * 5.0).ln() + 1.0 / 5.0);
        assert!((ll - expected).abs() < 1e-12);
    }

    #[test]
    fn contrast_observation_moves_both_states() {
        let mut g = Gaussian::new(DVector::zeros(2), DMatrix::identity(2, 2));
        let h = DMatrix::from_row_slice(1, 2, &[1.0, -1.0]);
        g.update(&h, &DVector::from_element(1, 1.0), &DMatrix::from_element(1, 1, 0.01))
            .unwrap();
        assert!(g.mean[0] > 0.45 && g.mean[1] < -0.45);
        // The common level is untouched by a contrast.
        assert!((g.mean[0] + g.mean[1]).abs() < 1e-12);
    }

    #[test]
    fn random_walk_adds_variance() {
        let mut g = Gaussian::new(DVector::zeros(2), DMatrix::identity(2, 2));
        g.predict_random_walk(&[0.5, 0.0], 4.0);
        assert_eq!(g.cov[(0, 0)], 3.0);
        assert_eq!(g.cov[(1, 1)], 1.0);
    }

    #[test]
    fn sampler_reproduces_covariance() {
        let cov = DMatrix::from_row_slice(2, 2, &[1.0, 0.6, 0.6, 2.0]);
        let g = Gaussian::new(DVector::from_vec(vec![1.0, -1.0]), cov);
        let sampler = g.sampler().unwrap();
        let mut rng = Rng::new(4);
        let n = 100_000;
        let mut scratch = [0.0; 2];
        let mut out = [0.0; 2];
        let (mut s0, mut s1, mut s00, mut s01, mut s11) = (0.0, 0.0, 0.0, 0.0, 0.0);
        for _ in 0..n {
            sampler.draw(&mut rng, &mut scratch, &mut out);
            s0 += out[0];
            s1 += out[1];
            s00 += out[0] * out[0];
            s01 += out[0] * out[1];
            s11 += out[1] * out[1];
        }
        let n = n as f64;
        let (m0, m1) = (s0 / n, s1 / n);
        assert!((m0 - 1.0).abs() < 0.02 && (m1 + 1.0).abs() < 0.02);
        assert!((s00 / n - m0 * m0 - 1.0).abs() < 0.03);
        assert!((s01 / n - m0 * m1 - 0.6).abs() < 0.03);
        assert!((s11 / n - m1 * m1 - 2.0).abs() < 0.05);
    }
}
