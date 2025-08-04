use cargo_publish_makefile::Args;
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Cmd {
    PublishMakefile(Args),
}

#[derive(Parser)]
struct Wrapped {
    #[clap(subcommand)]
    cmd: Cmd,
}

fn main() -> anyhow::Result<()> {
    if std::env::current_exe()?.ends_with("cargo") {
        let Cmd::PublishMakefile(args) = Wrapped::parse().cmd;
        args
    } else {
        cargo_publish_makefile::Args::parse()
    }
    .run()
}
