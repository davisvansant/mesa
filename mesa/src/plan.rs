use serde::{Deserialize, Serialize};
use std::fs::write;

#[derive(Deserialize, Serialize)]
pub struct MesaPlan {
    pub name: String,
    pub version: String,
}

impl MesaPlan {
    pub async fn init() {
        let plan: MesaPlan = MesaPlan {
            name: String::from("default_mesa_plan_name"),
            version: String::from("default_mesa_plan_version"),
        };
        let toml = toml::to_string(&plan).unwrap();
        write("./target/test.toml", toml).unwrap();
        println!("toml created");
    }
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
