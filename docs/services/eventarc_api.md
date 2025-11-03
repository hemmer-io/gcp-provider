# Eventarc_api Service



**Resources**: 14

---

## Overview

The eventarc_api service provides access to 14 resource types:

- [Location](#location) [RU]
- [Enrollment](#enrollment) [CRUD]
- [Google_api_source](#google_api_source) [CRUD]
- [Channel](#channel) [CRUD]
- [Message_buse](#message_buse) [CRUD]
- [Pipeline](#pipeline) [CRUD]
- [Channel_connection](#channel_connection) [CRD]
- [Provider](#provider) [R]
- [Operation](#operation) [CRD]
- [Trigger](#trigger) [CRUD]
- [Kafka_source](#kafka_source) [CR]
- [Operation](#operation) [CRD]
- [Trigger](#trigger) [CRUD]
- [Location](#location) [R]

---

## Resources


### Location

Gets information about a location.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Required. The resource name of the config. Must be in the format of, `projects/{project}/locations/{location}/googleChannelConfig`. In API responses, the config name always includes the projectID, regardless of whether the projectID or projectNumber was provided. |
| `crypto_key_name` | String |  | Optional. Resource name of a KMS crypto key (managed by the user) used to encrypt/decrypt their event data. It must match the pattern `projects/*/locations/*/keyRings/*/cryptoKeys/*`. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels. |
| `update_time` | String |  | Output only. The last-modified time. |
| `name` | String | ✅ | Required. The resource name of the config. Must be in the format of, `projects/{project}/locations/{location}/googleChannelConfig`. In API responses, the config name always includes the projectID, regardless of whether the projectID or projectNumber was provided. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access location outputs
location_id = location.id
location_name = location.name
location_display_name = location.display_name
location_labels = location.labels
location_location_id = location.location_id
location_metadata = location.metadata
```

---


### Enrollment

Create a new Enrollment in a particular project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `destination` | String |  | Required. Destination is the Pipeline that the Enrollment is delivering to. It must point to the full resource name of a Pipeline. Format: "projects/{PROJECT_ID}/locations/{region}/pipelines/{PIPELINE_ID)" |
| `annotations` | HashMap<String, String> |  | Optional. Resource annotations. |
| `message_bus` | String |  | Required. Immutable. Resource name of the message bus identifying the source of the messages. It matches the form projects/{project}/locations/{location}/messageBuses/{messageBus}. |
| `name` | String |  | Identifier. Resource name of the form projects/{project}/locations/{location}/enrollments/{enrollment} |
| `create_time` | String |  | Output only. The creation time. |
| `display_name` | String |  | Optional. Resource display name. |
| `update_time` | String |  | Output only. The last-modified time. |
| `uid` | String |  | Output only. Server assigned unique identifier for the channel. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels. |
| `etag` | String |  | Output only. This checksum is computed by the server based on the value of other fields, and might be sent only on update and delete requests to ensure that the client has an up-to-date value before proceeding. |
| `cel_match` | String |  | Required. A CEL expression identifying which messages this enrollment applies to. |
| `parent` | String | ✅ | Required. The parent collection in which to add this enrollment. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `destination` | String | Required. Destination is the Pipeline that the Enrollment is delivering to. It must point to the full resource name of a Pipeline. Format: "projects/{PROJECT_ID}/locations/{region}/pipelines/{PIPELINE_ID)" |
| `annotations` | HashMap<String, String> | Optional. Resource annotations. |
| `message_bus` | String | Required. Immutable. Resource name of the message bus identifying the source of the messages. It matches the form projects/{project}/locations/{location}/messageBuses/{messageBus}. |
| `name` | String | Identifier. Resource name of the form projects/{project}/locations/{location}/enrollments/{enrollment} |
| `create_time` | String | Output only. The creation time. |
| `display_name` | String | Optional. Resource display name. |
| `update_time` | String | Output only. The last-modified time. |
| `uid` | String | Output only. Server assigned unique identifier for the channel. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted. |
| `labels` | HashMap<String, String> | Optional. Resource labels. |
| `etag` | String | Output only. This checksum is computed by the server based on the value of other fields, and might be sent only on update and delete requests to ensure that the client has an up-to-date value before proceeding. |
| `cel_match` | String | Required. A CEL expression identifying which messages this enrollment applies to. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create enrollment
enrollment = provider.eventarc_api.Enrollment {
    parent = "value"  # Required. The parent collection in which to add this enrollment.
}

# Access enrollment outputs
enrollment_id = enrollment.id
enrollment_destination = enrollment.destination
enrollment_annotations = enrollment.annotations
enrollment_message_bus = enrollment.message_bus
enrollment_name = enrollment.name
enrollment_create_time = enrollment.create_time
enrollment_display_name = enrollment.display_name
enrollment_update_time = enrollment.update_time
enrollment_uid = enrollment.uid
enrollment_labels = enrollment.labels
enrollment_etag = enrollment.etag
enrollment_cel_match = enrollment.cel_match
```

---


### Google_api_source

Create a new GoogleApiSource in a particular project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `destination` | String |  | Required. Destination is the message bus that the GoogleApiSource is delivering to. It must be point to the full resource name of a MessageBus. Format: "projects/{PROJECT_ID}/locations/{region}/messagesBuses/{MESSAGE_BUS_ID) |
| `create_time` | String |  | Output only. The creation time. |
| `uid` | String |  | Output only. Server assigned unique identifier for the channel. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted. |
| `display_name` | String |  | Optional. Resource display name. |
| `etag` | String |  | Output only. This checksum is computed by the server based on the value of other fields, and might be sent only on update and delete requests to ensure that the client has an up-to-date value before proceeding. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels. |
| `update_time` | String |  | Output only. The last-modified time. |
| `annotations` | HashMap<String, String> |  | Optional. Resource annotations. |
| `logging_config` | String |  | Optional. Config to control Platform logging for the GoogleApiSource. |
| `project_subscriptions` | String |  | Optional. Config to enable subscribing to all events from a list of projects. All the projects must be in the same org as the GoogleApiSource. |
| `name` | String |  | Identifier. Resource name of the form projects/{project}/locations/{location}/googleApiSources/{google_api_source} |
| `crypto_key_name` | String |  | Optional. Resource name of a KMS crypto key (managed by the user) used to encrypt/decrypt their event data. It must match the pattern `projects/*/locations/*/keyRings/*/cryptoKeys/*`. |
| `organization_subscription` | String |  | Optional. Config to enable subscribing to events from all projects in the GoogleApiSource's org. |
| `parent` | String | ✅ | Required. The parent collection in which to add this google api source. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `destination` | String | Required. Destination is the message bus that the GoogleApiSource is delivering to. It must be point to the full resource name of a MessageBus. Format: "projects/{PROJECT_ID}/locations/{region}/messagesBuses/{MESSAGE_BUS_ID) |
| `create_time` | String | Output only. The creation time. |
| `uid` | String | Output only. Server assigned unique identifier for the channel. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted. |
| `display_name` | String | Optional. Resource display name. |
| `etag` | String | Output only. This checksum is computed by the server based on the value of other fields, and might be sent only on update and delete requests to ensure that the client has an up-to-date value before proceeding. |
| `labels` | HashMap<String, String> | Optional. Resource labels. |
| `update_time` | String | Output only. The last-modified time. |
| `annotations` | HashMap<String, String> | Optional. Resource annotations. |
| `logging_config` | String | Optional. Config to control Platform logging for the GoogleApiSource. |
| `project_subscriptions` | String | Optional. Config to enable subscribing to all events from a list of projects. All the projects must be in the same org as the GoogleApiSource. |
| `name` | String | Identifier. Resource name of the form projects/{project}/locations/{location}/googleApiSources/{google_api_source} |
| `crypto_key_name` | String | Optional. Resource name of a KMS crypto key (managed by the user) used to encrypt/decrypt their event data. It must match the pattern `projects/*/locations/*/keyRings/*/cryptoKeys/*`. |
| `organization_subscription` | String | Optional. Config to enable subscribing to events from all projects in the GoogleApiSource's org. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create google_api_source
google_api_source = provider.eventarc_api.Google_api_source {
    parent = "value"  # Required. The parent collection in which to add this google api source.
}

# Access google_api_source outputs
google_api_source_id = google_api_source.id
google_api_source_destination = google_api_source.destination
google_api_source_create_time = google_api_source.create_time
google_api_source_uid = google_api_source.uid
google_api_source_display_name = google_api_source.display_name
google_api_source_etag = google_api_source.etag
google_api_source_labels = google_api_source.labels
google_api_source_update_time = google_api_source.update_time
google_api_source_annotations = google_api_source.annotations
google_api_source_logging_config = google_api_source.logging_config
google_api_source_project_subscriptions = google_api_source.project_subscriptions
google_api_source_name = google_api_source.name
google_api_source_crypto_key_name = google_api_source.crypto_key_name
google_api_source_organization_subscription = google_api_source.organization_subscription
```

---


### Channel

Create a new channel in a particular project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `activation_token` | String |  | Output only. The activation token for the channel. The token must be used by the provider to register the channel for publishing. |
| `provider` | String |  | The name of the event provider (e.g. Eventarc SaaS partner) associated with the channel. This provider will be granted permissions to publish events to the channel. Format: `projects/{project}/locations/{location}/providers/{provider_id}`. |
| `satisfies_pzs` | bool |  | Output only. Whether or not this Channel satisfies the requirements of physical zone separation |
| `state` | String |  | Output only. The state of a Channel. |
| `uid` | String |  | Output only. Server assigned unique identifier for the channel. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels. |
| `update_time` | String |  | Output only. The last-modified time. |
| `create_time` | String |  | Output only. The creation time. |
| `crypto_key_name` | String |  | Optional. Resource name of a KMS crypto key (managed by the user) used to encrypt/decrypt their event data. It must match the pattern `projects/*/locations/*/keyRings/*/cryptoKeys/*`. |
| `name` | String |  | Required. The resource name of the channel. Must be unique within the location on the project and must be in `projects/{project}/locations/{location}/channels/{channel_id}` format. |
| `pubsub_topic` | String |  | Output only. The name of the Pub/Sub topic created and managed by Eventarc system as a transport for the event delivery. Format: `projects/{project}/topics/{topic_id}`. |
| `parent` | String | ✅ | Required. The parent collection in which to add this channel. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `activation_token` | String | Output only. The activation token for the channel. The token must be used by the provider to register the channel for publishing. |
| `provider` | String | The name of the event provider (e.g. Eventarc SaaS partner) associated with the channel. This provider will be granted permissions to publish events to the channel. Format: `projects/{project}/locations/{location}/providers/{provider_id}`. |
| `satisfies_pzs` | bool | Output only. Whether or not this Channel satisfies the requirements of physical zone separation |
| `state` | String | Output only. The state of a Channel. |
| `uid` | String | Output only. Server assigned unique identifier for the channel. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted. |
| `labels` | HashMap<String, String> | Optional. Resource labels. |
| `update_time` | String | Output only. The last-modified time. |
| `create_time` | String | Output only. The creation time. |
| `crypto_key_name` | String | Optional. Resource name of a KMS crypto key (managed by the user) used to encrypt/decrypt their event data. It must match the pattern `projects/*/locations/*/keyRings/*/cryptoKeys/*`. |
| `name` | String | Required. The resource name of the channel. Must be unique within the location on the project and must be in `projects/{project}/locations/{location}/channels/{channel_id}` format. |
| `pubsub_topic` | String | Output only. The name of the Pub/Sub topic created and managed by Eventarc system as a transport for the event delivery. Format: `projects/{project}/topics/{topic_id}`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create channel
channel = provider.eventarc_api.Channel {
    parent = "value"  # Required. The parent collection in which to add this channel.
}

# Access channel outputs
channel_id = channel.id
channel_activation_token = channel.activation_token
channel_provider = channel.provider
channel_satisfies_pzs = channel.satisfies_pzs
channel_state = channel.state
channel_uid = channel.uid
channel_labels = channel.labels
channel_update_time = channel.update_time
channel_create_time = channel.create_time
channel_crypto_key_name = channel.crypto_key_name
channel_name = channel.name
channel_pubsub_topic = channel.pubsub_topic
```

---


### Message_buse

Create a new MessageBus in a particular project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `crypto_key_name` | String |  | Optional. Resource name of a KMS crypto key (managed by the user) used to encrypt/decrypt their event data. It must match the pattern `projects/*/locations/*/keyRings/*/cryptoKeys/*`. |
| `labels` | HashMap<String, String> |  | Optional. Resource labels. |
| `logging_config` | String |  | Optional. Config to control Platform logging for the Message Bus. This log configuration is applied to the Message Bus itself, and all the Enrollments attached to it. |
| `etag` | String |  | Output only. This checksum is computed by the server based on the value of other fields, and might be sent only on update and delete requests to ensure that the client has an up-to-date value before proceeding. |
| `annotations` | HashMap<String, String> |  | Optional. Resource annotations. |
| `uid` | String |  | Output only. Server assigned unique identifier for the channel. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted. |
| `update_time` | String |  | Output only. The last-modified time. |
| `create_time` | String |  | Output only. The creation time. |
| `display_name` | String |  | Optional. Resource display name. |
| `name` | String |  | Identifier. Resource name of the form projects/{project}/locations/{location}/messageBuses/{message_bus} |
| `parent` | String | ✅ | Required. The parent collection in which to add this message bus. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `crypto_key_name` | String | Optional. Resource name of a KMS crypto key (managed by the user) used to encrypt/decrypt their event data. It must match the pattern `projects/*/locations/*/keyRings/*/cryptoKeys/*`. |
| `labels` | HashMap<String, String> | Optional. Resource labels. |
| `logging_config` | String | Optional. Config to control Platform logging for the Message Bus. This log configuration is applied to the Message Bus itself, and all the Enrollments attached to it. |
| `etag` | String | Output only. This checksum is computed by the server based on the value of other fields, and might be sent only on update and delete requests to ensure that the client has an up-to-date value before proceeding. |
| `annotations` | HashMap<String, String> | Optional. Resource annotations. |
| `uid` | String | Output only. Server assigned unique identifier for the channel. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted. |
| `update_time` | String | Output only. The last-modified time. |
| `create_time` | String | Output only. The creation time. |
| `display_name` | String | Optional. Resource display name. |
| `name` | String | Identifier. Resource name of the form projects/{project}/locations/{location}/messageBuses/{message_bus} |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create message_buse
message_buse = provider.eventarc_api.Message_buse {
    parent = "value"  # Required. The parent collection in which to add this message bus.
}

# Access message_buse outputs
message_buse_id = message_buse.id
message_buse_crypto_key_name = message_buse.crypto_key_name
message_buse_labels = message_buse.labels
message_buse_logging_config = message_buse.logging_config
message_buse_etag = message_buse.etag
message_buse_annotations = message_buse.annotations
message_buse_uid = message_buse.uid
message_buse_update_time = message_buse.update_time
message_buse_create_time = message_buse.create_time
message_buse_display_name = message_buse.display_name
message_buse_name = message_buse.name
```

---


### Pipeline

Create a new Pipeline in a particular project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `annotations` | HashMap<String, String> |  | Optional. User-defined annotations. See https://google.aip.dev/128#annotations. |
| `mediations` | Vec<String> |  | Optional. List of mediation operations to be performed on the message. Currently, only one Transformation operation is allowed in each Pipeline. |
| `uid` | String |  | Output only. Server-assigned unique identifier for the Pipeline. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted. |
| `update_time` | String |  | Output only. The last-modified time. A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z". |
| `destinations` | Vec<String> |  | Required. List of destinations to which messages will be forwarded. Currently, exactly one destination is supported per Pipeline. |
| `name` | String |  | Identifier. The resource name of the Pipeline. Must be unique within the location of the project and must be in `projects/{project}/locations/{location}/pipelines/{pipeline}` format. |
| `create_time` | String |  | Output only. The creation time. A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z". |
| `labels` | HashMap<String, String> |  | Optional. User labels attached to the Pipeline that can be used to group resources. An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }. |
| `crypto_key_name` | String |  | Optional. Resource name of a KMS crypto key (managed by the user) used to encrypt/decrypt the event data. If not set, an internal Google-owned key will be used to encrypt messages. It must match the pattern "projects/{project}/locations/{location}/keyRings/{keyring}/cryptoKeys/{key}". |
| `display_name` | String |  | Optional. Display name of resource. |
| `logging_config` | String |  | Optional. Config to control Platform Logging for Pipelines. |
| `satisfies_pzs` | bool |  | Output only. Whether or not this Pipeline satisfies the requirements of physical zone separation |
| `etag` | String |  | Output only. This checksum is computed by the server based on the value of other fields, and might be sent only on create requests to ensure that the client has an up-to-date value before proceeding. |
| `input_payload_format` | String |  | Optional. The payload format expected for the messages received by the Pipeline. If input_payload_format is set then any messages not matching this format will be treated as persistent errors. If input_payload_format is not set, then the message data will be treated as an opaque binary and no output format can be set on the Pipeline through the Pipeline.Destination.output_payload_format field. Any Mediations on the Pipeline that involve access to the data field will fail as persistent errors. |
| `retry_policy` | String |  | Optional. The retry policy to use in the pipeline. |
| `parent` | String | ✅ | Required. The parent collection in which to add this pipeline. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `annotations` | HashMap<String, String> | Optional. User-defined annotations. See https://google.aip.dev/128#annotations. |
| `mediations` | Vec<String> | Optional. List of mediation operations to be performed on the message. Currently, only one Transformation operation is allowed in each Pipeline. |
| `uid` | String | Output only. Server-assigned unique identifier for the Pipeline. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted. |
| `update_time` | String | Output only. The last-modified time. A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z". |
| `destinations` | Vec<String> | Required. List of destinations to which messages will be forwarded. Currently, exactly one destination is supported per Pipeline. |
| `name` | String | Identifier. The resource name of the Pipeline. Must be unique within the location of the project and must be in `projects/{project}/locations/{location}/pipelines/{pipeline}` format. |
| `create_time` | String | Output only. The creation time. A timestamp in RFC3339 UTC "Zulu" format, with nanosecond resolution and up to nine fractional digits. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z". |
| `labels` | HashMap<String, String> | Optional. User labels attached to the Pipeline that can be used to group resources. An object containing a list of "key": value pairs. Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }. |
| `crypto_key_name` | String | Optional. Resource name of a KMS crypto key (managed by the user) used to encrypt/decrypt the event data. If not set, an internal Google-owned key will be used to encrypt messages. It must match the pattern "projects/{project}/locations/{location}/keyRings/{keyring}/cryptoKeys/{key}". |
| `display_name` | String | Optional. Display name of resource. |
| `logging_config` | String | Optional. Config to control Platform Logging for Pipelines. |
| `satisfies_pzs` | bool | Output only. Whether or not this Pipeline satisfies the requirements of physical zone separation |
| `etag` | String | Output only. This checksum is computed by the server based on the value of other fields, and might be sent only on create requests to ensure that the client has an up-to-date value before proceeding. |
| `input_payload_format` | String | Optional. The payload format expected for the messages received by the Pipeline. If input_payload_format is set then any messages not matching this format will be treated as persistent errors. If input_payload_format is not set, then the message data will be treated as an opaque binary and no output format can be set on the Pipeline through the Pipeline.Destination.output_payload_format field. Any Mediations on the Pipeline that involve access to the data field will fail as persistent errors. |
| `retry_policy` | String | Optional. The retry policy to use in the pipeline. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create pipeline
pipeline = provider.eventarc_api.Pipeline {
    parent = "value"  # Required. The parent collection in which to add this pipeline.
}

# Access pipeline outputs
pipeline_id = pipeline.id
pipeline_annotations = pipeline.annotations
pipeline_mediations = pipeline.mediations
pipeline_uid = pipeline.uid
pipeline_update_time = pipeline.update_time
pipeline_destinations = pipeline.destinations
pipeline_name = pipeline.name
pipeline_create_time = pipeline.create_time
pipeline_labels = pipeline.labels
pipeline_crypto_key_name = pipeline.crypto_key_name
pipeline_display_name = pipeline.display_name
pipeline_logging_config = pipeline.logging_config
pipeline_satisfies_pzs = pipeline.satisfies_pzs
pipeline_etag = pipeline.etag
pipeline_input_payload_format = pipeline.input_payload_format
pipeline_retry_policy = pipeline.retry_policy
```

---


### Channel_connection

Create a new ChannelConnection in a particular project and location.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `labels` | HashMap<String, String> |  | Optional. Resource labels. |
| `uid` | String |  | Output only. Server assigned ID of the resource. The server guarantees uniqueness and immutability until deleted. |
| `update_time` | String |  | Output only. The last-modified time. |
| `create_time` | String |  | Output only. The creation time. |
| `channel` | String |  | Required. The name of the connected subscriber Channel. This is a weak reference to avoid cross project and cross accounts references. This must be in `projects/{project}/location/{location}/channels/{channel_id}` format. |
| `name` | String |  | Required. The name of the connection. |
| `activation_token` | String |  | Input only. Activation token for the channel. The token will be used during the creation of ChannelConnection to bind the channel with the provider project. This field will not be stored in the provider resource. |
| `parent` | String | ✅ | Required. The parent collection in which to add this channel connection. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Optional. Resource labels. |
| `uid` | String | Output only. Server assigned ID of the resource. The server guarantees uniqueness and immutability until deleted. |
| `update_time` | String | Output only. The last-modified time. |
| `create_time` | String | Output only. The creation time. |
| `channel` | String | Required. The name of the connected subscriber Channel. This is a weak reference to avoid cross project and cross accounts references. This must be in `projects/{project}/location/{location}/channels/{channel_id}` format. |
| `name` | String | Required. The name of the connection. |
| `activation_token` | String | Input only. Activation token for the channel. The token will be used during the creation of ChannelConnection to bind the channel with the provider project. This field will not be stored in the provider resource. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create channel_connection
channel_connection = provider.eventarc_api.Channel_connection {
    parent = "value"  # Required. The parent collection in which to add this channel connection.
}

# Access channel_connection outputs
channel_connection_id = channel_connection.id
channel_connection_labels = channel_connection.labels
channel_connection_uid = channel_connection.uid
channel_connection_update_time = channel_connection.update_time
channel_connection_create_time = channel_connection.create_time
channel_connection_channel = channel_connection.channel
channel_connection_name = channel_connection.name
channel_connection_activation_token = channel_connection.activation_token
```

---


### Provider

Get a single Provider.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | Output only. Human friendly name for the Provider. For example "Cloud Storage". |
| `event_types` | Vec<String> | Output only. Event types for this provider. |
| `name` | String | Output only. In `projects/{project}/locations/{location}/providers/{provider_id}` format. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access provider outputs
provider_id = provider.id
provider_display_name = provider.display_name
provider_event_types = provider.event_types
provider_name = provider.name
```

---


### Operation

Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of `1`, corresponding to `Code.CANCELLED`.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | The name of the operation resource to be cancelled. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.eventarc_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_error = operation.error
operation_metadata = operation.metadata
operation_done = operation.done
operation_name = operation.name
```

---


### Trigger

Create a new trigger in a particular project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `conditions` | HashMap<String, String> |  | Output only. The reason(s) why a trigger is in FAILED state. |
| `etag` | String |  | Output only. This checksum is computed by the server based on the value of other fields, and might be sent only on create requests to ensure that the client has an up-to-date value before proceeding. |
| `event_filters` | Vec<String> |  | Required. Unordered list. The list of filters that applies to event attributes. Only events that match all the provided filters are sent to the destination. |
| `destination` | String |  | Required. Destination specifies where the events should be sent to. |
| `name` | String |  | Required. The resource name of the trigger. Must be unique within the location of the project and must be in `projects/{project}/locations/{location}/triggers/{trigger}` format. |
| `create_time` | String |  | Output only. The creation time. |
| `service_account` | String |  | Optional. The IAM service account email associated with the trigger. The service account represents the identity of the trigger. The `iam.serviceAccounts.actAs` permission must be granted on the service account to allow a principal to impersonate the service account. For more information, see the [Roles and permissions](/eventarc/docs/all-roles-permissions) page specific to the trigger destination. |
| `uid` | String |  | Output only. Server-assigned unique identifier for the trigger. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted. |
| `labels` | HashMap<String, String> |  | Optional. User labels attached to the triggers that can be used to group resources. |
| `update_time` | String |  | Output only. The last-modified time. |
| `transport` | String |  | Optional. To deliver messages, Eventarc might use other Google Cloud products as a transport intermediary. This field contains a reference to that transport intermediary. This information can be used for debugging purposes. |
| `event_data_content_type` | String |  | Optional. EventDataContentType specifies the type of payload in MIME format that is expected from the CloudEvent data field. This is set to `application/json` if the value is not defined. |
| `satisfies_pzs` | bool |  | Output only. Whether or not this Trigger satisfies the requirements of physical zone separation |
| `channel` | String |  | Optional. The name of the channel associated with the trigger in `projects/{project}/locations/{location}/channels/{channel}` format. You must provide a channel to receive events from Eventarc SaaS partners. |
| `parent` | String | ✅ | Required. The parent collection in which to add this trigger. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `conditions` | HashMap<String, String> | Output only. The reason(s) why a trigger is in FAILED state. |
| `etag` | String | Output only. This checksum is computed by the server based on the value of other fields, and might be sent only on create requests to ensure that the client has an up-to-date value before proceeding. |
| `event_filters` | Vec<String> | Required. Unordered list. The list of filters that applies to event attributes. Only events that match all the provided filters are sent to the destination. |
| `destination` | String | Required. Destination specifies where the events should be sent to. |
| `name` | String | Required. The resource name of the trigger. Must be unique within the location of the project and must be in `projects/{project}/locations/{location}/triggers/{trigger}` format. |
| `create_time` | String | Output only. The creation time. |
| `service_account` | String | Optional. The IAM service account email associated with the trigger. The service account represents the identity of the trigger. The `iam.serviceAccounts.actAs` permission must be granted on the service account to allow a principal to impersonate the service account. For more information, see the [Roles and permissions](/eventarc/docs/all-roles-permissions) page specific to the trigger destination. |
| `uid` | String | Output only. Server-assigned unique identifier for the trigger. The value is a UUID4 string and guaranteed to remain unchanged until the resource is deleted. |
| `labels` | HashMap<String, String> | Optional. User labels attached to the triggers that can be used to group resources. |
| `update_time` | String | Output only. The last-modified time. |
| `transport` | String | Optional. To deliver messages, Eventarc might use other Google Cloud products as a transport intermediary. This field contains a reference to that transport intermediary. This information can be used for debugging purposes. |
| `event_data_content_type` | String | Optional. EventDataContentType specifies the type of payload in MIME format that is expected from the CloudEvent data field. This is set to `application/json` if the value is not defined. |
| `satisfies_pzs` | bool | Output only. Whether or not this Trigger satisfies the requirements of physical zone separation |
| `channel` | String | Optional. The name of the channel associated with the trigger in `projects/{project}/locations/{location}/channels/{channel}` format. You must provide a channel to receive events from Eventarc SaaS partners. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create trigger
trigger = provider.eventarc_api.Trigger {
    parent = "value"  # Required. The parent collection in which to add this trigger.
}

# Access trigger outputs
trigger_id = trigger.id
trigger_conditions = trigger.conditions
trigger_etag = trigger.etag
trigger_event_filters = trigger.event_filters
trigger_destination = trigger.destination
trigger_name = trigger.name
trigger_create_time = trigger.create_time
trigger_service_account = trigger.service_account
trigger_uid = trigger.uid
trigger_labels = trigger.labels
trigger_update_time = trigger.update_time
trigger_transport = trigger.transport
trigger_event_data_content_type = trigger.event_data_content_type
trigger_satisfies_pzs = trigger.satisfies_pzs
trigger_channel = trigger.channel
```

---


### Kafka_source

Sets the access control policy on the specified resource. Replaces any existing policy. Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED` errors.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_mask` | String |  | OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only the fields in the mask will be modified. If no mask is provided, the following default mask is used: `paths: "bindings, etag"` |
| `policy` | String |  | REQUIRED: The complete policy to be applied to the `resource`. The size of the policy is limited to a few 10s of KB. An empty policy is a valid policy but certain Google Cloud services (such as Projects) might reject them. |
| `resource` | String | ✅ | REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `bindings` | Vec<String> | Associates a list of `members`, or principals, with a `role`. Optionally, may specify a `condition` that determines how and when the `bindings` are applied. Each of the `bindings` must contain at least one principal. The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250 of these principals can be Google groups. Each occurrence of a principal counts towards these limits. For example, if the `bindings` grant 50 different roles to `user:alice@example.com`, and not to any other principal, then you can add another 1,450 principals to the `bindings` in the `Policy`. |
| `audit_configs` | Vec<String> | Specifies cloud audit logging configuration for this policy. |
| `version` | i64 | Specifies the format of the policy. Valid values are `0`, `1`, and `3`. Requests that specify an invalid value are rejected. Any operation that affects conditional role bindings must specify version `3`. This requirement applies to the following operations: * Getting a policy that includes a conditional role binding * Adding a conditional role binding to a policy * Changing a conditional role binding in a policy * Removing any role binding, with or without a condition, from a policy that includes conditions **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. If a policy does not include any conditions, operations on that policy may specify any valid version or leave the field unset. To learn which resources support conditions in their IAM policies, see the [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies). |
| `etag` | String | `etag` is used for optimistic concurrency control as a way to help prevent simultaneous updates of a policy from overwriting each other. It is strongly suggested that systems make use of the `etag` in the read-modify-write cycle to perform policy updates in order to avoid race conditions: An `etag` is returned in the response to `getIamPolicy`, and systems are expected to put that etag in the request to `setIamPolicy` to ensure that their change will be applied to the same version of the policy. **Important:** If you use IAM Conditions, you must include the `etag` field whenever you call `setIamPolicy`. If you omit this field, then IAM allows you to overwrite a version `3` policy with a version `1` policy, and all of the conditions in the version `3` policy are lost. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create kafka_source
kafka_source = provider.eventarc_api.Kafka_source {
    resource = "value"  # REQUIRED: The resource for which the policy is being specified. See [Resource names](https://cloud.google.com/apis/design/resource_names) for the appropriate value for this field.
}

# Access kafka_source outputs
kafka_source_id = kafka_source.id
kafka_source_bindings = kafka_source.bindings
kafka_source_audit_configs = kafka_source.audit_configs
kafka_source_version = kafka_source.version
kafka_source_etag = kafka_source.etag
```

---


### Operation

Starts asynchronous cancellation on a long-running operation. The server makes a best effort to cancel the operation, but success is not guaranteed. If the server doesn't support this method, it returns `google.rpc.Code.UNIMPLEMENTED`. Clients can use Operations.GetOperation or other methods to check whether the cancellation succeeded or whether the operation completed despite cancellation. On successful cancellation, the operation is not deleted; instead, it becomes an operation with an Operation.error value with a google.rpc.Status.code of 1, corresponding to `Code.CANCELLED`.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | The name of the operation resource to be cancelled. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.eventarc_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_response = operation.response
operation_error = operation.error
operation_name = operation.name
operation_metadata = operation.metadata
```

---


### Trigger

Create a new trigger in a particular project and location.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The creation time. |
| `transport` | String |  | Output only. In order to deliver messages, Eventarc may use other Google Cloud products as transport intermediary. This field contains a reference to that transport intermediary. This information can be used for debugging purposes. |
| `matching_criteria` | Vec<String> |  | Required. Unordered list. The criteria by which events are filtered. Only events that match with this criteria will be sent to the destination. |
| `name` | String |  | Required. The resource name of the trigger. Must be unique within the location on the project and must in `projects/{project}/locations/{location}/triggers/{trigger}` format. |
| `destination` | String |  | Required. Destination specifies where the events should be sent to. |
| `labels` | HashMap<String, String> |  | Optional. User labels attached to the triggers that can be used to group resources. |
| `update_time` | String |  | Output only. The last-modified time. |
| `etag` | String |  | Output only. This checksum is computed by the server based on the value of other fields, and may be sent only on create requests to ensure the client has an up-to-date value before proceeding. |
| `service_account` | String |  | Optional. The IAM service account email associated with the trigger. The service account represents the identity of the trigger. The principal who calls this API must have `iam.serviceAccounts.actAs` permission in the service account. See https://cloud.google.com/iam/docs/understanding-service-accounts?hl=en#sa_common for more information. For Cloud Run destinations, this service account is used to generate identity tokens when invoking the service. See https://cloud.google.com/run/docs/triggering/pubsub-push#create-service-account for information on how to invoke authenticated Cloud Run services. In order to create Audit Log triggers, the service account should also have 'eventarc.events.receiveAuditLogV1Written' permission. |
| `parent` | String | ✅ | Required. The parent collection in which to add this trigger. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The creation time. |
| `transport` | String | Output only. In order to deliver messages, Eventarc may use other Google Cloud products as transport intermediary. This field contains a reference to that transport intermediary. This information can be used for debugging purposes. |
| `matching_criteria` | Vec<String> | Required. Unordered list. The criteria by which events are filtered. Only events that match with this criteria will be sent to the destination. |
| `name` | String | Required. The resource name of the trigger. Must be unique within the location on the project and must in `projects/{project}/locations/{location}/triggers/{trigger}` format. |
| `destination` | String | Required. Destination specifies where the events should be sent to. |
| `labels` | HashMap<String, String> | Optional. User labels attached to the triggers that can be used to group resources. |
| `update_time` | String | Output only. The last-modified time. |
| `etag` | String | Output only. This checksum is computed by the server based on the value of other fields, and may be sent only on create requests to ensure the client has an up-to-date value before proceeding. |
| `service_account` | String | Optional. The IAM service account email associated with the trigger. The service account represents the identity of the trigger. The principal who calls this API must have `iam.serviceAccounts.actAs` permission in the service account. See https://cloud.google.com/iam/docs/understanding-service-accounts?hl=en#sa_common for more information. For Cloud Run destinations, this service account is used to generate identity tokens when invoking the service. See https://cloud.google.com/run/docs/triggering/pubsub-push#create-service-account for information on how to invoke authenticated Cloud Run services. In order to create Audit Log triggers, the service account should also have 'eventarc.events.receiveAuditLogV1Written' permission. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create trigger
trigger = provider.eventarc_api.Trigger {
    parent = "value"  # Required. The parent collection in which to add this trigger.
}

# Access trigger outputs
trigger_id = trigger.id
trigger_create_time = trigger.create_time
trigger_transport = trigger.transport
trigger_matching_criteria = trigger.matching_criteria
trigger_name = trigger.name
trigger_destination = trigger.destination
trigger_labels = trigger.labels
trigger_update_time = trigger.update_time
trigger_etag = trigger.etag
trigger_service_account = trigger.service_account
```

---


### Location

Gets information about a location.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `labels` | HashMap<String, String> | Cross-service attributes for the location. For example {"cloud.googleapis.com/region": "us-east1"} |
| `display_name` | String | The friendly name for this location, typically a nearby city name. For example, "Tokyo". |
| `location_id` | String | The canonical id for this location. For example: `"us-east1"`. |
| `metadata` | HashMap<String, String> | Service-specific metadata. For example the available capacity at the given location. |
| `name` | String | Resource name for the location, which may vary between implementations. For example: `"projects/example-project/locations/us-east1"` |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access location outputs
location_id = location.id
location_labels = location.labels
location_display_name = location.display_name
location_location_id = location.location_id
location_metadata = location.metadata
location_name = location.name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple location resources
location_0 = provider.eventarc_api.Location {
    name = "value-0"
}
location_1 = provider.eventarc_api.Location {
    name = "value-1"
}
location_2 = provider.eventarc_api.Location {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    location = provider.eventarc_api.Location {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Eventarc_api Documentation](https://cloud.google.com/eventarc_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
