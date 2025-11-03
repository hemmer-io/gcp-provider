# Gamesconfiguration_api Service



**Resources**: 2

---

## Overview

The gamesconfiguration_api service provides access to 2 resource types:

- [Leaderboard_configuration](#leaderboard_configuration) [CRUD]
- [Achievement_configuration](#achievement_configuration) [CRUD]

---

## Resources


### Leaderboard_configuration

Insert a new leaderboard configuration in this application.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String |  | The ID of the leaderboard. |
| `draft` | String |  | The draft data of the leaderboard. |
| `score_order` | String |  |  |
| `kind` | String |  | Uniquely identifies the type of this resource. Value is always the fixed string `gamesConfiguration#leaderboardConfiguration`. |
| `token` | String |  | The token for this resource. |
| `published` | String |  | The read-only published data of the leaderboard. |
| `score_max` | String |  | Maximum score that can be posted to this leaderboard. |
| `score_min` | String |  | Minimum score that can be posted to this leaderboard. |
| `application_id` | String | ✅ | The application ID from the Google Play developer console. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | The ID of the leaderboard. |
| `draft` | String | The draft data of the leaderboard. |
| `score_order` | String |  |
| `kind` | String | Uniquely identifies the type of this resource. Value is always the fixed string `gamesConfiguration#leaderboardConfiguration`. |
| `token` | String | The token for this resource. |
| `published` | String | The read-only published data of the leaderboard. |
| `score_max` | String | Maximum score that can be posted to this leaderboard. |
| `score_min` | String | Minimum score that can be posted to this leaderboard. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create leaderboard_configuration
leaderboard_configuration = provider.gamesconfiguration_api.Leaderboard_configuration {
    application_id = "value"  # The application ID from the Google Play developer console.
}

# Access leaderboard_configuration outputs
leaderboard_configuration_id = leaderboard_configuration.id
leaderboard_configuration_id = leaderboard_configuration.id
leaderboard_configuration_draft = leaderboard_configuration.draft
leaderboard_configuration_score_order = leaderboard_configuration.score_order
leaderboard_configuration_kind = leaderboard_configuration.kind
leaderboard_configuration_token = leaderboard_configuration.token
leaderboard_configuration_published = leaderboard_configuration.published
leaderboard_configuration_score_max = leaderboard_configuration.score_max
leaderboard_configuration_score_min = leaderboard_configuration.score_min
```

---


### Achievement_configuration

Insert a new achievement configuration in this application.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `initial_state` | String |  | The initial state of the achievement. |
| `published` | String |  | The read-only published data of the achievement. |
| `steps_to_unlock` | i64 |  | Steps to unlock. Only applicable to incremental achievements. |
| `draft` | String |  | The draft data of the achievement. |
| `achievement_type` | String |  | The type of the achievement. |
| `id` | String |  | The ID of the achievement. |
| `kind` | String |  | Uniquely identifies the type of this resource. Value is always the fixed string `gamesConfiguration#achievementConfiguration`. |
| `token` | String |  | The token for this resource. |
| `application_id` | String | ✅ | The application ID from the Google Play developer console. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `initial_state` | String | The initial state of the achievement. |
| `published` | String | The read-only published data of the achievement. |
| `steps_to_unlock` | i64 | Steps to unlock. Only applicable to incremental achievements. |
| `draft` | String | The draft data of the achievement. |
| `achievement_type` | String | The type of the achievement. |
| `id` | String | The ID of the achievement. |
| `kind` | String | Uniquely identifies the type of this resource. Value is always the fixed string `gamesConfiguration#achievementConfiguration`. |
| `token` | String | The token for this resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create achievement_configuration
achievement_configuration = provider.gamesconfiguration_api.Achievement_configuration {
    application_id = "value"  # The application ID from the Google Play developer console.
}

# Access achievement_configuration outputs
achievement_configuration_id = achievement_configuration.id
achievement_configuration_initial_state = achievement_configuration.initial_state
achievement_configuration_published = achievement_configuration.published
achievement_configuration_steps_to_unlock = achievement_configuration.steps_to_unlock
achievement_configuration_draft = achievement_configuration.draft
achievement_configuration_achievement_type = achievement_configuration.achievement_type
achievement_configuration_id = achievement_configuration.id
achievement_configuration_kind = achievement_configuration.kind
achievement_configuration_token = achievement_configuration.token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple leaderboard_configuration resources
leaderboard_configuration_0 = provider.gamesconfiguration_api.Leaderboard_configuration {
    application_id = "value-0"
}
leaderboard_configuration_1 = provider.gamesconfiguration_api.Leaderboard_configuration {
    application_id = "value-1"
}
leaderboard_configuration_2 = provider.gamesconfiguration_api.Leaderboard_configuration {
    application_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    leaderboard_configuration = provider.gamesconfiguration_api.Leaderboard_configuration {
        application_id = "production-value"
    }
```

---

## Related Documentation

- [GCP Gamesconfiguration_api Documentation](https://cloud.google.com/gamesconfiguration_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
