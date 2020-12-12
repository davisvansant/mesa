pub async fn mesa_erode() {
    let container = String::from("tester");
    mesa_strata::docker_local::DockerLocal::erode(container).await;
}
