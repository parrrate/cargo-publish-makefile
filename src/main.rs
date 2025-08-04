use clap::Parser;

fn main() -> anyhow::Result<()> {
    // https://github.com/onur/cargo-license/blob/57cfb0b73aa2d48fec9fcf172e7bcb439ce465ed/src/main.rs#L258-L265
    let args = std::env::args().enumerate().filter_map(|(i, x)| {
        if (i, x.as_str()) == (1, "publish-makefile") {
            None
        } else {
            Some(x)
        }
    });
    cargo_publish_makefile::Args::parse_from(args).run()
}
