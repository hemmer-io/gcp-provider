# Games_api Service



**Resources**: 13

---

## Overview

The games_api service provides access to 13 resource types:

- [Revision](#revision) [R]
- [Recall](#recall) [CR]
- [Application](#application) [CR]
- [Accesstoken](#accesstoken) [C]
- [Score](#score) [CR]
- [Leaderboard](#leaderboard) [R]
- [Metagame](#metagame) [R]
- [Snapshot](#snapshot) [R]
- [Player](#player) [R]
- [Achievement_definition](#achievement_definition) [R]
- [Event](#event) [CR]
- [Stat](#stat) [R]
- [Achievement](#achievement) [CR]

---

## Resources


### Revision

Checks whether the games client is out of date.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `api_version` | String | The version of the API this client revision should use when calling API methods. |
| `revision_status` | String | The result of the revision check. |
| `kind` | String | Uniquely identifies the type of this resource. Value is always the fixed string `games#revisionCheckResponse`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access revision outputs
revision_id = revision.id
revision_api_version = revision.api_version
revision_revision_status = revision.revision_status
revision_kind = revision.kind
```

---


### Recall

Associate the PGS Player principal encoded in the provided recall session id with an in-game account

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `session_id` | String |  | Required. Opaque server-generated string that encodes all the necessary information to identify the PGS player / Google user and application. |
| `persona` | String |  | Required. Stable identifier of the in-game account. Please refrain from re-using the same persona for different games. |
| `cardinality_constraint` | String |  | Required. Cardinality constraint to observe when linking a persona to a player in the scope of a game. |
| `token` | String |  | Required. Value of the token to create. Opaque to Play Games and assumed to be non-stable (encrypted with key rotation). |
| `conflicting_links_resolution_policy` | String |  | Required. Resolution policy to apply when the linking of a persona to a player would result in violating the specified cardinality constraint. |
| `ttl` | String |  | Input only. Optional time-to-live. |
| `expire_time` | String |  | Input only. Optional expiration time. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `game_player_tokens` | Vec<String> | The requested applications along with the recall tokens for the player. If the player does not have recall tokens for an application, that application is not included in the response. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create recall
recall = provider.games_api.Recall {
}

# Access recall outputs
recall_id = recall.id
recall_game_player_tokens = recall.game_player_tokens
```

---


### Application

Returns a URL for the requested end point type.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `theme_color` | String | A hint to the client UI for what color to use as an app-themed color. The color is given as an RGB triplet (e.g. "E0E0E0"). |
| `assets` | Vec<String> | The assets of the application. |
| `leaderboard_count` | i64 | The number of leaderboards visible to the currently authenticated player. |
| `achievement_count` | i64 | The number of achievements visible to the currently authenticated player. |
| `description` | String | The description of the application. |
| `id` | String | The ID of the application. |
| `instances` | Vec<String> | The instances of the application. |
| `kind` | String | Uniquely identifies the type of this resource. Value is always the fixed string `games#application`. |
| `enabled_features` | Vec<String> | A list of features that have been enabled for the application. |
| `author` | String | The author of the application. |
| `category` | String | The category of the application. |
| `last_updated_timestamp` | String | The last updated timestamp of the application. |
| `name` | String | The name of the application. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create application
application = provider.games_api.Application {
}

# Access application outputs
application_id = application.id
application_theme_color = application.theme_color
application_assets = application.assets
application_leaderboard_count = application.leaderboard_count
application_achievement_count = application.achievement_count
application_description = application.description
application_id = application.id
application_instances = application.instances
application_kind = application.kind
application_enabled_features = application.enabled_features
application_author = application.author
application_category = application.category
application_last_updated_timestamp = application.last_updated_timestamp
application_name = application.name
```

---


### Accesstoken

Generates a Play Grouping API token for the PGS user identified by the Recall session ID provided in the request.

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

# Create accesstoken
accesstoken = provider.games_api.Accesstoken {
}

```

---


### Score

Submits multiple scores to leaderboards.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `scores` | Vec<String> |  | The score submissions. |
| `kind` | String |  | Uniquely identifies the type of this resource. Value is always the fixed string `games#playerScoreSubmissionList`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Uniquely identifies the type of this resource. Value is always the fixed string `games#playerLeaderboardScoreListResponse`. |
| `next_page_token` | String | The pagination token for the next page of results. |
| `items` | Vec<String> | The leaderboard scores. |
| `player` | String | The Player resources for the owner of this score. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create score
score = provider.games_api.Score {
}

# Access score outputs
score_id = score.id
score_kind = score.kind
score_next_page_token = score.next_page_token
score_items = score.items
score_player = score.player
```

---


### Leaderboard

Retrieves the metadata of the leaderboard with the given ID.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `is_icon_url_default` | bool | Indicates whether the icon image being returned is a default image, or is game-provided. |
| `kind` | String | Uniquely identifies the type of this resource. Value is always the fixed string `games#leaderboard`. |
| `name` | String | The name of the leaderboard. |
| `order` | String | How scores are ordered. |
| `id` | String | The leaderboard ID. |
| `icon_url` | String | The icon for the leaderboard. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access leaderboard outputs
leaderboard_id = leaderboard.id
leaderboard_is_icon_url_default = leaderboard.is_icon_url_default
leaderboard_kind = leaderboard.kind
leaderboard_name = leaderboard.name
leaderboard_order = leaderboard.order
leaderboard_id = leaderboard.id
leaderboard_icon_url = leaderboard.icon_url
```

---


### Metagame

List play data aggregated per category for the player corresponding to `playerId`.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Uniquely identifies the type of this resource. Value is always the fixed string `games#categoryListResponse`. |
| `next_page_token` | String | Token corresponding to the next page of results. |
| `items` | Vec<String> | The list of categories with usage data. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access metagame outputs
metagame_id = metagame.id
metagame_kind = metagame.kind
metagame_next_page_token = metagame.next_page_token
metagame_items = metagame.items
```

---


### Snapshot

Retrieves the metadata for a given snapshot ID.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `duration_millis` | String | The duration associated with this snapshot, in millis. |
| `unique_name` | String | The unique name provided when the snapshot was created. |
| `description` | String | The description of this snapshot. |
| `progress_value` | String | The progress value (64-bit integer set by developer) associated with this snapshot. |
| `type` | String | The type of this snapshot. |
| `cover_image` | String | The cover image of this snapshot. May be absent if there is no image. |
| `kind` | String | Uniquely identifies the type of this resource. Value is always the fixed string `games#snapshot`. |
| `title` | String | The title of this snapshot. |
| `id` | String | The ID of the snapshot. |
| `last_modified_millis` | String | The timestamp (in millis since Unix epoch) of the last modification to this snapshot. |
| `drive_id` | String | The ID of the file underlying this snapshot in the Drive API. Only present if the snapshot is a view on a Drive file and the file is owned by the caller. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access snapshot outputs
snapshot_id = snapshot.id
snapshot_duration_millis = snapshot.duration_millis
snapshot_unique_name = snapshot.unique_name
snapshot_description = snapshot.description
snapshot_progress_value = snapshot.progress_value
snapshot_type = snapshot.type
snapshot_cover_image = snapshot.cover_image
snapshot_kind = snapshot.kind
snapshot_title = snapshot.title
snapshot_id = snapshot.id
snapshot_last_modified_millis = snapshot.last_modified_millis
snapshot_drive_id = snapshot.drive_id
```

---


### Player

Retrieves the Player resource with the given ID. To retrieve the player for the currently authenticated user, set `playerId` to `me`.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `title` | String | The player's title rewarded for their game activities. |
| `player_id` | String | The ID of the player. |
| `kind` | String | Uniquely identifies the type of this resource. Value is always the fixed string `games#player` |
| `banner_url_portrait` | String | The url to the portrait mode player banner image. |
| `name` | String | A representation of the individual components of the name. |
| `original_player_id` | String | The player ID that was used for this player the first time they signed into the game in question. This is only populated for calls to player.get for the requesting player, only if the player ID has subsequently changed, and only to clients that support remapping player IDs. |
| `banner_url_landscape` | String | The url to the landscape mode player banner image. |
| `experience_info` | String | An object to represent Play Game experience information for the player. |
| `avatar_image_url` | String | The base URL for the image that represents the player. |
| `friend_status` | String | The friend status of the given player, relative to the requester. This is unset if the player is not sharing their friends list with the game. |
| `game_player_id` | String | Per-application unique player identifier. |
| `profile_settings` | String | The player's profile settings. Controls whether or not the player's profile is visible to other players. |
| `display_name` | String | The name to display for the player. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access player outputs
player_id = player.id
player_title = player.title
player_player_id = player.player_id
player_kind = player.kind
player_banner_url_portrait = player.banner_url_portrait
player_name = player.name
player_original_player_id = player.original_player_id
player_banner_url_landscape = player.banner_url_landscape
player_experience_info = player.experience_info
player_avatar_image_url = player.avatar_image_url
player_friend_status = player.friend_status
player_game_player_id = player.game_player_id
player_profile_settings = player.profile_settings
player_display_name = player.display_name
```

---


### Achievement_definition

Lists all the achievement definitions for your application.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | The achievement definitions. |
| `kind` | String | Uniquely identifies the type of this resource. Value is always the fixed string `games#achievementDefinitionsListResponse`. |
| `next_page_token` | String | Token corresponding to the next page of results. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access achievement_definition outputs
achievement_definition_id = achievement_definition.id
achievement_definition_items = achievement_definition.items
achievement_definition_kind = achievement_definition.kind
achievement_definition_next_page_token = achievement_definition.next_page_token
```

---


### Event

Records a batch of changes to the number of times events have occurred for the currently authenticated user of this application.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `time_periods` | Vec<String> |  | A list of the time period updates being made in this request. |
| `kind` | String |  | Uniquely identifies the type of this resource. Value is always the fixed string `games#eventRecordRequest`. |
| `current_time_millis` | String |  | The current time when this update was sent, in milliseconds, since 1970 UTC (Unix Epoch). |
| `request_id` | String |  | The request ID used to identify this attempt to record events. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | The player events. |
| `next_page_token` | String | The pagination token for the next page of results. |
| `kind` | String | Uniquely identifies the type of this resource. Value is always the fixed string `games#playerEventListResponse`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create event
event = provider.games_api.Event {
}

# Access event outputs
event_id = event.id
event_items = event.items
event_next_page_token = event.next_page_token
event_kind = event.kind
```

---


### Stat

Returns engagement and spend statistics in this application for the currently authenticated user.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `num_sessions` | i64 | The approximate number of sessions of the player within the last 28 days, where a session begins when the player is connected to Play Games Services and ends when they are disconnected. E.g., 0, 1, 5, 10, ... . Not populated if there is not enough information. |
| `num_sessions_percentile` | f64 | The approximation of the sessions percentile of the player within the last 30 days, where a session begins when the player is connected to Play Games Services and ends when they are disconnected. E.g., 0, 0.25, 0.5, 0.75. Not populated if there is not enough information. |
| `num_purchases` | i64 | Number of in-app purchases made by the player in this game. E.g., 0, 1, 5, 10, ... . Not populated if there is not enough information. |
| `high_spender_probability` | f64 | The probability of the player going to spend beyond a threshold amount of money. E.g., 0, 0.25, 0.50, 0.75. Not populated if there is not enough information. |
| `kind` | String | Uniquely identifies the type of this resource. Value is always the fixed string `games#statsResponse`. |
| `churn_probability` | f64 | The probability of the player not returning to play the game in the next day. E.g., 0, 0.1, 0.5, ..., 1.0. Not populated if there is not enough information. |
| `days_since_last_played` | i64 | Number of days since the player last played this game. E.g., 0, 1, 5, 10, ... . Not populated if there is not enough information. |
| `avg_session_length_minutes` | f64 | Average session length in minutes of the player. E.g., 1, 30, 60, ... . Not populated if there is not enough information. |
| `spend_probability` | f64 | The probability of the player going to spend the game in the next seven days. E.g., 0, 0.25, 0.50, 0.75. Not populated if there is not enough information. |
| `total_spend_next_28_days` | f64 | The predicted amount of money that the player going to spend in the next 28 days. E.g., 1, 30, 60, ... . Not populated if there is not enough information. |
| `spend_percentile` | f64 | The approximate spend percentile of the player in this game. E.g., 0, 0.25, 0.5, 0.75. Not populated if there is not enough information. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access stat outputs
stat_id = stat.id
stat_num_sessions = stat.num_sessions
stat_num_sessions_percentile = stat.num_sessions_percentile
stat_num_purchases = stat.num_purchases
stat_high_spender_probability = stat.high_spender_probability
stat_kind = stat.kind
stat_churn_probability = stat.churn_probability
stat_days_since_last_played = stat.days_since_last_played
stat_avg_session_length_minutes = stat.avg_session_length_minutes
stat_spend_probability = stat.spend_probability
stat_total_spend_next_28_days = stat.total_spend_next_28_days
stat_spend_percentile = stat.spend_percentile
```

---


### Achievement

Unlocks this achievement for the currently authenticated player.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `achievement_id` | String | ✅ | The ID of the achievement used by this method. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Uniquely identifies the type of this resource. Value is always the fixed string `games#playerAchievementListResponse`. |
| `items` | Vec<String> | The achievements. |
| `next_page_token` | String | Token corresponding to the next page of results. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create achievement
achievement = provider.games_api.Achievement {
    achievement_id = "value"  # The ID of the achievement used by this method.
}

# Access achievement outputs
achievement_id = achievement.id
achievement_kind = achievement.kind
achievement_items = achievement.items
achievement_next_page_token = achievement.next_page_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple revision resources
revision_0 = provider.games_api.Revision {
}
revision_1 = provider.games_api.Revision {
}
revision_2 = provider.games_api.Revision {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    revision = provider.games_api.Revision {
    }
```

---

## Related Documentation

- [GCP Games_api Documentation](https://cloud.google.com/games_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
