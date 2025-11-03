# Webrisk_api Service



**Resources**: 5

---

## Overview

The webrisk_api service provides access to 5 resource types:

- [Submission](#submission) [C]
- [Uri](#uri) [R]
- [Operation](#operation) [CRD]
- [Hashe](#hashe) [R]
- [Threat_list](#threat_list) [R]

---

## Resources


### Submission

Creates a Submission of a URI suspected of containing phishing content to be reviewed. If the result verifies the existence of malicious phishing content, the site will be added to the [Google's Social Engineering lists](https://support.google.com/webmasters/answer/6350487/) in order to protect users that could get exposed to this threat in the future. Only allowlisted projects can use this method during Early Access. Please reach out to Sales or your customer engineer to obtain access.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `uri` | String |  | Required. The URI that is being reported for malicious content to be analyzed. |
| `parent` | String | ✅ | Required. The name of the project that is making the submission. This string is in the format "projects/{project_number}". |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create submission
submission = provider.webrisk_api.Submission {
    parent = "value"  # Required. The name of the project that is making the submission. This string is in the format "projects/{project_number}".
}

```

---


### Uri

This method is used to check whether a URI is on a given threatList. Multiple threatLists may be searched in a single query. The response will list all requested threatLists the URI was found to match. If the URI is not found on any of the requested ThreatList an empty response will be returned.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `threat` | String | The threat list matches. This might be empty if the URI is on no list. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access uri outputs
uri_id = uri.id
uri_threat = uri.threat
```

---


### Operation

Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of `1`, corresponding to `Code.CANCELLED`.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | The name of the operation resource to be cancelled. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `name` | String | Matches the `/v1/{project-name}/operations/{operation-id}` pattern. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `metadata` | HashMap<String, String> | Contains a `SubmitUriMetadata` object. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.webrisk_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_name = operation.name
operation_response = operation.response
operation_done = operation.done
operation_metadata = operation.metadata
```

---


### Hashe

Gets the full hashes that match the requested hash prefix. This is used after a hash prefix is looked up in a threatList and there is a match. The client side threatList only holds partial hashes so the client must query this method to determine if there is a full hash match of a threat.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `negative_expire_time` | String | For requested entities that did not match the threat list, how long to cache the response until. |
| `threats` | Vec<String> | The full hashes that matched the requested prefixes. The hash will be populated in the key. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access hashe outputs
hashe_id = hashe.id
hashe_negative_expire_time = hashe.negative_expire_time
hashe_threats = hashe.threats
```

---


### Threat_list

Gets the most recent threat list diffs. These diffs should be applied to a local database of hashes to keep it up-to-date. If the local database is empty or excessively out-of-date, a complete snapshot of the database will be returned. This Method only updates a single ThreatList at a time. To update multiple ThreatList databases, this method needs to be called once for each list.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `recommended_next_diff` | String | The soonest the client should wait before issuing any diff request. Querying sooner is unlikely to produce a meaningful diff. Waiting longer is acceptable considering the use case. If this field is not set clients may update as soon as they want. |
| `new_version_token` | String | The new opaque client version token. This should be retained by the client and passed into the next call of ComputeThreatListDiff as 'version_token'. A separate version token should be stored and used for each threatList. |
| `checksum` | String | The expected SHA256 hash of the client state; that is, of the sorted list of all hashes present in the database after applying the provided diff. If the client state doesn't match the expected state, the client must discard this diff and retry later. |
| `response_type` | String | The type of response. This may indicate that an action must be taken by the client when the response is received. |
| `additions` | String | A set of entries to add to a local threat type's list. |
| `removals` | String | A set of entries to remove from a local threat type's list. This field may be empty. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access threat_list outputs
threat_list_id = threat_list.id
threat_list_recommended_next_diff = threat_list.recommended_next_diff
threat_list_new_version_token = threat_list.new_version_token
threat_list_checksum = threat_list.checksum
threat_list_response_type = threat_list.response_type
threat_list_additions = threat_list.additions
threat_list_removals = threat_list.removals
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple submission resources
submission_0 = provider.webrisk_api.Submission {
    parent = "value-0"
}
submission_1 = provider.webrisk_api.Submission {
    parent = "value-1"
}
submission_2 = provider.webrisk_api.Submission {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    submission = provider.webrisk_api.Submission {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Webrisk_api Documentation](https://cloud.google.com/webrisk_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
