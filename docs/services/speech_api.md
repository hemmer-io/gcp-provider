# Speech_api Service



**Resources**: 12

---

## Overview

The speech_api service provides access to 12 resource types:

- [Phrase_set](#phrase_set) [CRUD]
- [Custom_classe](#custom_classe) [CRUD]
- [Operation](#operation) [R]
- [Speech](#speech) [C]
- [Operation](#operation) [R]
- [Operation](#operation) [R]
- [Speech](#speech) [C]
- [Operation](#operation) [R]
- [Speech](#speech) [C]
- [Custom_classe](#custom_classe) [CRUD]
- [Phrase_set](#phrase_set) [CRUD]
- [Operation](#operation) [R]

---

## Resources


### Phrase_set

Create a set of phrase hints. Each item in the set can be a single word or a multi-word phrase. The items in the PhraseSet are favored by the recognition model when you send a call that includes the PhraseSet.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `phrase_set` | String |  | Required. The phrase set to create. |
| `phrase_set_id` | String |  | Required. The ID to use for the phrase set, which will become the final component of the phrase set's resource name. This value should restrict to letters, numbers, and hyphens, with the first character a letter, the last a letter or a number, and be 4-63 characters. |
| `parent` | String | ✅ | Required. The parent resource where this phrase set will be created. Format: `projects/{project}/locations/{location}` Speech-to-Text supports three locations: `global`, `us` (US North America), and `eu` (Europe). If you are calling the `speech.googleapis.com` endpoint, use the `global` location. To specify a region, use a [regional endpoint](https://cloud.google.com/speech-to-text/docs/endpoints) with matching `us` or `eu` location value. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `delete_time` | String | Output only. The time at which this resource was requested for deletion. This field is not used. |
| `etag` | String | Output only. This checksum is computed by the server based on the value of other fields. This may be sent on update, undelete, and delete requests to ensure the client has an up-to-date value before proceeding. This field is not used. |
| `expire_time` | String | Output only. The time at which this resource will be purged. This field is not used. |
| `kms_key_name` | String | Output only. The [KMS key name](https://cloud.google.com/kms/docs/resource-hierarchy#keys) with which the content of the PhraseSet is encrypted. The expected format is `projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}`. |
| `display_name` | String | Output only. User-settable, human-readable name for the PhraseSet. Must be 63 characters or less. This field is not used. |
| `phrases` | Vec<String> | A list of word and phrases. |
| `annotations` | HashMap<String, String> | Output only. Allows users to store small amounts of arbitrary data. Both the key and the value must be 63 characters or less each. At most 100 annotations. This field is not used. |
| `reconciling` | bool | Output only. Whether or not this PhraseSet is in the process of being updated. This field is not used. |
| `name` | String | The resource name of the phrase set. |
| `state` | String | Output only. The CustomClass lifecycle state. This field is not used. |
| `uid` | String | Output only. System-assigned unique identifier for the PhraseSet. This field is not used. |
| `kms_key_version_name` | String | Output only. The [KMS key version name](https://cloud.google.com/kms/docs/resource-hierarchy#key_versions) with which content of the PhraseSet is encrypted. The expected format is `projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}/cryptoKeyVersions/{crypto_key_version}`. |
| `boost` | f64 | Hint Boost. Positive value will increase the probability that a specific phrase will be recognized over other similar sounding phrases. The higher the boost, the higher the chance of false positive recognition as well. Negative boost values would correspond to anti-biasing. Anti-biasing is not enabled, so negative boost will simply be ignored. Though `boost` can accept a wide range of positive values, most use cases are best served with values between 0 (exclusive) and 20. We recommend using a binary search approach to finding the optimal value for your use case as well as adding phrases both with and without boost to your requests. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create phrase_set
phrase_set = provider.speech_api.Phrase_set {
    parent = "value"  # Required. The parent resource where this phrase set will be created. Format: `projects/{project}/locations/{location}` Speech-to-Text supports three locations: `global`, `us` (US North America), and `eu` (Europe). If you are calling the `speech.googleapis.com` endpoint, use the `global` location. To specify a region, use a [regional endpoint](https://cloud.google.com/speech-to-text/docs/endpoints) with matching `us` or `eu` location value.
}

# Access phrase_set outputs
phrase_set_id = phrase_set.id
phrase_set_delete_time = phrase_set.delete_time
phrase_set_etag = phrase_set.etag
phrase_set_expire_time = phrase_set.expire_time
phrase_set_kms_key_name = phrase_set.kms_key_name
phrase_set_display_name = phrase_set.display_name
phrase_set_phrases = phrase_set.phrases
phrase_set_annotations = phrase_set.annotations
phrase_set_reconciling = phrase_set.reconciling
phrase_set_name = phrase_set.name
phrase_set_state = phrase_set.state
phrase_set_uid = phrase_set.uid
phrase_set_kms_key_version_name = phrase_set.kms_key_version_name
phrase_set_boost = phrase_set.boost
```

---


### Custom_classe

Create a custom class.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `custom_class` | String |  | Required. The custom class to create. |
| `custom_class_id` | String |  | Required. The ID to use for the custom class, which will become the final component of the custom class' resource name. This value should restrict to letters, numbers, and hyphens, with the first character a letter, the last a letter or a number, and be 4-63 characters. |
| `parent` | String | ✅ | Required. The parent resource where this custom class will be created. Format: `projects/{project}/locations/{location}/customClasses` Speech-to-Text supports three locations: `global`, `us` (US North America), and `eu` (Europe). If you are calling the `speech.googleapis.com` endpoint, use the `global` location. To specify a region, use a [regional endpoint](https://cloud.google.com/speech-to-text/docs/endpoints) with matching `us` or `eu` location value. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | Output only. This checksum is computed by the server based on the value of other fields. This may be sent on update, undelete, and delete requests to ensure the client has an up-to-date value before proceeding. This field is not used. |
| `expire_time` | String | Output only. The time at which this resource will be purged. This field is not used. |
| `kms_key_name` | String | Output only. The [KMS key name](https://cloud.google.com/kms/docs/resource-hierarchy#keys) with which the content of the ClassItem is encrypted. The expected format is `projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}`. |
| `custom_class_id` | String | If this custom class is a resource, the custom_class_id is the resource id of the CustomClass. Case sensitive. |
| `kms_key_version_name` | String | Output only. The [KMS key version name](https://cloud.google.com/kms/docs/resource-hierarchy#key_versions) with which content of the ClassItem is encrypted. The expected format is `projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}/cryptoKeyVersions/{crypto_key_version}`. |
| `items` | Vec<String> | A collection of class items. |
| `annotations` | HashMap<String, String> | Output only. Allows users to store small amounts of arbitrary data. Both the key and the value must be 63 characters or less each. At most 100 annotations. This field is not used. |
| `state` | String | Output only. The CustomClass lifecycle state. This field is not used. |
| `uid` | String | Output only. System-assigned unique identifier for the CustomClass. This field is not used. |
| `reconciling` | bool | Output only. Whether or not this CustomClass is in the process of being updated. This field is not used. |
| `delete_time` | String | Output only. The time at which this resource was requested for deletion. This field is not used. |
| `name` | String | The resource name of the custom class. |
| `display_name` | String | Output only. User-settable, human-readable name for the CustomClass. Must be 63 characters or less. This field is not used. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create custom_classe
custom_classe = provider.speech_api.Custom_classe {
    parent = "value"  # Required. The parent resource where this custom class will be created. Format: `projects/{project}/locations/{location}/customClasses` Speech-to-Text supports three locations: `global`, `us` (US North America), and `eu` (Europe). If you are calling the `speech.googleapis.com` endpoint, use the `global` location. To specify a region, use a [regional endpoint](https://cloud.google.com/speech-to-text/docs/endpoints) with matching `us` or `eu` location value.
}

# Access custom_classe outputs
custom_classe_id = custom_classe.id
custom_classe_etag = custom_classe.etag
custom_classe_expire_time = custom_classe.expire_time
custom_classe_kms_key_name = custom_classe.kms_key_name
custom_classe_custom_class_id = custom_classe.custom_class_id
custom_classe_kms_key_version_name = custom_classe.kms_key_version_name
custom_classe_items = custom_classe.items
custom_classe_annotations = custom_classe.annotations
custom_classe_state = custom_classe.state
custom_classe_uid = custom_classe.uid
custom_classe_reconciling = custom_classe.reconciling
custom_classe_delete_time = custom_classe.delete_time
custom_classe_name = custom_classe.name
custom_classe_display_name = custom_classe.display_name
```

---


### Operation

Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_metadata = operation.metadata
operation_error = operation.error
operation_name = operation.name
operation_done = operation.done
```

---


### Speech

Performs synchronous speech recognition: receive results after all audio has been sent and processed.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `audio` | String |  | Required. The audio data to be recognized. |
| `config` | String |  | Required. Provides information to the recognizer that specifies how to process the request. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create speech
speech = provider.speech_api.Speech {
}

```

---


### Operation

Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `response` | HashMap<String, String> | The normal response of the operation in case of success. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_error = operation.error
operation_response = operation.response
operation_name = operation.name
operation_done = operation.done
```

---


### Operation

Gets the latest state of a long-running operation.  Clients can use this
method to poll the operation result at intervals as recommended by the API
service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation.  It typically
contains progress information and common metadata such as create time.
Some services might not provide such metadata.  Any method that returns a
long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that
originally returns it. If you use the default HTTP mapping, the
`name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal response of the operation in case of success.  If the original
method returns no data on success, such as `Delete`, the response is
`google.protobuf.Empty`.  If the original method is standard
`Get`/`Create`/`Update`, the response should be the resource.  For other
methods, the response should have the type `XxxResponse`, where `Xxx`
is the original method name.  For example, if the original method name
is `TakeSnapshot()`, the inferred response type is
`TakeSnapshotResponse`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `done` | bool | If the value is `false`, it means the operation is still in progress.
If `true`, the operation is completed, and either `error` or `response` is
available. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_name = operation.name
operation_response = operation.response
operation_error = operation.error
operation_done = operation.done
```

---


### Speech

Performs synchronous speech recognition: receive results after all audio
has been sent and processed.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `audio` | String |  | *Required* The audio data to be recognized. |
| `config` | String |  | *Required* Provides information to the recognizer that specifies how to
process the request. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create speech
speech = provider.speech_api.Speech {
}

```

---


### Operation

Gets the latest state of a long-running operation.  Clients can use this
method to poll the operation result at intervals as recommended by the API
service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `response` | HashMap<String, String> | The normal response of the operation in case of success.  If the original
method returns no data on success, such as `Delete`, the response is
`google.protobuf.Empty`.  If the original method is standard
`Get`/`Create`/`Update`, the response should be the resource.  For other
methods, the response should have the type `XxxResponse`, where `Xxx`
is the original method name.  For example, if the original method name
is `TakeSnapshot()`, the inferred response type is
`TakeSnapshotResponse`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation.  It typically
contains progress information and common metadata such as create time.
Some services might not provide such metadata.  Any method that returns a
long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that
originally returns it. If you use the default HTTP mapping, the
`name` should have the format of `operations/some/unique/name`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress.
If `true`, the operation is completed, and either `error` or `response` is
available. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_response = operation.response
operation_metadata = operation.metadata
operation_name = operation.name
operation_done = operation.done
```

---


### Speech

Performs asynchronous speech recognition: receive results via the google.longrunning.Operations interface. Returns either an `Operation.error` or an `Operation.response` which contains a `LongRunningRecognizeResponse` message. For more information on asynchronous speech recognition, see the [how-to](https://cloud.google.com/speech-to-text/docs/async-recognize).

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `config` | String |  | Required. Provides information to the recognizer that specifies how to process the request. |
| `output_config` | String |  | Optional. Specifies an optional destination for the recognition results. |
| `audio` | String |  | Required. The audio data to be recognized. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create speech
speech = provider.speech_api.Speech {
}

```

---


### Custom_classe

Create a custom class.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `custom_class_id` | String |  | Required. The ID to use for the custom class, which will become the final component of the custom class' resource name. This value should restrict to letters, numbers, and hyphens, with the first character a letter, the last a letter or a number, and be 4-63 characters. |
| `custom_class` | String |  | Required. The custom class to create. |
| `parent` | String | ✅ | Required. The parent resource where this custom class will be created. Format: `projects/{project}/locations/{location}/customClasses` Speech-to-Text supports three locations: `global`, `us` (US North America), and `eu` (Europe). If you are calling the `speech.googleapis.com` endpoint, use the `global` location. To specify a region, use a [regional endpoint](https://cloud.google.com/speech-to-text/docs/endpoints) with matching `us` or `eu` location value. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The resource name of the custom class. |
| `expire_time` | String | Output only. The time at which this resource will be purged. This field is not used. |
| `items` | Vec<String> | A collection of class items. |
| `annotations` | HashMap<String, String> | Output only. Allows users to store small amounts of arbitrary data. Both the key and the value must be 63 characters or less each. At most 100 annotations. This field is not used. |
| `kms_key_version_name` | String | Output only. The [KMS key version name](https://cloud.google.com/kms/docs/resource-hierarchy#key_versions) with which content of the ClassItem is encrypted. The expected format is `projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}/cryptoKeyVersions/{crypto_key_version}`. |
| `etag` | String | Output only. This checksum is computed by the server based on the value of other fields. This may be sent on update, undelete, and delete requests to ensure the client has an up-to-date value before proceeding. This field is not used. |
| `delete_time` | String | Output only. The time at which this resource was requested for deletion. This field is not used. |
| `state` | String | Output only. The CustomClass lifecycle state. This field is not used. |
| `uid` | String | Output only. System-assigned unique identifier for the CustomClass. This field is not used. |
| `kms_key_name` | String | Output only. The [KMS key name](https://cloud.google.com/kms/docs/resource-hierarchy#keys) with which the content of the ClassItem is encrypted. The expected format is `projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}`. |
| `custom_class_id` | String | If this custom class is a resource, the custom_class_id is the resource id of the CustomClass. Case sensitive. |
| `reconciling` | bool | Output only. Whether or not this CustomClass is in the process of being updated. This field is not used. |
| `display_name` | String | Output only. User-settable, human-readable name for the CustomClass. Must be 63 characters or less. This field is not used. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create custom_classe
custom_classe = provider.speech_api.Custom_classe {
    parent = "value"  # Required. The parent resource where this custom class will be created. Format: `projects/{project}/locations/{location}/customClasses` Speech-to-Text supports three locations: `global`, `us` (US North America), and `eu` (Europe). If you are calling the `speech.googleapis.com` endpoint, use the `global` location. To specify a region, use a [regional endpoint](https://cloud.google.com/speech-to-text/docs/endpoints) with matching `us` or `eu` location value.
}

# Access custom_classe outputs
custom_classe_id = custom_classe.id
custom_classe_name = custom_classe.name
custom_classe_expire_time = custom_classe.expire_time
custom_classe_items = custom_classe.items
custom_classe_annotations = custom_classe.annotations
custom_classe_kms_key_version_name = custom_classe.kms_key_version_name
custom_classe_etag = custom_classe.etag
custom_classe_delete_time = custom_classe.delete_time
custom_classe_state = custom_classe.state
custom_classe_uid = custom_classe.uid
custom_classe_kms_key_name = custom_classe.kms_key_name
custom_classe_custom_class_id = custom_classe.custom_class_id
custom_classe_reconciling = custom_classe.reconciling
custom_classe_display_name = custom_classe.display_name
```

---


### Phrase_set

Create a set of phrase hints. Each item in the set can be a single word or a multi-word phrase. The items in the PhraseSet are favored by the recognition model when you send a call that includes the PhraseSet.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `phrase_set` | String |  | Required. The phrase set to create. |
| `phrase_set_id` | String |  | Required. The ID to use for the phrase set, which will become the final component of the phrase set's resource name. This value should restrict to letters, numbers, and hyphens, with the first character a letter, the last a letter or a number, and be 4-63 characters. |
| `parent` | String | ✅ | Required. The parent resource where this phrase set will be created. Format: `projects/{project}/locations/{location}` Speech-to-Text supports three locations: `global`, `us` (US North America), and `eu` (Europe). If you are calling the `speech.googleapis.com` endpoint, use the `global` location. To specify a region, use a [regional endpoint](https://cloud.google.com/speech-to-text/docs/endpoints) with matching `us` or `eu` location value. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `expire_time` | String | Output only. The time at which this resource will be purged. This field is not used. |
| `delete_time` | String | Output only. The time at which this resource was requested for deletion. This field is not used. |
| `kms_key_name` | String | Output only. The [KMS key name](https://cloud.google.com/kms/docs/resource-hierarchy#keys) with which the content of the PhraseSet is encrypted. The expected format is `projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}`. |
| `state` | String | Output only. The CustomClass lifecycle state. This field is not used. |
| `reconciling` | bool | Output only. Whether or not this PhraseSet is in the process of being updated. This field is not used. |
| `kms_key_version_name` | String | Output only. The [KMS key version name](https://cloud.google.com/kms/docs/resource-hierarchy#key_versions) with which content of the PhraseSet is encrypted. The expected format is `projects/{project}/locations/{location}/keyRings/{key_ring}/cryptoKeys/{crypto_key}/cryptoKeyVersions/{crypto_key_version}`. |
| `uid` | String | Output only. System-assigned unique identifier for the PhraseSet. This field is not used. |
| `phrases` | Vec<String> | A list of word and phrases. |
| `boost` | f64 | Hint Boost. Positive value will increase the probability that a specific phrase will be recognized over other similar sounding phrases. The higher the boost, the higher the chance of false positive recognition as well. Negative boost values would correspond to anti-biasing. Anti-biasing is not enabled, so negative boost will simply be ignored. Though `boost` can accept a wide range of positive values, most use cases are best served with values between 0 (exclusive) and 20. We recommend using a binary search approach to finding the optimal value for your use case as well as adding phrases both with and without boost to your requests. |
| `name` | String | The resource name of the phrase set. |
| `etag` | String | Output only. This checksum is computed by the server based on the value of other fields. This may be sent on update, undelete, and delete requests to ensure the client has an up-to-date value before proceeding. This field is not used. |
| `annotations` | HashMap<String, String> | Output only. Allows users to store small amounts of arbitrary data. Both the key and the value must be 63 characters or less each. At most 100 annotations. This field is not used. |
| `display_name` | String | Output only. User-settable, human-readable name for the PhraseSet. Must be 63 characters or less. This field is not used. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create phrase_set
phrase_set = provider.speech_api.Phrase_set {
    parent = "value"  # Required. The parent resource where this phrase set will be created. Format: `projects/{project}/locations/{location}` Speech-to-Text supports three locations: `global`, `us` (US North America), and `eu` (Europe). If you are calling the `speech.googleapis.com` endpoint, use the `global` location. To specify a region, use a [regional endpoint](https://cloud.google.com/speech-to-text/docs/endpoints) with matching `us` or `eu` location value.
}

# Access phrase_set outputs
phrase_set_id = phrase_set.id
phrase_set_expire_time = phrase_set.expire_time
phrase_set_delete_time = phrase_set.delete_time
phrase_set_kms_key_name = phrase_set.kms_key_name
phrase_set_state = phrase_set.state
phrase_set_reconciling = phrase_set.reconciling
phrase_set_kms_key_version_name = phrase_set.kms_key_version_name
phrase_set_uid = phrase_set.uid
phrase_set_phrases = phrase_set.phrases
phrase_set_boost = phrase_set.boost
phrase_set_name = phrase_set.name
phrase_set_etag = phrase_set.etag
phrase_set_annotations = phrase_set.annotations
phrase_set_display_name = phrase_set.display_name
```

---


### Operation

Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_response = operation.response
operation_metadata = operation.metadata
operation_name = operation.name
operation_done = operation.done
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple phrase_set resources
phrase_set_0 = provider.speech_api.Phrase_set {
    parent = "value-0"
}
phrase_set_1 = provider.speech_api.Phrase_set {
    parent = "value-1"
}
phrase_set_2 = provider.speech_api.Phrase_set {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    phrase_set = provider.speech_api.Phrase_set {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Speech_api Documentation](https://cloud.google.com/speech_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
