# Firebasehosting_api Service



**Resources**: 9

---

## Overview

The firebasehosting_api service provides access to 9 resource types:

- [Operation](#operation) [CRD]
- [Operation](#operation) [R]
- [Release](#release) [CR]
- [Site](#site) [CRUD]
- [Version](#version) [CRUD]
- [Domain](#domain) [CRUD]
- [Custom_domain](#custom_domain) [CRUD]
- [Channel](#channel) [CRUD]
- [File](#file) [R]

---

## Resources


### Operation

CancelOperation is a part of the google.longrunning.Operations interface, but is not implemented for CustomDomain resources.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String | ✅ | The name of the operation resource to be cancelled. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `operations` | Vec<String> | A list of operations that matches the specified filter in the request. |
| `unreachable` | Vec<String> | Unordered list. Unreachable resources. Populated when the request sets `ListOperationsRequest.return_partial_success` and reads across collections e.g. when attempting to list all resources across all supported locations. |
| `next_page_token` | String | The standard List next-page token. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create operation
operation = provider.firebasehosting_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_operations = operation.operations
operation_unreachable = operation.unreachable
operation_next_page_token = operation.next_page_token
```

---


### Operation

Gets the latest state of a long-running operation. Use this method to poll the operation result at intervals as recommended by the API service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |


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
operation_name = operation.name
operation_response = operation.response
operation_metadata = operation.metadata
operation_done = operation.done
operation_error = operation.error
```

---


### Release

Creates a new release, which makes the content of the specified version actively display on the appropriate URL(s).

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `version` | String |  | Output only. The configuration and content that was released. |
| `name` | String |  | Output only. The unique identifier for the release, in either of the following formats: - sites/SITE_ID/releases/RELEASE_ID - sites/SITE_ID/channels/CHANNEL_ID/releases/RELEASE_ID This name is provided in the response body when you call [`releases.create`](sites.releases/create) or [`channels.releases.create`](sites.channels.releases/create). |
| `release_time` | String |  | Output only. The time at which the version is set to be public. |
| `message` | String |  | The deploy description when the release was created. The value can be up to 512 characters. |
| `release_user` | String |  | Output only. Identifies the user who created the release. |
| `type` | String |  | Explains the reason for the release. Specify a value for this field only when creating a `SITE_DISABLE` type release. |
| `parent` | String | ✅ | Required. The site or channel to which the release belongs, in either of the following formats: - sites/SITE_ID - sites/SITE_ID/channels/CHANNEL_ID |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `version` | String | Output only. The configuration and content that was released. |
| `name` | String | Output only. The unique identifier for the release, in either of the following formats: - sites/SITE_ID/releases/RELEASE_ID - sites/SITE_ID/channels/CHANNEL_ID/releases/RELEASE_ID This name is provided in the response body when you call [`releases.create`](sites.releases/create) or [`channels.releases.create`](sites.channels.releases/create). |
| `release_time` | String | Output only. The time at which the version is set to be public. |
| `message` | String | The deploy description when the release was created. The value can be up to 512 characters. |
| `release_user` | String | Output only. Identifies the user who created the release. |
| `type` | String | Explains the reason for the release. Specify a value for this field only when creating a `SITE_DISABLE` type release. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create release
release = provider.firebasehosting_api.Release {
    parent = "value"  # Required. The site or channel to which the release belongs, in either of the following formats: - sites/SITE_ID - sites/SITE_ID/channels/CHANNEL_ID
}

# Access release outputs
release_id = release.id
release_version = release.version
release_name = release.name
release_release_time = release.release_time
release_message = release.message
release_release_user = release.release_user
release_type = release.type
```

---


### Site

Creates a new Hosting Site in the specified parent Firebase project. Note that Hosting sites can take several minutes to propagate through Firebase systems.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `app_id` | String |  | Optional. The [ID of a Web App](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.webApps#WebApp.FIELDS.app_id) associated with the Hosting site. |
| `default_url` | String |  | Output only. The default URL for the Hosting site. |
| `name` | String |  | Output only. The fully-qualified resource name of the Hosting site, in the format: projects/PROJECT_IDENTIFIER/sites/SITE_ID PROJECT_IDENTIFIER: the Firebase project's [`ProjectNumber`](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects#FirebaseProject.FIELDS.project_number) ***(recommended)*** or its [`ProjectId`](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects#FirebaseProject.FIELDS.project_id). Learn more about using project identifiers in Google's [AIP 2510 standard](https://google.aip.dev/cloud/2510). |
| `type` | String |  | Output only. The type of Hosting site. Every Firebase project has a `DEFAULT_SITE`, which is created when Hosting is provisioned for the project. All additional sites are `USER_SITE`. |
| `labels` | HashMap<String, String> |  | Optional. User-specified labels for the Hosting site. |
| `parent` | String | ✅ | Required. The Firebase project in which to create a Hosting site, in the format: projects/PROJECT_IDENTIFIER Refer to the `Site` [`name`](../projects#Site.FIELDS.name) field for details about PROJECT_IDENTIFIER values. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `app_id` | String | Optional. The [ID of a Web App](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects.webApps#WebApp.FIELDS.app_id) associated with the Hosting site. |
| `default_url` | String | Output only. The default URL for the Hosting site. |
| `name` | String | Output only. The fully-qualified resource name of the Hosting site, in the format: projects/PROJECT_IDENTIFIER/sites/SITE_ID PROJECT_IDENTIFIER: the Firebase project's [`ProjectNumber`](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects#FirebaseProject.FIELDS.project_number) ***(recommended)*** or its [`ProjectId`](https://firebase.google.com/docs/reference/firebase-management/rest/v1beta1/projects#FirebaseProject.FIELDS.project_id). Learn more about using project identifiers in Google's [AIP 2510 standard](https://google.aip.dev/cloud/2510). |
| `type` | String | Output only. The type of Hosting site. Every Firebase project has a `DEFAULT_SITE`, which is created when Hosting is provisioned for the project. All additional sites are `USER_SITE`. |
| `labels` | HashMap<String, String> | Optional. User-specified labels for the Hosting site. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create site
site = provider.firebasehosting_api.Site {
    parent = "value"  # Required. The Firebase project in which to create a Hosting site, in the format: projects/PROJECT_IDENTIFIER Refer to the `Site` [`name`](../projects#Site.FIELDS.name) field for details about PROJECT_IDENTIFIER values.
}

# Access site outputs
site_id = site.id
site_app_id = site.app_id
site_default_url = site.default_url
site_name = site.name
site_type = site.type
site_labels = site.labels
```

---


### Version

Creates a new version for the specified site.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `config` | String |  | The configuration for the behavior of the site. This configuration exists in the [`firebase.json`](https://firebase.google.com/docs/cli/#the_firebasejson_file) file. |
| `create_user` | String |  | Output only. Identifies the user who created the version. |
| `file_count` | String |  | Output only. The total number of files associated with the version. This value is calculated after a version is `FINALIZED`. |
| `status` | String |  | The deploy status of the version. For a successful deploy, call [`CreateVersion`](sites.versions/create) to make a new version (`CREATED` status), [upload all desired files](sites.versions/populateFiles) to the version, then [update](sites.versions/patch) the version to the `FINALIZED` status. Note that if you leave the version in the `CREATED` state for more than 12 hours, the system will automatically mark the version as `ABANDONED`. You can also change the status of a version to `DELETED` by calling [`DeleteVersion`](sites.versions/delete). |
| `delete_time` | String |  | Output only. The time at which the version was `DELETED`. |
| `finalize_time` | String |  | Output only. The time at which the version was `FINALIZED`. |
| `delete_user` | String |  | Output only. Identifies the user who `DELETED` the version. |
| `create_time` | String |  | Output only. The time at which the version was created. |
| `finalize_user` | String |  | Output only. Identifies the user who `FINALIZED` the version. |
| `labels` | HashMap<String, String> |  | The labels used for extra metadata and/or filtering. |
| `name` | String |  | The fully-qualified resource name for the version, in the format: sites/ SITE_ID/versions/VERSION_ID This name is provided in the response body when you call [`CreateVersion`](sites.versions/create). |
| `version_bytes` | String |  | Output only. The total stored bytesize of the version. This value is calculated after a version is `FINALIZED`. |
| `parent` | String | ✅ | Required. The site in which to create the version, in the format: sites/ SITE_ID |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `config` | String | The configuration for the behavior of the site. This configuration exists in the [`firebase.json`](https://firebase.google.com/docs/cli/#the_firebasejson_file) file. |
| `create_user` | String | Output only. Identifies the user who created the version. |
| `file_count` | String | Output only. The total number of files associated with the version. This value is calculated after a version is `FINALIZED`. |
| `status` | String | The deploy status of the version. For a successful deploy, call [`CreateVersion`](sites.versions/create) to make a new version (`CREATED` status), [upload all desired files](sites.versions/populateFiles) to the version, then [update](sites.versions/patch) the version to the `FINALIZED` status. Note that if you leave the version in the `CREATED` state for more than 12 hours, the system will automatically mark the version as `ABANDONED`. You can also change the status of a version to `DELETED` by calling [`DeleteVersion`](sites.versions/delete). |
| `delete_time` | String | Output only. The time at which the version was `DELETED`. |
| `finalize_time` | String | Output only. The time at which the version was `FINALIZED`. |
| `delete_user` | String | Output only. Identifies the user who `DELETED` the version. |
| `create_time` | String | Output only. The time at which the version was created. |
| `finalize_user` | String | Output only. Identifies the user who `FINALIZED` the version. |
| `labels` | HashMap<String, String> | The labels used for extra metadata and/or filtering. |
| `name` | String | The fully-qualified resource name for the version, in the format: sites/ SITE_ID/versions/VERSION_ID This name is provided in the response body when you call [`CreateVersion`](sites.versions/create). |
| `version_bytes` | String | Output only. The total stored bytesize of the version. This value is calculated after a version is `FINALIZED`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create version
version = provider.firebasehosting_api.Version {
    parent = "value"  # Required. The site in which to create the version, in the format: sites/ SITE_ID
}

# Access version outputs
version_id = version.id
version_config = version.config
version_create_user = version.create_user
version_file_count = version.file_count
version_status = version.status
version_delete_time = version.delete_time
version_finalize_time = version.finalize_time
version_delete_user = version.delete_user
version_create_time = version.create_time
version_finalize_user = version.finalize_user
version_labels = version.labels
version_name = version.name
version_version_bytes = version.version_bytes
```

---


### Domain

Creates a domain mapping on the specified site.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `provisioning` | String |  | Output only. Information about the provisioning of certificates and the health of the DNS resolution for the domain. |
| `site` | String |  | Required. The site name of the association. |
| `status` | String |  | Output only. Additional status of the domain association. |
| `domain_name` | String |  | Required. The domain name of the association. |
| `update_time` | String |  | Output only. The time at which the domain was last updated. |
| `domain_redirect` | String |  | If set, the domain should redirect with the provided parameters. |
| `parent` | String | ✅ | Required. The parent to create the domain association for, in the format: sites/site-name |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `provisioning` | String | Output only. Information about the provisioning of certificates and the health of the DNS resolution for the domain. |
| `site` | String | Required. The site name of the association. |
| `status` | String | Output only. Additional status of the domain association. |
| `domain_name` | String | Required. The domain name of the association. |
| `update_time` | String | Output only. The time at which the domain was last updated. |
| `domain_redirect` | String | If set, the domain should redirect with the provided parameters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create domain
domain = provider.firebasehosting_api.Domain {
    parent = "value"  # Required. The parent to create the domain association for, in the format: sites/site-name
}

# Access domain outputs
domain_id = domain.id
domain_provisioning = domain.provisioning
domain_site = domain.site
domain_status = domain.status
domain_domain_name = domain.domain_name
domain_update_time = domain.update_time
domain_domain_redirect = domain.domain_redirect
```

---


### Custom_domain

Creates a `CustomDomain`.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cert_preference` | String |  | A field that lets you specify which SSL certificate type Hosting creates for your domain name. Spark plan custom domains only have access to the `GROUPED` cert type, while Blaze plan domains can select any option. |
| `labels` | HashMap<String, String> |  | Labels used for extra metadata and/or filtering. |
| `delete_time` | String |  | Output only. The time the `CustomDomain` was deleted; null for custom domains that haven't been deleted. Deleted custom domains persist for approximately 30 days, after which time Hosting removes them completely. To restore a deleted custom domain, make an `UndeleteCustomDomain` request. |
| `cert` | String |  | Output only. The SSL certificate Hosting has for this custom domain's domain name. For new custom domains, this often represents Hosting's intent to create a certificate, rather than an actual cert. Check the `state` field for more. |
| `update_time` | String |  | Output only. The last time the `CustomDomain` was updated. |
| `redirect_target` | String |  | A domain name that this `CustomDomain` should direct traffic towards. If specified, Hosting will respond to requests against this custom domain with an HTTP 301 code, and route traffic to the specified `redirect_target` instead. |
| `ownership_state` | String |  | Output only. The `OwnershipState` of the domain name this `CustomDomain` refers to. |
| `reconciling` | bool |  | Output only. A field that, if true, indicates that Hosting's systems are attmepting to make the custom domain's state match your preferred state. This is most frequently `true` when initially provisioning a `CustomDomain` after a `CreateCustomDomain` request or when creating a new SSL certificate to match an updated `cert_preference` after an `UpdateCustomDomain` request. |
| `expire_time` | String |  | Output only. The minimum time before a soft-deleted `CustomDomain` is completely removed from Hosting; null for custom domains that haven't been deleted. |
| `name` | String |  | Output only. The fully-qualified name of the `CustomDomain`. |
| `create_time` | String |  | Output only. The custom domain's create time. |
| `issues` | Vec<String> |  | Output only. A set of errors Hosting systems encountered when trying to establish Hosting's ability to serve secure content for your domain name. Resolve these issues to ensure your `CustomDomain` behaves properly. |
| `annotations` | HashMap<String, String> |  | Annotations you can add to leave both human- and machine-readable metadata about your `CustomDomain`. |
| `required_dns_updates` | String |  | Output only. A set of updates you should make to the domain name's DNS records to let Hosting serve secure content on its behalf. |
| `etag` | String |  | Output only. A string that represents the current state of the `CustomDomain` and allows you to confirm its initial state in requests that would modify it. Use the tag to ensure consistency when making `UpdateCustomDomain`, `DeleteCustomDomain`, and `UndeleteCustomDomain` requests. |
| `host_state` | String |  | Output only. The `HostState` of the domain name this `CustomDomain` refers to. |
| `parent` | String | ✅ | Required. The custom domain's parent, specifically a Firebase Hosting `Site`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `cert_preference` | String | A field that lets you specify which SSL certificate type Hosting creates for your domain name. Spark plan custom domains only have access to the `GROUPED` cert type, while Blaze plan domains can select any option. |
| `labels` | HashMap<String, String> | Labels used for extra metadata and/or filtering. |
| `delete_time` | String | Output only. The time the `CustomDomain` was deleted; null for custom domains that haven't been deleted. Deleted custom domains persist for approximately 30 days, after which time Hosting removes them completely. To restore a deleted custom domain, make an `UndeleteCustomDomain` request. |
| `cert` | String | Output only. The SSL certificate Hosting has for this custom domain's domain name. For new custom domains, this often represents Hosting's intent to create a certificate, rather than an actual cert. Check the `state` field for more. |
| `update_time` | String | Output only. The last time the `CustomDomain` was updated. |
| `redirect_target` | String | A domain name that this `CustomDomain` should direct traffic towards. If specified, Hosting will respond to requests against this custom domain with an HTTP 301 code, and route traffic to the specified `redirect_target` instead. |
| `ownership_state` | String | Output only. The `OwnershipState` of the domain name this `CustomDomain` refers to. |
| `reconciling` | bool | Output only. A field that, if true, indicates that Hosting's systems are attmepting to make the custom domain's state match your preferred state. This is most frequently `true` when initially provisioning a `CustomDomain` after a `CreateCustomDomain` request or when creating a new SSL certificate to match an updated `cert_preference` after an `UpdateCustomDomain` request. |
| `expire_time` | String | Output only. The minimum time before a soft-deleted `CustomDomain` is completely removed from Hosting; null for custom domains that haven't been deleted. |
| `name` | String | Output only. The fully-qualified name of the `CustomDomain`. |
| `create_time` | String | Output only. The custom domain's create time. |
| `issues` | Vec<String> | Output only. A set of errors Hosting systems encountered when trying to establish Hosting's ability to serve secure content for your domain name. Resolve these issues to ensure your `CustomDomain` behaves properly. |
| `annotations` | HashMap<String, String> | Annotations you can add to leave both human- and machine-readable metadata about your `CustomDomain`. |
| `required_dns_updates` | String | Output only. A set of updates you should make to the domain name's DNS records to let Hosting serve secure content on its behalf. |
| `etag` | String | Output only. A string that represents the current state of the `CustomDomain` and allows you to confirm its initial state in requests that would modify it. Use the tag to ensure consistency when making `UpdateCustomDomain`, `DeleteCustomDomain`, and `UndeleteCustomDomain` requests. |
| `host_state` | String | Output only. The `HostState` of the domain name this `CustomDomain` refers to. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create custom_domain
custom_domain = provider.firebasehosting_api.Custom_domain {
    parent = "value"  # Required. The custom domain's parent, specifically a Firebase Hosting `Site`.
}

# Access custom_domain outputs
custom_domain_id = custom_domain.id
custom_domain_cert_preference = custom_domain.cert_preference
custom_domain_labels = custom_domain.labels
custom_domain_delete_time = custom_domain.delete_time
custom_domain_cert = custom_domain.cert
custom_domain_update_time = custom_domain.update_time
custom_domain_redirect_target = custom_domain.redirect_target
custom_domain_ownership_state = custom_domain.ownership_state
custom_domain_reconciling = custom_domain.reconciling
custom_domain_expire_time = custom_domain.expire_time
custom_domain_name = custom_domain.name
custom_domain_create_time = custom_domain.create_time
custom_domain_issues = custom_domain.issues
custom_domain_annotations = custom_domain.annotations
custom_domain_required_dns_updates = custom_domain.required_dns_updates
custom_domain_etag = custom_domain.etag
custom_domain_host_state = custom_domain.host_state
```

---


### Channel

Creates a new channel in the specified site.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. The time at which the channel was created. |
| `update_time` | String |  | Output only. The time at which the channel was last updated. |
| `labels` | HashMap<String, String> |  | Text labels used for extra metadata and/or filtering. |
| `expire_time` | String |  | The time at which the channel will be automatically deleted. If null, the channel will not be automatically deleted. This field is present in the output whether it's set directly or via the `ttl` field. |
| `retained_release_count` | i64 |  | The number of previous releases to retain on the channel for rollback or other purposes. Must be a number between 1-100. Defaults to 10 for new channels. |
| `ttl` | String |  | Input only. A time-to-live for this channel. Sets `expire_time` to the provided duration past the time of the request. |
| `url` | String |  | Output only. The URL at which the content of this channel's current release can be viewed. This URL is a Firebase-provided subdomain of `web.app`. The content of this channel's current release can also be viewed at the Firebase-provided subdomain of `firebaseapp.com`. If this channel is the `live` channel for the Hosting site, then the content of this channel's current release can also be viewed at any connected custom domains. |
| `name` | String |  | The fully-qualified resource name for the channel, in the format: sites/ SITE_ID/channels/CHANNEL_ID |
| `release` | String |  | Output only. The current release for the channel, if any. |
| `parent` | String | ✅ | Required. The site in which to create this channel, in the format: sites/ SITE_ID |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. The time at which the channel was created. |
| `update_time` | String | Output only. The time at which the channel was last updated. |
| `labels` | HashMap<String, String> | Text labels used for extra metadata and/or filtering. |
| `expire_time` | String | The time at which the channel will be automatically deleted. If null, the channel will not be automatically deleted. This field is present in the output whether it's set directly or via the `ttl` field. |
| `retained_release_count` | i64 | The number of previous releases to retain on the channel for rollback or other purposes. Must be a number between 1-100. Defaults to 10 for new channels. |
| `ttl` | String | Input only. A time-to-live for this channel. Sets `expire_time` to the provided duration past the time of the request. |
| `url` | String | Output only. The URL at which the content of this channel's current release can be viewed. This URL is a Firebase-provided subdomain of `web.app`. The content of this channel's current release can also be viewed at the Firebase-provided subdomain of `firebaseapp.com`. If this channel is the `live` channel for the Hosting site, then the content of this channel's current release can also be viewed at any connected custom domains. |
| `name` | String | The fully-qualified resource name for the channel, in the format: sites/ SITE_ID/channels/CHANNEL_ID |
| `release` | String | Output only. The current release for the channel, if any. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create channel
channel = provider.firebasehosting_api.Channel {
    parent = "value"  # Required. The site in which to create this channel, in the format: sites/ SITE_ID
}

# Access channel outputs
channel_id = channel.id
channel_create_time = channel.create_time
channel_update_time = channel.update_time
channel_labels = channel.labels
channel_expire_time = channel.expire_time
channel_retained_release_count = channel.retained_release_count
channel_ttl = channel.ttl
channel_url = channel.url
channel_name = channel.name
channel_release = channel.release
```

---


### File

Lists the remaining files to be uploaded for the specified version.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | The pagination token, if more results exist beyond the ones in this response. Include this token in your next call to `ListVersionFiles`. Page tokens are short-lived and should not be stored. |
| `files` | Vec<String> |  The list of paths to the hashes of the files in the specified version. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access file outputs
file_id = file.id
file_next_page_token = file.next_page_token
file_files = file.files
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple operation resources
operation_0 = provider.firebasehosting_api.Operation {
    name = "value-0"
}
operation_1 = provider.firebasehosting_api.Operation {
    name = "value-1"
}
operation_2 = provider.firebasehosting_api.Operation {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    operation = provider.firebasehosting_api.Operation {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Firebasehosting_api Documentation](https://cloud.google.com/firebasehosting_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
