pub async fn view() -> Result<(), Box<dyn std::error::Error>> {
    let plan = crate::plan::MesaPlan::excavate().await;
    match plan {
        Ok(plan_details) => crate::strata::docker_local::DockerLocal::view(plan_details).await?,
        Err(error) => println!("mesa view | unable to read plan : {}", error),
    }

    Ok(())
}
