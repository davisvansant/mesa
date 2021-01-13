pub async fn mesa_erode() -> Result<(), Box<dyn std::error::Error>> {
    let plan = crate::plan::MesaPlan::excavate().await;
    match plan {
        Ok(plan_details) => {
            mesa::strata::docker_local::DockerLocal::erode(plan_details.name, plan_details.version)
                .await?
        }
        Err(error) => println!("mesa erode | unable to read plan : {}", error),
    }

    Ok(())
}
