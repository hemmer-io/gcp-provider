# Cloudsupport_api Service



**Resources**: 10

---

## Overview

The cloudsupport_api service provides access to 10 resource types:

- [Case_classification](#case_classification) [R]
- [Attachment](#attachment) [R]
- [Comment](#comment) [CR]
- [Media](#media) [CR]
- [Case](#case) [CRU]
- [Attachment](#attachment) [R]
- [Comment](#comment) [CR]
- [Case](#case) [CRU]
- [Case_classification](#case_classification) [R]
- [Media](#media) [CR]

---

## Resources


### Case_classification

Retrieve valid classifications to use when creating a support case. Classifications are hierarchical. Each classification is a string containing all levels of the hierarchy separated by `" > "`. For example, `"Technical Issue > Compute > Compute Engine"`. Classification IDs returned by this endpoint are valid for at least six months. When a classification is deactivated, this endpoint immediately stops returning it. After six months, `case.create` requests using the classification will fail. EXAMPLES: cURL: ```shell curl \ --header "Authorization: Bearer $(gcloud auth print-access-token)" \ 'https://cloudsupport.googleapis.com/v2/caseClassifications:search?query=display_name:"*Compute%20Engine*"' ``` Python: ```python import googleapiclient.discovery supportApiService = googleapiclient.discovery.build( serviceName="cloudsupport", version="v2", discoveryServiceUrl=f"https://cloudsupport.googleapis.com/$discovery/rest?version=v2", ) request = supportApiService.caseClassifications().search( query='display_name:"*Compute Engine*"' ) print(request.execute()) ```

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `case_classifications` | Vec<String> | The classifications retrieved. |
| `next_page_token` | String | A token to retrieve the next page of results. Set this in the `page_token` field of subsequent `caseClassifications.list` requests. If unspecified, there are no more results to retrieve. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access case_classification outputs
case_classification_id = case_classification.id
case_classification_case_classifications = case_classification.case_classifications
case_classification_next_page_token = case_classification.next_page_token
```

---


### Attachment

Retrieve an attachment associated with a support case. EXAMPLES: cURL: ```shell attachment="projects/some-project/cases/23598314/attachments/0684M00000P3h1fQAB" curl \ --header "Authorization: Bearer $(gcloud auth print-access-token)" \ "https://cloudsupport.googleapis.com/v2/$attachment" ``` Python: ```python import googleapiclient.discovery api_version = "v2beta" supportApiService = googleapiclient.discovery.build( serviceName="cloudsupport", version=api_version, discoveryServiceUrl=f"https://cloudsupport.googleapis.com/$discovery/rest?version={api_version}", ) request = ( supportApiService.cases() .attachments() .get(name="projects/some-project/cases/43595344/attachments/0684M00000P3h1fQAB") ) print(request.execute()) ```

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time at which the attachment was created. |
| `name` | String | Output only. Identifier. The resource name of the attachment. |
| `filename` | String | The filename of the attachment (e.g. `"graph.jpg"`). |
| `creator` | String | Output only. The user who uploaded the attachment. Note, the name and email will be obfuscated if the attachment was uploaded by Google support. |
| `mime_type` | String | Output only. The MIME type of the attachment (e.g. text/plain). |
| `size_bytes` | String | Output only. The size of the attachment in bytes. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access attachment outputs
attachment_id = attachment.id
attachment_create_time = attachment.create_time
attachment_name = attachment.name
attachment_filename = attachment.filename
attachment_creator = attachment.creator
attachment_mime_type = attachment.mime_type
attachment_size_bytes = attachment.size_bytes
```

---


### Comment

Add a new comment to a case. The comment must have the following fields set: `body`. EXAMPLES: cURL: ```shell case="projects/some-project/cases/43591344" curl \ --request POST \ --header "Authorization: Bearer $(gcloud auth print-access-token)" \ --header 'Content-Type: application/json' \ --data '{ "body": "This is a test comment." }' \ "https://cloudsupport.googleapis.com/v2/$case/comments" ``` Python: ```python import googleapiclient.discovery api_version = "v2" supportApiService = googleapiclient.discovery.build( serviceName="cloudsupport", version=api_version, discoveryServiceUrl=f"https://cloudsupport.googleapis.com/$discovery/rest?version={api_version}", ) request = ( supportApiService.cases() .comments() .create( parent="projects/some-project/cases/43595344", body={"body": "This is a test comment."}, ) ) print(request.execute()) ```

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `creator` | String |  | Output only. The user or Google Support agent who created the comment. |
| `body` | String |  | The full comment body. Maximum of 12800 characters. |
| `name` | String |  | Output only. Identifier. The resource name of the comment. |
| `plain_text_body` | String |  | Output only. DEPRECATED. DO NOT USE. A duplicate of the `body` field. This field is only present for legacy reasons. |
| `create_time` | String |  | Output only. The time when the comment was created. |
| `parent` | String | ✅ | Required. The name of the case to which the comment should be added. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creator` | String | Output only. The user or Google Support agent who created the comment. |
| `body` | String | The full comment body. Maximum of 12800 characters. |
| `name` | String | Output only. Identifier. The resource name of the comment. |
| `plain_text_body` | String | Output only. DEPRECATED. DO NOT USE. A duplicate of the `body` field. This field is only present for legacy reasons. |
| `create_time` | String | Output only. The time when the comment was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create comment
comment = provider.cloudsupport_api.Comment {
    parent = "value"  # Required. The name of the case to which the comment should be added.
}

# Access comment outputs
comment_id = comment.id
comment_creator = comment.creator
comment_body = comment.body
comment_name = comment.name
comment_plain_text_body = comment.plain_text_body
comment_create_time = comment.create_time
```

---


### Media

Create a file attachment on a case or Cloud resource. The attachment must have the following fields set: `filename`. EXAMPLES: cURL: ```shell echo "This text is in a file I'm uploading using CSAPI." \ > "./example_file.txt" case="projects/some-project/cases/43594844" curl \ --header "Authorization: Bearer $(gcloud auth print-access-token)" \ --data-binary @"./example_file.txt" \ "https://cloudsupport.googleapis.com/upload/v2beta/$case/attachments?attachment.filename=uploaded_via_curl.txt" ``` Python: ```python import googleapiclient.discovery api_version = "v2" supportApiService = googleapiclient.discovery.build( serviceName="cloudsupport", version=api_version, discoveryServiceUrl=f"https://cloudsupport.googleapis.com/$discovery/rest?version={api_version}", ) file_path = "./example_file.txt" with open(file_path, "w") as file: file.write( "This text is inside a file I'm going to upload using the Cloud Support API.", ) request = supportApiService.media().upload( parent="projects/some-project/cases/43595344", media_body=file_path ) request.uri = request.uri.split("?")[0] + "?attachment.filename=uploaded_via_python.txt" print(request.execute()) ```

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `attachment` | String |  | Required. The attachment to be created. |
| `parent` | String | ✅ | Required. The name of the case or Cloud resource to which the attachment should be attached. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `download_parameters` | String | # gdata.* are outside protos with mising documentation |
| `sha1_hash` | String | # gdata.* are outside protos with mising documentation |
| `is_potential_retry` | bool | # gdata.* are outside protos with mising documentation |
| `bigstore_object_ref` | String | # gdata.* are outside protos with mising documentation |
| `blob_ref` | String | # gdata.* are outside protos with mising documentation |
| `length` | String | # gdata.* are outside protos with mising documentation |
| `inline` | String | # gdata.* are outside protos with mising documentation |
| `diff_download_response` | String | # gdata.* are outside protos with mising documentation |
| `crc32c_hash` | i64 | # gdata.* are outside protos with mising documentation |
| `path` | String | # gdata.* are outside protos with mising documentation |
| `filename` | String | # gdata.* are outside protos with mising documentation |
| `object_id` | String | # gdata.* are outside protos with mising documentation |
| `hash_verified` | bool | # gdata.* are outside protos with mising documentation |
| `diff_checksums_response` | String | # gdata.* are outside protos with mising documentation |
| `algorithm` | String | # gdata.* are outside protos with mising documentation |
| `diff_upload_request` | String | # gdata.* are outside protos with mising documentation |
| `diff_version_response` | String | # gdata.* are outside protos with mising documentation |
| `md5_hash` | String | # gdata.* are outside protos with mising documentation |
| `reference_type` | String | # gdata.* are outside protos with mising documentation |
| `composite_media` | Vec<String> | # gdata.* are outside protos with mising documentation |
| `content_type_info` | String | # gdata.* are outside protos with mising documentation |
| `cosmo_binary_reference` | String | # gdata.* are outside protos with mising documentation |
| `hash` | String | # gdata.* are outside protos with mising documentation |
| `sha256_hash` | String | # gdata.* are outside protos with mising documentation |
| `timestamp` | String | # gdata.* are outside protos with mising documentation |
| `blobstore2_info` | String | # gdata.* are outside protos with mising documentation |
| `token` | String | # gdata.* are outside protos with mising documentation |
| `content_type` | String | # gdata.* are outside protos with mising documentation |
| `media_id` | String | # gdata.* are outside protos with mising documentation |
| `diff_upload_response` | String | # gdata.* are outside protos with mising documentation |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create media
media = provider.cloudsupport_api.Media {
    parent = "value"  # Required. The name of the case or Cloud resource to which the attachment should be attached.
}

# Access media outputs
media_id = media.id
media_download_parameters = media.download_parameters
media_sha1_hash = media.sha1_hash
media_is_potential_retry = media.is_potential_retry
media_bigstore_object_ref = media.bigstore_object_ref
media_blob_ref = media.blob_ref
media_length = media.length
media_inline = media.inline
media_diff_download_response = media.diff_download_response
media_crc32c_hash = media.crc32c_hash
media_path = media.path
media_filename = media.filename
media_object_id = media.object_id
media_hash_verified = media.hash_verified
media_diff_checksums_response = media.diff_checksums_response
media_algorithm = media.algorithm
media_diff_upload_request = media.diff_upload_request
media_diff_version_response = media.diff_version_response
media_md5_hash = media.md5_hash
media_reference_type = media.reference_type
media_composite_media = media.composite_media
media_content_type_info = media.content_type_info
media_cosmo_binary_reference = media.cosmo_binary_reference
media_hash = media.hash
media_sha256_hash = media.sha256_hash
media_timestamp = media.timestamp
media_blobstore2_info = media.blobstore2_info
media_token = media.token
media_content_type = media.content_type
media_media_id = media.media_id
media_diff_upload_response = media.diff_upload_response
```

---


### Case

Create a new case and associate it with a parent. It must have the following fields set: `display_name`, `description`, `classification`, and `priority`. If you're just testing the API and don't want to route your case to an agent, set `testCase=true`. EXAMPLES: cURL: ```shell parent="projects/some-project" curl \ --request POST \ --header "Authorization: Bearer $(gcloud auth print-access-token)" \ --header 'Content-Type: application/json' \ --data '{ "display_name": "Test case created by me.", "description": "a random test case, feel free to close", "classification": { "id": "100IK2AKCLHMGRJ9CDGMOCGP8DM6UTB4BT262T31BT1M2T31DHNMENPO6KS36CPJ786L2TBFEHGN6NPI64R3CDHN8880G08I1H3MURR7DHII0GRCDTQM8" }, "time_zone": "-07:00", "subscriber_email_addresses": [ "foo@domain.com", "bar@domain.com" ], "testCase": true, "priority": "P3" }' \ "https://cloudsupport.googleapis.com/v2/$parent/cases" ``` Python: ```python import googleapiclient.discovery api_version = "v2" supportApiService = googleapiclient.discovery.build( serviceName="cloudsupport", version=api_version, discoveryServiceUrl=f"https://cloudsupport.googleapis.com/$discovery/rest?version={api_version}", ) request = supportApiService.cases().create( parent="projects/some-project", body={ "displayName": "A Test Case", "description": "This is a test case.", "testCase": True, "priority": "P2", "classification": { "id": "100IK2AKCLHMGRJ9CDGMOCGP8DM6UTB4BT262T31BT1M2T31DHNMENPO6KS36CPJ786L2TBFEHGN6NPI64R3CDHN8880G08I1H3MURR7DHII0GRCDTQM8" }, }, ) print(request.execute()) ```

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `subscriber_email_addresses` | Vec<String> |  | The email addresses to receive updates on this case. |
| `description` | String |  | A broad description of the issue. |
| `language_code` | String |  | The language the user has requested to receive support in. This should be a BCP 47 language code (e.g., `"en"`, `"zh-CN"`, `"zh-TW"`, `"ja"`, `"ko"`). If no language or an unsupported language is specified, this field defaults to English (en). Language selection during case creation may affect your available support options. For a list of supported languages and their support working hours, see: https://cloud.google.com/support/docs/language-working-hours |
| `escalated` | bool |  | Whether the case is currently escalated. |
| `update_time` | String |  | Output only. The time this case was last updated. |
| `display_name` | String |  | The short summary of the issue reported in this case. |
| `name` | String |  | Identifier. The resource name for the case. |
| `state` | String |  | Output only. The current status of the support case. |
| `creator` | String |  | The user who created the case. Note: The name and email will be obfuscated if the case was created by Google Support. |
| `time_zone` | String |  | The timezone of the user who created the support case. It should be in a format IANA recognizes: https://www.iana.org/time-zones. There is no additional validation done by the API. |
| `create_time` | String |  | Output only. The time this case was created. |
| `contact_email` | String |  | A user-supplied email address to send case update notifications for. This should only be used in BYOID flows, where we cannot infer the user's email address directly from their EUCs. |
| `classification` | String |  | The issue classification applicable to this case. |
| `priority` | String |  | The priority of this case. |
| `severity` | String |  | REMOVED. The severity of this case. Use priority instead. |
| `test_case` | bool |  | Whether this case was created for internal API testing and should not be acted on by the support team. |
| `parent` | String | ✅ | Required. The name of the parent under which the case should be created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `subscriber_email_addresses` | Vec<String> | The email addresses to receive updates on this case. |
| `description` | String | A broad description of the issue. |
| `language_code` | String | The language the user has requested to receive support in. This should be a BCP 47 language code (e.g., `"en"`, `"zh-CN"`, `"zh-TW"`, `"ja"`, `"ko"`). If no language or an unsupported language is specified, this field defaults to English (en). Language selection during case creation may affect your available support options. For a list of supported languages and their support working hours, see: https://cloud.google.com/support/docs/language-working-hours |
| `escalated` | bool | Whether the case is currently escalated. |
| `update_time` | String | Output only. The time this case was last updated. |
| `display_name` | String | The short summary of the issue reported in this case. |
| `name` | String | Identifier. The resource name for the case. |
| `state` | String | Output only. The current status of the support case. |
| `creator` | String | The user who created the case. Note: The name and email will be obfuscated if the case was created by Google Support. |
| `time_zone` | String | The timezone of the user who created the support case. It should be in a format IANA recognizes: https://www.iana.org/time-zones. There is no additional validation done by the API. |
| `create_time` | String | Output only. The time this case was created. |
| `contact_email` | String | A user-supplied email address to send case update notifications for. This should only be used in BYOID flows, where we cannot infer the user's email address directly from their EUCs. |
| `classification` | String | The issue classification applicable to this case. |
| `priority` | String | The priority of this case. |
| `severity` | String | REMOVED. The severity of this case. Use priority instead. |
| `test_case` | bool | Whether this case was created for internal API testing and should not be acted on by the support team. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create case
case = provider.cloudsupport_api.Case {
    parent = "value"  # Required. The name of the parent under which the case should be created.
}

# Access case outputs
case_id = case.id
case_subscriber_email_addresses = case.subscriber_email_addresses
case_description = case.description
case_language_code = case.language_code
case_escalated = case.escalated
case_update_time = case.update_time
case_display_name = case.display_name
case_name = case.name
case_state = case.state
case_creator = case.creator
case_time_zone = case.time_zone
case_create_time = case.create_time
case_contact_email = case.contact_email
case_classification = case.classification
case_priority = case.priority
case_severity = case.severity
case_test_case = case.test_case
```

---


### Attachment

List all the attachments associated with a support case. EXAMPLES: cURL: ```shell case="projects/some-project/cases/23598314" curl \ --header "Authorization: Bearer $(gcloud auth print-access-token)" \ "https://cloudsupport.googleapis.com/v2/$case/attachments" ``` Python: ```python import googleapiclient.discovery api_version = "v2" supportApiService = googleapiclient.discovery.build( serviceName="cloudsupport", version=api_version, discoveryServiceUrl=f"https://cloudsupport.googleapis.com/$discovery/rest?version={api_version}", ) request = ( supportApiService.cases() .attachments() .list(parent="projects/some-project/cases/43595344") ) print(request.execute()) ```

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token to retrieve the next page of results. Set this in the `page_token` field of subsequent `cases.attachments.list` requests. If unspecified, there are no more results to retrieve. |
| `attachments` | Vec<String> | The list of attachments associated with a case. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access attachment outputs
attachment_id = attachment.id
attachment_next_page_token = attachment.next_page_token
attachment_attachments = attachment.attachments
```

---


### Comment

Add a new comment to a case. The comment must have the following fields set: `body`. EXAMPLES: cURL: ```shell case="projects/some-project/cases/43591344" curl \ --request POST \ --header "Authorization: Bearer $(gcloud auth print-access-token)" \ --header 'Content-Type: application/json' \ --data '{ "body": "This is a test comment." }' \ "https://cloudsupport.googleapis.com/v2/$case/comments" ``` Python: ```python import googleapiclient.discovery api_version = "v2" supportApiService = googleapiclient.discovery.build( serviceName="cloudsupport", version=api_version, discoveryServiceUrl=f"https://cloudsupport.googleapis.com/$discovery/rest?version={api_version}", ) request = ( supportApiService.cases() .comments() .create( parent="projects/some-project/cases/43595344", body={"body": "This is a test comment."}, ) ) print(request.execute()) ```

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `creator` | String |  | Output only. The user or Google Support agent who created the comment. |
| `name` | String |  | Output only. Identifier. The resource name of the comment. |
| `plain_text_body` | String |  | Output only. DEPRECATED. DO NOT USE. A duplicate of the `body` field. This field is only present for legacy reasons. |
| `create_time` | String |  | Output only. The time when the comment was created. |
| `body` | String |  | The full comment body. Maximum of 12800 characters. |
| `parent` | String | ✅ | Required. The name of the case to which the comment should be added. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token to retrieve the next page of results. Set this in the `page_token` field of subsequent `cases.comments.list` requests. If unspecified, there are no more results to retrieve. |
| `comments` | Vec<String> | List of the comments associated with the case. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create comment
comment = provider.cloudsupport_api.Comment {
    parent = "value"  # Required. The name of the case to which the comment should be added.
}

# Access comment outputs
comment_id = comment.id
comment_next_page_token = comment.next_page_token
comment_comments = comment.comments
```

---


### Case

Create a new case and associate it with a parent. It must have the following fields set: `display_name`, `description`, `classification`, and `priority`. If you're just testing the API and don't want to route your case to an agent, set `testCase=true`. EXAMPLES: cURL: ```shell parent="projects/some-project" curl \ --request POST \ --header "Authorization: Bearer $(gcloud auth print-access-token)" \ --header 'Content-Type: application/json' \ --data '{ "display_name": "Test case created by me.", "description": "a random test case, feel free to close", "classification": { "id": "100IK2AKCLHMGRJ9CDGMOCGP8DM6UTB4BT262T31BT1M2T31DHNMENPO6KS36CPJ786L2TBFEHGN6NPI64R3CDHN8880G08I1H3MURR7DHII0GRCDTQM8" }, "time_zone": "-07:00", "subscriber_email_addresses": [ "foo@domain.com", "bar@domain.com" ], "testCase": true, "priority": "P3" }' \ "https://cloudsupport.googleapis.com/v2/$parent/cases" ``` Python: ```python import googleapiclient.discovery api_version = "v2" supportApiService = googleapiclient.discovery.build( serviceName="cloudsupport", version=api_version, discoveryServiceUrl=f"https://cloudsupport.googleapis.com/$discovery/rest?version={api_version}", ) request = supportApiService.cases().create( parent="projects/some-project", body={ "displayName": "A Test Case", "description": "This is a test case.", "testCase": True, "priority": "P2", "classification": { "id": "100IK2AKCLHMGRJ9CDGMOCGP8DM6UTB4BT262T31BT1M2T31DHNMENPO6KS36CPJ786L2TBFEHGN6NPI64R3CDHN8880G08I1H3MURR7DHII0GRCDTQM8" }, }, ) print(request.execute()) ```

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. The time this case was last updated. |
| `language_code` | String |  | The language the user has requested to receive support in. This should be a BCP 47 language code (e.g., `"en"`, `"zh-CN"`, `"zh-TW"`, `"ja"`, `"ko"`). If no language or an unsupported language is specified, this field defaults to English (en). Language selection during case creation may affect your available support options. For a list of supported languages and their support working hours, see: https://cloud.google.com/support/docs/language-working-hours |
| `description` | String |  | A broad description of the issue. |
| `time_zone` | String |  | The timezone of the user who created the support case. It should be in a format IANA recognizes: https://www.iana.org/time-zones. There is no additional validation done by the API. |
| `creator` | String |  | The user who created the case. Note: The name and email will be obfuscated if the case was created by Google Support. |
| `subscriber_email_addresses` | Vec<String> |  | The email addresses to receive updates on this case. |
| `name` | String |  | Identifier. The resource name for the case. |
| `contact_email` | String |  | A user-supplied email address to send case update notifications for. This should only be used in BYOID flows, where we cannot infer the user's email address directly from their EUCs. |
| `escalated` | bool |  | Whether the case is currently escalated. |
| `test_case` | bool |  | Whether this case was created for internal API testing and should not be acted on by the support team. |
| `display_name` | String |  | The short summary of the issue reported in this case. |
| `priority` | String |  | The priority of this case. |
| `classification` | String |  | The issue classification applicable to this case. |
| `state` | String |  | Output only. The current status of the support case. |
| `create_time` | String |  | Output only. The time this case was created. |
| `parent` | String | ✅ | Required. The name of the parent under which the case should be created. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The time this case was last updated. |
| `language_code` | String | The language the user has requested to receive support in. This should be a BCP 47 language code (e.g., `"en"`, `"zh-CN"`, `"zh-TW"`, `"ja"`, `"ko"`). If no language or an unsupported language is specified, this field defaults to English (en). Language selection during case creation may affect your available support options. For a list of supported languages and their support working hours, see: https://cloud.google.com/support/docs/language-working-hours |
| `description` | String | A broad description of the issue. |
| `time_zone` | String | The timezone of the user who created the support case. It should be in a format IANA recognizes: https://www.iana.org/time-zones. There is no additional validation done by the API. |
| `creator` | String | The user who created the case. Note: The name and email will be obfuscated if the case was created by Google Support. |
| `subscriber_email_addresses` | Vec<String> | The email addresses to receive updates on this case. |
| `name` | String | Identifier. The resource name for the case. |
| `contact_email` | String | A user-supplied email address to send case update notifications for. This should only be used in BYOID flows, where we cannot infer the user's email address directly from their EUCs. |
| `escalated` | bool | Whether the case is currently escalated. |
| `test_case` | bool | Whether this case was created for internal API testing and should not be acted on by the support team. |
| `display_name` | String | The short summary of the issue reported in this case. |
| `priority` | String | The priority of this case. |
| `classification` | String | The issue classification applicable to this case. |
| `state` | String | Output only. The current status of the support case. |
| `create_time` | String | Output only. The time this case was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create case
case = provider.cloudsupport_api.Case {
    parent = "value"  # Required. The name of the parent under which the case should be created.
}

# Access case outputs
case_id = case.id
case_update_time = case.update_time
case_language_code = case.language_code
case_description = case.description
case_time_zone = case.time_zone
case_creator = case.creator
case_subscriber_email_addresses = case.subscriber_email_addresses
case_name = case.name
case_contact_email = case.contact_email
case_escalated = case.escalated
case_test_case = case.test_case
case_display_name = case.display_name
case_priority = case.priority
case_classification = case.classification
case_state = case.state
case_create_time = case.create_time
```

---


### Case_classification

Retrieve valid classifications to use when creating a support case. Classifications are hierarchical. Each classification is a string containing all levels of the hierarchy separated by `" > "`. For example, `"Technical Issue > Compute > Compute Engine"`. Classification IDs returned by this endpoint are valid for at least six months. When a classification is deactivated, this endpoint immediately stops returning it. After six months, `case.create` requests using the classification will fail. EXAMPLES: cURL: ```shell curl \ --header "Authorization: Bearer $(gcloud auth print-access-token)" \ 'https://cloudsupport.googleapis.com/v2/caseClassifications:search?query=display_name:"*Compute%20Engine*"' ``` Python: ```python import googleapiclient.discovery supportApiService = googleapiclient.discovery.build( serviceName="cloudsupport", version="v2", discoveryServiceUrl=f"https://cloudsupport.googleapis.com/$discovery/rest?version=v2", ) request = supportApiService.caseClassifications().search( query='display_name:"*Compute Engine*"' ) print(request.execute()) ```

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token to retrieve the next page of results. Set this in the `page_token` field of subsequent `caseClassifications.list` requests. If unspecified, there are no more results to retrieve. |
| `case_classifications` | Vec<String> | The classifications retrieved. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access case_classification outputs
case_classification_id = case_classification.id
case_classification_next_page_token = case_classification.next_page_token
case_classification_case_classifications = case_classification.case_classifications
```

---


### Media

Create a file attachment on a case or Cloud resource. The attachment must have the following fields set: `filename`. EXAMPLES: cURL: ```shell echo "This text is in a file I'm uploading using CSAPI." \ > "./example_file.txt" case="projects/some-project/cases/43594844" curl \ --header "Authorization: Bearer $(gcloud auth print-access-token)" \ --data-binary @"./example_file.txt" \ "https://cloudsupport.googleapis.com/upload/v2beta/$case/attachments?attachment.filename=uploaded_via_curl.txt" ``` Python: ```python import googleapiclient.discovery api_version = "v2" supportApiService = googleapiclient.discovery.build( serviceName="cloudsupport", version=api_version, discoveryServiceUrl=f"https://cloudsupport.googleapis.com/$discovery/rest?version={api_version}", ) file_path = "./example_file.txt" with open(file_path, "w") as file: file.write( "This text is inside a file I'm going to upload using the Cloud Support API.", ) request = supportApiService.media().upload( parent="projects/some-project/cases/43595344", media_body=file_path ) request.uri = request.uri.split("?")[0] + "?attachment.filename=uploaded_via_python.txt" print(request.execute()) ```

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `attachment` | String |  | Required. The attachment to be created. |
| `parent` | String | ✅ | Required. The name of the case or Cloud resource to which the attachment should be attached. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `crc32c_hash` | i64 | # gdata.* are outside protos with mising documentation |
| `timestamp` | String | # gdata.* are outside protos with mising documentation |
| `md5_hash` | String | # gdata.* are outside protos with mising documentation |
| `path` | String | # gdata.* are outside protos with mising documentation |
| `filename` | String | # gdata.* are outside protos with mising documentation |
| `blobstore2_info` | String | # gdata.* are outside protos with mising documentation |
| `media_id` | String | # gdata.* are outside protos with mising documentation |
| `diff_download_response` | String | # gdata.* are outside protos with mising documentation |
| `inline` | String | # gdata.* are outside protos with mising documentation |
| `diff_checksums_response` | String | # gdata.* are outside protos with mising documentation |
| `token` | String | # gdata.* are outside protos with mising documentation |
| `composite_media` | Vec<String> | # gdata.* are outside protos with mising documentation |
| `content_type_info` | String | # gdata.* are outside protos with mising documentation |
| `reference_type` | String | # gdata.* are outside protos with mising documentation |
| `diff_version_response` | String | # gdata.* are outside protos with mising documentation |
| `hash` | String | # gdata.* are outside protos with mising documentation |
| `object_id` | String | # gdata.* are outside protos with mising documentation |
| `length` | String | # gdata.* are outside protos with mising documentation |
| `hash_verified` | bool | # gdata.* are outside protos with mising documentation |
| `is_potential_retry` | bool | # gdata.* are outside protos with mising documentation |
| `bigstore_object_ref` | String | # gdata.* are outside protos with mising documentation |
| `algorithm` | String | # gdata.* are outside protos with mising documentation |
| `cosmo_binary_reference` | String | # gdata.* are outside protos with mising documentation |
| `blob_ref` | String | # gdata.* are outside protos with mising documentation |
| `content_type` | String | # gdata.* are outside protos with mising documentation |
| `diff_upload_response` | String | # gdata.* are outside protos with mising documentation |
| `sha1_hash` | String | # gdata.* are outside protos with mising documentation |
| `sha256_hash` | String | # gdata.* are outside protos with mising documentation |
| `diff_upload_request` | String | # gdata.* are outside protos with mising documentation |
| `download_parameters` | String | # gdata.* are outside protos with mising documentation |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create media
media = provider.cloudsupport_api.Media {
    parent = "value"  # Required. The name of the case or Cloud resource to which the attachment should be attached.
}

# Access media outputs
media_id = media.id
media_crc32c_hash = media.crc32c_hash
media_timestamp = media.timestamp
media_md5_hash = media.md5_hash
media_path = media.path
media_filename = media.filename
media_blobstore2_info = media.blobstore2_info
media_media_id = media.media_id
media_diff_download_response = media.diff_download_response
media_inline = media.inline
media_diff_checksums_response = media.diff_checksums_response
media_token = media.token
media_composite_media = media.composite_media
media_content_type_info = media.content_type_info
media_reference_type = media.reference_type
media_diff_version_response = media.diff_version_response
media_hash = media.hash
media_object_id = media.object_id
media_length = media.length
media_hash_verified = media.hash_verified
media_is_potential_retry = media.is_potential_retry
media_bigstore_object_ref = media.bigstore_object_ref
media_algorithm = media.algorithm
media_cosmo_binary_reference = media.cosmo_binary_reference
media_blob_ref = media.blob_ref
media_content_type = media.content_type
media_diff_upload_response = media.diff_upload_response
media_sha1_hash = media.sha1_hash
media_sha256_hash = media.sha256_hash
media_diff_upload_request = media.diff_upload_request
media_download_parameters = media.download_parameters
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple case_classification resources
case_classification_0 = provider.cloudsupport_api.Case_classification {
}
case_classification_1 = provider.cloudsupport_api.Case_classification {
}
case_classification_2 = provider.cloudsupport_api.Case_classification {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    case_classification = provider.cloudsupport_api.Case_classification {
    }
```

---

## Related Documentation

- [GCP Cloudsupport_api Documentation](https://cloud.google.com/cloudsupport_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
