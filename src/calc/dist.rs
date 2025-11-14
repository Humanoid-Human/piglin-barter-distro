use rayon::prelude::*;
use crate::DENOM;

pub struct Dist {
    pub max: usize,
    pub pdf: Vec<f64>,
    pub name: &'static str,
    pub original: &'static ConstDist
}

impl Dist {
    pub fn from(name: &'static str, d: &'static ConstDist) -> Dist {
        let mut v = vec![0.0; d.max + 1];
        v[0] = d.zero_chance;
        for item in v.iter_mut().skip(d.min) {
            *item = d.chance;
        }
        Dist {
            max: d.max,
            pdf: v,
            name,
            original: d
        }
    }

    pub fn add_original(&mut self) {
        let new_max = self.max + self.original.max;
        let mut new_pdf = vec![0.0; new_max + 1];
        new_pdf.par_iter_mut().enumerate().for_each(
            |(i, v)| {
                for j in 0..i+1 {
                    let self_val = if j <= self.max {self.pdf[j]} else {0.0};
                    let other_val =
                        if i == j {
                            self.original.zero_chance
                        } else if i - j <= self.original.max && i - j >= self.original.min {
                            self.original.chance
                        } else {
                            0.0
                        };
                    *v += self_val * other_val;
                }
            }
        );

        self.max = new_max;
        self.pdf = new_pdf;
    }

    // TODO: take advantage of the fact that it's adding to itself to speed this up.
    pub fn double(&mut self) {
        let new_max = self.max * 2;
        let mut new_pdf = vec![0.0; new_max + 1];

        new_pdf.par_iter_mut().enumerate().for_each(
            |(i, v)| {
                for j in 0..i+1 {
                    *v += (if j <= self.max {self.pdf[j]} else {0.0})
                        * (if i - j <= self.max {self.pdf[i-j]} else {0.0})
                }
            }
        );

        self.max = new_max;
        self.pdf = new_pdf;
    }

    // Find the value for the given percentile. Input should be in [0, 1]
    pub fn percentile(&self, n: f64) -> usize {
        if n == 1.0 { return self.max; }
        let mut index = 0;
        let mut sum = n;
        while sum > 0.0 && index <= self.max {
            sum -= self.pdf[index];
            index += 1;
        }
        index - 1
    }

    // print name and a list of given percentiles.
    pub fn print(&self, percentiles: &Vec<f64>) {
        println!("{}:", self.name);
        for p in percentiles {
            let val = self.percentile(*p);
            println!("    {:.2}th percentile: {} ({}s + {})",
                100.0 * p,
                val,
                val / 64,
                val % 64
            )
        }
    }

    pub fn to_csv(&self) -> String {
        let mut s = String::new();
        for (i, v) in self.pdf.iter().enumerate() {
            s.push_str(&format!("{},{},{}\n", self.name, i, *v));
        }
        s
    }
}

pub struct ConstDist {
    min: usize,
    max: usize,
    chance: f64,
    zero_chance: f64
}

impl ConstDist {
    pub const fn from_bounds(max: usize, min: usize, chance: f64) -> ConstDist {
        let roll = chance / DENOM;
        ConstDist { min, max, chance: roll / ((1 + max - min) as f64), zero_chance: 1.0 - roll }
    }
}