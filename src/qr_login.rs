//! Types for QR code login

use matrix_sdk_crypto::types::qr_login;
use url::Url;
use wasm_bindgen::prelude::*;

use crate::vodozemac::Curve25519PublicKey;

/// The mode of the QR code login.
///
/// The QR code login mechanism supports both, the new device, as well as the
/// existing device to display the QR code.
///
/// The different modes have an explicit one-byte identifier which gets added to
/// the QR code data.
#[wasm_bindgen]
#[derive(Debug)]
pub enum QrCodeMode {
    /// The new device is displaying the QR code.
    Login,
    /// The existing device is displaying the QR code.
    Reciprocate,
}

impl From<qr_login::QrCodeMode> for QrCodeMode {
    fn from(value: qr_login::QrCodeMode) -> Self {
        match value {
            qr_login::QrCodeMode::Login => Self::Login,
            qr_login::QrCodeMode::Reciprocate => Self::Reciprocate,
        }
    }
}

/// Data for the QR code login mechanism.
///
/// The {@link QrCodeData} can be serialized and encoded as a QR code or it can
/// be decoded from a QR code.
#[wasm_bindgen]
#[derive(Debug)]
pub struct QrCodeData {
    inner: qr_login::QrCodeData,
}

#[wasm_bindgen]
impl QrCodeData {
    /// Create new {@link QrCodeData} from a given public key, a rendezvous URL
    /// and, optionally, a server name for the homeserver.
    ///
    /// If a server name is given, then the {@link QrCodeData} mode will be
    /// {@link QrCodeMode.Reciprocate}, i.e. the QR code will contain data for
    /// the existing device to display the QR code.
    ///
    /// If no server name is given, the {@link QrCodeData} mode will be
    /// {@link QrCodeMode.Login}, i.e. the QR code will contain data for the
    /// new device to display the QR code.
    #[wasm_bindgen(constructor)]
    pub fn new(
        public_key: Curve25519PublicKey,
        rendezvous_url: &str,
        server_name: Option<String>,
    ) -> Result<QrCodeData, JsError> {
        let public_key = public_key.inner;
        let rendezvous_url = Url::parse(rendezvous_url)?;

        let mode_data = if let Some(server_name) = server_name {
            qr_login::QrCodeModeData::Reciprocate { server_name }
        } else {
            qr_login::QrCodeModeData::Login
        };

        let inner = qr_login::QrCodeData { public_key, rendezvous_url, mode_data };

        Ok(QrCodeData { inner })
    }

    /// Attempt to decode a slice of bytes into a {@link QrCodeData} object.
    ///
    /// The slice of bytes would generally be returned by a QR code decoder.
    pub fn from_bytes(bytes: &[u8]) -> Result<QrCodeData, JsError> {
        Ok(Self { inner: qr_login::QrCodeData::from_bytes(bytes)? })
    }

    /// Encode the {@link QrCodeData} into a list of bytes.
    ///
    /// The list of bytes can be used by a QR code generator to create an image
    /// containing a QR code.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.inner.to_bytes()
    }

    /// Attempt to decode a base64 encoded string into a {@link QrCodeData}
    /// object.
    pub fn from_base64(data: &str) -> Result<QrCodeData, JsError> {
        Ok(Self { inner: qr_login::QrCodeData::from_base64(data)? })
    }

    /// Encode the {@link QrCodeData} into a string using base64.
    ///
    /// This format can be used for debugging purposes and the
    /// [`QrcodeData::from_base64()`] method can be used to parse the string
    /// again.
    pub fn to_base64(&self) -> String {
        self.inner.to_base64()
    }

    /// Get the Curve25519 public key embedded in the {@link QrCodeData}.
    ///
    /// This Curve25519 public key should be used to establish an
    /// [ECIES](https://en.wikipedia.org/wiki/Integrated_Encryption_Scheme)
    /// (Elliptic Curve Integrated Encryption Scheme) channel with the other
    /// device.
    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> Curve25519PublicKey {
        self.inner.public_key.into()
    }

    /// Get the URL of the rendezvous server which will be used to exchange
    /// messages between the two devices.
    #[wasm_bindgen(getter)]
    pub fn rendezvous_url(&self) -> String {
        self.inner.rendezvous_url.as_str().to_owned()
    }

    /// Get the server name of the homeserver which the new device will be
    /// logged in to.
    ///
    /// This will be only available if the existing device has generated the QR
    /// code and the new device is the one scanning the QR code.
    #[wasm_bindgen(getter)]
    pub fn server_name(&self) -> Option<String> {
        if let qr_login::QrCodeModeData::Reciprocate { server_name } = &self.inner.mode_data {
            Some(server_name.to_owned())
        } else {
            None
        }
    }

    /// Get the mode of this {@link QrCodeData} instance.
    #[wasm_bindgen(getter)]
    pub fn mode(&self) -> QrCodeMode {
        self.inner.mode().into()
    }
}
