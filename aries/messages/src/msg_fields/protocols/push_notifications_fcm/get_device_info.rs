use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use crate::decorators::thread::Thread;
use crate::msg_parts::MsgParts;

pub type GetDeviceInfo = MsgParts<GetDeviceInfoContent, GetDeviceInfoDecorators>;

#[derive(Clone, Debug, Deserialize, Serialize, Default, PartialEq, TypedBuilder)]
pub struct GetDeviceInfoContent {}

#[derive(Clone, Debug, Deserialize, Serialize, Default, PartialEq, TypedBuilder)]
pub struct GetDeviceInfoDecorators {
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
    fn test_minimal_get_device_info() {
        let content = GetDeviceInfoContent::default();

        let decorators = GetDeviceInfoDecorators::default();

        let expected = json!({});

        test_utils::test_msg(
            content,
            decorators,
            PushNotificationsFCMTypeV1_0::GetDeviceInfo,
            expected,
        );
    }
}
