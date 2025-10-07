mod dist;
use dist::*;

const GR_BS: ConstDist = ConstDist::from_bounds(16, 8, 40.0);
const SS_NB: ConstDist = ConstDist::from_bounds(8, 2, 40.0);
const QUARTZ: ConstDist = ConstDist::from_bounds(12, 5, 20.0);
const OBS: ConstDist = ConstDist::from_bounds(1, 1, 40.0);
const CRY: ConstDist = ConstDist::from_bounds(3, 1, 40.0);

pub fn calculate_dist(n: u16) -> [Dist; 5] {
    let gr_bs = Dist::from("Gravel & Blackstone", &GR_BS);
    let ss_nb = Dist::from("Soul Sand & Nether Brick", &SS_NB);
    let cry = Dist::from("Crying Obsidian", &CRY);
    let obs = Dist::from("Obsidian", &OBS);
    let quartz = Dist::from("Nether Quartz", &QUARTZ);

    let mut list = [gr_bs, ss_nb, cry, obs, quartz];

    if n == 1 { return list; }

    // skip the first 1, because we already have one of the pdf by default
    let start = 14 - n.leading_zeros();

    list.iter_mut().for_each(
        |dist| {
            // iterate over the bits of n
            for i in 0..start + 1 {
            dist.double();
            if (n >> (start - i)) & 1 != 0 {
                dist.add_original();
            }
        }
        }
    );

    list
}

pub fn print_dists(l: [Dist; 5]) {
    for d in l {
        d.print(vec![0.5, 0.9, 0.95, 0.99]);
    }
}