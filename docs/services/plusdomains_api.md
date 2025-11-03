# Plusdomains_api Service



**Resources**: 6

---

## Overview

The plusdomains_api service provides access to 6 resource types:

- [Activitie](#activitie) [R]
- [Audience](#audience) [R]
- [Circle](#circle) [R]
- [Comment](#comment) [R]
- [People](#people) [R]
- [Media](#media) [C]

---

## Resources


### Activitie

Shut down. See https://developers.google.com/+/api-shutdown for more details.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `url` | String | The link to this activity. |
| `etag` | String | ETag of this response for caching purposes. |
| `geocode` | String | Latitude and longitude where this activity occurred. Format is latitude followed by longitude, space separated. |
| `id` | String | The ID of this activity. |
| `address` | String | Street address where this activity occurred. |
| `annotation` | String | Additional content added by the person who shared this activity, applicable only when resharing an activity. |
| `title` | String | Title of this activity. |
| `kind` | String | Identifies this resource as an activity. Value: "plus#activity". |
| `place_id` | String | ID of the place where this activity occurred. |
| `crosspost_source` | String | If this activity is a crosspost from another system, this property specifies the ID of the original activity. |
| `access` | String | Identifies who has access to see this activity. |
| `place_name` | String | Name of the place where this activity occurred. |
| `published` | String | The time at which this activity was initially published. Formatted as an RFC 3339 timestamp. |
| `actor` | String | The person who performed this activity. |
| `object` | String | The object of this activity. |
| `provider` | String | The service provider that initially published this activity. |
| `verb` | String | This activity's verb, which indicates the action that was performed. Possible values include, but are not limited to, the following values:  
- "post" - Publish content to the stream. 
- "share" - Reshare an activity. |
| `location` | String | The location where this activity occurred. |
| `updated` | String | The time at which this activity was last updated. Formatted as an RFC 3339 timestamp. |
| `radius` | String | Radius, in meters, of the region where this activity occurred, centered at the latitude and longitude identified in geocode. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access activitie outputs
activitie_id = activitie.id
activitie_url = activitie.url
activitie_etag = activitie.etag
activitie_geocode = activitie.geocode
activitie_id = activitie.id
activitie_address = activitie.address
activitie_annotation = activitie.annotation
activitie_title = activitie.title
activitie_kind = activitie.kind
activitie_place_id = activitie.place_id
activitie_crosspost_source = activitie.crosspost_source
activitie_access = activitie.access
activitie_place_name = activitie.place_name
activitie_published = activitie.published
activitie_actor = activitie.actor
activitie_object = activitie.object
activitie_provider = activitie.provider
activitie_verb = activitie.verb
activitie_location = activitie.location
activitie_updated = activitie.updated
activitie_radius = activitie.radius
```

---


### Audience

Shut down. See https://developers.google.com/+/api-shutdown for more details.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | The continuation token, which is used to page through large result sets. Provide this value in a subsequent request to return the next page of results. |
| `etag` | String | ETag of this response for caching purposes. |
| `items` | Vec<String> | The audiences in this result. |
| `kind` | String | Identifies this resource as a collection of audiences. Value: "plus#audienceFeed". |
| `total_items` | i64 | The total number of ACL entries. The number of entries in this response may be smaller due to paging. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access audience outputs
audience_id = audience.id
audience_next_page_token = audience.next_page_token
audience_etag = audience.etag
audience_items = audience.items
audience_kind = audience.kind
audience_total_items = audience.total_items
```

---


### Circle

Shut down. See https://developers.google.com/+/api-shutdown for more details.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | ETag of this response for caching purposes. |
| `self_link` | String | Link to this page of circles. |
| `items` | Vec<String> | The circles in this page of results. |
| `next_link` | String | Link to the next page of circles. |
| `kind` | String | Identifies this resource as a collection of circles. Value: "plus#circleFeed". |
| `total_items` | i64 | The total number of circles. The number of circles in this response may be smaller due to paging. |
| `next_page_token` | String | The continuation token, which is used to page through large result sets. Provide this value in a subsequent request to return the next page of results. |
| `title` | String | The title of this list of resources. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access circle outputs
circle_id = circle.id
circle_etag = circle.etag
circle_self_link = circle.self_link
circle_items = circle.items
circle_next_link = circle.next_link
circle_kind = circle.kind
circle_total_items = circle.total_items
circle_next_page_token = circle.next_page_token
circle_title = circle.title
```

---


### Comment

Shut down. See https://developers.google.com/+/api-shutdown for more details.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `object` | String | The object of this comment. |
| `actor` | String | The person who posted this comment. |
| `published` | String | The time at which this comment was initially published. Formatted as an RFC 3339 timestamp. |
| `id` | String | The ID of this comment. |
| `in_reply_to` | Vec<String> | The activity this comment replied to. |
| `self_link` | String | Link to this comment resource. |
| `updated` | String | The time at which this comment was last updated. Formatted as an RFC 3339 timestamp. |
| `kind` | String | Identifies this resource as a comment. Value: "plus#comment". |
| `plusoners` | String | People who +1'd this comment. |
| `etag` | String | ETag of this response for caching purposes. |
| `verb` | String | This comment's verb, indicating what action was performed. Possible values are:  
- "post" - Publish content to the stream. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access comment outputs
comment_id = comment.id
comment_object = comment.object
comment_actor = comment.actor
comment_published = comment.published
comment_id = comment.id
comment_in_reply_to = comment.in_reply_to
comment_self_link = comment.self_link
comment_updated = comment.updated
comment_kind = comment.kind
comment_plusoners = comment.plusoners
comment_etag = comment.etag
comment_verb = comment.verb
```

---


### People

Get a person's profile.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `relationship_status` | String | The person's relationship status. Possible values include, but are not limited to, the following values:  
- "single" - Person is single. 
- "in_a_relationship" - Person is in a relationship. 
- "engaged" - Person is engaged. 
- "married" - Person is married. 
- "its_complicated" - The relationship is complicated. 
- "open_relationship" - Person is in an open relationship. 
- "widowed" - Person is widowed. 
- "in_domestic_partnership" - Person is in a domestic partnership. 
- "in_civil_union" - Person is in a civil union. |
| `id` | String | The ID of this person. |
| `occupation` | String | The occupation of this person. |
| `cover` | String | The cover photo content. |
| `organizations` | Vec<String> | A list of current or past organizations with which this person is associated. |
| `emails` | Vec<String> | A list of email addresses that this person has, including their Google account email address, and the public verified email addresses on their Google+ profile. The plus.profile.emails.read scope is needed to retrieve these email addresses, or the email scope can be used to retrieve just the Google account email address. |
| `plus_one_count` | i64 | If a Google+ Page, the number of people who have +1'd this page. |
| `about_me` | String | A short biography for this person. |
| `etag` | String | ETag of this response for caching purposes. |
| `is_plus_user` | bool | Whether this user has signed up for Google+. |
| `kind` | String | Identifies this resource as a person. Value: "plus#person". |
| `url` | String | The URL of this person's profile. |
| `name` | String | An object representation of the individual components of a person's name. |
| `nickname` | String | The nickname of this person. |
| `bragging_rights` | String | The "bragging rights" line of this person. |
| `circled_by_count` | i64 | For followers who are visible, the number of people who have added this person or page to a circle. |
| `gender` | String | The person's gender. Possible values include, but are not limited to, the following values:  
- "male" - Male gender. 
- "female" - Female gender. 
- "other" - Other. |
| `object_type` | String | Type of person within Google+. Possible values include, but are not limited to, the following values:  
- "person" - represents an actual person. 
- "page" - represents a page. |
| `image` | String | The representation of the person's profile photo. |
| `places_lived` | Vec<String> | A list of places where this person has lived. |
| `skills` | String | The person's skills. |
| `domain` | String | The hosted domain name for the user's Google Apps account. For instance, example.com. The plus.profile.emails.read or email scope is needed to get this domain name. |
| `birthday` | String | The person's date of birth, represented as YYYY-MM-DD. |
| `current_location` | String | (this field is not currently used) |
| `tagline` | String | The brief description (tagline) of this person. |
| `display_name` | String | The name of this person, which is suitable for display. |
| `urls` | Vec<String> | A list of URLs for this person. |
| `verified` | bool | Whether the person or Google+ Page has been verified. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access people outputs
people_id = people.id
people_relationship_status = people.relationship_status
people_id = people.id
people_occupation = people.occupation
people_cover = people.cover
people_organizations = people.organizations
people_emails = people.emails
people_plus_one_count = people.plus_one_count
people_about_me = people.about_me
people_etag = people.etag
people_is_plus_user = people.is_plus_user
people_kind = people.kind
people_url = people.url
people_name = people.name
people_nickname = people.nickname
people_bragging_rights = people.bragging_rights
people_circled_by_count = people.circled_by_count
people_gender = people.gender
people_object_type = people.object_type
people_image = people.image
people_places_lived = people.places_lived
people_skills = people.skills
people_domain = people.domain
people_birthday = people.birthday
people_current_location = people.current_location
people_tagline = people.tagline
people_display_name = people.display_name
people_urls = people.urls
people_verified = people.verified
```

---


### Media

Shut down. See https://developers.google.com/+/api-shutdown for more details.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `summary` | String |  | A description, or caption, for this media. |
| `id` | String |  | ID of this media, which is generated by the API. |
| `url` | String |  | The URL for the page that hosts this media. |
| `etag` | String |  | ETag of this response for caching purposes. |
| `video_duration` | String |  | The duration in milliseconds of this video. |
| `size_bytes` | String |  | The size in bytes of this video. |
| `author` | String |  | The person who uploaded this media. |
| `updated` | String |  | The time at which this media was last updated. This includes changes to media metadata. Formatted as an RFC 3339 timestamp. |
| `exif` | String |  | Exif information of the media item. |
| `published` | String |  | The time at which this media was uploaded. Formatted as an RFC 3339 timestamp. |
| `kind` | String |  | The type of resource. |
| `height` | i64 |  | The height in pixels of the original image. |
| `media_url` | String |  | The URL of this photo or video's still image. |
| `width` | i64 |  | The width in pixels of the original image. |
| `display_name` | String |  | The display name for this media. |
| `video_status` | String |  | The encoding status of this video. Possible values are:  
- "UPLOADING" - Not all the video bytes have been received. 
- "PENDING" - Video not yet processed. 
- "FAILED" - Video processing failed. 
- "READY" - A single video stream is playable. 
- "FINAL" - All video streams are playable. |
| `streams` | Vec<String> |  | The list of video streams for this video. There might be several different streams available for a single video, either Flash or MPEG, of various sizes |
| `media_created_time` | String |  | The time at which this media was originally created in UTC. Formatted as an RFC 3339 timestamp that matches this example: 2010-11-25T14:30:27.655Z |
| `collection` | String | ✅ |  |
| `user_id` | String | ✅ | The ID of the user to create the activity on behalf of. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create media
media = provider.plusdomains_api.Media {
    collection = "value"  # Required field
    user_id = "value"  # The ID of the user to create the activity on behalf of.
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

# Create multiple activitie resources
activitie_0 = provider.plusdomains_api.Activitie {
}
activitie_1 = provider.plusdomains_api.Activitie {
}
activitie_2 = provider.plusdomains_api.Activitie {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    activitie = provider.plusdomains_api.Activitie {
    }
```

---

## Related Documentation

- [GCP Plusdomains_api Documentation](https://cloud.google.com/plusdomains_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
