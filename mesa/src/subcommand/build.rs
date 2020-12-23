// use crate::plan::MesaPlan;
// use std::fs::read;

pub async fn mesa_build() -> Result<(), Box<dyn std::error::Error>> {
    // let file = read("./target/test.toml")?;
    // let plan: MesaPlan = toml::from_slice(&file)?;
    let plan = crate::plan::MesaPlan::excavate().await?;
    mesa_strata::docker_local::DockerLocal::build(
        plan.name,
        plan.version,
        plan.language.version,
        plan.formation.layer,
    )
    .await?;
    Ok(())
}
