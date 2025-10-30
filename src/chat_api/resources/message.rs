//! Message resource
//!
//! Creates a message in a Google Chat space. For an example, see [Send a message](https://developers.google.com/workspace/chat/create-messages). Supports the following types of [authentication](https://developers.google.com/workspace/chat/authenticate-authorize): - [App authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-app) with the authorization scope: - `https://www.googleapis.com/auth/chat.bot` - [User authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user) with one of the following authorization scopes: - `https://www.googleapis.com/auth/chat.messages.create` - `https://www.googleapis.com/auth/chat.messages` - `https://www.googleapis.com/auth/chat.import` (import mode spaces only) Chat attributes the message sender differently depending on the type of authentication that you use in your request. The following image shows how Chat attributes a message when you use app authentication. Chat displays the Chat app as the message sender. The content of the message can contain text (`text`), cards (`cardsV2`), and accessory widgets (`accessoryWidgets`). ![Message sent with app authentication](https://developers.google.com/workspace/chat/images/message-app-auth.svg) The following image shows how Chat attributes a message when you use user authentication. Chat displays the user as the message sender and attributes the Chat app to the message by displaying its name. The content of message can only contain text (`text`). ![Message sent with user authentication](https://developers.google.com/workspace/chat/images/message-user-auth.svg) The maximum message size, including the message contents, is 32,000 bytes. For [webhook](https://developers.google.com/workspace/chat/quickstart/webhooks) requests, the response doesn't contain the full message. The response only populates the `name` and `thread.name` fields in addition to the information that was in the request.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Message resource handler
pub struct Message<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Message<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new message
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, annotations: Option<Vec<String>>, text: Option<String>, attached_gifs: Option<Vec<String>>, deletion_metadata: Option<String>, client_assigned_message_id: Option<String>, action_response: Option<String>, argument_text: Option<String>, quoted_message_metadata: Option<String>, fallback_text: Option<String>, slash_command: Option<String>, thread_reply: Option<bool>, cards: Option<Vec<String>>, emoji_reaction_summaries: Option<Vec<String>>, space: Option<String>, name: Option<String>, cards_v2: Option<Vec<String>>, create_time: Option<String>, formatted_text: Option<String>, matched_url: Option<String>, accessory_widgets: Option<Vec<String>>, private_message_viewer: Option<String>, sender: Option<String>, delete_time: Option<String>, attachment: Option<Vec<String>>, last_update_time: Option<String>, thread: Option<String>, parent: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }



    /// Read/describe a message
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        todo!("Implement read for Gcp")

    }



    /// Update a message
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str, annotations: Option<Vec<String>>, text: Option<String>, attached_gifs: Option<Vec<String>>, deletion_metadata: Option<String>, client_assigned_message_id: Option<String>, action_response: Option<String>, argument_text: Option<String>, quoted_message_metadata: Option<String>, fallback_text: Option<String>, slash_command: Option<String>, thread_reply: Option<bool>, cards: Option<Vec<String>>, emoji_reaction_summaries: Option<Vec<String>>, space: Option<String>, name: Option<String>, cards_v2: Option<Vec<String>>, create_time: Option<String>, formatted_text: Option<String>, matched_url: Option<String>, accessory_widgets: Option<Vec<String>>, private_message_viewer: Option<String>, sender: Option<String>, delete_time: Option<String>, attachment: Option<Vec<String>>, last_update_time: Option<String>, thread: Option<String>) -> Result<()> {

        todo!("Implement update for Gcp")

    }



    /// Delete a message
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn delete(&self, id: &str) -> Result<()> {

        todo!("Implement delete for Gcp")

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_message_operations() {
        // Test message CRUD operations
    }
}
