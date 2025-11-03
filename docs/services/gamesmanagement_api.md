# Gamesmanagement_api Service



**Resources**: 5

---

## Overview

The gamesmanagement_api service provides access to 5 resource types:

- [Application](#application) [R]
- [Player](#player) [CD]
- [Score](#score) [C]
- [Event](#event) [C]
- [Achievement](#achievement) [C]

---

## Resources


### Application

Get the list of players hidden from the given application. This method is only available to user accounts for your developer console.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | The players. |
| `kind` | String | Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#hiddenPlayerList`. |
| `next_page_token` | String | The pagination token for the next page of results. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access application outputs
application_id = application.id
application_items = application.items
application_kind = application.kind
application_next_page_token = application.next_page_token
```

---


### Player

Hide the given player's leaderboard scores from the given application. This method is only available to user accounts for your developer console.

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `application_id` | String | ✅ | The application ID from the Google Play developer console. |
| `player_id` | String | ✅ | A player ID. A value of `me` may be used in place of the authenticated player's ID. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create player
player = provider.gamesmanagement_api.Player {
    application_id = "value"  # The application ID from the Google Play developer console.
    player_id = "value"  # A player ID. A value of `me` may be used in place of the authenticated player's ID.
}

```

---


### Score

Resets scores for all draft leaderboards for all players. This method is only available to user accounts for your developer console.

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

# Create score
score = provider.gamesmanagement_api.Score {
}

```

---


### Event

Resets events with the given IDs for all players. This method is only available to user accounts for your developer console. Only draft events may be reset.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `event_ids` | Vec<String> |  | The IDs of events to reset. |
| `kind` | String |  | Uniquely identifies the type of this resource. Value is always the fixed string `gamesManagement#eventsResetMultipleForAllRequest`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create event
event = provider.gamesmanagement_api.Event {
}

```

---


### Achievement

Resets all achievements for the currently authenticated player for your application. This method is only accessible to whitelisted tester accounts for your application.

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

# Create achievement
achievement = provider.gamesmanagement_api.Achievement {
}

```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple application resources
application_0 = provider.gamesmanagement_api.Application {
}
application_1 = provider.gamesmanagement_api.Application {
}
application_2 = provider.gamesmanagement_api.Application {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    application = provider.gamesmanagement_api.Application {
    }
```

---

## Related Documentation

- [GCP Gamesmanagement_api Documentation](https://cloud.google.com/gamesmanagement_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
