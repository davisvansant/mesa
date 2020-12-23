use crate::plan::MesaPlan;

pub async fn mesa_survey() -> Result<(), Box<dyn std::error::Error>> {
    MesaPlan::init().await;
    mesa_strata::docker_local::DockerLocal::survey().await?;
    Ok(())
}
