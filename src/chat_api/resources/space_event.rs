//! Space_event resource
//!
//! Returns an event from a Google Chat space. The [event payload](https://developers.google.com/workspace/chat/api/reference/rest/v1/spaces.spaceEvents#SpaceEvent.FIELDS.oneof_payload) contains the most recent version of the resource that changed. For example, if you request an event about a new message but the message was later updated, the server returns the updated `Message` resource in the event payload. Note: The `permissionSettings` field is not returned in the Space object of the Space event data for this request. Supports the following types of [authentication](https://developers.google.com/workspace/chat/authenticate-authorize) with an [authorization scope](https://developers.google.com/workspace/chat/authenticate-authorize#chat-api-scopes) appropriate for reading the requested data: - [App authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-app) with [administrator approval](https://support.google.com/a?p=chat-app-auth) in [Developer Preview](https://developers.google.com/workspace/preview) with one of the following authorization scopes: - `https://www.googleapis.com/auth/chat.app.spaces` - `https://www.googleapis.com/auth/chat.app.messages.readonly` - `https://www.googleapis.com/auth/chat.app.memberships` - [User authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user) with one of the following authorization scopes: - `https://www.googleapis.com/auth/chat.spaces.readonly` - `https://www.googleapis.com/auth/chat.spaces` - `https://www.googleapis.com/auth/chat.messages.readonly` - `https://www.googleapis.com/auth/chat.messages` - `https://www.googleapis.com/auth/chat.messages.reactions.readonly` - `https://www.googleapis.com/auth/chat.messages.reactions` - `https://www.googleapis.com/auth/chat.memberships.readonly` - `https://www.googleapis.com/auth/chat.memberships` To get an event, the authenticated caller must be a member of the space. For an example, see [Get details about an event from a Google Chat space](https://developers.google.com/workspace/chat/get-space-event).

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Space_event resource handler
pub struct Space_event<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Space_event<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a space_event
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        todo!("Implement read for Gcp")

    }





}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_space_event_operations() {
        // Test space_event CRUD operations
    }
}
