use std::fs::{write, create_dir_all};
use std::rc::Rc;
use rand::rngs::ThreadRng;

use genark::modeling::dists::{Distribution, categorical};
use genark::{Trace,ChoiceBuffer,ChoiceHashMap};
use genark::types_2d;
use pointed::{PointedModel, PointedTrace};

pub mod pointed;

#[test]
fn test_importance_sampling() -> std::io::Result<()> {
    create_dir_all("data")?;

    let mut rng = ThreadRng::default();

    let num_samples = 100000;

    let model = PointedModel { obs_std: 0.25 };
    let bounds = types_2d::Bounds { xmin: -1., xmax: 1., ymin: -1., ymax: 1. };
    let obs = types_2d::Point { x: 0., y: 0. };

    let mut constraints = ChoiceHashMap::new();
    constraints.set_value("obs", &Rc::new(obs));
    PointedTrace::new(Rc::new(bounds), constraints.clone(), 0.);

    let (traces, log_normalized_weights, log_ml_estimate) = 
        genark::importance_sampling(&mut rng, model, Rc::new(bounds), constraints, num_samples);

    dbg!(log_ml_estimate);

    let data = traces.iter().map(|tr| *tr.get_choices()["latent"]).collect::<Vec<types_2d::Point>>();
    let json = serde_json::to_string(&data)?;
    write("data/initial_traces.json", json)?;

    let probs = &log_normalized_weights.iter()
        .map(|w| (w - log_ml_estimate).exp())
        .collect::<Vec<f32>>();
    let traces = (0..num_samples/10)
        .map(|_| categorical.random(&mut rng, probs))
        .map(|idx| &traces[idx])
        .collect::<Vec<&PointedTrace>>();
    
    let data = traces.iter().map(|tr| *tr.get_choices()["latent"]).collect::<Vec<types_2d::Point>>();
    let json = serde_json::to_string(&data)?;
    write("data/resampled_traces.json", json)?;

    Ok(())
}