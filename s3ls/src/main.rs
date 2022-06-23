use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    #[structopt(name = "BUCKET")]
    bucket: String,
}

fn main() {
    let opt = Opt::from_args();
    println!("{}", opt.bucket);
}
