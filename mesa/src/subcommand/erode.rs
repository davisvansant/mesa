use crate::plan::MesaPlan;
use std::fs::read;

pub async fn mesa_erode() {
    let file = read("./target/test.toml").unwrap();
    let plan: MesaPlan = toml::from_slice(&file).unwrap();
    // let container = String::from("mesa_rust_1.48.0");
    // mesa_strata::docker_local::DockerLocal::erode(container).await;
    mesa_strata::docker_local::DockerLocal::erode(plan.name, plan.version).await;
}
