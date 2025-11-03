# Drive_api Service



**Resources**: 27

---

## Overview

The drive_api service provides access to 27 resource types:

- [Teamdrive](#teamdrive) [CRUD]
- [Comment](#comment) [CRUD]
- [About](#about) [R]
- [Children](#children) [CRD]
- [App](#app) [R]
- [File](#file) [CRUD]
- [Propertie](#propertie) [CRUD]
- [Drive](#drive) [CRUD]
- [Permission](#permission) [CRUD]
- [Channel](#channel) [C]
- [Parent](#parent) [CRD]
- [Revision](#revision) [RUD]
- [Change](#change) [CR]
- [Replie](#replie) [CRUD]
- [Change](#change) [CR]
- [Revision](#revision) [RUD]
- [App](#app) [R]
- [Accessproposal](#accessproposal) [CR]
- [Replie](#replie) [CRUD]
- [Permission](#permission) [CRUD]
- [Channel](#channel) [C]
- [Drive](#drive) [CRUD]
- [Operation](#operation) [R]
- [Comment](#comment) [CRUD]
- [About](#about) [R]
- [Teamdrive](#teamdrive) [CRUD]
- [File](#file) [CRUD]

---

## Resources


### Teamdrive

Deprecated: Use `drives.insert` instead.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `restrictions` | String |  | A set of restrictions that apply to this Team Drive or items inside this Team Drive. |
| `theme_id` | String |  | The ID of the theme from which the background image and color will be set. The set of possible `teamDriveThemes` can be retrieved from a `drive.about.get` response. When not specified on a `drive.teamdrives.insert` request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set `colorRgb` or `backgroundImageFile`. |
| `created_date` | String |  | The time at which the Team Drive was created (RFC 3339 date-time). |
| `background_image_link` | String |  | A short-lived link to this Team Drive's background image. |
| `id` | String |  | The ID of this Team Drive which is also the ID of the top level folder of this Team Drive. |
| `kind` | String |  | This is always `drive#teamDrive` |
| `background_image_file` | String |  | An image file and cropping parameters from which a background image for this Team Drive is set. This is a write only field; it can only be set on `drive.teamdrives.update` requests that don't set `themeId`. When specified, all fields of the `backgroundImageFile` must be set. |
| `capabilities` | String |  | Capabilities the current user has on this Team Drive. |
| `name` | String |  | The name of this Team Drive. |
| `color_rgb` | String |  | The color of this Team Drive as an RGB hex string. It can only be set on a `drive.teamdrives.update` request that does not set `themeId`. |
| `org_unit_id` | String |  | The organizational unit of this shared drive. This field is only populated on `drives.list` responses when the `useDomainAdminAccess` parameter is set to `true`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `restrictions` | String | A set of restrictions that apply to this Team Drive or items inside this Team Drive. |
| `theme_id` | String | The ID of the theme from which the background image and color will be set. The set of possible `teamDriveThemes` can be retrieved from a `drive.about.get` response. When not specified on a `drive.teamdrives.insert` request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set `colorRgb` or `backgroundImageFile`. |
| `created_date` | String | The time at which the Team Drive was created (RFC 3339 date-time). |
| `background_image_link` | String | A short-lived link to this Team Drive's background image. |
| `id` | String | The ID of this Team Drive which is also the ID of the top level folder of this Team Drive. |
| `kind` | String | This is always `drive#teamDrive` |
| `background_image_file` | String | An image file and cropping parameters from which a background image for this Team Drive is set. This is a write only field; it can only be set on `drive.teamdrives.update` requests that don't set `themeId`. When specified, all fields of the `backgroundImageFile` must be set. |
| `capabilities` | String | Capabilities the current user has on this Team Drive. |
| `name` | String | The name of this Team Drive. |
| `color_rgb` | String | The color of this Team Drive as an RGB hex string. It can only be set on a `drive.teamdrives.update` request that does not set `themeId`. |
| `org_unit_id` | String | The organizational unit of this shared drive. This field is only populated on `drives.list` responses when the `useDomainAdminAccess` parameter is set to `true`. |


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
teamdrive_created_date = teamdrive.created_date
teamdrive_background_image_link = teamdrive.background_image_link
teamdrive_id = teamdrive.id
teamdrive_kind = teamdrive.kind
teamdrive_background_image_file = teamdrive.background_image_file
teamdrive_capabilities = teamdrive.capabilities
teamdrive_name = teamdrive.name
teamdrive_color_rgb = teamdrive.color_rgb
teamdrive_org_unit_id = teamdrive.org_unit_id
```

---


### Comment

Creates a new comment on the given file.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `content` | String |  | The plain text content used to create this comment. This is not HTML safe and should only be used as a starting point to make edits to a comment's content. |
| `kind` | String |  | This is always drive#comment. |
| `created_date` | String |  | The date when this comment was first created. |
| `anchor` | String |  | A region of the document represented as a JSON string. For details on defining anchor properties, refer to [Manage comments and replies](https://developers.google.com/workspace/drive/api/v3/manage-comments). |
| `file_title` | String |  | The title of the file which this comment is addressing. |
| `status` | String |  | The status of this comment. Status can be changed by posting a reply to a comment with the desired status. Possible values are: * `open` - The comment is still open. * `resolved` - The comment has been resolved by one of its replies. |
| `self_link` | String |  | A link back to this comment. |
| `context` | String |  | Context of a file which is being commented on. |
| `file_id` | String |  | The file which this comment is addressing. |
| `comment_id` | String |  | The ID of the comment. |
| `author` | String |  | The user who wrote this comment. |
| `modified_date` | String |  | The date when this comment or any of its replies were last modified. |
| `replies` | Vec<String> |  | Replies to this post. |
| `html_content` | String |  | HTML formatted content for this comment. |
| `deleted` | bool |  | Whether this comment has been deleted. If a comment has been deleted the content will be cleared and this will only represent a comment that once existed. |
| `file_id` | String | ✅ | The ID of the file. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `content` | String | The plain text content used to create this comment. This is not HTML safe and should only be used as a starting point to make edits to a comment's content. |
| `kind` | String | This is always drive#comment. |
| `created_date` | String | The date when this comment was first created. |
| `anchor` | String | A region of the document represented as a JSON string. For details on defining anchor properties, refer to [Manage comments and replies](https://developers.google.com/workspace/drive/api/v3/manage-comments). |
| `file_title` | String | The title of the file which this comment is addressing. |
| `status` | String | The status of this comment. Status can be changed by posting a reply to a comment with the desired status. Possible values are: * `open` - The comment is still open. * `resolved` - The comment has been resolved by one of its replies. |
| `self_link` | String | A link back to this comment. |
| `context` | String | Context of a file which is being commented on. |
| `file_id` | String | The file which this comment is addressing. |
| `comment_id` | String | The ID of the comment. |
| `author` | String | The user who wrote this comment. |
| `modified_date` | String | The date when this comment or any of its replies were last modified. |
| `replies` | Vec<String> | Replies to this post. |
| `html_content` | String | HTML formatted content for this comment. |
| `deleted` | bool | Whether this comment has been deleted. If a comment has been deleted the content will be cleared and this will only represent a comment that once existed. |


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
comment_kind = comment.kind
comment_created_date = comment.created_date
comment_anchor = comment.anchor
comment_file_title = comment.file_title
comment_status = comment.status
comment_self_link = comment.self_link
comment_context = comment.context
comment_file_id = comment.file_id
comment_comment_id = comment.comment_id
comment_author = comment.author
comment_modified_date = comment.modified_date
comment_replies = comment.replies
comment_html_content = comment.html_content
comment_deleted = comment.deleted
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
| `domain_sharing_policy` | String | The domain sharing policy for the current user. Possible values are: * `allowed` * `allowedWithWarning` * `incomingOnly` * `disallowed` |
| `import_formats` | Vec<String> | The allowable import formats. |
| `name` | String | The name of the current user. |
| `quota_bytes_used_aggregate` | String | The number of quota bytes used by all Google apps (Drive, Picasa, etc.). |
| `quota_bytes_by_service` | Vec<String> | The amount of storage quota used by different Google services. |
| `remaining_change_ids` | String | The number of remaining change ids, limited to no more than 2500. |
| `language_code` | String | The user's language or locale code, as defined by BCP 47, with some extensions from Unicode's LDML format (http://www.unicode.org/reports/tr35/). |
| `additional_role_info` | Vec<String> | Information about supported additional roles per file type. The most specific type takes precedence. |
| `permission_id` | String | The current user's ID as visible in the permissions collection. |
| `largest_change_id` | String | The largest change id. |
| `max_upload_sizes` | Vec<String> | List of max upload sizes for each file type. The most specific type takes precedence. |
| `quota_bytes_used_in_trash` | String | The number of quota bytes used by trashed items. |
| `quota_bytes_used` | String | The number of quota bytes used by Google Drive. |
| `quota_type` | String | The type of the user's storage quota. Possible values are: * `LIMITED` * `UNLIMITED` |
| `can_create_drives` | bool | Whether the user can create shared drives. |
| `can_create_team_drives` | bool | Deprecated: Use `canCreateDrives` instead. |
| `is_current_app_installed` | bool | A boolean indicating whether the authenticated app is installed by the authenticated user. |
| `kind` | String | This is always `drive#about`. |
| `folder_color_palette` | Vec<String> | The palette of allowable folder colors as RGB hex strings. |
| `self_link` | String | A link back to this item. |
| `team_drive_themes` | Vec<String> | Deprecated: Use `driveThemes` instead. |
| `user` | String | The authenticated user. |
| `drive_themes` | Vec<String> | A list of themes that are supported for shared drives. |
| `quota_bytes_total` | String | The total number of quota bytes. This is only relevant when quotaType is LIMITED. |
| `etag` | String | The ETag of the item. |
| `export_formats` | Vec<String> | The allowable export formats. |
| `features` | Vec<String> | List of additional features enabled on this account. |
| `root_folder_id` | String | The id of the root folder. |


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
about_domain_sharing_policy = about.domain_sharing_policy
about_import_formats = about.import_formats
about_name = about.name
about_quota_bytes_used_aggregate = about.quota_bytes_used_aggregate
about_quota_bytes_by_service = about.quota_bytes_by_service
about_remaining_change_ids = about.remaining_change_ids
about_language_code = about.language_code
about_additional_role_info = about.additional_role_info
about_permission_id = about.permission_id
about_largest_change_id = about.largest_change_id
about_max_upload_sizes = about.max_upload_sizes
about_quota_bytes_used_in_trash = about.quota_bytes_used_in_trash
about_quota_bytes_used = about.quota_bytes_used
about_quota_type = about.quota_type
about_can_create_drives = about.can_create_drives
about_can_create_team_drives = about.can_create_team_drives
about_is_current_app_installed = about.is_current_app_installed
about_kind = about.kind
about_folder_color_palette = about.folder_color_palette
about_self_link = about.self_link
about_team_drive_themes = about.team_drive_themes
about_user = about.user
about_drive_themes = about.drive_themes
about_quota_bytes_total = about.quota_bytes_total
about_etag = about.etag
about_export_formats = about.export_formats
about_features = about.features
about_root_folder_id = about.root_folder_id
```

---


### Children

Inserts a file into a folder.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `child_link` | String |  | Output only. A link to the child. |
| `kind` | String |  | Output only. This is always `drive#childReference`. |
| `self_link` | String |  | Output only. A link back to this reference. |
| `id` | String |  | The ID of the child. |
| `folder_id` | String | ✅ | The ID of the folder. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `child_link` | String | Output only. A link to the child. |
| `kind` | String | Output only. This is always `drive#childReference`. |
| `self_link` | String | Output only. A link back to this reference. |
| `id` | String | The ID of the child. |


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
children_child_link = children.child_link
children_kind = children.kind
children_self_link = children.self_link
children_id = children.id
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
| `name` | String | The name of the app. |
| `authorized` | bool | Whether the app is authorized to access data on the user's Drive. |
| `icons` | Vec<String> | The various icons for the app. |
| `secondary_file_extensions` | Vec<String> | The list of secondary file extensions. |
| `primary_mime_types` | Vec<String> | The list of primary mime types. |
| `kind` | String | This is always `drive#app`. |
| `id` | String | The ID of the app. |
| `primary_file_extensions` | Vec<String> | The list of primary file extensions. |
| `short_description` | String | A short description of the app. |
| `create_in_folder_template` | String | The template url to create a new file with this app in a given folder. The template will contain {folderId} to be replaced by the folder to create the new file in. |
| `installed` | bool | Whether the app is installed. |
| `product_url` | String | A link to the product listing for this app. |
| `product_id` | String | The ID of the product listing for this app. |
| `object_type` | String | The type of object this app creates (e.g. Chart). If empty, the app name should be used instead. |
| `secondary_mime_types` | Vec<String> | The list of secondary mime types. |
| `long_description` | String | A long description of the app. |
| `supports_create` | bool | Whether this app supports creating new objects. |
| `create_url` | String | The url to create a new file with this app. |
| `has_drive_wide_scope` | bool | Whether the app has drive-wide scope. An app with drive-wide scope can access all files in the user's drive. |
| `supports_import` | bool | Whether this app supports importing from Docs Editors. |
| `supports_offline_create` | bool | Whether this app supports creating new files when offline. |
| `use_by_default` | bool | Whether the app is selected as the default handler for the types it supports. |
| `supports_multi_open` | bool | Whether this app supports opening more than one file. |
| `open_url_template` | String | The template url for opening files with this app. The template will contain `{ids}` and/or `{exportIds}` to be replaced by the actual file ids. See Open Files for the full documentation. |


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
app_name = app.name
app_authorized = app.authorized
app_icons = app.icons
app_secondary_file_extensions = app.secondary_file_extensions
app_primary_mime_types = app.primary_mime_types
app_kind = app.kind
app_id = app.id
app_primary_file_extensions = app.primary_file_extensions
app_short_description = app.short_description
app_create_in_folder_template = app.create_in_folder_template
app_installed = app.installed
app_product_url = app.product_url
app_product_id = app.product_id
app_object_type = app.object_type
app_secondary_mime_types = app.secondary_mime_types
app_long_description = app.long_description
app_supports_create = app.supports_create
app_create_url = app.create_url
app_has_drive_wide_scope = app.has_drive_wide_scope
app_supports_import = app.supports_import
app_supports_offline_create = app.supports_offline_create
app_use_by_default = app.use_by_default
app_supports_multi_open = app.supports_multi_open
app_open_url_template = app.open_url_template
```

---


### File

 Inserts a new file. This method supports an */upload* URI and accepts uploaded media with the following characteristics: - *Maximum file size:* 5,120 GB - *Accepted Media MIME types:*`*/*` Note: Specify a valid MIME type, rather than the literal `*/*` value. The literal `*/*` is only used to indicate that any valid MIME type can be uploaded. For more information on uploading files, see [Upload file data](/workspace/drive/api/guides/manage-uploads). Apps creating shortcuts with `files.insert` must specify the MIME type `application/vnd.google-apps.shortcut`. Apps should specify a file extension in the `title` property when inserting files with the API. For example, an operation to insert a JPEG file should specify something like `"title": "cat.jpg"` in the metadata. Subsequent `GET` requests include the read-only `fileExtension` property populated with the extension originally specified in the `title` property. When a Google Drive user requests to download a file, or when the file is downloaded through the sync client, Drive builds a full filename (with extension) based on the title. In cases where the extension is missing, Drive attempts to determine the extension based on the file's MIME type.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `marked_viewed_by_me_date` | String |  | Deprecated. |
| `export_links` | HashMap<String, String> |  | Output only. Links for exporting Docs Editors files to specific formats. |
| `quota_bytes_used` | String |  | Output only. The number of quota bytes used by this file. |
| `shared_with_me_date` | String |  | Time at which this file was shared with the user (formatted RFC 3339 timestamp). |
| `trashed_date` | String |  | The time that the item was trashed (formatted RFC 3339 timestamp). Only populated for items in shared drives. |
| `trashing_user` | String |  | Output only. If the file has been explicitly trashed, the user who trashed it. Only populated for items in shared drives. |
| `md5_checksum` | String |  | Output only. An MD5 checksum for the content of this file. This field is only populated for files with content stored in Google Drive; it is not populated for Docs Editors or shortcut files. |
| `head_revision_id` | String |  | Output only. The ID of the file's head revision. This field is only populated for files with content stored in Google Drive; it is not populated for Docs Editors or shortcut files. |
| `last_viewed_by_me_date` | String |  | Last time this file was viewed by the user (formatted RFC 3339 timestamp). |
| `mime_type` | String |  | The MIME type of the file. This is only mutable on update when uploading new content. This field can be left blank, and the mimetype will be determined from the uploaded content's MIME type. |
| `description` | String |  | A short description of the file. |
| `explicitly_trashed` | bool |  | Output only. Whether this file has been explicitly trashed, as opposed to recursively trashed. |
| `file_size` | String |  | Output only. Size in bytes of blobs and first party editor files. Won't be populated for files that have no size, like shortcuts and folders. |
| `link_share_metadata` | String |  | Contains details about the link URLs that clients are using to refer to this item. |
| `default_open_with_link` | String |  | Output only. A link to open this file with the user's default app for this file. Only populated when the drive.apps.readonly scope is used. |
| `has_thumbnail` | bool |  | Output only. Whether this file has a thumbnail. This does not indicate whether the requesting app has access to the thumbnail. To check access, look for the presence of the thumbnailLink field. |
| `is_app_authorized` | bool |  | Output only. Whether the file was created or opened by the requesting app. |
| `folder_color_rgb` | String |  | Folder color as an RGB hex string if the file is a folder or a shortcut to a folder. The list of supported colors is available in the folderColorPalette field of the About resource. If an unsupported color is specified, it will be changed to the closest color in the palette. |
| `app_data_contents` | bool |  | Output only. Whether this file is in the Application Data folder. |
| `copy_requires_writer_permission` | bool |  | Whether the options to copy, print, or download this file, should be disabled for readers and commenters. |
| `copyable` | bool |  | Output only. Deprecated: Use `capabilities/canCopy` instead. |
| `resource_key` | String |  | Output only. A key needed to access the item via a shared link. |
| `owner_names` | Vec<String> |  | Output only. Name(s) of the owner(s) of this file. Not populated for items in shared drives. |
| `drive_id` | String |  | Output only. ID of the shared drive the file resides in. Only populated for items in shared drives. |
| `version` | String |  | Output only. A monotonically increasing version number for the file. This reflects every change made to the file on the server, even those not visible to the requesting user. |
| `can_comment` | bool |  | Output only. Deprecated: Use `capabilities/canComment` instead. |
| `etag` | String |  | Output only. ETag of the file. |
| `modified_date` | String |  | Last time this file was modified by anyone (formatted RFC 3339 timestamp). This is only mutable on update when the setModifiedDate parameter is set. |
| `web_content_link` | String |  | Output only. A link for downloading the content of the file in a browser using cookie based authentication. In cases where the content is shared publicly, the content can be downloaded without any credentials. |
| `inherited_permissions_disabled` | bool |  | Whether this file has inherited permissions disabled. Inherited permissions are enabled by default. |
| `last_modifying_user` | String |  | Output only. The last user to modify this file. This field is only populated when the last modification was performed by a signed-in user. |
| `video_media_metadata` | String |  | Output only. Metadata about video media. This will only be present for video types. |
| `labels` | String |  | A group of labels for the file. |
| `alternate_link` | String |  | Output only. A link for opening the file in a relevant Google editor or viewer. |
| `permissions` | Vec<String> |  | Output only. The list of permissions for users with access to this file. Not populated for items in shared drives. |
| `icon_link` | String |  | Output only. A link to the file's icon. |
| `label_info` | String |  | Output only. An overview of the labels on the file. |
| `permission_ids` | Vec<String> |  | Output only. List of permission IDs for users with access to this file. |
| `shared` | bool |  | Output only. Whether the file has been shared. Not populated for items in shared drives. |
| `shortcut_details` | String |  | Shortcut file details. Only populated for shortcut files, which have the mimeType field set to `application/vnd.google-apps.shortcut`. Can only be set on `files.insert` requests. |
| `file_extension` | String |  | Output only. The final component of `fullFileExtension` with trailing text that does not appear to be part of the extension removed. This field is only populated for files with content stored in Google Drive; it is not populated for Docs Editors or shortcut files. |
| `sharing_user` | String |  | Output only. User that shared the item with the current user, if available. |
| `open_with_links` | HashMap<String, String> |  | Output only. A map of the id of each of the user's apps to a link to open this file with that app. Only populated when the drive.apps.readonly scope is used. |
| `shareable` | bool |  | Output only. Deprecated: Use `capabilities/canShare` instead. |
| `id` | String |  | The ID of the file. |
| `kind` | String |  | Output only. The type of file. This is always `drive#file`. |
| `user_permission` | String |  | Output only. The permissions for the authenticated user on this file. |
| `original_filename` | String |  | The original filename of the uploaded content if available, or else the original value of the `title` field. This is only available for files with binary content in Google Drive. |
| `can_read_revisions` | bool |  | Output only. Deprecated: Use `capabilities/canReadRevisions` instead. |
| `spaces` | Vec<String> |  | Output only. The list of spaces which contain the file. Supported values are `drive`, `appDataFolder` and `photos`. |
| `title` | String |  | The title of this file. Note that for immutable items such as the top level folders of shared drives, My Drive root folder, and Application Data folder the title is constant. |
| `full_file_extension` | String |  | Output only. The full file extension; extracted from the title. May contain multiple concatenated extensions, such as "tar.gz". Removing an extension from the title does not clear this field; however, changing the extension on the title does update this field. This field is only populated for files with content stored in Google Drive; it is not populated for Docs Editors or shortcut files. |
| `sha1_checksum` | String |  | Output only. The SHA1 checksum associated with this file, if available. This field is only populated for files with content stored in Google Drive; it is not populated for Docs Editors or shortcut files. |
| `parents` | Vec<String> |  | The ID of the parent folder containing the file. A file can only have one parent folder; specifying multiple parents isn't supported. If not specified as part of an insert request, the file is placed directly in the user's My Drive folder. If not specified as part of a copy request, the file inherits any discoverable parent of the source file. Update requests must use the `addParents` and `removeParents` parameters to modify the parents list. |
| `capabilities` | String |  | Output only. Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take. |
| `embed_link` | String |  | Output only. A link for embedding the file. |
| `properties` | Vec<String> |  | The list of properties. |
| `download_url` | String |  | Output only. Short lived download URL for the file. This field is only populated for files with content stored in Google Drive; it is not populated for Google Docs or shortcut files. |
| `editable` | bool |  | Output only. Deprecated: Use `capabilities/canEdit` instead. |
| `writers_can_share` | bool |  | Whether writers can share the document with other users. Not populated for items in shared drives. |
| `self_link` | String |  | Output only. A link back to this file. |
| `owners` | Vec<String> |  | Output only. The owner of this file. Only certain legacy files may have more than one owner. This field isn't populated for items in shared drives. |
| `owned_by_me` | bool |  | Output only. Whether the file is owned by the current user. Not populated for items in shared drives. |
| `sha256_checksum` | String |  | Output only. The SHA256 checksum associated with this file, if available. This field is only populated for files with content stored in Google Drive; it is not populated for Docs Editors or shortcut files. |
| `web_view_link` | String |  | Output only. A link only available on public folders for viewing their static web assets (HTML, CSS, JS, etc) via Google Drive's Website Hosting. |
| `indexable_text` | String |  | Indexable text attributes for the file (can only be written) |
| `modified_by_me_date` | String |  | Last time this file was modified by the user (formatted RFC 3339 timestamp). Note that setting modifiedDate will also update the modifiedByMe date for the user which set the date. |
| `thumbnail` | String |  | A thumbnail for the file. This will only be used if a standard thumbnail cannot be generated. |
| `team_drive_id` | String |  | Output only. Deprecated: Use `driveId` instead. |
| `thumbnail_link` | String |  | Output only. A short-lived link to the file's thumbnail, if available. Typically lasts on the order of hours. Not intended for direct usage on web applications due to [Cross-Origin Resource Sharing (CORS)](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS), consider using a proxy server. Only populated when the requesting app can access the file's content. If the file isn't shared publicly, the URL returned in `Files.thumbnailLink` must be fetched using a credentialed request. |
| `image_media_metadata` | String |  | Output only. Metadata about image media. This will only be present for image types, and its contents will depend on what can be parsed from the image content. |
| `created_date` | String |  | Create time for this file (formatted RFC 3339 timestamp). |
| `last_modifying_user_name` | String |  | Output only. Name of the last user to modify this file. |
| `thumbnail_version` | String |  | Output only. The thumbnail version for use in thumbnail cache invalidation. |
| `has_augmented_permissions` | bool |  | Output only. Whether there are permissions directly on this file. This field is only populated for items in shared drives. |
| `content_restrictions` | Vec<String> |  | Restrictions for accessing the content of the file. Only populated if such a restriction exists. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `marked_viewed_by_me_date` | String | Deprecated. |
| `export_links` | HashMap<String, String> | Output only. Links for exporting Docs Editors files to specific formats. |
| `quota_bytes_used` | String | Output only. The number of quota bytes used by this file. |
| `shared_with_me_date` | String | Time at which this file was shared with the user (formatted RFC 3339 timestamp). |
| `trashed_date` | String | The time that the item was trashed (formatted RFC 3339 timestamp). Only populated for items in shared drives. |
| `trashing_user` | String | Output only. If the file has been explicitly trashed, the user who trashed it. Only populated for items in shared drives. |
| `md5_checksum` | String | Output only. An MD5 checksum for the content of this file. This field is only populated for files with content stored in Google Drive; it is not populated for Docs Editors or shortcut files. |
| `head_revision_id` | String | Output only. The ID of the file's head revision. This field is only populated for files with content stored in Google Drive; it is not populated for Docs Editors or shortcut files. |
| `last_viewed_by_me_date` | String | Last time this file was viewed by the user (formatted RFC 3339 timestamp). |
| `mime_type` | String | The MIME type of the file. This is only mutable on update when uploading new content. This field can be left blank, and the mimetype will be determined from the uploaded content's MIME type. |
| `description` | String | A short description of the file. |
| `explicitly_trashed` | bool | Output only. Whether this file has been explicitly trashed, as opposed to recursively trashed. |
| `file_size` | String | Output only. Size in bytes of blobs and first party editor files. Won't be populated for files that have no size, like shortcuts and folders. |
| `link_share_metadata` | String | Contains details about the link URLs that clients are using to refer to this item. |
| `default_open_with_link` | String | Output only. A link to open this file with the user's default app for this file. Only populated when the drive.apps.readonly scope is used. |
| `has_thumbnail` | bool | Output only. Whether this file has a thumbnail. This does not indicate whether the requesting app has access to the thumbnail. To check access, look for the presence of the thumbnailLink field. |
| `is_app_authorized` | bool | Output only. Whether the file was created or opened by the requesting app. |
| `folder_color_rgb` | String | Folder color as an RGB hex string if the file is a folder or a shortcut to a folder. The list of supported colors is available in the folderColorPalette field of the About resource. If an unsupported color is specified, it will be changed to the closest color in the palette. |
| `app_data_contents` | bool | Output only. Whether this file is in the Application Data folder. |
| `copy_requires_writer_permission` | bool | Whether the options to copy, print, or download this file, should be disabled for readers and commenters. |
| `copyable` | bool | Output only. Deprecated: Use `capabilities/canCopy` instead. |
| `resource_key` | String | Output only. A key needed to access the item via a shared link. |
| `owner_names` | Vec<String> | Output only. Name(s) of the owner(s) of this file. Not populated for items in shared drives. |
| `drive_id` | String | Output only. ID of the shared drive the file resides in. Only populated for items in shared drives. |
| `version` | String | Output only. A monotonically increasing version number for the file. This reflects every change made to the file on the server, even those not visible to the requesting user. |
| `can_comment` | bool | Output only. Deprecated: Use `capabilities/canComment` instead. |
| `etag` | String | Output only. ETag of the file. |
| `modified_date` | String | Last time this file was modified by anyone (formatted RFC 3339 timestamp). This is only mutable on update when the setModifiedDate parameter is set. |
| `web_content_link` | String | Output only. A link for downloading the content of the file in a browser using cookie based authentication. In cases where the content is shared publicly, the content can be downloaded without any credentials. |
| `inherited_permissions_disabled` | bool | Whether this file has inherited permissions disabled. Inherited permissions are enabled by default. |
| `last_modifying_user` | String | Output only. The last user to modify this file. This field is only populated when the last modification was performed by a signed-in user. |
| `video_media_metadata` | String | Output only. Metadata about video media. This will only be present for video types. |
| `labels` | String | A group of labels for the file. |
| `alternate_link` | String | Output only. A link for opening the file in a relevant Google editor or viewer. |
| `permissions` | Vec<String> | Output only. The list of permissions for users with access to this file. Not populated for items in shared drives. |
| `icon_link` | String | Output only. A link to the file's icon. |
| `label_info` | String | Output only. An overview of the labels on the file. |
| `permission_ids` | Vec<String> | Output only. List of permission IDs for users with access to this file. |
| `shared` | bool | Output only. Whether the file has been shared. Not populated for items in shared drives. |
| `shortcut_details` | String | Shortcut file details. Only populated for shortcut files, which have the mimeType field set to `application/vnd.google-apps.shortcut`. Can only be set on `files.insert` requests. |
| `file_extension` | String | Output only. The final component of `fullFileExtension` with trailing text that does not appear to be part of the extension removed. This field is only populated for files with content stored in Google Drive; it is not populated for Docs Editors or shortcut files. |
| `sharing_user` | String | Output only. User that shared the item with the current user, if available. |
| `open_with_links` | HashMap<String, String> | Output only. A map of the id of each of the user's apps to a link to open this file with that app. Only populated when the drive.apps.readonly scope is used. |
| `shareable` | bool | Output only. Deprecated: Use `capabilities/canShare` instead. |
| `id` | String | The ID of the file. |
| `kind` | String | Output only. The type of file. This is always `drive#file`. |
| `user_permission` | String | Output only. The permissions for the authenticated user on this file. |
| `original_filename` | String | The original filename of the uploaded content if available, or else the original value of the `title` field. This is only available for files with binary content in Google Drive. |
| `can_read_revisions` | bool | Output only. Deprecated: Use `capabilities/canReadRevisions` instead. |
| `spaces` | Vec<String> | Output only. The list of spaces which contain the file. Supported values are `drive`, `appDataFolder` and `photos`. |
| `title` | String | The title of this file. Note that for immutable items such as the top level folders of shared drives, My Drive root folder, and Application Data folder the title is constant. |
| `full_file_extension` | String | Output only. The full file extension; extracted from the title. May contain multiple concatenated extensions, such as "tar.gz". Removing an extension from the title does not clear this field; however, changing the extension on the title does update this field. This field is only populated for files with content stored in Google Drive; it is not populated for Docs Editors or shortcut files. |
| `sha1_checksum` | String | Output only. The SHA1 checksum associated with this file, if available. This field is only populated for files with content stored in Google Drive; it is not populated for Docs Editors or shortcut files. |
| `parents` | Vec<String> | The ID of the parent folder containing the file. A file can only have one parent folder; specifying multiple parents isn't supported. If not specified as part of an insert request, the file is placed directly in the user's My Drive folder. If not specified as part of a copy request, the file inherits any discoverable parent of the source file. Update requests must use the `addParents` and `removeParents` parameters to modify the parents list. |
| `capabilities` | String | Output only. Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take. |
| `embed_link` | String | Output only. A link for embedding the file. |
| `properties` | Vec<String> | The list of properties. |
| `download_url` | String | Output only. Short lived download URL for the file. This field is only populated for files with content stored in Google Drive; it is not populated for Google Docs or shortcut files. |
| `editable` | bool | Output only. Deprecated: Use `capabilities/canEdit` instead. |
| `writers_can_share` | bool | Whether writers can share the document with other users. Not populated for items in shared drives. |
| `self_link` | String | Output only. A link back to this file. |
| `owners` | Vec<String> | Output only. The owner of this file. Only certain legacy files may have more than one owner. This field isn't populated for items in shared drives. |
| `owned_by_me` | bool | Output only. Whether the file is owned by the current user. Not populated for items in shared drives. |
| `sha256_checksum` | String | Output only. The SHA256 checksum associated with this file, if available. This field is only populated for files with content stored in Google Drive; it is not populated for Docs Editors or shortcut files. |
| `web_view_link` | String | Output only. A link only available on public folders for viewing their static web assets (HTML, CSS, JS, etc) via Google Drive's Website Hosting. |
| `indexable_text` | String | Indexable text attributes for the file (can only be written) |
| `modified_by_me_date` | String | Last time this file was modified by the user (formatted RFC 3339 timestamp). Note that setting modifiedDate will also update the modifiedByMe date for the user which set the date. |
| `thumbnail` | String | A thumbnail for the file. This will only be used if a standard thumbnail cannot be generated. |
| `team_drive_id` | String | Output only. Deprecated: Use `driveId` instead. |
| `thumbnail_link` | String | Output only. A short-lived link to the file's thumbnail, if available. Typically lasts on the order of hours. Not intended for direct usage on web applications due to [Cross-Origin Resource Sharing (CORS)](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS), consider using a proxy server. Only populated when the requesting app can access the file's content. If the file isn't shared publicly, the URL returned in `Files.thumbnailLink` must be fetched using a credentialed request. |
| `image_media_metadata` | String | Output only. Metadata about image media. This will only be present for image types, and its contents will depend on what can be parsed from the image content. |
| `created_date` | String | Create time for this file (formatted RFC 3339 timestamp). |
| `last_modifying_user_name` | String | Output only. Name of the last user to modify this file. |
| `thumbnail_version` | String | Output only. The thumbnail version for use in thumbnail cache invalidation. |
| `has_augmented_permissions` | bool | Output only. Whether there are permissions directly on this file. This field is only populated for items in shared drives. |
| `content_restrictions` | Vec<String> | Restrictions for accessing the content of the file. Only populated if such a restriction exists. |


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
file_marked_viewed_by_me_date = file.marked_viewed_by_me_date
file_export_links = file.export_links
file_quota_bytes_used = file.quota_bytes_used
file_shared_with_me_date = file.shared_with_me_date
file_trashed_date = file.trashed_date
file_trashing_user = file.trashing_user
file_md5_checksum = file.md5_checksum
file_head_revision_id = file.head_revision_id
file_last_viewed_by_me_date = file.last_viewed_by_me_date
file_mime_type = file.mime_type
file_description = file.description
file_explicitly_trashed = file.explicitly_trashed
file_file_size = file.file_size
file_link_share_metadata = file.link_share_metadata
file_default_open_with_link = file.default_open_with_link
file_has_thumbnail = file.has_thumbnail
file_is_app_authorized = file.is_app_authorized
file_folder_color_rgb = file.folder_color_rgb
file_app_data_contents = file.app_data_contents
file_copy_requires_writer_permission = file.copy_requires_writer_permission
file_copyable = file.copyable
file_resource_key = file.resource_key
file_owner_names = file.owner_names
file_drive_id = file.drive_id
file_version = file.version
file_can_comment = file.can_comment
file_etag = file.etag
file_modified_date = file.modified_date
file_web_content_link = file.web_content_link
file_inherited_permissions_disabled = file.inherited_permissions_disabled
file_last_modifying_user = file.last_modifying_user
file_video_media_metadata = file.video_media_metadata
file_labels = file.labels
file_alternate_link = file.alternate_link
file_permissions = file.permissions
file_icon_link = file.icon_link
file_label_info = file.label_info
file_permission_ids = file.permission_ids
file_shared = file.shared
file_shortcut_details = file.shortcut_details
file_file_extension = file.file_extension
file_sharing_user = file.sharing_user
file_open_with_links = file.open_with_links
file_shareable = file.shareable
file_id = file.id
file_kind = file.kind
file_user_permission = file.user_permission
file_original_filename = file.original_filename
file_can_read_revisions = file.can_read_revisions
file_spaces = file.spaces
file_title = file.title
file_full_file_extension = file.full_file_extension
file_sha1_checksum = file.sha1_checksum
file_parents = file.parents
file_capabilities = file.capabilities
file_embed_link = file.embed_link
file_properties = file.properties
file_download_url = file.download_url
file_editable = file.editable
file_writers_can_share = file.writers_can_share
file_self_link = file.self_link
file_owners = file.owners
file_owned_by_me = file.owned_by_me
file_sha256_checksum = file.sha256_checksum
file_web_view_link = file.web_view_link
file_indexable_text = file.indexable_text
file_modified_by_me_date = file.modified_by_me_date
file_thumbnail = file.thumbnail
file_team_drive_id = file.team_drive_id
file_thumbnail_link = file.thumbnail_link
file_image_media_metadata = file.image_media_metadata
file_created_date = file.created_date
file_last_modifying_user_name = file.last_modifying_user_name
file_thumbnail_version = file.thumbnail_version
file_has_augmented_permissions = file.has_augmented_permissions
file_content_restrictions = file.content_restrictions
```

---


### Propertie

Adds a property to a file, or updates it if it already exists.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `etag` | String |  | Output only. ETag of the property. |
| `value` | String |  | The value of this property. |
| `self_link` | String |  | Output only. The link back to this property. |
| `visibility` | String |  | The visibility of this property. Allowed values are PRIVATE (default) and PUBLIC. Private properties can only be retrieved using an authenticated request. An authenticated request uses an access token obtained with a OAuth 2 client ID. You cannot use an API key to retrieve private properties. |
| `kind` | String |  | Output only. This is always `drive#property`. |
| `key` | String |  | The key of this property. |
| `file_id` | String | ✅ | The ID of the file. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | Output only. ETag of the property. |
| `value` | String | The value of this property. |
| `self_link` | String | Output only. The link back to this property. |
| `visibility` | String | The visibility of this property. Allowed values are PRIVATE (default) and PUBLIC. Private properties can only be retrieved using an authenticated request. An authenticated request uses an access token obtained with a OAuth 2 client ID. You cannot use an API key to retrieve private properties. |
| `kind` | String | Output only. This is always `drive#property`. |
| `key` | String | The key of this property. |


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
propertie_etag = propertie.etag
propertie_value = propertie.value
propertie_self_link = propertie.self_link
propertie_visibility = propertie.visibility
propertie_kind = propertie.kind
propertie_key = propertie.key
```

---


### Drive

Creates a new shared drive.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `org_unit_id` | String |  | Output only. The organizational unit of this shared drive. This field is only populated on `drives.list` responses when the `useDomainAdminAccess` parameter is set to `true`. |
| `color_rgb` | String |  | The color of this shared drive as an RGB hex string. It can only be set on a `drive.drives.update` request that does not set `themeId`. |
| `created_date` | String |  | The time at which the shared drive was created (RFC 3339 date-time). |
| `kind` | String |  | Output only. This is always `drive#drive` |
| `restrictions` | String |  | A set of restrictions that apply to this shared drive or items inside this shared drive. |
| `theme_id` | String |  | The ID of the theme from which the background image and color will be set. The set of possible `driveThemes` can be retrieved from a `drive.about.get` response. When not specified on a `drive.drives.insert` request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set `colorRgb` or `backgroundImageFile`. |
| `capabilities` | String |  | Output only. Capabilities the current user has on this shared drive. |
| `background_image_file` | String |  | An image file and cropping parameters from which a background image for this shared drive is set. This is a write only field; it can only be set on `drive.drives.update` requests that don't set `themeId`. When specified, all fields of the `backgroundImageFile` must be set. |
| `background_image_link` | String |  | Output only. A short-lived link to this shared drive's background image. |
| `name` | String |  | The name of this shared drive. |
| `hidden` | bool |  | Whether the shared drive is hidden from default view. |
| `id` | String |  | Output only. The ID of this shared drive which is also the ID of the top level folder of this shared drive. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `org_unit_id` | String | Output only. The organizational unit of this shared drive. This field is only populated on `drives.list` responses when the `useDomainAdminAccess` parameter is set to `true`. |
| `color_rgb` | String | The color of this shared drive as an RGB hex string. It can only be set on a `drive.drives.update` request that does not set `themeId`. |
| `created_date` | String | The time at which the shared drive was created (RFC 3339 date-time). |
| `kind` | String | Output only. This is always `drive#drive` |
| `restrictions` | String | A set of restrictions that apply to this shared drive or items inside this shared drive. |
| `theme_id` | String | The ID of the theme from which the background image and color will be set. The set of possible `driveThemes` can be retrieved from a `drive.about.get` response. When not specified on a `drive.drives.insert` request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set `colorRgb` or `backgroundImageFile`. |
| `capabilities` | String | Output only. Capabilities the current user has on this shared drive. |
| `background_image_file` | String | An image file and cropping parameters from which a background image for this shared drive is set. This is a write only field; it can only be set on `drive.drives.update` requests that don't set `themeId`. When specified, all fields of the `backgroundImageFile` must be set. |
| `background_image_link` | String | Output only. A short-lived link to this shared drive's background image. |
| `name` | String | The name of this shared drive. |
| `hidden` | bool | Whether the shared drive is hidden from default view. |
| `id` | String | Output only. The ID of this shared drive which is also the ID of the top level folder of this shared drive. |


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
drive_org_unit_id = drive.org_unit_id
drive_color_rgb = drive.color_rgb
drive_created_date = drive.created_date
drive_kind = drive.kind
drive_restrictions = drive.restrictions
drive_theme_id = drive.theme_id
drive_capabilities = drive.capabilities
drive_background_image_file = drive.background_image_file
drive_background_image_link = drive.background_image_link
drive_name = drive.name
drive_hidden = drive.hidden
drive_id = drive.id
```

---


### Permission

Inserts a permission for a file or shared drive. **Warning:** Concurrent permissions operations on the same file are not supported; only the last update is applied.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `deleted` | bool |  | Output only. Whether the account associated with this permission has been deleted. This field only pertains to user and group permissions. |
| `etag` | String |  | Output only. The ETag of the permission. |
| `auth_key` | String |  | Output only. Deprecated. |
| `expiration_date` | String |  | The time at which this permission will expire (RFC 3339 date-time). Expiration dates have the following restrictions: - They can only be set on user and group permissions - The date must be in the future - The date cannot be more than a year in the future - The date can only be set on drive.permissions.update or drive.permissions.patch requests |
| `view` | String |  | Indicates the view for this permission. Only populated for permissions that belong to a view. published and metadata are the only supported values. - published: The permission's role is published_reader. - metadata: The item is only visible to the metadata view because the item has limited access and the scope has at least read access to the parent. Note: The metadata view is currently only supported on folders.  |
| `additional_roles` | Vec<String> |  | Additional roles for this user. Only `commenter` is currently allowed, though more may be supported in the future. |
| `pending_owner` | bool |  | Whether the account associated with this permission is a pending owner. Only populated for `user` type permissions for files that are not in a shared drive. |
| `with_link` | bool |  | Whether the link is required for this permission. |
| `permission_details` | Vec<String> |  | Output only. Details of whether the permissions on this item are inherited or directly on this item. |
| `domain` | String |  | Output only. The domain name of the entity this permission refers to. This is an output-only field which is present when the permission type is `user`, `group` or `domain`. |
| `name` | String |  | Output only. The name for this permission. |
| `email_address` | String |  | Output only. The email address of the user or group this permission refers to. This is an output-only field which is present when the permission type is `user` or `group`. |
| `self_link` | String |  | Output only. A link back to this permission. |
| `team_drive_permission_details` | Vec<String> |  | Output only. Deprecated: Use `permissionDetails` instead. |
| `id` | String |  | The ID of the user this permission refers to, and identical to the `permissionId` in the About and Files resources. When making a `drive.permissions.insert` request, exactly one of the `id` or `value` fields must be specified unless the permission type is `anyone`, in which case both `id` and `value` are ignored. |
| `inherited_permissions_disabled` | bool |  | When true, only organizers, owners, and users with permissions added directly on the item can access it. |
| `photo_link` | String |  | Output only. A link to the profile photo, if available. |
| `role` | String |  | The primary role for this user. While new values may be supported in the future, the following are currently allowed: * `owner` * `organizer` * `fileOrganizer` * `writer` * `reader` |
| `kind` | String |  | Output only. This is always `drive#permission`. |
| `value` | String |  | The email address or domain name for the entity. This is used during inserts and is not populated in responses. When making a `drive.permissions.insert` request, exactly one of the `id` or `value` fields must be specified unless the permission type is `anyone`, in which case both `id` and `value` are ignored. |
| `type` | String |  | The account type. Allowed values are: * `user` * `group` * `domain` * `anyone` |
| `file_id` | String | ✅ | The ID for the file or shared drive. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `deleted` | bool | Output only. Whether the account associated with this permission has been deleted. This field only pertains to user and group permissions. |
| `etag` | String | Output only. The ETag of the permission. |
| `auth_key` | String | Output only. Deprecated. |
| `expiration_date` | String | The time at which this permission will expire (RFC 3339 date-time). Expiration dates have the following restrictions: - They can only be set on user and group permissions - The date must be in the future - The date cannot be more than a year in the future - The date can only be set on drive.permissions.update or drive.permissions.patch requests |
| `view` | String | Indicates the view for this permission. Only populated for permissions that belong to a view. published and metadata are the only supported values. - published: The permission's role is published_reader. - metadata: The item is only visible to the metadata view because the item has limited access and the scope has at least read access to the parent. Note: The metadata view is currently only supported on folders.  |
| `additional_roles` | Vec<String> | Additional roles for this user. Only `commenter` is currently allowed, though more may be supported in the future. |
| `pending_owner` | bool | Whether the account associated with this permission is a pending owner. Only populated for `user` type permissions for files that are not in a shared drive. |
| `with_link` | bool | Whether the link is required for this permission. |
| `permission_details` | Vec<String> | Output only. Details of whether the permissions on this item are inherited or directly on this item. |
| `domain` | String | Output only. The domain name of the entity this permission refers to. This is an output-only field which is present when the permission type is `user`, `group` or `domain`. |
| `name` | String | Output only. The name for this permission. |
| `email_address` | String | Output only. The email address of the user or group this permission refers to. This is an output-only field which is present when the permission type is `user` or `group`. |
| `self_link` | String | Output only. A link back to this permission. |
| `team_drive_permission_details` | Vec<String> | Output only. Deprecated: Use `permissionDetails` instead. |
| `id` | String | The ID of the user this permission refers to, and identical to the `permissionId` in the About and Files resources. When making a `drive.permissions.insert` request, exactly one of the `id` or `value` fields must be specified unless the permission type is `anyone`, in which case both `id` and `value` are ignored. |
| `inherited_permissions_disabled` | bool | When true, only organizers, owners, and users with permissions added directly on the item can access it. |
| `photo_link` | String | Output only. A link to the profile photo, if available. |
| `role` | String | The primary role for this user. While new values may be supported in the future, the following are currently allowed: * `owner` * `organizer` * `fileOrganizer` * `writer` * `reader` |
| `kind` | String | Output only. This is always `drive#permission`. |
| `value` | String | The email address or domain name for the entity. This is used during inserts and is not populated in responses. When making a `drive.permissions.insert` request, exactly one of the `id` or `value` fields must be specified unless the permission type is `anyone`, in which case both `id` and `value` are ignored. |
| `type` | String | The account type. Allowed values are: * `user` * `group` * `domain` * `anyone` |


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
permission_deleted = permission.deleted
permission_etag = permission.etag
permission_auth_key = permission.auth_key
permission_expiration_date = permission.expiration_date
permission_view = permission.view
permission_additional_roles = permission.additional_roles
permission_pending_owner = permission.pending_owner
permission_with_link = permission.with_link
permission_permission_details = permission.permission_details
permission_domain = permission.domain
permission_name = permission.name
permission_email_address = permission.email_address
permission_self_link = permission.self_link
permission_team_drive_permission_details = permission.team_drive_permission_details
permission_id = permission.id
permission_inherited_permissions_disabled = permission.inherited_permissions_disabled
permission_photo_link = permission.photo_link
permission_role = permission.role
permission_kind = permission.kind
permission_value = permission.value
permission_type = permission.type
```

---


### Channel

Stops watching resources through this channel.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_uri` | String |  | A version-specific identifier for the watched resource. |
| `resource_id` | String |  | An opaque ID that identifies the resource being watched on this channel. Stable across different API versions. |
| `address` | String |  | The address where notifications are delivered for this channel. |
| `expiration` | String |  | Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional. |
| `id` | String |  | A UUID or similar unique string that identifies this channel. |
| `params` | HashMap<String, String> |  | Additional parameters controlling delivery channel behavior. Optional. |
| `kind` | String |  | Identifies this as a notification channel used to watch for changes to a resource, which is `api#channel`. |
| `payload` | bool |  | A Boolean value to indicate whether payload is wanted. Optional. |
| `token` | String |  | An arbitrary string delivered to the target address with each notification delivered over this channel. Optional. |
| `type` | String |  | The type of delivery mechanism used for this channel. Valid values are "web_hook" or "webhook". |



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


### Parent

Adds a parent folder for a file.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String |  | The ID of the parent. |
| `is_root` | bool |  | Output only. Whether or not the parent is the root folder. |
| `kind` | String |  | Output only. This is always `drive#parentReference`. |
| `parent_link` | String |  | Output only. A link to the parent. |
| `self_link` | String |  | Output only. A link back to this reference. |
| `file_id` | String | ✅ | The ID of the file. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | The ID of the parent. |
| `is_root` | bool | Output only. Whether or not the parent is the root folder. |
| `kind` | String | Output only. This is always `drive#parentReference`. |
| `parent_link` | String | Output only. A link to the parent. |
| `self_link` | String | Output only. A link back to this reference. |


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
parent_id = parent.id
parent_is_root = parent.is_root
parent_kind = parent.kind
parent_parent_link = parent.parent_link
parent_self_link = parent.self_link
```

---


### Revision

Gets a specific revision.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `published` | bool |  | Whether this revision is published. This is only populated and can only be modified for Docs Editors files. |
| `published_link` | String |  | Output only. A link to the published revision. This is only populated for Docs Editors files. |
| `publish_auto` | bool |  | Whether subsequent revisions will be automatically republished. This is only populated and can only be modified for Docs Editors files. |
| `export_links` | HashMap<String, String> |  | Output only. Links for exporting Docs Editors files to specific formats. |
| `md5_checksum` | String |  | Output only. An MD5 checksum for the content of this revision. This will only be populated on files with content stored in Drive. |
| `etag` | String |  | Output only. The ETag of the revision. |
| `pinned` | bool |  | Whether this revision is pinned to prevent automatic purging. If not set, the revision is automatically purged 30 days after newer content is uploaded. This field can only be modified on files with content stored in Drive, excluding Docs Editors files. Revisions can also be pinned when they are created through the drive.files.insert/update/copy by using the pinned query parameter. Pinned revisions are stored indefinitely using additional storage quota, up to a maximum of 200 revisions. |
| `id` | String |  | Output only. The ID of the revision. |
| `file_size` | String |  | Output only. The size of the revision in bytes. This will only be populated on files with content stored in Drive. |
| `original_filename` | String |  | Output only. The original filename when this revision was created. This will only be populated on files with content stored in Drive. |
| `published_outside_domain` | bool |  | Whether this revision is published outside the domain. This is only populated and can only be modified for Docs Editors files. |
| `modified_date` | String |  | Last time this revision was modified (formatted RFC 3339 timestamp). |
| `last_modifying_user_name` | String |  | Output only. Name of the last user to modify this revision. |
| `self_link` | String |  | Output only. A link back to this revision. |
| `mime_type` | String |  | Output only. The MIME type of the revision. |
| `last_modifying_user` | String |  | Output only. The last user to modify this revision. This field is only populated when the last modification was performed by a signed-in user. |
| `download_url` | String |  | Output only. Short term download URL for the file. This will only be populated on files with content stored in Drive. |
| `kind` | String |  | Output only. This is always `drive#revision`. |
| `revision_id` | String | ✅ | The ID for the revision. |
| `file_id` | String | ✅ | The ID for the file. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `published` | bool | Whether this revision is published. This is only populated and can only be modified for Docs Editors files. |
| `published_link` | String | Output only. A link to the published revision. This is only populated for Docs Editors files. |
| `publish_auto` | bool | Whether subsequent revisions will be automatically republished. This is only populated and can only be modified for Docs Editors files. |
| `export_links` | HashMap<String, String> | Output only. Links for exporting Docs Editors files to specific formats. |
| `md5_checksum` | String | Output only. An MD5 checksum for the content of this revision. This will only be populated on files with content stored in Drive. |
| `etag` | String | Output only. The ETag of the revision. |
| `pinned` | bool | Whether this revision is pinned to prevent automatic purging. If not set, the revision is automatically purged 30 days after newer content is uploaded. This field can only be modified on files with content stored in Drive, excluding Docs Editors files. Revisions can also be pinned when they are created through the drive.files.insert/update/copy by using the pinned query parameter. Pinned revisions are stored indefinitely using additional storage quota, up to a maximum of 200 revisions. |
| `id` | String | Output only. The ID of the revision. |
| `file_size` | String | Output only. The size of the revision in bytes. This will only be populated on files with content stored in Drive. |
| `original_filename` | String | Output only. The original filename when this revision was created. This will only be populated on files with content stored in Drive. |
| `published_outside_domain` | bool | Whether this revision is published outside the domain. This is only populated and can only be modified for Docs Editors files. |
| `modified_date` | String | Last time this revision was modified (formatted RFC 3339 timestamp). |
| `last_modifying_user_name` | String | Output only. Name of the last user to modify this revision. |
| `self_link` | String | Output only. A link back to this revision. |
| `mime_type` | String | Output only. The MIME type of the revision. |
| `last_modifying_user` | String | Output only. The last user to modify this revision. This field is only populated when the last modification was performed by a signed-in user. |
| `download_url` | String | Output only. Short term download URL for the file. This will only be populated on files with content stored in Drive. |
| `kind` | String | Output only. This is always `drive#revision`. |


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
revision_published_link = revision.published_link
revision_publish_auto = revision.publish_auto
revision_export_links = revision.export_links
revision_md5_checksum = revision.md5_checksum
revision_etag = revision.etag
revision_pinned = revision.pinned
revision_id = revision.id
revision_file_size = revision.file_size
revision_original_filename = revision.original_filename
revision_published_outside_domain = revision.published_outside_domain
revision_modified_date = revision.modified_date
revision_last_modifying_user_name = revision.last_modifying_user_name
revision_self_link = revision.self_link
revision_mime_type = revision.mime_type
revision_last_modifying_user = revision.last_modifying_user
revision_download_url = revision.download_url
revision_kind = revision.kind
```

---


### Change

Subscribe to changes for a user.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_uri` | String |  | A version-specific identifier for the watched resource. |
| `resource_id` | String |  | An opaque ID that identifies the resource being watched on this channel. Stable across different API versions. |
| `address` | String |  | The address where notifications are delivered for this channel. |
| `expiration` | String |  | Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional. |
| `id` | String |  | A UUID or similar unique string that identifies this channel. |
| `params` | HashMap<String, String> |  | Additional parameters controlling delivery channel behavior. Optional. |
| `kind` | String |  | Identifies this as a notification channel used to watch for changes to a resource, which is `api#channel`. |
| `payload` | bool |  | A Boolean value to indicate whether payload is wanted. Optional. |
| `token` | String |  | An arbitrary string delivered to the target address with each notification delivered over this channel. Optional. |
| `type` | String |  | The type of delivery mechanism used for this channel. Valid values are "web_hook" or "webhook". |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `modification_date` | String | The time of this modification. |
| `deleted` | bool | Whether the file or shared drive has been removed from this list of changes, for example by deletion or loss of access. |
| `team_drive` | String | Deprecated: Use `drive` instead. |
| `change_type` | String | The type of the change. Possible values are `file` and `drive`. |
| `kind` | String | This is always `drive#change`. |
| `self_link` | String | A link back to this change. |
| `team_drive_id` | String | Deprecated: Use `driveId` instead. |
| `type` | String | Deprecated: Use `changeType` instead. |
| `drive` | String | The updated state of the shared drive. Present if the changeType is drive, the user is still a member of the shared drive, and the shared drive has not been deleted. |
| `drive_id` | String | The ID of the shared drive associated with this change. |
| `file_id` | String | The ID of the file associated with this change. |
| `file` | String | The updated state of the file. Present if the type is file and the file has not been removed from this list of changes. |
| `id` | String | The ID of the change. |


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
change_modification_date = change.modification_date
change_deleted = change.deleted
change_team_drive = change.team_drive
change_change_type = change.change_type
change_kind = change.kind
change_self_link = change.self_link
change_team_drive_id = change.team_drive_id
change_type = change.type
change_drive = change.drive
change_drive_id = change.drive_id
change_file_id = change.file_id
change_file = change.file
change_id = change.id
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
| `kind` | String |  | This is always drive#commentReply. |
| `modified_date` | String |  | The date when this reply was last modified. |
| `author` | String |  | The user who wrote this reply. |
| `html_content` | String |  | HTML formatted content for this reply. |
| `deleted` | bool |  | Whether this reply has been deleted. If a reply has been deleted the content will be cleared and this will only represent a reply that once existed. |
| `reply_id` | String |  | The ID of the reply. |
| `verb` | String |  | The action this reply performed to the parent comment. When creating a new reply this is the action to be perform tSo the parent comment. Possible values are: * `resolve` - To resolve a comment. * `reopen` - To reopen (un-resolve) a comment. |
| `file_id` | String | ✅ | The ID of the file. |
| `comment_id` | String | ✅ | The ID of the comment. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `content` | String | The plain text content used to create this reply. This is not HTML safe and should only be used as a starting point to make edits to a reply's content. This field is required on inserts if no verb is specified (resolve/reopen). |
| `created_date` | String | The date when this reply was first created. |
| `kind` | String | This is always drive#commentReply. |
| `modified_date` | String | The date when this reply was last modified. |
| `author` | String | The user who wrote this reply. |
| `html_content` | String | HTML formatted content for this reply. |
| `deleted` | bool | Whether this reply has been deleted. If a reply has been deleted the content will be cleared and this will only represent a reply that once existed. |
| `reply_id` | String | The ID of the reply. |
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
replie_kind = replie.kind
replie_modified_date = replie.modified_date
replie_author = replie.author
replie_html_content = replie.html_content
replie_deleted = replie.deleted
replie_reply_id = replie.reply_id
replie_verb = replie.verb
```

---


### Change

Subscribes to changes for a user. For more information, see [Notifications for resource changes](https://developers.google.com/workspace/drive/api/guides/push).

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | String |  | The type of delivery mechanism used for this channel. Valid values are "web_hook" or "webhook". |
| `resource_id` | String |  | An opaque ID that identifies the resource being watched on this channel. Stable across different API versions. |
| `id` | String |  | A UUID or similar unique string that identifies this channel. |
| `params` | HashMap<String, String> |  | Additional parameters controlling delivery channel behavior. Optional. |
| `token` | String |  | An arbitrary string delivered to the target address with each notification delivered over this channel. Optional. |
| `kind` | String |  | Identifies this as a notification channel used to watch for changes to a resource, which is `api#channel`. |
| `resource_uri` | String |  | A version-specific identifier for the watched resource. |
| `address` | String |  | The address where notifications are delivered for this channel. |
| `expiration` | String |  | Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional. |
| `payload` | bool |  | A Boolean value to indicate whether payload is wanted. Optional. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string `"drive#changeList"`. |
| `new_start_page_token` | String | The starting page token for future changes. This will be present only if the end of the current changes list has been reached. The page token doesn't expire. |
| `changes` | Vec<String> | The list of changes. If nextPageToken is populated, then this list may be incomplete and an additional page of results should be fetched. |
| `next_page_token` | String | The page token for the next page of changes. This will be absent if the end of the changes list has been reached. The page token doesn't expire. |


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
change_new_start_page_token = change.new_start_page_token
change_changes = change.changes
change_next_page_token = change.next_page_token
```

---


### Revision

Gets a revision's metadata or content by ID. For more information, see [Manage file revisions](https://developers.google.com/workspace/drive/api/guides/manage-revisions).

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `published_outside_domain` | bool |  | Whether this revision is published outside the domain. This is only applicable to Docs Editors files. |
| `keep_forever` | bool |  | Whether to keep this revision forever, even if it is no longer the head revision. If not set, the revision will be automatically purged 30 days after newer content is uploaded. This can be set on a maximum of 200 revisions for a file. This field is only applicable to files with binary content in Drive. |
| `export_links` | HashMap<String, String> |  | Output only. Links for exporting Docs Editors files to specific formats. |
| `id` | String |  | Output only. The ID of the revision. |
| `md5_checksum` | String |  | Output only. The MD5 checksum of the revision's content. This is only applicable to files with binary content in Drive. |
| `publish_auto` | bool |  | Whether subsequent revisions will be automatically republished. This is only applicable to Docs Editors files. |
| `modified_time` | String |  | The last time the revision was modified (RFC 3339 date-time). |
| `original_filename` | String |  | Output only. The original filename used to create this revision. This is only applicable to files with binary content in Drive. |
| `kind` | String |  | Output only. Identifies what kind of resource this is. Value: the fixed string `"drive#revision"`. |
| `mime_type` | String |  | Output only. The MIME type of the revision. |
| `last_modifying_user` | String |  | Output only. The last user to modify this revision. This field is only populated when the last modification was performed by a signed-in user. |
| `published` | bool |  | Whether this revision is published. This is only applicable to Docs Editors files. |
| `published_link` | String |  | Output only. A link to the published revision. This is only populated for Docs Editors files. |
| `size` | String |  | Output only. The size of the revision's content in bytes. This is only applicable to files with binary content in Drive. |
| `file_id` | String | ✅ | The ID of the file. |
| `revision_id` | String | ✅ | The ID of the revision. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `published_outside_domain` | bool | Whether this revision is published outside the domain. This is only applicable to Docs Editors files. |
| `keep_forever` | bool | Whether to keep this revision forever, even if it is no longer the head revision. If not set, the revision will be automatically purged 30 days after newer content is uploaded. This can be set on a maximum of 200 revisions for a file. This field is only applicable to files with binary content in Drive. |
| `export_links` | HashMap<String, String> | Output only. Links for exporting Docs Editors files to specific formats. |
| `id` | String | Output only. The ID of the revision. |
| `md5_checksum` | String | Output only. The MD5 checksum of the revision's content. This is only applicable to files with binary content in Drive. |
| `publish_auto` | bool | Whether subsequent revisions will be automatically republished. This is only applicable to Docs Editors files. |
| `modified_time` | String | The last time the revision was modified (RFC 3339 date-time). |
| `original_filename` | String | Output only. The original filename used to create this revision. This is only applicable to files with binary content in Drive. |
| `kind` | String | Output only. Identifies what kind of resource this is. Value: the fixed string `"drive#revision"`. |
| `mime_type` | String | Output only. The MIME type of the revision. |
| `last_modifying_user` | String | Output only. The last user to modify this revision. This field is only populated when the last modification was performed by a signed-in user. |
| `published` | bool | Whether this revision is published. This is only applicable to Docs Editors files. |
| `published_link` | String | Output only. A link to the published revision. This is only populated for Docs Editors files. |
| `size` | String | Output only. The size of the revision's content in bytes. This is only applicable to files with binary content in Drive. |


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
revision_published_outside_domain = revision.published_outside_domain
revision_keep_forever = revision.keep_forever
revision_export_links = revision.export_links
revision_id = revision.id
revision_md5_checksum = revision.md5_checksum
revision_publish_auto = revision.publish_auto
revision_modified_time = revision.modified_time
revision_original_filename = revision.original_filename
revision_kind = revision.kind
revision_mime_type = revision.mime_type
revision_last_modifying_user = revision.last_modifying_user
revision_published = revision.published
revision_published_link = revision.published_link
revision_size = revision.size
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
| `name` | String | The name of the app. |
| `supports_import` | bool | Whether this app supports importing from Google Docs. |
| `supports_multi_open` | bool | Whether this app supports opening more than one file. |
| `object_type` | String | The type of object this app creates such as a Chart. If empty, the app name should be used instead. |
| `id` | String | The ID of the app. |
| `primary_mime_types` | Vec<String> | The list of primary MIME types. |
| `installed` | bool | Whether the app is installed. |
| `short_description` | String | A short description of the app. |
| `primary_file_extensions` | Vec<String> | The list of primary file extensions. |
| `supports_offline_create` | bool | Whether this app supports creating files when offline. |
| `use_by_default` | bool | Whether the app is selected as the default handler for the types it supports. |
| `has_drive_wide_scope` | bool | Whether the app has Drive-wide scope. An app with Drive-wide scope can access all files in the user's Drive. |
| `icons` | Vec<String> | The various icons for the app. |
| `secondary_file_extensions` | Vec<String> | The list of secondary file extensions. |
| `kind` | String | Output only. Identifies what kind of resource this is. Value: the fixed string "drive#app". |
| `long_description` | String | A long description of the app. |
| `create_in_folder_template` | String | The template URL to create a file with this app in a given folder. The template contains the {folderId} to be replaced by the folder ID house the new file. |
| `product_id` | String | The ID of the product listing for this app. |
| `create_url` | String | The URL to create a file with this app. |
| `product_url` | String | A link to the product listing for this app. |
| `authorized` | bool | Whether the app is authorized to access data on the user's Drive. |
| `secondary_mime_types` | Vec<String> | The list of secondary MIME types. |
| `supports_create` | bool | Whether this app supports creating objects. |
| `open_url_template` | String | The template URL for opening files with this app. The template contains {ids} or {exportIds} to be replaced by the actual file IDs. For more information, see Open Files for the full documentation. |


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
app_name = app.name
app_supports_import = app.supports_import
app_supports_multi_open = app.supports_multi_open
app_object_type = app.object_type
app_id = app.id
app_primary_mime_types = app.primary_mime_types
app_installed = app.installed
app_short_description = app.short_description
app_primary_file_extensions = app.primary_file_extensions
app_supports_offline_create = app.supports_offline_create
app_use_by_default = app.use_by_default
app_has_drive_wide_scope = app.has_drive_wide_scope
app_icons = app.icons
app_secondary_file_extensions = app.secondary_file_extensions
app_kind = app.kind
app_long_description = app.long_description
app_create_in_folder_template = app.create_in_folder_template
app_product_id = app.product_id
app_create_url = app.create_url
app_product_url = app.product_url
app_authorized = app.authorized
app_secondary_mime_types = app.secondary_mime_types
app_supports_create = app.supports_create
app_open_url_template = app.open_url_template
```

---


### Accessproposal

Approves or denies an access proposal. For more information, see [Manage pending access proposals](https://developers.google.com/workspace/drive/api/guides/pending-access).

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `view` | String |  | Optional. Indicates the view for this access proposal. This should only be set when the proposal belongs to a view. Only `published` is supported. |
| `send_notification` | bool |  | Optional. Whether to send an email to the requester when the access proposal is denied or accepted. |
| `action` | String |  | Required. The action to take on the access proposal. |
| `role` | Vec<String> |  | Optional. The roles that the approver has allowed, if any. For more information, see [Roles and permissions](https://developers.google.com/workspace/drive/api/guides/ref-roles). Note: This field is required for the `ACCEPT` action. |
| `file_id` | String | ✅ | Required. The ID of the item the request is on. |
| `proposal_id` | String | ✅ | Required. The ID of the access proposal to resolve. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `requester_email_address` | String | The email address of the requesting user. |
| `roles_and_views` | Vec<String> | A wrapper for the role and view of an access proposal. For more information, see [Roles and permissions](https://developers.google.com/workspace/drive/api/guides/ref-roles). |
| `request_message` | String | The message that the requester added to the proposal. |
| `recipient_email_address` | String | The email address of the user that will receive permissions, if accepted. |
| `file_id` | String | The file ID that the proposal for access is on. |
| `create_time` | String | The creation time. |
| `proposal_id` | String | The ID of the access proposal. |


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
    file_id = "value"  # Required. The ID of the item the request is on.
    proposal_id = "value"  # Required. The ID of the access proposal to resolve.
}

# Access accessproposal outputs
accessproposal_id = accessproposal.id
accessproposal_requester_email_address = accessproposal.requester_email_address
accessproposal_roles_and_views = accessproposal.roles_and_views
accessproposal_request_message = accessproposal.request_message
accessproposal_recipient_email_address = accessproposal.recipient_email_address
accessproposal_file_id = accessproposal.file_id
accessproposal_create_time = accessproposal.create_time
accessproposal_proposal_id = accessproposal.proposal_id
```

---


### Replie

Creates a reply to a comment.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String |  | Output only. The ID of the reply. |
| `mentioned_email_addresses` | Vec<String> |  | Output only. The emails of the users who were mentioned in this reply, if none were mentioned this will be an empty list. |
| `author` | String |  | Output only. The author of the reply. The author's email address and permission ID will not be populated. |
| `assignee_email_address` | String |  | Output only. The email of the user who is assigned to this reply, if none is assigned this will be unset. |
| `created_time` | String |  | The time at which the reply was created (RFC 3339 date-time). |
| `action` | String |  | The action the reply performed to the parent comment. Valid values are: * `resolve` * `reopen` |
| `content` | String |  | The plain text content of the reply. This field is used for setting the content, while `htmlContent` should be displayed. This is required on creates if no `action` is specified. |
| `deleted` | bool |  | Output only. Whether the reply has been deleted. A deleted reply has no content. |
| `html_content` | String |  | Output only. The content of the reply with HTML formatting. |
| `modified_time` | String |  | The last time the reply was modified (RFC 3339 date-time). |
| `kind` | String |  | Output only. Identifies what kind of resource this is. Value: the fixed string `"drive#reply"`. |
| `comment_id` | String | ✅ | The ID of the comment. |
| `file_id` | String | ✅ | The ID of the file. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | Output only. The ID of the reply. |
| `mentioned_email_addresses` | Vec<String> | Output only. The emails of the users who were mentioned in this reply, if none were mentioned this will be an empty list. |
| `author` | String | Output only. The author of the reply. The author's email address and permission ID will not be populated. |
| `assignee_email_address` | String | Output only. The email of the user who is assigned to this reply, if none is assigned this will be unset. |
| `created_time` | String | The time at which the reply was created (RFC 3339 date-time). |
| `action` | String | The action the reply performed to the parent comment. Valid values are: * `resolve` * `reopen` |
| `content` | String | The plain text content of the reply. This field is used for setting the content, while `htmlContent` should be displayed. This is required on creates if no `action` is specified. |
| `deleted` | bool | Output only. Whether the reply has been deleted. A deleted reply has no content. |
| `html_content` | String | Output only. The content of the reply with HTML formatting. |
| `modified_time` | String | The last time the reply was modified (RFC 3339 date-time). |
| `kind` | String | Output only. Identifies what kind of resource this is. Value: the fixed string `"drive#reply"`. |


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
    comment_id = "value"  # The ID of the comment.
    file_id = "value"  # The ID of the file.
}

# Access replie outputs
replie_id = replie.id
replie_id = replie.id
replie_mentioned_email_addresses = replie.mentioned_email_addresses
replie_author = replie.author
replie_assignee_email_address = replie.assignee_email_address
replie_created_time = replie.created_time
replie_action = replie.action
replie_content = replie.content
replie_deleted = replie.deleted
replie_html_content = replie.html_content
replie_modified_time = replie.modified_time
replie_kind = replie.kind
```

---


### Permission

Creates a permission for a file or shared drive. For more information, see [Share files, folders, and drives](https://developers.google.com/workspace/drive/api/guides/manage-sharing). **Warning:** Concurrent permissions operations on the same file aren't supported; only the last update is applied.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `deleted` | bool |  | Output only. Whether the account associated with this permission has been deleted. This field only pertains to permissions of type `user` or `group`. |
| `id` | String |  | Output only. The ID of this permission. This is a unique identifier for the grantee, and is published in the [User resource](https://developers.google.com/workspace/drive/api/reference/rest/v3/User) as `permissionId`. IDs should be treated as opaque values. |
| `photo_link` | String |  | Output only. A link to the user's profile photo, if available. |
| `allow_file_discovery` | bool |  | Whether the permission allows the file to be discovered through search. This is only applicable for permissions of type `domain` or `anyone`. |
| `pending_owner` | bool |  | Whether the account associated with this permission is a pending owner. Only populated for permissions of type `user` for files that aren't in a shared drive. |
| `kind` | String |  | Output only. Identifies what kind of resource this is. Value: the fixed string `"drive#permission"`. |
| `team_drive_permission_details` | Vec<String> |  | Output only. Deprecated: Output only. Use `permissionDetails` instead. |
| `view` | String |  | Indicates the view for this permission. Only populated for permissions that belong to a view. The only supported values are `published` and `metadata`: * `published`: The permission's role is `publishedReader`. * `metadata`: The item is only visible to the `metadata` view because the item has limited access and the scope has at least read access to the parent. The `metadata` view is only supported on folders. For more information, see [Views](https://developers.google.com/workspace/drive/api/guides/ref-roles#views). |
| `type` | String |  | The type of the grantee. Supported values include: * `user` * `group` * `domain` * `anyone` When creating a permission, if `type` is `user` or `group`, you must provide an `emailAddress` for the user or group. If `type` is `domain`, you must provide a `domain`. If `type` is `anyone`, no extra information is required. |
| `domain` | String |  | The domain to which this permission refers. |
| `inherited_permissions_disabled` | bool |  | When `true`, only organizers, owners, and users with permissions added directly on the item can access it. |
| `permission_details` | Vec<String> |  | Output only. Details of whether the permissions on this item are inherited or are directly on this item. |
| `role` | String |  | The role granted by this permission. Supported values include: * `owner` * `organizer` * `fileOrganizer` * `writer` * `commenter` * `reader` For more information, see [Roles and permissions](https://developers.google.com/workspace/drive/api/guides/ref-roles). |
| `email_address` | String |  | The email address of the user or group to which this permission refers. |
| `expiration_time` | String |  | The time at which this permission will expire (RFC 3339 date-time). Expiration times have the following restrictions: - They can only be set on user and group permissions - The time must be in the future - The time cannot be more than a year in the future |
| `display_name` | String |  | Output only. The "pretty" name of the value of the permission. The following is a list of examples for each type of permission: * `user` - User's full name, as defined for their Google Account, such as "Dana A." * `group` - Name of the Google Group, such as "The Company Administrators." * `domain` - String domain name, such as "cymbalgroup.com." * `anyone` - No `displayName` is present. |
| `file_id` | String | ✅ | The ID of the file or shared drive. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `deleted` | bool | Output only. Whether the account associated with this permission has been deleted. This field only pertains to permissions of type `user` or `group`. |
| `id` | String | Output only. The ID of this permission. This is a unique identifier for the grantee, and is published in the [User resource](https://developers.google.com/workspace/drive/api/reference/rest/v3/User) as `permissionId`. IDs should be treated as opaque values. |
| `photo_link` | String | Output only. A link to the user's profile photo, if available. |
| `allow_file_discovery` | bool | Whether the permission allows the file to be discovered through search. This is only applicable for permissions of type `domain` or `anyone`. |
| `pending_owner` | bool | Whether the account associated with this permission is a pending owner. Only populated for permissions of type `user` for files that aren't in a shared drive. |
| `kind` | String | Output only. Identifies what kind of resource this is. Value: the fixed string `"drive#permission"`. |
| `team_drive_permission_details` | Vec<String> | Output only. Deprecated: Output only. Use `permissionDetails` instead. |
| `view` | String | Indicates the view for this permission. Only populated for permissions that belong to a view. The only supported values are `published` and `metadata`: * `published`: The permission's role is `publishedReader`. * `metadata`: The item is only visible to the `metadata` view because the item has limited access and the scope has at least read access to the parent. The `metadata` view is only supported on folders. For more information, see [Views](https://developers.google.com/workspace/drive/api/guides/ref-roles#views). |
| `type` | String | The type of the grantee. Supported values include: * `user` * `group` * `domain` * `anyone` When creating a permission, if `type` is `user` or `group`, you must provide an `emailAddress` for the user or group. If `type` is `domain`, you must provide a `domain`. If `type` is `anyone`, no extra information is required. |
| `domain` | String | The domain to which this permission refers. |
| `inherited_permissions_disabled` | bool | When `true`, only organizers, owners, and users with permissions added directly on the item can access it. |
| `permission_details` | Vec<String> | Output only. Details of whether the permissions on this item are inherited or are directly on this item. |
| `role` | String | The role granted by this permission. Supported values include: * `owner` * `organizer` * `fileOrganizer` * `writer` * `commenter` * `reader` For more information, see [Roles and permissions](https://developers.google.com/workspace/drive/api/guides/ref-roles). |
| `email_address` | String | The email address of the user or group to which this permission refers. |
| `expiration_time` | String | The time at which this permission will expire (RFC 3339 date-time). Expiration times have the following restrictions: - They can only be set on user and group permissions - The time must be in the future - The time cannot be more than a year in the future |
| `display_name` | String | Output only. The "pretty" name of the value of the permission. The following is a list of examples for each type of permission: * `user` - User's full name, as defined for their Google Account, such as "Dana A." * `group` - Name of the Google Group, such as "The Company Administrators." * `domain` - String domain name, such as "cymbalgroup.com." * `anyone` - No `displayName` is present. |


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
permission_id = permission.id
permission_photo_link = permission.photo_link
permission_allow_file_discovery = permission.allow_file_discovery
permission_pending_owner = permission.pending_owner
permission_kind = permission.kind
permission_team_drive_permission_details = permission.team_drive_permission_details
permission_view = permission.view
permission_type = permission.type
permission_domain = permission.domain
permission_inherited_permissions_disabled = permission.inherited_permissions_disabled
permission_permission_details = permission.permission_details
permission_role = permission.role
permission_email_address = permission.email_address
permission_expiration_time = permission.expiration_time
permission_display_name = permission.display_name
```

---


### Channel

Stops watching resources through this channel. For more information, see [Notifications for resource changes](https://developers.google.com/workspace/drive/api/guides/push).

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | String |  | The type of delivery mechanism used for this channel. Valid values are "web_hook" or "webhook". |
| `resource_id` | String |  | An opaque ID that identifies the resource being watched on this channel. Stable across different API versions. |
| `id` | String |  | A UUID or similar unique string that identifies this channel. |
| `params` | HashMap<String, String> |  | Additional parameters controlling delivery channel behavior. Optional. |
| `token` | String |  | An arbitrary string delivered to the target address with each notification delivered over this channel. Optional. |
| `kind` | String |  | Identifies this as a notification channel used to watch for changes to a resource, which is `api#channel`. |
| `resource_uri` | String |  | A version-specific identifier for the watched resource. |
| `address` | String |  | The address where notifications are delivered for this channel. |
| `expiration` | String |  | Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional. |
| `payload` | bool |  | A Boolean value to indicate whether payload is wanted. Optional. |



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


### Drive

Creates a shared drive. For more information, see [Manage shared drives](https://developers.google.com/workspace/drive/api/guides/manage-shareddrives).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The name of this shared drive. |
| `org_unit_id` | String |  | Output only. The organizational unit of this shared drive. This field is only populated on `drives.list` responses when the `useDomainAdminAccess` parameter is set to `true`. |
| `created_time` | String |  | The time at which the shared drive was created (RFC 3339 date-time). |
| `restrictions` | String |  | A set of restrictions that apply to this shared drive or items inside this shared drive. Note that restrictions can't be set when creating a shared drive. To add a restriction, first create a shared drive and then use `drives.update` to add restrictions. |
| `background_image_file` | String |  | An image file and cropping parameters from which a background image for this shared drive is set. This is a write only field; it can only be set on `drive.drives.update` requests that don't set `themeId`. When specified, all fields of the `backgroundImageFile` must be set. |
| `kind` | String |  | Output only. Identifies what kind of resource this is. Value: the fixed string `"drive#drive"`. |
| `background_image_link` | String |  | Output only. A short-lived link to this shared drive's background image. |
| `color_rgb` | String |  | The color of this shared drive as an RGB hex string. It can only be set on a `drive.drives.update` request that does not set `themeId`. |
| `theme_id` | String |  | The ID of the theme from which the background image and color will be set. The set of possible `driveThemes` can be retrieved from a `drive.about.get` response. When not specified on a `drive.drives.create` request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set `colorRgb` or `backgroundImageFile`. |
| `capabilities` | String |  | Output only. Capabilities the current user has on this shared drive. |
| `id` | String |  | Output only. The ID of this shared drive which is also the ID of the top level folder of this shared drive. |
| `hidden` | bool |  | Whether the shared drive is hidden from default view. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The name of this shared drive. |
| `org_unit_id` | String | Output only. The organizational unit of this shared drive. This field is only populated on `drives.list` responses when the `useDomainAdminAccess` parameter is set to `true`. |
| `created_time` | String | The time at which the shared drive was created (RFC 3339 date-time). |
| `restrictions` | String | A set of restrictions that apply to this shared drive or items inside this shared drive. Note that restrictions can't be set when creating a shared drive. To add a restriction, first create a shared drive and then use `drives.update` to add restrictions. |
| `background_image_file` | String | An image file and cropping parameters from which a background image for this shared drive is set. This is a write only field; it can only be set on `drive.drives.update` requests that don't set `themeId`. When specified, all fields of the `backgroundImageFile` must be set. |
| `kind` | String | Output only. Identifies what kind of resource this is. Value: the fixed string `"drive#drive"`. |
| `background_image_link` | String | Output only. A short-lived link to this shared drive's background image. |
| `color_rgb` | String | The color of this shared drive as an RGB hex string. It can only be set on a `drive.drives.update` request that does not set `themeId`. |
| `theme_id` | String | The ID of the theme from which the background image and color will be set. The set of possible `driveThemes` can be retrieved from a `drive.about.get` response. When not specified on a `drive.drives.create` request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set `colorRgb` or `backgroundImageFile`. |
| `capabilities` | String | Output only. Capabilities the current user has on this shared drive. |
| `id` | String | Output only. The ID of this shared drive which is also the ID of the top level folder of this shared drive. |
| `hidden` | bool | Whether the shared drive is hidden from default view. |


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
drive_name = drive.name
drive_org_unit_id = drive.org_unit_id
drive_created_time = drive.created_time
drive_restrictions = drive.restrictions
drive_background_image_file = drive.background_image_file
drive_kind = drive.kind
drive_background_image_link = drive.background_image_link
drive_color_rgb = drive.color_rgb
drive_theme_id = drive.theme_id
drive_capabilities = drive.capabilities
drive_id = drive.id
drive_hidden = drive.hidden
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
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |


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
operation_error = operation.error
operation_done = operation.done
operation_metadata = operation.metadata
operation_name = operation.name
operation_response = operation.response
```

---


### Comment

Creates a comment on a file. For more information, see [Manage comments and replies](https://developers.google.com/workspace/drive/api/guides/manage-comments). Required: The `fields` parameter must be set. To return the exact fields you need, see [Return specific fields](https://developers.google.com/workspace/drive/api/guides/fields-parameter).

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String |  | Output only. The ID of the comment. |
| `kind` | String |  | Output only. Identifies what kind of resource this is. Value: the fixed string `"drive#comment"`. |
| `html_content` | String |  | Output only. The content of the comment with HTML formatting. |
| `assignee_email_address` | String |  | Output only. The email of the user who is assigned to this comment, if none is assigned this will be unset. |
| `anchor` | String |  | A region of the document represented as a JSON string. For details on defining anchor properties, refer to [Manage comments and replies](https://developers.google.com/workspace/drive/api/v3/manage-comments). |
| `mentioned_email_addresses` | Vec<String> |  | Output only. The emails of the users who were mentioned in this comment, if none were mentioned this will be an empty list. |
| `modified_time` | String |  | The last time the comment or any of its replies was modified (RFC 3339 date-time). |
| `quoted_file_content` | String |  | The file content to which the comment refers, typically within the anchor region. For a text file, for example, this would be the text at the location of the comment. |
| `created_time` | String |  | The time at which the comment was created (RFC 3339 date-time). |
| `replies` | Vec<String> |  | Output only. The full list of replies to the comment in chronological order. |
| `resolved` | bool |  | Output only. Whether the comment has been resolved by one of its replies. |
| `author` | String |  | Output only. The author of the comment. The author's email address and permission ID will not be populated. |
| `content` | String |  | The plain text content of the comment. This field is used for setting the content, while `htmlContent` should be displayed. |
| `deleted` | bool |  | Output only. Whether the comment has been deleted. A deleted comment has no content. |
| `file_id` | String | ✅ | The ID of the file. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | Output only. The ID of the comment. |
| `kind` | String | Output only. Identifies what kind of resource this is. Value: the fixed string `"drive#comment"`. |
| `html_content` | String | Output only. The content of the comment with HTML formatting. |
| `assignee_email_address` | String | Output only. The email of the user who is assigned to this comment, if none is assigned this will be unset. |
| `anchor` | String | A region of the document represented as a JSON string. For details on defining anchor properties, refer to [Manage comments and replies](https://developers.google.com/workspace/drive/api/v3/manage-comments). |
| `mentioned_email_addresses` | Vec<String> | Output only. The emails of the users who were mentioned in this comment, if none were mentioned this will be an empty list. |
| `modified_time` | String | The last time the comment or any of its replies was modified (RFC 3339 date-time). |
| `quoted_file_content` | String | The file content to which the comment refers, typically within the anchor region. For a text file, for example, this would be the text at the location of the comment. |
| `created_time` | String | The time at which the comment was created (RFC 3339 date-time). |
| `replies` | Vec<String> | Output only. The full list of replies to the comment in chronological order. |
| `resolved` | bool | Output only. Whether the comment has been resolved by one of its replies. |
| `author` | String | Output only. The author of the comment. The author's email address and permission ID will not be populated. |
| `content` | String | The plain text content of the comment. This field is used for setting the content, while `htmlContent` should be displayed. |
| `deleted` | bool | Output only. Whether the comment has been deleted. A deleted comment has no content. |


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
comment_id = comment.id
comment_kind = comment.kind
comment_html_content = comment.html_content
comment_assignee_email_address = comment.assignee_email_address
comment_anchor = comment.anchor
comment_mentioned_email_addresses = comment.mentioned_email_addresses
comment_modified_time = comment.modified_time
comment_quoted_file_content = comment.quoted_file_content
comment_created_time = comment.created_time
comment_replies = comment.replies
comment_resolved = comment.resolved
comment_author = comment.author
comment_content = comment.content
comment_deleted = comment.deleted
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
| `drive_themes` | Vec<String> | A list of themes that are supported for shared drives. |
| `app_installed` | bool | Whether the user has installed the requesting app. |
| `folder_color_palette` | Vec<String> | The currently supported folder colors as RGB hex strings. |
| `import_formats` | HashMap<String, Vec<String>> | A map of source MIME type to possible targets for all supported imports. |
| `max_upload_size` | String | The maximum upload size in bytes. |
| `user` | String | The authenticated user. |
| `can_create_drives` | bool | Whether the user can create shared drives. |
| `max_import_sizes` | HashMap<String, String> | A map of maximum import sizes by MIME type, in bytes. |
| `export_formats` | HashMap<String, Vec<String>> | A map of source MIME type to possible targets for all supported exports. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string `"drive#about"`. |
| `team_drive_themes` | Vec<String> | Deprecated: Use `driveThemes` instead. |
| `storage_quota` | String | The user's storage quota limits and usage. For users that are part of an organization with pooled storage, information about the limit and usage across all services is for the organization, rather than the individual user. All fields are measured in bytes. |
| `can_create_team_drives` | bool | Deprecated: Use `canCreateDrives` instead. |


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
about_drive_themes = about.drive_themes
about_app_installed = about.app_installed
about_folder_color_palette = about.folder_color_palette
about_import_formats = about.import_formats
about_max_upload_size = about.max_upload_size
about_user = about.user
about_can_create_drives = about.can_create_drives
about_max_import_sizes = about.max_import_sizes
about_export_formats = about.export_formats
about_kind = about.kind
about_team_drive_themes = about.team_drive_themes
about_storage_quota = about.storage_quota
about_can_create_team_drives = about.can_create_team_drives
```

---


### Teamdrive

Deprecated: Use `drives.create` instead.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `capabilities` | String |  | Capabilities the current user has on this Team Drive. |
| `org_unit_id` | String |  | The organizational unit of this shared drive. This field is only populated on `drives.list` responses when the `useDomainAdminAccess` parameter is set to `true`. |
| `restrictions` | String |  | A set of restrictions that apply to this Team Drive or items inside this Team Drive. |
| `theme_id` | String |  | The ID of the theme from which the background image and color will be set. The set of possible `teamDriveThemes` can be retrieved from a `drive.about.get` response. When not specified on a `drive.teamdrives.create` request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set `colorRgb` or `backgroundImageFile`. |
| `color_rgb` | String |  | The color of this Team Drive as an RGB hex string. It can only be set on a `drive.teamdrives.update` request that does not set `themeId`. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string `"drive#teamDrive"`. |
| `created_time` | String |  | The time at which the Team Drive was created (RFC 3339 date-time). |
| `background_image_file` | String |  | An image file and cropping parameters from which a background image for this Team Drive is set. This is a write only field; it can only be set on `drive.teamdrives.update` requests that don't set `themeId`. When specified, all fields of the `backgroundImageFile` must be set. |
| `name` | String |  | The name of this Team Drive. |
| `id` | String |  | The ID of this Team Drive which is also the ID of the top level folder of this Team Drive. |
| `background_image_link` | String |  | A short-lived link to this Team Drive's background image. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `capabilities` | String | Capabilities the current user has on this Team Drive. |
| `org_unit_id` | String | The organizational unit of this shared drive. This field is only populated on `drives.list` responses when the `useDomainAdminAccess` parameter is set to `true`. |
| `restrictions` | String | A set of restrictions that apply to this Team Drive or items inside this Team Drive. |
| `theme_id` | String | The ID of the theme from which the background image and color will be set. The set of possible `teamDriveThemes` can be retrieved from a `drive.about.get` response. When not specified on a `drive.teamdrives.create` request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set `colorRgb` or `backgroundImageFile`. |
| `color_rgb` | String | The color of this Team Drive as an RGB hex string. It can only be set on a `drive.teamdrives.update` request that does not set `themeId`. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string `"drive#teamDrive"`. |
| `created_time` | String | The time at which the Team Drive was created (RFC 3339 date-time). |
| `background_image_file` | String | An image file and cropping parameters from which a background image for this Team Drive is set. This is a write only field; it can only be set on `drive.teamdrives.update` requests that don't set `themeId`. When specified, all fields of the `backgroundImageFile` must be set. |
| `name` | String | The name of this Team Drive. |
| `id` | String | The ID of this Team Drive which is also the ID of the top level folder of this Team Drive. |
| `background_image_link` | String | A short-lived link to this Team Drive's background image. |


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
teamdrive_capabilities = teamdrive.capabilities
teamdrive_org_unit_id = teamdrive.org_unit_id
teamdrive_restrictions = teamdrive.restrictions
teamdrive_theme_id = teamdrive.theme_id
teamdrive_color_rgb = teamdrive.color_rgb
teamdrive_kind = teamdrive.kind
teamdrive_created_time = teamdrive.created_time
teamdrive_background_image_file = teamdrive.background_image_file
teamdrive_name = teamdrive.name
teamdrive_id = teamdrive.id
teamdrive_background_image_link = teamdrive.background_image_link
```

---


### File

 Creates a file. For more information, see [Create and manage files](/workspace/drive/api/guides/create-file). This method supports an */upload* URI and accepts uploaded media with the following characteristics: - *Maximum file size:* 5,120 GB - *Accepted Media MIME types:* `*/*` (Specify a valid MIME type, rather than the literal `*/*` value. The literal `*/*` is only used to indicate that any valid MIME type can be uploaded. For more information, see [Google Workspace and Google Drive supported MIME types](/workspace/drive/api/guides/mime-types).) For more information on uploading files, see [Upload file data](/workspace/drive/api/guides/manage-uploads). Apps creating shortcuts with the `create` method must specify the MIME type `application/vnd.google-apps.shortcut`. Apps should specify a file extension in the `name` property when inserting files with the API. For example, an operation to insert a JPEG file should specify something like `"name": "cat.jpg"` in the metadata. Subsequent `GET` requests include the read-only `fileExtension` property populated with the extension originally specified in the `name` property. When a Google Drive user requests to download a file, or when the file is downloaded through the sync client, Drive builds a full filename (with extension) based on the name. In cases where the extension is missing, Drive attempts to determine the extension based on the file's MIME type.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `trashed` | bool |  | Whether the file has been trashed, either explicitly or from a trashed parent folder. Only the owner may trash a file, and other users cannot see files in the owner's trash. |
| `modified_by_me` | bool |  | Output only. Whether the file has been modified by this user. |
| `head_revision_id` | String |  | Output only. The ID of the file's head revision. This is currently only available for files with binary content in Google Drive. |
| `inherited_permissions_disabled` | bool |  | Whether this file has inherited permissions disabled. Inherited permissions are enabled by default. |
| `icon_link` | String |  | Output only. A static, unauthenticated link to the file's icon. |
| `mime_type` | String |  | The MIME type of the file. Google Drive attempts to automatically detect an appropriate value from uploaded content, if no value is provided. The value cannot be changed unless a new revision is uploaded. If a file is created with a Google Doc MIME type, the uploaded content is imported, if possible. The supported import formats are published in the [`about`](/workspace/drive/api/reference/rest/v3/about) resource. |
| `modified_by_me_time` | String |  | The last time the file was modified by the user (RFC 3339 date-time). |
| `shared` | bool |  | Output only. Whether the file has been shared. Not populated for items in shared drives. |
| `label_info` | String |  | Output only. An overview of the labels on the file. |
| `resource_key` | String |  | Output only. A key needed to access the item via a shared link. |
| `explicitly_trashed` | bool |  | Output only. Whether the file has been explicitly trashed, as opposed to recursively trashed from a parent folder. |
| `permissions` | Vec<String> |  | Output only. The full list of permissions for the file. This is only available if the requesting user can share the file. Not populated for items in shared drives. |
| `sha256_checksum` | String |  | Output only. The SHA256 checksum associated with this file, if available. This field is only populated for files with content stored in Google Drive; it's not populated for Docs Editors or shortcut files. |
| `link_share_metadata` | String |  | Contains details about the link URLs that clients are using to refer to this item. |
| `properties` | HashMap<String, String> |  | A collection of arbitrary key-value pairs which are visible to all apps.
Entries with null values are cleared in update and copy requests. |
| `trashing_user` | String |  | Output only. If the file has been explicitly trashed, the user who trashed it. Only populated for items in shared drives. |
| `parents` | Vec<String> |  | The ID of the parent folder containing the file. A file can only have one parent folder; specifying multiple parents isn't supported. If not specified as part of a create request, the file is placed directly in the user's My Drive folder. If not specified as part of a copy request, the file inherits any discoverable parent of the source file. Update requests must use the `addParents` and `removeParents` parameters to modify the parents list. |
| `owners` | Vec<String> |  | Output only. The owner of this file. Only certain legacy files may have more than one owner. This field isn't populated for items in shared drives. |
| `writers_can_share` | bool |  | Whether users with only `writer` permission can modify the file's permissions. Not populated for items in shared drives. |
| `content_restrictions` | Vec<String> |  | Restrictions for accessing the content of the file. Only populated if such a restriction exists. |
| `thumbnail_version` | String |  | Output only. The thumbnail version for use in thumbnail cache invalidation. |
| `sha1_checksum` | String |  | Output only. The SHA1 checksum associated with this file, if available. This field is only populated for files with content stored in Google Drive; it's not populated for Docs Editors or shortcut files. |
| `download_restrictions` | String |  | Download restrictions applied on the file. |
| `id` | String |  | The ID of the file. |
| `sharing_user` | String |  | Output only. The user who shared the file with the requesting user, if applicable. |
| `viewers_can_copy_content` | bool |  | Deprecated: Use `copyRequiresWriterPermission` instead. |
| `spaces` | Vec<String> |  | Output only. The list of spaces which contain the file. The currently supported values are `drive`, `appDataFolder`, and `photos`. |
| `export_links` | HashMap<String, String> |  | Output only. Links for exporting Docs Editors files to specific formats. |
| `original_filename` | String |  | The original filename of the uploaded content if available, or else the original value of the `name` field. This is only available for files with binary content in Google Drive. |
| `md5_checksum` | String |  | Output only. The MD5 checksum for the content of the file. This is only applicable to files with binary content in Google Drive. |
| `quota_bytes_used` | String |  | Output only. The number of storage quota bytes used by the file. This includes the head revision as well as previous revisions with `keepForever` enabled. |
| `is_app_authorized` | bool |  | Output only. Whether the file was created or opened by the requesting app. |
| `team_drive_id` | String |  | Deprecated: Output only. Use `driveId` instead. |
| `folder_color_rgb` | String |  | The color for a folder or a shortcut to a folder as an RGB hex string. The supported colors are published in the `folderColorPalette` field of the [`about`](/workspace/drive/api/reference/rest/v3/about) resource. If an unsupported color is specified, the closest color in the palette is used instead. |
| `trashed_time` | String |  | The time that the item was trashed (RFC 3339 date-time). Only populated for items in shared drives. |
| `name` | String |  | The name of the file. This isn't necessarily unique within a folder. Note that for immutable items such as the top-level folders of shared drives, the My Drive root folder, and the Application Data folder, the name is constant. |
| `file_extension` | String |  | Output only. The final component of `fullFileExtension`. This is only available for files with binary content in Google Drive. |
| `full_file_extension` | String |  | Output only. The full file extension extracted from the `name` field. May contain multiple concatenated extensions, such as "tar.gz". This is only available for files with binary content in Google Drive. This is automatically updated when the `name` field changes, however it's not cleared if the new name doesn't contain a valid extension. |
| `version` | String |  | Output only. A monotonically increasing version number for the file. This reflects every change made to the file on the server, even those not visible to the user. |
| `viewed_by_me` | bool |  | Output only. Whether the file has been viewed by this user. |
| `description` | String |  | A short description of the file. |
| `has_augmented_permissions` | bool |  | Output only. Whether there are permissions directly on this file. This field is only populated for items in shared drives. |
| `web_view_link` | String |  | Output only. A link for opening the file in a relevant Google editor or viewer in a browser. |
| `created_time` | String |  | The time at which the file was created (RFC 3339 date-time). |
| `starred` | bool |  | Whether the user has starred the file. |
| `web_content_link` | String |  | Output only. A link for downloading the content of the file in a browser. This is only available for files with binary content in Google Drive. |
| `app_properties` | HashMap<String, String> |  | A collection of arbitrary key-value pairs which are private to the requesting app.
Entries with null values are cleared in update and copy requests. These properties can only be retrieved using an authenticated request. An authenticated request uses an access token obtained with a OAuth 2 client ID. You cannot use an API key to retrieve private properties. |
| `kind` | String |  | Output only. Identifies what kind of resource this is. Value: the fixed string `"drive#file"`. |
| `owned_by_me` | bool |  | Output only. Whether the user owns the file. Not populated for items in shared drives. |
| `shortcut_details` | String |  | Shortcut file details. Only populated for shortcut files, which have the mimeType field set to `application/vnd.google-apps.shortcut`. Can only be set on `files.create` requests. |
| `image_media_metadata` | String |  | Output only. Additional metadata about image media, if available. |
| `viewed_by_me_time` | String |  | The last time the file was viewed by the user (RFC 3339 date-time). |
| `shared_with_me_time` | String |  | The time at which the file was shared with the user, if applicable (RFC 3339 date-time). |
| `video_media_metadata` | String |  | Output only. Additional metadata about video media. This may not be available immediately upon upload. |
| `capabilities` | String |  | Output only. Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take. For more information, see [Understand file capabilities](https://developers.google.com/workspace/drive/api/guides/manage-sharing#capabilities). |
| `content_hints` | String |  | Additional information about the content of the file. These fields are never populated in responses. |
| `modified_time` | String |  | he last time the file was modified by anyone (RFC 3339 date-time). Note that setting modifiedTime will also update modifiedByMeTime for the user. |
| `drive_id` | String |  | Output only. ID of the shared drive the file resides in. Only populated for items in shared drives. |
| `thumbnail_link` | String |  | Output only. A short-lived link to the file's thumbnail, if available. Typically lasts on the order of hours. Not intended for direct usage on web applications due to [Cross-Origin Resource Sharing (CORS)](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) policies. Consider using a proxy server. Only populated when the requesting app can access the file's content. If the file isn't shared publicly, the URL returned in `files.thumbnailLink` must be fetched using a credentialed request. |
| `size` | String |  | Output only. Size in bytes of blobs and Google Workspace editor files. Won't be populated for files that have no size, like shortcuts and folders. |
| `last_modifying_user` | String |  | Output only. The last user to modify the file. This field is only populated when the last modification was performed by a signed-in user. |
| `copy_requires_writer_permission` | bool |  | Whether the options to copy, print, or download this file should be disabled for readers and commenters. |
| `has_thumbnail` | bool |  | Output only. Whether this file has a thumbnail. This doesn't indicate whether the requesting app has access to the thumbnail. To check access, look for the presence of the thumbnailLink field. |
| `permission_ids` | Vec<String> |  | Output only. List of permission IDs for users with access to this file. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `trashed` | bool | Whether the file has been trashed, either explicitly or from a trashed parent folder. Only the owner may trash a file, and other users cannot see files in the owner's trash. |
| `modified_by_me` | bool | Output only. Whether the file has been modified by this user. |
| `head_revision_id` | String | Output only. The ID of the file's head revision. This is currently only available for files with binary content in Google Drive. |
| `inherited_permissions_disabled` | bool | Whether this file has inherited permissions disabled. Inherited permissions are enabled by default. |
| `icon_link` | String | Output only. A static, unauthenticated link to the file's icon. |
| `mime_type` | String | The MIME type of the file. Google Drive attempts to automatically detect an appropriate value from uploaded content, if no value is provided. The value cannot be changed unless a new revision is uploaded. If a file is created with a Google Doc MIME type, the uploaded content is imported, if possible. The supported import formats are published in the [`about`](/workspace/drive/api/reference/rest/v3/about) resource. |
| `modified_by_me_time` | String | The last time the file was modified by the user (RFC 3339 date-time). |
| `shared` | bool | Output only. Whether the file has been shared. Not populated for items in shared drives. |
| `label_info` | String | Output only. An overview of the labels on the file. |
| `resource_key` | String | Output only. A key needed to access the item via a shared link. |
| `explicitly_trashed` | bool | Output only. Whether the file has been explicitly trashed, as opposed to recursively trashed from a parent folder. |
| `permissions` | Vec<String> | Output only. The full list of permissions for the file. This is only available if the requesting user can share the file. Not populated for items in shared drives. |
| `sha256_checksum` | String | Output only. The SHA256 checksum associated with this file, if available. This field is only populated for files with content stored in Google Drive; it's not populated for Docs Editors or shortcut files. |
| `link_share_metadata` | String | Contains details about the link URLs that clients are using to refer to this item. |
| `properties` | HashMap<String, String> | A collection of arbitrary key-value pairs which are visible to all apps.
Entries with null values are cleared in update and copy requests. |
| `trashing_user` | String | Output only. If the file has been explicitly trashed, the user who trashed it. Only populated for items in shared drives. |
| `parents` | Vec<String> | The ID of the parent folder containing the file. A file can only have one parent folder; specifying multiple parents isn't supported. If not specified as part of a create request, the file is placed directly in the user's My Drive folder. If not specified as part of a copy request, the file inherits any discoverable parent of the source file. Update requests must use the `addParents` and `removeParents` parameters to modify the parents list. |
| `owners` | Vec<String> | Output only. The owner of this file. Only certain legacy files may have more than one owner. This field isn't populated for items in shared drives. |
| `writers_can_share` | bool | Whether users with only `writer` permission can modify the file's permissions. Not populated for items in shared drives. |
| `content_restrictions` | Vec<String> | Restrictions for accessing the content of the file. Only populated if such a restriction exists. |
| `thumbnail_version` | String | Output only. The thumbnail version for use in thumbnail cache invalidation. |
| `sha1_checksum` | String | Output only. The SHA1 checksum associated with this file, if available. This field is only populated for files with content stored in Google Drive; it's not populated for Docs Editors or shortcut files. |
| `download_restrictions` | String | Download restrictions applied on the file. |
| `id` | String | The ID of the file. |
| `sharing_user` | String | Output only. The user who shared the file with the requesting user, if applicable. |
| `viewers_can_copy_content` | bool | Deprecated: Use `copyRequiresWriterPermission` instead. |
| `spaces` | Vec<String> | Output only. The list of spaces which contain the file. The currently supported values are `drive`, `appDataFolder`, and `photos`. |
| `export_links` | HashMap<String, String> | Output only. Links for exporting Docs Editors files to specific formats. |
| `original_filename` | String | The original filename of the uploaded content if available, or else the original value of the `name` field. This is only available for files with binary content in Google Drive. |
| `md5_checksum` | String | Output only. The MD5 checksum for the content of the file. This is only applicable to files with binary content in Google Drive. |
| `quota_bytes_used` | String | Output only. The number of storage quota bytes used by the file. This includes the head revision as well as previous revisions with `keepForever` enabled. |
| `is_app_authorized` | bool | Output only. Whether the file was created or opened by the requesting app. |
| `team_drive_id` | String | Deprecated: Output only. Use `driveId` instead. |
| `folder_color_rgb` | String | The color for a folder or a shortcut to a folder as an RGB hex string. The supported colors are published in the `folderColorPalette` field of the [`about`](/workspace/drive/api/reference/rest/v3/about) resource. If an unsupported color is specified, the closest color in the palette is used instead. |
| `trashed_time` | String | The time that the item was trashed (RFC 3339 date-time). Only populated for items in shared drives. |
| `name` | String | The name of the file. This isn't necessarily unique within a folder. Note that for immutable items such as the top-level folders of shared drives, the My Drive root folder, and the Application Data folder, the name is constant. |
| `file_extension` | String | Output only. The final component of `fullFileExtension`. This is only available for files with binary content in Google Drive. |
| `full_file_extension` | String | Output only. The full file extension extracted from the `name` field. May contain multiple concatenated extensions, such as "tar.gz". This is only available for files with binary content in Google Drive. This is automatically updated when the `name` field changes, however it's not cleared if the new name doesn't contain a valid extension. |
| `version` | String | Output only. A monotonically increasing version number for the file. This reflects every change made to the file on the server, even those not visible to the user. |
| `viewed_by_me` | bool | Output only. Whether the file has been viewed by this user. |
| `description` | String | A short description of the file. |
| `has_augmented_permissions` | bool | Output only. Whether there are permissions directly on this file. This field is only populated for items in shared drives. |
| `web_view_link` | String | Output only. A link for opening the file in a relevant Google editor or viewer in a browser. |
| `created_time` | String | The time at which the file was created (RFC 3339 date-time). |
| `starred` | bool | Whether the user has starred the file. |
| `web_content_link` | String | Output only. A link for downloading the content of the file in a browser. This is only available for files with binary content in Google Drive. |
| `app_properties` | HashMap<String, String> | A collection of arbitrary key-value pairs which are private to the requesting app.
Entries with null values are cleared in update and copy requests. These properties can only be retrieved using an authenticated request. An authenticated request uses an access token obtained with a OAuth 2 client ID. You cannot use an API key to retrieve private properties. |
| `kind` | String | Output only. Identifies what kind of resource this is. Value: the fixed string `"drive#file"`. |
| `owned_by_me` | bool | Output only. Whether the user owns the file. Not populated for items in shared drives. |
| `shortcut_details` | String | Shortcut file details. Only populated for shortcut files, which have the mimeType field set to `application/vnd.google-apps.shortcut`. Can only be set on `files.create` requests. |
| `image_media_metadata` | String | Output only. Additional metadata about image media, if available. |
| `viewed_by_me_time` | String | The last time the file was viewed by the user (RFC 3339 date-time). |
| `shared_with_me_time` | String | The time at which the file was shared with the user, if applicable (RFC 3339 date-time). |
| `video_media_metadata` | String | Output only. Additional metadata about video media. This may not be available immediately upon upload. |
| `capabilities` | String | Output only. Capabilities the current user has on this file. Each capability corresponds to a fine-grained action that a user may take. For more information, see [Understand file capabilities](https://developers.google.com/workspace/drive/api/guides/manage-sharing#capabilities). |
| `content_hints` | String | Additional information about the content of the file. These fields are never populated in responses. |
| `modified_time` | String | he last time the file was modified by anyone (RFC 3339 date-time). Note that setting modifiedTime will also update modifiedByMeTime for the user. |
| `drive_id` | String | Output only. ID of the shared drive the file resides in. Only populated for items in shared drives. |
| `thumbnail_link` | String | Output only. A short-lived link to the file's thumbnail, if available. Typically lasts on the order of hours. Not intended for direct usage on web applications due to [Cross-Origin Resource Sharing (CORS)](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) policies. Consider using a proxy server. Only populated when the requesting app can access the file's content. If the file isn't shared publicly, the URL returned in `files.thumbnailLink` must be fetched using a credentialed request. |
| `size` | String | Output only. Size in bytes of blobs and Google Workspace editor files. Won't be populated for files that have no size, like shortcuts and folders. |
| `last_modifying_user` | String | Output only. The last user to modify the file. This field is only populated when the last modification was performed by a signed-in user. |
| `copy_requires_writer_permission` | bool | Whether the options to copy, print, or download this file should be disabled for readers and commenters. |
| `has_thumbnail` | bool | Output only. Whether this file has a thumbnail. This doesn't indicate whether the requesting app has access to the thumbnail. To check access, look for the presence of the thumbnailLink field. |
| `permission_ids` | Vec<String> | Output only. List of permission IDs for users with access to this file. |


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
file_trashed = file.trashed
file_modified_by_me = file.modified_by_me
file_head_revision_id = file.head_revision_id
file_inherited_permissions_disabled = file.inherited_permissions_disabled
file_icon_link = file.icon_link
file_mime_type = file.mime_type
file_modified_by_me_time = file.modified_by_me_time
file_shared = file.shared
file_label_info = file.label_info
file_resource_key = file.resource_key
file_explicitly_trashed = file.explicitly_trashed
file_permissions = file.permissions
file_sha256_checksum = file.sha256_checksum
file_link_share_metadata = file.link_share_metadata
file_properties = file.properties
file_trashing_user = file.trashing_user
file_parents = file.parents
file_owners = file.owners
file_writers_can_share = file.writers_can_share
file_content_restrictions = file.content_restrictions
file_thumbnail_version = file.thumbnail_version
file_sha1_checksum = file.sha1_checksum
file_download_restrictions = file.download_restrictions
file_id = file.id
file_sharing_user = file.sharing_user
file_viewers_can_copy_content = file.viewers_can_copy_content
file_spaces = file.spaces
file_export_links = file.export_links
file_original_filename = file.original_filename
file_md5_checksum = file.md5_checksum
file_quota_bytes_used = file.quota_bytes_used
file_is_app_authorized = file.is_app_authorized
file_team_drive_id = file.team_drive_id
file_folder_color_rgb = file.folder_color_rgb
file_trashed_time = file.trashed_time
file_name = file.name
file_file_extension = file.file_extension
file_full_file_extension = file.full_file_extension
file_version = file.version
file_viewed_by_me = file.viewed_by_me
file_description = file.description
file_has_augmented_permissions = file.has_augmented_permissions
file_web_view_link = file.web_view_link
file_created_time = file.created_time
file_starred = file.starred
file_web_content_link = file.web_content_link
file_app_properties = file.app_properties
file_kind = file.kind
file_owned_by_me = file.owned_by_me
file_shortcut_details = file.shortcut_details
file_image_media_metadata = file.image_media_metadata
file_viewed_by_me_time = file.viewed_by_me_time
file_shared_with_me_time = file.shared_with_me_time
file_video_media_metadata = file.video_media_metadata
file_capabilities = file.capabilities
file_content_hints = file.content_hints
file_modified_time = file.modified_time
file_drive_id = file.drive_id
file_thumbnail_link = file.thumbnail_link
file_size = file.size
file_last_modifying_user = file.last_modifying_user
file_copy_requires_writer_permission = file.copy_requires_writer_permission
file_has_thumbnail = file.has_thumbnail
file_permission_ids = file.permission_ids
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple teamdrive resources
teamdrive_0 = provider.drive_api.Teamdrive {
}
teamdrive_1 = provider.drive_api.Teamdrive {
}
teamdrive_2 = provider.drive_api.Teamdrive {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    teamdrive = provider.drive_api.Teamdrive {
    }
```

---

## Related Documentation

- [GCP Drive_api Documentation](https://cloud.google.com/drive_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
