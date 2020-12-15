pub async fn mesa_view() {
    let container = String::from("amazon/aws-lambda-provided:al2");
    mesa_strata::docker_local::DockerLocal::view(container).await;
}
