use anyhow::{Ok, Result};
use capnpc::CompilerCommand;
use tokio::fs::copy;

#[tokio::main]
async fn main() -> Result<()> {
    CompilerCommand::new()
        .src_prefix("schema")
        .file("schema/fe2o3-agent_schema.capnp")
        .output_path("schema")
        .run()?;

    /* Copy .gitignore to .containerignore. This is used later on to ensure the proper
    .gitignore files are considered during building the container */
    copy("../.gitignore", ".containerignore").await?;

    Ok(())
}
