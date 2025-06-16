use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use crate::decorators::thread::Thread;
use crate::msg_parts::MsgParts;

pub type SetDeviceInfo = MsgParts<SetDeviceInfoContent, SetDeviceInfoDecorators>;

#[derive(Clone, Debug, Deserialize, Serialize, Default, PartialEq, TypedBuilder)]
pub struct SetDeviceInfoContent {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_token: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_platform: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default, PartialEq, TypedBuilder)]
pub struct SetDeviceInfoDecorators {
    #[builder(default, setter(strip_option))]
    #[serde(rename = "~thread")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread: Option<Thread>,
}

#[cfg(test)]
#[allow(clippy::field_reassign_with_default)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::{
        misc::test_utils,
    };
    use crate::msg_types::protocols::push_notifications_fcm::PushNotificationsFCMTypeV1_0;

    #[test]
    fn test_minimal_set_device_info() {
        let content = SetDeviceInfoContent::default();

        let decorators = SetDeviceInfoDecorators::default();

        let expected = json!({});

        test_utils::test_msg(
            content,
            decorators,
            PushNotificationsFCMTypeV1_0::SetDeviceInfo,
            expected,
        );
    }

    #[test]
    fn test_extended_set_device_info() {
        let content = SetDeviceInfoContent::builder()
            .device_token(Some("test_token".to_owned()))
            .device_platform(Some("test_platform".to_owned()))
            .build();

        let decorators = SetDeviceInfoDecorators::default();

        let expected = json!({
            "device_token": content.device_token,
            "device_platform": content.device_platform,
        });

        test_utils::test_msg(
            content,
            decorators,
            PushNotificationsFCMTypeV1_0::SetDeviceInfo,
            expected,
        );
    }
}
