use clap::Parser;
use rand::seq::SliceRandom;
use zxcvbn;

const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*";

#[derive(Parser, Debug)]
pub struct GenPasswordOpts {
    #[arg(short, long, default_value_t = 12)]
    pub len: u8,

    #[arg(long, default_value_t = false)]
    pub no_number: bool,

    #[arg(long, default_value_t = false)]
    pub no_symbol: bool,

    #[arg(long, default_value_t = false)]
    pub no_lower: bool,

    #[arg(long, default_value_t = false)]
    pub no_upper: bool,
}

pub fn process(opts: &GenPasswordOpts) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut chars = Vec::new();
    let mut pwd = Vec::new();

    if !opts.no_lower {
        chars.extend_from_slice(LOWER);
    }

    if !opts.no_upper {
        chars.extend_from_slice(UPPER);
    }

    if !opts.no_number {
        chars.extend_from_slice(NUMBER);
    }

    if !opts.no_symbol {
        chars.extend_from_slice(SYMBOL);
    }

    for _ in 0..opts.len {
        let c = chars.choose(&mut rng).unwrap();
        pwd.push(*c);
    }

    pwd.shuffle(&mut rng);
    let pwd_str = String::from_utf8(pwd)?;
    println!("{}", pwd_str);

    let estimate = zxcvbn::zxcvbn(&pwd_str, &[])?;
    eprintln!("Password Strength: {}", estimate.score());
    Ok(())
}
