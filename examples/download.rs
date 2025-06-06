#[cfg(not(feature = "ureq"))]
#[cfg(not(feature = "tokio"))]
fn main() {}

#[cfg(feature = "ureq")]
#[cfg(not(feature = "tokio"))]
fn main() {
    let api = hf_hub::api::sync::Api::new().unwrap();

    let _filename = api
        // .model("meta-llama/Llama-2-7b-hf".to_string())
        // .get("model-00001-of-00002.safetensors")
        .model("BAAI/bge-small-zh-v1.5".to_string())
        .get("pytorch_model.bin")
        .unwrap();
}

#[cfg(feature = "tokio")]
#[tokio::main]
async fn main() {
    let api = hf_hub::api::tokio::Api::new().unwrap();

    let _filename = api
        // .model("meta-llama/Llama-2-7b-hf".to_string())
        // .get("model-00001-of-00002.safetensors")
        .model("BAAI/bge-small-zh-v1.5".to_string())
        .get("pytorch_model.bin")
        .await
        .unwrap();
}
