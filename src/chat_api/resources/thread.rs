//! Thread resource
//!
//! Returns details about a user's read state within a thread, used to identify read and unread messages. For an example, see [Get details about a user's thread read state](https://developers.google.com/workspace/chat/get-thread-read-state). Requires [user authentication](https://developers.google.com/workspace/chat/authenticate-authorize-chat-user) with one of the following [authorization scopes](https://developers.google.com/workspace/chat/authenticate-authorize#chat-api-scopes): - `https://www.googleapis.com/auth/chat.users.readstate.readonly` - `https://www.googleapis.com/auth/chat.users.readstate`

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Thread resource handler
pub struct Thread<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Thread<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }




    /// Read/describe a thread
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
    async fn test_thread_operations() {
        // Test thread CRUD operations
    }
}
