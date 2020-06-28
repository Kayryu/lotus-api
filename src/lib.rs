pub mod transports;
pub mod error;
pub mod api;
pub mod types;
mod helper;

pub use transports::Http;

#[cfg(test)]
mod tests {
    use super::*;
    use super::api::common::CommpnApi;
    use tokio::runtime::Runtime;

    #[test]
    fn test() {
        let mut rt = Runtime::new().unwrap();
        let http = Http::new("http://127.0.0.1:1234/rpc/v0");
        let ret = rt.block_on(http.net_addrs_listen()).unwrap();

        println!("result: {:?}", ret);
    }
}