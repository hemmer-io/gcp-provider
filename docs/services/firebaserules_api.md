# Firebaserules_api Service



**Resources**: 3

---

## Overview

The firebaserules_api service provides access to 3 resource types:

- [Release](#release) [CRUD]
- [Project](#project) [C]
- [Ruleset](#ruleset) [CRD]

---

## Resources


### Release

Create a `Release`. Release names should reflect the developer's deployment practices. For example, the release name may include the environment name, application name, application version, or any other name meaningful to the developer. Once a `Release` refers to a `Ruleset`, the rules can be enforced by Firebase Rules-enabled services. More than one `Release` may be 'live' concurrently. Consider the following three `Release` names for `projects/foo` and the `Ruleset` to which they refer. Release Name -> Ruleset Name * projects/foo/releases/prod -> projects/foo/rulesets/uuid123 * projects/foo/releases/prod/beta -> projects/foo/rulesets/uuid123 * projects/foo/releases/prod/v23 -> projects/foo/rulesets/uuid456 The relationships reflect a `Ruleset` rollout in progress. The `prod` and `prod/beta` releases refer to the same `Ruleset`. However, `prod/v23` refers to a new `Ruleset`. The `Ruleset` reference for a `Release` may be updated using the UpdateRelease method.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Time the release was created. |
| `ruleset_name` | String |  | Required. Name of the `Ruleset` referred to by this `Release`. The `Ruleset` must exist for the `Release` to be created. |
| `update_time` | String |  | Output only. Time the release was updated. |
| `name` | String |  | Required. Format: `projects/{project_id}/releases/{release_id}` |
| `name` | String | ✅ | Required. Resource name for the project which owns this `Release`. Format: `projects/{project_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Time the release was created. |
| `ruleset_name` | String | Required. Name of the `Ruleset` referred to by this `Release`. The `Ruleset` must exist for the `Release` to be created. |
| `update_time` | String | Output only. Time the release was updated. |
| `name` | String | Required. Format: `projects/{project_id}/releases/{release_id}` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create release
release = provider.firebaserules_api.Release {
    name = "value"  # Required. Resource name for the project which owns this `Release`. Format: `projects/{project_id}`
}

# Access release outputs
release_id = release.id
release_create_time = release.create_time
release_ruleset_name = release.ruleset_name
release_update_time = release.update_time
release_name = release.name
```

---


### Project

Test `Source` for syntactic and semantic correctness. Issues present, if any, will be returned to the caller with a description, severity, and source location. The test method may be executed with `Source` or a `Ruleset` name. Passing `Source` is useful for unit testing new rules. Passing a `Ruleset` name is useful for regression testing an existing rule. The following is an example of `Source` that permits users to upload images to a bucket bearing their user id and matching the correct metadata: _*Example*_ // Users are allowed to subscribe and unsubscribe to the blog. service firebase.storage { match /users/{userId}/images/{imageName} { allow write: if userId == request.auth.uid && (imageName.matches('*.png$') || imageName.matches('*.jpg$')) && resource.mimeType.matches('^image/') } }

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `test_suite` | String |  | Required. The tests to execute against the `Source`. When `Source` is provided inline, the test cases will only be run if the `Source` is syntactically and semantically valid. Inline `TestSuite` to run. |
| `source` | String |  | Optional. Optional `Source` to be checked for correctness. This field must not be set when the resource name refers to a `Ruleset`. |
| `name` | String | ✅ | Required. Tests may either provide `source` or a `Ruleset` resource name. For tests against `source`, the resource name must refer to the project: Format: `projects/{project_id}` For tests against a `Ruleset`, this must be the `Ruleset` resource name: Format: `projects/{project_id}/rulesets/{ruleset_id}` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create project
project = provider.firebaserules_api.Project {
    name = "value"  # Required. Tests may either provide `source` or a `Ruleset` resource name. For tests against `source`, the resource name must refer to the project: Format: `projects/{project_id}` For tests against a `Ruleset`, this must be the `Ruleset` resource name: Format: `projects/{project_id}/rulesets/{ruleset_id}`
}

```

---


### Ruleset

Create a `Ruleset` from `Source`. The `Ruleset` is given a unique generated name which is returned to the caller. `Source` containing syntactic or semantics errors will result in an error response indicating the first error encountered. For a detailed view of `Source` issues, use TestRuleset.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Time the `Ruleset` was created. |
| `source` | String |  | Required. `Source` for the `Ruleset`. |
| `attachment_point` | String |  | Immutable. Intended resource to which this Ruleset should be released. May be left blank to signify the resource associated with the default release. Expected format: firestore.googleapis.com/projects//databases/ |
| `metadata` | String |  | Output only. The metadata for this ruleset. |
| `name` | String |  | Output only. Name of the `Ruleset`. The ruleset_id is auto generated by the service. Format: `projects/{project_id}/rulesets/{ruleset_id}` |
| `name` | String | ✅ | Required. Resource name for Project which owns this `Ruleset`. Format: `projects/{project_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Time the `Ruleset` was created. |
| `source` | String | Required. `Source` for the `Ruleset`. |
| `attachment_point` | String | Immutable. Intended resource to which this Ruleset should be released. May be left blank to signify the resource associated with the default release. Expected format: firestore.googleapis.com/projects//databases/ |
| `metadata` | String | Output only. The metadata for this ruleset. |
| `name` | String | Output only. Name of the `Ruleset`. The ruleset_id is auto generated by the service. Format: `projects/{project_id}/rulesets/{ruleset_id}` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create ruleset
ruleset = provider.firebaserules_api.Ruleset {
    name = "value"  # Required. Resource name for Project which owns this `Ruleset`. Format: `projects/{project_id}`
}

# Access ruleset outputs
ruleset_id = ruleset.id
ruleset_create_time = ruleset.create_time
ruleset_source = ruleset.source
ruleset_attachment_point = ruleset.attachment_point
ruleset_metadata = ruleset.metadata
ruleset_name = ruleset.name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple release resources
release_0 = provider.firebaserules_api.Release {
    name = "value-0"
}
release_1 = provider.firebaserules_api.Release {
    name = "value-1"
}
release_2 = provider.firebaserules_api.Release {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    release = provider.firebaserules_api.Release {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Firebaserules_api Documentation](https://cloud.google.com/firebaserules_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
