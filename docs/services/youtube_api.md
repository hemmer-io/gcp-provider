# Youtube_api Service



**Resources**: 33

---

## Overview

The youtube_api service provides access to 33 resource types:

- [Playlist_image](#playlist_image) [CRUD]
- [Subscription](#subscription) [CRD]
- [V3](#v3) [U]
- [Channel](#channel) [RU]
- [Member](#member) [R]
- [Watermark](#watermark) [C]
- [Video_trainability](#video_trainability) [R]
- [Live_chat_moderator](#live_chat_moderator) [CRD]
- [Playlist](#playlist) [CRUD]
- [Search](#search) [R]
- [Abuse_report](#abuse_report) [C]
- [Super_chat_event](#super_chat_event) [R]
- [Thumbnail](#thumbnail) [C]
- [Activitie](#activitie) [R]
- [Live_stream](#live_stream) [CRUD]
- [Live_chat_message](#live_chat_message) [CRD]
- [Caption](#caption) [CRUD]
- [Video_abuse_report_reason](#video_abuse_report_reason) [R]
- [Channel_section](#channel_section) [CRUD]
- [Comment](#comment) [CRUD]
- [Third_party_link](#third_party_link) [CRUD]
- [Video_categorie](#video_categorie) [R]
- [I18n_region](#i18n_region) [R]
- [I18n_language](#i18n_language) [R]
- [Memberships_level](#memberships_level) [R]
- [Video](#video) [CRUD]
- [Channel_banner](#channel_banner) [C]
- [Test](#test) [C]
- [Playlist_item](#playlist_item) [CRUD]
- [Comment_thread](#comment_thread) [CR]
- [Live_broadcast](#live_broadcast) [CRUD]
- [Live_chat_ban](#live_chat_ban) [CD]
- [Message](#message) [R]

---

## Resources


### Playlist_image

Inserts a new resource into this collection.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "youtube#playlistImages". |
| `id` | String |  | Identifies this resource (playlist id and image type). |
| `snippet` | String |  |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> |  |
| `page_info` | String | General pagination information. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "youtube#playlistImageListResponse". |
| `prev_page_token` | String | The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set. |
| `next_page_token` | String | The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create playlist_image
playlist_image = provider.youtube_api.Playlist_image {
}

# Access playlist_image outputs
playlist_image_id = playlist_image.id
playlist_image_items = playlist_image.items
playlist_image_page_info = playlist_image.page_info
playlist_image_kind = playlist_image.kind
playlist_image_prev_page_token = playlist_image.prev_page_token
playlist_image_next_page_token = playlist_image.next_page_token
```

---


### Subscription

Inserts a new resource into this collection.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `snippet` | String |  | The snippet object contains basic details about the subscription, including its title and the channel that the user subscribed to. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "youtube#subscription". |
| `etag` | String |  | Etag of this resource. |
| `content_details` | String |  | The contentDetails object contains basic statistics about the subscription. |
| `id` | String |  | The ID that YouTube uses to uniquely identify the subscription. |
| `subscriber_snippet` | String |  | The subscriberSnippet object contains basic details about the subscriber. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | A list of subscriptions that match the request criteria. |
| `event_id` | String | Serialized EventId of the request which produced this response. |
| `next_page_token` | String | The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set. |
| `token_pagination` | String |  |
| `visitor_id` | String | The visitorId identifies the visitor. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "youtube#subscriptionListResponse". |
| `prev_page_token` | String | The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set. |
| `page_info` | String |  |
| `etag` | String | Etag of this resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create subscription
subscription = provider.youtube_api.Subscription {
}

# Access subscription outputs
subscription_id = subscription.id
subscription_items = subscription.items
subscription_event_id = subscription.event_id
subscription_next_page_token = subscription.next_page_token
subscription_token_pagination = subscription.token_pagination
subscription_visitor_id = subscription.visitor_id
subscription_kind = subscription.kind
subscription_prev_page_token = subscription.prev_page_token
subscription_page_info = subscription.page_info
subscription_etag = subscription.etag
```

---


### V3



**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `snippet` | String |  | The snippet object contains basic details about the comment thread and also the top level comment. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "youtube#commentThread". |
| `replies` | String |  | The replies object contains a limited number of replies (if any) to the top level comment found in the snippet. |
| `etag` | String |  | Etag of this resource. |
| `id` | String |  | The ID that YouTube uses to uniquely identify the comment thread. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

```

---


### Channel

Retrieves a list of resources, possibly filtered.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `snippet` | String |  | The snippet object contains basic details about the channel, such as its title, description, and thumbnail images. |
| `conversion_pings` | String |  | The conversionPings object encapsulates information about conversion pings that need to be respected by the channel. |
| `statistics` | String |  | The statistics object encapsulates statistics for the channel. |
| `status` | String |  | The status object encapsulates information about the privacy status of the channel. |
| `topic_details` | String |  | The topicDetails object encapsulates information about Freebase topics associated with the channel. |
| `audit_details` | String |  | The auditionDetails object encapsulates channel data that is relevant for YouTube Partners during the audition process. |
| `content_details` | String |  | The contentDetails object encapsulates information about the channel's content. |
| `content_owner_details` | String |  | The contentOwnerDetails object encapsulates channel data that is relevant for YouTube Partners linked with the channel. |
| `etag` | String |  | Etag of this resource. |
| `id` | String |  | The ID that YouTube uses to uniquely identify the channel. |
| `branding_settings` | String |  | The brandingSettings object encapsulates information about the branding of the channel. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "youtube#channel". |
| `localizations` | HashMap<String, String> |  | Localizations for different languages |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `event_id` | String | Serialized EventId of the request which produced this response. |
| `page_info` | String | General pagination information. |
| `etag` | String | Etag of this resource. |
| `visitor_id` | String | The visitorId identifies the visitor. |
| `token_pagination` | String |  |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "youtube#channelListResponse". |
| `items` | Vec<String> |  |
| `prev_page_token` | String | The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set. |
| `next_page_token` | String | The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access channel outputs
channel_id = channel.id
channel_event_id = channel.event_id
channel_page_info = channel.page_info
channel_etag = channel.etag
channel_visitor_id = channel.visitor_id
channel_token_pagination = channel.token_pagination
channel_kind = channel.kind
channel_items = channel.items
channel_prev_page_token = channel.prev_page_token
channel_next_page_token = channel.next_page_token
```

---


### Member

Retrieves a list of members that match the request criteria for a channel.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `token_pagination` | String |  |
| `etag` | String | Etag of this resource. |
| `visitor_id` | String | The visitorId identifies the visitor. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "youtube#memberListResponse". |
| `event_id` | String | Serialized EventId of the request which produced this response. |
| `next_page_token` | String | The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set. |
| `page_info` | String |  |
| `items` | Vec<String> | A list of members that match the request criteria. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access member outputs
member_id = member.id
member_token_pagination = member.token_pagination
member_etag = member.etag
member_visitor_id = member.visitor_id
member_kind = member.kind
member_event_id = member.event_id
member_next_page_token = member.next_page_token
member_page_info = member.page_info
member_items = member.items
```

---


### Watermark

Allows upload of watermark image and setting it for a channel.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `image_bytes` | String |  | The bytes the uploaded image. Only used in api to youtube communication. |
| `target_channel_id` | String |  | The channel to which this branding links. If not present it defaults to the current channel. |
| `image_url` | String |  | The url of the uploaded image. Only used in apiary to api communication. |
| `position` | String |  | The spatial position within the video where the branding watermark will be displayed. |
| `timing` | String |  | The temporal position within the video where watermark will be displayed. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create watermark
watermark = provider.youtube_api.Watermark {
}

```

---


### Video_trainability

Returns the trainability status of a video.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "youtube#videoTrainability". |
| `etag` | String | Etag of this resource. |
| `video_id` | String | The ID of the video. |
| `permitted` | Vec<String> | Specifies who is allowed to train on the video. Valid values are: - a single string "all" - a single string "none" - a list of allowed parties |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access video_trainability outputs
video_trainability_id = video_trainability.id
video_trainability_kind = video_trainability.kind
video_trainability_etag = video_trainability.etag
video_trainability_video_id = video_trainability.video_id
video_trainability_permitted = video_trainability.permitted
```

---


### Live_chat_moderator

Inserts a new resource into this collection.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String |  | The ID that YouTube assigns to uniquely identify the moderator. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "youtube#liveChatModerator". |
| `snippet` | String |  | The snippet object contains basic details about the moderator. |
| `etag` | String |  | Etag of this resource. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | Etag of this resource. |
| `page_info` | String | General pagination information. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "youtube#liveChatModeratorListResponse". |
| `next_page_token` | String | The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set. |
| `prev_page_token` | String | The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set. |
| `event_id` | String | Serialized EventId of the request which produced this response. |
| `items` | Vec<String> | A list of moderators that match the request criteria. |
| `token_pagination` | String |  |
| `visitor_id` | String | The visitorId identifies the visitor. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create live_chat_moderator
live_chat_moderator = provider.youtube_api.Live_chat_moderator {
}

# Access live_chat_moderator outputs
live_chat_moderator_id = live_chat_moderator.id
live_chat_moderator_etag = live_chat_moderator.etag
live_chat_moderator_page_info = live_chat_moderator.page_info
live_chat_moderator_kind = live_chat_moderator.kind
live_chat_moderator_next_page_token = live_chat_moderator.next_page_token
live_chat_moderator_prev_page_token = live_chat_moderator.prev_page_token
live_chat_moderator_event_id = live_chat_moderator.event_id
live_chat_moderator_items = live_chat_moderator.items
live_chat_moderator_token_pagination = live_chat_moderator.token_pagination
live_chat_moderator_visitor_id = live_chat_moderator.visitor_id
```

---


### Playlist

Inserts a new resource into this collection.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `player` | String |  | The player object contains information that you would use to play the playlist in an embedded player. |
| `status` | String |  | The status object contains status information for the playlist. |
| `content_details` | String |  | The contentDetails object contains information like video count. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "youtube#playlist". |
| `etag` | String |  | Etag of this resource. |
| `snippet` | String |  | The snippet object contains basic details about the playlist, such as its title and description. |
| `localizations` | HashMap<String, String> |  | Localizations for different languages |
| `id` | String |  | The ID that YouTube uses to uniquely identify the playlist. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set. |
| `page_info` | String | General pagination information. |
| `visitor_id` | String | The visitorId identifies the visitor. |
| `items` | Vec<String> | A list of playlists that match the request criteria |
| `event_id` | String | Serialized EventId of the request which produced this response. |
| `etag` | String | Etag of this resource. |
| `prev_page_token` | String | The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "youtube#playlistListResponse". |
| `token_pagination` | String |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create playlist
playlist = provider.youtube_api.Playlist {
}

# Access playlist outputs
playlist_id = playlist.id
playlist_next_page_token = playlist.next_page_token
playlist_page_info = playlist.page_info
playlist_visitor_id = playlist.visitor_id
playlist_items = playlist.items
playlist_event_id = playlist.event_id
playlist_etag = playlist.etag
playlist_prev_page_token = playlist.prev_page_token
playlist_kind = playlist.kind
playlist_token_pagination = playlist.token_pagination
```

---


### Search

Retrieves a list of search resources

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `event_id` | String | Serialized EventId of the request which produced this response. |
| `items` | Vec<String> | Pagination information for token pagination. |
| `etag` | String | Etag of this resource. |
| `page_info` | String | General pagination information. |
| `prev_page_token` | String | The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set. |
| `region_code` | String |  |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "youtube#searchListResponse". |
| `next_page_token` | String | The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set. |
| `token_pagination` | String |  |
| `visitor_id` | String | The visitorId identifies the visitor. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access search outputs
search_id = search.id
search_event_id = search.event_id
search_items = search.items
search_etag = search.etag
search_page_info = search.page_info
search_prev_page_token = search.prev_page_token
search_region_code = search.region_code
search_kind = search.kind
search_next_page_token = search.next_page_token
search_token_pagination = search.token_pagination
search_visitor_id = search.visitor_id
```

---


### Abuse_report

Inserts a new resource into this collection.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `abuse_types` | Vec<String> |  |  |
| `description` | String |  |  |
| `subject` | String |  |  |
| `related_entities` | Vec<String> |  |  |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create abuse_report
abuse_report = provider.youtube_api.Abuse_report {
}

```

---


### Super_chat_event

Retrieves a list of resources, possibly filtered.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `page_info` | String |  |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "youtube#superChatEventListResponse". |
| `items` | Vec<String> | A list of Super Chat purchases that match the request criteria. |
| `next_page_token` | String | The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set. |
| `token_pagination` | String |  |
| `visitor_id` | String | The visitorId identifies the visitor. |
| `event_id` | String | Serialized EventId of the request which produced this response. |
| `etag` | String | Etag of this resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access super_chat_event outputs
super_chat_event_id = super_chat_event.id
super_chat_event_page_info = super_chat_event.page_info
super_chat_event_kind = super_chat_event.kind
super_chat_event_items = super_chat_event.items
super_chat_event_next_page_token = super_chat_event.next_page_token
super_chat_event_token_pagination = super_chat_event.token_pagination
super_chat_event_visitor_id = super_chat_event.visitor_id
super_chat_event_event_id = super_chat_event.event_id
super_chat_event_etag = super_chat_event.etag
```

---


### Thumbnail

As this is not an insert in a strict sense (it supports uploading/setting of a thumbnail for multiple videos, which doesn't result in creation of a single resource), I use a custom verb here.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create thumbnail
thumbnail = provider.youtube_api.Thumbnail {
}

```

---


### Activitie

Retrieves a list of resources, possibly filtered.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `token_pagination` | String |  |
| `visitor_id` | String | The visitorId identifies the visitor. |
| `etag` | String | Etag of this resource. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "youtube#activityListResponse". |
| `event_id` | String | Serialized EventId of the request which produced this response. |
| `next_page_token` | String | The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set. |
| `prev_page_token` | String | The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set. |
| `page_info` | String | General pagination information. |
| `items` | Vec<String> |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access activitie outputs
activitie_id = activitie.id
activitie_token_pagination = activitie.token_pagination
activitie_visitor_id = activitie.visitor_id
activitie_etag = activitie.etag
activitie_kind = activitie.kind
activitie_event_id = activitie.event_id
activitie_next_page_token = activitie.next_page_token
activitie_prev_page_token = activitie.prev_page_token
activitie_page_info = activitie.page_info
activitie_items = activitie.items
```

---


### Live_stream

Inserts a new stream for the authenticated user.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cdn` | String |  | The cdn object defines the live stream's content delivery network (CDN) settings. These settings provide details about the manner in which you stream your content to YouTube. |
| `content_details` | String |  | The content_details object contains information about the stream, including the closed captions ingestion URL. |
| `id` | String |  | The ID that YouTube assigns to uniquely identify the stream. |
| `status` | String |  | The status object contains information about live stream's status. |
| `snippet` | String |  | The snippet object contains basic details about the stream, including its channel, title, and description. |
| `etag` | String |  | Etag of this resource. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "youtube#liveStream". |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `visitor_id` | String | The visitorId identifies the visitor. |
| `page_info` | String |  |
| `items` | Vec<String> | A list of live streams that match the request criteria. |
| `next_page_token` | String | The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "youtube#liveStreamListResponse". |
| `token_pagination` | String |  |
| `etag` | String | Etag of this resource. |
| `event_id` | String | Serialized EventId of the request which produced this response. |
| `prev_page_token` | String | The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create live_stream
live_stream = provider.youtube_api.Live_stream {
}

# Access live_stream outputs
live_stream_id = live_stream.id
live_stream_visitor_id = live_stream.visitor_id
live_stream_page_info = live_stream.page_info
live_stream_items = live_stream.items
live_stream_next_page_token = live_stream.next_page_token
live_stream_kind = live_stream.kind
live_stream_token_pagination = live_stream.token_pagination
live_stream_etag = live_stream.etag
live_stream_event_id = live_stream.event_id
live_stream_prev_page_token = live_stream.prev_page_token
```

---


### Live_chat_message

Inserts a new resource into this collection.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "youtube#liveChatMessage". |
| `author_details` | String |  | The authorDetails object contains basic details about the user that posted this message. |
| `id` | String |  | The ID that YouTube assigns to uniquely identify the message. |
| `snippet` | String |  | The snippet object contains basic details about the message. |
| `etag` | String |  | Etag of this resource. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `event_id` | String | Serialized EventId of the request which produced this response. |
| `etag` | String | Etag of this resource. |
| `page_info` | String | General pagination information. |
| `visitor_id` | String | The visitorId identifies the visitor. |
| `offline_at` | String | The date and time when the underlying stream went offline. |
| `items` | Vec<String> |  |
| `active_poll_item` | String | Set when there is an active poll. |
| `token_pagination` | String |  |
| `polling_interval_millis` | i64 | The amount of time the client should wait before polling again. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "youtube#liveChatMessageListResponse". |
| `next_page_token` | String |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create live_chat_message
live_chat_message = provider.youtube_api.Live_chat_message {
}

# Access live_chat_message outputs
live_chat_message_id = live_chat_message.id
live_chat_message_event_id = live_chat_message.event_id
live_chat_message_etag = live_chat_message.etag
live_chat_message_page_info = live_chat_message.page_info
live_chat_message_visitor_id = live_chat_message.visitor_id
live_chat_message_offline_at = live_chat_message.offline_at
live_chat_message_items = live_chat_message.items
live_chat_message_active_poll_item = live_chat_message.active_poll_item
live_chat_message_token_pagination = live_chat_message.token_pagination
live_chat_message_polling_interval_millis = live_chat_message.polling_interval_millis
live_chat_message_kind = live_chat_message.kind
live_chat_message_next_page_token = live_chat_message.next_page_token
```

---


### Caption

Inserts a new resource into this collection.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String |  | The ID that YouTube uses to uniquely identify the caption track. |
| `snippet` | String |  | The snippet object contains basic details about the caption. |
| `etag` | String |  | Etag of this resource. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "youtube#caption". |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create caption
caption = provider.youtube_api.Caption {
}

# Access caption outputs
caption_id = caption.id
```

---


### Video_abuse_report_reason

Retrieves a list of resources, possibly filtered.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `event_id` | String | Serialized EventId of the request which produced this response. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string `"youtube#videoAbuseReportReasonListResponse"`. |
| `etag` | String | Etag of this resource. |
| `visitor_id` | String | The `visitorId` identifies the visitor. |
| `items` | Vec<String> | A list of valid abuse reasons that are used with `video.ReportAbuse`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access video_abuse_report_reason outputs
video_abuse_report_reason_id = video_abuse_report_reason.id
video_abuse_report_reason_event_id = video_abuse_report_reason.event_id
video_abuse_report_reason_kind = video_abuse_report_reason.kind
video_abuse_report_reason_etag = video_abuse_report_reason.etag
video_abuse_report_reason_visitor_id = video_abuse_report_reason.visitor_id
video_abuse_report_reason_items = video_abuse_report_reason.items
```

---


### Channel_section

Inserts a new resource into this collection.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `targeting` | String |  | The targeting object contains basic targeting settings about the channel section. |
| `snippet` | String |  | The snippet object contains basic details about the channel section, such as its type, style and title. |
| `content_details` | String |  | The contentDetails object contains details about the channel section content, such as a list of playlists or channels featured in the section. |
| `id` | String |  | The ID that YouTube uses to uniquely identify the channel section. |
| `localizations` | HashMap<String, String> |  | Localizations for different languages |
| `etag` | String |  | Etag of this resource. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "youtube#channelSection". |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "youtube#channelSectionListResponse". |
| `visitor_id` | String | The visitorId identifies the visitor. |
| `event_id` | String | Serialized EventId of the request which produced this response. |
| `etag` | String | Etag of this resource. |
| `items` | Vec<String> | A list of ChannelSections that match the request criteria. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create channel_section
channel_section = provider.youtube_api.Channel_section {
}

# Access channel_section outputs
channel_section_id = channel_section.id
channel_section_kind = channel_section.kind
channel_section_visitor_id = channel_section.visitor_id
channel_section_event_id = channel_section.event_id
channel_section_etag = channel_section.etag
channel_section_items = channel_section.items
```

---


### Comment

Inserts a new resource into this collection.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "youtube#comment". |
| `etag` | String |  | Etag of this resource. |
| `snippet` | String |  | The snippet object contains basic details about the comment. |
| `id` | String |  | The ID that YouTube uses to uniquely identify the comment. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | Etag of this resource. |
| `items` | Vec<String> | A list of comments that match the request criteria. |
| `event_id` | String | Serialized EventId of the request which produced this response. |
| `visitor_id` | String | The visitorId identifies the visitor. |
| `next_page_token` | String | The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "youtube#commentListResponse". |
| `page_info` | String | General pagination information. |
| `token_pagination` | String |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create comment
comment = provider.youtube_api.Comment {
}

# Access comment outputs
comment_id = comment.id
comment_etag = comment.etag
comment_items = comment.items
comment_event_id = comment.event_id
comment_visitor_id = comment.visitor_id
comment_next_page_token = comment.next_page_token
comment_kind = comment.kind
comment_page_info = comment.page_info
comment_token_pagination = comment.token_pagination
```

---


### Third_party_link

Inserts a new resource into this collection.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `linking_token` | String |  | The linking_token identifies a YouTube account and channel with which the third party account is linked. |
| `status` | String |  | The status object contains information about the status of the link. |
| `etag` | String |  | Etag of this resource |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "youtube#thirdPartyLink". |
| `snippet` | String |  | The snippet object contains basic details about the third- party account link. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "youtube#thirdPartyLinkListResponse". |
| `etag` | String | Etag of this resource. |
| `items` | Vec<String> |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create third_party_link
third_party_link = provider.youtube_api.Third_party_link {
}

# Access third_party_link outputs
third_party_link_id = third_party_link.id
third_party_link_kind = third_party_link.kind
third_party_link_etag = third_party_link.etag
third_party_link_items = third_party_link.items
```

---


### Video_categorie

Retrieves a list of resources, possibly filtered.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `event_id` | String | Serialized EventId of the request which produced this response. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "youtube#videoCategoryListResponse". |
| `next_page_token` | String | The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set. |
| `prev_page_token` | String | The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set. |
| `items` | Vec<String> | A list of video categories that can be associated with YouTube videos. In this map, the video category ID is the map key, and its value is the corresponding videoCategory resource. |
| `visitor_id` | String | The visitorId identifies the visitor. |
| `page_info` | String | General pagination information. |
| `token_pagination` | String |  |
| `etag` | String | Etag of this resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access video_categorie outputs
video_categorie_id = video_categorie.id
video_categorie_event_id = video_categorie.event_id
video_categorie_kind = video_categorie.kind
video_categorie_next_page_token = video_categorie.next_page_token
video_categorie_prev_page_token = video_categorie.prev_page_token
video_categorie_items = video_categorie.items
video_categorie_visitor_id = video_categorie.visitor_id
video_categorie_page_info = video_categorie.page_info
video_categorie_token_pagination = video_categorie.token_pagination
video_categorie_etag = video_categorie.etag
```

---


### I18n_region

Retrieves a list of resources, possibly filtered.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "youtube#i18nRegionListResponse". |
| `items` | Vec<String> | A list of regions where YouTube is available. In this map, the i18n region ID is the map key, and its value is the corresponding i18nRegion resource. |
| `visitor_id` | String | The visitorId identifies the visitor. |
| `event_id` | String | Serialized EventId of the request which produced this response. |
| `etag` | String | Etag of this resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access i18n_region outputs
i18n_region_id = i18n_region.id
i18n_region_kind = i18n_region.kind
i18n_region_items = i18n_region.items
i18n_region_visitor_id = i18n_region.visitor_id
i18n_region_event_id = i18n_region.event_id
i18n_region_etag = i18n_region.etag
```

---


### I18n_language

Retrieves a list of resources, possibly filtered.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `visitor_id` | String | The visitorId identifies the visitor. |
| `items` | Vec<String> | A list of supported i18n languages. In this map, the i18n language ID is the map key, and its value is the corresponding i18nLanguage resource. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "youtube#i18nLanguageListResponse". |
| `event_id` | String | Serialized EventId of the request which produced this response. |
| `etag` | String | Etag of this resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access i18n_language outputs
i18n_language_id = i18n_language.id
i18n_language_visitor_id = i18n_language.visitor_id
i18n_language_items = i18n_language.items
i18n_language_kind = i18n_language.kind
i18n_language_event_id = i18n_language.event_id
i18n_language_etag = i18n_language.etag
```

---


### Memberships_level

Retrieves a list of all pricing levels offered by a creator to the fans.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `event_id` | String | Serialized EventId of the request which produced this response. |
| `etag` | String | Etag of this resource. |
| `visitor_id` | String | The visitorId identifies the visitor. |
| `items` | Vec<String> | A list of pricing levels offered by a creator to the fans. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "youtube#membershipsLevelListResponse". |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access memberships_level outputs
memberships_level_id = memberships_level.id
memberships_level_event_id = memberships_level.event_id
memberships_level_etag = memberships_level.etag
memberships_level_visitor_id = memberships_level.visitor_id
memberships_level_items = memberships_level.items
memberships_level_kind = memberships_level.kind
```

---


### Video

Inserts a new resource into this collection.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String |  | The ID that YouTube uses to uniquely identify the video. |
| `monetization_details` | String |  | The monetizationDetails object encapsulates information about the monetization status of the video. |
| `paid_product_placement_details` | String |  |  |
| `snippet` | String |  | The snippet object contains basic details about the video, such as its title, description, and category. |
| `recording_details` | String |  | The recordingDetails object encapsulates information about the location, date and address where the video was recorded. |
| `content_details` | String |  | The contentDetails object contains information about the video content, including the length of the video and its aspect ratio. |
| `etag` | String |  | Etag of this resource. |
| `live_streaming_details` | String |  | The liveStreamingDetails object contains metadata about a live video broadcast. The object will only be present in a video resource if the video is an upcoming, live, or completed live broadcast. |
| `statistics` | String |  | The statistics object contains statistics about the video. |
| `age_gating` | String |  | Age restriction details related to a video. This data can only be retrieved by the video owner. |
| `player` | String |  | The player object contains information that you would use to play the video in an embedded player. |
| `file_details` | String |  | The fileDetails object encapsulates information about the video file that was uploaded to YouTube, including the file's resolution, duration, audio and video codecs, stream bitrates, and more. This data can only be retrieved by the video owner. |
| `status` | String |  | The status object contains information about the video's uploading, processing, and privacy statuses. |
| `topic_details` | String |  | The topicDetails object encapsulates information about Freebase topics associated with the video. |
| `suggestions` | String |  | The suggestions object encapsulates suggestions that identify opportunities to improve the video quality or the metadata for the uploaded video. This data can only be retrieved by the video owner. |
| `localizations` | HashMap<String, String> |  | The localizations object contains localized versions of the basic details about the video, such as its title and description. |
| `processing_details` | String |  | The processingDetails object encapsulates information about YouTube's progress in processing the uploaded video file. The properties in the object identify the current processing status and an estimate of the time remaining until YouTube finishes processing the video. This part also indicates whether different types of data or content, such as file details or thumbnail images, are available for the video. The processingProgress object is designed to be polled so that the video uploaded can track the progress that YouTube has made in processing the uploaded video file. This data can only be retrieved by the video owner. |
| `project_details` | String |  | The projectDetails object contains information about the project specific video metadata. b/157517979: This part was never populated after it was added. However, it sees non-zero traffic because there is generated client code in the wild that refers to it [1]. We keep this field and do NOT remove it because otherwise V3 would return an error when this part gets requested [2]. [1] https://developers.google.com/resources/api-libraries/documentation/youtube/v3/csharp/latest/classGoogle_1_1Apis_1_1YouTube_1_1v3_1_1Data_1_1VideoProjectDetails.html [2] http://google3/video/youtube/src/python/servers/data_api/common.py?l=1565-1569&rcl=344141677 |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "youtube#video". |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `visitor_id` | String | The visitorId identifies the visitor. |
| `event_id` | String | Serialized EventId of the request which produced this response. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "youtube#videoGetRatingResponse". |
| `etag` | String | Etag of this resource. |
| `items` | Vec<String> | A list of ratings that match the request criteria. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create video
video = provider.youtube_api.Video {
}

# Access video outputs
video_id = video.id
video_visitor_id = video.visitor_id
video_event_id = video.event_id
video_kind = video.kind
video_etag = video.etag
video_items = video.items
```

---


### Channel_banner

Inserts a new resource into this collection.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "youtube#channelBannerResource". |
| `etag` | String |  |  |
| `url` | String |  | The URL of this banner image. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create channel_banner
channel_banner = provider.youtube_api.Channel_banner {
}

```

---


### Test

POST method.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `gaia` | String |  |  |
| `featured_part` | bool |  |  |
| `snippet` | String |  |  |
| `id` | String |  |  |
| `etag` | String |  | Etag for the resource. See https://en.wikipedia.org/wiki/HTTP_ETag. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create test
test = provider.youtube_api.Test {
}

```

---


### Playlist_item

Inserts a new resource into this collection.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `content_details` | String |  | The contentDetails object is included in the resource if the included item is a YouTube video. The object contains additional information about the video. |
| `snippet` | String |  | The snippet object contains basic details about the playlist item, such as its title and position in the playlist. |
| `status` | String |  | The status object contains information about the playlist item's privacy status. |
| `id` | String |  | The ID that YouTube uses to uniquely identify the playlist item. |
| `etag` | String |  | Etag of this resource. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "youtube#playlistItem". |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set. |
| `token_pagination` | String |  |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "youtube#playlistItemListResponse". |
| `items` | Vec<String> | A list of playlist items that match the request criteria. |
| `visitor_id` | String | The visitorId identifies the visitor. |
| `event_id` | String | Serialized EventId of the request which produced this response. |
| `page_info` | String | General pagination information. |
| `prev_page_token` | String | The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set. |
| `etag` | String |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create playlist_item
playlist_item = provider.youtube_api.Playlist_item {
}

# Access playlist_item outputs
playlist_item_id = playlist_item.id
playlist_item_next_page_token = playlist_item.next_page_token
playlist_item_token_pagination = playlist_item.token_pagination
playlist_item_kind = playlist_item.kind
playlist_item_items = playlist_item.items
playlist_item_visitor_id = playlist_item.visitor_id
playlist_item_event_id = playlist_item.event_id
playlist_item_page_info = playlist_item.page_info
playlist_item_prev_page_token = playlist_item.prev_page_token
playlist_item_etag = playlist_item.etag
```

---


### Comment_thread

Inserts a new resource into this collection.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `snippet` | String |  | The snippet object contains basic details about the comment thread and also the top level comment. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "youtube#commentThread". |
| `replies` | String |  | The replies object contains a limited number of replies (if any) to the top level comment found in the snippet. |
| `etag` | String |  | Etag of this resource. |
| `id` | String |  | The ID that YouTube uses to uniquely identify the comment thread. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | Etag of this resource. |
| `token_pagination` | String |  |
| `items` | Vec<String> | A list of comment threads that match the request criteria. |
| `page_info` | String | General pagination information. |
| `visitor_id` | String | The visitorId identifies the visitor. |
| `event_id` | String | Serialized EventId of the request which produced this response. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "youtube#commentThreadListResponse". |
| `next_page_token` | String | The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create comment_thread
comment_thread = provider.youtube_api.Comment_thread {
}

# Access comment_thread outputs
comment_thread_id = comment_thread.id
comment_thread_etag = comment_thread.etag
comment_thread_token_pagination = comment_thread.token_pagination
comment_thread_items = comment_thread.items
comment_thread_page_info = comment_thread.page_info
comment_thread_visitor_id = comment_thread.visitor_id
comment_thread_event_id = comment_thread.event_id
comment_thread_kind = comment_thread.kind
comment_thread_next_page_token = comment_thread.next_page_token
```

---


### Live_broadcast

Inserts a new stream for the authenticated user.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String |  | The ID that YouTube assigns to uniquely identify the broadcast. |
| `monetization_details` | String |  | The monetizationDetails object contains information about the event's monetization details. |
| `content_details` | String |  | The contentDetails object contains information about the event's video content, such as whether the content can be shown in an embedded video player or if it will be archived and therefore available for viewing after the event has concluded. |
| `statistics` | String |  | The statistics object contains info about the event's current stats. These include concurrent viewers and total chat count. Statistics can change (in either direction) during the lifetime of an event. Statistics are only returned while the event is live. |
| `snippet` | String |  | The snippet object contains basic details about the event, including its title, description, start time, and end time. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string "youtube#liveBroadcast". |
| `etag` | String |  | Etag of this resource. |
| `status` | String |  | The status object contains information about the event's status. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `event_id` | String | Serialized EventId of the request which produced this response. |
| `items` | Vec<String> | A list of broadcasts that match the request criteria. |
| `next_page_token` | String | The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "youtube#liveBroadcastListResponse". |
| `token_pagination` | String |  |
| `page_info` | String | General pagination information. |
| `etag` | String | Etag of this resource. |
| `prev_page_token` | String | The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set. |
| `visitor_id` | String | The visitorId identifies the visitor. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create live_broadcast
live_broadcast = provider.youtube_api.Live_broadcast {
}

# Access live_broadcast outputs
live_broadcast_id = live_broadcast.id
live_broadcast_event_id = live_broadcast.event_id
live_broadcast_items = live_broadcast.items
live_broadcast_next_page_token = live_broadcast.next_page_token
live_broadcast_kind = live_broadcast.kind
live_broadcast_token_pagination = live_broadcast.token_pagination
live_broadcast_page_info = live_broadcast.page_info
live_broadcast_etag = live_broadcast.etag
live_broadcast_prev_page_token = live_broadcast.prev_page_token
live_broadcast_visitor_id = live_broadcast.visitor_id
```

---


### Live_chat_ban

Inserts a new resource into this collection.

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `etag` | String |  | Etag of this resource. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string `"youtube#liveChatBan"`. |
| `snippet` | String |  | The `snippet` object contains basic details about the ban. |
| `id` | String |  | The ID that YouTube assigns to uniquely identify the ban. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create live_chat_ban
live_chat_ban = provider.youtube_api.Live_chat_ban {
}

```

---


### Message

Allows a user to load live chat through a server-streamed RPC.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `event_id` | String | Serialized EventId of the request which produced this response. |
| `etag` | String | Etag of this resource. |
| `page_info` | String | General pagination information. |
| `visitor_id` | String | The visitorId identifies the visitor. |
| `offline_at` | String | The date and time when the underlying stream went offline. |
| `items` | Vec<String> |  |
| `active_poll_item` | String | Set when there is an active poll. |
| `token_pagination` | String |  |
| `polling_interval_millis` | i64 | The amount of time the client should wait before polling again. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string "youtube#liveChatMessageListResponse". |
| `next_page_token` | String |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access message outputs
message_id = message.id
message_event_id = message.event_id
message_etag = message.etag
message_page_info = message.page_info
message_visitor_id = message.visitor_id
message_offline_at = message.offline_at
message_items = message.items
message_active_poll_item = message.active_poll_item
message_token_pagination = message.token_pagination
message_polling_interval_millis = message.polling_interval_millis
message_kind = message.kind
message_next_page_token = message.next_page_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple playlist_image resources
playlist_image_0 = provider.youtube_api.Playlist_image {
}
playlist_image_1 = provider.youtube_api.Playlist_image {
}
playlist_image_2 = provider.youtube_api.Playlist_image {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    playlist_image = provider.youtube_api.Playlist_image {
    }
```

---

## Related Documentation

- [GCP Youtube_api Documentation](https://cloud.google.com/youtube_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
