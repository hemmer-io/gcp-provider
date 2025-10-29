//! Youtube Service
//!
//! Auto-generated service module for youtube

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for youtube
pub struct YoutubeService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> YoutubeService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get playlist_item resource handler
    pub fn playlist_item(&self) -> resources::Playlist_item<'_> {
        resources::Playlist_item::new(self.provider)
    }
    /// Get thumbnail resource handler
    pub fn thumbnail(&self) -> resources::Thumbnail<'_> {
        resources::Thumbnail::new(self.provider)
    }
    /// Get video resource handler
    pub fn video(&self) -> resources::Video<'_> {
        resources::Video::new(self.provider)
    }
    /// Get member resource handler
    pub fn member(&self) -> resources::Member<'_> {
        resources::Member::new(self.provider)
    }
    /// Get live_chat_message resource handler
    pub fn live_chat_message(&self) -> resources::Live_chat_message<'_> {
        resources::Live_chat_message::new(self.provider)
    }
    /// Get comment resource handler
    pub fn comment(&self) -> resources::Comment<'_> {
        resources::Comment::new(self.provider)
    }
    /// Get comment_thread resource handler
    pub fn comment_thread(&self) -> resources::Comment_thread<'_> {
        resources::Comment_thread::new(self.provider)
    }
    /// Get subscription resource handler
    pub fn subscription(&self) -> resources::Subscription<'_> {
        resources::Subscription::new(self.provider)
    }
    /// Get search resource handler
    pub fn search(&self) -> resources::Search<'_> {
        resources::Search::new(self.provider)
    }
    /// Get i18n_region resource handler
    pub fn i18n_region(&self) -> resources::I18n_region<'_> {
        resources::I18n_region::new(self.provider)
    }
    /// Get v3 resource handler
    pub fn v3(&self) -> resources::V3<'_> {
        resources::V3::new(self.provider)
    }
    /// Get live_broadcast resource handler
    pub fn live_broadcast(&self) -> resources::Live_broadcast<'_> {
        resources::Live_broadcast::new(self.provider)
    }
    /// Get video_categorie resource handler
    pub fn video_categorie(&self) -> resources::Video_categorie<'_> {
        resources::Video_categorie::new(self.provider)
    }
    /// Get channel resource handler
    pub fn channel(&self) -> resources::Channel<'_> {
        resources::Channel::new(self.provider)
    }
    /// Get message resource handler
    pub fn message(&self) -> resources::Message<'_> {
        resources::Message::new(self.provider)
    }
    /// Get video_trainability resource handler
    pub fn video_trainability(&self) -> resources::Video_trainability<'_> {
        resources::Video_trainability::new(self.provider)
    }
    /// Get live_chat_moderator resource handler
    pub fn live_chat_moderator(&self) -> resources::Live_chat_moderator<'_> {
        resources::Live_chat_moderator::new(self.provider)
    }
    /// Get channel_banner resource handler
    pub fn channel_banner(&self) -> resources::Channel_banner<'_> {
        resources::Channel_banner::new(self.provider)
    }
    /// Get playlist_image resource handler
    pub fn playlist_image(&self) -> resources::Playlist_image<'_> {
        resources::Playlist_image::new(self.provider)
    }
    /// Get i18n_language resource handler
    pub fn i18n_language(&self) -> resources::I18n_language<'_> {
        resources::I18n_language::new(self.provider)
    }
    /// Get abuse_report resource handler
    pub fn abuse_report(&self) -> resources::Abuse_report<'_> {
        resources::Abuse_report::new(self.provider)
    }
    /// Get playlist resource handler
    pub fn playlist(&self) -> resources::Playlist<'_> {
        resources::Playlist::new(self.provider)
    }
    /// Get live_chat_ban resource handler
    pub fn live_chat_ban(&self) -> resources::Live_chat_ban<'_> {
        resources::Live_chat_ban::new(self.provider)
    }
    /// Get third_party_link resource handler
    pub fn third_party_link(&self) -> resources::Third_party_link<'_> {
        resources::Third_party_link::new(self.provider)
    }
    /// Get memberships_level resource handler
    pub fn memberships_level(&self) -> resources::Memberships_level<'_> {
        resources::Memberships_level::new(self.provider)
    }
    /// Get watermark resource handler
    pub fn watermark(&self) -> resources::Watermark<'_> {
        resources::Watermark::new(self.provider)
    }
    /// Get channel_section resource handler
    pub fn channel_section(&self) -> resources::Channel_section<'_> {
        resources::Channel_section::new(self.provider)
    }
    /// Get activitie resource handler
    pub fn activitie(&self) -> resources::Activitie<'_> {
        resources::Activitie::new(self.provider)
    }
    /// Get video_abuse_report_reason resource handler
    pub fn video_abuse_report_reason(&self) -> resources::Video_abuse_report_reason<'_> {
        resources::Video_abuse_report_reason::new(self.provider)
    }
    /// Get caption resource handler
    pub fn caption(&self) -> resources::Caption<'_> {
        resources::Caption::new(self.provider)
    }
    /// Get live_stream resource handler
    pub fn live_stream(&self) -> resources::Live_stream<'_> {
        resources::Live_stream::new(self.provider)
    }
    /// Get test resource handler
    pub fn test(&self) -> resources::Test<'_> {
        resources::Test::new(self.provider)
    }
    /// Get super_chat_event resource handler
    pub fn super_chat_event(&self) -> resources::Super_chat_event<'_> {
        resources::Super_chat_event::new(self.provider)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        // Service creation test
    }
}
