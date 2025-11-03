# People_api Service



**Resources**: 5

---

## Overview

The people_api service provides access to 5 resource types:

- [Other_contact](#other_contact) [CR]
- [Member](#member) [C]
- [Connection](#connection) [R]
- [People](#people) [CRUD]
- [Contact_group](#contact_group) [CRUD]

---

## Resources


### Other_contact

Copies an "Other contact" to a new contact in the user's "myContacts" group Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `sources` | Vec<String> |  | Optional. A mask of what source types to return. Defaults to READ_SOURCE_TYPE_CONTACT and READ_SOURCE_TYPE_PROFILE if not set. |
| `copy_mask` | String |  | Required. A field mask to restrict which fields are copied into the new contact. Valid values are: * emailAddresses * names * phoneNumbers |
| `read_mask` | String |  | Optional. A field mask to restrict which fields on the person are returned. Multiple fields can be specified by separating them with commas. Defaults to the copy mask with metadata and membership fields if not set. Valid values are: * addresses * ageRanges * biographies * birthdays * calendarUrls * clientData * coverPhotos * emailAddresses * events * externalIds * genders * imClients * interests * locales * locations * memberships * metadata * miscKeywords * names * nicknames * occupations * organizations * phoneNumbers * photos * relations * sipAddresses * skills * urls * userDefined |
| `resource_name` | String | ✅ | Required. The resource name of the "Other contact" to copy. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `total_size` | i64 | The total number of other contacts in the list without pagination. |
| `other_contacts` | Vec<String> | The list of "Other contacts" returned as Person resources. "Other contacts" support a limited subset of fields. See ListOtherContactsRequest.request_mask for more detailed information. |
| `next_sync_token` | String | A token, which can be sent as `sync_token` to retrieve changes since the last request. Request must set `request_sync_token` to return the sync token. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create other_contact
other_contact = provider.people_api.Other_contact {
    resource_name = "value"  # Required. The resource name of the "Other contact" to copy.
}

# Access other_contact outputs
other_contact_id = other_contact.id
other_contact_next_page_token = other_contact.next_page_token
other_contact_total_size = other_contact.total_size
other_contact_other_contacts = other_contact.other_contacts
other_contact_next_sync_token = other_contact.next_sync_token
```

---


### Member

Modify the members of a contact group owned by the authenticated user. The only system contact groups that can have members added are `contactGroups/myContacts` and `contactGroups/starred`. Other system contact groups are deprecated and can only have contacts removed.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `resource_names_to_add` | Vec<String> |  | Optional. The resource names of the contact people to add in the form of `people/{person_id}`. The total number of resource names in `resource_names_to_add` and `resource_names_to_remove` must be less than or equal to 1000. |
| `resource_names_to_remove` | Vec<String> |  | Optional. The resource names of the contact people to remove in the form of `people/{person_id}`. The total number of resource names in `resource_names_to_add` and `resource_names_to_remove` must be less than or equal to 1000. |
| `resource_name` | String | ✅ | Required. The resource name of the contact group to modify. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create member
member = provider.people_api.Member {
    resource_name = "value"  # Required. The resource name of the contact group to modify.
}

```

---


### Connection

Provides a list of the authenticated user's contacts. Sync tokens expire 7 days after the full sync. A request with an expired sync token will get an error with an [google.rpc.ErrorInfo](https://cloud.google.com/apis/design/errors#error_info) with reason "EXPIRED_SYNC_TOKEN". In the case of such an error clients should make a full sync request without a `sync_token`. The first page of a full sync request has an additional quota. If the quota is exceeded, a 429 error will be returned. This quota is fixed and can not be increased. When the `sync_token` is specified, resources deleted since the last sync will be returned as a person with `PersonMetadata.deleted` set to true. When the `page_token` or `sync_token` is specified, all other request parameters must match the first call. Writes may have a propagation delay of several minutes for sync requests. Incremental syncs are not intended for read-after-write use cases. See example usage at [List the user's contacts that have changed](/people/v1/contacts#list_the_users_contacts_that_have_changed).

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token, which can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `total_items` | i64 | The total number of items in the list without pagination. |
| `next_sync_token` | String | A token, which can be sent as `sync_token` to retrieve changes since the last request. Request must set `request_sync_token` to return the sync token. When the response is paginated, only the last page will contain `nextSyncToken`. |
| `total_people` | i64 | **DEPRECATED** (Please use totalItems) The total number of people in the list without pagination. |
| `connections` | Vec<String> | The list of people that the requestor is connected to. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access connection outputs
connection_id = connection.id
connection_next_page_token = connection.next_page_token
connection_total_items = connection.total_items
connection_next_sync_token = connection.next_sync_token
connection_total_people = connection.total_people
connection_connections = connection.connections
```

---


### People

Update a batch of contacts and return a map of resource names to PersonResponses for the updated contacts. Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `read_mask` | String |  | Required. A field mask to restrict which fields on each person are returned. Multiple fields can be specified by separating them with commas. If read mask is left empty, the post-mutate-get is skipped and no data will be returned in the response. Valid values are: * addresses * ageRanges * biographies * birthdays * calendarUrls * clientData * coverPhotos * emailAddresses * events * externalIds * genders * imClients * interests * locales * locations * memberships * metadata * miscKeywords * names * nicknames * occupations * organizations * phoneNumbers * photos * relations * sipAddresses * skills * urls * userDefined |
| `contacts` | HashMap<String, String> |  | Required. A map of resource names to the person data to be updated. Allows up to 200 contacts in a single request. |
| `sources` | Vec<String> |  | Optional. A mask of what source types to return. Defaults to READ_SOURCE_TYPE_CONTACT and READ_SOURCE_TYPE_PROFILE if not set. |
| `update_mask` | String |  | Required. A field mask to restrict which fields on the person are updated. Multiple fields can be specified by separating them with commas. All specified fields will be replaced, or cleared if left empty for each person. Valid values are: * addresses * biographies * birthdays * calendarUrls * clientData * emailAddresses * events * externalIds * genders * imClients * interests * locales * locations * memberships * miscKeywords * names * nicknames * occupations * organizations * phoneNumbers * relations * sipAddresses * urls * userDefined |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `names` | Vec<String> | The person's names. This field is a singleton for contact sources. |
| `cover_photos` | Vec<String> | Output only. The person's cover photos. |
| `email_addresses` | Vec<String> | The person's email addresses. For `people.connections.list` and `otherContacts.list` the number of email addresses is limited to 100. If a Person has more email addresses the entire set can be obtained by calling GetPeople. |
| `skills` | Vec<String> | The person's skills. |
| `nicknames` | Vec<String> | The person's nicknames. |
| `sip_addresses` | Vec<String> | The person's SIP addresses. |
| `im_clients` | Vec<String> | The person's instant messaging clients. |
| `urls` | Vec<String> | The person's associated URLs. |
| `relationship_statuses` | Vec<String> | Output only. **DEPRECATED**: No data will be returned The person's relationship statuses. |
| `biographies` | Vec<String> | The person's biographies. This field is a singleton for contact sources. |
| `external_ids` | Vec<String> | The person's external IDs. |
| `addresses` | Vec<String> | The person's street addresses. |
| `age_range` | String | Output only. **DEPRECATED** (Please use `person.ageRanges` instead) The person's age range. |
| `etag` | String | The [HTTP entity tag](https://en.wikipedia.org/wiki/HTTP_ETag) of the resource. Used for web cache validation. |
| `misc_keywords` | Vec<String> | The person's miscellaneous keywords. |
| `file_ases` | Vec<String> | The person's file-ases. |
| `age_ranges` | Vec<String> | Output only. The person's age ranges. |
| `memberships` | Vec<String> | The person's group memberships. |
| `organizations` | Vec<String> | The person's past or current organizations. |
| `birthdays` | Vec<String> | The person's birthdays. This field is a singleton for contact sources. |
| `calendar_urls` | Vec<String> | The person's calendar URLs. |
| `locales` | Vec<String> | The person's locale preferences. |
| `relationship_interests` | Vec<String> | Output only. **DEPRECATED**: No data will be returned The person's relationship interests. |
| `occupations` | Vec<String> | The person's occupations. |
| `residences` | Vec<String> | **DEPRECATED**: (Please use `person.locations` instead) The person's residences. |
| `photos` | Vec<String> | Output only. The person's photos. |
| `taglines` | Vec<String> | Output only. **DEPRECATED**: No data will be returned The person's taglines. |
| `phone_numbers` | Vec<String> | The person's phone numbers. For `people.connections.list` and `otherContacts.list` the number of phone numbers is limited to 100. If a Person has more phone numbers the entire set can be obtained by calling GetPeople. |
| `metadata` | String | Output only. Metadata about the person. |
| `relations` | Vec<String> | The person's relations. |
| `client_data` | Vec<String> | The person's client data. |
| `user_defined` | Vec<String> | The person's user defined data. |
| `locations` | Vec<String> | The person's locations. |
| `interests` | Vec<String> | The person's interests. |
| `events` | Vec<String> | The person's events. |
| `bragging_rights` | Vec<String> | **DEPRECATED**: No data will be returned The person's bragging rights. |
| `genders` | Vec<String> | The person's genders. This field is a singleton for contact sources. |
| `resource_name` | String | The resource name for the person, assigned by the server. An ASCII string in the form of `people/{person_id}`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create people
people = provider.people_api.People {
}

# Access people outputs
people_id = people.id
people_names = people.names
people_cover_photos = people.cover_photos
people_email_addresses = people.email_addresses
people_skills = people.skills
people_nicknames = people.nicknames
people_sip_addresses = people.sip_addresses
people_im_clients = people.im_clients
people_urls = people.urls
people_relationship_statuses = people.relationship_statuses
people_biographies = people.biographies
people_external_ids = people.external_ids
people_addresses = people.addresses
people_age_range = people.age_range
people_etag = people.etag
people_misc_keywords = people.misc_keywords
people_file_ases = people.file_ases
people_age_ranges = people.age_ranges
people_memberships = people.memberships
people_organizations = people.organizations
people_birthdays = people.birthdays
people_calendar_urls = people.calendar_urls
people_locales = people.locales
people_relationship_interests = people.relationship_interests
people_occupations = people.occupations
people_residences = people.residences
people_photos = people.photos
people_taglines = people.taglines
people_phone_numbers = people.phone_numbers
people_metadata = people.metadata
people_relations = people.relations
people_client_data = people.client_data
people_user_defined = people.user_defined
people_locations = people.locations
people_interests = people.interests
people_events = people.events
people_bragging_rights = people.bragging_rights
people_genders = people.genders
people_resource_name = people.resource_name
```

---


### Contact_group

Create a new contact group owned by the authenticated user. Created contact group names must be unique to the users contact groups. Attempting to create a group with a duplicate name will return a HTTP 409 error. Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `read_group_fields` | String |  | Optional. A field mask to restrict which fields on the group are returned. Defaults to `metadata`, `groupType`, and `name` if not set or set to empty. Valid fields are: * clientData * groupType * metadata * name |
| `contact_group` | String |  | Required. The contact group to create. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `client_data` | Vec<String> | The group's client data. |
| `etag` | String | The [HTTP entity tag](https://en.wikipedia.org/wiki/HTTP_ETag) of the resource. Used for web cache validation. |
| `metadata` | String | Output only. Metadata about the contact group. |
| `group_type` | String | Output only. The contact group type. |
| `name` | String | The contact group name set by the group owner or a system provided name for system groups. For [`contactGroups.create`](/people/api/rest/v1/contactGroups/create) or [`contactGroups.update`](/people/api/rest/v1/contactGroups/update) the name must be unique to the users contact groups. Attempting to create a group with a duplicate name will return a HTTP 409 error. |
| `member_resource_names` | Vec<String> | Output only. The list of contact person resource names that are members of the contact group. The field is only populated for GET requests and will only return as many members as `maxMembers` in the get request. |
| `member_count` | i64 | Output only. The total number of contacts in the group irrespective of max members in specified in the request. |
| `resource_name` | String | The resource name for the contact group, assigned by the server. An ASCII string, in the form of `contactGroups/{contact_group_id}`. |
| `formatted_name` | String | Output only. The name translated and formatted in the viewer's account locale or the `Accept-Language` HTTP header locale for system groups names. Group names set by the owner are the same as name. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create contact_group
contact_group = provider.people_api.Contact_group {
}

# Access contact_group outputs
contact_group_id = contact_group.id
contact_group_client_data = contact_group.client_data
contact_group_etag = contact_group.etag
contact_group_metadata = contact_group.metadata
contact_group_group_type = contact_group.group_type
contact_group_name = contact_group.name
contact_group_member_resource_names = contact_group.member_resource_names
contact_group_member_count = contact_group.member_count
contact_group_resource_name = contact_group.resource_name
contact_group_formatted_name = contact_group.formatted_name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple other_contact resources
other_contact_0 = provider.people_api.Other_contact {
    resource_name = "value-0"
}
other_contact_1 = provider.people_api.Other_contact {
    resource_name = "value-1"
}
other_contact_2 = provider.people_api.Other_contact {
    resource_name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    other_contact = provider.people_api.Other_contact {
        resource_name = "production-value"
    }
```

---

## Related Documentation

- [GCP People_api Documentation](https://cloud.google.com/people_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
