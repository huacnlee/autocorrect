use crate::{CliOption, CONFIG_TEMPLATE};
use std::{fs, path::Path, time::Duration};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

static CONFIG_TEMPLATE_URL: &str =
    "https://github.com/huacnlee/autocorrect/raw/main/.autocorrectrc.template";

#[derive(Default)]
pub struct InitOption {
    pub force: bool,
    pub local: bool,
}

pub fn run(option: &CliOption, init_option: &InitOption) {
    if Path::exists(Path::new(&option.config_file)) && !init_option.force {
        log::warn!("{} already exists.", option.config_file);
        return;
    }

    let mut template = CONFIG_TEMPLATE.to_string();

    if !init_option.local {
        match fetch_config_template() {
            Ok(out) => {
                template = out;
            }
            Err(e) => {
                log::error!("Fetch config template error: \n\n{}", e);
                log::error!("\nTry use --local init config without remote download.\n\n  autocorrect init --local\n");
                return;
            }
        }
    }

    log::info!("AutoCorrect init config: {}", option.config_file);
    fs::write(Path::new(&option.config_file), template)
        .unwrap_or_else(|_| panic!("Failed to write config file: {}", &option.config_file));
}

pub fn fetch_config_template() -> Result<String> {
    log::info!("Fetching {}", CONFIG_TEMPLATE_URL);

    let client = reqwest::blocking::Client::builder()
        .connect_timeout(Duration::from_secs(5))
        .build()?;

    let resp = client
        .get(CONFIG_TEMPLATE_URL)
        .timeout(Duration::from_secs(10))
        .send()?;

    if !resp.status().is_success() {
        return Err(format!("Fetch config template error: {}", resp.status()).into());
    }

    let body = resp.text()?;

    Ok(body)
}
