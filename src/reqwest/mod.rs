//! reqwest 工具

pub mod ip;

use anyhow::Result;
use reqwest::{
    header::{ACCEPT, CONNECTION, USER_AGENT},
    Client, RequestBuilder,
};
use std::sync::LazyLock;

static CLIENT: LazyLock<Client> = LazyLock::new(|| {
    let client = Client::new();
    CLIENT.set(client).unwrap();
});

/// 配置 Request
fn config_request(request: RequestBuilder) -> RequestBuilder {
    request
        .header(ACCEPT, "*/*")
        .header(CONNECTION, "Keep-Alive")
        .header(
            USER_AGENT,
            "Mozilla/4.0 (compatible; MSIE 6.0; Windows NT 5.1;SV1)",
        )
}

/// 发送 GET 请求
pub async fn get(url: &str) -> Result<String> {
    let client = CLIENT.get().expect("Client is not initialized");
    let request = config_request(client.get(url));
    Ok(request.send().await?.text().await?)
}
