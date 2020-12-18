use crate::plan::MesaPlan;
use std::fs::read;

pub async fn mesa_view() {
    let file = read("./target/test.toml").unwrap();
    let plan: MesaPlan = toml::from_slice(&file).unwrap();
    // let container = String::from("amazon/aws-lambda-provided:al2");
    // mesa_strata::docker_local::DockerLocal::view(container).await;
    mesa_strata::docker_local::DockerLocal::view(plan.name, plan.version).await;
}
