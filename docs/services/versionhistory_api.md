# Versionhistory_api Service



**Resources**: 4

---

## Overview

The versionhistory_api service provides access to 4 resource types:

- [Channel](#channel) [R]
- [Platform](#platform) [R]
- [Release](#release) [R]
- [Version](#version) [R]

---

## Resources


### Channel

Returns list of channels that are available for a given platform.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `channels` | Vec<String> | The list of channels. |
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access channel outputs
channel_id = channel.id
channel_channels = channel.channels
channel_next_page_token = channel.next_page_token
```

---


### Platform

Returns list of platforms that are available for a given product. The resource "product" has no resource name in its name.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `platforms` | Vec<String> | The list of platforms. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access platform outputs
platform_id = platform.id
platform_next_page_token = platform.next_page_token
platform_platforms = platform.platforms
```

---


### Release

Returns list of releases of the given version.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `releases` | Vec<String> | The list of releases. |
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access release outputs
release_id = release.id
release_releases = release.releases
release_next_page_token = release.next_page_token
```

---


### Version

Returns list of version for the given platform/channel.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `versions` | Vec<String> | The list of versions. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access version outputs
version_id = version.id
version_next_page_token = version.next_page_token
version_versions = version.versions
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple channel resources
channel_0 = provider.versionhistory_api.Channel {
}
channel_1 = provider.versionhistory_api.Channel {
}
channel_2 = provider.versionhistory_api.Channel {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    channel = provider.versionhistory_api.Channel {
    }
```

---

## Related Documentation

- [GCP Versionhistory_api Documentation](https://cloud.google.com/versionhistory_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
