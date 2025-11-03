# Calendar_api Service



**Resources**: 8

---

## Overview

The calendar_api service provides access to 8 resource types:

- [Setting](#setting) [CR]
- [Acl](#acl) [CRUD]
- [Calendar_list](#calendar_list) [CRUD]
- [Calendar](#calendar) [CRUD]
- [Freebusy](#freebusy) [C]
- [Channel](#channel) [C]
- [Color](#color) [R]
- [Event](#event) [CRUD]

---

## Resources


### Setting

Watch for changes to Settings resources.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `address` | String |  | The address where notifications are delivered for this channel. |
| `expiration` | String |  | Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional. |
| `resource_id` | String |  | An opaque ID that identifies the resource being watched on this channel. Stable across different API versions. |
| `token` | String |  | An arbitrary string delivered to the target address with each notification delivered over this channel. Optional. |
| `type` | String |  | The type of delivery mechanism used for this channel. Valid values are "web_hook" (or "webhook"). Both values refer to a channel where Http requests are used to deliver messages. |
| `id` | String |  | A UUID or similar unique string that identifies this channel. |
| `params` | HashMap<String, String> |  | Additional parameters controlling delivery channel behavior. Optional. |
| `resource_uri` | String |  | A version-specific identifier for the watched resource. |
| `kind` | String |  | Identifies this as a notification channel used to watch for changes to a resource, which is "api#channel". |
| `payload` | bool |  | A Boolean value to indicate whether payload is wanted. Optional. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | The id of the user setting. |
| `etag` | String | ETag of the resource. |
| `kind` | String | Type of the resource ("calendar#setting"). |
| `value` | String | Value of the user setting. The format of the value depends on the ID of the setting. It must always be a UTF-8 string of length up to 1024 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create setting
setting = provider.calendar_api.Setting {
}

# Access setting outputs
setting_id = setting.id
setting_id = setting.id
setting_etag = setting.etag
setting_kind = setting.kind
setting_value = setting.value
```

---


### Acl

Creates an access control rule.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `etag` | String |  | ETag of the resource. |
| `kind` | String |  | Type of the resource ("calendar#aclRule"). |
| `scope` | String |  | The extent to which calendar access is granted by this ACL rule. |
| `id` | String |  | Identifier of the Access Control List (ACL) rule. See Sharing calendars. |
| `role` | String |  | The role assigned to the scope. Possible values are:  
- "none" - Provides no access. 
- "freeBusyReader" - Provides read access to free/busy information. 
- "reader" - Provides read access to the calendar. Private events will appear to users with reader access, but event details will be hidden. 
- "writer" - Provides read and write access to the calendar. Private events will appear to users with writer access, and event details will be visible. Provides read access to the calendar's ACLs. 
- "owner" - Provides ownership of the calendar. This role has all of the permissions of the writer role with the additional ability to manipulate ACLs. |
| `calendar_id` | String | ✅ | Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | ETag of the resource. |
| `kind` | String | Type of the resource ("calendar#aclRule"). |
| `scope` | String | The extent to which calendar access is granted by this ACL rule. |
| `id` | String | Identifier of the Access Control List (ACL) rule. See Sharing calendars. |
| `role` | String | The role assigned to the scope. Possible values are:  
- "none" - Provides no access. 
- "freeBusyReader" - Provides read access to free/busy information. 
- "reader" - Provides read access to the calendar. Private events will appear to users with reader access, but event details will be hidden. 
- "writer" - Provides read and write access to the calendar. Private events will appear to users with writer access, and event details will be visible. Provides read access to the calendar's ACLs. 
- "owner" - Provides ownership of the calendar. This role has all of the permissions of the writer role with the additional ability to manipulate ACLs. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create acl
acl = provider.calendar_api.Acl {
    calendar_id = "value"  # Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
}

# Access acl outputs
acl_id = acl.id
acl_etag = acl.etag
acl_kind = acl.kind
acl_scope = acl.scope
acl_id = acl.id
acl_role = acl.role
```

---


### Calendar_list

Inserts an existing calendar into the user's calendar list.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `background_color` | String |  | The main color of the calendar in the hexadecimal format "#0088aa". This property supersedes the index-based colorId property. To set or change this property, you need to specify colorRgbFormat=true in the parameters of the insert, update and patch methods. Optional. |
| `conference_properties` | String |  | Conferencing properties for this calendar, for example what types of conferences are allowed. |
| `access_role` | String |  | The effective access role that the authenticated user has on the calendar. Read-only. Possible values are:  
- "freeBusyReader" - Provides read access to free/busy information. 
- "reader" - Provides read access to the calendar. Private events will appear to users with reader access, but event details will be hidden. 
- "writer" - Provides read and write access to the calendar. Private events will appear to users with writer access, and event details will be visible. 
- "owner" - Provides ownership of the calendar. This role has all of the permissions of the writer role with the additional ability to see and manipulate ACLs. |
| `id` | String |  | Identifier of the calendar. |
| `hidden` | bool |  | Whether the calendar has been hidden from the list. Optional. The attribute is only returned when the calendar is hidden, in which case the value is true. |
| `primary` | bool |  | Whether the calendar is the primary calendar of the authenticated user. Read-only. Optional. The default is False. |
| `summary` | String |  | Title of the calendar. Read-only. |
| `color_id` | String |  | The color of the calendar. This is an ID referring to an entry in the calendar section of the colors definition (see the colors endpoint). This property is superseded by the backgroundColor and foregroundColor properties and can be ignored when using these properties. Optional. |
| `default_reminders` | Vec<String> |  | The default reminders that the authenticated user has for this calendar. |
| `description` | String |  | Description of the calendar. Optional. Read-only. |
| `deleted` | bool |  | Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False. |
| `notification_settings` | String |  | The notifications that the authenticated user is receiving for this calendar. |
| `summary_override` | String |  | The summary that the authenticated user has set for this calendar. Optional. |
| `foreground_color` | String |  | The foreground color of the calendar in the hexadecimal format "#ffffff". This property supersedes the index-based colorId property. To set or change this property, you need to specify colorRgbFormat=true in the parameters of the insert, update and patch methods. Optional. |
| `selected` | bool |  | Whether the calendar content shows up in the calendar UI. Optional. The default is False. |
| `kind` | String |  | Type of the resource ("calendar#calendarListEntry"). |
| `etag` | String |  | ETag of the resource. |
| `time_zone` | String |  | The time zone of the calendar. Optional. Read-only. |
| `location` | String |  | Geographic location of the calendar as free-form text. Optional. Read-only. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `background_color` | String | The main color of the calendar in the hexadecimal format "#0088aa". This property supersedes the index-based colorId property. To set or change this property, you need to specify colorRgbFormat=true in the parameters of the insert, update and patch methods. Optional. |
| `conference_properties` | String | Conferencing properties for this calendar, for example what types of conferences are allowed. |
| `access_role` | String | The effective access role that the authenticated user has on the calendar. Read-only. Possible values are:  
- "freeBusyReader" - Provides read access to free/busy information. 
- "reader" - Provides read access to the calendar. Private events will appear to users with reader access, but event details will be hidden. 
- "writer" - Provides read and write access to the calendar. Private events will appear to users with writer access, and event details will be visible. 
- "owner" - Provides ownership of the calendar. This role has all of the permissions of the writer role with the additional ability to see and manipulate ACLs. |
| `id` | String | Identifier of the calendar. |
| `hidden` | bool | Whether the calendar has been hidden from the list. Optional. The attribute is only returned when the calendar is hidden, in which case the value is true. |
| `primary` | bool | Whether the calendar is the primary calendar of the authenticated user. Read-only. Optional. The default is False. |
| `summary` | String | Title of the calendar. Read-only. |
| `color_id` | String | The color of the calendar. This is an ID referring to an entry in the calendar section of the colors definition (see the colors endpoint). This property is superseded by the backgroundColor and foregroundColor properties and can be ignored when using these properties. Optional. |
| `default_reminders` | Vec<String> | The default reminders that the authenticated user has for this calendar. |
| `description` | String | Description of the calendar. Optional. Read-only. |
| `deleted` | bool | Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False. |
| `notification_settings` | String | The notifications that the authenticated user is receiving for this calendar. |
| `summary_override` | String | The summary that the authenticated user has set for this calendar. Optional. |
| `foreground_color` | String | The foreground color of the calendar in the hexadecimal format "#ffffff". This property supersedes the index-based colorId property. To set or change this property, you need to specify colorRgbFormat=true in the parameters of the insert, update and patch methods. Optional. |
| `selected` | bool | Whether the calendar content shows up in the calendar UI. Optional. The default is False. |
| `kind` | String | Type of the resource ("calendar#calendarListEntry"). |
| `etag` | String | ETag of the resource. |
| `time_zone` | String | The time zone of the calendar. Optional. Read-only. |
| `location` | String | Geographic location of the calendar as free-form text. Optional. Read-only. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create calendar_list
calendar_list = provider.calendar_api.Calendar_list {
}

# Access calendar_list outputs
calendar_list_id = calendar_list.id
calendar_list_background_color = calendar_list.background_color
calendar_list_conference_properties = calendar_list.conference_properties
calendar_list_access_role = calendar_list.access_role
calendar_list_id = calendar_list.id
calendar_list_hidden = calendar_list.hidden
calendar_list_primary = calendar_list.primary
calendar_list_summary = calendar_list.summary
calendar_list_color_id = calendar_list.color_id
calendar_list_default_reminders = calendar_list.default_reminders
calendar_list_description = calendar_list.description
calendar_list_deleted = calendar_list.deleted
calendar_list_notification_settings = calendar_list.notification_settings
calendar_list_summary_override = calendar_list.summary_override
calendar_list_foreground_color = calendar_list.foreground_color
calendar_list_selected = calendar_list.selected
calendar_list_kind = calendar_list.kind
calendar_list_etag = calendar_list.etag
calendar_list_time_zone = calendar_list.time_zone
calendar_list_location = calendar_list.location
```

---


### Calendar

Creates a secondary calendar.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `conference_properties` | String |  | Conferencing properties for this calendar, for example what types of conferences are allowed. |
| `location` | String |  | Geographic location of the calendar as free-form text. Optional. |
| `summary` | String |  | Title of the calendar. |
| `etag` | String |  | ETag of the resource. |
| `description` | String |  | Description of the calendar. Optional. |
| `time_zone` | String |  | The time zone of the calendar. (Formatted as an IANA Time Zone Database name, e.g. "Europe/Zurich".) Optional. |
| `id` | String |  | Identifier of the calendar. To retrieve IDs call the calendarList.list() method. |
| `kind` | String |  | Type of the resource ("calendar#calendar"). |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `conference_properties` | String | Conferencing properties for this calendar, for example what types of conferences are allowed. |
| `location` | String | Geographic location of the calendar as free-form text. Optional. |
| `summary` | String | Title of the calendar. |
| `etag` | String | ETag of the resource. |
| `description` | String | Description of the calendar. Optional. |
| `time_zone` | String | The time zone of the calendar. (Formatted as an IANA Time Zone Database name, e.g. "Europe/Zurich".) Optional. |
| `id` | String | Identifier of the calendar. To retrieve IDs call the calendarList.list() method. |
| `kind` | String | Type of the resource ("calendar#calendar"). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create calendar
calendar = provider.calendar_api.Calendar {
}

# Access calendar outputs
calendar_id = calendar.id
calendar_conference_properties = calendar.conference_properties
calendar_location = calendar.location
calendar_summary = calendar.summary
calendar_etag = calendar.etag
calendar_description = calendar.description
calendar_time_zone = calendar.time_zone
calendar_id = calendar.id
calendar_kind = calendar.kind
```

---


### Freebusy

Returns free/busy information for a set of calendars.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `time_zone` | String |  | Time zone used in the response. Optional. The default is UTC. |
| `group_expansion_max` | i64 |  | Maximal number of calendar identifiers to be provided for a single group. Optional. An error is returned for a group with more members than this value. Maximum value is 100. |
| `time_min` | String |  | The start of the interval for the query formatted as per RFC3339. |
| `calendar_expansion_max` | i64 |  | Maximal number of calendars for which FreeBusy information is to be provided. Optional. Maximum value is 50. |
| `time_max` | String |  | The end of the interval for the query formatted as per RFC3339. |
| `items` | Vec<String> |  | List of calendars and/or groups to query. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create freebusy
freebusy = provider.calendar_api.Freebusy {
}

```

---


### Channel

Stop watching resources through this channel

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `address` | String |  | The address where notifications are delivered for this channel. |
| `expiration` | String |  | Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional. |
| `resource_id` | String |  | An opaque ID that identifies the resource being watched on this channel. Stable across different API versions. |
| `token` | String |  | An arbitrary string delivered to the target address with each notification delivered over this channel. Optional. |
| `type` | String |  | The type of delivery mechanism used for this channel. Valid values are "web_hook" (or "webhook"). Both values refer to a channel where Http requests are used to deliver messages. |
| `id` | String |  | A UUID or similar unique string that identifies this channel. |
| `params` | HashMap<String, String> |  | Additional parameters controlling delivery channel behavior. Optional. |
| `resource_uri` | String |  | A version-specific identifier for the watched resource. |
| `kind` | String |  | Identifies this as a notification channel used to watch for changes to a resource, which is "api#channel". |
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
channel = provider.calendar_api.Channel {
}

```

---


### Color

Returns the color definitions for calendars and events.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `calendar` | HashMap<String, String> | A global palette of calendar colors, mapping from the color ID to its definition. A calendarListEntry resource refers to one of these color IDs in its colorId field. Read-only. |
| `kind` | String | Type of the resource ("calendar#colors"). |
| `updated` | String | Last modification time of the color palette (as a RFC3339 timestamp). Read-only. |
| `event` | HashMap<String, String> | A global palette of event colors, mapping from the color ID to its definition. An event resource may refer to one of these color IDs in its colorId field. Read-only. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access color outputs
color_id = color.id
color_calendar = color.calendar
color_kind = color.kind
color_updated = color.updated
color_event = color.event
```

---


### Event

Creates an event.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `conference_data` | String |  | The conference-related information, such as details of a Google Meet conference. To create new conference details use the createRequest field. To persist your changes, remember to set the conferenceDataVersion request parameter to 1 for all event modification requests. |
| `private_copy` | bool |  | If set to True, Event propagation is disabled. Note that it is not the same thing as Private event properties. Optional. Immutable. The default is False. |
| `start` | String |  | The (inclusive) start time of the event. For a recurring event, this is the start time of the first instance. |
| `end` | String |  | The (exclusive) end time of the event. For a recurring event, this is the end time of the first instance. |
| `anyone_can_add_self` | bool |  | Whether anyone can invite themselves to the event (deprecated). Optional. The default is False. |
| `out_of_office_properties` | String |  | Out of office event data. Used if eventType is outOfOffice. |
| `location` | String |  | Geographic location of the event as free-form text. Optional. |
| `etag` | String |  | ETag of the resource. |
| `id` | String |  | Opaque identifier of the event. When creating new single or recurring events, you can specify their IDs. Provided IDs must follow these rules:  
- characters allowed in the ID are those used in base32hex encoding, i.e. lowercase letters a-v and digits 0-9, see section 3.1.2 in RFC2938 
- the length of the ID must be between 5 and 1024 characters 
- the ID must be unique per calendar  Due to the globally distributed nature of the system, we cannot guarantee that ID collisions will be detected at event creation time. To minimize the risk of collisions we recommend using an established UUID algorithm such as one described in RFC4122.
If you do not specify an ID, it will be automatically generated by the server.
Note that the icalUID and the id are not identical and only one of them should be supplied at event creation time. One difference in their semantics is that in recurring events, all occurrences of one event have different ids while they all share the same icalUIDs. |
| `gadget` | String |  | A gadget that extends this event. Gadgets are deprecated; this structure is instead only used for returning birthday calendar metadata. |
| `summary` | String |  | Title of the event. |
| `description` | String |  | Description of the event. Can contain HTML. Optional. |
| `i_cal_uid` | String |  | Event unique identifier as defined in RFC5545. It is used to uniquely identify events accross calendaring systems and must be supplied when importing events via the import method.
Note that the iCalUID and the id are not identical and only one of them should be supplied at event creation time. One difference in their semantics is that in recurring events, all occurrences of one event have different ids while they all share the same iCalUIDs. To retrieve an event using its iCalUID, call the events.list method using the iCalUID parameter. To retrieve an event using its id, call the events.get method. |
| `locked` | bool |  | Whether this is a locked event copy where no changes can be made to the main event fields "summary", "description", "location", "start", "end" or "recurrence". The default is False. Read-Only. |
| `reminders` | String |  | Information about the event's reminders for the authenticated user. Note that changing reminders does not also change the updated property of the enclosing event. |
| `guests_can_see_other_guests` | bool |  | Whether attendees other than the organizer can see who the event's attendees are. Optional. The default is True. |
| `attendees_omitted` | bool |  | Whether attendees may have been omitted from the event's representation. When retrieving an event, this may be due to a restriction specified by the maxAttendee query parameter. When updating an event, this can be used to only update the participant's response. Optional. The default is False. |
| `sequence` | i64 |  | Sequence number as per iCalendar. |
| `working_location_properties` | String |  | Working location event data. |
| `updated` | String |  | Last modification time of the main event data (as a RFC3339 timestamp). Updating event reminders will not cause this to change. Read-only. |
| `focus_time_properties` | String |  | Focus Time event data. Used if eventType is focusTime. |
| `html_link` | String |  | An absolute link to this event in the Google Calendar Web UI. Read-only. |
| `guests_can_invite_others` | bool |  | Whether attendees other than the organizer can invite others to the event. Optional. The default is True. |
| `hangout_link` | String |  | An absolute link to the Google Hangout associated with this event. Read-only. |
| `creator` | String |  | The creator of the event. Read-only. |
| `birthday_properties` | String |  | Birthday or special event data. Used if eventType is "birthday". Immutable. |
| `created` | String |  | Creation time of the event (as a RFC3339 timestamp). Read-only. |
| `event_type` | String |  | Specific type of the event. This cannot be modified after the event is created. Possible values are:  
- "birthday" - A special all-day event with an annual recurrence. 
- "default" - A regular event or not further specified. 
- "focusTime" - A focus-time event. 
- "fromGmail" - An event from Gmail. This type of event cannot be created. 
- "outOfOffice" - An out-of-office event. 
- "workingLocation" - A working location event. |
| `guests_can_modify` | bool |  | Whether attendees other than the organizer can modify the event. Optional. The default is False. |
| `attendees` | Vec<String> |  | The attendees of the event. See the Events with attendees guide for more information on scheduling events with other calendar users. Service accounts need to use domain-wide delegation of authority to populate the attendee list. |
| `attachments` | Vec<String> |  | File attachments for the event.
In order to modify attachments the supportsAttachments request parameter should be set to true.
There can be at most 25 attachments per event, |
| `color_id` | String |  | The color of the event. This is an ID referring to an entry in the event section of the colors definition (see the  colors endpoint). Optional. |
| `kind` | String |  | Type of the resource ("calendar#event"). |
| `end_time_unspecified` | bool |  | Whether the end time is actually unspecified. An end time is still provided for compatibility reasons, even if this attribute is set to True. The default is False. |
| `source` | String |  | Source from which the event was created. For example, a web page, an email message or any document identifiable by an URL with HTTP or HTTPS scheme. Can only be seen or modified by the creator of the event. |
| `status` | String |  | Status of the event. Optional. Possible values are:  
- "confirmed" - The event is confirmed. This is the default status. 
- "tentative" - The event is tentatively confirmed. 
- "cancelled" - The event is cancelled (deleted). The list method returns cancelled events only on incremental sync (when syncToken or updatedMin are specified) or if the showDeleted flag is set to true. The get method always returns them.
A cancelled status represents two different states depending on the event type:  
- Cancelled exceptions of an uncancelled recurring event indicate that this instance should no longer be presented to the user. Clients should store these events for the lifetime of the parent recurring event.
Cancelled exceptions are only guaranteed to have values for the id, recurringEventId and originalStartTime fields populated. The other fields might be empty.  
- All other cancelled events represent deleted events. Clients should remove their locally synced copies. Such cancelled events will eventually disappear, so do not rely on them being available indefinitely.
Deleted events are only guaranteed to have the id field populated.   On the organizer's calendar, cancelled events continue to expose event details (summary, location, etc.) so that they can be restored (undeleted). Similarly, the events to which the user was invited and that they manually removed continue to provide details. However, incremental sync requests with showDeleted set to false will not return these details.
If an event changes its organizer (for example via the move operation) and the original organizer is not on the attendee list, it will leave behind a cancelled event where only the id field is guaranteed to be populated. |
| `transparency` | String |  | Whether the event blocks time on the calendar. Optional. Possible values are:  
- "opaque" - Default value. The event does block time on the calendar. This is equivalent to setting Show me as to Busy in the Calendar UI. 
- "transparent" - The event does not block time on the calendar. This is equivalent to setting Show me as to Available in the Calendar UI. |
| `recurring_event_id` | String |  | For an instance of a recurring event, this is the id of the recurring event to which this instance belongs. Immutable. |
| `recurrence` | Vec<String> |  | List of RRULE, EXRULE, RDATE and EXDATE lines for a recurring event, as specified in RFC5545. Note that DTSTART and DTEND lines are not allowed in this field; event start and end times are specified in the start and end fields. This field is omitted for single events or instances of recurring events. |
| `extended_properties` | String |  | Extended properties of the event. |
| `organizer` | String |  | The organizer of the event. If the organizer is also an attendee, this is indicated with a separate entry in attendees with the organizer field set to True. To change the organizer, use the move operation. Read-only, except when importing an event. |
| `visibility` | String |  | Visibility of the event. Optional. Possible values are:  
- "default" - Uses the default visibility for events on the calendar. This is the default value. 
- "public" - The event is public and event details are visible to all readers of the calendar. 
- "private" - The event is private and only event attendees may view event details. 
- "confidential" - The event is private. This value is provided for compatibility reasons. |
| `original_start_time` | String |  | For an instance of a recurring event, this is the time at which this event would start according to the recurrence data in the recurring event identified by recurringEventId. It uniquely identifies the instance within the recurring event series even if the instance was moved to a different time. Immutable. |
| `calendar_id` | String | ✅ | Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `conference_data` | String | The conference-related information, such as details of a Google Meet conference. To create new conference details use the createRequest field. To persist your changes, remember to set the conferenceDataVersion request parameter to 1 for all event modification requests. |
| `private_copy` | bool | If set to True, Event propagation is disabled. Note that it is not the same thing as Private event properties. Optional. Immutable. The default is False. |
| `start` | String | The (inclusive) start time of the event. For a recurring event, this is the start time of the first instance. |
| `end` | String | The (exclusive) end time of the event. For a recurring event, this is the end time of the first instance. |
| `anyone_can_add_self` | bool | Whether anyone can invite themselves to the event (deprecated). Optional. The default is False. |
| `out_of_office_properties` | String | Out of office event data. Used if eventType is outOfOffice. |
| `location` | String | Geographic location of the event as free-form text. Optional. |
| `etag` | String | ETag of the resource. |
| `id` | String | Opaque identifier of the event. When creating new single or recurring events, you can specify their IDs. Provided IDs must follow these rules:  
- characters allowed in the ID are those used in base32hex encoding, i.e. lowercase letters a-v and digits 0-9, see section 3.1.2 in RFC2938 
- the length of the ID must be between 5 and 1024 characters 
- the ID must be unique per calendar  Due to the globally distributed nature of the system, we cannot guarantee that ID collisions will be detected at event creation time. To minimize the risk of collisions we recommend using an established UUID algorithm such as one described in RFC4122.
If you do not specify an ID, it will be automatically generated by the server.
Note that the icalUID and the id are not identical and only one of them should be supplied at event creation time. One difference in their semantics is that in recurring events, all occurrences of one event have different ids while they all share the same icalUIDs. |
| `gadget` | String | A gadget that extends this event. Gadgets are deprecated; this structure is instead only used for returning birthday calendar metadata. |
| `summary` | String | Title of the event. |
| `description` | String | Description of the event. Can contain HTML. Optional. |
| `i_cal_uid` | String | Event unique identifier as defined in RFC5545. It is used to uniquely identify events accross calendaring systems and must be supplied when importing events via the import method.
Note that the iCalUID and the id are not identical and only one of them should be supplied at event creation time. One difference in their semantics is that in recurring events, all occurrences of one event have different ids while they all share the same iCalUIDs. To retrieve an event using its iCalUID, call the events.list method using the iCalUID parameter. To retrieve an event using its id, call the events.get method. |
| `locked` | bool | Whether this is a locked event copy where no changes can be made to the main event fields "summary", "description", "location", "start", "end" or "recurrence". The default is False. Read-Only. |
| `reminders` | String | Information about the event's reminders for the authenticated user. Note that changing reminders does not also change the updated property of the enclosing event. |
| `guests_can_see_other_guests` | bool | Whether attendees other than the organizer can see who the event's attendees are. Optional. The default is True. |
| `attendees_omitted` | bool | Whether attendees may have been omitted from the event's representation. When retrieving an event, this may be due to a restriction specified by the maxAttendee query parameter. When updating an event, this can be used to only update the participant's response. Optional. The default is False. |
| `sequence` | i64 | Sequence number as per iCalendar. |
| `working_location_properties` | String | Working location event data. |
| `updated` | String | Last modification time of the main event data (as a RFC3339 timestamp). Updating event reminders will not cause this to change. Read-only. |
| `focus_time_properties` | String | Focus Time event data. Used if eventType is focusTime. |
| `html_link` | String | An absolute link to this event in the Google Calendar Web UI. Read-only. |
| `guests_can_invite_others` | bool | Whether attendees other than the organizer can invite others to the event. Optional. The default is True. |
| `hangout_link` | String | An absolute link to the Google Hangout associated with this event. Read-only. |
| `creator` | String | The creator of the event. Read-only. |
| `birthday_properties` | String | Birthday or special event data. Used if eventType is "birthday". Immutable. |
| `created` | String | Creation time of the event (as a RFC3339 timestamp). Read-only. |
| `event_type` | String | Specific type of the event. This cannot be modified after the event is created. Possible values are:  
- "birthday" - A special all-day event with an annual recurrence. 
- "default" - A regular event or not further specified. 
- "focusTime" - A focus-time event. 
- "fromGmail" - An event from Gmail. This type of event cannot be created. 
- "outOfOffice" - An out-of-office event. 
- "workingLocation" - A working location event. |
| `guests_can_modify` | bool | Whether attendees other than the organizer can modify the event. Optional. The default is False. |
| `attendees` | Vec<String> | The attendees of the event. See the Events with attendees guide for more information on scheduling events with other calendar users. Service accounts need to use domain-wide delegation of authority to populate the attendee list. |
| `attachments` | Vec<String> | File attachments for the event.
In order to modify attachments the supportsAttachments request parameter should be set to true.
There can be at most 25 attachments per event, |
| `color_id` | String | The color of the event. This is an ID referring to an entry in the event section of the colors definition (see the  colors endpoint). Optional. |
| `kind` | String | Type of the resource ("calendar#event"). |
| `end_time_unspecified` | bool | Whether the end time is actually unspecified. An end time is still provided for compatibility reasons, even if this attribute is set to True. The default is False. |
| `source` | String | Source from which the event was created. For example, a web page, an email message or any document identifiable by an URL with HTTP or HTTPS scheme. Can only be seen or modified by the creator of the event. |
| `status` | String | Status of the event. Optional. Possible values are:  
- "confirmed" - The event is confirmed. This is the default status. 
- "tentative" - The event is tentatively confirmed. 
- "cancelled" - The event is cancelled (deleted). The list method returns cancelled events only on incremental sync (when syncToken or updatedMin are specified) or if the showDeleted flag is set to true. The get method always returns them.
A cancelled status represents two different states depending on the event type:  
- Cancelled exceptions of an uncancelled recurring event indicate that this instance should no longer be presented to the user. Clients should store these events for the lifetime of the parent recurring event.
Cancelled exceptions are only guaranteed to have values for the id, recurringEventId and originalStartTime fields populated. The other fields might be empty.  
- All other cancelled events represent deleted events. Clients should remove their locally synced copies. Such cancelled events will eventually disappear, so do not rely on them being available indefinitely.
Deleted events are only guaranteed to have the id field populated.   On the organizer's calendar, cancelled events continue to expose event details (summary, location, etc.) so that they can be restored (undeleted). Similarly, the events to which the user was invited and that they manually removed continue to provide details. However, incremental sync requests with showDeleted set to false will not return these details.
If an event changes its organizer (for example via the move operation) and the original organizer is not on the attendee list, it will leave behind a cancelled event where only the id field is guaranteed to be populated. |
| `transparency` | String | Whether the event blocks time on the calendar. Optional. Possible values are:  
- "opaque" - Default value. The event does block time on the calendar. This is equivalent to setting Show me as to Busy in the Calendar UI. 
- "transparent" - The event does not block time on the calendar. This is equivalent to setting Show me as to Available in the Calendar UI. |
| `recurring_event_id` | String | For an instance of a recurring event, this is the id of the recurring event to which this instance belongs. Immutable. |
| `recurrence` | Vec<String> | List of RRULE, EXRULE, RDATE and EXDATE lines for a recurring event, as specified in RFC5545. Note that DTSTART and DTEND lines are not allowed in this field; event start and end times are specified in the start and end fields. This field is omitted for single events or instances of recurring events. |
| `extended_properties` | String | Extended properties of the event. |
| `organizer` | String | The organizer of the event. If the organizer is also an attendee, this is indicated with a separate entry in attendees with the organizer field set to True. To change the organizer, use the move operation. Read-only, except when importing an event. |
| `visibility` | String | Visibility of the event. Optional. Possible values are:  
- "default" - Uses the default visibility for events on the calendar. This is the default value. 
- "public" - The event is public and event details are visible to all readers of the calendar. 
- "private" - The event is private and only event attendees may view event details. 
- "confidential" - The event is private. This value is provided for compatibility reasons. |
| `original_start_time` | String | For an instance of a recurring event, this is the time at which this event would start according to the recurrence data in the recurring event identified by recurringEventId. It uniquely identifies the instance within the recurring event series even if the instance was moved to a different time. Immutable. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create event
event = provider.calendar_api.Event {
    calendar_id = "value"  # Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
}

# Access event outputs
event_id = event.id
event_conference_data = event.conference_data
event_private_copy = event.private_copy
event_start = event.start
event_end = event.end
event_anyone_can_add_self = event.anyone_can_add_self
event_out_of_office_properties = event.out_of_office_properties
event_location = event.location
event_etag = event.etag
event_id = event.id
event_gadget = event.gadget
event_summary = event.summary
event_description = event.description
event_i_cal_uid = event.i_cal_uid
event_locked = event.locked
event_reminders = event.reminders
event_guests_can_see_other_guests = event.guests_can_see_other_guests
event_attendees_omitted = event.attendees_omitted
event_sequence = event.sequence
event_working_location_properties = event.working_location_properties
event_updated = event.updated
event_focus_time_properties = event.focus_time_properties
event_html_link = event.html_link
event_guests_can_invite_others = event.guests_can_invite_others
event_hangout_link = event.hangout_link
event_creator = event.creator
event_birthday_properties = event.birthday_properties
event_created = event.created
event_event_type = event.event_type
event_guests_can_modify = event.guests_can_modify
event_attendees = event.attendees
event_attachments = event.attachments
event_color_id = event.color_id
event_kind = event.kind
event_end_time_unspecified = event.end_time_unspecified
event_source = event.source
event_status = event.status
event_transparency = event.transparency
event_recurring_event_id = event.recurring_event_id
event_recurrence = event.recurrence
event_extended_properties = event.extended_properties
event_organizer = event.organizer
event_visibility = event.visibility
event_original_start_time = event.original_start_time
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple setting resources
setting_0 = provider.calendar_api.Setting {
}
setting_1 = provider.calendar_api.Setting {
}
setting_2 = provider.calendar_api.Setting {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    setting = provider.calendar_api.Setting {
    }
```

---

## Related Documentation

- [GCP Calendar_api Documentation](https://cloud.google.com/calendar_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
