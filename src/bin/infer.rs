use anyhow::Result;
use burn::{backend::WebGpu, data::dataset::Dataset};
use clap::Parser;
use logic_hammer::inference;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    /// From whence to load checkpoints.
    #[arg(long = "artifacts", default_value = "./artifacts", env)]
    artifacts: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    inference::infer::<WebGpu>(
        &args.artifacts,
        Default::default(),
        burn::data::dataset::vision::MnistDataset::test()
            .get(42)
            .unwrap(),
    )
}
