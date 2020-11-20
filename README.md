# lotus-api

JSON API application for Lotus.


# Usage
```
use lotus_api::api::ChainApi;
use tokio::runtime::Runtime;

let mut rt = Runtime::new().unwrap();
let http = Http::new("http://47.52.21.141:1234/rpc/v0");
let ret:TipSet = rt.block_on(http.chain_head()).unwrap();
```
