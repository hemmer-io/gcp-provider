# Datamanager_api Service



**Resources**: 3

---

## Overview

The datamanager_api service provides access to 3 resource types:

- [Request_statu](#request_statu) [R]
- [Event](#event) [C]
- [Audience_member](#audience_member) [C]

---

## Resources


### Request_statu

Gets the status of a request given request id.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `request_status_per_destination` | Vec<String> | A list of request statuses per destination. The order of the statuses matches the order of the destinations in the original request. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access request_statu outputs
request_statu_id = request_statu.id
request_statu_request_status_per_destination = request_statu.request_status_per_destination
```

---


### Event

Uploads a list of Event resources from the provided Destination.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `encoding` | String |  | Optional. Required for UserData uploads. The encoding type of the user identifiers. For hashed user identifiers, this is the encoding type of the hashed string. For encrypted hashed user identifiers, this is the encoding type of the outer encrypted string, but not necessarily the inner hashed string, meaning the inner hashed string could be encoded in a different way than the outer encrypted string. For non `UserData` uploads, this field is ignored. |
| `consent` | String |  | Optional. Request-level consent to apply to all users in the request. User-level consent overrides request-level consent, and can be specified in each Event. |
| `encryption_info` | String |  | Optional. Encryption information for UserData uploads. If not set, it's assumed that uploaded identifying information is hashed but not encrypted. For non `UserData` uploads, this field is ignored. |
| `destinations` | Vec<String> |  | Required. The list of destinations to send the events to. |
| `events` | Vec<String> |  | Required. The list of events to send to the specified destinations. At most 2000 Event resources can be sent in a single request. |
| `validate_only` | bool |  | Optional. For testing purposes. If `true`, the request is validated but not executed. Only errors are returned, not results. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create event
event = provider.datamanager_api.Event {
}

```

---


### Audience_member

Uploads a list of AudienceMember resources to the provided Destination.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `terms_of_service` | String |  | Optional. The terms of service that the user has accepted/rejected. |
| `audience_members` | Vec<String> |  | Required. The list of users to send to the specified destinations. At most 10000 AudienceMember resources can be sent in a single request. |
| `destinations` | Vec<String> |  | Required. The list of destinations to send the audience members to. |
| `validate_only` | bool |  | Optional. For testing purposes. If `true`, the request is validated but not executed. Only errors are returned, not results. |
| `consent` | String |  | Optional. Request-level consent to apply to all users in the request. User-level consent overrides request-level consent, and can be specified in each AudienceMember. |
| `encoding` | String |  | Optional. Required for UserData uploads. The encoding type of the user identifiers. For hashed user identifiers, this is the encoding type of the hashed string. For encrypted hashed user identifiers, this is the encoding type of the outer encrypted string, but not necessarily the inner hashed string, meaning the inner hashed string could be encoded in a different way than the outer encrypted string. For non `UserData` uploads, this field is ignored. |
| `encryption_info` | String |  | Optional. Encryption information for UserData uploads. If not set, it's assumed that uploaded identifying information is hashed but not encrypted. For non `UserData` uploads, this field is ignored. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create audience_member
audience_member = provider.datamanager_api.Audience_member {
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

# Create multiple request_statu resources
request_statu_0 = provider.datamanager_api.Request_statu {
}
request_statu_1 = provider.datamanager_api.Request_statu {
}
request_statu_2 = provider.datamanager_api.Request_statu {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    request_statu = provider.datamanager_api.Request_statu {
    }
```

---

## Related Documentation

- [GCP Datamanager_api Documentation](https://cloud.google.com/datamanager_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
