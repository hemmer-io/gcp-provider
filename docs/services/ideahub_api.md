# Ideahub_api Service



**Resources**: 10

---

## Overview

The ideahub_api service provides access to 10 resource types:

- [Topic_state](#topic_state) [U]
- [Idea_activitie](#idea_activitie) [C]
- [Idea_state](#idea_state) [U]
- [Idea](#idea) [R]
- [Locale](#locale) [R]
- [Idea_activitie](#idea_activitie) [C]
- [Idea](#idea) [R]
- [Locale](#locale) [R]
- [Idea_state](#idea_state) [U]
- [Topic_state](#topic_state) [U]

---

## Resources


### Topic_state



**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dismissed` | bool |  | Whether the topic is dismissed. |
| `name` | String |  | Unique identifier for the topic state. Format: platforms/{platform}/properties/{property}/topicStates/{topic_state} |
| `saved` | bool |  | Whether the topic is saved. |
| `name` | String | ✅ | Unique identifier for the topic state. Format: platforms/{platform}/properties/{property}/topicStates/{topic_state} |



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


### Idea_activitie

Creates an idea activity entry.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `topics` | Vec<String> |  | The Topic IDs for this entry. If empty, ideas should be set. |
| `ideas` | Vec<String> |  | The Idea IDs for this entry. If empty, topics should be set. |
| `type` | String |  | The type of activity performed. |
| `uri` | String |  | The uri the activity relates to. |
| `name` | String |  | Unique identifier for the idea activity. The name is ignored when creating an idea activity. Format: platforms/{platform}/properties/{property}/ideaActivities/{idea_activity} |
| `parent` | String | ✅ | Required. The parent resource where this idea activity will be created. Format: platforms/{platform}/property/{property} |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create idea_activitie
idea_activitie = provider.ideahub_api.Idea_activitie {
    parent = "value"  # Required. The parent resource where this idea activity will be created. Format: platforms/{platform}/property/{property}
}

```

---


### Idea_state



**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Unique identifier for the idea state. Format: platforms/{platform}/properties/{property}/ideaStates/{idea_state} |
| `saved` | bool |  | Whether the idea is saved. |
| `dismissed` | bool |  | Whether the idea is dismissed. |
| `name` | String | ✅ | Unique identifier for the idea state. Format: platforms/{platform}/properties/{property}/ideaStates/{idea_state} |



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


### Idea

List ideas for a given Creator and filter and sort options.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ideas` | Vec<String> | Results for the ListIdeasRequest. |
| `next_page_token` | String | Used to fetch the next page in a subsequent request. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access idea outputs
idea_id = idea.id
idea_ideas = idea.ideas
idea_next_page_token = idea.next_page_token
```

---


### Locale

Returns which locales ideas are available in for a given Creator.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `available_locales` | Vec<String> | Locales for which ideas are available for the given Creator. |
| `next_page_token` | String | A token that can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access locale outputs
locale_id = locale.id
locale_available_locales = locale.available_locales
locale_next_page_token = locale.next_page_token
```

---


### Idea_activitie

Creates an idea activity entry.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `topics` | Vec<String> |  | The Topic IDs for this entry. If empty, ideas should be set. |
| `uri` | String |  | The uri the activity relates to. |
| `ideas` | Vec<String> |  | The Idea IDs for this entry. If empty, topics should be set. |
| `name` | String |  | Unique identifier for the idea activity. The name is ignored when creating an idea activity. Format: platforms/{platform}/properties/{property}/ideaActivities/{idea_activity} |
| `type` | String |  | The type of activity performed. |
| `parent` | String | ✅ | Required. The parent resource where this idea activity will be created. Format: platforms/{platform}/property/{property} |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create idea_activitie
idea_activitie = provider.ideahub_api.Idea_activitie {
    parent = "value"  # Required. The parent resource where this idea activity will be created. Format: platforms/{platform}/property/{property}
}

```

---


### Idea

List ideas for a given Creator and filter and sort options.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ideas` | Vec<String> | Results for the ListIdeasRequest. |
| `next_page_token` | String | Used to fetch the next page in a subsequent request. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access idea outputs
idea_id = idea.id
idea_ideas = idea.ideas
idea_next_page_token = idea.next_page_token
```

---


### Locale

Returns which locales ideas are available in for a given Creator.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `available_locales` | Vec<String> | Locales for which ideas are available for the given Creator. |
| `next_page_token` | String | A token that can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access locale outputs
locale_id = locale.id
locale_available_locales = locale.available_locales
locale_next_page_token = locale.next_page_token
```

---


### Idea_state



**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `dismissed` | bool |  | Whether the idea is dismissed. |
| `name` | String |  | Unique identifier for the idea state. Format: platforms/{platform}/properties/{property}/ideaStates/{idea_state} |
| `saved` | bool |  | Whether the idea is saved. |
| `name` | String | ✅ | Unique identifier for the idea state. Format: platforms/{platform}/properties/{property}/ideaStates/{idea_state} |



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


### Topic_state



**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `saved` | bool |  | Whether the topic is saved. |
| `dismissed` | bool |  | Whether the topic is dismissed. |
| `name` | String |  | Unique identifier for the topic state. Format: platforms/{platform}/properties/{property}/topicStates/{topic_state} |
| `name` | String | ✅ | Unique identifier for the topic state. Format: platforms/{platform}/properties/{property}/topicStates/{topic_state} |



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



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple topic_state resources
topic_state_0 = provider.ideahub_api.Topic_state {
    name = "value-0"
}
topic_state_1 = provider.ideahub_api.Topic_state {
    name = "value-1"
}
topic_state_2 = provider.ideahub_api.Topic_state {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    topic_state = provider.ideahub_api.Topic_state {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Ideahub_api Documentation](https://cloud.google.com/ideahub_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
