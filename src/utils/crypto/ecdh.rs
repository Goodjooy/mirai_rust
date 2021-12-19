//! **本源码参考Mirai源码完成**
//! https://github.com/mamoe/mirai/blob/dev/mirai-core/src/commonMain/kotlin/utils/crypto/ECDH.kt

use std::{io::Write, slice::SliceIndex};

use crate::binary::data_writer::DataWriter;

use super::tea::{CryptoResult, Tea};
use k256::{
    ecdh::EphemeralSecret,
    elliptic_curve::{ecdh::SharedSecret, PublicKey},
    EncodedPoint, Secp256k1,
};
use reqwest::Url;

const DEFAULT_PUBLIC_KEY: [u8; 65] = [
    0x04, 0xed, 0xb8, 0x90, 0x60, 0x46, 0xf5, 0xbf, 0xbe, 0x9a, 0xbb, 0xc5, 0xa8, 0x8b, 0x37, 0xd7,
    0x0a, 0x60, 0x06, 0xbf, 0xba, 0xbc, 0x1f, 0x0c, 0xd4, 0x9d, 0xfb, 0x33, 0x50, 0x5e, 0x63, 0xef,
    0xc5, 0xd7, 0x8e, 0xe4, 0xe0, 0xa4, 0x59, 0x5, 0x033, 0xb9, 0x3d, 0x02, 0x09, 0x6d, 0xcd, 0x31,
    0x90, 0x27, 0x92, 0x11, 0xf7, 0xb4, 0xf6, 0x78, 0x50, 0x79, 0xe1, 0x90, 0x04, 0xaa, 0x0e, 0x03,
    0xbc,
];

const DEFAULT_SHARE_KEY: [u8; 16] = [
    0xc1, 0x29, 0xed, 0xba, 0x73, 0x6f, 0x49, 0x09, 0xec, 0xc4, 0xab, 0x8e, 0x01, 0x0f, 0x46, 0xa3,
];

/// ECDH 加密
/// 参考： [ECDH in Rust using secp256k1](https://asecuritysite.com/rust/rust_ecdh2)
/// 参考： miraiGo 源码：https://github.com/Mrs4s/MiraiGo/blob/master/internal/crypto/crypto.go
pub struct Ecdh {
    secret: EphemeralSecret,
    server_pub: PublicKey<Secp256k1>,
    share_key: SharedSecret<Secp256k1>,
    pub_ver: u16,
}

pub struct EcdhSession {
    t133: Vec<u8>,
}
#[derive(serde::Deserialize)]
struct ServerKeyPayload {
    #[serde(rename(deserialize = "PubKeyMeta"))]
    meta: ServerKeyCore,
}
#[derive(serde::Deserialize)]
struct ServerKeyCore {
    #[serde(rename(deserialize = "KeyVer"))]
    version: u16,
    #[serde(rename(deserialize = "PubKey"))]
    key: String,
}

impl Ecdh {
    pub fn new<'s>(ver: Option<u16>, pubkey: Option<&[u8]>) -> CryptoResult<Self> {
        // server pub key
        let pub_key = PublicKey::from_sec1_bytes(pubkey.unwrap_or(&DEFAULT_PUBLIC_KEY))?;
        // local secret
        let es = EphemeralSecret::random(rand_core::OsRng);
        let share_key = es.diffie_hellman(&pub_key);
        Ok(Self {
            secret: es,
            server_pub: pub_key,
            pub_ver: ver.unwrap_or(1),
            share_key,
        })
    }

    pub async fn load_pub_key_from_server(uid: u64) -> CryptoResult<Self> {
        let mut url = Url::parse("https://keyrotate.qq.com/rotate_key").expect("Wrong URL");
        url.query_pairs_mut()
            .clear()
            .append_pair("cipher_suite_ver", "305")
            .append_pair("uin", &uid.to_string());

        let res = reqwest::get(url).await?.json::<ServerKeyPayload>().await?;

        Self::new(Some(res.meta.version), Some(&hex::decode(&res.meta.key)?))
    }
}

impl Ecdh {
    pub fn encrypt<'a, 'b>(
        &self,
        key: impl AsRef<&'a [u8]>,
        data: impl AsRef<&'b [u8]>,
    ) -> CryptoResult<Vec<u8>> {
        let pubkey = EncodedPoint::from(self.secret.public_key());
        let pubkey_slice = pubkey.as_ref();

        Ok(DataWriter::new_filled(|w| {
            w.write_data(&0x02u8)?;
            w.write_data(&0x01u8)?;
            w.write_data(key.as_ref())?;
            w.write_data(&0x01_31_u16)?;
            w.write_data(&self.pub_ver)?;
            w.write_data(&(pubkey_slice.len() as u16))?;
            w.write_data(&pubkey_slice)?;
            w.encrypted_write(self.share_key.as_bytes(), data.as_ref())?;
            Ok(())
        })?)
    }

    pub fn id(&self) -> u8 {
        0x87
    }
}

impl EcdhSession {
    pub fn new(t133: impl Into<Vec<u8>>) -> Self {
        Self { t133: t133.into() }
    }
}

impl EcdhSession {
    pub fn encrypt<'a, 'b>(
        &self,
        key: impl AsRef<&'a [u8]>,
        data: impl AsRef<&'b [u8]>,
    ) -> CryptoResult<Vec<u8>> {
        Ok(DataWriter::new_filled(|w| {
            let encrypted = Tea::new(key.as_ref())?.encrypt(data.as_ref())?;
            w.write_data(&(self.t133.len() as u16))?;
            w.write_data(&self.t133)?;
            w.write_data(&encrypted)?;

            Ok(())
        })?)
    }

    pub fn id(&self) -> u8 {
        69
    }
}
