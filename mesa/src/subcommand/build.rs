use crate::plan::MesaPlan;
use std::fs::read;

pub async fn mesa_build() {
    let file = read("./target/test.toml").unwrap();
    let plan: MesaPlan = toml::from_slice(&file).unwrap();
    // let tag: MesaPlan = toml::from_slice(&file).unwrap();
    println!("{:?}", plan.language.name);
    println!("{:?}", plan.language.version);
    // let build = mesa_strata::docker_local::DockerLocal::build(plan.name);
    // match build {
    //     Ok(output) => println!("{}", String::from_utf8(output.stdout).unwrap()),
    //     Err(error) => println!("{}", error),
    // };
    mesa_strata::docker_local::DockerLocal::build(plan.name, plan.version).await;
}
