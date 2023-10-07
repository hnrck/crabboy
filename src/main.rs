use clap::Parser;

#[derive(Parser)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"), about = env!("CARGO_PKG_DESCRIPTION"))]
struct Opt {
    #[clap(short = 'r', long = "rom", required = true)]
    rom: String,
}

fn main() {
    let opt = Opt::parse();
    let rom_path: &String = &opt.rom;
}
