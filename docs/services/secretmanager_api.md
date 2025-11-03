# Secretmanager_api Service



**Resources**: 9

---

## Overview

The secretmanager_api service provides access to 9 resource types:

- [Version](#version) [CR]
- [Location](#location) [R]
- [Secret](#secret) [CRUD]
- [Secret](#secret) [CRUD]
- [Version](#version) [CR]
- [Location](#location) [R]
- [Version](#version) [CR]
- [Location](#location) [R]
- [Secret](#secret) [CRUD]

---

## Resources


### Version

Enables a SecretVersion. Sets the state of the SecretVersion to ENABLED.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `etag` | String |  | Optional. Etag of the SecretVersion. The request succeeds if it matches the etag of the currently stored secret version object. If the etag is omitted, the request succeeds. |
| `name` | String | ✅ | Required. The resource name of the SecretVersion to enable in the format `projects/*/secrets/*/versions/*` or `projects/*/locations/*/secrets/*/versions/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `replication_status` | String | The replication status of the SecretVersion. |
| `etag` | String | Output only. Etag of the currently stored SecretVersion. |
| `client_specified_payload_checksum` | bool | Output only. True if payload checksum specified in SecretPayload object has been received by SecretManagerService on SecretManagerService.AddSecretVersion. |
| `customer_managed_encryption` | String | Output only. The customer-managed encryption status of the SecretVersion. Only populated if customer-managed encryption is used and Secret is a regionalized secret. |
| `create_time` | String | Output only. The time at which the SecretVersion was created. |
| `name` | String | Output only. The resource name of the SecretVersion in the format `projects/*/secrets/*/versions/*`. SecretVersion IDs in a Secret start at 1 and are incremented for each subsequent version of the secret. |
| `scheduled_destroy_time` | String | Optional. Output only. Scheduled destroy time for secret version. This is a part of the Delayed secret version destroy feature. For a Secret with a valid version destroy TTL, when a secert version is destroyed, version is moved to disabled state and it is scheduled for destruction Version is destroyed only after the scheduled_destroy_time. |
| `state` | String | Output only. The current state of the SecretVersion. |
| `destroy_time` | String | Output only. The time this SecretVersion was destroyed. Only present if state is DESTROYED. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create version
version = provider.secretmanager_api.Version {
    name = "value"  # Required. The resource name of the SecretVersion to enable in the format `projects/*/secrets/*/versions/*` or `projects/*/locations/*/secrets/*/versions/*`.
}

# Access version outputs
version_id = version.id
version_replication_status = version.replication_status
version_etag = version.etag
version_client_specified_payload_checksum = version.client_specified_payload_checksum
version_customer_managed_encryption = version.customer_managed_encryption
version_create_time = version.create_time
version_name = version.name
version_scheduled_destroy_time = version.scheduled_destroy_time
version_state = version.state
version_destroy_time = version.destroy_time
```

---


### Location

Gets information about a location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |


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
location_location_id = location.location_id
location_display_name = location.display_name
location_labels = location.labels
location_metadata = location.metadata
location_name = location.name
```

---


### Secret

Creates a new Secret containing no SecretVersions.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Mapping of Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" Tags are used to organize and group resources. Tags can be used to control policy evaluation for the resource. |
| `replication` | String |  | Optional. Immutable. The replication policy of the secret data attached to the Secret. The replication policy cannot be changed after the Secret has been created. |
| `labels` | HashMap<String, String> |  | The labels assigned to this Secret. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: `\p{Ll}\p{Lo}{0,62}` Label values must be between 0 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: `[\p{Ll}\p{Lo}\p{N}_-]{0,63}` No more than 64 labels can be assigned to a given resource. |
| `name` | String |  | Output only. The resource name of the Secret in the format `projects/*/secrets/*`. |
| `ttl` | String |  | Input only. The TTL for the Secret. |
| `version_destroy_ttl` | String |  | Optional. Secret Version TTL after destruction request This is a part of the Delayed secret version destroy feature. For secret with TTL>0, version destruction doesn't happen immediately on calling destroy instead the version goes to a disabled state and destruction happens after the TTL expires. |
| `topics` | Vec<String> |  | Optional. A list of up to 10 Pub/Sub topics to which messages are published when control plane operations are called on the secret or its versions. |
| `create_time` | String |  | Output only. The time at which the Secret was created. |
| `version_aliases` | HashMap<String, String> |  | Optional. Mapping from version alias to version name. A version alias is a string with a maximum length of 63 characters and can contain uppercase and lowercase letters, numerals, and the hyphen (`-`) and underscore ('_') characters. An alias string must start with a letter and cannot be the string 'latest' or 'NEW'. No more than 50 aliases can be assigned to a given secret. Version-Alias pairs will be viewable via GetSecret and modifiable via UpdateSecret. Access by alias is only be supported on GetSecretVersion and AccessSecretVersion. |
| `customer_managed_encryption` | String |  | Optional. The customer-managed encryption configuration of the regionalized secrets. If no configuration is provided, Google-managed default encryption is used. Updates to the Secret encryption configuration only apply to SecretVersions added afterwards. They do not apply retroactively to existing SecretVersions. |
| `etag` | String |  | Optional. Etag of the currently stored Secret. |
| `expire_time` | String |  | Optional. Timestamp in UTC when the Secret is scheduled to expire. This is always provided on output, regardless of what was sent on input. |
| `annotations` | HashMap<String, String> |  | Optional. Custom metadata about the secret. Annotations are distinct from various forms of labels. Annotations exist to allow client tools to store their own state information without requiring a database. Annotation keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, begin and end with an alphanumeric character ([a-z0-9A-Z]), and may have dashes (-), underscores (_), dots (.), and alphanumerics in between these symbols. The total size of annotation keys and values must be less than 16KiB. |
| `rotation` | String |  | Optional. Rotation policy attached to the Secret. May be excluded if there is no rotation policy. |
| `parent` | String | ✅ | Required. The resource name of the project to associate with the Secret, in the format `projects/*` or `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Mapping of Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" Tags are used to organize and group resources. Tags can be used to control policy evaluation for the resource. |
| `replication` | String | Optional. Immutable. The replication policy of the secret data attached to the Secret. The replication policy cannot be changed after the Secret has been created. |
| `labels` | HashMap<String, String> | The labels assigned to this Secret. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: `\p{Ll}\p{Lo}{0,62}` Label values must be between 0 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: `[\p{Ll}\p{Lo}\p{N}_-]{0,63}` No more than 64 labels can be assigned to a given resource. |
| `name` | String | Output only. The resource name of the Secret in the format `projects/*/secrets/*`. |
| `ttl` | String | Input only. The TTL for the Secret. |
| `version_destroy_ttl` | String | Optional. Secret Version TTL after destruction request This is a part of the Delayed secret version destroy feature. For secret with TTL>0, version destruction doesn't happen immediately on calling destroy instead the version goes to a disabled state and destruction happens after the TTL expires. |
| `topics` | Vec<String> | Optional. A list of up to 10 Pub/Sub topics to which messages are published when control plane operations are called on the secret or its versions. |
| `create_time` | String | Output only. The time at which the Secret was created. |
| `version_aliases` | HashMap<String, String> | Optional. Mapping from version alias to version name. A version alias is a string with a maximum length of 63 characters and can contain uppercase and lowercase letters, numerals, and the hyphen (`-`) and underscore ('_') characters. An alias string must start with a letter and cannot be the string 'latest' or 'NEW'. No more than 50 aliases can be assigned to a given secret. Version-Alias pairs will be viewable via GetSecret and modifiable via UpdateSecret. Access by alias is only be supported on GetSecretVersion and AccessSecretVersion. |
| `customer_managed_encryption` | String | Optional. The customer-managed encryption configuration of the regionalized secrets. If no configuration is provided, Google-managed default encryption is used. Updates to the Secret encryption configuration only apply to SecretVersions added afterwards. They do not apply retroactively to existing SecretVersions. |
| `etag` | String | Optional. Etag of the currently stored Secret. |
| `expire_time` | String | Optional. Timestamp in UTC when the Secret is scheduled to expire. This is always provided on output, regardless of what was sent on input. |
| `annotations` | HashMap<String, String> | Optional. Custom metadata about the secret. Annotations are distinct from various forms of labels. Annotations exist to allow client tools to store their own state information without requiring a database. Annotation keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, begin and end with an alphanumeric character ([a-z0-9A-Z]), and may have dashes (-), underscores (_), dots (.), and alphanumerics in between these symbols. The total size of annotation keys and values must be less than 16KiB. |
| `rotation` | String | Optional. Rotation policy attached to the Secret. May be excluded if there is no rotation policy. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create secret
secret = provider.secretmanager_api.Secret {
    parent = "value"  # Required. The resource name of the project to associate with the Secret, in the format `projects/*` or `projects/*/locations/*`.
}

# Access secret outputs
secret_id = secret.id
secret_tags = secret.tags
secret_replication = secret.replication
secret_labels = secret.labels
secret_name = secret.name
secret_ttl = secret.ttl
secret_version_destroy_ttl = secret.version_destroy_ttl
secret_topics = secret.topics
secret_create_time = secret.create_time
secret_version_aliases = secret.version_aliases
secret_customer_managed_encryption = secret.customer_managed_encryption
secret_etag = secret.etag
secret_expire_time = secret.expire_time
secret_annotations = secret.annotations
secret_rotation = secret.rotation
```

---


### Secret

Creates a new Secret containing no SecretVersions.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `version_destroy_ttl` | String |  | Optional. Secret Version TTL after destruction request This is a part of the Delayed secret version destroy feature. For secret with TTL>0, version destruction doesn't happen immediately on calling destroy instead the version goes to a disabled state and destruction happens after the TTL expires. |
| `labels` | HashMap<String, String> |  | The labels assigned to this Secret. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: `\p{Ll}\p{Lo}{0,62}` Label values must be between 0 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: `[\p{Ll}\p{Lo}\p{N}_-]{0,63}` No more than 64 labels can be assigned to a given resource. |
| `expire_time` | String |  | Optional. Timestamp in UTC when the Secret is scheduled to expire. This is always provided on output, regardless of what was sent on input. |
| `name` | String |  | Output only. The resource name of the Secret in the format `projects/*/secrets/*`. |
| `create_time` | String |  | Output only. The time at which the Secret was created. |
| `annotations` | HashMap<String, String> |  | Optional. Custom metadata about the secret. Annotations are distinct from various forms of labels. Annotations exist to allow client tools to store their own state information without requiring a database. Annotation keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, begin and end with an alphanumeric character ([a-z0-9A-Z]), and may have dashes (-), underscores (_), dots (.), and alphanumerics in between these symbols. The total size of annotation keys and values must be less than 16KiB. |
| `customer_managed_encryption` | String |  | Optional. The customer-managed encryption configuration of the Regionalised Secrets. If no configuration is provided, Google-managed default encryption is used. Updates to the Secret encryption configuration only apply to SecretVersions added afterwards. They do not apply retroactively to existing SecretVersions. |
| `etag` | String |  | Optional. Etag of the currently stored Secret. |
| `replication` | String |  | Optional. Immutable. The replication policy of the secret data attached to the Secret. The replication policy cannot be changed after the Secret has been created. |
| `rotation` | String |  | Optional. Rotation policy attached to the Secret. May be excluded if there is no rotation policy. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Mapping of Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" Tags are used to organize and group resources. Tags can be used to control policy evaluation for the resource. |
| `topics` | Vec<String> |  | Optional. A list of up to 10 Pub/Sub topics to which messages are published when control plane operations are called on the secret or its versions. |
| `ttl` | String |  | Input only. The TTL for the Secret. |
| `version_aliases` | HashMap<String, String> |  | Optional. Mapping from version alias to version name. A version alias is a string with a maximum length of 63 characters and can contain uppercase and lowercase letters, numerals, and the hyphen (`-`) and underscore ('_') characters. An alias string must start with a letter and cannot be the string 'latest' or 'NEW'. No more than 50 aliases can be assigned to a given secret. Version-Alias pairs will be viewable via GetSecret and modifiable via UpdateSecret. Access by alias is only supported for GetSecretVersion and AccessSecretVersion. |
| `parent` | String | ✅ | Required. The resource name of the project to associate with the Secret, in the format `projects/*` or `projects/*/locations/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `version_destroy_ttl` | String | Optional. Secret Version TTL after destruction request This is a part of the Delayed secret version destroy feature. For secret with TTL>0, version destruction doesn't happen immediately on calling destroy instead the version goes to a disabled state and destruction happens after the TTL expires. |
| `labels` | HashMap<String, String> | The labels assigned to this Secret. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: `\p{Ll}\p{Lo}{0,62}` Label values must be between 0 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: `[\p{Ll}\p{Lo}\p{N}_-]{0,63}` No more than 64 labels can be assigned to a given resource. |
| `expire_time` | String | Optional. Timestamp in UTC when the Secret is scheduled to expire. This is always provided on output, regardless of what was sent on input. |
| `name` | String | Output only. The resource name of the Secret in the format `projects/*/secrets/*`. |
| `create_time` | String | Output only. The time at which the Secret was created. |
| `annotations` | HashMap<String, String> | Optional. Custom metadata about the secret. Annotations are distinct from various forms of labels. Annotations exist to allow client tools to store their own state information without requiring a database. Annotation keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, begin and end with an alphanumeric character ([a-z0-9A-Z]), and may have dashes (-), underscores (_), dots (.), and alphanumerics in between these symbols. The total size of annotation keys and values must be less than 16KiB. |
| `customer_managed_encryption` | String | Optional. The customer-managed encryption configuration of the Regionalised Secrets. If no configuration is provided, Google-managed default encryption is used. Updates to the Secret encryption configuration only apply to SecretVersions added afterwards. They do not apply retroactively to existing SecretVersions. |
| `etag` | String | Optional. Etag of the currently stored Secret. |
| `replication` | String | Optional. Immutable. The replication policy of the secret data attached to the Secret. The replication policy cannot be changed after the Secret has been created. |
| `rotation` | String | Optional. Rotation policy attached to the Secret. May be excluded if there is no rotation policy. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Mapping of Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" Tags are used to organize and group resources. Tags can be used to control policy evaluation for the resource. |
| `topics` | Vec<String> | Optional. A list of up to 10 Pub/Sub topics to which messages are published when control plane operations are called on the secret or its versions. |
| `ttl` | String | Input only. The TTL for the Secret. |
| `version_aliases` | HashMap<String, String> | Optional. Mapping from version alias to version name. A version alias is a string with a maximum length of 63 characters and can contain uppercase and lowercase letters, numerals, and the hyphen (`-`) and underscore ('_') characters. An alias string must start with a letter and cannot be the string 'latest' or 'NEW'. No more than 50 aliases can be assigned to a given secret. Version-Alias pairs will be viewable via GetSecret and modifiable via UpdateSecret. Access by alias is only supported for GetSecretVersion and AccessSecretVersion. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create secret
secret = provider.secretmanager_api.Secret {
    parent = "value"  # Required. The resource name of the project to associate with the Secret, in the format `projects/*` or `projects/*/locations/*`.
}

# Access secret outputs
secret_id = secret.id
secret_version_destroy_ttl = secret.version_destroy_ttl
secret_labels = secret.labels
secret_expire_time = secret.expire_time
secret_name = secret.name
secret_create_time = secret.create_time
secret_annotations = secret.annotations
secret_customer_managed_encryption = secret.customer_managed_encryption
secret_etag = secret.etag
secret_replication = secret.replication
secret_rotation = secret.rotation
secret_tags = secret.tags
secret_topics = secret.topics
secret_ttl = secret.ttl
secret_version_aliases = secret.version_aliases
```

---


### Version

Disables a SecretVersion. Sets the state of the SecretVersion to DISABLED.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `etag` | String |  | Optional. Etag of the SecretVersion. The request succeeds if it matches the etag of the currently stored secret version object. If the etag is omitted, the request succeeds. |
| `name` | String | ✅ | Required. The resource name of the SecretVersion to disable in the format `projects/*/secrets/*/versions/*` or `projects/*/locations/*/secrets/*/versions/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | Output only. Etag of the currently stored SecretVersion. |
| `client_specified_payload_checksum` | bool | Output only. True if payload checksum specified in SecretPayload object has been received by SecretManagerService on SecretManagerService.AddSecretVersion. |
| `create_time` | String | Output only. The time at which the SecretVersion was created. |
| `destroy_time` | String | Output only. The time this SecretVersion was destroyed. Only present if state is DESTROYED. |
| `name` | String | Output only. The resource name of the SecretVersion in the format `projects/*/secrets/*/versions/*`. SecretVersion IDs in a Secret start at 1 and are incremented for each subsequent version of the secret. |
| `replication_status` | String | The replication status of the SecretVersion. |
| `state` | String | Output only. The current state of the SecretVersion. |
| `customer_managed_encryption` | String | Output only. The customer-managed encryption status of the SecretVersion. Only populated if customer-managed encryption is used and Secret is a Regionalised Secret. |
| `scheduled_destroy_time` | String | Optional. Output only. Scheduled destroy time for secret version. This is a part of the Delayed secret version destroy feature. For a Secret with a valid version destroy TTL, when a secert version is destroyed, version is moved to disabled state and it is scheduled for destruction Version is destroyed only after the scheduled_destroy_time. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create version
version = provider.secretmanager_api.Version {
    name = "value"  # Required. The resource name of the SecretVersion to disable in the format `projects/*/secrets/*/versions/*` or `projects/*/locations/*/secrets/*/versions/*`.
}

# Access version outputs
version_id = version.id
version_etag = version.etag
version_client_specified_payload_checksum = version.client_specified_payload_checksum
version_create_time = version.create_time
version_destroy_time = version.destroy_time
version_name = version.name
version_replication_status = version.replication_status
version_state = version.state
version_customer_managed_encryption = version.customer_managed_encryption
version_scheduled_destroy_time = version.scheduled_destroy_time
```

---


### Location

Gets information about a location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |


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
location_location_id = location.location_id
location_display_name = location.display_name
location_labels = location.labels
location_metadata = location.metadata
location_name = location.name
```

---


### Version

Disables a SecretVersion. Sets the state of the SecretVersion to DISABLED.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | Required. The resource name of the SecretVersion to disable in the format `projects/*/secrets/*/versions/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. The resource name of the SecretVersion in the format `projects/*/secrets/*/versions/*`. SecretVersion IDs in a Secret start at 1 and are incremented for each subsequent version of the secret. |
| `state` | String | Output only. The current state of the SecretVersion. |
| `destroy_time` | String | Output only. The time this SecretVersion was destroyed. Only present if state is DESTROYED. |
| `create_time` | String | Output only. The time at which the SecretVersion was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create version
version = provider.secretmanager_api.Version {
    name = "value"  # Required. The resource name of the SecretVersion to disable in the format `projects/*/secrets/*/versions/*`.
}

# Access version outputs
version_id = version.id
version_name = version.name
version_state = version.state
version_destroy_time = version.destroy_time
version_create_time = version.create_time
```

---


### Location

Gets information about a location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |


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
location_location_id = location.location_id
location_name = location.name
location_metadata = location.metadata
location_display_name = location.display_name
location_labels = location.labels
```

---


### Secret

Creates a new Secret containing no SecretVersions.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. The resource name of the Secret in the format `projects/*/secrets/*`. |
| `replication` | String |  | Required. Immutable. The replication policy of the secret data attached to the Secret. The replication policy cannot be changed after the Secret has been created. |
| `tags` | HashMap<String, String> |  | Optional. Input only. Immutable. Mapping of Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" Tags are used to organize and group resources. Tags can be used to control policy evaluation for the resource. |
| `create_time` | String |  | Output only. The time at which the Secret was created. |
| `labels` | HashMap<String, String> |  | The labels assigned to this Secret. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: `\p{Ll}\p{Lo}{0,62}` Label values must be between 0 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: `[\p{Ll}\p{Lo}\p{N}_-]{0,63}` No more than 64 labels can be assigned to a given resource. |
| `parent` | String | ✅ | Required. The resource name of the project to associate with the Secret, in the format `projects/*`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. The resource name of the Secret in the format `projects/*/secrets/*`. |
| `replication` | String | Required. Immutable. The replication policy of the secret data attached to the Secret. The replication policy cannot be changed after the Secret has been created. |
| `tags` | HashMap<String, String> | Optional. Input only. Immutable. Mapping of Tag keys/values directly bound to this resource. For example: "123/environment": "production", "123/costCenter": "marketing" Tags are used to organize and group resources. Tags can be used to control policy evaluation for the resource. |
| `create_time` | String | Output only. The time at which the Secret was created. |
| `labels` | HashMap<String, String> | The labels assigned to this Secret. Label keys must be between 1 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: `\p{Ll}\p{Lo}{0,62}` Label values must be between 0 and 63 characters long, have a UTF-8 encoding of maximum 128 bytes, and must conform to the following PCRE regular expression: `[\p{Ll}\p{Lo}\p{N}_-]{0,63}` No more than 64 labels can be assigned to a given resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create secret
secret = provider.secretmanager_api.Secret {
    parent = "value"  # Required. The resource name of the project to associate with the Secret, in the format `projects/*`.
}

# Access secret outputs
secret_id = secret.id
secret_name = secret.name
secret_replication = secret.replication
secret_tags = secret.tags
secret_create_time = secret.create_time
secret_labels = secret.labels
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple version resources
version_0 = provider.secretmanager_api.Version {
    name = "value-0"
}
version_1 = provider.secretmanager_api.Version {
    name = "value-1"
}
version_2 = provider.secretmanager_api.Version {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    version = provider.secretmanager_api.Version {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Secretmanager_api Documentation](https://cloud.google.com/secretmanager_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
