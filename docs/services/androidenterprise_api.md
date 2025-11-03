# Androidenterprise_api Service



**Resources**: 17

---

## Overview

The androidenterprise_api service provides access to 17 resource types:

- [Permission](#permission) [R]
- [User](#user) [CRUD]
- [Grouplicense](#grouplicense) [R]
- [Install](#install) [RUD]
- [Managedconfigurationsfordevice](#managedconfigurationsfordevice) [RUD]
- [Device](#device) [CRU]
- [Product](#product) [CR]
- [Managedconfigurationsforuser](#managedconfigurationsforuser) [RUD]
- [Grouplicenseuser](#grouplicenseuser) [R]
- [Storelayoutpage](#storelayoutpage) [CRUD]
- [Serviceaccountkey](#serviceaccountkey) [CRD]
- [Entitlement](#entitlement) [RUD]
- [Enterprise](#enterprise) [CRU]
- [Managedconfigurationssetting](#managedconfigurationssetting) [R]
- [Enrollment_token](#enrollment_token) [C]
- [Storelayoutcluster](#storelayoutcluster) [CRUD]
- [Webapp](#webapp) [CRUD]

---

## Resources


### Permission

Retrieves details of an Android app permission for display to an enterprise admin.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The name of the permission. |
| `permission_id` | String | An opaque string uniquely identifying the permission. |
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
permission_name = permission.name
permission_permission_id = permission.permission_id
permission_description = permission.description
```

---


### User

Creates a new EMM-managed user. The Users resource passed in the body of the request should include an accountIdentifier and an accountType. If a corresponding user already exists with the same account identifier, the user will be updated with the resource. In this case only the displayName field can be changed.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `management_type` | String |  | The entity that manages the user. With googleManaged users, the source of truth is Google so EMMs have to make sure a Google Account exists for the user. With emmManaged users, the EMM is in charge. |
| `primary_email` | String |  | The user's primary email address, for example, "jsmith@example.com". Will always be set for Google managed users and not set for EMM managed users. |
| `account_identifier` | String |  | A unique identifier you create for this user, such as "user342" or "asset#44418". Do not use personally identifiable information (PII) for this property. Must always be set for EMM-managed users. Not set for Google-managed users. |
| `account_type` | String |  | The type of account that this user represents. A userAccount can be installed on multiple devices, but a deviceAccount is specific to a single device. An EMM-managed user (emmManaged) can be either type (userAccount, deviceAccount), but a Google-managed user (googleManaged) is always a userAccount. |
| `id` | String |  | The unique ID for the user. |
| `display_name` | String |  | The name that will appear in user interfaces. Setting this property is optional when creating EMM-managed users. If you do set this property, use something generic about the organization (such as "Example, Inc.") or your name (as EMM). Not used for Google-managed user accounts. @mutable androidenterprise.users.update |
| `enterprise_id` | String | ✅ | The ID of the enterprise. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `management_type` | String | The entity that manages the user. With googleManaged users, the source of truth is Google so EMMs have to make sure a Google Account exists for the user. With emmManaged users, the EMM is in charge. |
| `primary_email` | String | The user's primary email address, for example, "jsmith@example.com". Will always be set for Google managed users and not set for EMM managed users. |
| `account_identifier` | String | A unique identifier you create for this user, such as "user342" or "asset#44418". Do not use personally identifiable information (PII) for this property. Must always be set for EMM-managed users. Not set for Google-managed users. |
| `account_type` | String | The type of account that this user represents. A userAccount can be installed on multiple devices, but a deviceAccount is specific to a single device. An EMM-managed user (emmManaged) can be either type (userAccount, deviceAccount), but a Google-managed user (googleManaged) is always a userAccount. |
| `id` | String | The unique ID for the user. |
| `display_name` | String | The name that will appear in user interfaces. Setting this property is optional when creating EMM-managed users. If you do set this property, use something generic about the organization (such as "Example, Inc.") or your name (as EMM). Not used for Google-managed user accounts. @mutable androidenterprise.users.update |


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
user_management_type = user.management_type
user_primary_email = user.primary_email
user_account_identifier = user.account_identifier
user_account_type = user.account_type
user_id = user.id
user_display_name = user.display_name
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
| `num_provisioned` | i64 | The total number of provisioned licenses for this product. Returned by read operations, but ignored in write operations. |
| `product_id` | String | The ID of the product that the license is for. For example, "app:com.google.android.gm". |
| `num_purchased` | i64 | The number of purchased licenses (possibly in multiple purchases). If this field is omitted, then there is no limit on the number of licenses that can be provisioned (for example, if the acquisition kind is "free"). |
| `approval` | String | Whether the product to which this group license relates is currently approved by the enterprise. Products are approved when a group license is first created, but this approval may be revoked by an enterprise admin via Google Play. Unapproved products will not be visible to end users in collections, and new entitlements to them should not normally be created. |
| `permissions` | String | The permission approval status of the product. This field is only set if the product is approved. Possible states are: - "currentApproved", the current set of permissions is approved, but additional permissions will require the administrator to reapprove the product (If the product was approved without specifying the approved permissions setting, then this is the default behavior.), - "needsReapproval", the product has unapproved permissions. No additional product licenses can be assigned until the product is reapproved, - "allCurrentAndFutureApproved", the current permissions are approved and any future permission updates will be automatically approved without administrator review.  |
| `acquisition_kind` | String | How this group license was acquired. "bulkPurchase" means that this Grouplicenses resource was created because the enterprise purchased licenses for this product; otherwise, the value is "free" (for free products). |


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
grouplicense_num_provisioned = grouplicense.num_provisioned
grouplicense_product_id = grouplicense.product_id
grouplicense_num_purchased = grouplicense.num_purchased
grouplicense_approval = grouplicense.approval
grouplicense_permissions = grouplicense.permissions
grouplicense_acquisition_kind = grouplicense.acquisition_kind
```

---


### Install

Retrieves details of an installation of an app on a device.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `product_id` | String |  | The ID of the product that the install is for. For example, "app:com.google.android.gm". |
| `install_state` | String |  | Install state. The state "installPending" means that an install request has recently been made and download to the device is in progress. The state "installed" means that the app has been installed. This field is read-only. |
| `version_code` | i64 |  | The version of the installed product. Guaranteed to be set only if the install state is "installed". |
| `device_id` | String | ✅ | The Android ID of the device. |
| `enterprise_id` | String | ✅ | The ID of the enterprise. |
| `user_id` | String | ✅ | The ID of the user. |
| `install_id` | String | ✅ | The ID of the product represented by the install, e.g. "app:com.google.android.gm". |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `product_id` | String | The ID of the product that the install is for. For example, "app:com.google.android.gm". |
| `install_state` | String | Install state. The state "installPending" means that an install request has recently been made and download to the device is in progress. The state "installed" means that the app has been installed. This field is read-only. |
| `version_code` | i64 | The version of the installed product. Guaranteed to be set only if the install state is "installed". |


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
install_product_id = install.product_id
install_install_state = install.install_state
install_version_code = install.version_code
```

---


### Managedconfigurationsfordevice

Retrieves details of a per-device managed configuration.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `product_id` | String |  | The ID of the product that the managed configuration is for, e.g. "app:com.google.android.gm". |
| `configuration_variables` | String |  | Contains the ID of the managed configuration profile and the set of configuration variables (if any) defined for the user. |
| `kind` | String |  | Deprecated. |
| `managed_property` | Vec<String> |  | The set of managed properties for this configuration. |
| `device_id` | String | ✅ | The Android ID of the device. |
| `user_id` | String | ✅ | The ID of the user. |
| `managed_configuration_for_device_id` | String | ✅ | The ID of the managed configuration (a product ID), e.g. "app:com.google.android.gm". |
| `enterprise_id` | String | ✅ | The ID of the enterprise. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `product_id` | String | The ID of the product that the managed configuration is for, e.g. "app:com.google.android.gm". |
| `configuration_variables` | String | Contains the ID of the managed configuration profile and the set of configuration variables (if any) defined for the user. |
| `kind` | String | Deprecated. |
| `managed_property` | Vec<String> | The set of managed properties for this configuration. |


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
managedconfigurationsfordevice_product_id = managedconfigurationsfordevice.product_id
managedconfigurationsfordevice_configuration_variables = managedconfigurationsfordevice.configuration_variables
managedconfigurationsfordevice_kind = managedconfigurationsfordevice.kind
managedconfigurationsfordevice_managed_property = managedconfigurationsfordevice.managed_property
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
| `report` | String | The device report updated with the latest app states. |
| `management_type` | String | Identifies the extent to which the device is controlled by a managed Google Play EMM in various deployment configurations. Possible values include: - "managedDevice", a device that has the EMM's device policy controller (DPC) as the device owner. - "managedProfile", a device that has a profile managed by the DPC (DPC is profile owner) in addition to a separate, personal profile that is unavailable to the DPC. - "containerApp", no longer used (deprecated). - "unmanagedProfile", a device that has been allowed (by the domain's admin, using the Admin Console to enable the privilege) to use managed Google Play, but the profile is itself not owned by a DPC.  |
| `policy` | String | The policy enforced on the device. |
| `product` | String | The product name of the device. This comes from android.os.Build.PRODUCT. |
| `device` | String | The internal hardware codename of the device. This comes from android.os.Build.DEVICE. (field named "device" per logs/wireless/android/android_checkin.proto) |
| `maker` | String | The manufacturer of the device. This comes from android.os.Build.MANUFACTURER. |
| `retail_brand` | String | Retail brand for the device, if set. See android.os.Build.BRAND |
| `latest_build_fingerprint` | String | The build fingerprint of the device if known. |
| `model` | String | The model name of the device. This comes from android.os.Build.MODEL. |
| `sdk_version` | i64 | API compatibility version. |
| `android_id` | String | The Google Play Services Android ID for the device encoded as a lowercase hex string. For example, "123456789abcdef0". |


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
device_report = device.report
device_management_type = device.management_type
device_policy = device.policy
device_product = device.product
device_device = device.device
device_maker = device.maker
device_retail_brand = device.retail_brand
device_latest_build_fingerprint = device.latest_build_fingerprint
device_model = device.model
device_sdk_version = device.sdk_version
device_android_id = device.android_id
```

---


### Product

Unapproves the specified product (and the relevant app permissions, if any) **Note:** This item has been deprecated. New integrations cannot use this method and can refer to our new recommendations.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `enterprise_id` | String | ✅ | The ID of the enterprise. |
| `product_id` | String | ✅ | The ID of the product. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `recent_changes` | String | A description of the recent changes made to the app. |
| `title` | String | The name of the product. |
| `full_description` | String | The localized full app store description, if available. |
| `category` | String | The app category (e.g. RACING, SOCIAL, etc.) |
| `last_updated_timestamp_millis` | String | The approximate time (within 7 days) the app was last published, expressed in milliseconds since epoch. |
| `icon_url` | String | A link to an image that can be used as an icon for the product. This image is suitable for use at up to 512px x 512px. |
| `app_version` | Vec<String> | App versions currently available for this product. |
| `content_rating` | String | The content rating for this app. |
| `requires_container_app` | bool | Deprecated. |
| `distribution_channel` | String | How and to whom the package is made available. The value publicGoogleHosted means that the package is available through the Play store and not restricted to a specific enterprise. The value privateGoogleHosted means that the package is a private app (restricted to an enterprise) but hosted by Google. The value privateSelfHosted means that the package is a private app (restricted to an enterprise) and is privately hosted. |
| `author_name` | String | The name of the author of the product (for example, the app developer). |
| `screenshot_urls` | Vec<String> | A list of screenshot links representing the app. |
| `signing_certificate` | String | The certificate used to sign this product. |
| `app_tracks` | Vec<String> | The tracks visible to the enterprise. |
| `work_details_url` | String | A link to the managed Google Play details page for the product, for use by an Enterprise admin. |
| `product_pricing` | String | Whether this product is free, free with in-app purchases, or paid. If the pricing is unknown, this means the product is not generally available anymore (even though it might still be available to people who own it). |
| `details_url` | String | A link to the (consumer) Google Play details page for the product. |
| `available_countries` | Vec<String> | The countries which this app is available in. |
| `available_tracks` | Vec<String> | Deprecated, use appTracks instead. |
| `description` | String | The localized promotional description, if available. |
| `app_restrictions_schema` | String | The app restriction schema |
| `product_id` | String | A string of the form *app:<package name>*. For example, app:com.google.android.gm represents the Gmail app. |
| `features` | Vec<String> | Noteworthy features (if any) of this product. |
| `min_android_sdk_version` | i64 | The minimum Android SDK necessary to run the app. |
| `small_icon_url` | String | A link to a smaller image that can be used as an icon for the product. This image is suitable for use at up to 128px x 128px. |
| `permissions` | Vec<String> | A list of permissions required by the app. |


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
product_recent_changes = product.recent_changes
product_title = product.title
product_full_description = product.full_description
product_category = product.category
product_last_updated_timestamp_millis = product.last_updated_timestamp_millis
product_icon_url = product.icon_url
product_app_version = product.app_version
product_content_rating = product.content_rating
product_requires_container_app = product.requires_container_app
product_distribution_channel = product.distribution_channel
product_author_name = product.author_name
product_screenshot_urls = product.screenshot_urls
product_signing_certificate = product.signing_certificate
product_app_tracks = product.app_tracks
product_work_details_url = product.work_details_url
product_product_pricing = product.product_pricing
product_details_url = product.details_url
product_available_countries = product.available_countries
product_available_tracks = product.available_tracks
product_description = product.description
product_app_restrictions_schema = product.app_restrictions_schema
product_product_id = product.product_id
product_features = product.features
product_min_android_sdk_version = product.min_android_sdk_version
product_small_icon_url = product.small_icon_url
product_permissions = product.permissions
```

---


### Managedconfigurationsforuser

Retrieves details of a per-user managed configuration for an app for the specified user.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `product_id` | String |  | The ID of the product that the managed configuration is for, e.g. "app:com.google.android.gm". |
| `configuration_variables` | String |  | Contains the ID of the managed configuration profile and the set of configuration variables (if any) defined for the user. |
| `kind` | String |  | Deprecated. |
| `managed_property` | Vec<String> |  | The set of managed properties for this configuration. |
| `enterprise_id` | String | ✅ | The ID of the enterprise. |
| `user_id` | String | ✅ | The ID of the user. |
| `managed_configuration_for_user_id` | String | ✅ | The ID of the managed configuration (a product ID), e.g. "app:com.google.android.gm". |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `product_id` | String | The ID of the product that the managed configuration is for, e.g. "app:com.google.android.gm". |
| `configuration_variables` | String | Contains the ID of the managed configuration profile and the set of configuration variables (if any) defined for the user. |
| `kind` | String | Deprecated. |
| `managed_property` | Vec<String> | The set of managed properties for this configuration. |


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
managedconfigurationsforuser_product_id = managedconfigurationsforuser.product_id
managedconfigurationsforuser_configuration_variables = managedconfigurationsforuser.configuration_variables
managedconfigurationsforuser_kind = managedconfigurationsforuser.kind
managedconfigurationsforuser_managed_property = managedconfigurationsforuser.managed_property
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


### Storelayoutpage

Inserts a new store page.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | Vec<String> |  | Ordered list of localized strings giving the name of this page. The text displayed is the one that best matches the user locale, or the first entry if there is no good match. There needs to be at least one entry. |
| `id` | String |  | Unique ID of this page. Assigned by the server. Immutable once assigned. |
| `link` | Vec<String> |  | Ordered list of pages a user should be able to reach from this page. The list can't include this page. It is recommended that the basic pages are created first, before adding the links between pages. The API doesn't verify that the pages exist or the pages are reachable. |
| `enterprise_id` | String | ✅ | The ID of the enterprise. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | Vec<String> | Ordered list of localized strings giving the name of this page. The text displayed is the one that best matches the user locale, or the first entry if there is no good match. There needs to be at least one entry. |
| `id` | String | Unique ID of this page. Assigned by the server. Immutable once assigned. |
| `link` | Vec<String> | Ordered list of pages a user should be able to reach from this page. The list can't include this page. It is recommended that the basic pages are created first, before adding the links between pages. The API doesn't verify that the pages exist or the pages are reachable. |


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
storelayoutpage_name = storelayoutpage.name
storelayoutpage_id = storelayoutpage.id
storelayoutpage_link = storelayoutpage.link
```

---


### Serviceaccountkey

Generates new credentials for the service account associated with this enterprise. The calling service account must have been retrieved by calling Enterprises.GetServiceAccount and must have been set as the enterprise service account by calling Enterprises.SetAccount. Only the type of the key should be populated in the resource to be inserted.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | String |  | The file format of the generated key data. |
| `public_data` | String |  | Public key data for the credentials file. This is an X.509 cert. If you are using the googleCredentials key type, this is identical to the cert that can be retrieved by using the X.509 cert url inside of the credentials file. |
| `id` | String |  | An opaque, unique identifier for this ServiceAccountKey. Assigned by the server. |
| `data` | String |  | The body of the private key credentials file, in string format. This is only populated when the ServiceAccountKey is created, and is not stored by Google. |
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


### Entitlement

Retrieves details of an entitlement. **Note:** This item has been deprecated. New integrations cannot use this method and can refer to our new recommendations.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `product_id` | String |  | The ID of the product that the entitlement is for. For example, "app:com.google.android.gm". |
| `reason` | String |  | The reason for the entitlement. For example, "free" for free apps. This property is temporary: it will be replaced by the acquisition kind field of group licenses. |
| `user_id` | String | ✅ | The ID of the user. |
| `entitlement_id` | String | ✅ | The ID of the entitlement (a product ID), e.g. "app:com.google.android.gm". |
| `enterprise_id` | String | ✅ | The ID of the enterprise. |


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


### Enterprise

Acknowledges notifications that were received from Enterprises.PullNotificationSet to prevent subsequent calls from returning the same notifications.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The name of the enterprise, for example, "Example, Inc". |
| `primary_domain` | String | The enterprise's primary domain, such as "example.com". |
| `administrator` | Vec<String> | Admins of the enterprise. This is only supported for enterprises created via the EMM-initiated flow. |
| `id` | String | The unique ID for the enterprise. |
| `enterprise_type` | String | The type of the enterprise. |
| `google_authentication_settings` | String | Output only. Settings for Google-provided user authentication. |
| `managed_google_domain_type` | String | The type of managed Google domain |


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
enterprise_name = enterprise.name
enterprise_primary_domain = enterprise.primary_domain
enterprise_administrator = enterprise.administrator
enterprise_id = enterprise.id
enterprise_enterprise_type = enterprise.enterprise_type
enterprise_google_authentication_settings = enterprise.google_authentication_settings
enterprise_managed_google_domain_type = enterprise.managed_google_domain_type
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


### Enrollment_token

Returns a token for device enrollment. The DPC can encode this token within the QR/NFC/zero-touch enrollment payload or fetch it before calling the on-device API to authenticate the user. The token can be generated for each device or reused across multiple devices.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `enrollment_token_type` | String |  | [Required] The type of the enrollment token. |
| `google_authentication_options` | String |  | [Optional] Provides options related to Google authentication during the enrollment. |
| `token` | String |  | The token value that's passed to the device and authorizes the device to enroll. This is a read-only field generated by the server. |
| `duration` | String |  | [Optional] The length of time the enrollment token is valid, ranging from 1 minute to [`Durations.MAX_VALUE`](https://developers.google.com/protocol-buffers/docs/reference/java/com/google/protobuf/util/Durations.html#MAX_VALUE), approximately 10,000 years. If not specified, the default duration is 1 hour. |
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


### Storelayoutcluster

Inserts a new cluster in a page.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `product_id` | Vec<String> |  | List of products in the order they are displayed in the cluster. There should not be duplicates within a cluster. |
| `id` | String |  | Unique ID of this cluster. Assigned by the server. Immutable once assigned. |
| `order_in_page` | String |  | String (US-ASCII only) used to determine order of this cluster within the parent page's elements. Page elements are sorted in lexicographic order of this field. Duplicated values are allowed, but ordering between elements with duplicate order is undefined. The value of this field is never visible to a user, it is used solely for the purpose of defining an ordering. Maximum length is 256 characters. |
| `name` | Vec<String> |  | Ordered list of localized strings giving the name of this page. The text displayed is the one that best matches the user locale, or the first entry if there is no good match. There needs to be at least one entry. |
| `page_id` | String | ✅ | The ID of the page. |
| `enterprise_id` | String | ✅ | The ID of the enterprise. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `product_id` | Vec<String> | List of products in the order they are displayed in the cluster. There should not be duplicates within a cluster. |
| `id` | String | Unique ID of this cluster. Assigned by the server. Immutable once assigned. |
| `order_in_page` | String | String (US-ASCII only) used to determine order of this cluster within the parent page's elements. Page elements are sorted in lexicographic order of this field. Duplicated values are allowed, but ordering between elements with duplicate order is undefined. The value of this field is never visible to a user, it is used solely for the purpose of defining an ordering. Maximum length is 256 characters. |
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
storelayoutcluster_product_id = storelayoutcluster.product_id
storelayoutcluster_id = storelayoutcluster.id
storelayoutcluster_order_in_page = storelayoutcluster.order_in_page
storelayoutcluster_name = storelayoutcluster.name
```

---


### Webapp

Creates a new web app for the enterprise.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `version_code` | String |  | The current version of the app. Note that the version can automatically increase during the lifetime of the web app, while Google does internal housekeeping to keep the web app up-to-date. |
| `web_app_id` | String |  | The ID of the application. A string of the form "app:<package name>" where the package name always starts with the prefix "com.google.enterprise.webapp." followed by a random id. |
| `is_published` | bool |  | A flag whether the app has been published to the Play store yet. |
| `icons` | Vec<String> |  | A list of icons representing this website. If absent, a default icon (for create) or the current icon (for update) will be used. |
| `start_url` | String |  | The start URL, i.e. the URL that should load when the user opens the application. |
| `display_mode` | String |  | The display mode of the web app. Possible values include: - "minimalUi", the device's status bar, navigation bar, the app's URL, and a refresh button are visible when the app is open. For HTTP URLs, you can only select this option. - "standalone", the device's status bar and navigation bar are visible when the app is open. - "fullScreen", the app opens in full screen mode, hiding the device's status and navigation bars. All browser UI elements, page URL, system status bar and back button are not visible, and the web app takes up the entirety of the available display area.  |
| `title` | String |  | The title of the web app as displayed to the user (e.g., amongst a list of other applications, or as a label for an icon). |
| `enterprise_id` | String | ✅ | The ID of the enterprise. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `version_code` | String | The current version of the app. Note that the version can automatically increase during the lifetime of the web app, while Google does internal housekeeping to keep the web app up-to-date. |
| `web_app_id` | String | The ID of the application. A string of the form "app:<package name>" where the package name always starts with the prefix "com.google.enterprise.webapp." followed by a random id. |
| `is_published` | bool | A flag whether the app has been published to the Play store yet. |
| `icons` | Vec<String> | A list of icons representing this website. If absent, a default icon (for create) or the current icon (for update) will be used. |
| `start_url` | String | The start URL, i.e. the URL that should load when the user opens the application. |
| `display_mode` | String | The display mode of the web app. Possible values include: - "minimalUi", the device's status bar, navigation bar, the app's URL, and a refresh button are visible when the app is open. For HTTP URLs, you can only select this option. - "standalone", the device's status bar and navigation bar are visible when the app is open. - "fullScreen", the app opens in full screen mode, hiding the device's status and navigation bars. All browser UI elements, page URL, system status bar and back button are not visible, and the web app takes up the entirety of the available display area.  |
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
webapp_version_code = webapp.version_code
webapp_web_app_id = webapp.web_app_id
webapp_is_published = webapp.is_published
webapp_icons = webapp.icons
webapp_start_url = webapp.start_url
webapp_display_mode = webapp.display_mode
webapp_title = webapp.title
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple permission resources
permission_0 = provider.androidenterprise_api.Permission {
}
permission_1 = provider.androidenterprise_api.Permission {
}
permission_2 = provider.androidenterprise_api.Permission {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    permission = provider.androidenterprise_api.Permission {
    }
```

---

## Related Documentation

- [GCP Androidenterprise_api Documentation](https://cloud.google.com/androidenterprise_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
