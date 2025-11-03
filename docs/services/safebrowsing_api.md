# Safebrowsing_api Service



**Resources**: 9

---

## Overview

The safebrowsing_api service provides access to 9 resource types:

- [Hash_list](#hash_list) [R]
- [Hashe](#hashe) [R]
- [Threat_list_update](#threat_list_update) [C]
- [Threat_matche](#threat_matche) [C]
- [Threat_list](#threat_list) [R]
- [Encoded_full_hashe](#encoded_full_hashe) [R]
- [Encoded_update](#encoded_update) [R]
- [Threat_hit](#threat_hit) [C]
- [Full_hashe](#full_hashe) [C]

---

## Resources


### Hash_list

Get the latest contents of a hash list. A hash list may either by a threat list or a non-threat list such as the Global Cache. This is a standard Get method as defined by https://google.aip.dev/131 and the HTTP method is also GET.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `sha256_checksum` | String | The sorted list of all hashes, hashed again with SHA256. This is the checksum for the sorted list of all hashes present in the database after applying the provided update. In the case that no updates were provided, the server will omit this field to indicate that the client should use the existing checksum. |
| `additions_sixteen_bytes` | String | The 16-byte additions. |
| `minimum_wait_duration` | String | Clients should wait at least this long to get the hash list again. If omitted or zero, clients SHOULD fetch immediately because it indicates that the server has an additional update to be sent to the client, but could not due to the client-specified constraints. |
| `additions_thirty_two_bytes` | String | The 32-byte additions. |
| `compressed_removals` | String | The Rice-delta encoded version of removal indices. Since each hash list definitely has less than 2^32 entries, the indices are treated as 32-bit integers and encoded. |
| `name` | String | The name of the hash list. Note that the Global Cache is also just a hash list and can be referred to here. |
| `metadata` | String | Metadata about the hash list. This is not populated by the `GetHashList` method, but this is populated by the `ListHashLists` method. |
| `additions_eight_bytes` | String | The 8-byte additions. |
| `version` | String | The version of the hash list. The client MUST NOT manipulate those bytes. |
| `additions_four_bytes` | String | The 4-byte additions. |
| `partial_update` | bool | When true, this is a partial diff containing additions and removals based on what the client already has. When false, this is the complete hash list. When false, the client MUST delete any locally stored version for this hash list. This means that either the version possessed by the client is seriously out-of-date or the client data is believed to be corrupt. The `compressed_removals` field will be empty. When true, the client MUST apply an incremental update by applying removals and then additions. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access hash_list outputs
hash_list_id = hash_list.id
hash_list_sha256_checksum = hash_list.sha256_checksum
hash_list_additions_sixteen_bytes = hash_list.additions_sixteen_bytes
hash_list_minimum_wait_duration = hash_list.minimum_wait_duration
hash_list_additions_thirty_two_bytes = hash_list.additions_thirty_two_bytes
hash_list_compressed_removals = hash_list.compressed_removals
hash_list_name = hash_list.name
hash_list_metadata = hash_list.metadata
hash_list_additions_eight_bytes = hash_list.additions_eight_bytes
hash_list_version = hash_list.version
hash_list_additions_four_bytes = hash_list.additions_four_bytes
hash_list_partial_update = hash_list.partial_update
```

---


### Hashe

Search for full hashes matching the specified prefixes. This is a custom method as defined by https://google.aip.dev/136 (the custom method refers to this method having a custom name within Google's general API development nomenclature; it does not refer to using a custom HTTP method).

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cache_duration` | String | The client-side cache duration. The client MUST add this duration to the current time to determine the expiration time. The expiration time then applies to every hash prefix queried by the client in the request, regardless of how many full hashes are returned in the response. Even if the server returns no full hashes for a particular hash prefix, this fact MUST also be cached by the client. If and only if the field `full_hashes` is empty, the client MAY increase the `cache_duration` to determine a new expiration that is later than that specified by the server. In any case, the increased cache duration must not be longer than 24 hours. Important: the client MUST NOT assume that the server will return the same cache duration for all responses. The server MAY choose different cache durations for different responses depending on the situation. |
| `full_hashes` | Vec<String> | Unordered list. The unordered list of full hashes found. |


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
hashe_cache_duration = hashe.cache_duration
hashe_full_hashes = hashe.full_hashes
```

---


### Threat_list_update

Fetches the most recent threat list updates. A client can request updates for multiple lists at once.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client` | String |  | The client metadata. |
| `list_update_requests` | Vec<String> |  | The requested threat list updates. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create threat_list_update
threat_list_update = provider.safebrowsing_api.Threat_list_update {
}

```

---


### Threat_matche

Finds the threat entries that match the Safe Browsing lists.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `threat_info` | String |  | The lists and entries to be checked for matches. |
| `client` | String |  | The client metadata. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create threat_matche
threat_matche = provider.safebrowsing_api.Threat_matche {
}

```

---


### Threat_list

Lists the Safe Browsing threat lists available for download.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `threat_lists` | Vec<String> | The lists available for download by the client. |


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
threat_list_threat_lists = threat_list.threat_lists
```

---


### Encoded_full_hashe



**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `matches` | Vec<String> | The full hashes that matched the requested prefixes. |
| `negative_cache_duration` | String | For requested entities that did not match the threat list, how long to cache the response. |
| `minimum_wait_duration` | String | The minimum duration the client must wait before issuing any find hashes request. If this field is not set, clients can issue a request as soon as they want. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access encoded_full_hashe outputs
encoded_full_hashe_id = encoded_full_hashe.id
encoded_full_hashe_matches = encoded_full_hashe.matches
encoded_full_hashe_negative_cache_duration = encoded_full_hashe.negative_cache_duration
encoded_full_hashe_minimum_wait_duration = encoded_full_hashe.minimum_wait_duration
```

---


### Encoded_update



**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `minimum_wait_duration` | String | The minimum duration the client must wait before issuing any update request. If this field is not set clients may update as soon as they want. |
| `list_update_responses` | Vec<String> | The list updates requested by the clients. The number of responses here may be less than the number of requests sent by clients. This is the case, for example, if the server has no updates for a particular list. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access encoded_update outputs
encoded_update_id = encoded_update.id
encoded_update_minimum_wait_duration = encoded_update.minimum_wait_duration
encoded_update_list_update_responses = encoded_update.list_update_responses
```

---


### Threat_hit

Reports a Safe Browsing threat list hit to Google. Only projects with TRUSTED_REPORTER visibility can use this method.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `threat_type` | String |  | The threat type reported. |
| `user_info` | String |  | Details about the user that encountered the threat. |
| `client_info` | String |  | Client-reported identification. |
| `platform_type` | String |  | The platform type reported. |
| `resources` | Vec<String> |  | The resources related to the threat hit. |
| `entry` | String |  | The threat entry responsible for the hit. Full hash should be reported for hash-based hits. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create threat_hit
threat_hit = provider.safebrowsing_api.Threat_hit {
}

```

---


### Full_hashe

Finds the full hashes that match the requested hash prefixes.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `api_client` | String |  | Client metadata associated with callers of higher-level APIs built on top of the client's implementation. |
| `threat_info` | String |  | The lists and hashes to be checked. |
| `client` | String |  | The client metadata. |
| `client_states` | Vec<String> |  | The current client states for each of the client's local threat lists. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create full_hashe
full_hashe = provider.safebrowsing_api.Full_hashe {
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

# Create multiple hash_list resources
hash_list_0 = provider.safebrowsing_api.Hash_list {
}
hash_list_1 = provider.safebrowsing_api.Hash_list {
}
hash_list_2 = provider.safebrowsing_api.Hash_list {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    hash_list = provider.safebrowsing_api.Hash_list {
    }
```

---

## Related Documentation

- [GCP Safebrowsing_api Documentation](https://cloud.google.com/safebrowsing_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
