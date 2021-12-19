use std::fmt::Debug;
use std::{fmt::Display, io};

pub mod tea;

pub mod ecdh;

pub enum CryptoError {
    Io(io::Error),
    Size(
        //except size
        usize,
        //recive size
        usize,
    ),
    GroupAble(usize),
    HexFormat(hex::FromHexError),
    // todo change to sutiable name
    PublicKeyInvalid,
    Request(reqwest::Error),
}

impl std::error::Error for CryptoError {}

impl Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoError::Io(ierr) => Display::fmt(ierr, f),
            CryptoError::HexFormat(herr) => Display::fmt(herr, f),
            CryptoError::Size(exc, rec) => {
                write!(f, "Except Slice Size: {}, But get: {}", exc, rec)
            }
            CryptoError::GroupAble(size) => {
                write!(f, "Except Slice Size Can be div by 8, But get: {}", size)
            }
            CryptoError::PublicKeyInvalid => write!(f, "public key invalid"),
            CryptoError::Request(err) => write!(f, "Request Error: {}", &err),
        }
    }
}

impl Debug for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(arg0) => f.debug_tuple("Io").field(arg0).finish(),
            Self::Size(arg0, arg1) => f
                .debug_struct("Size")
                .field("expect size", arg0)
                .field("recive size", arg1)
                .finish(),
            Self::GroupAble(arg0) => f.debug_tuple("GroupAble").field(arg0).finish(),
            CryptoError::HexFormat(hex) => f.debug_tuple("Hex Format").field(hex).finish(),
            CryptoError::PublicKeyInvalid => f
                .debug_tuple("PublicKey")
                .field(&"public key invalid")
                .finish(),
            CryptoError::Request(err) => f.debug_tuple("RequestError").field(&err).finish(),
        }
    }
}

impl From<io::Error> for CryptoError {
    fn from(src: io::Error) -> Self {
        CryptoError::Io(src)
    }
}

impl From<hex::FromHexError> for CryptoError {
    fn from(e: hex::FromHexError) -> Self {
        Self::HexFormat(e)
    }
}

impl From<k256::elliptic_curve::Error> for CryptoError {
    fn from(e: k256::elliptic_curve::Error) -> Self {
        CryptoError::PublicKeyInvalid
    }
}

impl From<reqwest::Error> for CryptoError {
    fn from(e: reqwest::Error) -> Self {
        CryptoError::Request(e)
    }
}