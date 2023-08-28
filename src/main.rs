mod config_parser {

    use serde::Deserialize;
    use std::{fmt::Error, fs};
    use toml;

    #[derive(Deserialize, Debug)]
    struct WalletTracked {
        name: String,
        address: String,
        chains: Vec<String>,
    }

    #[derive(Deserialize, Debug)]
    struct Chain {
        name: String,
        minimum_native_amount_in_ether: f64,
    }

    #[derive(Deserialize, Debug)]
    pub struct Config {
        wallets: Vec<WalletTracked>,
        chains: Vec<Chain>,
    }

    pub fn parse_configs() -> Result<Config, String> {
        let contents = fs::read_to_string("config.toml").expect("failed to read config file");
        let configs: Config = toml::from_str(&contents).expect("Failed parsing configs");

        validate_chains(&configs)?;

        Ok(configs)
    }

    fn get_chains(configs: &Config) -> Vec<String> {
        let mut chains = Vec::<String>::new();
        for chain in configs.chains.iter() {
            chains.push(chain.name.to_string());
        }
        chains
    }

    fn validate_chains(configs: &Config) -> Result<(), String> {
        let configured_chains = &get_chains(configs);

        for wallet in &configs.wallets {
            for chain in &wallet.chains {
                if !configured_chains.contains(chain) {
                    Err("Invalid chain in walet")
                }
            }
        }

        Ok(())
    }
}

fn main() {
    let configs: config_parser::Config = config_parser::parse_configs().unwrap();
}
