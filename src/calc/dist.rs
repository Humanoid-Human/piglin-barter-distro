use rayon::prelude::*;

const DENOM: f64 = 459.0;

pub struct Dist {
    pub max: usize,
    pub pdf: Vec<f64>,
    pub name: &'static str,
    pub original: &'static ConstDist
}

impl Dist {
    pub fn from(name: &'static str, d: &'static ConstDist) -> Dist {
        Dist {
            max: d.max,
            pdf: Vec::from(d.pdf),
            name,
            original: d
        }
    }

    pub fn add_original(&mut self) {
        let new_max: usize = self.max + self.original.max;
        let mut new_pdf: Vec<f64> = vec![0.0; new_max + 1];
        new_pdf.par_iter_mut().enumerate().for_each(
            |(i, v)| {
                let mut sum: f64 = 0.0;
                for j in 0..i+1 {
                    let self_val: f64 = if j <= self.max {self.pdf[j]} else {0.0};
                    let other_val: f64 = if i - j <= self.original.max {self.original.pdf[i-j]} else {0.0};
                    sum += self_val * other_val;
                }
                *v = sum;
            }
        );

        self.max = new_max;
        self.pdf = new_pdf;
    }

    pub fn double(&mut self) {
        let new_max: usize = self.max * 2;
        let mut new_pdf: Vec<f64> = vec![0.0; new_max + 1];

        new_pdf.par_iter_mut().enumerate().for_each(
            |(i, v)| {
                let mut sum: f64 = 0.0;
                for j in 0..i+1 {
                    sum += (if j <= self.max {self.pdf[j]} else {0.0})
                            * (if i - j <= self.max {self.pdf[i-j]} else {0.0})
                }
                *v = sum;
            }
        );

        self.max = new_max;
        self.pdf = new_pdf;
    }

    pub fn percentile(&self, n: f64) -> usize {
        let mut index = 0;
        let mut sum = n;
        while sum > 0.0 && index < self.max {
            sum -= self.pdf[index];
            index += 1;
        }
        index
    }

    pub fn print(&self, percentiles: Vec<f64>) {
        println!("{}:", self.name);
        for p in percentiles {
            let val = self.percentile(p);
            println!("    {}th percentile: {} ({}s + {})",
                (100.0 * p) as u8,
                val,
                val / 64,
                val % 64
            )
        }
    }
}

pub struct ConstDist {
    pdf: [f64; 17],
    max: usize
}

impl ConstDist {
    pub const fn from_bounds(max: usize, min: usize, chance: f64) -> ConstDist {
        let item_chance = chance / DENOM / ((1 + max - min) as f64);
        let mut pdf = [0.0; 17];
        pdf[0] = (DENOM - chance) / DENOM;
        let mut i = min;
        while i <= max {
            pdf[i] = item_chance;
            i += 1;
        }
        ConstDist { pdf, max }
    }
}