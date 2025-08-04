use clap::Parser;

fn main() -> anyhow::Result<()> {
    cargo_publish_makefile::Args::parse().run()
}