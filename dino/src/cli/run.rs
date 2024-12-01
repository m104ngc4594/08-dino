use crate::{build_project, CmdExecutor, JsWorker, Req};
use clap::Parser;
use std::{collections::HashMap, fs};

#[derive(Debug, Parser)]
pub struct RunOpts {}

impl CmdExecutor for RunOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let filename = build_project(".")?;
        let content = fs::read_to_string(filename)?;
        let worker = JsWorker::try_new(&content)?;
        // TODO: normally this should run axum and let it load the worker
        let req = Req::builder()
            .method("GET")
            .url("https://example.com")
            .headers(HashMap::new())
            .build();
        let ret = worker.run("hello", req)?;
        println!("Response: {:?}", ret);

        Ok(())
    }
}
