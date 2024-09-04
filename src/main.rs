use std::time::Instant;
use reqwest::Client;
use trust_dns_resolver::config::*;
use trust_dns_resolver::TokioAsyncResolver;
use std::net::IpAddr;

async fn test_latency(client: &Client, ip: IpAddr) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("http://{}", ip);
    println!("Testing latency to {}", url);

    for i in 1..=5 {
        let start = Instant::now();
        let response = client.get(&url).send().await?;
        let duration = start.elapsed();

        println!("Request {}: Status: {}, Latency: {:?}", i, response.status(), duration);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());
    let response = resolver.lookup_ip("tmi.twitch.tv").await?;
    let ips: Vec<IpAddr> = response.iter().collect();

    println!("Found {} IP addresses for tmi.twitch.tv", ips.len());

    let client = Client::new();

    for ip in ips {
        println!("\nTesting IP: {}", ip);
        if let Err(e) = test_latency(&client, ip).await {
            println!("Error testing {}: {}", ip, e);
        }
    }

    Ok(())
}
