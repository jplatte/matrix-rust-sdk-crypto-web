//! Encryption types & siblings.

use std::time::Duration;

use matrix_sdk_common::deserialized_responses::ShieldState as RustShieldState;
use wasm_bindgen::prelude::*;

use crate::events;

/// Settings for an encrypted room.
///
/// This determines the algorithm and rotation periods of a group
/// session.
#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct EncryptionSettings {
    /// The encryption algorithm that should be used in the room.
    pub algorithm: EncryptionAlgorithm,

    /// How long the session should be used before changing it,
    /// expressed in microseconds.
    #[wasm_bindgen(js_name = "rotationPeriod")]
    pub rotation_period: u64,

    /// How many messages should be sent before changing the session.
    #[wasm_bindgen(js_name = "rotationPeriodMessages")]
    pub rotation_period_messages: u64,

    /// The history visibility of the room when the session was
    /// created.
    #[wasm_bindgen(js_name = "historyVisibility")]
    pub history_visibility: events::HistoryVisibility,

    /// Should untrusted devices receive the room key, or should they be
    /// excluded from the conversation.
    #[wasm_bindgen(js_name = "sharingStrategy")]
    pub sharing_strategy: CollectStrategy,
}

impl Default for EncryptionSettings {
    fn default() -> Self {
        let default = matrix_sdk_crypto::olm::EncryptionSettings::default();

        Self {
            algorithm: default.algorithm.into(),
            rotation_period: default.rotation_period.as_micros().try_into().unwrap(),
            rotation_period_messages: default.rotation_period_msgs,
            history_visibility: default.history_visibility.into(),
            sharing_strategy: default.sharing_strategy.into(),
        }
    }
}

#[wasm_bindgen]
impl EncryptionSettings {
    /// Create a new `EncryptionSettings` with default values.
    #[wasm_bindgen(constructor)]
    pub fn new() -> EncryptionSettings {
        Self::default()
    }
}

impl From<&EncryptionSettings> for matrix_sdk_crypto::olm::EncryptionSettings {
    fn from(value: &EncryptionSettings) -> Self {
        let algorithm = value.algorithm.clone().into();

        Self {
            algorithm,
            rotation_period: Duration::from_micros(value.rotation_period),
            rotation_period_msgs: value.rotation_period_messages,
            history_visibility: value.history_visibility.clone().into(),
            sharing_strategy: value.sharing_strategy.clone().into(),
        }
    }
}

/// An encryption algorithm to be used to encrypt messages sent to a
/// room.
#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncryptionAlgorithm {
    /// Olm version 1 using Curve25519, AES-256, and SHA-256.
    OlmV1Curve25519AesSha2,

    /// Megolm version 1 using AES-256 and SHA-256.
    MegolmV1AesSha2,

    /// Unsupported algorithm.
    ///
    /// Applications should ignore this value if it is received, and should
    /// never set it.
    Unknown,
}

impl From<EncryptionAlgorithm> for matrix_sdk_crypto::types::EventEncryptionAlgorithm {
    fn from(value: EncryptionAlgorithm) -> Self {
        use EncryptionAlgorithm::*;

        match value {
            OlmV1Curve25519AesSha2 => Self::OlmV1Curve25519AesSha2,
            MegolmV1AesSha2 => Self::MegolmV1AesSha2,
            _ => unreachable!("Unknown variant"),
        }
    }
}

impl From<matrix_sdk_crypto::types::EventEncryptionAlgorithm> for EncryptionAlgorithm {
    fn from(value: matrix_sdk_crypto::types::EventEncryptionAlgorithm) -> Self {
        use matrix_sdk_crypto::types::EventEncryptionAlgorithm::*;

        match value {
            OlmV1Curve25519AesSha2 => Self::OlmV1Curve25519AesSha2,
            MegolmV1AesSha2 => Self::MegolmV1AesSha2,
            _ => Self::Unknown,
        }
    }
}

/// Strategy to collect the devices that should receive room keys for the
/// current discussion.
#[wasm_bindgen()]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollectStrategy {
    /// Device based sharing strategy, excluding devices that are not trusted.
    /// A device is trusted if any of the following is true:
    ///     - It was manually marked as trusted.
    ///     - It was marked as verified via interactive verification.
    ///     - It is signed by its owner identity, and this identity has been
    ///       trusted via interactive verification.
    ///     - It is the current own device of the user.
    DeviceBasedStrategyOnlyTrustedDevices,
    /// Device based sharing strategy, including all devices.
    DeviceBasedStrategyAllDevices,
    /// Share based on identity. Only distribute to devices signed by their
    /// owner. If a user has no published identity he will not receive
    /// any room keys.
    IdentityBasedStrategy,
}

impl From<CollectStrategy> for matrix_sdk_crypto::CollectStrategy {
    fn from(value: CollectStrategy) -> Self {
        match value {
            CollectStrategy::DeviceBasedStrategyOnlyTrustedDevices => {
                Self::DeviceBasedStrategy { only_allow_trusted_devices: true }
            }
            CollectStrategy::DeviceBasedStrategyAllDevices => {
                Self::DeviceBasedStrategy { only_allow_trusted_devices: false }
            }
            CollectStrategy::IdentityBasedStrategy => Self::IdentityBasedStrategy,
        }
    }
}

impl From<matrix_sdk_crypto::CollectStrategy> for CollectStrategy {
    fn from(value: matrix_sdk_crypto::CollectStrategy) -> Self {
        match value {
            matrix_sdk_crypto::CollectStrategy::DeviceBasedStrategy {
                only_allow_trusted_devices: true,
            } => Self::DeviceBasedStrategyOnlyTrustedDevices,
            matrix_sdk_crypto::CollectStrategy::DeviceBasedStrategy {
                only_allow_trusted_devices: false,
            } => Self::DeviceBasedStrategyAllDevices,
            matrix_sdk_crypto::CollectStrategy::IdentityBasedStrategy => {
                Self::IdentityBasedStrategy
            }
        }
    }
}

/// Take a look at [`matrix_sdk_common::deserialized_responses::ShieldState`]
/// for more info.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum ShieldColor {
    /// Important warning
    Red,
    /// Low warning
    Grey,
    /// No warning
    None,
}

/// Take a look at [`matrix_sdk_common::deserialized_responses::ShieldState`]
/// for more info.
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct ShieldState {
    /// The shield color
    pub color: ShieldColor,
    message: Option<String>,
}

#[wasm_bindgen]
impl ShieldState {
    /// Error message that can be displayed as a tooltip
    #[wasm_bindgen(getter)]
    pub fn message(&self) -> Option<String> {
        self.message.clone()
    }
}

impl From<RustShieldState> for ShieldState {
    fn from(value: RustShieldState) -> Self {
        match value {
            RustShieldState::Red { message } => {
                Self { color: ShieldColor::Red, message: Some(message.to_owned()) }
            }
            RustShieldState::Grey { message } => {
                Self { color: ShieldColor::Grey, message: Some(message.to_owned()) }
            }
            RustShieldState::None => Self { color: ShieldColor::None, message: None },
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::EncryptionAlgorithm;

    #[wasm_bindgen_test]
    fn test_convert_encryption_algorithm_to_js() {
        assert!(
            EncryptionAlgorithm::from(
                matrix_sdk_crypto::types::EventEncryptionAlgorithm::OlmV1Curve25519AesSha2
            ) == EncryptionAlgorithm::OlmV1Curve25519AesSha2
        );
        assert!(
            EncryptionAlgorithm::from(
                matrix_sdk_crypto::types::EventEncryptionAlgorithm::MegolmV1AesSha2
            ) == EncryptionAlgorithm::MegolmV1AesSha2
        );
        assert!(
            EncryptionAlgorithm::from(matrix_sdk_crypto::types::EventEncryptionAlgorithm::from(
                "unknown"
            )) == EncryptionAlgorithm::Unknown
        );
    }
}
