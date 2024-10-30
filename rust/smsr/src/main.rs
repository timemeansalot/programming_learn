use futures::future::join_all;
use log::*;
use reqwest;
use std::io::Write;
use tokio::task;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn get_and_analyze(n: usize) -> Result<(u64, u64)> {
    // request url and get response
    let response: reqwest::Response = reqwest::get(slowwly(1000)).await?;
    info!("Dataset: {}", n);

    // analyze the response, count ones and zeros
    let txt = response.text().await?;
    let res = task::spawn_blocking(move || analyze(&txt)).await?;
    info!("Processed {}", n);
    Ok(res)
}

fn analyze(txt: &str) -> (u64, u64) {
    let txt = txt.as_bytes();
    let ones = txt
        .iter()
        .fold(0u64, |acc, b: &u8| acc + b.count_ones() as u64);
    let zeros = txt
        .iter()
        .fold(0u64, |acc, b: &u8| acc + b.count_zeros() as u64);
    (ones, zeros)
}

/// parse the url after 1 seconds
fn slowwly(delay_ms: u32) -> reqwest::Url {
    let url = format!(
        "http://slowwly.robertomurray.co.uk/delay/{}/url/http://www.google.co.uk",
        delay_ms
    );
    reqwest::Url::parse(&url).unwrap()
}

#[allow(dead_code)]
async fn request(n: usize) -> Result<()> {
    reqwest::get(slowwly(1000)).await?;
    info!("Get response {}", n);
    Ok(())
}

async fn app() -> Result<()> {
    info!("starting");
    // let _resp1 = reqwest::get(slowwly(1000)).await?;
    // info!("Got response 1");
    // let _resp2 = reqwest::get(slowwly(1000)).await?;
    // info!("Got response 2");

    // use spawn enalbe the thread to run in parallel
    // let resp1 = task::spawn(request(1));
    // let resp2 = task::spawn(request(2));
    // let _ = resp1.await?;
    // let _ = resp2.await?;

    let mut futures = vec![];

    for i in 0..10 {
        let fut = task::spawn(get_and_analyze(i));
        futures.push(fut);
    }

    // start all the futures in parallel
    let resutls = join_all(futures).await;
    let mut total_ones = 0;
    let mut total_zeros = 0;
    for result in resutls {
        let ones_res: Result<(u64, u64)> = result?;
        let (ones, zeros) = ones_res?;
        total_ones += ones;
        total_zeros += zeros;
    }
    info!("Total ones: {}, Total zeros: {}", total_ones, total_zeros);
    Ok(())
}

fn main() {
    let start = std::time::Instant::now();
    env_logger::Builder::from_default_env()
        .format(move |buf, rec| {
            let t = start.elapsed().as_secs_f32();
            writeln!(buf, "{:.03} [{}] - {}", t, rec.level(), rec.args())
        })
        .init();
    let rt = tokio::runtime::Runtime::new().unwrap();
    match rt.block_on(app()) {
        Ok(()) => info!("Done!"),
        Err(e) => error!("An error occurred: {}", e),
    }
}
