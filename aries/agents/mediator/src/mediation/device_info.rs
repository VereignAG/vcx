use std::sync::Arc;
use uuid::Uuid;
use messages::msg_fields::protocols::{
    push_notifications_fcm::{
        PushNotificationsFCM,
        device_info::{DeviceInfo, DeviceInfoContent, DeviceInfoDecorators},
        set_device_info::SetDeviceInfo
    },
};
use messages::msg_fields::protocols::push_notifications_fcm::problem_report::{ProblemCode, ProblemReport, ProblemReportContent};
use crate::persistence::MediatorPersistence;

pub async fn handle_device_info(
    storage: Arc<impl MediatorPersistence>,
    push_notification_fcm: PushNotificationsFCM,
    auth_pubkey: &str,
) -> PushNotificationsFCM {
    match push_notification_fcm {
        PushNotificationsFCM::DeviceInfo(_) => {
            let problem_report = report_problem(
                ProblemCode::RequestNotAccepted,
                "DeviceInfo message should not be handled here."
            );
            PushNotificationsFCM::ReportProblem(problem_report)
        }
        PushNotificationsFCM::GetDeviceInfo(_) => {
            // Handle GetDeviceInfo request
            handle_get_device_info(storage, auth_pubkey).await
        }
        PushNotificationsFCM::SetDeviceInfo(set_device_info) => {
            // Handle SetDeviceInfo request
            handle_set_device_info(storage, set_device_info, auth_pubkey).await
        }
        _ => {
            let problem_report = report_problem(
                ProblemCode::RequestProcessingError,
                "This operation is not supported in the current context."
            );
            PushNotificationsFCM::ReportProblem(problem_report)
        }
    }
}

async fn handle_set_device_info<T: MediatorPersistence>(
    storage: Arc<T>,
    set_device_info: SetDeviceInfo,
    auth_pubkey: &str,
) -> PushNotificationsFCM {
    let token = set_device_info
        .content
        .device_token;
    let platform = set_device_info
        .content
        .device_platform;

    // Validate that both token and platform are provided or omitted together (delete device info)
    if (token.is_none() && platform.is_some()) ||
         (token.is_some() && platform.is_none()) {
          let problem_report = report_problem(
                ProblemCode::RequestNotAccepted,
                "Both device token and platform must be provided or omitted together."
          );
          return PushNotificationsFCM::ReportProblem(problem_report);
     }

    // validate platform
    if platform.is_some() {
        let platform_value = platform.as_ref().unwrap();
        if !["android", "ios"].contains(&platform_value.as_str()) {
            let problem_report = report_problem(
                ProblemCode::RequestNotAccepted,
                "Device platform must be one of: android, ios"
            );
            return PushNotificationsFCM::ReportProblem(problem_report);
        }
    }

    match storage
        .set_device_info(auth_pubkey, token.clone(), platform.clone())
        .await {
        Ok(_) => {
            let content = DeviceInfoContent::builder()
                .device_token(token.unwrap_or_default())
                .device_platform(platform.unwrap_or_default())
                .build();
            let device_info = DeviceInfo::builder()
                .content(content)
                .decorators(DeviceInfoDecorators::default())
                .id(Uuid::new_v4().to_string())
                .build();
            PushNotificationsFCM::DeviceInfo(device_info)
        }
        Err(_e) => {
            let problem_report = report_problem(
                ProblemCode::ResponseProcessingError,
                "Failed to set device info"
            );
            PushNotificationsFCM::ReportProblem(problem_report)
        }

    }
}

async fn handle_get_device_info<T: MediatorPersistence>(
    storage: Arc<T>,
    auth_pubkey: &str,
) -> PushNotificationsFCM {
    let device_info = storage
        .retrieve_device_info(auth_pubkey).await;
    match device_info {
        Ok(info) => {
            let token = info.token.unwrap_or_default();
            let platform = info.platform.unwrap_or_default();
            let content = DeviceInfoContent::builder()
                .device_token(token)
                .device_platform(platform)
                .build();
            let device_info = DeviceInfo::builder()
                .content(content)
                .decorators(DeviceInfoDecorators::default())
                .id(Uuid::new_v4().to_string())
                .build();
            PushNotificationsFCM::DeviceInfo(device_info)
        }
        Err(_) => {
            let problem_report = report_problem(
                ProblemCode::RequestNotAccepted,
                "Device info not found"
            );
            PushNotificationsFCM::ReportProblem(problem_report)
        }
    }
}

fn report_problem(
    code: ProblemCode,
    message: &str,
) -> ProblemReport {
    ProblemReport::builder()
        .content(ProblemReportContent::builder()
            .problem_code(code)
            .explain(message.to_string())
            .build())
        .decorators(Default::default())
        .id(Uuid::new_v4().to_string())
        .build()
}
