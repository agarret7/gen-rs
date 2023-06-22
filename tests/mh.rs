use std::fs::{write, create_dir_all};
use std::rc::Rc;
use rand::rngs::ThreadRng;

use genark::{GenerativeFunction,Trace,ChoiceBuffer,ChoiceHashMap};
use genark::types_2d::{Bounds,Point};

pub mod pointed;
use pointed::{PointedModel, DriftProposal};

#[test]
fn test_metropolis_hastings() -> std::io::Result<()> {
    create_dir_all("data")?;

    let mut rng = ThreadRng::default();
    const NUM_ITERS: usize = 100000;

    let model = &PointedModel { obs_std: 0.25 };
    let proposal = &DriftProposal { drift_std: 0.025 };
    let bounds = Rc::new(Bounds { xmin: -1., xmax: 1., ymin: -1., ymax: 1. });
    let obs = Point { x: 0., y: 0. };

    let mut constraints = ChoiceHashMap::<Point>::new();
    constraints.set_value("obs", &Rc::new(obs));

    let trace = Rc::new(model.generate(&mut rng, bounds.clone(), constraints));

    let data = *trace.get_choices()["latent"];
    let json = serde_json::to_string(&data)?;
    write("data/initial_mcmc_trace.json", json)?;

    for i in 0..NUM_ITERS {
        let (trace, accepted) = genark::mh(&mut rng, model, trace.clone(), proposal, bounds.clone());
        dbg!(accepted);
        let data = *trace.get_choices()["latent"];
        let json = serde_json::to_string(&data)?;
        write("data/mcmc_trace.json", json)?;
    }
    
    Ok(())
}