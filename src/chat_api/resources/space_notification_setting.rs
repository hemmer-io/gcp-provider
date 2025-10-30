//! Space_notification_setting resource
//!
//! Gets the space notification setting. For an example, see [Get the caller's space notification setting](https://developers.google.com/workspace/chat/get-space-notification-setting). Requires [user authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user) with the [authorization scope](https://developers.google.com/workspace/chat/authenticate-authorize#chat-api-scopes): - `https://www.googleapis.com/auth/chat.users.spacesettings`

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Space_notification_setting resource handler
pub struct Space_notification_setting<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Space_notification_setting<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a space_notification_setting
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        todo!("Implement read for Gcp")

    }



    /// Update a space_notification_setting
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, name: Option<String>, mute_setting: Option<String>, notification_setting: Option<String>) -> Result<()> {

        todo!("Implement update for Gcp")

    }



}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_space_notification_setting_operations() {
        // Test space_notification_setting CRUD operations
    }
}
