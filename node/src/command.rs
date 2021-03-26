// Copyright (C) 2019-2021 Calcu Network Technologies Ltd.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

use crate::chain_spec;
use crate::cli::{Cli, Subcommand};
use crate::service as calcu_service;
use crate::executor::Executor;
use calcu_service::{new_partial, new_full, new_light};
use sc_service::PartialComponents;
use sc_cli::{SubstrateCli, RuntimeVersion, Role, ChainSpec};
use calcu_runtime::Block;

impl SubstrateCli for Cli {
    fn impl_name() -> String { "Calcu".into() }

    fn impl_version() -> String { env!("SUBSTRATE_CLI_IMPL_VERSION").into() }

    fn executable_name() -> String { "calcu".into() }

    fn description() -> String { env!("CARGO_PKG_DESCRIPTION").into() }

    fn author() -> String { env!("CARGO_PKG_AUTHORS").into() }

    fn support_url() -> String { "https://github.com/calcux/calcu/issues/new".into() }

    fn copyright_start_year() -> i32 { 2020 }

    fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
        Ok(match id {
            "dev" => Box::new(chain_spec::development_config()?),
            "" | "local" => Box::new(chain_spec::local_testnet_config()?),
            path => Box::new(chain_spec::CalcuChainSpec::from_json_file(
                std::path::PathBuf::from(path),
            )?),
        })
    }

    fn native_runtime_version(_chain_spec: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
        &calcu_runtime::VERSION
    }
}

/// Parse and run command line arguments
pub fn run() -> sc_cli::Result<()> {
    let cli = Cli::from_args();
    std::println!("Hello, world!");

    match &cli.subcommand {
        Some(Subcommand::BuildSpec(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            std::println!("BuildSpec!");
            runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
        },
        Some(Subcommand::CheckBlock(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            std::println!("CheckBlock!");
            runner.async_run(|config| {
                let PartialComponents { client, task_manager, import_queue, .. }
                    = new_partial(&config)?;
                Ok((cmd.run(client, import_queue), task_manager))
            })
        },
        Some(Subcommand::ExportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            std::println!("ExportBlocks!");
            runner.async_run(|config| {
                let PartialComponents { client, task_manager, ..}
                    = new_partial(&config)?;
                Ok((cmd.run(client, config.database), task_manager))
            })
        },
        Some(Subcommand::ExportState(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            std::println!("ExportState!");
            runner.async_run(|config| {
                let PartialComponents { client, task_manager, ..}
                    = new_partial(&config)?;
                Ok((cmd.run(client, config.chain_spec), task_manager))
            })
        },
        Some(Subcommand::ImportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            std::println!("ImportBlocks!");
            runner.async_run(|config| {
                let PartialComponents { client, task_manager, import_queue, ..}
                    = new_partial(&config)?;
                Ok((cmd.run(client, import_queue), task_manager))
            })
        },
        Some(Subcommand::PurgeChain(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            std::println!("PurgeChain!");
            runner.sync_run(|config| cmd.run(config.database))
        },
        Some(Subcommand::Revert(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            std::println!("Revert!");
            runner.async_run(|config| {
                let PartialComponents { client, task_manager, backend, ..}
                    = new_partial(&config)?;
                Ok((cmd.run(client, backend), task_manager))
            })
        },
        Some(Subcommand::Benchmark(subcommand)) => {
            if cfg!(feature = "runtime-benchmarks") {
                let runner = cli.create_runner(subcommand)?;
                std::println!("Benchmark!");

                runner.sync_run(|config| subcommand.run::<Block, Executor>(config))
            } else {
                println!("Benchmarking wasn't enabled when building the node. \
                You can enable it with `--features runtime-benchmarks`.");
                Ok(())
            }
        },
        None => {
            let runner = cli.create_runner(&cli.run)?;
            std::println!("create_runner!");
            runner.run_node_until_exit(|config| async move {
                std::println!("{:?}", config);
                match config.role {
                    Role::Light => new_light(config),
                    _ => new_full(config),
                }.map_err(sc_cli::Error::Service)
            })
        }
    }
}