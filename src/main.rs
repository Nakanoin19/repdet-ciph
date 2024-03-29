use structopt::StructOpt;
use nalgebra::Matrix2;
use rand::prelude::*;

const X2: [&[Matrix2<usize>]; 3] = [
    &[
        Matrix2::new(0, 1, 1, 0),
        Matrix2::new(1, 1, 1, 0),
        Matrix2::new(0, 1, 1, 1)
    ],
    &[
        Matrix2::new(0, 0, 0, 0),
        Matrix2::new(1, 0, 0, 0),
        Matrix2::new(0, 1, 0, 0),
        Matrix2::new(0, 0, 1, 0),
        Matrix2::new(0, 0, 0, 1),
        Matrix2::new(1, 1, 0, 0),
        Matrix2::new(0, 0, 1, 1),
        Matrix2::new(1, 0, 1, 0),
        Matrix2::new(0, 1, 0, 1),
        Matrix2::new(1, 1, 1, 1)
    ],
    &[
        Matrix2::new(1, 0, 0, 1),
        Matrix2::new(1, 1, 0, 1),
        Matrix2::new(1, 0, 1, 1)
    ]
];

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(default_value)]
    input: String,
    #[structopt(short, long, conflicts_with = "decode")]
    encode: bool,
    #[structopt(short, long, conflicts_with = "encode")]
    decode: bool,
    #[structopt(short, long)]
    line: bool
}

fn main() {
    let opt = Opt::from_args();
    if opt.encode {
        let len = opt.input.trim().len();
        let cols = if opt.line { len } else { (len as f64).sqrt().ceil() as usize };
        let rows = if opt.line { 1 } else { (len as f64 / cols as f64).ceil() as usize };
        let mats: &[_] = &opt.input.trim().chars().map(|c| c.to_digit(3).unwrap_or(0) as usize).map(|c| X2[c]).map(|c| c.choose(&mut thread_rng()).unwrap()).collect::<Vec<_>>();
        let pad = cols * rows - len;
        for row in 0..rows {
            for i in 0..=1 {
                for col in 0..cols {
                    let j = row * cols + col;
                    if j >= pad {
                        let v = mats[j - pad].row(i).transpose();
                        print!("{}{}", v[0], v[1])
                    } else {
                        if i == 0 {
                            print!("01");
                        } else {
                            print!("10");
                        }
                    }
                }
                println!()
            }
        }
    }
    if opt.decode {
        let lines = opt.input.trim().lines().collect::<Vec<_>>();
        let bins = [
            lines.iter().step_by(2).map(|&c| c).collect::<String>().chars().map(|c| c.to_digit(2).unwrap() as isize).collect::<Vec<_>>(),
            lines.iter().skip(1).step_by(2).map(|&c| c).collect::<String>().chars().map(|c| c.to_digit(2).unwrap() as isize).collect::<Vec<_>>()
        ];
        println!("{}", bins[0].chunks(2).zip(bins[1].chunks(2)).map(|(c, d)| c[0] * d[1] - c[1] * d[0] + 1).map(|c| c.to_string()).collect::<String>());
    }
}
