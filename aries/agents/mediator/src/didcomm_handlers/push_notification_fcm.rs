use aries_vcx_wallet::wallet::base_wallet::BaseWallet;
use messages::msg_fields::protocols::push_notifications_fcm::PushNotificationsFCM;

use super::utils::prelude::*;
use crate::mediation::device_info::handle_device_info;

pub async fn handle_push_notification_fcm_protocol(
    agent: &ArcAgent<impl BaseWallet, impl MediatorPersistence>,
    push_notification: PushNotificationsFCM,
    auth_pubkey: &str,
) -> Result<PushNotificationsFCM, String>  {
    let response = handle_device_info(
        agent.get_persistence_ref(),
        push_notification,
        auth_pubkey
    )
    .await;
    Ok(response)
}
