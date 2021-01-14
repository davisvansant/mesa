pub async fn mesa_build(ignore_tests: bool) -> Result<(), Box<dyn std::error::Error>> {
    let plan = mesa::plan::MesaPlan::excavate().await;

    match plan {
        Ok(plan_details) => {
            mesa::strata::docker_local::DockerLocal::build(plan_details, ignore_tests).await?
        }
        Err(error) => println!("mesa build | unable to read plan : {}", error),
    }

    Ok(())
}
