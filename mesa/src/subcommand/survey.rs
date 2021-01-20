pub async fn survey() -> Result<(), Box<dyn std::error::Error>> {
    crate::plan::MesaPlan::init().await?;
    crate::strata::docker_local::DockerLocal::survey().await?;
    Ok(())
}
