use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client, Error, Region};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short)]
    region: Option<String>,

    bucket: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    let Opt { region, bucket } = Opt::from_args();

    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("ap-northeast-1"));

    // println!("{}", region_provider.region().await.unwrap().as_ref());

    let shared_config = aws_config::from_env().region(region_provider).load().await;

    let client = Client::new(&shared_config);

    let resp = client.list_objects_v2().bucket(bucket).send().await?;

    for object in resp.contents().unwrap_or_default() {
        println!("{}", object.key().unwrap_or_default());
    }

    Ok(())
}
