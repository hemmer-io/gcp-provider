# Firebase_api Service



**Resources**: 9

---

## Overview

The firebase_api service provides access to 9 resource types:

- [Web_app](#web_app) [CRU]
- [Available_location](#available_location) [R]
- [Project](#project) [CRU]
- [Sha](#sha) [CRD]
- [Android_app](#android_app) [CRU]
- [Default_location](#default_location) [C]
- [Ios_app](#ios_app) [CRU]
- [Operation](#operation) [R]
- [Available_project](#available_project) [R]

---

## Resources


### Web_app

Requests the creation of a new WebApp in the specified FirebaseProject. The result of this call is an `Operation` which can be used to track the provisioning process. The `Operation` is automatically deleted after completion, so there is no need to call `DeleteOperation`.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `app_urls` | Vec<String> |  | The URLs where the `WebApp` is hosted. |
| `state` | String |  | Output only. The lifecycle state of the App. |
| `api_key_id` | String |  | The globally unique, Google-assigned identifier (UID) for the Firebase API key associated with the `WebApp`. Be aware that this value is the UID of the API key, _not_ the [`keyString`](https://cloud.google.com/api-keys/docs/reference/rest/v2/projects.locations.keys#Key.FIELDS.key_string) of the API key. The `keyString` is the value that can be found in the App's [configuration artifact](../../rest/v1beta1/projects.webApps/getConfig). If `api_key_id` is not set in requests to [`webApps.Create`](../../rest/v1beta1/projects.webApps/create), then Firebase automatically associates an `api_key_id` with the `WebApp`. This auto-associated key may be an existing valid key or, if no valid key exists, a new one will be provisioned. In patch requests, `api_key_id` cannot be set to an empty value, and the new UID must have no restrictions or only have restrictions that are valid for the associated `WebApp`. We recommend using the [Google Cloud Console](https://console.cloud.google.com/apis/credentials) to manage API keys. |
| `name` | String |  | The resource name of the WebApp, in the format: projects/PROJECT_IDENTIFIER /webApps/APP_ID * PROJECT_IDENTIFIER: the parent Project's [`ProjectNumber`](../projects#FirebaseProject.FIELDS.project_number) ***(recommended)*** or its [`ProjectId`](../projects#FirebaseProject.FIELDS.project_id). Learn more about using project identifiers in Google's [AIP 2510 standard](https://google.aip.dev/cloud/2510). Note that the value for PROJECT_IDENTIFIER in any response body will be the `ProjectId`. * APP_ID: the globally unique, Firebase-assigned identifier for the App (see [`appId`](../projects.webApps#WebApp.FIELDS.app_id)). |
| `project_id` | String |  | Output only. Immutable. A user-assigned unique identifier of the parent FirebaseProject for the `WebApp`. |
| `app_id` | String |  | Output only. Immutable. The globally unique, Firebase-assigned identifier for the `WebApp`. This identifier should be treated as an opaque token, as the data format is not specified. |
| `display_name` | String |  | The user-assigned display name for the `WebApp`. |
| `web_id` | String |  | Output only. Immutable. A unique, Firebase-assigned identifier for the `WebApp`. This identifier is only used to populate the `namespace` value for the `WebApp`. For most use cases, use `appId` to identify or reference the App. The `webId` value is only unique within a `FirebaseProject` and its associated Apps. |
| `etag` | String |  | This checksum is computed by the server based on the value of other fields, and it may be sent with update requests to ensure the client has an up-to-date value before proceeding. Learn more about `etag` in Google's [AIP-154 standard](https://google.aip.dev/154#declarative-friendly-resources). This etag is strongly validated. |
| `expire_time` | String |  | Output only. If the App has been removed from the Project, this is the timestamp of when the App is considered expired and will be permanently deleted. After this time, the App cannot be undeleted (that is, restored to the Project). This value is only provided if the App is in the `DELETED` state. |
| `parent` | String | ✅ | The resource name of the parent FirebaseProject in which to create a WebApp, in the format: projects/PROJECT_IDENTIFIER/webApps Refer to the `FirebaseProject` [`name`](../projects#FirebaseProject.FIELDS.name) field for details about PROJECT_IDENTIFIER values. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `app_urls` | Vec<String> | The URLs where the `WebApp` is hosted. |
| `state` | String | Output only. The lifecycle state of the App. |
| `api_key_id` | String | The globally unique, Google-assigned identifier (UID) for the Firebase API key associated with the `WebApp`. Be aware that this value is the UID of the API key, _not_ the [`keyString`](https://cloud.google.com/api-keys/docs/reference/rest/v2/projects.locations.keys#Key.FIELDS.key_string) of the API key. The `keyString` is the value that can be found in the App's [configuration artifact](../../rest/v1beta1/projects.webApps/getConfig). If `api_key_id` is not set in requests to [`webApps.Create`](../../rest/v1beta1/projects.webApps/create), then Firebase automatically associates an `api_key_id` with the `WebApp`. This auto-associated key may be an existing valid key or, if no valid key exists, a new one will be provisioned. In patch requests, `api_key_id` cannot be set to an empty value, and the new UID must have no restrictions or only have restrictions that are valid for the associated `WebApp`. We recommend using the [Google Cloud Console](https://console.cloud.google.com/apis/credentials) to manage API keys. |
| `name` | String | The resource name of the WebApp, in the format: projects/PROJECT_IDENTIFIER /webApps/APP_ID * PROJECT_IDENTIFIER: the parent Project's [`ProjectNumber`](../projects#FirebaseProject.FIELDS.project_number) ***(recommended)*** or its [`ProjectId`](../projects#FirebaseProject.FIELDS.project_id). Learn more about using project identifiers in Google's [AIP 2510 standard](https://google.aip.dev/cloud/2510). Note that the value for PROJECT_IDENTIFIER in any response body will be the `ProjectId`. * APP_ID: the globally unique, Firebase-assigned identifier for the App (see [`appId`](../projects.webApps#WebApp.FIELDS.app_id)). |
| `project_id` | String | Output only. Immutable. A user-assigned unique identifier of the parent FirebaseProject for the `WebApp`. |
| `app_id` | String | Output only. Immutable. The globally unique, Firebase-assigned identifier for the `WebApp`. This identifier should be treated as an opaque token, as the data format is not specified. |
| `display_name` | String | The user-assigned display name for the `WebApp`. |
| `web_id` | String | Output only. Immutable. A unique, Firebase-assigned identifier for the `WebApp`. This identifier is only used to populate the `namespace` value for the `WebApp`. For most use cases, use `appId` to identify or reference the App. The `webId` value is only unique within a `FirebaseProject` and its associated Apps. |
| `etag` | String | This checksum is computed by the server based on the value of other fields, and it may be sent with update requests to ensure the client has an up-to-date value before proceeding. Learn more about `etag` in Google's [AIP-154 standard](https://google.aip.dev/154#declarative-friendly-resources). This etag is strongly validated. |
| `expire_time` | String | Output only. If the App has been removed from the Project, this is the timestamp of when the App is considered expired and will be permanently deleted. After this time, the App cannot be undeleted (that is, restored to the Project). This value is only provided if the App is in the `DELETED` state. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create web_app
web_app = provider.firebase_api.Web_app {
    parent = "value"  # The resource name of the parent FirebaseProject in which to create a WebApp, in the format: projects/PROJECT_IDENTIFIER/webApps Refer to the `FirebaseProject` [`name`](../projects#FirebaseProject.FIELDS.name) field for details about PROJECT_IDENTIFIER values.
}

# Access web_app outputs
web_app_id = web_app.id
web_app_app_urls = web_app.app_urls
web_app_state = web_app.state
web_app_api_key_id = web_app.api_key_id
web_app_name = web_app.name
web_app_project_id = web_app.project_id
web_app_app_id = web_app.app_id
web_app_display_name = web_app.display_name
web_app_web_id = web_app.web_id
web_app_etag = web_app.etag
web_app_expire_time = web_app.expire_time
```

---


### Available_location

**DECOMMISSIONED.** **If called, this endpoint will return a 404 error.** _Instead, use the applicable resource-specific REST API (or associated documentation, as needed) to determine valid locations for each resource used in your Project._ Lists the valid ["locations for default Google Cloud resources"](https://firebase.google.com/docs/projects/locations#default-cloud-location) for the specified Project (including a FirebaseProject). One of these locations can be selected as the Project's location for default Google Cloud resources, which is the geographical location where the Project's resources associated with Google App Engine (such as the default Cloud Firestore instance) will be provisioned by default. However, if the location for default Google Cloud resources has already been set for the Project, then this setting cannot be changed. This call checks for any possible [location restrictions](https://cloud.google.com/resource-manager/docs/organization-policy/defining-locations) for the specified Project and, thus, might return a subset of all possible locations. To list all locations (regardless of any restrictions), call the endpoint without specifying a unique project identifier (that is, `/v1beta1/{parent=projects/-}/listAvailableLocations`). To call `ListAvailableLocations` with a specified project, a member must be at minimum a Viewer of the Project. Calls without a specified project do not require any specific project permissions.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `locations` | Vec<String> | One page of results from a call to `ListAvailableLocations`. |
| `next_page_token` | String | If the result list is too large to fit in a single response, then a token is returned. If the string is empty, then this response is the last page of results and all available locations have been listed. This token can be used in a subsequent call to `ListAvailableLocations` to find more locations. Page tokens are short-lived and should not be persisted. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access available_location outputs
available_location_id = available_location.id
available_location_locations = available_location.locations
available_location_next_page_token = available_location.next_page_token
```

---


### Project

Unlinks the specified FirebaseProject from its Google Analytics account. This call removes the association of the specified `FirebaseProject` with its current Google Analytics property. However, this call does not delete the Google Analytics resources, such as the Google Analytics property or any data streams. These resources may be re-associated later to the `FirebaseProject` by calling [`AddGoogleAnalytics`](../../v1beta1/projects/addGoogleAnalytics) and specifying the same `analyticsPropertyId`. For Android Apps and iOS Apps, this call re-links data streams with their corresponding apps. However, for Web Apps, this call provisions a *new* data stream for each Web App. To call `RemoveAnalytics`, a project member must be an Owner for the `FirebaseProject`.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `analytics_property_id` | String |  | Optional. The ID of the Google Analytics property associated with the specified `FirebaseProject`. - If not set, then the Google Analytics property that is currently associated with the specified `FirebaseProject` is removed. - If set, and the specified `FirebaseProject` is currently associated with a *different* Google Analytics property, then the response is a `412 Precondition Failed` error.  |
| `parent` | String | ✅ | The resource name of the FirebaseProject to unlink from its Google Analytics account, in the format: projects/PROJECT_IDENTIFIER Refer to the `FirebaseProject` [`name`](../projects#FirebaseProject.FIELDS.name) field for details about PROJECT_IDENTIFIER values. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `annotations` | HashMap<String, String> | A set of user-defined annotations for the FirebaseProject. Learn more about annotations in Google's [AIP-128 standard](https://google.aip.dev/128#annotations). These annotations are intended solely for developers and client-side tools. Firebase services will not mutate this annotations set. |
| `project_id` | String | Output only. Immutable. A user-assigned unique identifier for the Project. This identifier may appear in URLs or names for some Firebase resources associated with the Project, but it should generally be treated as a convenience alias to reference the Project. |
| `resources` | String | Output only. **DEPRECATED.** _Auto-provisioning of these resources is changing, so this object no longer reliably provides information about the Project. Instead, retrieve information about each resource directly from its resource-specific API._ The default Firebase resources associated with the Project. |
| `display_name` | String | The user-assigned display name of the Project. |
| `state` | String | Output only. The lifecycle state of the Project. |
| `etag` | String | This checksum is computed by the server based on the value of other fields, and it may be sent with update requests to ensure the client has an up-to-date value before proceeding. Learn more about `etag` in Google's [AIP-154 standard](https://google.aip.dev/154#declarative-friendly-resources). This etag is strongly validated. |
| `project_number` | String | Output only. Immutable. The globally unique, Google-assigned canonical identifier for the Project. Use this identifier when configuring integrations and/or making API calls to Firebase or third-party services. |
| `name` | String | The resource name of the Project, in the format: projects/PROJECT_IDENTIFIER PROJECT_IDENTIFIER: the Project's [`ProjectNumber`](../projects#FirebaseProject.FIELDS.project_number) ***(recommended)*** or its [`ProjectId`](../projects#FirebaseProject.FIELDS.project_id). Learn more about using project identifiers in Google's [AIP 2510 standard](https://google.aip.dev/cloud/2510). Note that the value for PROJECT_IDENTIFIER in any response body will be the `ProjectId`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create project
project = provider.firebase_api.Project {
    parent = "value"  # The resource name of the FirebaseProject to unlink from its Google Analytics account, in the format: projects/PROJECT_IDENTIFIER Refer to the `FirebaseProject` [`name`](../projects#FirebaseProject.FIELDS.name) field for details about PROJECT_IDENTIFIER values.
}

# Access project outputs
project_id = project.id
project_annotations = project.annotations
project_project_id = project.project_id
project_resources = project.resources
project_display_name = project.display_name
project_state = project.state
project_etag = project.etag
project_project_number = project.project_number
project_name = project.name
```

---


### Sha

Adds a ShaCertificate to the specified AndroidApp.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `cert_type` | String |  | The type of SHA certificate encoded in the hash. |
| `sha_hash` | String |  | The certificate hash for the `AndroidApp`. |
| `name` | String |  | The resource name of the ShaCertificate for the AndroidApp, in the format: projects/PROJECT_IDENTIFIER/androidApps/APP_ID/sha/SHA_HASH * PROJECT_IDENTIFIER: the parent Project's [`ProjectNumber`](../projects#FirebaseProject.FIELDS.project_number) ***(recommended)*** or its [`ProjectId`](../projects#FirebaseProject.FIELDS.project_id). Learn more about using project identifiers in Google's [AIP 2510 standard](https://google.aip.dev/cloud/2510). Note that the value for PROJECT_IDENTIFIER in any response body will be the `ProjectId`. * APP_ID: the globally unique, Firebase-assigned identifier for the App (see [`appId`](../projects.androidApps#AndroidApp.FIELDS.app_id)). * SHA_HASH: the certificate hash for the App (see [`shaHash`](../projects.androidApps.sha#ShaCertificate.FIELDS.sha_hash)). |
| `parent` | String | ✅ | The resource name of the parent AndroidApp to which to add a ShaCertificate, in the format: projects/PROJECT_IDENTIFIER/androidApps/ APP_ID Since an APP_ID is a unique identifier, the Unique Resource from Sub-Collection access pattern may be used here, in the format: projects/-/androidApps/APP_ID Refer to the `AndroidApp` [`name`](../projects.androidApps#AndroidApp.FIELDS.name) field for details about PROJECT_IDENTIFIER and APP_ID values. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `certificates` | Vec<String> | The list of each `ShaCertificate` associated with the `AndroidApp`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create sha
sha = provider.firebase_api.Sha {
    parent = "value"  # The resource name of the parent AndroidApp to which to add a ShaCertificate, in the format: projects/PROJECT_IDENTIFIER/androidApps/ APP_ID Since an APP_ID is a unique identifier, the Unique Resource from Sub-Collection access pattern may be used here, in the format: projects/-/androidApps/APP_ID Refer to the `AndroidApp` [`name`](../projects.androidApps#AndroidApp.FIELDS.name) field for details about PROJECT_IDENTIFIER and APP_ID values.
}

# Access sha outputs
sha_id = sha.id
sha_certificates = sha.certificates
```

---


### Android_app

Requests the creation of a new AndroidApp in the specified FirebaseProject. The result of this call is an `Operation` which can be used to track the provisioning process. The `Operation` is automatically deleted after completion, so there is no need to call `DeleteOperation`.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | The user-assigned display name for the `AndroidApp`. |
| `sha1_hashes` | Vec<String> |  | The SHA1 certificate hashes for the AndroidApp. |
| `name` | String |  | The resource name of the AndroidApp, in the format: projects/ PROJECT_IDENTIFIER/androidApps/APP_ID * PROJECT_IDENTIFIER: the parent Project's [`ProjectNumber`](../projects#FirebaseProject.FIELDS.project_number) ***(recommended)*** or its [`ProjectId`](../projects#FirebaseProject.FIELDS.project_id). Learn more about using project identifiers in Google's [AIP 2510 standard](https://google.aip.dev/cloud/2510). Note that the value for PROJECT_IDENTIFIER in any response body will be the `ProjectId`. * APP_ID: the globally unique, Firebase-assigned identifier for the App (see [`appId`](../projects.androidApps#AndroidApp.FIELDS.app_id)). |
| `project_id` | String |  | Output only. Immutable. A user-assigned unique identifier of the parent FirebaseProject for the `AndroidApp`. |
| `app_id` | String |  | Output only. Immutable. The globally unique, Firebase-assigned identifier for the `AndroidApp`. This identifier should be treated as an opaque token, as the data format is not specified. |
| `package_name` | String |  | Immutable. The canonical package name of the Android app as would appear in the Google Play Developer Console. |
| `state` | String |  | Output only. The lifecycle state of the App. |
| `etag` | String |  | This checksum is computed by the server based on the value of other fields, and it may be sent with update requests to ensure the client has an up-to-date value before proceeding. Learn more about `etag` in Google's [AIP-154 standard](https://google.aip.dev/154#declarative-friendly-resources). This etag is strongly validated. |
| `expire_time` | String |  | Output only. If the App has been removed from the Project, this is the timestamp of when the App is considered expired and will be permanently deleted. After this time, the App cannot be undeleted (that is, restored to the Project). This value is only provided if the App is in the `DELETED` state. |
| `sha256_hashes` | Vec<String> |  | The SHA256 certificate hashes for the AndroidApp. |
| `api_key_id` | String |  | The globally unique, Google-assigned identifier (UID) for the Firebase API key associated with the `AndroidApp`. Be aware that this value is the UID of the API key, _not_ the [`keyString`](https://cloud.google.com/api-keys/docs/reference/rest/v2/projects.locations.keys#Key.FIELDS.key_string) of the API key. The `keyString` is the value that can be found in the App's [configuration artifact](../../rest/v1beta1/projects.androidApps/getConfig). If `api_key_id` is not set in requests to [`androidApps.Create`](../../rest/v1beta1/projects.androidApps/create), then Firebase automatically associates an `api_key_id` with the `AndroidApp`. This auto-associated key may be an existing valid key or, if no valid key exists, a new one will be provisioned. In patch requests, `api_key_id` cannot be set to an empty value, and the new UID must have no restrictions or only have restrictions that are valid for the associated `AndroidApp`. We recommend using the [Google Cloud Console](https://console.cloud.google.com/apis/credentials) to manage API keys. |
| `parent` | String | ✅ | The resource name of the parent FirebaseProject in which to create an AndroidApp, in the format: projects/PROJECT_IDENTIFIER/androidApps Refer to the `FirebaseProject` [`name`](../projects#FirebaseProject.FIELDS.name) field for details about PROJECT_IDENTIFIER values. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | The user-assigned display name for the `AndroidApp`. |
| `sha1_hashes` | Vec<String> | The SHA1 certificate hashes for the AndroidApp. |
| `name` | String | The resource name of the AndroidApp, in the format: projects/ PROJECT_IDENTIFIER/androidApps/APP_ID * PROJECT_IDENTIFIER: the parent Project's [`ProjectNumber`](../projects#FirebaseProject.FIELDS.project_number) ***(recommended)*** or its [`ProjectId`](../projects#FirebaseProject.FIELDS.project_id). Learn more about using project identifiers in Google's [AIP 2510 standard](https://google.aip.dev/cloud/2510). Note that the value for PROJECT_IDENTIFIER in any response body will be the `ProjectId`. * APP_ID: the globally unique, Firebase-assigned identifier for the App (see [`appId`](../projects.androidApps#AndroidApp.FIELDS.app_id)). |
| `project_id` | String | Output only. Immutable. A user-assigned unique identifier of the parent FirebaseProject for the `AndroidApp`. |
| `app_id` | String | Output only. Immutable. The globally unique, Firebase-assigned identifier for the `AndroidApp`. This identifier should be treated as an opaque token, as the data format is not specified. |
| `package_name` | String | Immutable. The canonical package name of the Android app as would appear in the Google Play Developer Console. |
| `state` | String | Output only. The lifecycle state of the App. |
| `etag` | String | This checksum is computed by the server based on the value of other fields, and it may be sent with update requests to ensure the client has an up-to-date value before proceeding. Learn more about `etag` in Google's [AIP-154 standard](https://google.aip.dev/154#declarative-friendly-resources). This etag is strongly validated. |
| `expire_time` | String | Output only. If the App has been removed from the Project, this is the timestamp of when the App is considered expired and will be permanently deleted. After this time, the App cannot be undeleted (that is, restored to the Project). This value is only provided if the App is in the `DELETED` state. |
| `sha256_hashes` | Vec<String> | The SHA256 certificate hashes for the AndroidApp. |
| `api_key_id` | String | The globally unique, Google-assigned identifier (UID) for the Firebase API key associated with the `AndroidApp`. Be aware that this value is the UID of the API key, _not_ the [`keyString`](https://cloud.google.com/api-keys/docs/reference/rest/v2/projects.locations.keys#Key.FIELDS.key_string) of the API key. The `keyString` is the value that can be found in the App's [configuration artifact](../../rest/v1beta1/projects.androidApps/getConfig). If `api_key_id` is not set in requests to [`androidApps.Create`](../../rest/v1beta1/projects.androidApps/create), then Firebase automatically associates an `api_key_id` with the `AndroidApp`. This auto-associated key may be an existing valid key or, if no valid key exists, a new one will be provisioned. In patch requests, `api_key_id` cannot be set to an empty value, and the new UID must have no restrictions or only have restrictions that are valid for the associated `AndroidApp`. We recommend using the [Google Cloud Console](https://console.cloud.google.com/apis/credentials) to manage API keys. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create android_app
android_app = provider.firebase_api.Android_app {
    parent = "value"  # The resource name of the parent FirebaseProject in which to create an AndroidApp, in the format: projects/PROJECT_IDENTIFIER/androidApps Refer to the `FirebaseProject` [`name`](../projects#FirebaseProject.FIELDS.name) field for details about PROJECT_IDENTIFIER values.
}

# Access android_app outputs
android_app_id = android_app.id
android_app_display_name = android_app.display_name
android_app_sha1_hashes = android_app.sha1_hashes
android_app_name = android_app.name
android_app_project_id = android_app.project_id
android_app_app_id = android_app.app_id
android_app_package_name = android_app.package_name
android_app_state = android_app.state
android_app_etag = android_app.etag
android_app_expire_time = android_app.expire_time
android_app_sha256_hashes = android_app.sha256_hashes
android_app_api_key_id = android_app.api_key_id
```

---


### Default_location

**DECOMMISSIONED.** **If called, this endpoint will return a 404 error.** _Instead, use the applicable resource-specific REST API to set the location for each resource used in your Project._ Sets the ["location for default Google Cloud resources"](https://firebase.google.com/docs/projects/locations#default-cloud-location) for the specified FirebaseProject. This method creates a Google App Engine application with a [default Cloud Storage bucket](https://cloud.google.com/appengine/docs/standard/python/googlecloudstorageclient/setting-up-cloud-storage#activating_a_cloud_storage_bucket), located in the specified [`locationId`](#body.request_body.FIELDS.location_id). This location must be one of the available [App Engine locations](https://cloud.google.com/about/locations#region). After the location for default Google Cloud resources is finalized, or if it was already set, it cannot be changed. The location for default Google Cloud resources for the specified `FirebaseProject` might already be set because either the underlying Google Cloud `Project` already has an App Engine application or `FinalizeDefaultLocation` was previously called with a specified `locationId`. The result of this call is an [`Operation`](../../v1beta1/operations), which can be used to track the provisioning process. The [`response`](../../v1beta1/operations#Operation.FIELDS.response) type of the `Operation` is google.protobuf.Empty. The `Operation` can be polled by its `name` using GetOperation until `done` is true. When `done` is true, the `Operation` has either succeeded or failed. If the `Operation` has succeeded, its [`response`](../../v1beta1/operations#Operation.FIELDS.response) will be set to a google.protobuf.Empty; if the `Operation` has failed, its `error` will be set to a google.rpc.Status. The `Operation` is automatically deleted after completion, so there is no need to call DeleteOperation. All fields listed in the [request body](#request-body) are required. To call `FinalizeDefaultLocation`, a member must be an Owner of the Project.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `location_id` | String |  | **DEPRECATED** The ID of the Project's ["location for default Google Cloud resources"](https://firebase.google.com/docs/projects/locations#default-cloud-location), which are resources associated with Google App Engine. The location must be one of the available [Google App Engine locations](https://cloud.google.com/about/locations#region). |
| `parent` | String | ✅ | The resource name of the FirebaseProject for which the ["location for default Google Cloud resources"](https://firebase.google.com/docs/projects/locations#default-cloud-location) will be set, in the format: projects/PROJECT_IDENTIFIER Refer to the `FirebaseProject` [`name`](../projects#FirebaseProject.FIELDS.name) field for details about PROJECT_IDENTIFIER values. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create default_location
default_location = provider.firebase_api.Default_location {
    parent = "value"  # The resource name of the FirebaseProject for which the ["location for default Google Cloud resources"](https://firebase.google.com/docs/projects/locations#default-cloud-location) will be set, in the format: projects/PROJECT_IDENTIFIER Refer to the `FirebaseProject` [`name`](../projects#FirebaseProject.FIELDS.name) field for details about PROJECT_IDENTIFIER values.
}

```

---


### Ios_app

Requests the creation of a new IosApp in the specified FirebaseProject. The result of this call is an `Operation` which can be used to track the provisioning process. The `Operation` is automatically deleted after completion, so there is no need to call `DeleteOperation`.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `team_id` | String |  | The Apple Developer Team ID associated with the App in the App Store. |
| `expire_time` | String |  | Output only. If the App has been removed from the Project, this is the timestamp of when the App is considered expired and will be permanently deleted. After this time, the App cannot be undeleted (that is, restored to the Project). This value is only provided if the App is in the `DELETED` state. |
| `etag` | String |  | This checksum is computed by the server based on the value of other fields, and it may be sent with update requests to ensure the client has an up-to-date value before proceeding. Learn more about `etag` in Google's [AIP-154 standard](https://google.aip.dev/154#declarative-friendly-resources). This etag is strongly validated. |
| `display_name` | String |  | The user-assigned display name for the `IosApp`. |
| `project_id` | String |  | Output only. Immutable. A user-assigned unique identifier of the parent FirebaseProject for the `IosApp`. |
| `bundle_id` | String |  | Immutable. The canonical bundle ID of the iOS app as it would appear in the iOS AppStore. |
| `app_id` | String |  | Output only. Immutable. The globally unique, Firebase-assigned identifier for the `IosApp`. This identifier should be treated as an opaque token, as the data format is not specified. |
| `api_key_id` | String |  | The globally unique, Google-assigned identifier (UID) for the Firebase API key associated with the `IosApp`. Be aware that this value is the UID of the API key, _not_ the [`keyString`](https://cloud.google.com/api-keys/docs/reference/rest/v2/projects.locations.keys#Key.FIELDS.key_string) of the API key. The `keyString` is the value that can be found in the App's [configuration artifact](../../rest/v1beta1/projects.iosApps/getConfig). If `api_key_id` is not set in requests to [`iosApps.Create`](../../rest/v1beta1/projects.iosApps/create), then Firebase automatically associates an `api_key_id` with the `IosApp`. This auto-associated key may be an existing valid key or, if no valid key exists, a new one will be provisioned. In patch requests, `api_key_id` cannot be set to an empty value, and the new UID must have no restrictions or only have restrictions that are valid for the associated `IosApp`. We recommend using the [Google Cloud Console](https://console.cloud.google.com/apis/credentials) to manage API keys. |
| `app_store_id` | String |  | The automatically generated Apple ID assigned to the iOS app by Apple in the iOS App Store. |
| `name` | String |  | The resource name of the IosApp, in the format: projects/PROJECT_IDENTIFIER /iosApps/APP_ID * PROJECT_IDENTIFIER: the parent Project's [`ProjectNumber`](../projects#FirebaseProject.FIELDS.project_number) ***(recommended)*** or its [`ProjectId`](../projects#FirebaseProject.FIELDS.project_id). Learn more about using project identifiers in Google's [AIP 2510 standard](https://google.aip.dev/cloud/2510). Note that the value for PROJECT_IDENTIFIER in any response body will be the `ProjectId`. * APP_ID: the globally unique, Firebase-assigned identifier for the App (see [`appId`](../projects.iosApps#IosApp.FIELDS.app_id)). |
| `state` | String |  | Output only. The lifecycle state of the App. |
| `parent` | String | ✅ | The resource name of the parent FirebaseProject in which to create an IosApp, in the format: projects/PROJECT_IDENTIFIER/iosApps Refer to the `FirebaseProject` [`name`](../projects#FirebaseProject.FIELDS.name) field for details about PROJECT_IDENTIFIER values. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `team_id` | String | The Apple Developer Team ID associated with the App in the App Store. |
| `expire_time` | String | Output only. If the App has been removed from the Project, this is the timestamp of when the App is considered expired and will be permanently deleted. After this time, the App cannot be undeleted (that is, restored to the Project). This value is only provided if the App is in the `DELETED` state. |
| `etag` | String | This checksum is computed by the server based on the value of other fields, and it may be sent with update requests to ensure the client has an up-to-date value before proceeding. Learn more about `etag` in Google's [AIP-154 standard](https://google.aip.dev/154#declarative-friendly-resources). This etag is strongly validated. |
| `display_name` | String | The user-assigned display name for the `IosApp`. |
| `project_id` | String | Output only. Immutable. A user-assigned unique identifier of the parent FirebaseProject for the `IosApp`. |
| `bundle_id` | String | Immutable. The canonical bundle ID of the iOS app as it would appear in the iOS AppStore. |
| `app_id` | String | Output only. Immutable. The globally unique, Firebase-assigned identifier for the `IosApp`. This identifier should be treated as an opaque token, as the data format is not specified. |
| `api_key_id` | String | The globally unique, Google-assigned identifier (UID) for the Firebase API key associated with the `IosApp`. Be aware that this value is the UID of the API key, _not_ the [`keyString`](https://cloud.google.com/api-keys/docs/reference/rest/v2/projects.locations.keys#Key.FIELDS.key_string) of the API key. The `keyString` is the value that can be found in the App's [configuration artifact](../../rest/v1beta1/projects.iosApps/getConfig). If `api_key_id` is not set in requests to [`iosApps.Create`](../../rest/v1beta1/projects.iosApps/create), then Firebase automatically associates an `api_key_id` with the `IosApp`. This auto-associated key may be an existing valid key or, if no valid key exists, a new one will be provisioned. In patch requests, `api_key_id` cannot be set to an empty value, and the new UID must have no restrictions or only have restrictions that are valid for the associated `IosApp`. We recommend using the [Google Cloud Console](https://console.cloud.google.com/apis/credentials) to manage API keys. |
| `app_store_id` | String | The automatically generated Apple ID assigned to the iOS app by Apple in the iOS App Store. |
| `name` | String | The resource name of the IosApp, in the format: projects/PROJECT_IDENTIFIER /iosApps/APP_ID * PROJECT_IDENTIFIER: the parent Project's [`ProjectNumber`](../projects#FirebaseProject.FIELDS.project_number) ***(recommended)*** or its [`ProjectId`](../projects#FirebaseProject.FIELDS.project_id). Learn more about using project identifiers in Google's [AIP 2510 standard](https://google.aip.dev/cloud/2510). Note that the value for PROJECT_IDENTIFIER in any response body will be the `ProjectId`. * APP_ID: the globally unique, Firebase-assigned identifier for the App (see [`appId`](../projects.iosApps#IosApp.FIELDS.app_id)). |
| `state` | String | Output only. The lifecycle state of the App. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create ios_app
ios_app = provider.firebase_api.Ios_app {
    parent = "value"  # The resource name of the parent FirebaseProject in which to create an IosApp, in the format: projects/PROJECT_IDENTIFIER/iosApps Refer to the `FirebaseProject` [`name`](../projects#FirebaseProject.FIELDS.name) field for details about PROJECT_IDENTIFIER values.
}

# Access ios_app outputs
ios_app_id = ios_app.id
ios_app_team_id = ios_app.team_id
ios_app_expire_time = ios_app.expire_time
ios_app_etag = ios_app.etag
ios_app_display_name = ios_app.display_name
ios_app_project_id = ios_app.project_id
ios_app_bundle_id = ios_app.bundle_id
ios_app_app_id = ios_app.app_id
ios_app_api_key_id = ios_app.api_key_id
ios_app_app_store_id = ios_app.app_store_id
ios_app_name = ios_app.name
ios_app_state = ios_app.state
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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |


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
operation_done = operation.done
operation_metadata = operation.metadata
operation_name = operation.name
operation_response = operation.response
```

---


### Available_project

Lists each [Google Cloud `Project`](https://cloud.google.com/resource-manager/reference/rest/v1/projects) that can have Firebase resources added and Firebase services enabled. A Project will only be listed if: - The caller has sufficient [Google IAM](https://cloud.google.com/iam) permissions to call AddFirebase. - The Project is not already a FirebaseProject. - The Project is not in an Organization which has policies that prevent Firebase resources from being added. 

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | If the result list is too large to fit in a single response, then a token is returned. If the string is empty, then this response is the last page of results. This token can be used in a subsequent calls to `ListAvailableProjects` to find the next group of Projects. Page tokens are short-lived and should not be persisted. |
| `project_info` | Vec<String> | The list of Google Cloud `Projects` which can have Firebase resources added to them. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access available_project outputs
available_project_id = available_project.id
available_project_next_page_token = available_project.next_page_token
available_project_project_info = available_project.project_info
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple web_app resources
web_app_0 = provider.firebase_api.Web_app {
    parent = "value-0"
}
web_app_1 = provider.firebase_api.Web_app {
    parent = "value-1"
}
web_app_2 = provider.firebase_api.Web_app {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    web_app = provider.firebase_api.Web_app {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Firebase_api Documentation](https://cloud.google.com/firebase_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
