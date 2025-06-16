use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::decorators::thread::Thread;
use crate::msg_parts::MsgParts;

pub type DeviceInfo = MsgParts<DeviceInfoContent, DeviceInfoDecorators>;

#[derive(Clone, Debug, Deserialize, Serialize, Default, PartialEq, TypedBuilder)]
pub struct DeviceInfoContent {
    #[builder(default)]
    pub device_token: String,
    #[builder(default)]
    pub device_platform: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default, PartialEq, TypedBuilder)]
pub struct DeviceInfoDecorators {
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
        decorators::{thread::tests::make_extended_thread,},
        misc::test_utils,
    };
    use crate::msg_types::protocols::push_notifications_fcm::PushNotificationsFCMTypeV1_0;

    #[test]
    fn test_minimal_device_info() {
        let content = DeviceInfoContent::default();

        let decorators = DeviceInfoDecorators::builder()
            .thread(make_extended_thread())
            .build();

        let expected = json!({
            "device_token": content.device_token,
            "device_platform": content.device_platform,
            "~thread": decorators.thread
        });

        test_utils::test_msg(
            content,
            decorators,
            PushNotificationsFCMTypeV1_0::DeviceInfo,
            expected,
        );
    }

    #[test]
    fn test_extended_device_info() {
        let content = DeviceInfoContent::builder()
            .device_token("test_token".to_owned())
            .device_platform("test_platform".to_owned())
            .build();

        let decorators = DeviceInfoDecorators::builder()
            .thread(make_extended_thread())
            .build();

        let expected = json!({
            "device_token": content.device_token,
            "device_platform": content.device_platform,
            "~thread": decorators.thread,
        });

        test_utils::test_msg(
            content,
            decorators,
            PushNotificationsFCMTypeV1_0::DeviceInfo,
            expected,
        );
    }
}
