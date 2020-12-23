pub async fn mesa_erode() -> Result<(), Box<dyn std::error::Error>> {
    let plan = crate::plan::MesaPlan::excavate().await?;
    mesa_strata::docker_local::DockerLocal::erode(plan.name, plan.version).await?;
    Ok(())
}
