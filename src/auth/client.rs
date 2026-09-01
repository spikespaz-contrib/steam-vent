use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use steam_machine_id::MachineID;
use steam_vent_proto_common::protobuf::Enum;
use steam_vent_proto_steam::steammessages_auth_steamclient::EAuthTokenPlatformType;

/// Information about the client to present to steam when authenticating
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(from = "RawClientInfo")]
#[serde(into = "RawClientInfo")]
#[non_exhaustive]
pub struct ClientInfo {
    pub name: String,
    pub machine_id: MachineId,
    pub platform_type: EAuthTokenPlatformType,
    pub os: Os,
}

impl ClientInfo {
    pub fn new(name: String) -> Self {
        ClientInfo {
            name,
            ..Self::default()
        }
    }
}

impl Default for ClientInfo {
    fn default() -> Self {
        Self {
            name: "DESKTOP-VENT".into(),
            machine_id: Default::default(),
            platform_type: EAuthTokenPlatformType::k_EAuthTokenPlatformType_SteamClient,
            os: Default::default(),
        }
    }
}

/// Client OS type
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
pub enum Os {
    Web,
    Linux,
    MacOs,
    #[default]
    Windows,
}

/// The Steam `EOSType` value for this OS.
///
/// This is the wire mapping only. `Os` is persisted by variant name, so this
/// conversion is never part of the serde round-trip.
impl From<Os> for i32 {
    fn from(value: Os) -> Self {
        match value {
            Os::Web => -700,
            Os::Linux => -203,
            Os::MacOs => -102,
            Os::Windows => 0,
        }
    }
}

/// Unique identifier for the machine
#[derive(Debug, Clone)]
pub struct MachineId {
    // newtype to prevent steam-machine-id from being part of the public api
    id: MachineID,
}

impl MachineId {
    /// Generate a machine id based on the provided string.
    ///
    /// The logic for generating the machine id is one way, but repeatable
    pub fn new(identifier: &str) -> Self {
        MachineId {
            id: MachineID::from_account_name(identifier),
        }
    }

    pub(crate) fn encode(&self) -> Vec<u8> {
        self.id.to_message()
    }
}

impl Default for MachineId {
    fn default() -> Self {
        Self {
            id: MachineID::random(),
        }
    }
}

impl From<ClientInfo> for RawClientInfo {
    fn from(value: ClientInfo) -> Self {
        RawClientInfo {
            platform_type: value.platform_type.value(),
            name: value.name,
            os: value.os,
            machine_id_value_bb3: value.machine_id.id.value_bb3,
            machine_id_value_ff2: value.machine_id.id.value_ff2,
            machine_id_value_3b3: value.machine_id.id.value_3b3,
        }
    }
}

impl From<RawClientInfo> for ClientInfo {
    fn from(value: RawClientInfo) -> Self {
        ClientInfo {
            platform_type: EAuthTokenPlatformType::from_i32(value.platform_type)
                .unwrap_or_default(),
            name: value.name,
            os: value.os,
            machine_id: MachineId {
                id: MachineID {
                    value_bb3: value.machine_id_value_bb3,
                    value_ff2: value.machine_id_value_ff2,
                    value_3b3: value.machine_id_value_3b3,
                },
            },
        }
    }
}

#[serde_as]
#[derive(Deserialize, Serialize)]
struct RawClientInfo {
    platform_type: i32,
    name: String,
    os: Os,
    machine_id_value_bb3: [u8; 20],
    machine_id_value_ff2: [u8; 20],
    machine_id_value_3b3: [u8; 20],
}

#[test]
fn test_client_info_os_round_trip() {
    for os in [Os::Web, Os::Linux, Os::MacOs, Os::Windows] {
        let info = ClientInfo {
            os,
            ..ClientInfo::default()
        };

        let json = serde_json::to_string(&info).expect("client info should serialize");

        // Persisting the variant name, not a number, is the point: the
        // `EOSType` values belong to the wire and must not leak into the
        // stored form.
        assert!(
            json.contains(&format!("\"os\":\"{os:?}\"")),
            "expected {os:?} to persist by variant name, got {json}"
        );

        let restored: ClientInfo =
            serde_json::from_str(&json).expect("client info should deserialize");
        assert_eq!(os, restored.os, "{os:?} did not survive the round trip");
    }
}
