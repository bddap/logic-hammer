use crate::{data::MnistBatcher, training::TrainingConfig};
use anyhow::{Context, Result};
use burn::{
    data::{dataloader::batcher::Batcher, dataset::vision::MnistItem},
    prelude::*,
    record::{DefaultRecorder, Recorder},
};
use std::path::Path;

pub fn infer<B: Backend>(artifact_dir: &Path, device: B::Device, item: MnistItem) -> Result<()> {
    let config = TrainingConfig::load(artifact_dir.join("config.json"))
        .context("loading config (did you run training first?)")?;

    let record = DefaultRecorder::new()
        .load(artifact_dir.join("model"), &device)
        .context("loading record (did you run training first?)")?;

    let model = config.model.init::<B>(&device).load_record(record);

    let label = item.label;
    let batcher = MnistBatcher::default();
    let batch = batcher.batch(vec![item], &device);
    let output = model.forward(batch.images);
    let predicted = output.argmax(1).flatten::<1>(0, 1).into_scalar();

    println!("Predicted {predicted} Expected {label}");

    Ok(())
}
