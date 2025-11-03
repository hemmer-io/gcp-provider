# Cloudprofiler_api Service



**Resources**: 1

---

## Overview

The cloudprofiler_api service provides access to 1 resource type:

- [Profile](#profile) [CRU]

---

## Resources


### Profile

CreateProfile creates a new profile resource in the online mode. _Direct use of this API is discouraged, please use a [supported profiler agent](https://cloud.google.com/profiler/docs/about-profiler#profiling_agent) instead for profile collection._ The server ensures that the new profiles are created at a constant rate per deployment, so the creation request may hang for some time until the next profile session is available. The request may fail with ABORTED error if the creation is not available within ~1m, the response will indicate the duration of the backoff the client should take before attempting creating a profile again. The backoff duration is returned in google.rpc.RetryInfo extension on the response status. To a gRPC client, the extension will be return as a binary-serialized proto in the trailing metadata item named "google.rpc.retryinfo-bin". 

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `deployment` | String |  | Deployment details. |
| `profile_type` | Vec<String> |  | One or more profile types that the agent is capable of providing. |
| `parent` | String | ✅ | Parent project to create the profile in. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Token to receive the next page of results. This field maybe empty if there are no more profiles to fetch. |
| `profiles` | Vec<String> | List of profiles fetched. |
| `skipped_profiles` | i64 | Number of profiles that were skipped in the current page since they were not able to be fetched successfully. This should typically be zero. A non-zero value may indicate a transient failure, in which case if the number is too high for your use case, the call may be retried. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create profile
profile = provider.cloudprofiler_api.Profile {
    parent = "value"  # Parent project to create the profile in.
}

# Access profile outputs
profile_id = profile.id
profile_next_page_token = profile.next_page_token
profile_profiles = profile.profiles
profile_skipped_profiles = profile.skipped_profiles
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple profile resources
profile_0 = provider.cloudprofiler_api.Profile {
    parent = "value-0"
}
profile_1 = provider.cloudprofiler_api.Profile {
    parent = "value-1"
}
profile_2 = provider.cloudprofiler_api.Profile {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    profile = provider.cloudprofiler_api.Profile {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Cloudprofiler_api Documentation](https://cloud.google.com/cloudprofiler_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
