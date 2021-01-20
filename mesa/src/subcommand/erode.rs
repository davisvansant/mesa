pub async fn erode() -> Result<(), Box<dyn std::error::Error>> {
    let plan = crate::plan::MesaPlan::excavate().await;
    match plan {
        Ok(plan_details) => crate::strata::docker_local::DockerLocal::erode(plan_details).await?,
        Err(error) => println!("mesa erode | unable to read plan : {}", error),
    }

    Ok(())
}
