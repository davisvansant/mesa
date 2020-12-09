pub async fn mesa_survey() {
    mesa_strata::docker_local::DockerLocal::survey().await;
}
