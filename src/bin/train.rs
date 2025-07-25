#![recursion_limit = "131"]
use burn::{
    backend::{Autodiff, WebGpu},
    data::dataset::Dataset,
    optim::AdamConfig,
};
use logic_hammer::{
    inference,
    model::ModelConfig,
    training::{self, TrainingConfig},
};

fn main() {
    type MyBackend = WebGpu<f32, i32>;
    type MyAutodiffBackend = Autodiff<MyBackend>;

    let device = burn::backend::wgpu::WgpuDevice::default();

    let artifact_dir = "/tmp/guide";

    training::train::<MyAutodiffBackend>(
        artifact_dir,
        TrainingConfig::new(ModelConfig::new(10, 512), AdamConfig::new()),
        device.clone(),
    );

    inference::infer::<MyBackend>(
        artifact_dir,
        device,
        burn::data::dataset::vision::MnistDataset::test()
            .get(42)
            .unwrap(),
    );
}
