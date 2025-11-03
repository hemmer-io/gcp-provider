# Books_api Service



**Resources**: 21

---

## Overview

The books_api service provides access to 21 resource types:

- [Promooffer](#promooffer) [CR]
- [Bookshelve](#bookshelve) [CR]
- [Mybook](#mybook) [R]
- [Annotation_data](#annotation_data) [R]
- [Associated](#associated) [R]
- [Useruploaded](#useruploaded) [R]
- [Recommended](#recommended) [CR]
- [Volume](#volume) [R]
- [Onboarding](#onboarding) [R]
- [Cloudloading](#cloudloading) [C]
- [Serie](#serie) [R]
- [Membership](#membership) [R]
- [Dictionary](#dictionary) [R]
- [Volume_annotation](#volume_annotation) [R]
- [Readingposition](#readingposition) [CR]
- [Myconfig](#myconfig) [CR]
- [Personalizedstream](#personalizedstream) [R]
- [Notification](#notification) [R]
- [Familysharing](#familysharing) [CR]
- [Layer](#layer) [R]
- [Annotation](#annotation) [CRUD]

---

## Resources


### Promooffer

Accepts the promo offer.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | A list of offers. |
| `kind` | String | Resource type. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create promooffer
promooffer = provider.books_api.Promooffer {
}

# Access promooffer outputs
promooffer_id = promooffer.id
promooffer_items = promooffer.items
promooffer_kind = promooffer.kind
```

---


### Bookshelve

Removes a volume from a bookshelf.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `shelf` | String | ✅ | ID of bookshelf from which to remove a volume. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `volume_count` | i64 | Number of volumes in this bookshelf. |
| `volumes_last_updated` | String | Last time a volume was added or removed from this bookshelf (formatted UTC timestamp with millisecond resolution). |
| `title` | String | Title of this bookshelf. |
| `created` | String | Created time for this bookshelf (formatted UTC timestamp with millisecond resolution). |
| `access` | String | Whether this bookshelf is PUBLIC or PRIVATE. |
| `id` | i64 | Id of this bookshelf, only unique by user. |
| `description` | String | Description of this bookshelf. |
| `kind` | String | Resource type for bookshelf metadata. |
| `updated` | String | Last modified time of this bookshelf (formatted UTC timestamp with millisecond resolution). |
| `self_link` | String | URL to this resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create bookshelve
bookshelve = provider.books_api.Bookshelve {
    shelf = "value"  # ID of bookshelf from which to remove a volume.
}

# Access bookshelve outputs
bookshelve_id = bookshelve.id
bookshelve_volume_count = bookshelve.volume_count
bookshelve_volumes_last_updated = bookshelve.volumes_last_updated
bookshelve_title = bookshelve.title
bookshelve_created = bookshelve.created
bookshelve_access = bookshelve.access
bookshelve_id = bookshelve.id
bookshelve_description = bookshelve.description
bookshelve_kind = bookshelve.kind
bookshelve_updated = bookshelve.updated
bookshelve_self_link = bookshelve.self_link
```

---


### Mybook

Return a list of books in My Library.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | A list of volumes. |
| `kind` | String | Resource type. |
| `total_items` | i64 | Total number of volumes found. This might be greater than the number of volumes returned in this response if results have been paginated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access mybook outputs
mybook_id = mybook.id
mybook_items = mybook.items
mybook_kind = mybook.kind
mybook_total_items = mybook.total_items
```

---


### Annotation_data

Gets the annotation data.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `updated` | String | Timestamp for the last time this data was updated. (RFC 3339 UTC date-time format). |
| `data` | String | JSON encoded data for this dictionary annotation data. Emitted with name 'data' in JSON output. Either this or geo_data will be populated. |
| `volume_id` | String | The volume id for this data. * |
| `id` | String | Unique id for this annotation data. |
| `encoded_data` | String | Base64 encoded data for this annotation data. |
| `annotation_type` | String | The type of annotation this data is for. |
| `layer_id` | String | The Layer id for this data. * |
| `self_link` | String | URL for this resource. * |
| `kind` | String | Resource Type |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access annotation_data outputs
annotation_data_id = annotation_data.id
annotation_data_updated = annotation_data.updated
annotation_data_data = annotation_data.data
annotation_data_volume_id = annotation_data.volume_id
annotation_data_id = annotation_data.id
annotation_data_encoded_data = annotation_data.encoded_data
annotation_data_annotation_type = annotation_data.annotation_type
annotation_data_layer_id = annotation_data.layer_id
annotation_data_self_link = annotation_data.self_link
annotation_data_kind = annotation_data.kind
```

---


### Associated

Return a list of associated books.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | A list of volumes. |
| `kind` | String | Resource type. |
| `total_items` | i64 | Total number of volumes found. This might be greater than the number of volumes returned in this response if results have been paginated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access associated outputs
associated_id = associated.id
associated_items = associated.items
associated_kind = associated.kind
associated_total_items = associated.total_items
```

---


### Useruploaded

Return a list of books uploaded by the current user.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | A list of volumes. |
| `kind` | String | Resource type. |
| `total_items` | i64 | Total number of volumes found. This might be greater than the number of volumes returned in this response if results have been paginated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access useruploaded outputs
useruploaded_id = useruploaded.id
useruploaded_items = useruploaded.items
useruploaded_kind = useruploaded.kind
useruploaded_total_items = useruploaded.total_items
```

---


### Recommended

Rate a recommended book for the current user.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | A list of volumes. |
| `kind` | String | Resource type. |
| `total_items` | i64 | Total number of volumes found. This might be greater than the number of volumes returned in this response if results have been paginated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create recommended
recommended = provider.books_api.Recommended {
}

# Access recommended outputs
recommended_id = recommended.id
recommended_items = recommended.items
recommended_kind = recommended.kind
recommended_total_items = recommended.total_items
```

---


### Volume

Gets volume information for a single volume.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `recommended_info` | String | Recommendation related information for this volume. |
| `user_info` | String | User specific information related to this volume. (e.g. page this user last read or whether they purchased this book) |
| `volume_info` | String | General volume information. |
| `etag` | String | Opaque identifier for a specific version of a volume resource. (In LITE projection) |
| `access_info` | String | Any information about a volume related to reading or obtaining that volume text. This information can depend on country (books may be public domain in one country but not in another, e.g.). |
| `kind` | String | Resource type for a volume. (In LITE projection.) |
| `layer_info` | String | What layers exist in this volume and high level information about them. |
| `id` | String | Unique identifier for a volume. (In LITE projection.) |
| `sale_info` | String | Any information about a volume related to the eBookstore and/or purchaseability. This information can depend on the country where the request originates from (i.e. books may not be for sale in certain countries). |
| `search_info` | String | Search result information related to this volume. |
| `self_link` | String | URL to this resource. (In LITE projection.) |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access volume outputs
volume_id = volume.id
volume_recommended_info = volume.recommended_info
volume_user_info = volume.user_info
volume_volume_info = volume.volume_info
volume_etag = volume.etag
volume_access_info = volume.access_info
volume_kind = volume.kind
volume_layer_info = volume.layer_info
volume_id = volume.id
volume_sale_info = volume.sale_info
volume_search_info = volume.search_info
volume_self_link = volume.self_link
```

---


### Onboarding

List available volumes under categories for onboarding experience.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Resource type. |
| `next_page_token` | String |  |
| `items` | Vec<String> | A list of volumes. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access onboarding outputs
onboarding_id = onboarding.id
onboarding_kind = onboarding.kind
onboarding_next_page_token = onboarding.next_page_token
onboarding_items = onboarding.items
```

---


### Cloudloading

Add a user-upload volume and triggers processing.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create cloudloading
cloudloading = provider.books_api.Cloudloading {
}

```

---


### Serie

Returns Series metadata for the given series ids.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Resource type. |
| `series` | Vec<String> |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access serie outputs
serie_id = serie.id
serie_kind = serie.kind
serie_series = serie.series
```

---


### Membership

Returns Series membership data given the series id.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String |  |
| `member` | Vec<String> |  |
| `kind` | String | Resorce type. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access membership outputs
membership_id = membership.id
membership_next_page_token = membership.next_page_token
membership_member = membership.member
membership_kind = membership.kind
```

---


### Dictionary

Returns a list of offline dictionary metadata available

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `items` | Vec<String> | A list of offline dictionary metadata. |
| `kind` | String | Resource type. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access dictionary outputs
dictionary_id = dictionary.id
dictionary_items = dictionary.items
dictionary_kind = dictionary.kind
```

---


### Volume_annotation

Gets the volume annotation.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | Unique id of this volume annotation. |
| `data` | String | Data for this annotation. |
| `annotation_data_id` | String | The annotation data id for this volume annotation. |
| `annotation_type` | String | The type of annotation this is. |
| `kind` | String | Resource Type |
| `annotation_data_link` | String | Link to get data for this annotation. |
| `page_ids` | Vec<String> | Pages the annotation spans. |
| `self_link` | String | URL to this resource. |
| `deleted` | bool | Indicates that this annotation is deleted. |
| `selected_text` | String | Excerpt from the volume. |
| `volume_id` | String | The Volume this annotation is for. |
| `layer_id` | String | The Layer this annotation is for. |
| `updated` | String | Timestamp for the last time this anntoation was updated. (RFC 3339 UTC date-time format). |
| `content_ranges` | String | The content ranges to identify the selected text. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access volume_annotation outputs
volume_annotation_id = volume_annotation.id
volume_annotation_id = volume_annotation.id
volume_annotation_data = volume_annotation.data
volume_annotation_annotation_data_id = volume_annotation.annotation_data_id
volume_annotation_annotation_type = volume_annotation.annotation_type
volume_annotation_kind = volume_annotation.kind
volume_annotation_annotation_data_link = volume_annotation.annotation_data_link
volume_annotation_page_ids = volume_annotation.page_ids
volume_annotation_self_link = volume_annotation.self_link
volume_annotation_deleted = volume_annotation.deleted
volume_annotation_selected_text = volume_annotation.selected_text
volume_annotation_volume_id = volume_annotation.volume_id
volume_annotation_layer_id = volume_annotation.layer_id
volume_annotation_updated = volume_annotation.updated
volume_annotation_content_ranges = volume_annotation.content_ranges
```

---


### Readingposition

Sets my reading position information for a volume.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `volume_id` | String | ✅ | ID of volume for which to update the reading position. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `volume_id` | String | Volume id associated with this reading position. |
| `epub_cfi_position` | String | Position in an EPUB as a CFI. |
| `pdf_position` | String | Position in a PDF file. |
| `gb_text_position` | String | Position in a volume for text-based content. |
| `kind` | String | Resource type for a reading position. |
| `updated` | String | Timestamp when this reading position was last updated (formatted UTC timestamp with millisecond resolution). |
| `gb_image_position` | String | Position in a volume for image-based content. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create readingposition
readingposition = provider.books_api.Readingposition {
    volume_id = "value"  # ID of volume for which to update the reading position.
}

# Access readingposition outputs
readingposition_id = readingposition.id
readingposition_volume_id = readingposition.volume_id
readingposition_epub_cfi_position = readingposition.epub_cfi_position
readingposition_pdf_position = readingposition.pdf_position
readingposition_gb_text_position = readingposition.gb_text_position
readingposition_kind = readingposition.kind
readingposition_updated = readingposition.updated
readingposition_gb_image_position = readingposition.gb_image_position
```

---


### Myconfig

Request downloaded content access for specified volumes on the My eBooks shelf.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Resource type. |
| `notes_export` | String | User settings in sub-objects, each for different purposes. |
| `notification` | String |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create myconfig
myconfig = provider.books_api.Myconfig {
}

# Access myconfig outputs
myconfig_id = myconfig.id
myconfig_kind = myconfig.kind
myconfig_notes_export = myconfig.notes_export
myconfig_notification = myconfig.notification
```

---


### Personalizedstream

Returns a stream of personalized book clusters

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `clusters` | Vec<String> |  |
| `total_clusters` | i64 |  |
| `kind` | String | Resorce type. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access personalizedstream outputs
personalizedstream_id = personalizedstream.id
personalizedstream_clusters = personalizedstream.clusters
personalizedstream_total_clusters = personalizedstream.total_clusters
personalizedstream_kind = personalizedstream.kind
```

---


### Notification

Returns notification details for a given notification id.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dont_show_notification` | bool |  |
| `title` | String |  |
| `body` | String |  |
| `time_to_expire_ms` | String |  |
| `show_notification_settings_action` | bool |  |
| `notification_group` | String |  |
| `notification_type` | String |  |
| `is_document_mature` | bool |  |
| `crm_experiment_ids` | Vec<String> | The list of crm experiment ids. |
| `target_url` | String |  |
| `reason` | String |  |
| `kind` | String | Resource type. |
| `pcampaign_id` | String |  |
| `doc_type` | String |  |
| `icon_url` | String |  |
| `doc_id` | String |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access notification outputs
notification_id = notification.id
notification_dont_show_notification = notification.dont_show_notification
notification_title = notification.title
notification_body = notification.body
notification_time_to_expire_ms = notification.time_to_expire_ms
notification_show_notification_settings_action = notification.show_notification_settings_action
notification_notification_group = notification.notification_group
notification_notification_type = notification.notification_type
notification_is_document_mature = notification.is_document_mature
notification_crm_experiment_ids = notification.crm_experiment_ids
notification_target_url = notification.target_url
notification_reason = notification.reason
notification_kind = notification.kind
notification_pcampaign_id = notification.pcampaign_id
notification_doc_type = notification.doc_type
notification_icon_url = notification.icon_url
notification_doc_id = notification.doc_id
```

---


### Familysharing

Initiates sharing of the content with the user's family. Empty response indicates success.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Resource type. |
| `membership` | String | Family membership info of the user that made the request. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create familysharing
familysharing = provider.books_api.Familysharing {
}

# Access familysharing outputs
familysharing_id = familysharing.id
familysharing_kind = familysharing.kind
familysharing_membership = familysharing.membership
```

---


### Layer

Gets the layer summary for a volume.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | Unique id of this layer summary. |
| `layer_id` | String | The layer id for this summary. |
| `volume_annotations_version` | String | The current version of this layer's volume annotations. Note that this version applies only to the data in the books.layers.volumeAnnotations.* responses. The actual annotation data is versioned separately. |
| `volume_id` | String | The volume id this resource is for. |
| `annotation_count` | i64 | The number of annotations for this layer. |
| `annotations_link` | String | The link to get the annotations for this layer. |
| `annotation_types` | Vec<String> | The list of annotation types contained for this layer. |
| `self_link` | String | URL to this resource. |
| `data_count` | i64 | The number of data items for this layer. |
| `annotations_data_link` | String | Link to get data for this annotation. |
| `kind` | String | Resource Type |
| `content_version` | String | The content version this resource is for. |
| `updated` | String | Timestamp for the last time an item in this layer was updated. (RFC 3339 UTC date-time format). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access layer outputs
layer_id = layer.id
layer_id = layer.id
layer_layer_id = layer.layer_id
layer_volume_annotations_version = layer.volume_annotations_version
layer_volume_id = layer.volume_id
layer_annotation_count = layer.annotation_count
layer_annotations_link = layer.annotations_link
layer_annotation_types = layer.annotation_types
layer_self_link = layer.self_link
layer_data_count = layer.data_count
layer_annotations_data_link = layer.annotations_data_link
layer_kind = layer.kind
layer_content_version = layer.content_version
layer_updated = layer.updated
```

---


### Annotation

Inserts a new annotation.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `after_selected_text` | String |  | Anchor text after excerpt. For requests, if the user bookmarked a screen that has no flowing text on it, then this field should be empty. |
| `client_version_ranges` | String |  | Selection ranges sent from the client. |
| `highlight_style` | String |  | The highlight style for this annotation. |
| `kind` | String |  | Resource type. |
| `layer_id` | String |  | The layer this annotation is for. |
| `layer_summary` | String |  |  |
| `updated` | String |  | Timestamp for the last time this annotation was modified. |
| `created` | String |  | Timestamp for the created time of this annotation. |
| `before_selected_text` | String |  | Anchor text before excerpt. For requests, if the user bookmarked a screen that has no flowing text on it, then this field should be empty. |
| `deleted` | bool |  | Indicates that this annotation is deleted. |
| `volume_id` | String |  | The volume that this annotation belongs to. |
| `current_version_ranges` | String |  | Selection ranges for the most recent content version. |
| `page_ids` | Vec<String> |  | Pages that this annotation spans. |
| `selected_text` | String |  | Excerpt from the volume. |
| `data` | String |  | User-created data for this annotation. |
| `self_link` | String |  | URL to this resource. |
| `id` | String |  | Id of this annotation, in the form of a GUID. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Token to pass in for pagination for the next page. This will not be present if this request does not have more results. |
| `kind` | String | Resource type. |
| `items` | Vec<String> | A list of annotations. |
| `total_items` | i64 | Total number of annotations found. This may be greater than the number of notes returned in this response if results have been paginated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create annotation
annotation = provider.books_api.Annotation {
}

# Access annotation outputs
annotation_id = annotation.id
annotation_next_page_token = annotation.next_page_token
annotation_kind = annotation.kind
annotation_items = annotation.items
annotation_total_items = annotation.total_items
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple promooffer resources
promooffer_0 = provider.books_api.Promooffer {
}
promooffer_1 = provider.books_api.Promooffer {
}
promooffer_2 = provider.books_api.Promooffer {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    promooffer = provider.books_api.Promooffer {
    }
```

---

## Related Documentation

- [GCP Books_api Documentation](https://cloud.google.com/books_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
