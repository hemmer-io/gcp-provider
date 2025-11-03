# Appsactivity_api Service



**Resources**: 1

---

## Overview

The appsactivity_api service provides access to 1 resource type:

- [Activitie](#activitie) [R]

---

## Resources


### Activitie

Returns a list of activities visible to the current logged in user. Visible activities are determined by the visibility settings of the object that was acted on, e.g. Drive files a user can see. An activity is a record of past events. Multiple events may be merged if they are similar. A request is scoped to activities from a given Google service using the source parameter.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Token for the next page of results. |
| `activities` | Vec<String> | List of activities. |


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
activitie_next_page_token = activitie.next_page_token
activitie_activities = activitie.activities
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple activitie resources
activitie_0 = provider.appsactivity_api.Activitie {
}
activitie_1 = provider.appsactivity_api.Activitie {
}
activitie_2 = provider.appsactivity_api.Activitie {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    activitie = provider.appsactivity_api.Activitie {
    }
```

---

## Related Documentation

- [GCP Appsactivity_api Documentation](https://cloud.google.com/appsactivity_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
