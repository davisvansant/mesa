use serde::{Deserialize, Serialize};
use std::fs::write;

#[derive(Deserialize, Serialize)]
pub struct MesaPlan {
    pub name: String,
    pub version: String,
    pub language: Language,
}

impl MesaPlan {
    pub async fn init() {
        let plan: MesaPlan = MesaPlan {
            name: String::from("default_mesa_plan_name"),
            version: String::from("default_mesa_plan_version"),
            language: Language {
                name: SupportedLanguage::Rust,
                version: String::from("1.48.0"),
            },
        };
        let toml = toml::to_string(&plan).unwrap();
        write("./target/test.toml", toml).unwrap();
        println!("toml created");
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Language {
    name: SupportedLanguage,
    version: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum SupportedLanguage {
    Rust,
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

            [language]
            name = "Rust"
            version = "1.48.0"
            "#,
        )
        .unwrap();
        assert_eq!(test_mesa_plan.name, String::from("test_mesa_plan"));
        assert_eq!(test_mesa_plan.version, String::from("0.1.0"));
        assert_eq!(test_mesa_plan.language.name, SupportedLanguage::Rust);
        assert_eq!(test_mesa_plan.language.version, String::from("1.48.0"));
    }
}
