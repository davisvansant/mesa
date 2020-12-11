use crate::plan::MesaPlan;

pub async fn mesa_survey() {
    MesaPlan::init().await;
    mesa_strata::docker_local::DockerLocal::survey().await;
}
