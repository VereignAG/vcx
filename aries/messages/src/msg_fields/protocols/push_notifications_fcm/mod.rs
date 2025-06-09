//! Module containing the `fcm push notifications` protocol messages, as defined in the [RFC](<https://github.com/decentralized-identity/aries-rfcs/blob/main/features/0734-push-notifications-fcm/README.md>).

pub mod device_info;
pub mod set_device_info;
pub mod get_device_info;
pub mod problem_report;

use derive_more::From;
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

use self::{
    device_info::{DeviceInfo, DeviceInfoContent, DeviceInfoDecorators},
    get_device_info::{GetDeviceInfo, GetDeviceInfoContent, GetDeviceInfoDecorators},
    set_device_info::{SetDeviceInfo, SetDeviceInfoContent, SetDeviceInfoDecorators},
    problem_report::{ ProblemReport, ProblemReportContent, ProblemReportDecorators },
};

use crate::{
    misc::utils::{into_msg_with_type, transit_to_aries_msg},
    msg_fields::traits::DelayedSerde,
    msg_types::{
        protocols::push_notifications_fcm::{
            PushNotificationsFCMType as PushNotificationsFCMKind,
            PushNotificationsFCMTypeV1, PushNotificationsFCMTypeV1_0,
        },
        MsgWithType,
    },

};

#[derive(Clone, Debug, From, PartialEq)]
pub enum PushNotificationsFCM {
    DeviceInfo(DeviceInfo),
    GetDeviceInfo(GetDeviceInfo),
    SetDeviceInfo(SetDeviceInfo),
    ReportProblem(ProblemReport),
}

impl DelayedSerde for PushNotificationsFCM {
    type MsgType<'a> = (PushNotificationsFCMKind, &'a str);

    fn delayed_deserialize<'de, D>(
        msg_type: Self::MsgType<'de>,
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (protocol, kind_str) = msg_type;

        let kind = match protocol {
            PushNotificationsFCMKind::V1(PushNotificationsFCMTypeV1::V1_0(kind)) => kind.kind_from_str(kind_str),
        };

        match kind.map_err(D::Error::custom)? {
            PushNotificationsFCMTypeV1_0::DeviceInfo => DeviceInfo::deserialize(deserializer).map(From::from),
            PushNotificationsFCMTypeV1_0::GetDeviceInfo => {
                GetDeviceInfo::deserialize(deserializer).map(From::from)
            },
            PushNotificationsFCMTypeV1_0::SetDeviceInfo => {
                SetDeviceInfo::deserialize(deserializer).map(From::from)
            },
            PushNotificationsFCMTypeV1_0::ReportProblem => {
                ProblemReport::deserialize(deserializer).map(From::from)
            }
        }
    }

    fn delayed_serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::DeviceInfo(v) => MsgWithType::from(v).serialize(serializer),
            Self::GetDeviceInfo(v) => MsgWithType::from(v).serialize(serializer),
            Self::SetDeviceInfo(v) => MsgWithType::from(v).serialize(serializer),
            Self::ReportProblem(v) => MsgWithType::from(v).serialize(serializer),
        }
    }
}

transit_to_aries_msg!(DeviceInfoContent: DeviceInfoDecorators, PushNotificationsFCM);
transit_to_aries_msg!(GetDeviceInfoContent: GetDeviceInfoDecorators, PushNotificationsFCM);
transit_to_aries_msg!(SetDeviceInfoContent: SetDeviceInfoDecorators, PushNotificationsFCM);
transit_to_aries_msg!(ProblemReportContent: ProblemReportDecorators, PushNotificationsFCM);

into_msg_with_type!(DeviceInfo, PushNotificationsFCMTypeV1_0, DeviceInfo);
into_msg_with_type!(GetDeviceInfo, PushNotificationsFCMTypeV1_0, GetDeviceInfo);
into_msg_with_type!(SetDeviceInfo, PushNotificationsFCMTypeV1_0, SetDeviceInfo);
into_msg_with_type!(ProblemReport, PushNotificationsFCMTypeV1_0, ReportProblem);
