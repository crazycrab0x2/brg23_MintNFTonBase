use http::{Request, Response, StatusCode};
use ic_cdk::api::management_canister::http_request::{
    CanisterHttpRequestArgument, HttpHeader, HttpMethod, HttpResponse as HttpResponseCdk,
    TransformArgs as TransformArgsCdk, TransformContext as TransformContextCdk,
};
use ic_cdk::api::management_canister::http_request::{HttpResponse, TransformArgs};
use ic_web3::{
    contract::{Contract, Error, Options},
    ethabi::ethereum_types::U256,
    futures::future::OrElse,
    ic::{get_eth_addr, KeyInfo},
    transports::ICHttp,
    types::{Address, TransactionParameters},
    Web3,
};
use junobuild_utils::decode_doc_data;
use serde::Serialize;
use serde_json::json;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::str::FromStr;
mod types;

type ImageStore = BTreeMap<String, Vec<u8>>;
type ReceiptStore = BTreeMap<String, String>;

thread_local! {
    static IMAGE_STORE: RefCell<ImageStore> = RefCell::default();
    static RECEIPT_STORE: RefCell<ReceiptStore> = RefCell::default();
}

const NFT_ABI: &[u8] = include_bytes!("./NFTABI.json");
const RPC_ENDPOINT: &str = "https://mainnet.base.org";
const CONTRACT_ADDRESS: &str = "0xaf431cb24485fa30d0b480e2c147ae7a1d84c479";
const KEY_NAME:&str = "test_key_1";
// const KEY_NAME:&str = "key_1";

#[derive(Clone, Serialize)]
pub struct Contact {
    pub name: String,
    pub address: String,
}

#[ic_cdk::query]
pub fn whoami() -> String {
    ic_cdk::api::caller().to_string()
}

#[ic_cdk::update]
async fn upload_image(image_key: String, image_data: Vec<u8>) -> String {
    IMAGE_STORE.with(|image_store| {
        if image_store.borrow().get(&image_key).is_some() {
            "Image name already exist".to_string()
        } else {
            image_store
                .borrow_mut()
                .insert(image_key.clone(), image_data);
            image_key
        }
    })
}

#[ic_cdk::update]
async fn upload_data(receipt_key: String, receipt_data: String) -> String {
    RECEIPT_STORE.with(|receipt_store| {
        if receipt_store.borrow().get(&receipt_key).is_some() {
            "data key already exist".to_string()
        } else {
            receipt_store
                .borrow_mut()
                .insert(receipt_key.clone(), receipt_data);
            receipt_key
        }
    })
}

#[ic_cdk::update]
pub async fn get_evm_address() -> String {
    let principal = ic_cdk::api::caller().to_string();
    let derivation: Vec<Vec<u8>> = principal
        .split('-')
        .map(|word| word.as_bytes().to_vec())
        .collect();
    let res = get_eth_addr(None, Some(derivation), KEY_NAME.to_string()).await;
    format!("0x{}", hex::encode(res.unwrap()))
}

#[ic_cdk::update]
pub async fn get_eth_balance(address: String) -> (u64, String) {
    let w3 = match ICHttp::new(RPC_ENDPOINT, None) {
        Ok(v) => Web3::new(v),
        Err(e) => return (0, e.to_string()),
    };
    let evm_address = &address[2..];
    let balance = w3
        .eth()
        .balance(Address::from_str(evm_address).unwrap(), None)
        .await;
    match balance {
        Ok(bal) => (bal.as_u64(), "".to_string()),
        Err(err) => (0, err.to_string()),
    }
}

#[ic_cdk::update]
pub async fn get_contract() -> String {
    let w3 = match ICHttp::new(RPC_ENDPOINT, None) {
        Ok(v) => Web3::new(v),
        Err(e) => {
            return e.to_string();
        }
    };
    let contract_address = Address::from_str(&CONTRACT_ADDRESS[2..]).unwrap();
    let contract_res = Contract::from_json(w3.eth(), contract_address, NFT_ABI);
    match contract_res {
        Ok(contract) => {
            // let txhash_res = contract.signed_call("transfer", (to_addr, amount,), options, hex::encode(from_addr.clone()), key_info, chain_id).await;
            "success".to_string()
        }
        Err(error) => error.to_string(),
    }
}

#[ic_cdk::query]
fn http_request(req: types::HttpRequest) -> types::HttpResponse {
    if req.url.contains("image") {
        let res: Vec<&str> = req.url.split("image").collect();
        let image_key = res[1][1..].to_string();
        IMAGE_STORE.with(|image_store| {
            if image_store.borrow().get(&image_key).is_some() {
                let image_data = image_store.borrow().get(&image_key).unwrap().clone();
                let mut http_header = req.headers;
                http_header.push(("CONTENT_TYPE".to_string(), "image/png".to_string()));
                let response = types::HttpResponse {
                    headers: http_header,
                    status_code: 200,
                    body: image_data.into(),
                };
                response
            } else {
                let response = types::HttpResponse {
                    headers: req.headers,
                    status_code: 404,
                    body: b"Image not found!".to_vec().into(),
                };
                response
            }
        })
    }
    else if req.url.contains("receipt") {
        let res: Vec<&str> = req.url.split("receipt").collect();
        let receipt_key = res[1][1..].to_string();
        RECEIPT_STORE.with(|receipt_store| {
            if receipt_store.borrow().get(&receipt_key).is_some() {
                let receipt_data = receipt_store.borrow().get(&receipt_key).unwrap().clone();
                let mut http_header = req.headers;
                http_header.push(("CONTENT_TYPE".to_string(), "text/json".to_string()));
                let response = types::HttpResponse {
                    headers: http_header,
                    status_code: 200,
                    body: receipt_data.as_bytes().to_vec().into(),
                };
                response
            } else {
                let response = types::HttpResponse {
                    headers: req.headers,
                    status_code: 404,
                    body: b"Image not found!".to_vec().into(),
                };
                response
            }
        })
    }
    else {
        let response = types::HttpResponse {
            headers: req.headers,
            status_code: 404,
            body: req.url.as_bytes().to_vec().into(),
        };
        response
    }
}

#[ic_cdk::query]
pub fn transform(response: TransformArgs) -> HttpResponse {
    let mut t = response.response;
    t.headers = vec![];
    t
}
