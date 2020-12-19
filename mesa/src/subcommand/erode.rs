use crate::plan::MesaPlan;
use std::fs::read;

pub async fn mesa_erode() {
    let file = read("./target/test.toml").unwrap();
    let plan: MesaPlan = toml::from_slice(&file).unwrap();
    mesa_strata::docker_local::DockerLocal::erode(plan.name, plan.version).await;
}
