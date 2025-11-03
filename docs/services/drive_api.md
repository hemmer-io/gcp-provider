# Drive_api Service



**Resources**: 27

---

## Overview

The drive_api service provides access to 27 resource types:

- [Change](#change) [CR]
- [Revision](#revision) [RUD]
- [App](#app) [R]
- [About](#about) [R]
- [Teamdrive](#teamdrive) [CRUD]
- [Drive](#drive) [CRUD]
- [Propertie](#propertie) [CRUD]
- [Parent](#parent) [CRD]
- [Channel](#channel) [C]
- [Replie](#replie) [CRUD]
- [Children](#children) [CRD]
- [Permission](#permission) [CRUD]
- [Comment](#comment) [CRUD]
- [File](#file) [CRUD]
- [Drive](#drive) [CRUD]
- [About](#about) [R]
- [Revision](#revision) [RUD]
- [Replie](#replie) [CRUD]
- [Permission](#permission) [CRUD]
- [Operation](#operation) [R]
- [Accessproposal](#accessproposal) [CR]
- [Channel](#channel) [C]
- [File](#file) [CRUD]
- [Teamdrive](#teamdrive) [CRUD]
- [App](#app) [R]
- [Comment](#comment) [CRUD]
- [Change](#change) [CR]

---

## Resources


### Change

Subscribe to changes for a user.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `payload` | bool |  | A Boolean value to indicate whether payload is wanted. Optional. |
| `params` | HashMap<String, String> |  | Additional parameters controlling delivery channel behavior. Optional. |
| `kind` | String |  | Identifies this as a notification channel used to watch for changes to a resource, which is `api#channel`. |
| `expiration` | String |  | Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional. |
| `token` | String |  | An arbitrary string delivered to the target address with each notification delivered over this channel. Optional. |
| `type` | String |  | The type of delivery mechanism used for this channel. Valid values are "web_hook" or "webhook". |
| `resource_id` | String |  | An opaque ID that identifies the resource being watched on this channel. Stable across different API versions. |
| `address` | String |  | The address where notifications are delivered for this channel. |
| `resource_uri` | String |  | A version-specific identifier for the watched resource. |
| `id` | String |  | A UUID or similar unique string that identifies this channel. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `drive` | String | The updated state of the shared drive. Present if the changeType is drive, the user is still a member of the shared drive, and the shared drive has not been deleted. |
| `deleted` | bool | Whether the file or shared drive has been removed from this list of changes, for example by deletion or loss of access. |
| `modification_date` | String | The time of this modification. |
| `change_type` | String | The type of the change. Possible values are `file` and `drive`. |
| `file` | String | The updated state of the file. Present if the type is file and the file has not been removed from this list of changes. |
| `self_link` | String | A link back to this change. |
| `team_drive_id` | String | Deprecated: Use `driveId` instead. |
| `type` | String | Deprecated: Use `changeType` instead. |
| `kind` | String | This is always `drive#change`. |
| `id` | String | The ID of the change. |
| `team_drive` | String | Deprecated: Use `drive` instead. |
| `drive_id` | String | The ID of the shared drive associated with this change. |
| `file_id` | String | The ID of the file associated with this change. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create change
change = provider.drive_api.Change {
}

# Access change outputs
change_id = change.id
change_drive = change.drive
change_deleted = change.deleted
change_modification_date = change.modification_date
change_change_type = change.change_type
change_file = change.file
change_self_link = change.self_link
change_team_drive_id = change.team_drive_id
change_type = change.type
change_kind = change.kind
change_id = change.id
change_team_drive = change.team_drive
change_drive_id = change.drive_id
change_file_id = change.file_id
```

---


### Revision

Gets a specific revision.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `etag` | String |  | Output only. The ETag of the revision. |
| `publish_auto` | bool |  | Whether subsequent revisions will be automatically republished. This is only populated and can only be modified for Docs Editors files. |
| `self_link` | String |  | Output only. A link back to this revision. |
| `modified_date` | String |  | Last time this revision was modified (formatted RFC 3339 timestamp). |
| `kind` | String |  | Output only. This is always `drive#revision`. |
| `last_modifying_user_name` | String |  | Output only. Name of the last user to modify this revision. |
| `file_size` | String |  | Output only. The size of the revision in bytes. This will only be populated on files with content stored in Drive. |
| `last_modifying_user` | String |  | Output only. The last user to modify this revision. This field is only populated when the last modification was performed by a signed-in user. |
| `md5_checksum` | String |  | Output only. An MD5 checksum for the content of this revision. This will only be populated on files with content stored in Drive. |
| `original_filename` | String |  | Output only. The original filename when this revision was created. This will only be populated on files with content stored in Drive. |
| `pinned` | bool |  | Whether this revision is pinned to prevent automatic purging. If not set, the revision is automatically purged 30 days after newer content is uploaded. This field can only be modified on files with content stored in Drive, excluding Docs Editors files. Revisions can also be pinned when they are created through the drive.files.insert/update/copy by using the pinned query parameter. Pinned revisions are stored indefinitely using additional storage quota, up to a maximum of 200 revisions. |
| `mime_type` | String |  | Output only. The MIME type of the revision. |
| `download_url` | String |  | Output only. Short term download URL for the file. This will only be populated on files with content stored in Drive. |
| `published_link` | String |  | Output only. A link to the published revision. This is only populated for Docs Editors files. |
| `published` | bool |  | Whether this revision is published. This is only populated and can only be modified for Docs Editors files. |
| `published_outside_domain` | bool |  | Whether this revision is published outside the domain. This is only populated and can only be modified for Docs Editors files. |
| `id` | String |  | Output only. The ID of the revision. |
| `export_links` | HashMap<String, String> |  | Output only. Links for exporting Docs Editors files to specific formats. |
| `file_id` | String | ✅ | The ID for the file. |
| `revision_id` | String | ✅ | The ID for the revision. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | Output only. The ETag of the revision. |
| `publish_auto` | bool | Whether subsequent revisions will be automatically republished. This is only populated and can only be modified for Docs Editors files. |
| `self_link` | String | Output only. A link back to this revision. |
| `modified_date` | String | Last time this revision was modified (formatted RFC 3339 timestamp). |
| `kind` | String | Output only. This is always `drive#revision`. |
| `last_modifying_user_name` | String | Output only. Name of the last user to modify this revision. |
| `file_size` | String | Output only. The size of the revision in bytes. This will only be populated on files with content stored in Drive. |
| `last_modifying_user` | String | Output only. The last user to modify this revision. This field is only populated when the last modification was performed by a signed-in user. |
| `md5_checksum` | String | Output only. An MD5 checksum for the content of this revision. This will only be populated on files with content stored in Drive. |
| `original_filename` | String | Output only. The original filename when this revision was created. This will only be populated on files with content stored in Drive. |
| `pinned` | bool | Whether this revision is pinned to prevent automatic purging. If not set, the revision is automatically purged 30 days after newer content is uploaded. This field can only be modified on files with content stored in Drive, excluding Docs Editors files. Revisions can also be pinned when they are created through the drive.files.insert/update/copy by using the pinned query parameter. Pinned revisions are stored indefinitely using additional storage quota, up to a maximum of 200 revisions. |
| `mime_type` | String | Output only. The MIME type of the revision. |
| `download_url` | String | Output only. Short term download URL for the file. This will only be populated on files with content stored in Drive. |
| `published_link` | String | Output only. A link to the published revision. This is only populated for Docs Editors files. |
| `published` | bool | Whether this revision is published. This is only populated and can only be modified for Docs Editors files. |
| `published_outside_domain` | bool | Whether this revision is published outside the domain. This is only populated and can only be modified for Docs Editors files. |
| `id` | String | Output only. The ID of the revision. |
| `export_links` | HashMap<String, String> | Output only. Links for exporting Docs Editors files to specific formats. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access revision outputs
revision_id = revision.id
revision_etag = revision.etag
revision_publish_auto = revision.publish_auto
revision_self_link = revision.self_link
revision_modified_date = revision.modified_date
revision_kind = revision.kind
revision_last_modifying_user_name = revision.last_modifying_user_name
revision_file_size = revision.file_size
revision_last_modifying_user = revision.last_modifying_user
revision_md5_checksum = revision.md5_checksum
revision_original_filename = revision.original_filename
revision_pinned = revision.pinned
revision_mime_type = revision.mime_type
revision_download_url = revision.download_url
revision_published_link = revision.published_link
revision_published = revision.published
revision_published_outside_domain = revision.published_outside_domain
revision_id = revision.id
revision_export_links = revision.export_links
```

---


### App

Gets a specific app.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `has_drive_wide_scope` | bool | Whether the app has drive-wide scope. An app with drive-wide scope can access all files in the user's drive. |
| `primary_mime_types` | Vec<String> | The list of primary mime types. |
| `create_in_folder_template` | String | The template url to create a new file with this app in a given folder. The template will contain {folderId} to be replaced by the folder to create the new file in. |
| `product_id` | String | The ID of the product listing for this app. |
| `create_url` | String | The url to create a new file with this app. |
| `supports_import` | bool | Whether this app supports importing from Docs Editors. |
| `primary_file_extensions` | Vec<String> | The list of primary file extensions. |
| `supports_offline_create` | bool | Whether this app supports creating new files when offline. |
| `authorized` | bool | Whether the app is authorized to access data on the user's Drive. |
| `use_by_default` | bool | Whether the app is selected as the default handler for the types it supports. |
| `product_url` | String | A link to the product listing for this app. |
| `short_description` | String | A short description of the app. |
| `id` | String | The ID of the app. |
| `supports_multi_open` | bool | Whether this app supports opening more than one file. |
| `object_type` | String | The type of object this app creates (e.g. Chart). If empty, the app name should be used instead. |
| `name` | String | The name of the app. |
| `kind` | String | This is always `drive#app`. |
| `icons` | Vec<String> | The various icons for the app. |
| `installed` | bool | Whether the app is installed. |
| `long_description` | String | A long description of the app. |
| `open_url_template` | String | The template url for opening files with this app. The template will contain `{ids}` and/or `{exportIds}` to be replaced by the actual file ids. See Open Files for the full documentation. |
| `secondary_file_extensions` | Vec<String> | The list of secondary file extensions. |
| `secondary_mime_types` | Vec<String> | The list of secondary mime types. |
| `supports_create` | bool | Whether this app supports creating new objects. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access app outputs
app_id = app.id
app_has_drive_wide_scope = app.has_drive_wide_scope
app_primary_mime_types = app.primary_mime_types
app_create_in_folder_template = app.create_in_folder_template
app_product_id = app.product_id
app_create_url = app.create_url
app_supports_import = app.supports_import
app_primary_file_extensions = app.primary_file_extensions
app_supports_offline_create = app.supports_offline_create
app_authorized = app.authorized
app_use_by_default = app.use_by_default
app_product_url = app.product_url
app_short_description = app.short_description
app_id = app.id
app_supports_multi_open = app.supports_multi_open
app_object_type = app.object_type
app_name = app.name
app_kind = app.kind
app_icons = app.icons
app_installed = app.installed
app_long_description = app.long_description
app_open_url_template = app.open_url_template
app_secondary_file_extensions = app.secondary_file_extensions
app_secondary_mime_types = app.secondary_mime_types
app_supports_create = app.supports_create
```

---


### About

Gets the information about the current user along with Drive API settings

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `import_formats` | Vec<String> | The allowable import formats. |
| `features` | Vec<String> | List of additional features enabled on this account. |
| `is_current_app_installed` | bool | A boolean indicating whether the authenticated app is installed by the authenticated user. |
| `can_create_drives` | bool | Whether the user can create shared drives. |
| `drive_themes` | Vec<String> | A list of themes that are supported for shared drives. |
| `permission_id` | String | The current user's ID as visible in the permissions collection. |
| `quota_bytes_used_aggregate` | String | The number of quota bytes used by all Google apps (Drive, Picasa, etc.). |
| `team_drive_themes` | Vec<String> | Deprecated: Use `driveThemes` instead. |
| `quota_bytes_by_service` | Vec<String> | The amount of storage quota used by different Google services. |
| `user` | String | The authenticated user. |
| `name` | String | The name of the current user. |
| `folder_color_palette` | Vec<String> | The palette of allowable folder colors as RGB hex strings. |
| `root_folder_id` | String | The id of the root folder. |
| `etag` | String | The ETag of the item. |
| `kind` | String | This is always `drive#about`. |
| `language_code` | String | The user's language or locale code, as defined by BCP 47, with some extensions from Unicode's LDML format (http://www.unicode.org/reports/tr35/). |
| `largest_change_id` | String | The largest change id. |
| `quota_bytes_total` | String | The total number of quota bytes. This is only relevant when quotaType is LIMITED. |
| `remaining_change_ids` | String | The number of remaining change ids, limited to no more than 2500. |
| `quota_bytes_used` | String | The number of quota bytes used by Google Drive. |
| `self_link` | String | A link back to this item. |
| `export_formats` | Vec<String> | The allowable export formats. |
| `can_create_team_drives` | bool | Deprecated: Use `canCreateDrives` instead. |
| `domain_sharing_policy` | String | The domain sharing policy for the current user. Possible values are: * `allowed` * `allowedWithWarning` * `incomingOnly` * `disallowed` |
| `additional_role_info` | Vec<String> | Information about supported additional roles per file type. The most specific type takes precedence. |
| `max_upload_sizes` | Vec<String> | List of max upload sizes for each file type. The most specific type takes precedence. |
| `quota_bytes_used_in_trash` | String | The number of quota bytes used by trashed items. |
| `quota_type` | String | The type of the user's storage quota. Possible values are: * `LIMITED` * `UNLIMITED` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access about outputs
about_id = about.id
about_import_formats = about.import_formats
about_features = about.features
about_is_current_app_installed = about.is_current_app_installed
about_can_create_drives = about.can_create_drives
about_drive_themes = about.drive_themes
about_permission_id = about.permission_id
about_quota_bytes_used_aggregate = about.quota_bytes_used_aggregate
about_team_drive_themes = about.team_drive_themes
about_quota_bytes_by_service = about.quota_bytes_by_service
about_user = about.user
about_name = about.name
about_folder_color_palette = about.folder_color_palette
about_root_folder_id = about.root_folder_id
about_etag = about.etag
about_kind = about.kind
about_language_code = about.language_code
about_largest_change_id = about.largest_change_id
about_quota_bytes_total = about.quota_bytes_total
about_remaining_change_ids = about.remaining_change_ids
about_quota_bytes_used = about.quota_bytes_used
about_self_link = about.self_link
about_export_formats = about.export_formats
about_can_create_team_drives = about.can_create_team_drives
about_domain_sharing_policy = about.domain_sharing_policy
about_additional_role_info = about.additional_role_info
about_max_upload_sizes = about.max_upload_sizes
about_quota_bytes_used_in_trash = about.quota_bytes_used_in_trash
about_quota_type = about.quota_type
```

---


### Teamdrive

Deprecated: Use `drives.insert` instead.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `restrictions` | String |  | A set of restrictions that apply to this Team Drive or items inside this Team Drive. |
| `theme_id` | String |  | The ID of the theme from which the background image and color will be set. The set of possible `teamDriveThemes` can be retrieved from a `drive.about.get` response. When not specified on a `drive.teamdrives.insert` request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set `colorRgb` or `backgroundImageFile`. |
| `background_image_link` | String |  | A short-lived link to this Team Drive's background image. |
| `capabilities` | String |  | Capabilities the current user has on this Team Drive. |
| `org_unit_id` | String |  | The organizational unit of this shared drive. This field is only populated on `drives.list` responses when the `useDomainAdminAccess` parameter is set to `true`. |
| `created_date` | String |  | The time at which the Team Drive was created (RFC 3339 date-time). |
| `background_image_file` | String |  | An image file and cropping parameters from which a background image for this Team Drive is set. This is a write only field; it can only be set on `drive.teamdrives.update` requests that don't set `themeId`. When specified, all fields of the `backgroundImageFile` must be set. |
| `color_rgb` | String |  | The color of this Team Drive as an RGB hex string. It can only be set on a `drive.teamdrives.update` request that does not set `themeId`. |
| `id` | String |  | The ID of this Team Drive which is also the ID of the top level folder of this Team Drive. |
| `kind` | String |  | This is always `drive#teamDrive` |
| `name` | String |  | The name of this Team Drive. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `restrictions` | String | A set of restrictions that apply to this Team Drive or items inside this Team Drive. |
| `theme_id` | String | The ID of the theme from which the background image and color will be set. The set of possible `teamDriveThemes` can be retrieved from a `drive.about.get` response. When not specified on a `drive.teamdrives.insert` request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set `colorRgb` or `backgroundImageFile`. |
| `background_image_link` | String | A short-lived link to this Team Drive's background image. |
| `capabilities` | String | Capabilities the current user has on this Team Drive. |
| `org_unit_id` | String | The organizational unit of this shared drive. This field is only populated on `drives.list` responses when the `useDomainAdminAccess` parameter is set to `true`. |
| `created_date` | String | The time at which the Team Drive was created (RFC 3339 date-time). |
| `background_image_file` | String | An image file and cropping parameters from which a background image for this Team Drive is set. This is a write only field; it can only be set on `drive.teamdrives.update` requests that don't set `themeId`. When specified, all fields of the `backgroundImageFile` must be set. |
| `color_rgb` | String | The color of this Team Drive as an RGB hex string. It can only be set on a `drive.teamdrives.update` request that does not set `themeId`. |
| `id` | String | The ID of this Team Drive which is also the ID of the top level folder of this Team Drive. |
| `kind` | String | This is always `drive#teamDrive` |
| `name` | String | The name of this Team Drive. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create teamdrive
teamdrive = provider.drive_api.Teamdrive {
}

# Access teamdrive outputs
teamdrive_id = teamdrive.id
teamdrive_restrictions = teamdrive.restrictions
teamdrive_theme_id = teamdrive.theme_id
teamdrive_background_image_link = teamdrive.background_image_link
teamdrive_capabilities = teamdrive.capabilities
teamdrive_org_unit_id = teamdrive.org_unit_id
teamdrive_created_date = teamdrive.created_date
teamdrive_background_image_file = teamdrive.background_image_file
teamdrive_color_rgb = teamdrive.color_rgb
teamdrive_id = teamdrive.id
teamdrive_kind = teamdrive.kind
teamdrive_name = teamdrive.name
```

---


### Drive

Creates a new shared drive.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `capabilities` | String |  | Output only. Capabilities the current user has on this shared drive. |
| `color_rgb` | String |  | The color of this shared drive as an RGB hex string. It can only be set on a `drive.drives.update` request that does not set `themeId`. |
| `theme_id` | String |  | The ID of the theme from which the background image and color will be set. The set of possible `driveThemes` can be retrieved from a `drive.about.get` response. When not specified on a `drive.drives.insert` request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set `colorRgb` or `backgroundImageFile`. |
| `background_image_link` | String |  | Output only. A short-lived link to this shared drive's background image. |
| `hidden` | bool |  | Whether the shared drive is hidden from default view. |
| `kind` | String |  | Output only. This is always `drive#drive` |
| `id` | String |  | Output only. The ID of this shared drive which is also the ID of the top level folder of this shared drive. |
| `restrictions` | String |  | A set of restrictions that apply to this shared drive or items inside this shared drive. |
| `name` | String |  | The name of this shared drive. |
| `background_image_file` | String |  | An image file and cropping parameters from which a background image for this shared drive is set. This is a write only field; it can only be set on `drive.drives.update` requests that don't set `themeId`. When specified, all fields of the `backgroundImageFile` must be set. |
| `created_date` | String |  | The time at which the shared drive was created (RFC 3339 date-time). |
| `org_unit_id` | String |  | Output only. The organizational unit of this shared drive. This field is only populated on `drives.list` responses when the `useDomainAdminAccess` parameter is set to `true`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `capabilities` | String | Output only. Capabilities the current user has on this shared drive. |
| `color_rgb` | String | The color of this shared drive as an RGB hex string. It can only be set on a `drive.drives.update` request that does not set `themeId`. |
| `theme_id` | String | The ID of the theme from which the background image and color will be set. The set of possible `driveThemes` can be retrieved from a `drive.about.get` response. When not specified on a `drive.drives.insert` request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set `colorRgb` or `backgroundImageFile`. |
| `background_image_link` | String | Output only. A short-lived link to this shared drive's background image. |
| `hidden` | bool | Whether the shared drive is hidden from default view. |
| `kind` | String | Output only. This is always `drive#drive` |
| `id` | String | Output only. The ID of this shared drive which is also the ID of the top level folder of this shared drive. |
| `restrictions` | String | A set of restrictions that apply to this shared drive or items inside this shared drive. |
| `name` | String | The name of this shared drive. |
| `background_image_file` | String | An image file and cropping parameters from which a background image for this shared drive is set. This is a write only field; it can only be set on `drive.drives.update` requests that don't set `themeId`. When specified, all fields of the `backgroundImageFile` must be set. |
| `created_date` | String | The time at which the shared drive was created (RFC 3339 date-time). |
| `org_unit_id` | String | Output only. The organizational unit of this shared drive. This field is only populated on `drives.list` responses when the `useDomainAdminAccess` parameter is set to `true`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create drive
drive = provider.drive_api.Drive {
}

# Access drive outputs
drive_id = drive.id
drive_capabilities = drive.capabilities
drive_color_rgb = drive.color_rgb
drive_theme_id = drive.theme_id
drive_background_image_link = drive.background_image_link
drive_hidden = drive.hidden
drive_kind = drive.kind
drive_id = drive.id
drive_restrictions = drive.restrictions
drive_name = drive.name
drive_background_image_file = drive.background_image_file
drive_created_date = drive.created_date
drive_org_unit_id = drive.org_unit_id
```

---


### Propertie

Adds a property to a file, or updates it if it already exists.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `visibility` | String |  | The visibility of this property. Allowed values are PRIVATE (default) and PUBLIC. Private properties can only be retrieved using an authenticated request. An authenticated request uses an access token obtained with a OAuth 2 client ID. You cannot use an API key to retrieve private properties. |
| `etag` | String |  | Output only. ETag of the property. |
| `kind` | String |  | Output only. This is always `drive#property`. |
| `value` | String |  | The value of this property. |
| `key` | String |  | The key of this property. |
| `self_link` | String |  | Output only. The link back to this property. |
| `file_id` | String | ✅ | The ID of the file. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `visibility` | String | The visibility of this property. Allowed values are PRIVATE (default) and PUBLIC. Private properties can only be retrieved using an authenticated request. An authenticated request uses an access token obtained with a OAuth 2 client ID. You cannot use an API key to retrieve private properties. |
| `etag` | String | Output only. ETag of the property. |
| `kind` | String | Output only. This is always `drive#property`. |
| `value` | String | The value of this property. |
| `key` | String | The key of this property. |
| `self_link` | String | Output only. The link back to this property. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create propertie
propertie = provider.drive_api.Propertie {
    file_id = "value"  # The ID of the file.
}

# Access propertie outputs
propertie_id = propertie.id
propertie_visibility = propertie.visibility
propertie_etag = propertie.etag
propertie_kind = propertie.kind
propertie_value = propertie.value
propertie_key = propertie.key
propertie_self_link = propertie.self_link
```

---


### Parent

Adds a parent folder for a file.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `self_link` | String |  | Output only. A link back to this reference. |
| `id` | String |  | The ID of the parent. |
| `parent_link` | String |  | Output only. A link to the parent. |
| `is_root` | bool |  | Output only. Whether or not the parent is the root folder. |
| `kind` | String |  | Output only. This is always `drive#parentReference`. |
| `file_id` | String | ✅ | The ID of the file. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `self_link` | String | Output only. A link back to this reference. |
| `id` | String | The ID of the parent. |
| `parent_link` | String | Output only. A link to the parent. |
| `is_root` | bool | Output only. Whether or not the parent is the root folder. |
| `kind` | String | Output only. This is always `drive#parentReference`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create parent
parent = provider.drive_api.Parent {
    file_id = "value"  # The ID of the file.
}

# Access parent outputs
parent_id = parent.id
parent_self_link = parent.self_link
parent_id = parent.id
parent_parent_link = parent.parent_link
parent_is_root = parent.is_root
parent_kind = parent.kind
```

---


### Channel

Stops watching resources through this channel.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `payload` | bool |  | A Boolean value to indicate whether payload is wanted. Optional. |
| `params` | HashMap<String, String> |  | Additional parameters controlling delivery channel behavior. Optional. |
| `kind` | String |  | Identifies this as a notification channel used to watch for changes to a resource, which is `api#channel`. |
| `expiration` | String |  | Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional. |
| `token` | String |  | An arbitrary string delivered to the target address with each notification delivered over this channel. Optional. |
| `type` | String |  | The type of delivery mechanism used for this channel. Valid values are "web_hook" or "webhook". |
| `resource_id` | String |  | An opaque ID that identifies the resource being watched on this channel. Stable across different API versions. |
| `address` | String |  | The address where notifications are delivered for this channel. |
| `resource_uri` | String |  | A version-specific identifier for the watched resource. |
| `id` | String |  | A UUID or similar unique string that identifies this channel. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create channel
channel = provider.drive_api.Channel {
}

```

---


### Replie

Creates a new reply to the given comment.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `content` | String |  | The plain text content used to create this reply. This is not HTML safe and should only be used as a starting point to make edits to a reply's content. This field is required on inserts if no verb is specified (resolve/reopen). |
| `created_date` | String |  | The date when this reply was first created. |
| `modified_date` | String |  | The date when this reply was last modified. |
| `reply_id` | String |  | The ID of the reply. |
| `deleted` | bool |  | Whether this reply has been deleted. If a reply has been deleted the content will be cleared and this will only represent a reply that once existed. |
| `author` | String |  | The user who wrote this reply. |
| `html_content` | String |  | HTML formatted content for this reply. |
| `kind` | String |  | This is always drive#commentReply. |
| `verb` | String |  | The action this reply performed to the parent comment. When creating a new reply this is the action to be perform tSo the parent comment. Possible values are: * `resolve` - To resolve a comment. * `reopen` - To reopen (un-resolve) a comment. |
| `file_id` | String | ✅ | The ID of the file. |
| `comment_id` | String | ✅ | The ID of the comment. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `content` | String | The plain text content used to create this reply. This is not HTML safe and should only be used as a starting point to make edits to a reply's content. This field is required on inserts if no verb is specified (resolve/reopen). |
| `created_date` | String | The date when this reply was first created. |
| `modified_date` | String | The date when this reply was last modified. |
| `reply_id` | String | The ID of the reply. |
| `deleted` | bool | Whether this reply has been deleted. If a reply has been deleted the content will be cleared and this will only represent a reply that once existed. |
| `author` | String | The user who wrote this reply. |
| `html_content` | String | HTML formatted content for this reply. |
| `kind` | String | This is always drive#commentReply. |
| `verb` | String | The action this reply performed to the parent comment. When creating a new reply this is the action to be perform tSo the parent comment. Possible values are: * `resolve` - To resolve a comment. * `reopen` - To reopen (un-resolve) a comment. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create replie
replie = provider.drive_api.Replie {
    file_id = "value"  # The ID of the file.
    comment_id = "value"  # The ID of the comment.
}

# Access replie outputs
replie_id = replie.id
replie_content = replie.content
replie_created_date = replie.created_date
replie_modified_date = replie.modified_date
replie_reply_id = replie.reply_id
replie_deleted = replie.deleted
replie_author = replie.author
replie_html_content = replie.html_content
replie_kind = replie.kind
replie_verb = replie.verb
```

---


### Children

Inserts a file into a folder.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `self_link` | String |  | Output only. A link back to this reference. |
| `kind` | String |  | Output only. This is always `drive#childReference`. |
| `id` | String |  | The ID of the child. |
| `child_link` | String |  | Output only. A link to the child. |
| `folder_id` | String | ✅ | The ID of the folder. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `self_link` | String | Output only. A link back to this reference. |
| `kind` | String | Output only. This is always `drive#childReference`. |
| `id` | String | The ID of the child. |
| `child_link` | String | Output only. A link to the child. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create children
children = provider.drive_api.Children {
    folder_id = "value"  # The ID of the folder.
}

# Access children outputs
children_id = children.id
children_self_link = children.self_link
children_kind = children.kind
children_id = children.id
children_child_link = children.child_link
```

---


### Permission

Inserts a permission for a file or shared drive. **Warning:** Concurrent permissions operations on the same file are not supported; only the last update is applied.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `pending_owner` | bool |  | Whether the account associated with this permission is a pending owner. Only populated for `user` type permissions for files that are not in a shared drive. |
| `email_address` | String |  | Output only. The email address of the user or group this permission refers to. This is an output-only field which is present when the permission type is `user` or `group`. |
| `permission_details` | Vec<String> |  | Output only. Details of whether the permissions on this item are inherited or directly on this item. |
| `auth_key` | String |  | Output only. Deprecated. |
| `id` | String |  | The ID of the user this permission refers to, and identical to the `permissionId` in the About and Files resources. When making a `drive.permissions.insert` request, exactly one of the `id` or `value` fields must be specified unless the permission type is `anyone`, in which case both `id` and `value` are ignored. |
| `inherited_permissions_disabled` | bool |  | When true, only organizers, owners, and users with permissions added directly on the item can access it. |
| `etag` | String |  | Output only. The ETag of the permission. |
| `kind` | String |  | Output only. This is always `drive#permission`. |
| `photo_link` | String |  | Output only. A link to the profile photo, if available. |
| `team_drive_permission_details` | Vec<String> |  | Output only. Deprecated: Use `permissionDetails` instead. |
| `value` | String |  | The email address or domain name for the entity. This is used during inserts and is not populated in responses. When making a `drive.permissions.insert` request, exactly one of the `id` or `value` fields must be specified unless the permission type is `anyone`, in which case both `id` and `value` are ignored. |
| `domain` | String |  | Output only. The domain name of the entity this permission refers to. This is an output-only field which is present when the permission type is `user`, `group` or `domain`. |
| `name` | String |  | Output only. The name for this permission. |
| `self_link` | String |  | Output only. A link back to this permission. |
| `additional_roles` | Vec<String> |  | Additional roles for this user. Only `commenter` is currently allowed, though more may be supported in the future. |
| `type` | String |  | The account type. Allowed values are: * `user` * `group` * `domain` * `anyone` |
| `expiration_date` | String |  | The time at which this permission will expire (RFC 3339 date-time). Expiration dates have the following restrictions: - They can only be set on user and group permissions - The date must be in the future - The date cannot be more than a year in the future - The date can only be set on drive.permissions.update or drive.permissions.patch requests |
| `view` | String |  | Indicates the view for this permission. Only populated for permissions that belong to a view. published and metadata are the only supported values. - published: The permission's role is published_reader. - metadata: The item is only visible to the metadata view because the item has limited access and the scope has at least read access to the parent. Note: The metadata view is currently only supported on folders.  |
| `with_link` | bool |  | Whether the link is required for this permission. |
| `role` | String |  | The primary role for this user. While new values may be supported in the future, the following are currently allowed: * `owner` * `organizer` * `fileOrganizer` * `writer` * `reader` |
| `deleted` | bool |  | Output only. Whether the account associated with this permission has been deleted. This field only pertains to user and group permissions. |
| `file_id` | String | ✅ | The ID for the file or shared drive. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `pending_owner` | bool | Whether the account associated with this permission is a pending owner. Only populated for `user` type permissions for files that are not in a shared drive. |
| `email_address` | String | Output only. The email address of the user or group this permission refers to. This is an output-only field which is present when the permission type is `user` or `group`. |
| `permission_details` | Vec<String> | Output only. Details of whether the permissions on this item are inherited or directly on this item. |
| `auth_key` | String | Output only. Deprecated. |
| `id` | String | The ID of the user this permission refers to, and identical to the `permissionId` in the About and Files resources. When making a `drive.permissions.insert` request, exactly one of the `id` or `value` fields must be specified unless the permission type is `anyone`, in which case both `id` and `value` are ignored. |
| `inherited_permissions_disabled` | bool | When true, only organizers, owners, and users with permissions added directly on the item can access it. |
| `etag` | String | Output only. The ETag of the permission. |
| `kind` | String | Output only. This is always `drive#permission`. |
| `photo_link` | String | Output only. A link to the profile photo, if available. |
| `team_drive_permission_details` | Vec<String> | Output only. Deprecated: Use `permissionDetails` instead. |
| `value` | String | The email address or domain name for the entity. This is used during inserts and is not populated in responses. When making a `drive.permissions.insert` request, exactly one of the `id` or `value` fields must be specified unless the permission type is `anyone`, in which case both `id` and `value` are ignored. |
| `domain` | String | Output only. The domain name of the entity this permission refers to. This is an output-only field which is present when the permission type is `user`, `group` or `domain`. |
| `name` | String | Output only. The name for this permission. |
| `self_link` | String | Output only. A link back to this permission. |
| `additional_roles` | Vec<String> | Additional roles for this user. Only `commenter` is currently allowed, though more may be supported in the future. |
| `type` | String | The account type. Allowed values are: * `user` * `group` * `domain` * `anyone` |
| `expiration_date` | String | The time at which this permission will expire (RFC 3339 date-time). Expiration dates have the following restrictions: - They can only be set on user and group permissions - The date must be in the future - The date cannot be more than a year in the future - The date can only be set on drive.permissions.update or drive.permissions.patch requests |
| `view` | String | Indicates the view for this permission. Only populated for permissions that belong to a view. published and metadata are the only supported values. - published: The permission's role is published_reader. - metadata: The item is only visible to the metadata view because the item has limited access and the scope has at least read access to the parent. Note: The metadata view is currently only supported on folders.  |
| `with_link` | bool | Whether the link is required for this permission. |
| `role` | String | The primary role for this user. While new values may be supported in the future, the following are currently allowed: * `owner` * `organizer` * `fileOrganizer` * `writer` * `reader` |
| `deleted` | bool | Output only. Whether the account associated with this permission has been deleted. This field only pertains to user and group permissions. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create permission
permission = provider.drive_api.Permission {
    file_id = "value"  # The ID for the file or shared drive.
}

# Access permission outputs
permission_id = permission.id
permission_pending_owner = permission.pending_owner
permission_email_address = permission.email_address
permission_permission_details = permission.permission_details
permission_auth_key = permission.auth_key
permission_id = permission.id
permission_inherited_permissions_disabled = permission.inherited_permissions_disabled
permission_etag = permission.etag
permission_kind = permission.kind
permission_photo_link = permission.photo_link
permission_team_drive_permission_details = permission.team_drive_permission_details
permission_value = permission.value
permission_domain = permission.domain
permission_name = permission.name
permission_self_link = permission.self_link
permission_additional_roles = permission.additional_roles
permission_type = permission.type
permission_expiration_date = permission.expiration_date
permission_view = permission.view
permission_with_link = permission.with_link
permission_role = permission.role
permission_deleted = permission.deleted
```

---


### Comment

Creates a new comment on the given file.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `content` | String |  | The plain text content used to create this comment. This is not HTML safe and should only be used as a starting point to make edits to a comment's content. |
| `modified_date` | String |  | The date when this comment or any of its replies were last modified. |
| `anchor` | String |  | A region of the document represented as a JSON string. For details on defining anchor properties, refer to [Manage comments and replies](https://developers.google.com/workspace/drive/api/v3/manage-comments). |
| `file_id` | String |  | The file which this comment is addressing. |
| `context` | String |  | Context of a file which is being commented on. |
| `created_date` | String |  | The date when this comment was first created. |
| `comment_id` | String |  | The ID of the comment. |
| `author` | String |  | The user who wrote this comment. |
| `kind` | String |  | This is always drive#comment. |
| `replies` | Vec<String> |  | Replies to this post. |
| `deleted` | bool |  | Whether this comment has been deleted. If a comment has been deleted the content will be cleared and this will only represent a comment that once existed. |
| `status` | String |  | The status of this comment. Status can be changed by posting a reply to a comment with the desired status. Possible values are: * `open` - The comment is still open. * `resolved` - The comment has been resolved by one of its replies. |
| `file_title` | String |  | The title of the file which this comment is addressing. |
| `html_content` | String |  | HTML formatted content for this comment. |
| `self_link` | String |  | A link back to this comment. |
| `file_id` | String | ✅ | The ID of the file. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `content` | String | The plain text content used to create this comment. This is not HTML safe and should only be used as a starting point to make edits to a comment's content. |
| `modified_date` | String | The date when this comment or any of its replies were last modified. |
| `anchor` | String | A region of the document represented as a JSON string. For details on defining anchor properties, refer to [Manage comments and replies](https://developers.google.com/workspace/drive/api/v3/manage-comments). |
| `file_id` | String | The file which this comment is addressing. |
| `context` | String | Context of a file which is being commented on. |
| `created_date` | String | The date when this comment was first created. |
| `comment_id` | String | The ID of the comment. |
| `author` | String | The user who wrote this comment. |
| `kind` | String | This is always drive#comment. |
| `replies` | Vec<String> | Replies to this post. |
| `deleted` | bool | Whether this comment has been deleted. If a comment has been deleted the content will be cleared and this will only represent a comment that once existed. |
| `status` | String | The status of this comment. Status can be changed by posting a reply to a comment with the desired status. Possible values are: * `open` - The comment is still open. * `resolved` - The comment has been resolved by one of its replies. |
| `file_title` | String | The title of the file which this comment is addressing. |
| `html_content` | String | HTML formatted content for this comment. |
| `self_link` | String | A link back to this comment. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create comment
comment = provider.drive_api.Comment {
    file_id = "value"  # The ID of the file.
}

# Access comment outputs
comment_id = comment.id
comment_content = comment.content
comment_modified_date = comment.modified_date
comment_anchor = comment.anchor
comment_file_id = comment.file_id
comment_context = comment.context
comment_created_date = comment.created_date
comment_comment_id = comment.comment_id
comment_author = comment.author
comment_kind = comment.kind
comment_replies = comment.replies
comment_deleted = comment.deleted
comment_status = comment.status
comment_file_title = comment.file_title
comment_html_content = comment.html_content
comment_self_link = comment.self_link
```

---


### File

 Inserts a new file. This method supports an */upload* URI and accepts uploaded media with the following characteristics: - *Maximum file size:* 5,120 GB - *Accepted Media MIME types:*`*/*` Note: Specify a valid MIME type, rather than the literal `*/*` value. The literal `*/*` is only used to indicate that any valid MIME type can be uploaded. For more information on uploading files, see [Upload file data](/workspace/drive/api/guides/manage-uploads). Apps creating shortcuts with `files.insert` must specify the MIME type `application/vnd.google-apps.shortcut`. Apps should specify a file extension in the `title` property when inserting files with the API. For example, an operation to insert a JPEG file should specify something like `"title": "cat.jpg"` in the metadata. Subsequent `GET` requests include the read-only `fileExtension` property populated with the extension originally specified in the `title` property. When a Google Drive user requests to download a file, or when the file is downloaded through the sync client, Drive builds a full filename (with extension) based on the title. In cases where the extension is missing, Drive attempts to determine the extension based on the file's MIME type.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `trashing_user` | String |  | Output only. If the file has been explicitly trashed, the user who trashed it. Only populated for items in shared drives. |
| `file_size` | String |  | Output only. Size in bytes of blobs and first party editor files. Won't be populated for files that have no size, like shortcuts and folders. |
| `copyable` | bool |  | Output only. Deprecated: Use `capabilities/canCopy` instead. |
| `head_revision_id` | String |  | Output only. The ID of the file's head revision. This field is only populated for files with content stored in Google Drive; it is not populated for Docs Editors or shortcut files. |
| `title` | String |  | The title of this file. Note that for immutable items such as the top level folders of shared drives, My Drive root folder, and Application Data folder the title is constant. |
| `modified_date` | String |  | Last time this file was modified by anyone (formatted RFC 3339 timestamp). This is only mutable on update when the setModifiedDate parameter is set. |
| `export_links` | HashMap<String, String> |  | Output only. Links for exporting Docs Editors files to specific formats. |
| `sha1_checksum` | String |  | Output only. The SHA1 checksum associated with this file, if available. This field is only populated for files with content stored in Google Drive; it is not populated for Docs Editors or shortcut files. |
| `drive_id` | String |  | Output only. ID of the shared drive the file resides in. Only populated for items in shared drives. |
| `folder_color_rgb` | String |  | Folder color as an RGB hex string if the file is a folder or a shortcut to a folder. The list of supported colors is available in the folderColorPalette field of the About resource. If an unsupported color is specified, it will be changed to the closest color in the palette. |
| `embed_link` | String |  | Output only. A link for embedding the file. |
| `shortcut_details` | String |  | Shortcut file details. Only populated for shortcut files, which have the mimeType field set to `application/vnd.google-apps.shortcut`. Can only be set on `files.insert` requests. |
| `modified_by_me_date` | String |  | Last time this file was modified by the user (formatted RFC 3339 timestamp). Note that setting modifiedDate will also update the modifiedByMe date for the user which set the date. |
| `self_link` | String |  | Output only. A link back to this file. |
| `user_permission` | String |  | Output only. The permissions for the authenticated user on this file. |
| `web_content_link` | String |  | Output only. A link for downloading the content of the file in a browser using cookie based authentication. In cases where the content is shared publicly, the content can be downloaded without any credentials. |
| `id` | String |  | The ID of the file. |
| `can_read_revisions` | bool |  | Output only. Deprecated: Use `capabilities/canReadRevisions` instead. |
| `kind` | String |  | Output only. The type of file. This is always `drive#file`. |
| `shareable` | bool |  | Output only. Deprecated: Use `capabilities/canShare` instead. |
| `label_info` | String |  | Output only. An overview of the labels on the file. |
| `parents` | Vec<String> |  | The ID of the parent folder containing the file. A file can only have one parent folder; specifying multiple parents isn't supported. If not specified as part of an insert request, the file is placed directly in the user's My Drive folder. If not specified as part of a copy request, the file inherits any discoverable parent of the source file. Update requests must use the `addParents` and `removeParents` parameters to modify the parents list. |
| `created_date` | String |  | Create time for this file (formatted RFC 3339 timestamp). |
| `web_view_link` | String |  | Output only. A link only available on public folders for viewing their static web assets (HTML, CSS, JS, etc) via Google Drive's Website Hosting. |
| `marked_viewed_by_me_date` | String |  | Deprecated. |
| `version` | String |  | Output only. A monotonically increasing version number for the file. This reflects every change made to the file on the server, even those not visible to the requesting user. |
| `default_open_with_link` | String |  | Output only. A link to open this file with the user's default app for this file. Only populated when the drive.apps.readonly scope is used. |
| `has_thumbnail` | bool |  | Output only. Whether this file has a thumbnail. This does not indicate whether the requesting app has access to the thumbnail. To check access, look for the presence of the thumbnailLink field. |
| `last_viewed_by_me_date` | String |  | Last time this file was viewed by the user (formatted RFC 3339 timestamp). |
| `thumbnail_version` | String |  | Output only. The thumbnail version for use in thumbnail cache invalidation. |
| `permissions` | Vec<String> |  | Output only. The list of permissions for users with access to this file. Not populated for items in shared drives. |
| `last_modifying_user_name` | String |  | Output only. Name of the last user to modify this file. |
| `app_data_contents` | bool |  | Output only. Whether this file is in the Application Data folder. |
| `open_with_links` | HashMap<String, String> |  | Output only. A map of the id of each of the user's apps to a link to open this file with that app. Only populated when the drive.apps.readonly scope is used. |
| `is_app_authorized` | bool |  | Output only. Whether the file was created or opened by the requesting app. |
| `mime_type` | String |  | The MIME type of the file. This is only mutable on update when uploading new content. This field can be left blank, and the mimetype will be determined from the uploaded content's MIME type. |
| `quota_bytes_used` | String |  | Output only. The number of quota bytes used by this file. |
| `editable` | bool |  | Output only. Deprecated: Use `capabilities/canEdit` instead. |
| `indexable_text` | String |  | Indexable text attributes for the file (can only be written) |
| `labels` | String |  | A group of labels for the file. |
| `link_share_metadata` | String |  | Contains details about the link URLs that clients are using to refer to this item. |
| `sha256_checksum` | String |  | Output only. The SHA256 checksum associated with this file, if available. This field is only populated for files with content stored in Google Drive; it is not populated for Docs Editors or shortcut files. |
| `icon_link` | String |  | Output only. A link to the file's icon. |
| `can_comment` | bool |  | Output only. Deprecated: Use `capabilities/canComment` instead. |
| `owned_by_me` | bool |  | Output only. Whether the file is owned by the current user. Not populated for items in shared drives. |
| `owner_names` | Vec<String> |  | Output only. Name(s) of the owner(s) of this file. Not populated for items in shared drives. |
| `description` | String |  | A short description of the file. |
| `permission_ids` | Vec<String> |  | Output only. List of permission IDs for users with access to this file. |
| `etag` | String |  | Output only. ETag of the file. |
| `explicitly_trashed` | bool |  | Output only. Whether this file has been explicitly trashed, as opposed to recursively trashed. |
| `original_filename` | String |  | The original filename of the uploaded content if available, or else the original value of the `title` field. This is only available for files with binary content in Google Drive. |
| `sharing_user` | String |  | Output only. User that shared the item with the current user, if available. |
| `owners` | Vec<String> |  | Output only. The owner of this file. Only certain legacy files may have more than one owner. This field isn't populated for items in shared drives. |
| `writers_can_share` | bool |  | Whether writers can share the document with other users. Not populated for items in shared drives. |
| `team_drive_id` | String |  | Output only. Deprecated: Use `driveId` instead. |
| `last_modifying_user` | String |  | Output only. The last user to modify this file. This field is only populated when the last modification was performed by a signed-in user. |
| `spaces` | Vec<String> |  | Output only. The list of spaces which contain the file. Supported values are `drive`, `appDataFolder` and `photos`. |
| `content_restrictions` | Vec<String> |  | Restrictions for accessing the content of the file. Only populated if such a restriction exists. |
| `has_augmented_permissions` | bool |  | Output only. Whether there are permissions directly on this file. This field is only populated for items in shared drives. |
| `inherited_permissions_disabled` | bool |  | Whether this file has inherited permissions disabled. Inherited permissions are enabled by default. |
| `capabilities` | String |  | Output only. Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take. |
| `file_extension` | String |  | Output only. The final component of `fullFileExtension` with trailing text that does not appear to be part of the extension removed. This field is only populated for files with content stored in Google Drive; it is not populated for Docs Editors or shortcut files. |
| `trashed_date` | String |  | The time that the item was trashed (formatted RFC 3339 timestamp). Only populated for items in shared drives. |
| `properties` | Vec<String> |  | The list of properties. |
| `video_media_metadata` | String |  | Output only. Metadata about video media. This will only be present for video types. |
| `thumbnail` | String |  | A thumbnail for the file. This will only be used if a standard thumbnail cannot be generated. |
| `download_url` | String |  | Output only. Short lived download URL for the file. This field is only populated for files with content stored in Google Drive; it is not populated for Google Docs or shortcut files. |
| `full_file_extension` | String |  | Output only. The full file extension; extracted from the title. May contain multiple concatenated extensions, such as "tar.gz". Removing an extension from the title does not clear this field; however, changing the extension on the title does update this field. This field is only populated for files with content stored in Google Drive; it is not populated for Docs Editors or shortcut files. |
| `thumbnail_link` | String |  | Output only. A short-lived link to the file's thumbnail, if available. Typically lasts on the order of hours. Not intended for direct usage on web applications due to [Cross-Origin Resource Sharing (CORS)](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS), consider using a proxy server. Only populated when the requesting app can access the file's content. If the file isn't shared publicly, the URL returned in `Files.thumbnailLink` must be fetched using a credentialed request. |
| `image_media_metadata` | String |  | Output only. Metadata about image media. This will only be present for image types, and its contents will depend on what can be parsed from the image content. |
| `resource_key` | String |  | Output only. A key needed to access the item via a shared link. |
| `shared_with_me_date` | String |  | Time at which this file was shared with the user (formatted RFC 3339 timestamp). |
| `copy_requires_writer_permission` | bool |  | Whether the options to copy, print, or download this file, should be disabled for readers and commenters. |
| `alternate_link` | String |  | Output only. A link for opening the file in a relevant Google editor or viewer. |
| `md5_checksum` | String |  | Output only. An MD5 checksum for the content of this file. This field is only populated for files with content stored in Google Drive; it is not populated for Docs Editors or shortcut files. |
| `shared` | bool |  | Output only. Whether the file has been shared. Not populated for items in shared drives. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `trashing_user` | String | Output only. If the file has been explicitly trashed, the user who trashed it. Only populated for items in shared drives. |
| `file_size` | String | Output only. Size in bytes of blobs and first party editor files. Won't be populated for files that have no size, like shortcuts and folders. |
| `copyable` | bool | Output only. Deprecated: Use `capabilities/canCopy` instead. |
| `head_revision_id` | String | Output only. The ID of the file's head revision. This field is only populated for files with content stored in Google Drive; it is not populated for Docs Editors or shortcut files. |
| `title` | String | The title of this file. Note that for immutable items such as the top level folders of shared drives, My Drive root folder, and Application Data folder the title is constant. |
| `modified_date` | String | Last time this file was modified by anyone (formatted RFC 3339 timestamp). This is only mutable on update when the setModifiedDate parameter is set. |
| `export_links` | HashMap<String, String> | Output only. Links for exporting Docs Editors files to specific formats. |
| `sha1_checksum` | String | Output only. The SHA1 checksum associated with this file, if available. This field is only populated for files with content stored in Google Drive; it is not populated for Docs Editors or shortcut files. |
| `drive_id` | String | Output only. ID of the shared drive the file resides in. Only populated for items in shared drives. |
| `folder_color_rgb` | String | Folder color as an RGB hex string if the file is a folder or a shortcut to a folder. The list of supported colors is available in the folderColorPalette field of the About resource. If an unsupported color is specified, it will be changed to the closest color in the palette. |
| `embed_link` | String | Output only. A link for embedding the file. |
| `shortcut_details` | String | Shortcut file details. Only populated for shortcut files, which have the mimeType field set to `application/vnd.google-apps.shortcut`. Can only be set on `files.insert` requests. |
| `modified_by_me_date` | String | Last time this file was modified by the user (formatted RFC 3339 timestamp). Note that setting modifiedDate will also update the modifiedByMe date for the user which set the date. |
| `self_link` | String | Output only. A link back to this file. |
| `user_permission` | String | Output only. The permissions for the authenticated user on this file. |
| `web_content_link` | String | Output only. A link for downloading the content of the file in a browser using cookie based authentication. In cases where the content is shared publicly, the content can be downloaded without any credentials. |
| `id` | String | The ID of the file. |
| `can_read_revisions` | bool | Output only. Deprecated: Use `capabilities/canReadRevisions` instead. |
| `kind` | String | Output only. The type of file. This is always `drive#file`. |
| `shareable` | bool | Output only. Deprecated: Use `capabilities/canShare` instead. |
| `label_info` | String | Output only. An overview of the labels on the file. |
| `parents` | Vec<String> | The ID of the parent folder containing the file. A file can only have one parent folder; specifying multiple parents isn't supported. If not specified as part of an insert request, the file is placed directly in the user's My Drive folder. If not specified as part of a copy request, the file inherits any discoverable parent of the source file. Update requests must use the `addParents` and `removeParents` parameters to modify the parents list. |
| `created_date` | String | Create time for this file (formatted RFC 3339 timestamp). |
| `web_view_link` | String | Output only. A link only available on public folders for viewing their static web assets (HTML, CSS, JS, etc) via Google Drive's Website Hosting. |
| `marked_viewed_by_me_date` | String | Deprecated. |
| `version` | String | Output only. A monotonically increasing version number for the file. This reflects every change made to the file on the server, even those not visible to the requesting user. |
| `default_open_with_link` | String | Output only. A link to open this file with the user's default app for this file. Only populated when the drive.apps.readonly scope is used. |
| `has_thumbnail` | bool | Output only. Whether this file has a thumbnail. This does not indicate whether the requesting app has access to the thumbnail. To check access, look for the presence of the thumbnailLink field. |
| `last_viewed_by_me_date` | String | Last time this file was viewed by the user (formatted RFC 3339 timestamp). |
| `thumbnail_version` | String | Output only. The thumbnail version for use in thumbnail cache invalidation. |
| `permissions` | Vec<String> | Output only. The list of permissions for users with access to this file. Not populated for items in shared drives. |
| `last_modifying_user_name` | String | Output only. Name of the last user to modify this file. |
| `app_data_contents` | bool | Output only. Whether this file is in the Application Data folder. |
| `open_with_links` | HashMap<String, String> | Output only. A map of the id of each of the user's apps to a link to open this file with that app. Only populated when the drive.apps.readonly scope is used. |
| `is_app_authorized` | bool | Output only. Whether the file was created or opened by the requesting app. |
| `mime_type` | String | The MIME type of the file. This is only mutable on update when uploading new content. This field can be left blank, and the mimetype will be determined from the uploaded content's MIME type. |
| `quota_bytes_used` | String | Output only. The number of quota bytes used by this file. |
| `editable` | bool | Output only. Deprecated: Use `capabilities/canEdit` instead. |
| `indexable_text` | String | Indexable text attributes for the file (can only be written) |
| `labels` | String | A group of labels for the file. |
| `link_share_metadata` | String | Contains details about the link URLs that clients are using to refer to this item. |
| `sha256_checksum` | String | Output only. The SHA256 checksum associated with this file, if available. This field is only populated for files with content stored in Google Drive; it is not populated for Docs Editors or shortcut files. |
| `icon_link` | String | Output only. A link to the file's icon. |
| `can_comment` | bool | Output only. Deprecated: Use `capabilities/canComment` instead. |
| `owned_by_me` | bool | Output only. Whether the file is owned by the current user. Not populated for items in shared drives. |
| `owner_names` | Vec<String> | Output only. Name(s) of the owner(s) of this file. Not populated for items in shared drives. |
| `description` | String | A short description of the file. |
| `permission_ids` | Vec<String> | Output only. List of permission IDs for users with access to this file. |
| `etag` | String | Output only. ETag of the file. |
| `explicitly_trashed` | bool | Output only. Whether this file has been explicitly trashed, as opposed to recursively trashed. |
| `original_filename` | String | The original filename of the uploaded content if available, or else the original value of the `title` field. This is only available for files with binary content in Google Drive. |
| `sharing_user` | String | Output only. User that shared the item with the current user, if available. |
| `owners` | Vec<String> | Output only. The owner of this file. Only certain legacy files may have more than one owner. This field isn't populated for items in shared drives. |
| `writers_can_share` | bool | Whether writers can share the document with other users. Not populated for items in shared drives. |
| `team_drive_id` | String | Output only. Deprecated: Use `driveId` instead. |
| `last_modifying_user` | String | Output only. The last user to modify this file. This field is only populated when the last modification was performed by a signed-in user. |
| `spaces` | Vec<String> | Output only. The list of spaces which contain the file. Supported values are `drive`, `appDataFolder` and `photos`. |
| `content_restrictions` | Vec<String> | Restrictions for accessing the content of the file. Only populated if such a restriction exists. |
| `has_augmented_permissions` | bool | Output only. Whether there are permissions directly on this file. This field is only populated for items in shared drives. |
| `inherited_permissions_disabled` | bool | Whether this file has inherited permissions disabled. Inherited permissions are enabled by default. |
| `capabilities` | String | Output only. Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take. |
| `file_extension` | String | Output only. The final component of `fullFileExtension` with trailing text that does not appear to be part of the extension removed. This field is only populated for files with content stored in Google Drive; it is not populated for Docs Editors or shortcut files. |
| `trashed_date` | String | The time that the item was trashed (formatted RFC 3339 timestamp). Only populated for items in shared drives. |
| `properties` | Vec<String> | The list of properties. |
| `video_media_metadata` | String | Output only. Metadata about video media. This will only be present for video types. |
| `thumbnail` | String | A thumbnail for the file. This will only be used if a standard thumbnail cannot be generated. |
| `download_url` | String | Output only. Short lived download URL for the file. This field is only populated for files with content stored in Google Drive; it is not populated for Google Docs or shortcut files. |
| `full_file_extension` | String | Output only. The full file extension; extracted from the title. May contain multiple concatenated extensions, such as "tar.gz". Removing an extension from the title does not clear this field; however, changing the extension on the title does update this field. This field is only populated for files with content stored in Google Drive; it is not populated for Docs Editors or shortcut files. |
| `thumbnail_link` | String | Output only. A short-lived link to the file's thumbnail, if available. Typically lasts on the order of hours. Not intended for direct usage on web applications due to [Cross-Origin Resource Sharing (CORS)](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS), consider using a proxy server. Only populated when the requesting app can access the file's content. If the file isn't shared publicly, the URL returned in `Files.thumbnailLink` must be fetched using a credentialed request. |
| `image_media_metadata` | String | Output only. Metadata about image media. This will only be present for image types, and its contents will depend on what can be parsed from the image content. |
| `resource_key` | String | Output only. A key needed to access the item via a shared link. |
| `shared_with_me_date` | String | Time at which this file was shared with the user (formatted RFC 3339 timestamp). |
| `copy_requires_writer_permission` | bool | Whether the options to copy, print, or download this file, should be disabled for readers and commenters. |
| `alternate_link` | String | Output only. A link for opening the file in a relevant Google editor or viewer. |
| `md5_checksum` | String | Output only. An MD5 checksum for the content of this file. This field is only populated for files with content stored in Google Drive; it is not populated for Docs Editors or shortcut files. |
| `shared` | bool | Output only. Whether the file has been shared. Not populated for items in shared drives. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create file
file = provider.drive_api.File {
}

# Access file outputs
file_id = file.id
file_trashing_user = file.trashing_user
file_file_size = file.file_size
file_copyable = file.copyable
file_head_revision_id = file.head_revision_id
file_title = file.title
file_modified_date = file.modified_date
file_export_links = file.export_links
file_sha1_checksum = file.sha1_checksum
file_drive_id = file.drive_id
file_folder_color_rgb = file.folder_color_rgb
file_embed_link = file.embed_link
file_shortcut_details = file.shortcut_details
file_modified_by_me_date = file.modified_by_me_date
file_self_link = file.self_link
file_user_permission = file.user_permission
file_web_content_link = file.web_content_link
file_id = file.id
file_can_read_revisions = file.can_read_revisions
file_kind = file.kind
file_shareable = file.shareable
file_label_info = file.label_info
file_parents = file.parents
file_created_date = file.created_date
file_web_view_link = file.web_view_link
file_marked_viewed_by_me_date = file.marked_viewed_by_me_date
file_version = file.version
file_default_open_with_link = file.default_open_with_link
file_has_thumbnail = file.has_thumbnail
file_last_viewed_by_me_date = file.last_viewed_by_me_date
file_thumbnail_version = file.thumbnail_version
file_permissions = file.permissions
file_last_modifying_user_name = file.last_modifying_user_name
file_app_data_contents = file.app_data_contents
file_open_with_links = file.open_with_links
file_is_app_authorized = file.is_app_authorized
file_mime_type = file.mime_type
file_quota_bytes_used = file.quota_bytes_used
file_editable = file.editable
file_indexable_text = file.indexable_text
file_labels = file.labels
file_link_share_metadata = file.link_share_metadata
file_sha256_checksum = file.sha256_checksum
file_icon_link = file.icon_link
file_can_comment = file.can_comment
file_owned_by_me = file.owned_by_me
file_owner_names = file.owner_names
file_description = file.description
file_permission_ids = file.permission_ids
file_etag = file.etag
file_explicitly_trashed = file.explicitly_trashed
file_original_filename = file.original_filename
file_sharing_user = file.sharing_user
file_owners = file.owners
file_writers_can_share = file.writers_can_share
file_team_drive_id = file.team_drive_id
file_last_modifying_user = file.last_modifying_user
file_spaces = file.spaces
file_content_restrictions = file.content_restrictions
file_has_augmented_permissions = file.has_augmented_permissions
file_inherited_permissions_disabled = file.inherited_permissions_disabled
file_capabilities = file.capabilities
file_file_extension = file.file_extension
file_trashed_date = file.trashed_date
file_properties = file.properties
file_video_media_metadata = file.video_media_metadata
file_thumbnail = file.thumbnail
file_download_url = file.download_url
file_full_file_extension = file.full_file_extension
file_thumbnail_link = file.thumbnail_link
file_image_media_metadata = file.image_media_metadata
file_resource_key = file.resource_key
file_shared_with_me_date = file.shared_with_me_date
file_copy_requires_writer_permission = file.copy_requires_writer_permission
file_alternate_link = file.alternate_link
file_md5_checksum = file.md5_checksum
file_shared = file.shared
```

---


### Drive

Creates a shared drive. For more information, see [Manage shared drives](https://developers.google.com/workspace/drive/api/guides/manage-shareddrives).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  | Output only. Identifies what kind of resource this is. Value: the fixed string `"drive#drive"`. |
| `capabilities` | String |  | Output only. Capabilities the current user has on this shared drive. |
| `restrictions` | String |  | A set of restrictions that apply to this shared drive or items inside this shared drive. Note that restrictions can't be set when creating a shared drive. To add a restriction, first create a shared drive and then use `drives.update` to add restrictions. |
| `color_rgb` | String |  | The color of this shared drive as an RGB hex string. It can only be set on a `drive.drives.update` request that does not set `themeId`. |
| `hidden` | bool |  | Whether the shared drive is hidden from default view. |
| `id` | String |  | Output only. The ID of this shared drive which is also the ID of the top level folder of this shared drive. |
| `background_image_file` | String |  | An image file and cropping parameters from which a background image for this shared drive is set. This is a write only field; it can only be set on `drive.drives.update` requests that don't set `themeId`. When specified, all fields of the `backgroundImageFile` must be set. |
| `background_image_link` | String |  | Output only. A short-lived link to this shared drive's background image. |
| `name` | String |  | The name of this shared drive. |
| `theme_id` | String |  | The ID of the theme from which the background image and color will be set. The set of possible `driveThemes` can be retrieved from a `drive.about.get` response. When not specified on a `drive.drives.create` request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set `colorRgb` or `backgroundImageFile`. |
| `created_time` | String |  | The time at which the shared drive was created (RFC 3339 date-time). |
| `org_unit_id` | String |  | Output only. The organizational unit of this shared drive. This field is only populated on `drives.list` responses when the `useDomainAdminAccess` parameter is set to `true`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Output only. Identifies what kind of resource this is. Value: the fixed string `"drive#drive"`. |
| `capabilities` | String | Output only. Capabilities the current user has on this shared drive. |
| `restrictions` | String | A set of restrictions that apply to this shared drive or items inside this shared drive. Note that restrictions can't be set when creating a shared drive. To add a restriction, first create a shared drive and then use `drives.update` to add restrictions. |
| `color_rgb` | String | The color of this shared drive as an RGB hex string. It can only be set on a `drive.drives.update` request that does not set `themeId`. |
| `hidden` | bool | Whether the shared drive is hidden from default view. |
| `id` | String | Output only. The ID of this shared drive which is also the ID of the top level folder of this shared drive. |
| `background_image_file` | String | An image file and cropping parameters from which a background image for this shared drive is set. This is a write only field; it can only be set on `drive.drives.update` requests that don't set `themeId`. When specified, all fields of the `backgroundImageFile` must be set. |
| `background_image_link` | String | Output only. A short-lived link to this shared drive's background image. |
| `name` | String | The name of this shared drive. |
| `theme_id` | String | The ID of the theme from which the background image and color will be set. The set of possible `driveThemes` can be retrieved from a `drive.about.get` response. When not specified on a `drive.drives.create` request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set `colorRgb` or `backgroundImageFile`. |
| `created_time` | String | The time at which the shared drive was created (RFC 3339 date-time). |
| `org_unit_id` | String | Output only. The organizational unit of this shared drive. This field is only populated on `drives.list` responses when the `useDomainAdminAccess` parameter is set to `true`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create drive
drive = provider.drive_api.Drive {
}

# Access drive outputs
drive_id = drive.id
drive_kind = drive.kind
drive_capabilities = drive.capabilities
drive_restrictions = drive.restrictions
drive_color_rgb = drive.color_rgb
drive_hidden = drive.hidden
drive_id = drive.id
drive_background_image_file = drive.background_image_file
drive_background_image_link = drive.background_image_link
drive_name = drive.name
drive_theme_id = drive.theme_id
drive_created_time = drive.created_time
drive_org_unit_id = drive.org_unit_id
```

---


### About

Gets information about the user, the user's Drive, and system capabilities. For more information, see [Return user info](https://developers.google.com/workspace/drive/api/guides/user-info). Required: The `fields` parameter must be set. To return the exact fields you need, see [Return specific fields](https://developers.google.com/workspace/drive/api/guides/fields-parameter).

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `max_import_sizes` | HashMap<String, String> | A map of maximum import sizes by MIME type, in bytes. |
| `app_installed` | bool | Whether the user has installed the requesting app. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string `"drive#about"`. |
| `import_formats` | HashMap<String, Vec<String>> | A map of source MIME type to possible targets for all supported imports. |
| `storage_quota` | String | The user's storage quota limits and usage. For users that are part of an organization with pooled storage, information about the limit and usage across all services is for the organization, rather than the individual user. All fields are measured in bytes. |
| `can_create_team_drives` | bool | Deprecated: Use `canCreateDrives` instead. |
| `can_create_drives` | bool | Whether the user can create shared drives. |
| `drive_themes` | Vec<String> | A list of themes that are supported for shared drives. |
| `export_formats` | HashMap<String, Vec<String>> | A map of source MIME type to possible targets for all supported exports. |
| `max_upload_size` | String | The maximum upload size in bytes. |
| `team_drive_themes` | Vec<String> | Deprecated: Use `driveThemes` instead. |
| `user` | String | The authenticated user. |
| `folder_color_palette` | Vec<String> | The currently supported folder colors as RGB hex strings. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access about outputs
about_id = about.id
about_max_import_sizes = about.max_import_sizes
about_app_installed = about.app_installed
about_kind = about.kind
about_import_formats = about.import_formats
about_storage_quota = about.storage_quota
about_can_create_team_drives = about.can_create_team_drives
about_can_create_drives = about.can_create_drives
about_drive_themes = about.drive_themes
about_export_formats = about.export_formats
about_max_upload_size = about.max_upload_size
about_team_drive_themes = about.team_drive_themes
about_user = about.user
about_folder_color_palette = about.folder_color_palette
```

---


### Revision

Gets a revision's metadata or content by ID. For more information, see [Manage file revisions](https://developers.google.com/workspace/drive/api/guides/manage-revisions).

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `published` | bool |  | Whether this revision is published. This is only applicable to Docs Editors files. |
| `kind` | String |  | Output only. Identifies what kind of resource this is. Value: the fixed string `"drive#revision"`. |
| `original_filename` | String |  | Output only. The original filename used to create this revision. This is only applicable to files with binary content in Drive. |
| `published_link` | String |  | Output only. A link to the published revision. This is only populated for Docs Editors files. |
| `id` | String |  | Output only. The ID of the revision. |
| `keep_forever` | bool |  | Whether to keep this revision forever, even if it is no longer the head revision. If not set, the revision will be automatically purged 30 days after newer content is uploaded. This can be set on a maximum of 200 revisions for a file. This field is only applicable to files with binary content in Drive. |
| `last_modifying_user` | String |  | Output only. The last user to modify this revision. This field is only populated when the last modification was performed by a signed-in user. |
| `size` | String |  | Output only. The size of the revision's content in bytes. This is only applicable to files with binary content in Drive. |
| `mime_type` | String |  | Output only. The MIME type of the revision. |
| `md5_checksum` | String |  | Output only. The MD5 checksum of the revision's content. This is only applicable to files with binary content in Drive. |
| `published_outside_domain` | bool |  | Whether this revision is published outside the domain. This is only applicable to Docs Editors files. |
| `modified_time` | String |  | The last time the revision was modified (RFC 3339 date-time). |
| `publish_auto` | bool |  | Whether subsequent revisions will be automatically republished. This is only applicable to Docs Editors files. |
| `export_links` | HashMap<String, String> |  | Output only. Links for exporting Docs Editors files to specific formats. |
| `revision_id` | String | ✅ | The ID of the revision. |
| `file_id` | String | ✅ | The ID of the file. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `published` | bool | Whether this revision is published. This is only applicable to Docs Editors files. |
| `kind` | String | Output only. Identifies what kind of resource this is. Value: the fixed string `"drive#revision"`. |
| `original_filename` | String | Output only. The original filename used to create this revision. This is only applicable to files with binary content in Drive. |
| `published_link` | String | Output only. A link to the published revision. This is only populated for Docs Editors files. |
| `id` | String | Output only. The ID of the revision. |
| `keep_forever` | bool | Whether to keep this revision forever, even if it is no longer the head revision. If not set, the revision will be automatically purged 30 days after newer content is uploaded. This can be set on a maximum of 200 revisions for a file. This field is only applicable to files with binary content in Drive. |
| `last_modifying_user` | String | Output only. The last user to modify this revision. This field is only populated when the last modification was performed by a signed-in user. |
| `size` | String | Output only. The size of the revision's content in bytes. This is only applicable to files with binary content in Drive. |
| `mime_type` | String | Output only. The MIME type of the revision. |
| `md5_checksum` | String | Output only. The MD5 checksum of the revision's content. This is only applicable to files with binary content in Drive. |
| `published_outside_domain` | bool | Whether this revision is published outside the domain. This is only applicable to Docs Editors files. |
| `modified_time` | String | The last time the revision was modified (RFC 3339 date-time). |
| `publish_auto` | bool | Whether subsequent revisions will be automatically republished. This is only applicable to Docs Editors files. |
| `export_links` | HashMap<String, String> | Output only. Links for exporting Docs Editors files to specific formats. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access revision outputs
revision_id = revision.id
revision_published = revision.published
revision_kind = revision.kind
revision_original_filename = revision.original_filename
revision_published_link = revision.published_link
revision_id = revision.id
revision_keep_forever = revision.keep_forever
revision_last_modifying_user = revision.last_modifying_user
revision_size = revision.size
revision_mime_type = revision.mime_type
revision_md5_checksum = revision.md5_checksum
revision_published_outside_domain = revision.published_outside_domain
revision_modified_time = revision.modified_time
revision_publish_auto = revision.publish_auto
revision_export_links = revision.export_links
```

---


### Replie

Creates a reply to a comment.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `created_time` | String |  | The time at which the reply was created (RFC 3339 date-time). |
| `assignee_email_address` | String |  | Output only. The email of the user who is assigned to this reply, if none is assigned this will be unset. |
| `deleted` | bool |  | Output only. Whether the reply has been deleted. A deleted reply has no content. |
| `id` | String |  | Output only. The ID of the reply. |
| `html_content` | String |  | Output only. The content of the reply with HTML formatting. |
| `kind` | String |  | Output only. Identifies what kind of resource this is. Value: the fixed string `"drive#reply"`. |
| `modified_time` | String |  | The last time the reply was modified (RFC 3339 date-time). |
| `action` | String |  | The action the reply performed to the parent comment. Valid values are: * `resolve` * `reopen` |
| `author` | String |  | Output only. The author of the reply. The author's email address and permission ID will not be populated. |
| `content` | String |  | The plain text content of the reply. This field is used for setting the content, while `htmlContent` should be displayed. This is required on creates if no `action` is specified. |
| `mentioned_email_addresses` | Vec<String> |  | Output only. The emails of the users who were mentioned in this reply, if none were mentioned this will be an empty list. |
| `file_id` | String | ✅ | The ID of the file. |
| `comment_id` | String | ✅ | The ID of the comment. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `created_time` | String | The time at which the reply was created (RFC 3339 date-time). |
| `assignee_email_address` | String | Output only. The email of the user who is assigned to this reply, if none is assigned this will be unset. |
| `deleted` | bool | Output only. Whether the reply has been deleted. A deleted reply has no content. |
| `id` | String | Output only. The ID of the reply. |
| `html_content` | String | Output only. The content of the reply with HTML formatting. |
| `kind` | String | Output only. Identifies what kind of resource this is. Value: the fixed string `"drive#reply"`. |
| `modified_time` | String | The last time the reply was modified (RFC 3339 date-time). |
| `action` | String | The action the reply performed to the parent comment. Valid values are: * `resolve` * `reopen` |
| `author` | String | Output only. The author of the reply. The author's email address and permission ID will not be populated. |
| `content` | String | The plain text content of the reply. This field is used for setting the content, while `htmlContent` should be displayed. This is required on creates if no `action` is specified. |
| `mentioned_email_addresses` | Vec<String> | Output only. The emails of the users who were mentioned in this reply, if none were mentioned this will be an empty list. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create replie
replie = provider.drive_api.Replie {
    file_id = "value"  # The ID of the file.
    comment_id = "value"  # The ID of the comment.
}

# Access replie outputs
replie_id = replie.id
replie_created_time = replie.created_time
replie_assignee_email_address = replie.assignee_email_address
replie_deleted = replie.deleted
replie_id = replie.id
replie_html_content = replie.html_content
replie_kind = replie.kind
replie_modified_time = replie.modified_time
replie_action = replie.action
replie_author = replie.author
replie_content = replie.content
replie_mentioned_email_addresses = replie.mentioned_email_addresses
```

---


### Permission

Creates a permission for a file or shared drive. For more information, see [Share files, folders, and drives](https://developers.google.com/workspace/drive/api/guides/manage-sharing). **Warning:** Concurrent permissions operations on the same file aren't supported; only the last update is applied.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `deleted` | bool |  | Output only. Whether the account associated with this permission has been deleted. This field only pertains to permissions of type `user` or `group`. |
| `allow_file_discovery` | bool |  | Whether the permission allows the file to be discovered through search. This is only applicable for permissions of type `domain` or `anyone`. |
| `id` | String |  | Output only. The ID of this permission. This is a unique identifier for the grantee, and is published in the [User resource](https://developers.google.com/workspace/drive/api/reference/rest/v3/User) as `permissionId`. IDs should be treated as opaque values. |
| `email_address` | String |  | The email address of the user or group to which this permission refers. |
| `expiration_time` | String |  | The time at which this permission will expire (RFC 3339 date-time). Expiration times have the following restrictions: - They can only be set on user and group permissions - The time must be in the future - The time cannot be more than a year in the future |
| `kind` | String |  | Output only. Identifies what kind of resource this is. Value: the fixed string `"drive#permission"`. |
| `photo_link` | String |  | Output only. A link to the user's profile photo, if available. |
| `inherited_permissions_disabled` | bool |  | When `true`, only organizers, owners, and users with permissions added directly on the item can access it. |
| `type` | String |  | The type of the grantee. Supported values include: * `user` * `group` * `domain` * `anyone` When creating a permission, if `type` is `user` or `group`, you must provide an `emailAddress` for the user or group. If `type` is `domain`, you must provide a `domain`. If `type` is `anyone`, no extra information is required. |
| `view` | String |  | Indicates the view for this permission. Only populated for permissions that belong to a view. The only supported values are `published` and `metadata`: * `published`: The permission's role is `publishedReader`. * `metadata`: The item is only visible to the `metadata` view because the item has limited access and the scope has at least read access to the parent. The `metadata` view is only supported on folders. For more information, see [Views](https://developers.google.com/workspace/drive/api/guides/ref-roles#views). |
| `role` | String |  | The role granted by this permission. Supported values include: * `owner` * `organizer` * `fileOrganizer` * `writer` * `commenter` * `reader` For more information, see [Roles and permissions](https://developers.google.com/workspace/drive/api/guides/ref-roles). |
| `domain` | String |  | The domain to which this permission refers. |
| `permission_details` | Vec<String> |  | Output only. Details of whether the permissions on this item are inherited or are directly on this item. |
| `display_name` | String |  | Output only. The "pretty" name of the value of the permission. The following is a list of examples for each type of permission: * `user` - User's full name, as defined for their Google Account, such as "Dana A." * `group` - Name of the Google Group, such as "The Company Administrators." * `domain` - String domain name, such as "cymbalgroup.com." * `anyone` - No `displayName` is present. |
| `pending_owner` | bool |  | Whether the account associated with this permission is a pending owner. Only populated for permissions of type `user` for files that aren't in a shared drive. |
| `team_drive_permission_details` | Vec<String> |  | Output only. Deprecated: Output only. Use `permissionDetails` instead. |
| `file_id` | String | ✅ | The ID of the file or shared drive. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `deleted` | bool | Output only. Whether the account associated with this permission has been deleted. This field only pertains to permissions of type `user` or `group`. |
| `allow_file_discovery` | bool | Whether the permission allows the file to be discovered through search. This is only applicable for permissions of type `domain` or `anyone`. |
| `id` | String | Output only. The ID of this permission. This is a unique identifier for the grantee, and is published in the [User resource](https://developers.google.com/workspace/drive/api/reference/rest/v3/User) as `permissionId`. IDs should be treated as opaque values. |
| `email_address` | String | The email address of the user or group to which this permission refers. |
| `expiration_time` | String | The time at which this permission will expire (RFC 3339 date-time). Expiration times have the following restrictions: - They can only be set on user and group permissions - The time must be in the future - The time cannot be more than a year in the future |
| `kind` | String | Output only. Identifies what kind of resource this is. Value: the fixed string `"drive#permission"`. |
| `photo_link` | String | Output only. A link to the user's profile photo, if available. |
| `inherited_permissions_disabled` | bool | When `true`, only organizers, owners, and users with permissions added directly on the item can access it. |
| `type` | String | The type of the grantee. Supported values include: * `user` * `group` * `domain` * `anyone` When creating a permission, if `type` is `user` or `group`, you must provide an `emailAddress` for the user or group. If `type` is `domain`, you must provide a `domain`. If `type` is `anyone`, no extra information is required. |
| `view` | String | Indicates the view for this permission. Only populated for permissions that belong to a view. The only supported values are `published` and `metadata`: * `published`: The permission's role is `publishedReader`. * `metadata`: The item is only visible to the `metadata` view because the item has limited access and the scope has at least read access to the parent. The `metadata` view is only supported on folders. For more information, see [Views](https://developers.google.com/workspace/drive/api/guides/ref-roles#views). |
| `role` | String | The role granted by this permission. Supported values include: * `owner` * `organizer` * `fileOrganizer` * `writer` * `commenter` * `reader` For more information, see [Roles and permissions](https://developers.google.com/workspace/drive/api/guides/ref-roles). |
| `domain` | String | The domain to which this permission refers. |
| `permission_details` | Vec<String> | Output only. Details of whether the permissions on this item are inherited or are directly on this item. |
| `display_name` | String | Output only. The "pretty" name of the value of the permission. The following is a list of examples for each type of permission: * `user` - User's full name, as defined for their Google Account, such as "Dana A." * `group` - Name of the Google Group, such as "The Company Administrators." * `domain` - String domain name, such as "cymbalgroup.com." * `anyone` - No `displayName` is present. |
| `pending_owner` | bool | Whether the account associated with this permission is a pending owner. Only populated for permissions of type `user` for files that aren't in a shared drive. |
| `team_drive_permission_details` | Vec<String> | Output only. Deprecated: Output only. Use `permissionDetails` instead. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create permission
permission = provider.drive_api.Permission {
    file_id = "value"  # The ID of the file or shared drive.
}

# Access permission outputs
permission_id = permission.id
permission_deleted = permission.deleted
permission_allow_file_discovery = permission.allow_file_discovery
permission_id = permission.id
permission_email_address = permission.email_address
permission_expiration_time = permission.expiration_time
permission_kind = permission.kind
permission_photo_link = permission.photo_link
permission_inherited_permissions_disabled = permission.inherited_permissions_disabled
permission_type = permission.type
permission_view = permission.view
permission_role = permission.role
permission_domain = permission.domain
permission_permission_details = permission.permission_details
permission_display_name = permission.display_name
permission_pending_owner = permission.pending_owner
permission_team_drive_permission_details = permission.team_drive_permission_details
```

---


### Operation

Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access operation outputs
operation_id = operation.id
operation_name = operation.name
operation_response = operation.response
operation_error = operation.error
operation_metadata = operation.metadata
operation_done = operation.done
```

---


### Accessproposal

Approves or denies an access proposal. For more information, see [Manage pending access proposals](https://developers.google.com/workspace/drive/api/guides/pending-access).

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `role` | Vec<String> |  | Optional. The roles that the approver has allowed, if any. For more information, see [Roles and permissions](https://developers.google.com/workspace/drive/api/guides/ref-roles). Note: This field is required for the `ACCEPT` action. |
| `send_notification` | bool |  | Optional. Whether to send an email to the requester when the access proposal is denied or accepted. |
| `action` | String |  | Required. The action to take on the access proposal. |
| `view` | String |  | Optional. Indicates the view for this access proposal. This should only be set when the proposal belongs to a view. Only `published` is supported. |
| `proposal_id` | String | ✅ | Required. The ID of the access proposal to resolve. |
| `file_id` | String | ✅ | Required. The ID of the item the request is on. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | The creation time. |
| `request_message` | String | The message that the requester added to the proposal. |
| `proposal_id` | String | The ID of the access proposal. |
| `recipient_email_address` | String | The email address of the user that will receive permissions, if accepted. |
| `file_id` | String | The file ID that the proposal for access is on. |
| `requester_email_address` | String | The email address of the requesting user. |
| `roles_and_views` | Vec<String> | A wrapper for the role and view of an access proposal. For more information, see [Roles and permissions](https://developers.google.com/workspace/drive/api/guides/ref-roles). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create accessproposal
accessproposal = provider.drive_api.Accessproposal {
    proposal_id = "value"  # Required. The ID of the access proposal to resolve.
    file_id = "value"  # Required. The ID of the item the request is on.
}

# Access accessproposal outputs
accessproposal_id = accessproposal.id
accessproposal_create_time = accessproposal.create_time
accessproposal_request_message = accessproposal.request_message
accessproposal_proposal_id = accessproposal.proposal_id
accessproposal_recipient_email_address = accessproposal.recipient_email_address
accessproposal_file_id = accessproposal.file_id
accessproposal_requester_email_address = accessproposal.requester_email_address
accessproposal_roles_and_views = accessproposal.roles_and_views
```

---


### Channel

Stops watching resources through this channel. For more information, see [Notifications for resource changes](https://developers.google.com/workspace/drive/api/guides/push).

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `token` | String |  | An arbitrary string delivered to the target address with each notification delivered over this channel. Optional. |
| `params` | HashMap<String, String> |  | Additional parameters controlling delivery channel behavior. Optional. |
| `id` | String |  | A UUID or similar unique string that identifies this channel. |
| `payload` | bool |  | A Boolean value to indicate whether payload is wanted. Optional. |
| `address` | String |  | The address where notifications are delivered for this channel. |
| `resource_uri` | String |  | A version-specific identifier for the watched resource. |
| `type` | String |  | The type of delivery mechanism used for this channel. Valid values are "web_hook" or "webhook". |
| `expiration` | String |  | Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional. |
| `kind` | String |  | Identifies this as a notification channel used to watch for changes to a resource, which is `api#channel`. |
| `resource_id` | String |  | An opaque ID that identifies the resource being watched on this channel. Stable across different API versions. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create channel
channel = provider.drive_api.Channel {
}

```

---


### File

 Creates a file. For more information, see [Create and manage files](/workspace/drive/api/guides/create-file). This method supports an */upload* URI and accepts uploaded media with the following characteristics: - *Maximum file size:* 5,120 GB - *Accepted Media MIME types:* `*/*` (Specify a valid MIME type, rather than the literal `*/*` value. The literal `*/*` is only used to indicate that any valid MIME type can be uploaded. For more information, see [Google Workspace and Google Drive supported MIME types](/workspace/drive/api/guides/mime-types).) For more information on uploading files, see [Upload file data](/workspace/drive/api/guides/manage-uploads). Apps creating shortcuts with the `create` method must specify the MIME type `application/vnd.google-apps.shortcut`. Apps should specify a file extension in the `name` property when inserting files with the API. For example, an operation to insert a JPEG file should specify something like `"name": "cat.jpg"` in the metadata. Subsequent `GET` requests include the read-only `fileExtension` property populated with the extension originally specified in the `name` property. When a Google Drive user requests to download a file, or when the file is downloaded through the sync client, Drive builds a full filename (with extension) based on the name. In cases where the extension is missing, Drive attempts to determine the extension based on the file's MIME type.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `version` | String |  | Output only. A monotonically increasing version number for the file. This reflects every change made to the file on the server, even those not visible to the user. |
| `modified_by_me` | bool |  | Output only. Whether the file has been modified by this user. |
| `copy_requires_writer_permission` | bool |  | Whether the options to copy, print, or download this file should be disabled for readers and commenters. |
| `kind` | String |  | Output only. Identifies what kind of resource this is. Value: the fixed string `"drive#file"`. |
| `video_media_metadata` | String |  | Output only. Additional metadata about video media. This may not be available immediately upon upload. |
| `folder_color_rgb` | String |  | The color for a folder or a shortcut to a folder as an RGB hex string. The supported colors are published in the `folderColorPalette` field of the [`about`](/workspace/drive/api/reference/rest/v3/about) resource. If an unsupported color is specified, the closest color in the palette is used instead. |
| `modified_time` | String |  | he last time the file was modified by anyone (RFC 3339 date-time). Note that setting modifiedTime will also update modifiedByMeTime for the user. |
| `viewers_can_copy_content` | bool |  | Deprecated: Use `copyRequiresWriterPermission` instead. |
| `description` | String |  | A short description of the file. |
| `mime_type` | String |  | The MIME type of the file. Google Drive attempts to automatically detect an appropriate value from uploaded content, if no value is provided. The value cannot be changed unless a new revision is uploaded. If a file is created with a Google Doc MIME type, the uploaded content is imported, if possible. The supported import formats are published in the [`about`](/workspace/drive/api/reference/rest/v3/about) resource. |
| `web_content_link` | String |  | Output only. A link for downloading the content of the file in a browser. This is only available for files with binary content in Google Drive. |
| `last_modifying_user` | String |  | Output only. The last user to modify the file. This field is only populated when the last modification was performed by a signed-in user. |
| `app_properties` | HashMap<String, String> |  | A collection of arbitrary key-value pairs which are private to the requesting app.
Entries with null values are cleared in update and copy requests. These properties can only be retrieved using an authenticated request. An authenticated request uses an access token obtained with a OAuth 2 client ID. You cannot use an API key to retrieve private properties. |
| `properties` | HashMap<String, String> |  | A collection of arbitrary key-value pairs which are visible to all apps.
Entries with null values are cleared in update and copy requests. |
| `size` | String |  | Output only. Size in bytes of blobs and Google Workspace editor files. Won't be populated for files that have no size, like shortcuts and folders. |
| `is_app_authorized` | bool |  | Output only. Whether the file was created or opened by the requesting app. |
| `link_share_metadata` | String |  | Contains details about the link URLs that clients are using to refer to this item. |
| `permission_ids` | Vec<String> |  | Output only. List of permission IDs for users with access to this file. |
| `shared` | bool |  | Output only. Whether the file has been shared. Not populated for items in shared drives. |
| `shared_with_me_time` | String |  | The time at which the file was shared with the user, if applicable (RFC 3339 date-time). |
| `web_view_link` | String |  | Output only. A link for opening the file in a relevant Google editor or viewer in a browser. |
| `image_media_metadata` | String |  | Output only. Additional metadata about image media, if available. |
| `created_time` | String |  | The time at which the file was created (RFC 3339 date-time). |
| `full_file_extension` | String |  | Output only. The full file extension extracted from the `name` field. May contain multiple concatenated extensions, such as "tar.gz". This is only available for files with binary content in Google Drive. This is automatically updated when the `name` field changes, however it's not cleared if the new name doesn't contain a valid extension. |
| `inherited_permissions_disabled` | bool |  | Whether this file has inherited permissions disabled. Inherited permissions are enabled by default. |
| `capabilities` | String |  | Output only. Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take. For more information, see [Understand file capabilities](https://developers.google.com/workspace/drive/api/guides/manage-sharing#capabilities). |
| `has_thumbnail` | bool |  | Output only. Whether this file has a thumbnail. This doesn't indicate whether the requesting app has access to the thumbnail. To check access, look for the presence of the thumbnailLink field. |
| `trashing_user` | String |  | Output only. If the file has been explicitly trashed, the user who trashed it. Only populated for items in shared drives. |
| `modified_by_me_time` | String |  | The last time the file was modified by the user (RFC 3339 date-time). |
| `resource_key` | String |  | Output only. A key needed to access the item via a shared link. |
| `permissions` | Vec<String> |  | Output only. The full list of permissions for the file. This is only available if the requesting user can share the file. Not populated for items in shared drives. |
| `head_revision_id` | String |  | Output only. The ID of the file's head revision. This is currently only available for files with binary content in Google Drive. |
| `thumbnail_version` | String |  | Output only. The thumbnail version for use in thumbnail cache invalidation. |
| `viewed_by_me` | bool |  | Output only. Whether the file has been viewed by this user. |
| `trashed` | bool |  | Whether the file has been trashed, either explicitly or from a trashed parent folder. Only the owner may trash a file, and other users cannot see files in the owner's trash. |
| `spaces` | Vec<String> |  | Output only. The list of spaces which contain the file. The currently supported values are `drive`, `appDataFolder`, and `photos`. |
| `original_filename` | String |  | The original filename of the uploaded content if available, or else the original value of the `name` field. This is only available for files with binary content in Google Drive. |
| `file_extension` | String |  | Output only. The final component of `fullFileExtension`. This is only available for files with binary content in Google Drive. |
| `parents` | Vec<String> |  | The ID of the parent folder containing the file. A file can only have one parent folder; specifying multiple parents isn't supported. If not specified as part of a create request, the file is placed directly in the user's My Drive folder. If not specified as part of a copy request, the file inherits any discoverable parent of the source file. Update requests must use the `addParents` and `removeParents` parameters to modify the parents list. |
| `quota_bytes_used` | String |  | Output only. The number of storage quota bytes used by the file. This includes the head revision as well as previous revisions with `keepForever` enabled. |
| `name` | String |  | The name of the file. This isn't necessarily unique within a folder. Note that for immutable items such as the top-level folders of shared drives, the My Drive root folder, and the Application Data folder, the name is constant. |
| `sharing_user` | String |  | Output only. The user who shared the file with the requesting user, if applicable. |
| `starred` | bool |  | Whether the user has starred the file. |
| `trashed_time` | String |  | The time that the item was trashed (RFC 3339 date-time). Only populated for items in shared drives. |
| `export_links` | HashMap<String, String> |  | Output only. Links for exporting Docs Editors files to specific formats. |
| `content_hints` | String |  | Additional information about the content of the file. These fields are never populated in responses. |
| `has_augmented_permissions` | bool |  | Output only. Whether there are permissions directly on this file. This field is only populated for items in shared drives. |
| `shortcut_details` | String |  | Shortcut file details. Only populated for shortcut files, which have the mimeType field set to `application/vnd.google-apps.shortcut`. Can only be set on `files.create` requests. |
| `thumbnail_link` | String |  | Output only. A short-lived link to the file's thumbnail, if available. Typically lasts on the order of hours. Not intended for direct usage on web applications due to [Cross-Origin Resource Sharing (CORS)](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) policies. Consider using a proxy server. Only populated when the requesting app can access the file's content. If the file isn't shared publicly, the URL returned in `files.thumbnailLink` must be fetched using a credentialed request. |
| `label_info` | String |  | Output only. An overview of the labels on the file. |
| `explicitly_trashed` | bool |  | Output only. Whether the file has been explicitly trashed, as opposed to recursively trashed from a parent folder. |
| `md5_checksum` | String |  | Output only. The MD5 checksum for the content of the file. This is only applicable to files with binary content in Google Drive. |
| `id` | String |  | The ID of the file. |
| `owners` | Vec<String> |  | Output only. The owner of this file. Only certain legacy files may have more than one owner. This field isn't populated for items in shared drives. |
| `download_restrictions` | String |  | Download restrictions applied on the file. |
| `sha1_checksum` | String |  | Output only. The SHA1 checksum associated with this file, if available. This field is only populated for files with content stored in Google Drive; it's not populated for Docs Editors or shortcut files. |
| `icon_link` | String |  | Output only. A static, unauthenticated link to the file's icon. |
| `viewed_by_me_time` | String |  | The last time the file was viewed by the user (RFC 3339 date-time). |
| `drive_id` | String |  | Output only. ID of the shared drive the file resides in. Only populated for items in shared drives. |
| `writers_can_share` | bool |  | Whether users with only `writer` permission can modify the file's permissions. Not populated for items in shared drives. |
| `owned_by_me` | bool |  | Output only. Whether the user owns the file. Not populated for items in shared drives. |
| `content_restrictions` | Vec<String> |  | Restrictions for accessing the content of the file. Only populated if such a restriction exists. |
| `sha256_checksum` | String |  | Output only. The SHA256 checksum associated with this file, if available. This field is only populated for files with content stored in Google Drive; it's not populated for Docs Editors or shortcut files. |
| `team_drive_id` | String |  | Deprecated: Output only. Use `driveId` instead. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `version` | String | Output only. A monotonically increasing version number for the file. This reflects every change made to the file on the server, even those not visible to the user. |
| `modified_by_me` | bool | Output only. Whether the file has been modified by this user. |
| `copy_requires_writer_permission` | bool | Whether the options to copy, print, or download this file should be disabled for readers and commenters. |
| `kind` | String | Output only. Identifies what kind of resource this is. Value: the fixed string `"drive#file"`. |
| `video_media_metadata` | String | Output only. Additional metadata about video media. This may not be available immediately upon upload. |
| `folder_color_rgb` | String | The color for a folder or a shortcut to a folder as an RGB hex string. The supported colors are published in the `folderColorPalette` field of the [`about`](/workspace/drive/api/reference/rest/v3/about) resource. If an unsupported color is specified, the closest color in the palette is used instead. |
| `modified_time` | String | he last time the file was modified by anyone (RFC 3339 date-time). Note that setting modifiedTime will also update modifiedByMeTime for the user. |
| `viewers_can_copy_content` | bool | Deprecated: Use `copyRequiresWriterPermission` instead. |
| `description` | String | A short description of the file. |
| `mime_type` | String | The MIME type of the file. Google Drive attempts to automatically detect an appropriate value from uploaded content, if no value is provided. The value cannot be changed unless a new revision is uploaded. If a file is created with a Google Doc MIME type, the uploaded content is imported, if possible. The supported import formats are published in the [`about`](/workspace/drive/api/reference/rest/v3/about) resource. |
| `web_content_link` | String | Output only. A link for downloading the content of the file in a browser. This is only available for files with binary content in Google Drive. |
| `last_modifying_user` | String | Output only. The last user to modify the file. This field is only populated when the last modification was performed by a signed-in user. |
| `app_properties` | HashMap<String, String> | A collection of arbitrary key-value pairs which are private to the requesting app.
Entries with null values are cleared in update and copy requests. These properties can only be retrieved using an authenticated request. An authenticated request uses an access token obtained with a OAuth 2 client ID. You cannot use an API key to retrieve private properties. |
| `properties` | HashMap<String, String> | A collection of arbitrary key-value pairs which are visible to all apps.
Entries with null values are cleared in update and copy requests. |
| `size` | String | Output only. Size in bytes of blobs and Google Workspace editor files. Won't be populated for files that have no size, like shortcuts and folders. |
| `is_app_authorized` | bool | Output only. Whether the file was created or opened by the requesting app. |
| `link_share_metadata` | String | Contains details about the link URLs that clients are using to refer to this item. |
| `permission_ids` | Vec<String> | Output only. List of permission IDs for users with access to this file. |
| `shared` | bool | Output only. Whether the file has been shared. Not populated for items in shared drives. |
| `shared_with_me_time` | String | The time at which the file was shared with the user, if applicable (RFC 3339 date-time). |
| `web_view_link` | String | Output only. A link for opening the file in a relevant Google editor or viewer in a browser. |
| `image_media_metadata` | String | Output only. Additional metadata about image media, if available. |
| `created_time` | String | The time at which the file was created (RFC 3339 date-time). |
| `full_file_extension` | String | Output only. The full file extension extracted from the `name` field. May contain multiple concatenated extensions, such as "tar.gz". This is only available for files with binary content in Google Drive. This is automatically updated when the `name` field changes, however it's not cleared if the new name doesn't contain a valid extension. |
| `inherited_permissions_disabled` | bool | Whether this file has inherited permissions disabled. Inherited permissions are enabled by default. |
| `capabilities` | String | Output only. Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take. For more information, see [Understand file capabilities](https://developers.google.com/workspace/drive/api/guides/manage-sharing#capabilities). |
| `has_thumbnail` | bool | Output only. Whether this file has a thumbnail. This doesn't indicate whether the requesting app has access to the thumbnail. To check access, look for the presence of the thumbnailLink field. |
| `trashing_user` | String | Output only. If the file has been explicitly trashed, the user who trashed it. Only populated for items in shared drives. |
| `modified_by_me_time` | String | The last time the file was modified by the user (RFC 3339 date-time). |
| `resource_key` | String | Output only. A key needed to access the item via a shared link. |
| `permissions` | Vec<String> | Output only. The full list of permissions for the file. This is only available if the requesting user can share the file. Not populated for items in shared drives. |
| `head_revision_id` | String | Output only. The ID of the file's head revision. This is currently only available for files with binary content in Google Drive. |
| `thumbnail_version` | String | Output only. The thumbnail version for use in thumbnail cache invalidation. |
| `viewed_by_me` | bool | Output only. Whether the file has been viewed by this user. |
| `trashed` | bool | Whether the file has been trashed, either explicitly or from a trashed parent folder. Only the owner may trash a file, and other users cannot see files in the owner's trash. |
| `spaces` | Vec<String> | Output only. The list of spaces which contain the file. The currently supported values are `drive`, `appDataFolder`, and `photos`. |
| `original_filename` | String | The original filename of the uploaded content if available, or else the original value of the `name` field. This is only available for files with binary content in Google Drive. |
| `file_extension` | String | Output only. The final component of `fullFileExtension`. This is only available for files with binary content in Google Drive. |
| `parents` | Vec<String> | The ID of the parent folder containing the file. A file can only have one parent folder; specifying multiple parents isn't supported. If not specified as part of a create request, the file is placed directly in the user's My Drive folder. If not specified as part of a copy request, the file inherits any discoverable parent of the source file. Update requests must use the `addParents` and `removeParents` parameters to modify the parents list. |
| `quota_bytes_used` | String | Output only. The number of storage quota bytes used by the file. This includes the head revision as well as previous revisions with `keepForever` enabled. |
| `name` | String | The name of the file. This isn't necessarily unique within a folder. Note that for immutable items such as the top-level folders of shared drives, the My Drive root folder, and the Application Data folder, the name is constant. |
| `sharing_user` | String | Output only. The user who shared the file with the requesting user, if applicable. |
| `starred` | bool | Whether the user has starred the file. |
| `trashed_time` | String | The time that the item was trashed (RFC 3339 date-time). Only populated for items in shared drives. |
| `export_links` | HashMap<String, String> | Output only. Links for exporting Docs Editors files to specific formats. |
| `content_hints` | String | Additional information about the content of the file. These fields are never populated in responses. |
| `has_augmented_permissions` | bool | Output only. Whether there are permissions directly on this file. This field is only populated for items in shared drives. |
| `shortcut_details` | String | Shortcut file details. Only populated for shortcut files, which have the mimeType field set to `application/vnd.google-apps.shortcut`. Can only be set on `files.create` requests. |
| `thumbnail_link` | String | Output only. A short-lived link to the file's thumbnail, if available. Typically lasts on the order of hours. Not intended for direct usage on web applications due to [Cross-Origin Resource Sharing (CORS)](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) policies. Consider using a proxy server. Only populated when the requesting app can access the file's content. If the file isn't shared publicly, the URL returned in `files.thumbnailLink` must be fetched using a credentialed request. |
| `label_info` | String | Output only. An overview of the labels on the file. |
| `explicitly_trashed` | bool | Output only. Whether the file has been explicitly trashed, as opposed to recursively trashed from a parent folder. |
| `md5_checksum` | String | Output only. The MD5 checksum for the content of the file. This is only applicable to files with binary content in Google Drive. |
| `id` | String | The ID of the file. |
| `owners` | Vec<String> | Output only. The owner of this file. Only certain legacy files may have more than one owner. This field isn't populated for items in shared drives. |
| `download_restrictions` | String | Download restrictions applied on the file. |
| `sha1_checksum` | String | Output only. The SHA1 checksum associated with this file, if available. This field is only populated for files with content stored in Google Drive; it's not populated for Docs Editors or shortcut files. |
| `icon_link` | String | Output only. A static, unauthenticated link to the file's icon. |
| `viewed_by_me_time` | String | The last time the file was viewed by the user (RFC 3339 date-time). |
| `drive_id` | String | Output only. ID of the shared drive the file resides in. Only populated for items in shared drives. |
| `writers_can_share` | bool | Whether users with only `writer` permission can modify the file's permissions. Not populated for items in shared drives. |
| `owned_by_me` | bool | Output only. Whether the user owns the file. Not populated for items in shared drives. |
| `content_restrictions` | Vec<String> | Restrictions for accessing the content of the file. Only populated if such a restriction exists. |
| `sha256_checksum` | String | Output only. The SHA256 checksum associated with this file, if available. This field is only populated for files with content stored in Google Drive; it's not populated for Docs Editors or shortcut files. |
| `team_drive_id` | String | Deprecated: Output only. Use `driveId` instead. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create file
file = provider.drive_api.File {
}

# Access file outputs
file_id = file.id
file_version = file.version
file_modified_by_me = file.modified_by_me
file_copy_requires_writer_permission = file.copy_requires_writer_permission
file_kind = file.kind
file_video_media_metadata = file.video_media_metadata
file_folder_color_rgb = file.folder_color_rgb
file_modified_time = file.modified_time
file_viewers_can_copy_content = file.viewers_can_copy_content
file_description = file.description
file_mime_type = file.mime_type
file_web_content_link = file.web_content_link
file_last_modifying_user = file.last_modifying_user
file_app_properties = file.app_properties
file_properties = file.properties
file_size = file.size
file_is_app_authorized = file.is_app_authorized
file_link_share_metadata = file.link_share_metadata
file_permission_ids = file.permission_ids
file_shared = file.shared
file_shared_with_me_time = file.shared_with_me_time
file_web_view_link = file.web_view_link
file_image_media_metadata = file.image_media_metadata
file_created_time = file.created_time
file_full_file_extension = file.full_file_extension
file_inherited_permissions_disabled = file.inherited_permissions_disabled
file_capabilities = file.capabilities
file_has_thumbnail = file.has_thumbnail
file_trashing_user = file.trashing_user
file_modified_by_me_time = file.modified_by_me_time
file_resource_key = file.resource_key
file_permissions = file.permissions
file_head_revision_id = file.head_revision_id
file_thumbnail_version = file.thumbnail_version
file_viewed_by_me = file.viewed_by_me
file_trashed = file.trashed
file_spaces = file.spaces
file_original_filename = file.original_filename
file_file_extension = file.file_extension
file_parents = file.parents
file_quota_bytes_used = file.quota_bytes_used
file_name = file.name
file_sharing_user = file.sharing_user
file_starred = file.starred
file_trashed_time = file.trashed_time
file_export_links = file.export_links
file_content_hints = file.content_hints
file_has_augmented_permissions = file.has_augmented_permissions
file_shortcut_details = file.shortcut_details
file_thumbnail_link = file.thumbnail_link
file_label_info = file.label_info
file_explicitly_trashed = file.explicitly_trashed
file_md5_checksum = file.md5_checksum
file_id = file.id
file_owners = file.owners
file_download_restrictions = file.download_restrictions
file_sha1_checksum = file.sha1_checksum
file_icon_link = file.icon_link
file_viewed_by_me_time = file.viewed_by_me_time
file_drive_id = file.drive_id
file_writers_can_share = file.writers_can_share
file_owned_by_me = file.owned_by_me
file_content_restrictions = file.content_restrictions
file_sha256_checksum = file.sha256_checksum
file_team_drive_id = file.team_drive_id
```

---


### Teamdrive

Deprecated: Use `drives.create` instead.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `background_image_link` | String |  | A short-lived link to this Team Drive's background image. |
| `name` | String |  | The name of this Team Drive. |
| `org_unit_id` | String |  | The organizational unit of this shared drive. This field is only populated on `drives.list` responses when the `useDomainAdminAccess` parameter is set to `true`. |
| `created_time` | String |  | The time at which the Team Drive was created (RFC 3339 date-time). |
| `theme_id` | String |  | The ID of the theme from which the background image and color will be set. The set of possible `teamDriveThemes` can be retrieved from a `drive.about.get` response. When not specified on a `drive.teamdrives.create` request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set `colorRgb` or `backgroundImageFile`. |
| `id` | String |  | The ID of this Team Drive which is also the ID of the top level folder of this Team Drive. |
| `capabilities` | String |  | Capabilities the current user has on this Team Drive. |
| `restrictions` | String |  | A set of restrictions that apply to this Team Drive or items inside this Team Drive. |
| `background_image_file` | String |  | An image file and cropping parameters from which a background image for this Team Drive is set. This is a write only field; it can only be set on `drive.teamdrives.update` requests that don't set `themeId`. When specified, all fields of the `backgroundImageFile` must be set. |
| `color_rgb` | String |  | The color of this Team Drive as an RGB hex string. It can only be set on a `drive.teamdrives.update` request that does not set `themeId`. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string `"drive#teamDrive"`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `background_image_link` | String | A short-lived link to this Team Drive's background image. |
| `name` | String | The name of this Team Drive. |
| `org_unit_id` | String | The organizational unit of this shared drive. This field is only populated on `drives.list` responses when the `useDomainAdminAccess` parameter is set to `true`. |
| `created_time` | String | The time at which the Team Drive was created (RFC 3339 date-time). |
| `theme_id` | String | The ID of the theme from which the background image and color will be set. The set of possible `teamDriveThemes` can be retrieved from a `drive.about.get` response. When not specified on a `drive.teamdrives.create` request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set `colorRgb` or `backgroundImageFile`. |
| `id` | String | The ID of this Team Drive which is also the ID of the top level folder of this Team Drive. |
| `capabilities` | String | Capabilities the current user has on this Team Drive. |
| `restrictions` | String | A set of restrictions that apply to this Team Drive or items inside this Team Drive. |
| `background_image_file` | String | An image file and cropping parameters from which a background image for this Team Drive is set. This is a write only field; it can only be set on `drive.teamdrives.update` requests that don't set `themeId`. When specified, all fields of the `backgroundImageFile` must be set. |
| `color_rgb` | String | The color of this Team Drive as an RGB hex string. It can only be set on a `drive.teamdrives.update` request that does not set `themeId`. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string `"drive#teamDrive"`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create teamdrive
teamdrive = provider.drive_api.Teamdrive {
}

# Access teamdrive outputs
teamdrive_id = teamdrive.id
teamdrive_background_image_link = teamdrive.background_image_link
teamdrive_name = teamdrive.name
teamdrive_org_unit_id = teamdrive.org_unit_id
teamdrive_created_time = teamdrive.created_time
teamdrive_theme_id = teamdrive.theme_id
teamdrive_id = teamdrive.id
teamdrive_capabilities = teamdrive.capabilities
teamdrive_restrictions = teamdrive.restrictions
teamdrive_background_image_file = teamdrive.background_image_file
teamdrive_color_rgb = teamdrive.color_rgb
teamdrive_kind = teamdrive.kind
```

---


### App

Gets a specific app. For more information, see [Return user info](https://developers.google.com/workspace/drive/api/guides/user-info).

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `authorized` | bool | Whether the app is authorized to access data on the user's Drive. |
| `has_drive_wide_scope` | bool | Whether the app has Drive-wide scope. An app with Drive-wide scope can access all files in the user's Drive. |
| `primary_mime_types` | Vec<String> | The list of primary MIME types. |
| `secondary_file_extensions` | Vec<String> | The list of secondary file extensions. |
| `open_url_template` | String | The template URL for opening files with this app. The template contains {ids} or {exportIds} to be replaced by the actual file IDs. For more information, see Open Files for the full documentation. |
| `secondary_mime_types` | Vec<String> | The list of secondary MIME types. |
| `supports_offline_create` | bool | Whether this app supports creating files when offline. |
| `use_by_default` | bool | Whether the app is selected as the default handler for the types it supports. |
| `kind` | String | Output only. Identifies what kind of resource this is. Value: the fixed string "drive#app". |
| `name` | String | The name of the app. |
| `create_url` | String | The URL to create a file with this app. |
| `supports_multi_open` | bool | Whether this app supports opening more than one file. |
| `object_type` | String | The type of object this app creates such as a Chart. If empty, the app name should be used instead. |
| `icons` | Vec<String> | The various icons for the app. |
| `product_url` | String | A link to the product listing for this app. |
| `id` | String | The ID of the app. |
| `short_description` | String | A short description of the app. |
| `supports_create` | bool | Whether this app supports creating objects. |
| `long_description` | String | A long description of the app. |
| `create_in_folder_template` | String | The template URL to create a file with this app in a given folder. The template contains the {folderId} to be replaced by the folder ID house the new file. |
| `supports_import` | bool | Whether this app supports importing from Google Docs. |
| `product_id` | String | The ID of the product listing for this app. |
| `installed` | bool | Whether the app is installed. |
| `primary_file_extensions` | Vec<String> | The list of primary file extensions. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access app outputs
app_id = app.id
app_authorized = app.authorized
app_has_drive_wide_scope = app.has_drive_wide_scope
app_primary_mime_types = app.primary_mime_types
app_secondary_file_extensions = app.secondary_file_extensions
app_open_url_template = app.open_url_template
app_secondary_mime_types = app.secondary_mime_types
app_supports_offline_create = app.supports_offline_create
app_use_by_default = app.use_by_default
app_kind = app.kind
app_name = app.name
app_create_url = app.create_url
app_supports_multi_open = app.supports_multi_open
app_object_type = app.object_type
app_icons = app.icons
app_product_url = app.product_url
app_id = app.id
app_short_description = app.short_description
app_supports_create = app.supports_create
app_long_description = app.long_description
app_create_in_folder_template = app.create_in_folder_template
app_supports_import = app.supports_import
app_product_id = app.product_id
app_installed = app.installed
app_primary_file_extensions = app.primary_file_extensions
```

---


### Comment

Creates a comment on a file. For more information, see [Manage comments and replies](https://developers.google.com/workspace/drive/api/guides/manage-comments). Required: The `fields` parameter must be set. To return the exact fields you need, see [Return specific fields](https://developers.google.com/workspace/drive/api/guides/fields-parameter).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `modified_time` | String |  | The last time the comment or any of its replies was modified (RFC 3339 date-time). |
| `anchor` | String |  | A region of the document represented as a JSON string. For details on defining anchor properties, refer to [Manage comments and replies](https://developers.google.com/workspace/drive/api/v3/manage-comments). |
| `author` | String |  | Output only. The author of the comment. The author's email address and permission ID will not be populated. |
| `deleted` | bool |  | Output only. Whether the comment has been deleted. A deleted comment has no content. |
| `mentioned_email_addresses` | Vec<String> |  | Output only. The emails of the users who were mentioned in this comment, if none were mentioned this will be an empty list. |
| `assignee_email_address` | String |  | Output only. The email of the user who is assigned to this comment, if none is assigned this will be unset. |
| `id` | String |  | Output only. The ID of the comment. |
| `quoted_file_content` | String |  | The file content to which the comment refers, typically within the anchor region. For a text file, for example, this would be the text at the location of the comment. |
| `created_time` | String |  | The time at which the comment was created (RFC 3339 date-time). |
| `replies` | Vec<String> |  | Output only. The full list of replies to the comment in chronological order. |
| `resolved` | bool |  | Output only. Whether the comment has been resolved by one of its replies. |
| `content` | String |  | The plain text content of the comment. This field is used for setting the content, while `htmlContent` should be displayed. |
| `kind` | String |  | Output only. Identifies what kind of resource this is. Value: the fixed string `"drive#comment"`. |
| `html_content` | String |  | Output only. The content of the comment with HTML formatting. |
| `file_id` | String | ✅ | The ID of the file. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `modified_time` | String | The last time the comment or any of its replies was modified (RFC 3339 date-time). |
| `anchor` | String | A region of the document represented as a JSON string. For details on defining anchor properties, refer to [Manage comments and replies](https://developers.google.com/workspace/drive/api/v3/manage-comments). |
| `author` | String | Output only. The author of the comment. The author's email address and permission ID will not be populated. |
| `deleted` | bool | Output only. Whether the comment has been deleted. A deleted comment has no content. |
| `mentioned_email_addresses` | Vec<String> | Output only. The emails of the users who were mentioned in this comment, if none were mentioned this will be an empty list. |
| `assignee_email_address` | String | Output only. The email of the user who is assigned to this comment, if none is assigned this will be unset. |
| `id` | String | Output only. The ID of the comment. |
| `quoted_file_content` | String | The file content to which the comment refers, typically within the anchor region. For a text file, for example, this would be the text at the location of the comment. |
| `created_time` | String | The time at which the comment was created (RFC 3339 date-time). |
| `replies` | Vec<String> | Output only. The full list of replies to the comment in chronological order. |
| `resolved` | bool | Output only. Whether the comment has been resolved by one of its replies. |
| `content` | String | The plain text content of the comment. This field is used for setting the content, while `htmlContent` should be displayed. |
| `kind` | String | Output only. Identifies what kind of resource this is. Value: the fixed string `"drive#comment"`. |
| `html_content` | String | Output only. The content of the comment with HTML formatting. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create comment
comment = provider.drive_api.Comment {
    file_id = "value"  # The ID of the file.
}

# Access comment outputs
comment_id = comment.id
comment_modified_time = comment.modified_time
comment_anchor = comment.anchor
comment_author = comment.author
comment_deleted = comment.deleted
comment_mentioned_email_addresses = comment.mentioned_email_addresses
comment_assignee_email_address = comment.assignee_email_address
comment_id = comment.id
comment_quoted_file_content = comment.quoted_file_content
comment_created_time = comment.created_time
comment_replies = comment.replies
comment_resolved = comment.resolved
comment_content = comment.content
comment_kind = comment.kind
comment_html_content = comment.html_content
```

---


### Change

Subscribes to changes for a user. For more information, see [Notifications for resource changes](https://developers.google.com/workspace/drive/api/guides/push).

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `token` | String |  | An arbitrary string delivered to the target address with each notification delivered over this channel. Optional. |
| `params` | HashMap<String, String> |  | Additional parameters controlling delivery channel behavior. Optional. |
| `id` | String |  | A UUID or similar unique string that identifies this channel. |
| `payload` | bool |  | A Boolean value to indicate whether payload is wanted. Optional. |
| `address` | String |  | The address where notifications are delivered for this channel. |
| `resource_uri` | String |  | A version-specific identifier for the watched resource. |
| `type` | String |  | The type of delivery mechanism used for this channel. Valid values are "web_hook" or "webhook". |
| `expiration` | String |  | Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional. |
| `kind` | String |  | Identifies this as a notification channel used to watch for changes to a resource, which is `api#channel`. |
| `resource_id` | String |  | An opaque ID that identifies the resource being watched on this channel. Stable across different API versions. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string `"drive#startPageToken"`. |
| `start_page_token` | String | The starting page token for listing future changes. The page token doesn't expire. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create change
change = provider.drive_api.Change {
}

# Access change outputs
change_id = change.id
change_kind = change.kind
change_start_page_token = change.start_page_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple change resources
change_0 = provider.drive_api.Change {
}
change_1 = provider.drive_api.Change {
}
change_2 = provider.drive_api.Change {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    change = provider.drive_api.Change {
    }
```

---

## Related Documentation

- [GCP Drive_api Documentation](https://cloud.google.com/drive_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
