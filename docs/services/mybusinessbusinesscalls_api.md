# Mybusinessbusinesscalls_api Service



**Resources**: 2

---

## Overview

The mybusinessbusinesscalls_api service provides access to 2 resource types:

- [Businesscallsinsight](#businesscallsinsight) [R]
- [Location](#location) [RU]

---

## Resources


### Businesscallsinsight

Returns insights for Business calls for a location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `business_calls_insights` | Vec<String> | A collection of business calls insights for the location. |
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. Some of the metric_types (e.g, AGGREGATE_COUNT) returns a single page. For these metrics, the next_page_token will be empty. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access businesscallsinsight outputs
businesscallsinsight_id = businesscallsinsight.id
businesscallsinsight_business_calls_insights = businesscallsinsight.business_calls_insights
businesscallsinsight_next_page_token = businesscallsinsight.next_page_token
```

---


### Location

Returns the Business calls settings resource for the given location.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `consent_time` | String |  | Input only. Time when the end user provided consent to the API user to enable business calls. |
| `calls_state` | String |  | Required. The state of this location's enrollment in Business calls. |
| `name` | String |  | Required. The resource name of the calls settings. Format: locations/{location}/businesscallssettings |
| `name` | String | ✅ | Required. The resource name of the calls settings. Format: locations/{location}/businesscallssettings |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `consent_time` | String | Input only. Time when the end user provided consent to the API user to enable business calls. |
| `calls_state` | String | Required. The state of this location's enrollment in Business calls. |
| `name` | String | Required. The resource name of the calls settings. Format: locations/{location}/businesscallssettings |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access location outputs
location_id = location.id
location_consent_time = location.consent_time
location_calls_state = location.calls_state
location_name = location.name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple businesscallsinsight resources
businesscallsinsight_0 = provider.mybusinessbusinesscalls_api.Businesscallsinsight {
}
businesscallsinsight_1 = provider.mybusinessbusinesscalls_api.Businesscallsinsight {
}
businesscallsinsight_2 = provider.mybusinessbusinesscalls_api.Businesscallsinsight {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    businesscallsinsight = provider.mybusinessbusinesscalls_api.Businesscallsinsight {
    }
```

---

## Related Documentation

- [GCP Mybusinessbusinesscalls_api Documentation](https://cloud.google.com/mybusinessbusinesscalls_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
