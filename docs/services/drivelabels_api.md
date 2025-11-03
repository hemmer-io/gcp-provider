# Drivelabels_api Service



**Resources**: 12

---

## Overview

The drivelabels_api service provides access to 12 resource types:

- [Revision](#revision) [U]
- [Permission](#permission) [CRD]
- [User](#user) [R]
- [Limit](#limit) [R]
- [Label](#label) [CRUD]
- [Lock](#lock) [R]
- [Permission](#permission) [CRD]
- [Revision](#revision) [U]
- [User](#user) [R]
- [Limit](#limit) [R]
- [Label](#label) [CRUD]
- [Lock](#lock) [R]

---

## Resources


### Revision



**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `email` | String |  | Specifies the email address for a user or group principal. Not populated for audience principals. User and group permissions may only be inserted using an email address. On update requests, if email address is specified, no principal should be specified. |
| `role` | String |  | The role the principal should have. |
| `audience` | String |  | Audience to grant a role to. The magic value of `audiences/default` may be used to apply the role to the default audience in the context of the organization that owns the label. |
| `group` | String |  | Group resource name. |
| `name` | String |  | Resource name of this permission. |
| `person` | String |  | Person resource name. |
| `parent` | String | ✅ | Required. The parent label resource name. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

```

---


### Permission

Updates a label's permissions. If a permission for the indicated principal doesn't exist, a label permission is created, otherwise the existing permission is updated. Permissions affect the label resource as a whole, aren't revisioned, and don't require publishing.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `email` | String |  | Specifies the email address for a user or group principal. Not populated for audience principals. User and group permissions may only be inserted using an email address. On update requests, if email address is specified, no principal should be specified. |
| `role` | String |  | The role the principal should have. |
| `audience` | String |  | Audience to grant a role to. The magic value of `audiences/default` may be used to apply the role to the default audience in the context of the organization that owns the label. |
| `group` | String |  | Group resource name. |
| `name` | String |  | Resource name of this permission. |
| `person` | String |  | Person resource name. |
| `parent` | String | ✅ | Required. The parent label resource name on the label permission is created. Format: `labels/{label}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | The token of the next page in the response. |
| `label_permissions` | Vec<String> | Label permissions. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create permission
permission = provider.drivelabels_api.Permission {
    parent = "value"  # Required. The parent label resource name on the label permission is created. Format: `labels/{label}`.
}

# Access permission outputs
permission_id = permission.id
permission_next_page_token = permission.next_page_token
permission_label_permissions = permission.label_permissions
```

---


### User

Gets the user capabilities.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `can_create_admin_labels` | bool | Output only. Whether the user is allowed to create admin labels. |
| `can_administrate_labels` | bool | Output only. Whether the user is an administrator for the shared labels feature. |
| `can_create_shared_labels` | bool | Output only. Whether the user is allowed to create shared labels. |
| `name` | String | Output only. Resource name for the user capabilities. |
| `can_access_label_manager` | bool | Output only. Whether the user is allowed access to the label manager. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access user outputs
user_id = user.id
user_can_create_admin_labels = user.can_create_admin_labels
user_can_administrate_labels = user.can_administrate_labels
user_can_create_shared_labels = user.can_create_shared_labels
user_name = user.name
user_can_access_label_manager = user.can_access_label_manager
```

---


### Limit

Get the constraints on the structure of a label; such as, the maximum number of fields allowed and maximum length of the label title.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `max_fields` | i64 | The maximum number of fields allowed within the label. |
| `field_limits` | String | The limits for fields. |
| `max_description_length` | i64 | The maximum number of characters allowed for the description. |
| `max_deleted_fields` | i64 | The maximum number of published fields that can be deleted. |
| `max_draft_revisions` | i64 | The maximum number of draft revisions that will be kept before deleting old drafts. |
| `max_title_length` | i64 | The maximum number of characters allowed for the title. |
| `name` | String | Resource name. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access limit outputs
limit_id = limit.id
limit_max_fields = limit.max_fields
limit_field_limits = limit.field_limits
limit_max_description_length = limit.max_description_length
limit_max_deleted_fields = limit.max_deleted_fields
limit_max_draft_revisions = limit.max_draft_revisions
limit_max_title_length = limit.max_title_length
limit_name = limit.name
```

---


### Label

Creates a label. For more information, see [Create and publish a label](https://developers.google.com/workspace/drive/labels/guides/create-label).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `customer` | String |  | Output only. The customer this label belongs to. For example: `customers/123abc789`. |
| `revision_creator` | String |  | Output only. The user who created this label revision. |
| `revision_id` | String |  | Output only. Revision ID of the label. Revision ID might be part of the label `name` depending on the request issued. A new revision is created whenever revisioned properties of a label are changed. Matches the regex: `([a-zA-Z0-9])+`. |
| `disable_time` | String |  | Output only. The time this label was disabled. This value has no meaning when the label isn't disabled. |
| `schema_capabilities` | String |  | Output only. The capabilities the user has on this label. |
| `create_time` | String |  | Output only. The time this label was created. |
| `lifecycle` | String |  | Output only. The lifecycle state of the label including whether it's published, deprecated, and has draft changes. |
| `name` | String |  | Output only. Resource name of the label. Will be in the form of either: `labels/{id}` or `labels/{id}@{revision_id}` depending on the request. See `id` and `revision_id` below. |
| `properties` | String |  | Required. The basic properties of the label. |
| `publish_time` | String |  | Output only. The time this label was published. This value has no meaning when the label isn't published. |
| `learn_more_uri` | String |  | Custom URL to present to users to allow them to learn more about this label and how it should be used. |
| `id` | String |  | Output only. Globally unique identifier of this label. ID makes up part of the label `name`, but unlike `name`, ID is consistent between revisions. Matches the regex: `([a-zA-Z0-9])+`. |
| `applied_label_policy` | String |  | Output only. Behavior of this label when it's applied to Drive items. |
| `creator` | String |  | Output only. The user who created this label. |
| `enabled_app_settings` | String |  | Optional. The `EnabledAppSettings` for this Label. |
| `lock_status` | String |  | Output only. The `LockStatus` of this label. |
| `disabler` | String |  | Output only. The user who disabled this label. This value has no meaning when the label isn't disabled. |
| `revision_create_time` | String |  | Output only. The time this label revision was created. |
| `publisher` | String |  | Output only. The user who published this label. This value has no meaning when the label isn't published.>> |
| `applied_capabilities` | String |  | Output only. The capabilities related to this label on applied metadata. |
| `fields` | Vec<String> |  | List of fields in descending priority order. |
| `label_type` | String |  | Required. The type of label. |
| `display_hints` | String |  | Output only. UI display hints for rendering the label. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `customer` | String | Output only. The customer this label belongs to. For example: `customers/123abc789`. |
| `revision_creator` | String | Output only. The user who created this label revision. |
| `revision_id` | String | Output only. Revision ID of the label. Revision ID might be part of the label `name` depending on the request issued. A new revision is created whenever revisioned properties of a label are changed. Matches the regex: `([a-zA-Z0-9])+`. |
| `disable_time` | String | Output only. The time this label was disabled. This value has no meaning when the label isn't disabled. |
| `schema_capabilities` | String | Output only. The capabilities the user has on this label. |
| `create_time` | String | Output only. The time this label was created. |
| `lifecycle` | String | Output only. The lifecycle state of the label including whether it's published, deprecated, and has draft changes. |
| `name` | String | Output only. Resource name of the label. Will be in the form of either: `labels/{id}` or `labels/{id}@{revision_id}` depending on the request. See `id` and `revision_id` below. |
| `properties` | String | Required. The basic properties of the label. |
| `publish_time` | String | Output only. The time this label was published. This value has no meaning when the label isn't published. |
| `learn_more_uri` | String | Custom URL to present to users to allow them to learn more about this label and how it should be used. |
| `id` | String | Output only. Globally unique identifier of this label. ID makes up part of the label `name`, but unlike `name`, ID is consistent between revisions. Matches the regex: `([a-zA-Z0-9])+`. |
| `applied_label_policy` | String | Output only. Behavior of this label when it's applied to Drive items. |
| `creator` | String | Output only. The user who created this label. |
| `enabled_app_settings` | String | Optional. The `EnabledAppSettings` for this Label. |
| `lock_status` | String | Output only. The `LockStatus` of this label. |
| `disabler` | String | Output only. The user who disabled this label. This value has no meaning when the label isn't disabled. |
| `revision_create_time` | String | Output only. The time this label revision was created. |
| `publisher` | String | Output only. The user who published this label. This value has no meaning when the label isn't published.>> |
| `applied_capabilities` | String | Output only. The capabilities related to this label on applied metadata. |
| `fields` | Vec<String> | List of fields in descending priority order. |
| `label_type` | String | Required. The type of label. |
| `display_hints` | String | Output only. UI display hints for rendering the label. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create label
label = provider.drivelabels_api.Label {
}

# Access label outputs
label_id = label.id
label_customer = label.customer
label_revision_creator = label.revision_creator
label_revision_id = label.revision_id
label_disable_time = label.disable_time
label_schema_capabilities = label.schema_capabilities
label_create_time = label.create_time
label_lifecycle = label.lifecycle
label_name = label.name
label_properties = label.properties
label_publish_time = label.publish_time
label_learn_more_uri = label.learn_more_uri
label_id = label.id
label_applied_label_policy = label.applied_label_policy
label_creator = label.creator
label_enabled_app_settings = label.enabled_app_settings
label_lock_status = label.lock_status
label_disabler = label.disabler
label_revision_create_time = label.revision_create_time
label_publisher = label.publisher
label_applied_capabilities = label.applied_capabilities
label_fields = label.fields
label_label_type = label.label_type
label_display_hints = label.display_hints
```

---


### Lock

Lists the label locks on a label.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | The token of the next page in the response. |
| `label_locks` | Vec<String> | Label locks. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access lock outputs
lock_id = lock.id
lock_next_page_token = lock.next_page_token
lock_label_locks = lock.label_locks
```

---


### Permission

Updates a label's permissions. If a permission for the indicated principal doesn't exist, a label permission is created, otherwise the existing permission is updated. Permissions affect the label resource as a whole, aren't revisioned, and don't require publishing.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `person` | String |  | Person resource name. |
| `role` | String |  | The role the principal should have. |
| `email` | String |  | Specifies the email address for a user or group principal. Not populated for audience principals. User and group permissions may only be inserted using an email address. On update requests, if email address is specified, no principal should be specified. |
| `name` | String |  | Resource name of this permission. |
| `group` | String |  | Group resource name. |
| `audience` | String |  | Audience to grant a role to. The magic value of `audiences/default` may be used to apply the role to the default audience in the context of the organization that owns the label. |
| `parent` | String | ✅ | Required. The parent label resource name on the label permission is created. Format: `labels/{label}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `label_permissions` | Vec<String> | Label permissions. |
| `next_page_token` | String | The token of the next page in the response. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create permission
permission = provider.drivelabels_api.Permission {
    parent = "value"  # Required. The parent label resource name on the label permission is created. Format: `labels/{label}`.
}

# Access permission outputs
permission_id = permission.id
permission_label_permissions = permission.label_permissions
permission_next_page_token = permission.next_page_token
```

---


### Revision



**Operations**: ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `person` | String |  | Person resource name. |
| `role` | String |  | The role the principal should have. |
| `email` | String |  | Specifies the email address for a user or group principal. Not populated for audience principals. User and group permissions may only be inserted using an email address. On update requests, if email address is specified, no principal should be specified. |
| `name` | String |  | Resource name of this permission. |
| `group` | String |  | Group resource name. |
| `audience` | String |  | Audience to grant a role to. The magic value of `audiences/default` may be used to apply the role to the default audience in the context of the organization that owns the label. |
| `parent` | String | ✅ | Required. The parent label resource name. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

```

---


### User

Gets the user capabilities.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `can_administrate_labels` | bool | Output only. Whether the user is an administrator for the shared labels feature. |
| `can_create_shared_labels` | bool | Output only. Whether the user is allowed to create shared labels. |
| `name` | String | Output only. Resource name for the user capabilities. |
| `can_access_label_manager` | bool | Output only. Whether the user is allowed access to the label manager. |
| `can_create_admin_labels` | bool | Output only. Whether the user is allowed to create admin labels. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access user outputs
user_id = user.id
user_can_administrate_labels = user.can_administrate_labels
user_can_create_shared_labels = user.can_create_shared_labels
user_name = user.name
user_can_access_label_manager = user.can_access_label_manager
user_can_create_admin_labels = user.can_create_admin_labels
```

---


### Limit

Get the constraints on the structure of a label; such as, the maximum number of fields allowed and maximum length of the label title.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Resource name. |
| `max_draft_revisions` | i64 | The maximum number of draft revisions that will be kept before deleting old drafts. |
| `max_fields` | i64 | The maximum number of fields allowed within the label. |
| `max_description_length` | i64 | The maximum number of characters allowed for the description. |
| `field_limits` | String | The limits for fields. |
| `max_title_length` | i64 | The maximum number of characters allowed for the title. |
| `max_deleted_fields` | i64 | The maximum number of published fields that can be deleted. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access limit outputs
limit_id = limit.id
limit_name = limit.name
limit_max_draft_revisions = limit.max_draft_revisions
limit_max_fields = limit.max_fields
limit_max_description_length = limit.max_description_length
limit_field_limits = limit.field_limits
limit_max_title_length = limit.max_title_length
limit_max_deleted_fields = limit.max_deleted_fields
```

---


### Label

Creates a label. For more information, see [Create and publish a label](https://developers.google.com/workspace/drive/labels/guides/create-label).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Resource name of the label. Will be in the form of either: `labels/{id}` or `labels/{id}@{revision_id}` depending on the request. See `id` and `revision_id` below. |
| `lock_status` | String |  | Output only. The `LockStatus` of this label. |
| `display_hints` | String |  | Output only. UI display hints for rendering the label. |
| `create_time` | String |  | Output only. The time this label was created. |
| `id` | String |  | Output only. Globally unique identifier of this label. ID makes up part of the label `name`, but unlike `name`, ID is consistent between revisions. Matches the regex: `([a-zA-Z0-9])+`. |
| `revision_id` | String |  | Output only. Revision ID of the label. Revision ID might be part of the label `name` depending on the request issued. A new revision is created whenever revisioned properties of a label are changed. Matches the regex: `([a-zA-Z0-9])+`. |
| `creator` | String |  | Output only. The user who created this label. |
| `label_type` | String |  | Required. The type of label. |
| `applied_label_policy` | String |  | Output only. Behavior of this label when it's applied to Drive items. |
| `disabler` | String |  | Output only. The user who disabled this label. This value has no meaning when the label isn't disabled. |
| `revision_creator` | String |  | Output only. The user who created this label revision. |
| `properties` | String |  | Required. The basic properties of the label. |
| `applied_capabilities` | String |  | Output only. The capabilities related to this label on applied metadata. |
| `disable_time` | String |  | Output only. The time this label was disabled. This value has no meaning when the label isn't disabled. |
| `lifecycle` | String |  | Output only. The lifecycle state of the label including whether it's published, deprecated, and has draft changes. |
| `fields` | Vec<String> |  | List of fields in descending priority order. |
| `enabled_app_settings` | String |  | Optional. The `EnabledAppSettings` for this Label. |
| `publish_time` | String |  | Output only. The time this label was published. This value has no meaning when the label isn't published. |
| `learn_more_uri` | String |  | Custom URL to present to users to allow them to learn more about this label and how it should be used. |
| `publisher` | String |  | Output only. The user who published this label. This value has no meaning when the label isn't published.>> |
| `revision_create_time` | String |  | Output only. The time this label revision was created. |
| `schema_capabilities` | String |  | Output only. The capabilities the user has on this label. |
| `customer` | String |  | Output only. The customer this label belongs to. For example: `customers/123abc789`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Resource name of the label. Will be in the form of either: `labels/{id}` or `labels/{id}@{revision_id}` depending on the request. See `id` and `revision_id` below. |
| `lock_status` | String | Output only. The `LockStatus` of this label. |
| `display_hints` | String | Output only. UI display hints for rendering the label. |
| `create_time` | String | Output only. The time this label was created. |
| `id` | String | Output only. Globally unique identifier of this label. ID makes up part of the label `name`, but unlike `name`, ID is consistent between revisions. Matches the regex: `([a-zA-Z0-9])+`. |
| `revision_id` | String | Output only. Revision ID of the label. Revision ID might be part of the label `name` depending on the request issued. A new revision is created whenever revisioned properties of a label are changed. Matches the regex: `([a-zA-Z0-9])+`. |
| `creator` | String | Output only. The user who created this label. |
| `label_type` | String | Required. The type of label. |
| `applied_label_policy` | String | Output only. Behavior of this label when it's applied to Drive items. |
| `disabler` | String | Output only. The user who disabled this label. This value has no meaning when the label isn't disabled. |
| `revision_creator` | String | Output only. The user who created this label revision. |
| `properties` | String | Required. The basic properties of the label. |
| `applied_capabilities` | String | Output only. The capabilities related to this label on applied metadata. |
| `disable_time` | String | Output only. The time this label was disabled. This value has no meaning when the label isn't disabled. |
| `lifecycle` | String | Output only. The lifecycle state of the label including whether it's published, deprecated, and has draft changes. |
| `fields` | Vec<String> | List of fields in descending priority order. |
| `enabled_app_settings` | String | Optional. The `EnabledAppSettings` for this Label. |
| `publish_time` | String | Output only. The time this label was published. This value has no meaning when the label isn't published. |
| `learn_more_uri` | String | Custom URL to present to users to allow them to learn more about this label and how it should be used. |
| `publisher` | String | Output only. The user who published this label. This value has no meaning when the label isn't published.>> |
| `revision_create_time` | String | Output only. The time this label revision was created. |
| `schema_capabilities` | String | Output only. The capabilities the user has on this label. |
| `customer` | String | Output only. The customer this label belongs to. For example: `customers/123abc789`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create label
label = provider.drivelabels_api.Label {
}

# Access label outputs
label_id = label.id
label_name = label.name
label_lock_status = label.lock_status
label_display_hints = label.display_hints
label_create_time = label.create_time
label_id = label.id
label_revision_id = label.revision_id
label_creator = label.creator
label_label_type = label.label_type
label_applied_label_policy = label.applied_label_policy
label_disabler = label.disabler
label_revision_creator = label.revision_creator
label_properties = label.properties
label_applied_capabilities = label.applied_capabilities
label_disable_time = label.disable_time
label_lifecycle = label.lifecycle
label_fields = label.fields
label_enabled_app_settings = label.enabled_app_settings
label_publish_time = label.publish_time
label_learn_more_uri = label.learn_more_uri
label_publisher = label.publisher
label_revision_create_time = label.revision_create_time
label_schema_capabilities = label.schema_capabilities
label_customer = label.customer
```

---


### Lock

Lists the label locks on a label.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `label_locks` | Vec<String> | Label locks. |
| `next_page_token` | String | The token of the next page in the response. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access lock outputs
lock_id = lock.id
lock_label_locks = lock.label_locks
lock_next_page_token = lock.next_page_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple revision resources
revision_0 = provider.drivelabels_api.Revision {
    parent = "value-0"
}
revision_1 = provider.drivelabels_api.Revision {
    parent = "value-1"
}
revision_2 = provider.drivelabels_api.Revision {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    revision = provider.drivelabels_api.Revision {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Drivelabels_api Documentation](https://cloud.google.com/drivelabels_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
