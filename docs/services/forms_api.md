# Forms_api Service



**Resources**: 3

---

## Overview

The forms_api service provides access to 3 resource types:

- [Form](#form) [CR]
- [Watche](#watche) [CRD]
- [Response](#response) [R]

---

## Resources


### Form

Create a new form using the title given in the provided form message in the request. *Important:* Only the form.info.title and form.info.document_title fields are copied to the new form. All other fields including the form description, items and settings are disallowed. To create a new form and add items, you must first call forms.create to create an empty form with a title and (optional) document title, and then call forms.update to add the items.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `revision_id` | String |  | Output only. The revision ID of the form. Used in the WriteControl in update requests to identify the revision on which the changes are based. The format of the revision ID may change over time, so it should be treated opaquely. A returned revision ID is only guaranteed to be valid for 24 hours after it has been returned and cannot be shared across users. If the revision ID is unchanged between calls, then the form *content* has not changed. Conversely, a changed ID (for the same form and user) usually means the form *content* has been updated; however, a changed ID can also be due to internal factors such as ID format changes. Form content excludes form metadata, including: * sharing settings (who has access to the form) * publish_settings (if the form supports publishing and if it is published) |
| `linked_sheet_id` | String |  | Output only. The ID of the linked Google Sheet which is accumulating responses from this Form (if such a Sheet exists). |
| `info` | String |  | Required. The title and description of the form. |
| `settings` | String |  | The form's settings. This must be updated with UpdateSettingsRequest; it is ignored during CreateForm and UpdateFormInfoRequest. |
| `items` | Vec<String> |  | Required. A list of the form's items, which can include section headers, questions, embedded media, etc. |
| `responder_uri` | String |  | Output only. The form URI to share with responders. This opens a page that allows the user to submit responses but not edit the questions. For forms that have publish_settings value set, this is the published form URI. |
| `form_id` | String |  | Output only. The form ID. |
| `publish_settings` | String |  | Output only. The publishing settings for a form. This field isn't set for legacy forms because they don't have the publish_settings field. All newly created forms support publish settings. Forms with publish_settings value set can call SetPublishSettings API to publish or unpublish the form. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `revision_id` | String | Output only. The revision ID of the form. Used in the WriteControl in update requests to identify the revision on which the changes are based. The format of the revision ID may change over time, so it should be treated opaquely. A returned revision ID is only guaranteed to be valid for 24 hours after it has been returned and cannot be shared across users. If the revision ID is unchanged between calls, then the form *content* has not changed. Conversely, a changed ID (for the same form and user) usually means the form *content* has been updated; however, a changed ID can also be due to internal factors such as ID format changes. Form content excludes form metadata, including: * sharing settings (who has access to the form) * publish_settings (if the form supports publishing and if it is published) |
| `linked_sheet_id` | String | Output only. The ID of the linked Google Sheet which is accumulating responses from this Form (if such a Sheet exists). |
| `info` | String | Required. The title and description of the form. |
| `settings` | String | The form's settings. This must be updated with UpdateSettingsRequest; it is ignored during CreateForm and UpdateFormInfoRequest. |
| `items` | Vec<String> | Required. A list of the form's items, which can include section headers, questions, embedded media, etc. |
| `responder_uri` | String | Output only. The form URI to share with responders. This opens a page that allows the user to submit responses but not edit the questions. For forms that have publish_settings value set, this is the published form URI. |
| `form_id` | String | Output only. The form ID. |
| `publish_settings` | String | Output only. The publishing settings for a form. This field isn't set for legacy forms because they don't have the publish_settings field. All newly created forms support publish settings. Forms with publish_settings value set can call SetPublishSettings API to publish or unpublish the form. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create form
form = provider.forms_api.Form {
}

# Access form outputs
form_id = form.id
form_revision_id = form.revision_id
form_linked_sheet_id = form.linked_sheet_id
form_info = form.info
form_settings = form.settings
form_items = form.items
form_responder_uri = form.responder_uri
form_form_id = form.form_id
form_publish_settings = form.publish_settings
```

---


### Watche

Create a new watch. If a watch ID is provided, it must be unused. For each invoking project, the per form limit is one watch per Watch.EventType. A watch expires seven days after it is created (see Watch.expire_time).

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `watch` | String |  | Required. The watch object. No ID should be set on this object; use `watch_id` instead. |
| `watch_id` | String |  | The ID to use for the watch. If specified, the ID must not already be in use. If not specified, an ID is generated. This value should be 4-63 characters, and valid characters are /a-z-/. |
| `form_id` | String | ✅ | Required. ID of the Form to watch. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `watches` | Vec<String> | The returned watches. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create watche
watche = provider.forms_api.Watche {
    form_id = "value"  # Required. ID of the Form to watch.
}

# Access watche outputs
watche_id = watche.id
watche_watches = watche.watches
```

---


### Response

Get one response from the form.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `answers` | HashMap<String, String> | Output only. The actual answers to the questions, keyed by question_id. |
| `form_id` | String | Output only. The form ID. |
| `create_time` | String | Output only. Timestamp for the first time the response was submitted. |
| `last_submitted_time` | String | Output only. Timestamp for the most recent time the response was submitted. Does not track changes to grades. |
| `respondent_email` | String | Output only. The email address (if collected) for the respondent. |
| `response_id` | String | Output only. The response ID. |
| `total_score` | f64 | Output only. The total number of points the respondent received for their submission Only set if the form was a quiz and the response was graded. This includes points automatically awarded via autograding adjusted by any manual corrections entered by the form owner. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access response outputs
response_id = response.id
response_answers = response.answers
response_form_id = response.form_id
response_create_time = response.create_time
response_last_submitted_time = response.last_submitted_time
response_respondent_email = response.respondent_email
response_response_id = response.response_id
response_total_score = response.total_score
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple form resources
form_0 = provider.forms_api.Form {
}
form_1 = provider.forms_api.Form {
}
form_2 = provider.forms_api.Form {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    form = provider.forms_api.Form {
    }
```

---

## Related Documentation

- [GCP Forms_api Documentation](https://cloud.google.com/forms_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
