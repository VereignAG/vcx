use derive_more::From;
use messages_macros::MessageType;
use strum_macros::{AsRefStr, EnumString};
use transitive::Transitive;

use super::Protocol;
use crate::msg_types::{MsgKindType};

#[derive(Copy, Clone, Debug, From, PartialEq, MessageType)]
#[msg_type(protocol = "push-notifications-fcm")]
pub enum PushNotificationsFCMType {
    V1(PushNotificationsFCMTypeV1),
}

#[derive(Copy, Clone, Debug, From, PartialEq, Transitive, MessageType)]
#[transitive(into(PushNotificationsFCMType, Protocol))]
#[msg_type(major = 1)]
pub enum PushNotificationsFCMTypeV1 {
    #[msg_type(minor = 0, roles = "")]
    V1_0(MsgKindType<PushNotificationsFCMTypeV1_0>),
}

#[derive(Copy, Clone, Debug, AsRefStr, EnumString, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum PushNotificationsFCMTypeV1_0 {
    DeviceInfo,
    SetDeviceInfo,
    GetDeviceInfo,
    ReportProblem,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::misc::test_utils;

    #[test]
    fn test_protocol_push_notifications_fcm() {
        test_utils::test_serde(
            Protocol::from(PushNotificationsFCMTypeV1::new_v1_0()),
            json!("https://didcomm.org/push-notifications-fcm/1.0"),
        )
    }

    #[test]
    fn test_version_resolution_push_notifications_fcm() {
        test_utils::test_msg_type_resolution(
            "https://didcomm.org/push-notifications-fcm/1.188",
            PushNotificationsFCMTypeV1::new_v1_0(),
        )
    }

    #[test]
    #[should_panic]
    fn test_unsupported_version_push_notifications_fcm() {
        test_utils::test_serde(
            Protocol::from(PushNotificationsFCMTypeV1::new_v1_0()),
            json!("https://didcomm.org/push-notifications-fcm/2.0"),
        )
    }

    #[test]
    fn test_msg_type_device_info() {
        test_utils::test_msg_type(
            "https://didcomm.org/push-notifications-fcm/1.0",
            "device_info",
            PushNotificationsFCMTypeV1::new_v1_0(),
        )
    }

    #[test]
    fn test_msg_type_set_device_info() {
        test_utils::test_msg_type(
            "https://didcomm.org/push-notifications-fcm/1.0",
            "set_device_info",
            PushNotificationsFCMTypeV1::new_v1_0(),
        )
    }

    #[test]
    fn test_msg_type_get_device_info() {
        test_utils::test_msg_type(
            "https://didcomm.org/push-notifications-fcm/1.0",
            "get_device_info",
            PushNotificationsFCMTypeV1::new_v1_0(),
        )
    }
}
