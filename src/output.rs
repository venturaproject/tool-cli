use clap::ValueEnum;
use colored::Colorize;

#[derive(Debug, Clone, Copy, ValueEnum, Default, PartialEq)]
pub enum OutputFormat {
    #[default]
    Plain,
    Json,
    Table,
}

impl OutputFormat {
    /// Print key-value pairs respecting the current format.
    pub fn print_kv(&self, pairs: &[(&str, &str)]) {
        match self {
            Self::Json => {
                let map: serde_json::Map<String, serde_json::Value> = pairs
                    .iter()
                    .map(|(k, v)| (k.to_string(), serde_json::Value::String(v.to_string())))
                    .collect();
                println!(
                    "{}",
                    serde_json::to_string_pretty(&serde_json::Value::Object(map)).unwrap()
                );
            }
            Self::Table => {
                let width = pairs.iter().map(|(k, _)| k.len()).max().unwrap_or(0);
                for (k, v) in pairs {
                    println!("{:<width$}  {}", k, v);
                }
            }
            Self::Plain => {
                for (k, v) in pairs {
                    println!("{} {}", format!("{}:", k).bold(), v);
                }
            }
        }
    }
}
