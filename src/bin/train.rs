use std::path::PathBuf;

use anyhow::Result;
use burn::{
    backend::{Autodiff, WebGpu},
    data::dataset::Dataset,
    optim::AdamConfig,
};
use clap::Parser;
use logic_hammer::{
    inference,
    model::ModelConfig,
    training::{self, TrainingConfig},
};

#[derive(Parser)]
struct Args {
    /// Where to store training data and checkpoints.
    #[arg(long = "artifacts", default_value = "./artifacts", env)]
    artifacts: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let device = burn::backend::wgpu::WgpuDevice::default();

    training::train::<Autodiff<WebGpu>>(
        &args.artifacts,
        TrainingConfig::new(ModelConfig::new(10, 512), AdamConfig::new()),
        device.clone(),
    );

    inference::infer::<WebGpu>(
        &args.artifacts,
        device,
        burn::data::dataset::vision::MnistDataset::test()
            .get(42)
            .unwrap(),
    )
}
