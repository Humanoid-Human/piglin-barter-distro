const DENOM: f64 = 459.0;

pub struct Dist {
    pub max: usize,
    pub pdf: Vec<f64>,
    pub name: String
}

impl Dist {
    fn new(name: &str, max: usize) -> Dist {
        Dist {
            max,
            pdf: vec![0.0; max + 1],
            name: String::from(name)
        }
    }

    pub fn from_bounds(name: &str, max: usize, min: usize, chance: f64) -> Dist {
        let mut out = Dist::new(name, max);
        out.pdf[0] = (DENOM - chance) / DENOM;
        let item_chance = chance / DENOM / ((1 + max - min) as f64);
        for v in &mut out.pdf[min..] {
            *v = item_chance;
        }
        out
    }

    pub fn add(&mut self, other: &Dist) {
        let new_max: usize = self.max + other.max;
        let mut new_pdf: Vec<f64> = Vec::new();
        for i in 0..new_max+1 {
            let mut sum: f64 = 0.0;
            for j in 0..i+1 {
                let self_val: f64 = if j <= self.max {self.pdf[j]} else {0.0};
                let other_val: f64 = if i - j <= other.max {other.pdf[i-j]} else {0.0};
                sum += self_val * other_val;
            }
            new_pdf.push(sum);
        }

        self.max = new_max;
        self.pdf = new_pdf;
    }

    pub fn add_self(&mut self) {
        let new_max: usize = self.max * 2;
        let mut new_pdf: Vec<f64> = Vec::new();
        for i in 0..new_max+1 {
            let mut sum: f64 = 0.0;
            for j in 0..i+1 {
                sum += (if j <= self.max {self.pdf[j]} else {0.0})
                        * (if i - j <= self.max {self.pdf[i-j]} else {0.0})
            }
            new_pdf.push(sum);
        }

        self.max = new_max;
        self.pdf = new_pdf;
    }

    pub fn clone(&self) -> Dist {
        Dist {
            name: (&self.name).clone(),
            max: self.max,
            pdf: (&self.pdf).clone(),
        }
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
}