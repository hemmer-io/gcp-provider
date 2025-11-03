# Androidenterprise_api Service



**Resources**: 17

---

## Overview

The androidenterprise_api service provides access to 17 resource types:

- [Serviceaccountkey](#serviceaccountkey) [CRD]
- [Managedconfigurationssetting](#managedconfigurationssetting) [R]
- [Permission](#permission) [R]
- [Storelayoutpage](#storelayoutpage) [CRUD]
- [Webapp](#webapp) [CRUD]
- [Storelayoutcluster](#storelayoutcluster) [CRUD]
- [Managedconfigurationsforuser](#managedconfigurationsforuser) [RUD]
- [Enrollment_token](#enrollment_token) [C]
- [Managedconfigurationsfordevice](#managedconfigurationsfordevice) [RUD]
- [Install](#install) [RUD]
- [Enterprise](#enterprise) [CRU]
- [User](#user) [CRUD]
- [Product](#product) [CR]
- [Entitlement](#entitlement) [RUD]
- [Grouplicenseuser](#grouplicenseuser) [R]
- [Grouplicense](#grouplicense) [R]
- [Device](#device) [CRU]

---

## Resources


### Serviceaccountkey

Generates new credentials for the service account associated with this enterprise. The calling service account must have been retrieved by calling Enterprises.GetServiceAccount and must have been set as the enterprise service account by calling Enterprises.SetAccount. Only the type of the key should be populated in the resource to be inserted.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | String |  | The file format of the generated key data. |
| `id` | String |  | An opaque, unique identifier for this ServiceAccountKey. Assigned by the server. |
| `data` | String |  | The body of the private key credentials file, in string format. This is only populated when the ServiceAccountKey is created, and is not stored by Google. |
| `public_data` | String |  | Public key data for the credentials file. This is an X.509 cert. If you are using the googleCredentials key type, this is identical to the cert that can be retrieved by using the X.509 cert url inside of the credentials file. |
| `enterprise_id` | String | ✅ | The ID of the enterprise. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service_account_key` | Vec<String> | The service account credentials. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create serviceaccountkey
serviceaccountkey = provider.androidenterprise_api.Serviceaccountkey {
    enterprise_id = "value"  # The ID of the enterprise.
}

# Access serviceaccountkey outputs
serviceaccountkey_id = serviceaccountkey.id
serviceaccountkey_service_account_key = serviceaccountkey.service_account_key
```

---


### Managedconfigurationssetting

Lists all the managed configurations settings for the specified app.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `managed_configurations_settings` | Vec<String> | A managed configurations settings for an app that may be assigned to a group of users in an enterprise. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access managedconfigurationssetting outputs
managedconfigurationssetting_id = managedconfigurationssetting.id
managedconfigurationssetting_managed_configurations_settings = managedconfigurationssetting.managed_configurations_settings
```

---


### Permission

Retrieves details of an Android app permission for display to an enterprise admin.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `permission_id` | String | An opaque string uniquely identifying the permission. |
| `name` | String | The name of the permission. |
| `description` | String | A longer description of the Permissions resource, giving more details of what it affects. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access permission outputs
permission_id = permission.id
permission_permission_id = permission.permission_id
permission_name = permission.name
permission_description = permission.description
```

---


### Storelayoutpage

Inserts a new store page.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `link` | Vec<String> |  | Ordered list of pages a user should be able to reach from this page. The list can't include this page. It is recommended that the basic pages are created first, before adding the links between pages. The API doesn't verify that the pages exist or the pages are reachable. |
| `name` | Vec<String> |  | Ordered list of localized strings giving the name of this page. The text displayed is the one that best matches the user locale, or the first entry if there is no good match. There needs to be at least one entry. |
| `id` | String |  | Unique ID of this page. Assigned by the server. Immutable once assigned. |
| `enterprise_id` | String | ✅ | The ID of the enterprise. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `link` | Vec<String> | Ordered list of pages a user should be able to reach from this page. The list can't include this page. It is recommended that the basic pages are created first, before adding the links between pages. The API doesn't verify that the pages exist or the pages are reachable. |
| `name` | Vec<String> | Ordered list of localized strings giving the name of this page. The text displayed is the one that best matches the user locale, or the first entry if there is no good match. There needs to be at least one entry. |
| `id` | String | Unique ID of this page. Assigned by the server. Immutable once assigned. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create storelayoutpage
storelayoutpage = provider.androidenterprise_api.Storelayoutpage {
    enterprise_id = "value"  # The ID of the enterprise.
}

# Access storelayoutpage outputs
storelayoutpage_id = storelayoutpage.id
storelayoutpage_link = storelayoutpage.link
storelayoutpage_name = storelayoutpage.name
storelayoutpage_id = storelayoutpage.id
```

---


### Webapp

Creates a new web app for the enterprise.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `web_app_id` | String |  | The ID of the application. A string of the form "app:<package name>" where the package name always starts with the prefix "com.google.enterprise.webapp." followed by a random id. |
| `start_url` | String |  | The start URL, i.e. the URL that should load when the user opens the application. |
| `is_published` | bool |  | A flag whether the app has been published to the Play store yet. |
| `icons` | Vec<String> |  | A list of icons representing this website. If absent, a default icon (for create) or the current icon (for update) will be used. |
| `display_mode` | String |  | The display mode of the web app. Possible values include: - "minimalUi", the device's status bar, navigation bar, the app's URL, and a refresh button are visible when the app is open. For HTTP URLs, you can only select this option. - "standalone", the device's status bar and navigation bar are visible when the app is open. - "fullScreen", the app opens in full screen mode, hiding the device's status and navigation bars. All browser UI elements, page URL, system status bar and back button are not visible, and the web app takes up the entirety of the available display area.  |
| `version_code` | String |  | The current version of the app. Note that the version can automatically increase during the lifetime of the web app, while Google does internal housekeeping to keep the web app up-to-date. |
| `title` | String |  | The title of the web app as displayed to the user (e.g., amongst a list of other applications, or as a label for an icon). |
| `enterprise_id` | String | ✅ | The ID of the enterprise. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `web_app_id` | String | The ID of the application. A string of the form "app:<package name>" where the package name always starts with the prefix "com.google.enterprise.webapp." followed by a random id. |
| `start_url` | String | The start URL, i.e. the URL that should load when the user opens the application. |
| `is_published` | bool | A flag whether the app has been published to the Play store yet. |
| `icons` | Vec<String> | A list of icons representing this website. If absent, a default icon (for create) or the current icon (for update) will be used. |
| `display_mode` | String | The display mode of the web app. Possible values include: - "minimalUi", the device's status bar, navigation bar, the app's URL, and a refresh button are visible when the app is open. For HTTP URLs, you can only select this option. - "standalone", the device's status bar and navigation bar are visible when the app is open. - "fullScreen", the app opens in full screen mode, hiding the device's status and navigation bars. All browser UI elements, page URL, system status bar and back button are not visible, and the web app takes up the entirety of the available display area.  |
| `version_code` | String | The current version of the app. Note that the version can automatically increase during the lifetime of the web app, while Google does internal housekeeping to keep the web app up-to-date. |
| `title` | String | The title of the web app as displayed to the user (e.g., amongst a list of other applications, or as a label for an icon). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create webapp
webapp = provider.androidenterprise_api.Webapp {
    enterprise_id = "value"  # The ID of the enterprise.
}

# Access webapp outputs
webapp_id = webapp.id
webapp_web_app_id = webapp.web_app_id
webapp_start_url = webapp.start_url
webapp_is_published = webapp.is_published
webapp_icons = webapp.icons
webapp_display_mode = webapp.display_mode
webapp_version_code = webapp.version_code
webapp_title = webapp.title
```

---


### Storelayoutcluster

Inserts a new cluster in a page.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `order_in_page` | String |  | String (US-ASCII only) used to determine order of this cluster within the parent page's elements. Page elements are sorted in lexicographic order of this field. Duplicated values are allowed, but ordering between elements with duplicate order is undefined. The value of this field is never visible to a user, it is used solely for the purpose of defining an ordering. Maximum length is 256 characters. |
| `product_id` | Vec<String> |  | List of products in the order they are displayed in the cluster. There should not be duplicates within a cluster. |
| `id` | String |  | Unique ID of this cluster. Assigned by the server. Immutable once assigned. |
| `name` | Vec<String> |  | Ordered list of localized strings giving the name of this page. The text displayed is the one that best matches the user locale, or the first entry if there is no good match. There needs to be at least one entry. |
| `page_id` | String | ✅ | The ID of the page. |
| `enterprise_id` | String | ✅ | The ID of the enterprise. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `order_in_page` | String | String (US-ASCII only) used to determine order of this cluster within the parent page's elements. Page elements are sorted in lexicographic order of this field. Duplicated values are allowed, but ordering between elements with duplicate order is undefined. The value of this field is never visible to a user, it is used solely for the purpose of defining an ordering. Maximum length is 256 characters. |
| `product_id` | Vec<String> | List of products in the order they are displayed in the cluster. There should not be duplicates within a cluster. |
| `id` | String | Unique ID of this cluster. Assigned by the server. Immutable once assigned. |
| `name` | Vec<String> | Ordered list of localized strings giving the name of this page. The text displayed is the one that best matches the user locale, or the first entry if there is no good match. There needs to be at least one entry. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create storelayoutcluster
storelayoutcluster = provider.androidenterprise_api.Storelayoutcluster {
    page_id = "value"  # The ID of the page.
    enterprise_id = "value"  # The ID of the enterprise.
}

# Access storelayoutcluster outputs
storelayoutcluster_id = storelayoutcluster.id
storelayoutcluster_order_in_page = storelayoutcluster.order_in_page
storelayoutcluster_product_id = storelayoutcluster.product_id
storelayoutcluster_id = storelayoutcluster.id
storelayoutcluster_name = storelayoutcluster.name
```

---


### Managedconfigurationsforuser

Retrieves details of a per-user managed configuration for an app for the specified user.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  | Deprecated. |
| `product_id` | String |  | The ID of the product that the managed configuration is for, e.g. "app:com.google.android.gm". |
| `managed_property` | Vec<String> |  | The set of managed properties for this configuration. |
| `configuration_variables` | String |  | Contains the ID of the managed configuration profile and the set of configuration variables (if any) defined for the user. |
| `enterprise_id` | String | ✅ | The ID of the enterprise. |
| `user_id` | String | ✅ | The ID of the user. |
| `managed_configuration_for_user_id` | String | ✅ | The ID of the managed configuration (a product ID), e.g. "app:com.google.android.gm". |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Deprecated. |
| `product_id` | String | The ID of the product that the managed configuration is for, e.g. "app:com.google.android.gm". |
| `managed_property` | Vec<String> | The set of managed properties for this configuration. |
| `configuration_variables` | String | Contains the ID of the managed configuration profile and the set of configuration variables (if any) defined for the user. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access managedconfigurationsforuser outputs
managedconfigurationsforuser_id = managedconfigurationsforuser.id
managedconfigurationsforuser_kind = managedconfigurationsforuser.kind
managedconfigurationsforuser_product_id = managedconfigurationsforuser.product_id
managedconfigurationsforuser_managed_property = managedconfigurationsforuser.managed_property
managedconfigurationsforuser_configuration_variables = managedconfigurationsforuser.configuration_variables
```

---


### Enrollment_token

Returns a token for device enrollment. The DPC can encode this token within the QR/NFC/zero-touch enrollment payload or fetch it before calling the on-device API to authenticate the user. The token can be generated for each device or reused across multiple devices.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `duration` | String |  | [Optional] The length of time the enrollment token is valid, ranging from 1 minute to [`Durations.MAX_VALUE`](https://developers.google.com/protocol-buffers/docs/reference/java/com/google/protobuf/util/Durations.html#MAX_VALUE), approximately 10,000 years. If not specified, the default duration is 1 hour. |
| `google_authentication_options` | String |  | [Optional] Provides options related to Google authentication during the enrollment. |
| `enrollment_token_type` | String |  | [Required] The type of the enrollment token. |
| `token` | String |  | The token value that's passed to the device and authorizes the device to enroll. This is a read-only field generated by the server. |
| `enterprise_id` | String | ✅ | Required. The ID of the enterprise. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create enrollment_token
enrollment_token = provider.androidenterprise_api.Enrollment_token {
    enterprise_id = "value"  # Required. The ID of the enterprise.
}

```

---


### Managedconfigurationsfordevice

Retrieves details of a per-device managed configuration.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  | Deprecated. |
| `product_id` | String |  | The ID of the product that the managed configuration is for, e.g. "app:com.google.android.gm". |
| `managed_property` | Vec<String> |  | The set of managed properties for this configuration. |
| `configuration_variables` | String |  | Contains the ID of the managed configuration profile and the set of configuration variables (if any) defined for the user. |
| `managed_configuration_for_device_id` | String | ✅ | The ID of the managed configuration (a product ID), e.g. "app:com.google.android.gm". |
| `enterprise_id` | String | ✅ | The ID of the enterprise. |
| `device_id` | String | ✅ | The Android ID of the device. |
| `user_id` | String | ✅ | The ID of the user. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Deprecated. |
| `product_id` | String | The ID of the product that the managed configuration is for, e.g. "app:com.google.android.gm". |
| `managed_property` | Vec<String> | The set of managed properties for this configuration. |
| `configuration_variables` | String | Contains the ID of the managed configuration profile and the set of configuration variables (if any) defined for the user. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access managedconfigurationsfordevice outputs
managedconfigurationsfordevice_id = managedconfigurationsfordevice.id
managedconfigurationsfordevice_kind = managedconfigurationsfordevice.kind
managedconfigurationsfordevice_product_id = managedconfigurationsfordevice.product_id
managedconfigurationsfordevice_managed_property = managedconfigurationsfordevice.managed_property
managedconfigurationsfordevice_configuration_variables = managedconfigurationsfordevice.configuration_variables
```

---


### Install

Retrieves details of an installation of an app on a device.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `install_state` | String |  | Install state. The state "installPending" means that an install request has recently been made and download to the device is in progress. The state "installed" means that the app has been installed. This field is read-only. |
| `version_code` | i64 |  | The version of the installed product. Guaranteed to be set only if the install state is "installed". |
| `product_id` | String |  | The ID of the product that the install is for. For example, "app:com.google.android.gm". |
| `device_id` | String | ✅ | The Android ID of the device. |
| `user_id` | String | ✅ | The ID of the user. |
| `enterprise_id` | String | ✅ | The ID of the enterprise. |
| `install_id` | String | ✅ | The ID of the product represented by the install, e.g. "app:com.google.android.gm". |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `install_state` | String | Install state. The state "installPending" means that an install request has recently been made and download to the device is in progress. The state "installed" means that the app has been installed. This field is read-only. |
| `version_code` | i64 | The version of the installed product. Guaranteed to be set only if the install state is "installed". |
| `product_id` | String | The ID of the product that the install is for. For example, "app:com.google.android.gm". |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access install outputs
install_id = install.id
install_install_state = install.install_state
install_version_code = install.version_code
install_product_id = install.product_id
```

---


### Enterprise

Completes the signup flow, by specifying the Completion token and Enterprise token. This request must not be called multiple times for a given Enterprise Token.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `administrator` | Vec<String> | Admins of the enterprise. This is only supported for enterprises created via the EMM-initiated flow. |
| `primary_domain` | String | The enterprise's primary domain, such as "example.com". |
| `id` | String | The unique ID for the enterprise. |
| `name` | String | The name of the enterprise, for example, "Example, Inc". |
| `managed_google_domain_type` | String | The type of managed Google domain |
| `google_authentication_settings` | String | Output only. Settings for Google-provided user authentication. |
| `enterprise_type` | String | The type of the enterprise. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create enterprise
enterprise = provider.androidenterprise_api.Enterprise {
}

# Access enterprise outputs
enterprise_id = enterprise.id
enterprise_administrator = enterprise.administrator
enterprise_primary_domain = enterprise.primary_domain
enterprise_id = enterprise.id
enterprise_name = enterprise.name
enterprise_managed_google_domain_type = enterprise.managed_google_domain_type
enterprise_google_authentication_settings = enterprise.google_authentication_settings
enterprise_enterprise_type = enterprise.enterprise_type
```

---


### User

Creates a new EMM-managed user. The Users resource passed in the body of the request should include an accountIdentifier and an accountType. If a corresponding user already exists with the same account identifier, the user will be updated with the resource. In this case only the displayName field can be changed.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `account_type` | String |  | The type of account that this user represents. A userAccount can be installed on multiple devices, but a deviceAccount is specific to a single device. An EMM-managed user (emmManaged) can be either type (userAccount, deviceAccount), but a Google-managed user (googleManaged) is always a userAccount. |
| `id` | String |  | The unique ID for the user. |
| `account_identifier` | String |  | A unique identifier you create for this user, such as "user342" or "asset#44418". Do not use personally identifiable information (PII) for this property. Must always be set for EMM-managed users. Not set for Google-managed users. |
| `display_name` | String |  | The name that will appear in user interfaces. Setting this property is optional when creating EMM-managed users. If you do set this property, use something generic about the organization (such as "Example, Inc.") or your name (as EMM). Not used for Google-managed user accounts. @mutable androidenterprise.users.update |
| `management_type` | String |  | The entity that manages the user. With googleManaged users, the source of truth is Google so EMMs have to make sure a Google Account exists for the user. With emmManaged users, the EMM is in charge. |
| `primary_email` | String |  | The user's primary email address, for example, "jsmith@example.com". Will always be set for Google managed users and not set for EMM managed users. |
| `enterprise_id` | String | ✅ | The ID of the enterprise. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `account_type` | String | The type of account that this user represents. A userAccount can be installed on multiple devices, but a deviceAccount is specific to a single device. An EMM-managed user (emmManaged) can be either type (userAccount, deviceAccount), but a Google-managed user (googleManaged) is always a userAccount. |
| `id` | String | The unique ID for the user. |
| `account_identifier` | String | A unique identifier you create for this user, such as "user342" or "asset#44418". Do not use personally identifiable information (PII) for this property. Must always be set for EMM-managed users. Not set for Google-managed users. |
| `display_name` | String | The name that will appear in user interfaces. Setting this property is optional when creating EMM-managed users. If you do set this property, use something generic about the organization (such as "Example, Inc.") or your name (as EMM). Not used for Google-managed user accounts. @mutable androidenterprise.users.update |
| `management_type` | String | The entity that manages the user. With googleManaged users, the source of truth is Google so EMMs have to make sure a Google Account exists for the user. With emmManaged users, the EMM is in charge. |
| `primary_email` | String | The user's primary email address, for example, "jsmith@example.com". Will always be set for Google managed users and not set for EMM managed users. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user
user = provider.androidenterprise_api.User {
    enterprise_id = "value"  # The ID of the enterprise.
}

# Access user outputs
user_id = user.id
user_account_type = user.account_type
user_id = user.id
user_account_identifier = user.account_identifier
user_display_name = user.display_name
user_management_type = user.management_type
user_primary_email = user.primary_email
```

---


### Product

Generates a URL that can be rendered in an iframe to display the permissions (if any) of a product. An enterprise admin must view these permissions and accept them on behalf of their organization in order to approve that product. Admins should accept the displayed permissions by interacting with a separate UI element in the EMM console, which in turn should trigger the use of this URL as the approvalUrlInfo.approvalUrl property in a Products.approve call to approve the product. This URL can only be used to display permissions for up to 1 day. **Note:** This item has been deprecated. New integrations cannot use this method and can refer to our new recommendations. 

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `enterprise_id` | String | ✅ | The ID of the enterprise. |
| `product_id` | String | ✅ | The ID of the product. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `app_restrictions_schema` | String | The app restriction schema |
| `small_icon_url` | String | A link to a smaller image that can be used as an icon for the product. This image is suitable for use at up to 128px x 128px. |
| `author_name` | String | The name of the author of the product (for example, the app developer). |
| `full_description` | String | The localized full app store description, if available. |
| `content_rating` | String | The content rating for this app. |
| `app_tracks` | Vec<String> | The tracks visible to the enterprise. |
| `icon_url` | String | A link to an image that can be used as an icon for the product. This image is suitable for use at up to 512px x 512px. |
| `description` | String | The localized promotional description, if available. |
| `details_url` | String | A link to the (consumer) Google Play details page for the product. |
| `last_updated_timestamp_millis` | String | The approximate time (within 7 days) the app was last published, expressed in milliseconds since epoch. |
| `available_countries` | Vec<String> | The countries which this app is available in. |
| `available_tracks` | Vec<String> | Deprecated, use appTracks instead. |
| `min_android_sdk_version` | i64 | The minimum Android SDK necessary to run the app. |
| `permissions` | Vec<String> | A list of permissions required by the app. |
| `recent_changes` | String | A description of the recent changes made to the app. |
| `features` | Vec<String> | Noteworthy features (if any) of this product. |
| `product_id` | String | A string of the form *app:<package name>*. For example, app:com.google.android.gm represents the Gmail app. |
| `signing_certificate` | String | The certificate used to sign this product. |
| `requires_container_app` | bool | Deprecated. |
| `title` | String | The name of the product. |
| `distribution_channel` | String | How and to whom the package is made available. The value publicGoogleHosted means that the package is available through the Play store and not restricted to a specific enterprise. The value privateGoogleHosted means that the package is a private app (restricted to an enterprise) but hosted by Google. The value privateSelfHosted means that the package is a private app (restricted to an enterprise) and is privately hosted. |
| `category` | String | The app category (e.g. RACING, SOCIAL, etc.) |
| `product_pricing` | String | Whether this product is free, free with in-app purchases, or paid. If the pricing is unknown, this means the product is not generally available anymore (even though it might still be available to people who own it). |
| `app_version` | Vec<String> | App versions currently available for this product. |
| `work_details_url` | String | A link to the managed Google Play details page for the product, for use by an Enterprise admin. |
| `screenshot_urls` | Vec<String> | A list of screenshot links representing the app. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create product
product = provider.androidenterprise_api.Product {
    enterprise_id = "value"  # The ID of the enterprise.
    product_id = "value"  # The ID of the product.
}

# Access product outputs
product_id = product.id
product_app_restrictions_schema = product.app_restrictions_schema
product_small_icon_url = product.small_icon_url
product_author_name = product.author_name
product_full_description = product.full_description
product_content_rating = product.content_rating
product_app_tracks = product.app_tracks
product_icon_url = product.icon_url
product_description = product.description
product_details_url = product.details_url
product_last_updated_timestamp_millis = product.last_updated_timestamp_millis
product_available_countries = product.available_countries
product_available_tracks = product.available_tracks
product_min_android_sdk_version = product.min_android_sdk_version
product_permissions = product.permissions
product_recent_changes = product.recent_changes
product_features = product.features
product_product_id = product.product_id
product_signing_certificate = product.signing_certificate
product_requires_container_app = product.requires_container_app
product_title = product.title
product_distribution_channel = product.distribution_channel
product_category = product.category
product_product_pricing = product.product_pricing
product_app_version = product.app_version
product_work_details_url = product.work_details_url
product_screenshot_urls = product.screenshot_urls
```

---


### Entitlement

Retrieves details of an entitlement. **Note:** This item has been deprecated. New integrations cannot use this method and can refer to our new recommendations.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `product_id` | String |  | The ID of the product that the entitlement is for. For example, "app:com.google.android.gm". |
| `reason` | String |  | The reason for the entitlement. For example, "free" for free apps. This property is temporary: it will be replaced by the acquisition kind field of group licenses. |
| `enterprise_id` | String | ✅ | The ID of the enterprise. |
| `entitlement_id` | String | ✅ | The ID of the entitlement (a product ID), e.g. "app:com.google.android.gm". |
| `user_id` | String | ✅ | The ID of the user. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `product_id` | String | The ID of the product that the entitlement is for. For example, "app:com.google.android.gm". |
| `reason` | String | The reason for the entitlement. For example, "free" for free apps. This property is temporary: it will be replaced by the acquisition kind field of group licenses. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access entitlement outputs
entitlement_id = entitlement.id
entitlement_product_id = entitlement.product_id
entitlement_reason = entitlement.reason
```

---


### Grouplicenseuser

Retrieves the IDs of the users who have been granted entitlements under the license. **Note:** This item has been deprecated. New integrations cannot use this method and can refer to our new recommendations.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `user` | Vec<String> | A user of an enterprise. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access grouplicenseuser outputs
grouplicenseuser_id = grouplicenseuser.id
grouplicenseuser_user = grouplicenseuser.user
```

---


### Grouplicense

Retrieves details of an enterprise's group license for a product. **Note:** This item has been deprecated. New integrations cannot use this method and can refer to our new recommendations.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `product_id` | String | The ID of the product that the license is for. For example, "app:com.google.android.gm". |
| `permissions` | String | The permission approval status of the product. This field is only set if the product is approved. Possible states are: - "currentApproved", the current set of permissions is approved, but additional permissions will require the administrator to reapprove the product (If the product was approved without specifying the approved permissions setting, then this is the default behavior.), - "needsReapproval", the product has unapproved permissions. No additional product licenses can be assigned until the product is reapproved, - "allCurrentAndFutureApproved", the current permissions are approved and any future permission updates will be automatically approved without administrator review.  |
| `num_purchased` | i64 | The number of purchased licenses (possibly in multiple purchases). If this field is omitted, then there is no limit on the number of licenses that can be provisioned (for example, if the acquisition kind is "free"). |
| `acquisition_kind` | String | How this group license was acquired. "bulkPurchase" means that this Grouplicenses resource was created because the enterprise purchased licenses for this product; otherwise, the value is "free" (for free products). |
| `num_provisioned` | i64 | The total number of provisioned licenses for this product. Returned by read operations, but ignored in write operations. |
| `approval` | String | Whether the product to which this group license relates is currently approved by the enterprise. Products are approved when a group license is first created, but this approval may be revoked by an enterprise admin via Google Play. Unapproved products will not be visible to end users in collections, and new entitlements to them should not normally be created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access grouplicense outputs
grouplicense_id = grouplicense.id
grouplicense_product_id = grouplicense.product_id
grouplicense_permissions = grouplicense.permissions
grouplicense_num_purchased = grouplicense.num_purchased
grouplicense_acquisition_kind = grouplicense.acquisition_kind
grouplicense_num_provisioned = grouplicense.num_provisioned
grouplicense_approval = grouplicense.approval
```

---


### Device

Uploads a report containing any changes in app states on the device since the last report was generated. You can call this method up to 3 times every 24 hours for a given device. If you exceed the quota, then the Google Play EMM API returns HTTP 429 Too Many Requests.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_id` | String | ✅ | The ID of the user. |
| `device_id` | String | ✅ | The ID of the device. |
| `enterprise_id` | String | ✅ | The ID of the enterprise. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `retail_brand` | String | Retail brand for the device, if set. See android.os.Build.BRAND |
| `sdk_version` | i64 | API compatibility version. |
| `product` | String | The product name of the device. This comes from android.os.Build.PRODUCT. |
| `report` | String | The device report updated with the latest app states. |
| `latest_build_fingerprint` | String | The build fingerprint of the device if known. |
| `android_id` | String | The Google Play Services Android ID for the device encoded as a lowercase hex string. For example, "123456789abcdef0". |
| `device` | String | The internal hardware codename of the device. This comes from android.os.Build.DEVICE. (field named "device" per logs/wireless/android/android_checkin.proto) |
| `maker` | String | The manufacturer of the device. This comes from android.os.Build.MANUFACTURER. |
| `management_type` | String | Identifies the extent to which the device is controlled by a managed Google Play EMM in various deployment configurations. Possible values include: - "managedDevice", a device that has the EMM's device policy controller (DPC) as the device owner. - "managedProfile", a device that has a profile managed by the DPC (DPC is profile owner) in addition to a separate, personal profile that is unavailable to the DPC. - "containerApp", no longer used (deprecated). - "unmanagedProfile", a device that has been allowed (by the domain's admin, using the Admin Console to enable the privilege) to use managed Google Play, but the profile is itself not owned by a DPC.  |
| `model` | String | The model name of the device. This comes from android.os.Build.MODEL. |
| `policy` | String | The policy enforced on the device. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create device
device = provider.androidenterprise_api.Device {
    user_id = "value"  # The ID of the user.
    device_id = "value"  # The ID of the device.
    enterprise_id = "value"  # The ID of the enterprise.
}

# Access device outputs
device_id = device.id
device_retail_brand = device.retail_brand
device_sdk_version = device.sdk_version
device_product = device.product
device_report = device.report
device_latest_build_fingerprint = device.latest_build_fingerprint
device_android_id = device.android_id
device_device = device.device
device_maker = device.maker
device_management_type = device.management_type
device_model = device.model
device_policy = device.policy
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple serviceaccountkey resources
serviceaccountkey_0 = provider.androidenterprise_api.Serviceaccountkey {
    enterprise_id = "value-0"
}
serviceaccountkey_1 = provider.androidenterprise_api.Serviceaccountkey {
    enterprise_id = "value-1"
}
serviceaccountkey_2 = provider.androidenterprise_api.Serviceaccountkey {
    enterprise_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    serviceaccountkey = provider.androidenterprise_api.Serviceaccountkey {
        enterprise_id = "production-value"
    }
```

---

## Related Documentation

- [GCP Androidenterprise_api Documentation](https://cloud.google.com/androidenterprise_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
