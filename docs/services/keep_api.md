# Keep_api Service



**Resources**: 3

---

## Overview

The keep_api service provides access to 3 resource types:

- [Media](#media) [R]
- [Note](#note) [CRD]
- [Permission](#permission) [C]

---

## Resources


### Media

Gets an attachment. To download attachment media via REST requires the alt=media query parameter. Returns a 400 bad request error if attachment media is not available in the requested MIME type.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `mime_type` | Vec<String> | The MIME types (IANA media types) in which the attachment is available. |
| `name` | String | The resource name; |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access media outputs
media_id = media.id
media_mime_type = media.mime_type
media_name = media.name
```

---


### Note

Creates a new note.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. When this note was last modified. |
| `name` | String |  | Output only. The resource name of this note. See general note on identifiers in KeepService. |
| `attachments` | Vec<String> |  | Output only. The attachments attached to this note. |
| `permissions` | Vec<String> |  | Output only. The list of permissions set on the note. Contains at least one entry for the note owner. |
| `body` | String |  | The body of the note. |
| `create_time` | String |  | Output only. When this note was created. |
| `trashed` | bool |  | Output only. `true` if this note has been trashed. If trashed, the note is eventually deleted. |
| `title` | String |  | The title of the note. Length must be less than 1,000 characters. |
| `trash_time` | String |  | Output only. When this note was trashed. If `trashed`, the note is eventually deleted. If the note is not trashed, this field is not set (and the trashed field is `false`). |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. When this note was last modified. |
| `name` | String | Output only. The resource name of this note. See general note on identifiers in KeepService. |
| `attachments` | Vec<String> | Output only. The attachments attached to this note. |
| `permissions` | Vec<String> | Output only. The list of permissions set on the note. Contains at least one entry for the note owner. |
| `body` | String | The body of the note. |
| `create_time` | String | Output only. When this note was created. |
| `trashed` | bool | Output only. `true` if this note has been trashed. If trashed, the note is eventually deleted. |
| `title` | String | The title of the note. Length must be less than 1,000 characters. |
| `trash_time` | String | Output only. When this note was trashed. If `trashed`, the note is eventually deleted. If the note is not trashed, this field is not set (and the trashed field is `false`). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create note
note = provider.keep_api.Note {
}

# Access note outputs
note_id = note.id
note_update_time = note.update_time
note_name = note.name
note_attachments = note.attachments
note_permissions = note.permissions
note_body = note.body
note_create_time = note.create_time
note_trashed = note.trashed
note_title = note.title
note_trash_time = note.trash_time
```

---


### Permission

Deletes one or more permissions on the note. The specified entities will immediately lose access. A permission with the `OWNER` role can't be removed. If removing a permission fails, then the entire request fails and no changes are made. Returns a 400 bad request error if a specified permission does not exist on the note.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `names` | Vec<String> |  | Required. The names of the permissions to delete. Format: `notes/{note}/permissions/{permission}` |
| `parent` | String | ✅ | The parent resource shared by all permissions being deleted. Format: `notes/{note}` If this is set, the parent of all of the permissions specified in the DeletePermissionRequest messages must match this field. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create permission
permission = provider.keep_api.Permission {
    parent = "value"  # The parent resource shared by all permissions being deleted. Format: `notes/{note}` If this is set, the parent of all of the permissions specified in the DeletePermissionRequest messages must match this field.
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

# Create multiple media resources
media_0 = provider.keep_api.Media {
}
media_1 = provider.keep_api.Media {
}
media_2 = provider.keep_api.Media {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    media = provider.keep_api.Media {
    }
```

---

## Related Documentation

- [GCP Keep_api Documentation](https://cloud.google.com/keep_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
