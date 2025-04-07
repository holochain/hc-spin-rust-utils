#![deny(clippy::all)]

use holo_hash::AgentPubKey;
use holochain_zome_types::prelude::Signature;
use lair_keystore_api::dependencies::sodoken;
use lair_keystore_api::{dependencies::url::Url, ipc_keystore::ipc_keystore_connect, LairClient};
use napi::Result;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

struct ZomeCallSigner {
  lair_client: LairClient,
}

impl ZomeCallSigner {
  /// Connect to lair keystore
  pub async fn new(connection_url: String, passphrase: String) -> Self {
    let connection_url_parsed = Url::parse(connection_url.deref()).unwrap();
    let passphrase_locked_read = Arc::new(Mutex::new(sodoken::LockedArray::from(
      passphrase.as_bytes().to_vec(),
    )));

    let lair_client = ipc_keystore_connect(connection_url_parsed, passphrase_locked_read)
      .await
      .unwrap();

    Self { lair_client }
  }

  /// Sign a zome call
  pub async fn sign_zome_call(&self, payload: Vec<u8>, pub_key: Vec<u8>) -> Result<Vec<u8>> {
    let pub_key = AgentPubKey::from_raw_39(pub_key);
    let mut pub_key_2 = [0; 32];
    pub_key_2.copy_from_slice(pub_key.get_raw_32());

    let sig = self
      .lair_client
      .sign_by_pub_key(pub_key_2.into(), None, payload.into())
      .await
      .unwrap();

    let signature = Signature(*sig.0);

    Ok(signature.0.to_vec())
  }
}

#[napi(js_name = "ZomeCallSigner")]
pub struct JsZomeCallSigner {
  zome_call_signer: Option<ZomeCallSigner>,
}

#[napi]
impl JsZomeCallSigner {
  #[napi(constructor)]
  pub fn new() -> Self {
    Self {
      zome_call_signer: None,
    }
  }

  #[napi]
  pub async fn connect(connection_url: String, passphrase: String) -> Self {
    let zome_call_signer = ZomeCallSigner::new(connection_url, passphrase).await;

    JsZomeCallSigner {
      zome_call_signer: Some(zome_call_signer),
    }
  }

  #[napi]
  pub async fn sign_zome_call(&self, payload: Vec<u8>, pub_key: Vec<u8>) -> Result<Vec<u8>> {
    self
      .zome_call_signer
      .as_ref()
      .unwrap()
      .sign_zome_call(payload, pub_key)
      .await
  }
}
