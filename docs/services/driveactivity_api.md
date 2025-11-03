# Driveactivity_api Service



**Resources**: 1

---

## Overview

The driveactivity_api service provides access to 1 resource type:

- [Activity](#activity) [C]

---

## Resources


### Activity

Query past activity in Google Drive.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `filter` | String |  | The filtering for items returned from this query request. The format of the filter string is a sequence of expressions, joined by an optional "AND", where each expression is of the form "field operator value". Supported fields: - `time`: Uses numerical operators on date values either in terms of milliseconds since Jan 1, 1970 or in RFC 3339 format. Examples: - `time > 1452409200000 AND time <= 1492812924310` - `time >= "2016-01-10T01:02:03-05:00"` - `detail.action_detail_case`: Uses the "has" operator (:) and either a singular value or a list of allowed action types enclosed in parentheses, separated by a space. To exclude a result from the response, prepend a hyphen (`-`) to the beginning of the filter string. Examples: - `detail.action_detail_case:RENAME` - `detail.action_detail_case:(CREATE RESTORE)` - `-detail.action_detail_case:MOVE`  |
| `ancestor_name` | String |  | Return activities for this Drive folder, plus all children and descendants. The format is `items/ITEM_ID`. |
| `page_token` | String |  | The token identifies which page of results to return. Set this to the next_page_token value returned from a previous query to obtain the following page of results. If not set, the first page of results is returned. |
| `item_name` | String |  | Return activities for this Drive item. The format is `items/ITEM_ID`. |
| `consolidation_strategy` | String |  | Details on how to consolidate related actions that make up the activity. If not set, then related actions aren't consolidated. |
| `page_size` | i64 |  | The minimum number of activities desired in the response; the server attempts to return at least this quantity. The server may also return fewer activities if it has a partial response ready before the request times out. If not set, a default value is used. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create activity
activity = provider.driveactivity_api.Activity {
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

# Create multiple activity resources
activity_0 = provider.driveactivity_api.Activity {
}
activity_1 = provider.driveactivity_api.Activity {
}
activity_2 = provider.driveactivity_api.Activity {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    activity = provider.driveactivity_api.Activity {
    }
```

---

## Related Documentation

- [GCP Driveactivity_api Documentation](https://cloud.google.com/driveactivity_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
