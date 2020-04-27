#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let resolver = trust_dns_resolver::AsyncResolver::tokio(
        Default::default(),
        trust_dns_resolver::config::ResolverOpts {
            ip_strategy: trust_dns_resolver::config::LookupIpStrategy::Ipv4Only, // oha --ipv4
            ..Default::default()
        },
    )
    .await?;

    let addrs = resolver
        .lookup_ip("internal.host") // put hostname here
        .await?
        .iter()
        .collect::<Vec<_>>();

    dbg!(addrs);

    Ok(())
}
