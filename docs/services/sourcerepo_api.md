# Sourcerepo_api Service



**Resources**: 2

---

## Overview

The sourcerepo_api service provides access to 2 resource types:

- [Repo](#repo) [CRUD]
- [Project](#project) [RU]

---

## Resources


### Repo

Creates a repo in the given project with the given name. If the named repository already exists, `CreateRepo` returns `ALREADY_EXISTS`.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `url` | String |  | URL to clone the repository from Google Cloud Source Repositories. Read-only field. |
| `size` | String |  | The disk usage of the repo, in bytes. Read-only field. Size is only returned by GetRepo. |
| `name` | String |  | Resource name of the repository, of the form `projects//repos/`. The repo name may contain slashes. eg, `projects/myproject/repos/name/with/slash` |
| `mirror_config` | String |  | How this repository mirrors a repository managed by another service. Read-only field. |
| `pubsub_configs` | HashMap<String, String> |  | How this repository publishes a change in the repository through Cloud Pub/Sub. Keyed by the topic names. |
| `parent` | String | ✅ | The project in which to create the repo. Values are of the form `projects/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `url` | String | URL to clone the repository from Google Cloud Source Repositories. Read-only field. |
| `size` | String | The disk usage of the repo, in bytes. Read-only field. Size is only returned by GetRepo. |
| `name` | String | Resource name of the repository, of the form `projects//repos/`. The repo name may contain slashes. eg, `projects/myproject/repos/name/with/slash` |
| `mirror_config` | String | How this repository mirrors a repository managed by another service. Read-only field. |
| `pubsub_configs` | HashMap<String, String> | How this repository publishes a change in the repository through Cloud Pub/Sub. Keyed by the topic names. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create repo
repo = provider.sourcerepo_api.Repo {
    parent = "value"  # The project in which to create the repo. Values are of the form `projects/`.
}

# Access repo outputs
repo_id = repo.id
repo_url = repo.url
repo_size = repo.size
repo_name = repo.name
repo_mirror_config = repo.mirror_config
repo_pubsub_configs = repo.pubsub_configs
```

---


### Project

Returns the Cloud Source Repositories configuration of the project.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_mask` | String |  | A FieldMask specifying which fields of the project_config to modify. Only the fields in the mask will be modified. If no mask is provided, this request is no-op. |
| `project_config` | String |  | The new configuration for the project. |
| `name` | String | ✅ | The name of the requested project. Values are of the form `projects/`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `pubsub_configs` | HashMap<String, String> | How this project publishes a change in the repositories through Cloud Pub/Sub. Keyed by the topic names. |
| `enable_private_key_check` | bool | Reject a Git push that contains a private key. |
| `name` | String | The name of the project. Values are of the form `projects/`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access project outputs
project_id = project.id
project_pubsub_configs = project.pubsub_configs
project_enable_private_key_check = project.enable_private_key_check
project_name = project.name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple repo resources
repo_0 = provider.sourcerepo_api.Repo {
    parent = "value-0"
}
repo_1 = provider.sourcerepo_api.Repo {
    parent = "value-1"
}
repo_2 = provider.sourcerepo_api.Repo {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    repo = provider.sourcerepo_api.Repo {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Sourcerepo_api Documentation](https://cloud.google.com/sourcerepo_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
