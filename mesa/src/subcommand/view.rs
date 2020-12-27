pub async fn mesa_view() -> Result<(), Box<dyn std::error::Error>> {
    let plan = crate::plan::MesaPlan::excavate().await;
    match plan {
        Ok(plan_details) => {
            mesa_strata::docker_local::DockerLocal::view(plan_details.name, plan_details.version)
                .await?
        }
        Err(error) => println!("mesa view | unable to read plan : {}", error),
    }

    Ok(())
}
