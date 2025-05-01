use crate::{
    core::company::CompanySymbol,
    simulation::settings::{SimulationSettings, SimulationSettingsBuilder},
    storage::config_file::StorageConfigFileImpl,
};
use clap::{ArgAction, Command};
use prettytable::{row, Table};
use std::fmt::{self, Display, Formatter};

pub struct Cli;

pub enum Action {
    StartServer(SimulationSettings),
}

impl Display for CompanySymbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for SimulationSettings {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut table = Table::new();

        table.add_row(row![cH2 -> "Simulation settings"]);
        table.add_row(row!["Max orders per tick", r -> self.max_orders_per_tick]);
        table.add_row(row!["Flush storage", r -> self.flush_storage]);
        table.add_row(row![
            "URL",
            format!("http://{}:{}", self.address, self.port)
        ]);

        write!(f, "{}", table.to_string().trim())
    }
}

impl Cli {
    pub async fn parse() -> Action {
        let matches = Command::new("market-sim")
            .about("Market Simulator")
            .subcommand_required(true)
            .arg_required_else_help(true)
            .subcommand(
                Command::new("start")
                    .about("Start the simulator")
                    .arg(
                        clap::Arg::new("flush_storage")
                            .short('f')
                            .long("flush_storage")
                            .action(ArgAction::SetTrue),
                    )
                    .arg(
                        clap::Arg::new("max_orders_per_tick")
                            .short('o')
                            .long("max_orders_per_tick")
                            .required(false),
                    )
                    .arg(
                        clap::Arg::new("address")
                            .short('a')
                            .long("address")
                            .required(false),
                    )
                    .arg(clap::Arg::new("port").long("port").required(false))
                    .arg(
                        clap::Arg::new("redis-url")
                            .long("redis-url")
                            .required(false),
                    )
                    .arg(
                        clap::Arg::new("prometheus-url")
                            .long("prometheus-url")
                            .required(false),
                    ),
            )
            .get_matches();

        match matches.subcommand() {
            Some(("start", sub_matches)) => {
                let max_orders_per_tick = sub_matches
                    .get_one::<String>("max_orders_per_tick")
                    .cloned()
                    .map(|s| {
                        s.parse::<u64>().unwrap_or_else(|_| {
                            eprintln!("Invalid value for max_orders_per_tick");
                            std::process::exit(1);
                        })
                    });

                let flush_storage = sub_matches.get_one::<bool>("flush_storage").cloned();

                let address = sub_matches.get_one::<String>("address").cloned();
                let port = sub_matches.get_one::<String>("port").cloned();
                let redis_url = sub_matches.get_one::<String>("redis-url").cloned();
                let prometheus_url = sub_matches.get_one::<String>("prometheus-url").cloned();

                let simulation_settings = SimulationSettingsBuilder {
                    address,
                    flush_storage,
                    max_investor_age: None,
                    max_orders_per_tick,
                    port,
                    prometheus_job_name: None,
                    prometheus_url,
                    redis_url,
                }
                .load_from_storage(&StorageConfigFileImpl)
                .await
                .unwrap_or_else(|e| {
                    eprintln!("Failed to load simulation settings: {}", e);
                    std::process::exit(1);
                });

                Action::StartServer(simulation_settings)
            }
            _ => {
                std::process::exit(1);
            }
        }
    }
}
