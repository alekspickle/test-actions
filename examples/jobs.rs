use gh_rs::{App, AppOptions, GenericResult};
use structopt::StructOpt;

#[tokio::main]
async fn main() -> GenericResult<()> {
    let opts = AppOptions::from_args();
    let gh = opts.clone().into_octo().await?;

    let app = App::new(gh, opts);

    // let jobs = app.get_jobs();

    Ok(())
}
