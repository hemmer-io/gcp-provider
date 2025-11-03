# Meet_api Service



**Resources**: 7

---

## Overview

The meet_api service provides access to 7 resource types:

- [Recording](#recording) [R]
- [Transcript](#transcript) [R]
- [Participant](#participant) [R]
- [Entrie](#entrie) [R]
- [Conference_record](#conference_record) [R]
- [Participant_session](#participant_session) [R]
- [Space](#space) [CRU]

---

## Resources


### Recording

Gets a recording by recording ID.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `drive_destination` | String | Output only. Recording is saved to Google Drive as an MP4 file. The `drive_destination` includes the Drive `fileId` that can be used to download the file using the `files.get` method of the Drive API. |
| `start_time` | String | Output only. Timestamp when the recording started. |
| `state` | String | Output only. Current state. |
| `end_time` | String | Output only. Timestamp when the recording ended. |
| `name` | String | Output only. Resource name of the recording. Format: `conferenceRecords/{conference_record}/recordings/{recording}` where `{recording}` is a 1:1 mapping to each unique recording session during the conference. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access recording outputs
recording_id = recording.id
recording_drive_destination = recording.drive_destination
recording_start_time = recording.start_time
recording_state = recording.state
recording_end_time = recording.end_time
recording_name = recording.name
```

---


### Transcript

Gets a transcript by transcript ID.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String | Output only. Current state. |
| `name` | String | Output only. Resource name of the transcript. Format: `conferenceRecords/{conference_record}/transcripts/{transcript}`, where `{transcript}` is a 1:1 mapping to each unique transcription session of the conference. |
| `docs_destination` | String | Output only. Where the Google Docs transcript is saved. |
| `start_time` | String | Output only. Timestamp when the transcript started. |
| `end_time` | String | Output only. Timestamp when the transcript stopped. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access transcript outputs
transcript_id = transcript.id
transcript_state = transcript.state
transcript_name = transcript.name
transcript_docs_destination = transcript.docs_destination
transcript_start_time = transcript.start_time
transcript_end_time = transcript.end_time
```

---


### Participant

Gets a participant by participant ID.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `signedin_user` | String | Signed-in user. |
| `anonymous_user` | String | Anonymous user. |
| `latest_end_time` | String | Output only. Time when the participant left the meeting for the last time. This can be null if it's an active meeting. |
| `earliest_start_time` | String | Output only. Time when the participant first joined the meeting. |
| `phone_user` | String | User calling from their phone. |
| `name` | String | Output only. Resource name of the participant. Format: `conferenceRecords/{conference_record}/participants/{participant}` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access participant outputs
participant_id = participant.id
participant_signedin_user = participant.signedin_user
participant_anonymous_user = participant.anonymous_user
participant_latest_end_time = participant.latest_end_time
participant_earliest_start_time = participant.earliest_start_time
participant_phone_user = participant.phone_user
participant_name = participant.name
```

---


### Entrie

Gets a `TranscriptEntry` resource by entry ID. Note: The transcript entries returned by the Google Meet API might not match the transcription found in the Google Docs transcript file. This can occur when 1) we have interleaved speakers within milliseconds, or 2) the Google Docs transcript file is modified after generation.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `start_time` | String | Output only. Timestamp when the transcript entry started. |
| `participant` | String | Output only. Refers to the participant who speaks. |
| `text` | String | Output only. The transcribed text of the participant's voice, at maximum 10K words. Note that the limit is subject to change. |
| `language_code` | String | Output only. Language of spoken text, such as "en-US". IETF BCP 47 syntax (https://tools.ietf.org/html/bcp47) |
| `end_time` | String | Output only. Timestamp when the transcript entry ended. |
| `name` | String | Output only. Resource name of the entry. Format: "conferenceRecords/{conference_record}/transcripts/{transcript}/entries/{entry}" |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access entrie outputs
entrie_id = entrie.id
entrie_start_time = entrie.start_time
entrie_participant = entrie.participant
entrie_text = entrie.text
entrie_language_code = entrie.language_code
entrie_end_time = entrie.end_time
entrie_name = entrie.name
```

---


### Conference_record

Gets a conference record by conference ID.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `end_time` | String | Output only. Timestamp when the conference ended. Set for past conferences. Unset if the conference is ongoing. |
| `expire_time` | String | Output only. Server enforced expiration time for when this conference record resource is deleted. The resource is deleted 30 days after the conference ends. |
| `space` | String | Output only. The space where the conference was held. |
| `name` | String | Identifier. Resource name of the conference record. Format: `conferenceRecords/{conference_record}` where `{conference_record}` is a unique ID for each instance of a call within a space. |
| `start_time` | String | Output only. Timestamp when the conference started. Always set. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access conference_record outputs
conference_record_id = conference_record.id
conference_record_end_time = conference_record.end_time
conference_record_expire_time = conference_record.expire_time
conference_record_space = conference_record.space
conference_record_name = conference_record.name
conference_record_start_time = conference_record.start_time
```

---


### Participant_session

Gets a participant session by participant session ID.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `start_time` | String | Output only. Timestamp when the user session starts. |
| `end_time` | String | Output only. Timestamp when the user session ends. Unset if the user session hasn’t ended. |
| `name` | String | Identifier. Session id. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access participant_session outputs
participant_session_id = participant_session.id
participant_session_start_time = participant_session.start_time
participant_session_end_time = participant_session.end_time
participant_session_name = participant_session.name
```

---


### Space

Creates a space.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Immutable. Resource name of the space. Format: `spaces/{space}`. `{space}` is the resource identifier for the space. It's a unique, server-generated ID and is case sensitive. For example, `jQCFfuBOdN5z`. For more information, see [How Meet identifies a meeting space](https://developers.google.com/workspace/meet/api/guides/meeting-spaces#identify-meeting-space). |
| `config` | String |  | Configuration pertaining to the meeting space. |
| `meeting_uri` | String |  | Output only. URI used to join meetings consisting of `https://meet.google.com/` followed by the `meeting_code`. For example, `https://meet.google.com/abc-mnop-xyz`. |
| `meeting_code` | String |  | Output only. Type friendly unique string used to join the meeting. Format: `[a-z]+-[a-z]+-[a-z]+`. For example, `abc-mnop-xyz`. The maximum length is 128 characters. Can only be used as an alias of the space name to get the space. |
| `active_conference` | String |  | Active conference, if it exists. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Immutable. Resource name of the space. Format: `spaces/{space}`. `{space}` is the resource identifier for the space. It's a unique, server-generated ID and is case sensitive. For example, `jQCFfuBOdN5z`. For more information, see [How Meet identifies a meeting space](https://developers.google.com/workspace/meet/api/guides/meeting-spaces#identify-meeting-space). |
| `config` | String | Configuration pertaining to the meeting space. |
| `meeting_uri` | String | Output only. URI used to join meetings consisting of `https://meet.google.com/` followed by the `meeting_code`. For example, `https://meet.google.com/abc-mnop-xyz`. |
| `meeting_code` | String | Output only. Type friendly unique string used to join the meeting. Format: `[a-z]+-[a-z]+-[a-z]+`. For example, `abc-mnop-xyz`. The maximum length is 128 characters. Can only be used as an alias of the space name to get the space. |
| `active_conference` | String | Active conference, if it exists. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create space
space = provider.meet_api.Space {
}

# Access space outputs
space_id = space.id
space_name = space.name
space_config = space.config
space_meeting_uri = space.meeting_uri
space_meeting_code = space.meeting_code
space_active_conference = space.active_conference
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple recording resources
recording_0 = provider.meet_api.Recording {
}
recording_1 = provider.meet_api.Recording {
}
recording_2 = provider.meet_api.Recording {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    recording = provider.meet_api.Recording {
    }
```

---

## Related Documentation

- [GCP Meet_api Documentation](https://cloud.google.com/meet_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
