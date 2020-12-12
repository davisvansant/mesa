pub async fn mesa_erode() {
    let container = String::from("mesa_rust_1.48.0");
    mesa_strata::docker_local::DockerLocal::erode(container).await;
}
