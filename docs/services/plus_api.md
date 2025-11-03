# Plus_api Service



**Resources**: 3

---

## Overview

The plus_api service provides access to 3 resource types:

- [Activitie](#activitie) [R]
- [Comment](#comment) [R]
- [People](#people) [R]

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
| `place_id` | String | ID of the place where this activity occurred. |
| `provider` | String | The service provider that initially published this activity. |
| `kind` | String | Identifies this resource as an activity. Value: "plus#activity". |
| `updated` | String | The time at which this activity was last updated. Formatted as an RFC 3339 timestamp. |
| `verb` | String | This activity's verb, which indicates the action that was performed. Possible values include, but are not limited to, the following values:  
- "post" - Publish content to the stream. 
- "share" - Reshare an activity. |
| `object` | String | The object of this activity. |
| `location` | String | The location where this activity occurred. |
| `id` | String | The ID of this activity. |
| `radius` | String | Radius, in meters, of the region where this activity occurred, centered at the latitude and longitude identified in geocode. |
| `address` | String | Street address where this activity occurred. |
| `geocode` | String | Latitude and longitude where this activity occurred. Format is latitude followed by longitude, space separated. |
| `actor` | String | The person who performed this activity. |
| `url` | String | The link to this activity. |
| `published` | String | The time at which this activity was initially published. Formatted as an RFC 3339 timestamp. |
| `title` | String | Title of this activity. |
| `annotation` | String | Additional content added by the person who shared this activity, applicable only when resharing an activity. |
| `access` | String | Identifies who has access to see this activity. |
| `crosspost_source` | String | If this activity is a crosspost from another system, this property specifies the ID of the original activity. |
| `place_name` | String | Name of the place where this activity occurred. |
| `etag` | String | ETag of this response for caching purposes. |


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
activitie_place_id = activitie.place_id
activitie_provider = activitie.provider
activitie_kind = activitie.kind
activitie_updated = activitie.updated
activitie_verb = activitie.verb
activitie_object = activitie.object
activitie_location = activitie.location
activitie_id = activitie.id
activitie_radius = activitie.radius
activitie_address = activitie.address
activitie_geocode = activitie.geocode
activitie_actor = activitie.actor
activitie_url = activitie.url
activitie_published = activitie.published
activitie_title = activitie.title
activitie_annotation = activitie.annotation
activitie_access = activitie.access
activitie_crosspost_source = activitie.crosspost_source
activitie_place_name = activitie.place_name
activitie_etag = activitie.etag
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
| `actor` | String | The person who posted this comment. |
| `plusoners` | String | People who +1'd this comment. |
| `etag` | String | ETag of this response for caching purposes. |
| `self_link` | String | Link to this comment resource. |
| `updated` | String | The time at which this comment was last updated. Formatted as an RFC 3339 timestamp. |
| `id` | String | The ID of this comment. |
| `in_reply_to` | Vec<String> | The activity this comment replied to. |
| `kind` | String | Identifies this resource as a comment. Value: "plus#comment". |
| `published` | String | The time at which this comment was initially published. Formatted as an RFC 3339 timestamp. |
| `object` | String | The object of this comment. |
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
comment_actor = comment.actor
comment_plusoners = comment.plusoners
comment_etag = comment.etag
comment_self_link = comment.self_link
comment_updated = comment.updated
comment_id = comment.id
comment_in_reply_to = comment.in_reply_to
comment_kind = comment.kind
comment_published = comment.published
comment_object = comment.object
comment_verb = comment.verb
```

---


### People

Get a person's profile. If your app uses scope https://www.googleapis.com/auth/plus.login, this method is guaranteed to return ageRange and language.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `emails` | Vec<String> | A list of email addresses that this person has, including their Google account email address, and the public verified email addresses on their Google+ profile. The plus.profile.emails.read scope is needed to retrieve these email addresses, or the email scope can be used to retrieve just the Google account email address. |
| `places_lived` | Vec<String> | A list of places where this person has lived. |
| `gender` | String | The person's gender. Possible values include, but are not limited to, the following values:  
- "male" - Male gender. 
- "female" - Female gender. 
- "other" - Other. |
| `birthday` | String | The person's date of birth, represented as YYYY-MM-DD. |
| `verified` | bool | Whether the person or Google+ Page has been verified. |
| `urls` | Vec<String> | A list of URLs for this person. |
| `id` | String | The ID of this person. |
| `age_range` | String | The age range of the person. Valid ranges are 17 or younger, 18 to 20, and 21 or older. Age is determined from the user's birthday using Western age reckoning. |
| `domain` | String | The hosted domain name for the user's Google Apps account. For instance, example.com. The plus.profile.emails.read or email scope is needed to get this domain name. |
| `bragging_rights` | String | The "bragging rights" line of this person. |
| `plus_one_count` | i64 | If a Google+ Page, the number of people who have +1'd this page. |
| `skills` | String | The person's skills. |
| `etag` | String | ETag of this response for caching purposes. |
| `language` | String | The user's preferred language for rendering. |
| `name` | String | An object representation of the individual components of a person's name. |
| `object_type` | String | Type of person within Google+. Possible values include, but are not limited to, the following values:  
- "person" - represents an actual person. 
- "page" - represents a page. |
| `display_name` | String | The name of this person, which is suitable for display. |
| `kind` | String | Identifies this resource as a person. Value: "plus#person". |
| `circled_by_count` | i64 | For followers who are visible, the number of people who have added this person or page to a circle. |
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
| `url` | String | The URL of this person's profile. |
| `image` | String | The representation of the person's profile photo. |
| `organizations` | Vec<String> | A list of current or past organizations with which this person is associated. |
| `nickname` | String | The nickname of this person. |
| `about_me` | String | A short biography for this person. |
| `tagline` | String | The brief description (tagline) of this person. |
| `is_plus_user` | bool | Whether this user has signed up for Google+. |
| `current_location` | String | (this field is not currently used) |
| `cover` | String | The cover photo content. |
| `occupation` | String | The occupation of this person. |


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
people_emails = people.emails
people_places_lived = people.places_lived
people_gender = people.gender
people_birthday = people.birthday
people_verified = people.verified
people_urls = people.urls
people_id = people.id
people_age_range = people.age_range
people_domain = people.domain
people_bragging_rights = people.bragging_rights
people_plus_one_count = people.plus_one_count
people_skills = people.skills
people_etag = people.etag
people_language = people.language
people_name = people.name
people_object_type = people.object_type
people_display_name = people.display_name
people_kind = people.kind
people_circled_by_count = people.circled_by_count
people_relationship_status = people.relationship_status
people_url = people.url
people_image = people.image
people_organizations = people.organizations
people_nickname = people.nickname
people_about_me = people.about_me
people_tagline = people.tagline
people_is_plus_user = people.is_plus_user
people_current_location = people.current_location
people_cover = people.cover
people_occupation = people.occupation
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
activitie_0 = provider.plus_api.Activitie {
}
activitie_1 = provider.plus_api.Activitie {
}
activitie_2 = provider.plus_api.Activitie {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    activitie = provider.plus_api.Activitie {
    }
```

---

## Related Documentation

- [GCP Plus_api Documentation](https://cloud.google.com/plus_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
