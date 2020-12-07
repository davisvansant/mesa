use serde::Deserialize;

#[derive(Deserialize)]
pub struct MesaPlan {
    pub name: String,
    pub version: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plan() {
        let test_mesa_plan: MesaPlan = toml::from_str(
            r#"
            name = "test_mesa_plan"
            version = "0.1.0"
            "#,
        )
        .unwrap();
        assert_eq!(test_mesa_plan.name, String::from("test_mesa_plan"));
        assert_eq!(test_mesa_plan.version, String::from("0.1.0"));
    }
}
