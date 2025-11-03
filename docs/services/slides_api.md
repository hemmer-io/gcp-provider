# Slides_api Service



**Resources**: 2

---

## Overview

The slides_api service provides access to 2 resource types:

- [Page](#page) [R]
- [Presentation](#presentation) [CR]

---

## Resources


### Page

Gets the latest version of the specified page in the presentation.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `layout_properties` | String | Layout specific properties. Only set if page_type = LAYOUT. |
| `page_type` | String | The type of the page. |
| `page_properties` | String | The properties of the page. |
| `revision_id` | String | Output only. The revision ID of the presentation. Can be used in update requests to assert the presentation revision hasn't changed since the last read operation. Only populated if the user has edit access to the presentation. The revision ID is not a sequential number but an opaque string. The format of the revision ID might change over time. A returned revision ID is only guaranteed to be valid for 24 hours after it has been returned and cannot be shared across users. If the revision ID is unchanged between calls, then the presentation has not changed. Conversely, a changed ID (for the same presentation and user) usually means the presentation has been updated. However, a changed ID can also be due to internal factors such as ID format changes. |
| `slide_properties` | String | Slide specific properties. Only set if page_type = SLIDE. |
| `notes_properties` | String | Notes specific properties. Only set if page_type = NOTES. |
| `master_properties` | String | Master specific properties. Only set if page_type = MASTER. |
| `page_elements` | Vec<String> | The page elements rendered on the page. |
| `object_id` | String | The object ID for this page. Object IDs used by Page and PageElement share the same namespace. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access page outputs
page_id = page.id
page_layout_properties = page.layout_properties
page_page_type = page.page_type
page_page_properties = page.page_properties
page_revision_id = page.revision_id
page_slide_properties = page.slide_properties
page_notes_properties = page.notes_properties
page_master_properties = page.master_properties
page_page_elements = page.page_elements
page_object_id = page.object_id
```

---


### Presentation

Creates a blank presentation using the title given in the request. If a `presentationId` is provided, it is used as the ID of the new presentation. Otherwise, a new ID is generated. Other fields in the request, including any provided content, are ignored. Returns the created presentation.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `slides` | Vec<String> |  | The slides in the presentation. A slide inherits properties from a slide layout. |
| `locale` | String |  | The locale of the presentation, as an IETF BCP 47 language tag. |
| `title` | String |  | The title of the presentation. |
| `layouts` | Vec<String> |  | The layouts in the presentation. A layout is a template that determines how content is arranged and styled on the slides that inherit from that layout. |
| `masters` | Vec<String> |  | The slide masters in the presentation. A slide master contains all common page elements and the common properties for a set of layouts. They serve three purposes: - Placeholder shapes on a master contain the default text styles and shape properties of all placeholder shapes on pages that use that master. - The master page properties define the common page properties inherited by its layouts. - Any other shapes on the master slide appear on all slides using that master, regardless of their layout. |
| `revision_id` | String |  | Output only. The revision ID of the presentation. Can be used in update requests to assert the presentation revision hasn't changed since the last read operation. Only populated if the user has edit access to the presentation. The revision ID is not a sequential number but a nebulous string. The format of the revision ID may change over time, so it should be treated opaquely. A returned revision ID is only guaranteed to be valid for 24 hours after it has been returned and cannot be shared across users. If the revision ID is unchanged between calls, then the presentation has not changed. Conversely, a changed ID (for the same presentation and user) usually means the presentation has been updated. However, a changed ID can also be due to internal factors such as ID format changes. |
| `presentation_id` | String |  | The ID of the presentation. |
| `page_size` | String |  | The size of pages in the presentation. |
| `notes_master` | String |  | The notes master in the presentation. It serves three purposes: - Placeholder shapes on a notes master contain the default text styles and shape properties of all placeholder shapes on notes pages. Specifically, a `SLIDE_IMAGE` placeholder shape contains the slide thumbnail, and a `BODY` placeholder shape contains the speaker notes. - The notes master page properties define the common page properties inherited by all notes pages. - Any other shapes on the notes master appear on all notes pages. The notes master is read-only. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `slides` | Vec<String> | The slides in the presentation. A slide inherits properties from a slide layout. |
| `locale` | String | The locale of the presentation, as an IETF BCP 47 language tag. |
| `title` | String | The title of the presentation. |
| `layouts` | Vec<String> | The layouts in the presentation. A layout is a template that determines how content is arranged and styled on the slides that inherit from that layout. |
| `masters` | Vec<String> | The slide masters in the presentation. A slide master contains all common page elements and the common properties for a set of layouts. They serve three purposes: - Placeholder shapes on a master contain the default text styles and shape properties of all placeholder shapes on pages that use that master. - The master page properties define the common page properties inherited by its layouts. - Any other shapes on the master slide appear on all slides using that master, regardless of their layout. |
| `revision_id` | String | Output only. The revision ID of the presentation. Can be used in update requests to assert the presentation revision hasn't changed since the last read operation. Only populated if the user has edit access to the presentation. The revision ID is not a sequential number but a nebulous string. The format of the revision ID may change over time, so it should be treated opaquely. A returned revision ID is only guaranteed to be valid for 24 hours after it has been returned and cannot be shared across users. If the revision ID is unchanged between calls, then the presentation has not changed. Conversely, a changed ID (for the same presentation and user) usually means the presentation has been updated. However, a changed ID can also be due to internal factors such as ID format changes. |
| `presentation_id` | String | The ID of the presentation. |
| `page_size` | String | The size of pages in the presentation. |
| `notes_master` | String | The notes master in the presentation. It serves three purposes: - Placeholder shapes on a notes master contain the default text styles and shape properties of all placeholder shapes on notes pages. Specifically, a `SLIDE_IMAGE` placeholder shape contains the slide thumbnail, and a `BODY` placeholder shape contains the speaker notes. - The notes master page properties define the common page properties inherited by all notes pages. - Any other shapes on the notes master appear on all notes pages. The notes master is read-only. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create presentation
presentation = provider.slides_api.Presentation {
}

# Access presentation outputs
presentation_id = presentation.id
presentation_slides = presentation.slides
presentation_locale = presentation.locale
presentation_title = presentation.title
presentation_layouts = presentation.layouts
presentation_masters = presentation.masters
presentation_revision_id = presentation.revision_id
presentation_presentation_id = presentation.presentation_id
presentation_page_size = presentation.page_size
presentation_notes_master = presentation.notes_master
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple page resources
page_0 = provider.slides_api.Page {
}
page_1 = provider.slides_api.Page {
}
page_2 = provider.slides_api.Page {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    page = provider.slides_api.Page {
    }
```

---

## Related Documentation

- [GCP Slides_api Documentation](https://cloud.google.com/slides_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
