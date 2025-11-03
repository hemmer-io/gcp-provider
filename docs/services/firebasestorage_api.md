# Firebasestorage_api Service



**Resources**: 3

---

## Overview

The firebasestorage_api service provides access to 3 resource types:

- [Project](#project) [RD]
- [Bucket](#bucket) [CR]
- [Default_bucket](#default_bucket) [C]

---

## Resources


### Project

Gets the default bucket.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bucket` | String | Output only. Underlying bucket resource. |
| `location` | String | Immutable. Location of the default bucket. |
| `name` | String | Resource name of the default bucket. |
| `storage_class` | String | Immutable. Storage class of the default bucket. Supported values are available at https://cloud.google.com/storage/docs/storage-classes#classes. |


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
project_bucket = project.bucket
project_location = project.location
project_name = project.name
project_storage_class = project.storage_class
```

---


### Bucket

Links a Google Cloud Storage bucket to a Firebase project.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `bucket` | String | ✅ | Required. Resource name of the bucket, mirrors the ID of the underlying Google Cloud Storage bucket, `projects/{project_id_or_number}/buckets/{bucket_id}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Resource name of the bucket. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create bucket
bucket = provider.firebasestorage_api.Bucket {
    bucket = "value"  # Required. Resource name of the bucket, mirrors the ID of the underlying Google Cloud Storage bucket, `projects/{project_id_or_number}/buckets/{bucket_id}`.
}

# Access bucket outputs
bucket_id = bucket.id
bucket_name = bucket.name
```

---


### Default_bucket

Creates a Spark tier-eligible Cloud Storage bucket and links it to your Firebase project. If the default bucket already exists, this method will re-link it to your Firebase project. See https://firebase.google.com/pricing for pricing details.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `bucket` | String |  | Output only. Underlying bucket resource. |
| `location` | String |  | Immutable. Location of the default bucket. |
| `name` | String |  | Resource name of the default bucket. |
| `storage_class` | String |  | Immutable. Storage class of the default bucket. Supported values are available at https://cloud.google.com/storage/docs/storage-classes#classes. |
| `parent` | String | ✅ | Required. The parent resource where the default bucket will be created, `projects/{project_id_or_number}`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create default_bucket
default_bucket = provider.firebasestorage_api.Default_bucket {
    parent = "value"  # Required. The parent resource where the default bucket will be created, `projects/{project_id_or_number}`.
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

# Create multiple project resources
project_0 = provider.firebasestorage_api.Project {
}
project_1 = provider.firebasestorage_api.Project {
}
project_2 = provider.firebasestorage_api.Project {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    project = provider.firebasestorage_api.Project {
    }
```

---

## Related Documentation

- [GCP Firebasestorage_api Documentation](https://cloud.google.com/firebasestorage_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
