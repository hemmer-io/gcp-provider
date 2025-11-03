# Essentialcontacts_api Service



**Resources**: 1

---

## Overview

The essentialcontacts_api service provides access to 1 resource type:

- [Contact](#contact) [CRUD]

---

## Resources


### Contact

Adds a new contact for a resource.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `validate_time` | String |  | Output only. The last time the validation_state was updated, either manually or automatically. A contact is considered stale if its validation state was updated more than 1 year ago. |
| `name` | String |  | Output only. The identifier for the contact. Format: {resource_type}/{resource_id}/contacts/{contact_id} |
| `language_tag` | String |  | Required. The preferred language for notifications, as a ISO 639-1 language code. See [Supported languages](https://cloud.google.com/resource-manager/docs/managing-notification-contacts#supported-languages) for a list of supported languages. |
| `notification_category_subscriptions` | Vec<String> |  | Required. The categories of notifications that the contact will receive communications for. |
| `validation_state` | String |  | Output only. The validity of the contact. A contact is considered valid if it is the correct recipient for notifications for a particular resource. |
| `email` | String |  | Required. The email address to send notifications to. The email address does not need to be a Google Account. |
| `parent` | String | ✅ | Required. The resource to save this contact for. Format: organizations/{organization_id}, folders/{folder_id} or projects/{project_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `validate_time` | String | Output only. The last time the validation_state was updated, either manually or automatically. A contact is considered stale if its validation state was updated more than 1 year ago. |
| `name` | String | Output only. The identifier for the contact. Format: {resource_type}/{resource_id}/contacts/{contact_id} |
| `language_tag` | String | Required. The preferred language for notifications, as a ISO 639-1 language code. See [Supported languages](https://cloud.google.com/resource-manager/docs/managing-notification-contacts#supported-languages) for a list of supported languages. |
| `notification_category_subscriptions` | Vec<String> | Required. The categories of notifications that the contact will receive communications for. |
| `validation_state` | String | Output only. The validity of the contact. A contact is considered valid if it is the correct recipient for notifications for a particular resource. |
| `email` | String | Required. The email address to send notifications to. The email address does not need to be a Google Account. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create contact
contact = provider.essentialcontacts_api.Contact {
    parent = "value"  # Required. The resource to save this contact for. Format: organizations/{organization_id}, folders/{folder_id} or projects/{project_id}
}

# Access contact outputs
contact_id = contact.id
contact_validate_time = contact.validate_time
contact_name = contact.name
contact_language_tag = contact.language_tag
contact_notification_category_subscriptions = contact.notification_category_subscriptions
contact_validation_state = contact.validation_state
contact_email = contact.email
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple contact resources
contact_0 = provider.essentialcontacts_api.Contact {
    parent = "value-0"
}
contact_1 = provider.essentialcontacts_api.Contact {
    parent = "value-1"
}
contact_2 = provider.essentialcontacts_api.Contact {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    contact = provider.essentialcontacts_api.Contact {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Essentialcontacts_api Documentation](https://cloud.google.com/essentialcontacts_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
