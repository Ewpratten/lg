use rocket::{http::ContentType, State};
use rocket_client_addr::ClientAddr;
use tera::{Context, Tera};

use crate::{
    configs::{GlobalConfig, LocalConfig},
    public_ip::PublicIpPair,
};

use super::WebAppAssets;

#[get("/")]
pub fn index(
    local_config: &State<LocalConfig>,
    global_config: &State<GlobalConfig>,
    client_addr: &ClientAddr,
    public_addrs: &State<PublicIpPair>,
) -> (ContentType, String) {
    // Load the HTML data either from disk or memory depending on build type
    let data = WebAppAssets::get("index.html").unwrap().data;
    let data = String::from_utf8((&data).to_vec()).unwrap();

    // Handle the client IP addresses
    let client_ipv4 = client_addr.get_ipv4_string();
    let mut client_ipv6 = Some(client_addr.get_ipv6_string());

    // Handle the v6 being a fake address
    if let Some(v4) = &client_ipv4 {
        if client_ipv6.take().unwrap() == format!("::ffff:{}", v4) {
            client_ipv6 = None;
        }
    }

    // Set up a render context
    let mut context = Context::new();
    context.insert("local_config", &local_config.inner());
    context.insert("global_config", &global_config.inner());
    context.insert("client_ipv4", &client_ipv4);
    context.insert("client_ipv6", &client_ipv6);
    context.insert("public_ipv4", &public_addrs.inner().ipv4);
    context.insert("public_ipv6", &public_addrs.inner().ipv6);

    // Render the loaded HTML via tera
    let rendered = Tera::one_off(&data, &context, false).unwrap();

    // Hand the finished data back to rocket
    (ContentType::HTML, rendered)
}
