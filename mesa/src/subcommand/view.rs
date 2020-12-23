pub async fn mesa_view() -> Result<(), Box<dyn std::error::Error>> {
    let plan = crate::plan::MesaPlan::excavate().await?;
    mesa_strata::docker_local::DockerLocal::view(plan.name, plan.version).await?;
    Ok(())
}
