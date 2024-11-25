use std::env;

use crate::{build_project, CmdExecutor};
use clap::Parser;

#[derive(Debug, Parser)]
pub struct BuildOpts {}

impl CmdExecutor for BuildOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let cur_dir = env::current_dir()?.display().to_string();
        let filename = build_project(&cur_dir)?;
        eprintln!("Build success: {}", filename);

        Ok(())
    }
}
