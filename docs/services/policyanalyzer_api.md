# Policyanalyzer_api Service



**Resources**: 2

---

## Overview

The policyanalyzer_api service provides access to 2 resource types:

- [Activitie](#activitie) [R]
- [Activitie](#activitie) [R]

---

## Resources


### Activitie

Queries policy activities on Google Cloud resources.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | If there might be more results than those appearing in this response, then `nextPageToken` is included. To get the next set of results, call this method again using the value of `nextPageToken` as `pageToken`. |
| `activities` | Vec<String> | The set of activities that match the filter included in the request. |


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


### Activitie

Queries policy activities on GCP resources.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `activities` | Vec<String> | The set of activities that match the filter included in the request. |
| `next_page_token` | String | If there might be more results than those appearing in this response, then `nextPageToken` is included. To get the next set of results, call this method again using the value of `nextPageToken` as `pageToken`. |


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
activitie_activities = activitie.activities
activitie_next_page_token = activitie.next_page_token
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
activitie_0 = provider.policyanalyzer_api.Activitie {
}
activitie_1 = provider.policyanalyzer_api.Activitie {
}
activitie_2 = provider.policyanalyzer_api.Activitie {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    activitie = provider.policyanalyzer_api.Activitie {
    }
```

---

## Related Documentation

- [GCP Policyanalyzer_api Documentation](https://cloud.google.com/policyanalyzer_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
