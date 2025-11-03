# Playablelocations_api Service



**Resources**: 1

---

## Overview

The playablelocations_api service provides access to 1 resource type:

- [Playablelocation](#playablelocation) [C]

---

## Resources


### Playablelocation

Logs bad playable location reports submitted by players. Reports are not partially saved; either all reports are saved and this request succeeds, or no reports are saved, and this request fails.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `client_info` | String |  | Required. Information about the client device (for example, device model and operating system). |
| `player_reports` | Vec<String> |  | Required. Player reports. The maximum number of player reports that you can log at once is 50. |
| `request_id` | String |  | Required. A string that uniquely identifies the log player reports request. This allows you to detect duplicate requests. We recommend that you use UUIDs for this value. The value must not exceed 50 characters. You should reuse the `request_id` only when retrying a request in the case of a failure. In that case, the request must be identical to the one that failed. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create playablelocation
playablelocation = provider.playablelocations_api.Playablelocation {
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

# Create multiple playablelocation resources
playablelocation_0 = provider.playablelocations_api.Playablelocation {
}
playablelocation_1 = provider.playablelocations_api.Playablelocation {
}
playablelocation_2 = provider.playablelocations_api.Playablelocation {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    playablelocation = provider.playablelocations_api.Playablelocation {
    }
```

---

## Related Documentation

- [GCP Playablelocations_api Documentation](https://cloud.google.com/playablelocations_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
