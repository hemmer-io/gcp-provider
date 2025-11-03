# Androiddeviceprovisioning_api Service



**Resources**: 6

---

## Overview

The androiddeviceprovisioning_api service provides access to 6 resource types:

- [Dpc](#dpc) [R]
- [Customer](#customer) [CR]
- [Device](#device) [CR]
- [Operation](#operation) [R]
- [Configuration](#configuration) [CRUD]
- [Vendor](#vendor) [R]

---

## Resources


### Dpc

Lists the DPCs (device policy controllers) that support zero-touch enrollment.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `dpcs` | Vec<String> | The list of DPCs available to the customer that support zero-touch enrollment. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access dpc outputs
dpc_id = dpc.id
dpc_dpcs = dpc.dpcs
```

---


### Customer

Creates a customer for zero-touch enrollment. After the method returns successfully, admin and owner roles can manage devices and EMM configs by calling API methods or using their zero-touch enrollment portal. The customer receives an email that welcomes them to zero-touch enrollment and explains how to sign into the portal.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `customer` | String |  | Required. The company data to populate the new customer. Must contain a value for `companyName` and at least one `owner_email` that's associated with a Google Account. The values for `companyId` and `name` must be empty. |
| `parent` | String | ✅ | Required. The parent resource ID in the format `partners/[PARTNER_ID]` that identifies the reseller. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `customers` | Vec<String> | List of customers related to this reseller partner. |
| `next_page_token` | String | A token to retrieve the next page of results. Omitted if no further results are available. |
| `total_size` | i64 | The total count of items in the list irrespective of pagination. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create customer
customer = provider.androiddeviceprovisioning_api.Customer {
    parent = "value"  # Required. The parent resource ID in the format `partners/[PARTNER_ID]` that identifies the reseller.
}

# Access customer outputs
customer_id = customer.id
customer_customers = customer.customers
customer_next_page_token = customer.next_page_token
customer_total_size = customer.total_size
```

---


### Device

Claims a batch of devices for a customer asynchronously. Adds the devices to zero-touch enrollment. To learn more, read [Long‑running batch operations](/zero-touch/guides/how-it-works#operations).

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `claims` | Vec<String> |  | Required. A list of device claims. |
| `partner_id` | String | ✅ | Required. The ID of the reseller partner. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `claims` | Vec<String> | Output only. The provisioning claims for a device. Devices claimed for zero-touch enrollment have a claim with the type `SECTION_TYPE_ZERO_TOUCH`. Call `partners.devices.unclaim` or `partners.devices.unclaimAsync` to remove the device from zero-touch enrollment. |
| `configuration` | String | Not available to resellers. |
| `device_id` | String | Output only. The ID of the device. Assigned by the server. |
| `device_identifier` | String | The hardware IDs that identify a manufactured device. To learn more, read [Identifiers](https://developers.google.com/zero-touch/guides/identifiers). |
| `name` | String | Output only. The API resource name in the format `partners/[PARTNER_ID]/devices/[DEVICE_ID]`. Assigned by the server. |
| `device_metadata` | String | The metadata attached to the device. Structured as key-value pairs. To learn more, read [Device metadata](https://developers.google.com/zero-touch/guides/metadata). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create device
device = provider.androiddeviceprovisioning_api.Device {
    partner_id = "value"  # Required. The ID of the reseller partner.
}

# Access device outputs
device_id = device.id
device_claims = device.claims
device_configuration = device.configuration
device_device_id = device.device_id
device_device_identifier = device.device_identifier
device_name = device.name
device_device_metadata = device.device_metadata
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
| `error` | String | This field will always be not set if the operation is created by `claimAsync`, `unclaimAsync`, or `updateMetadataAsync`. In this case, error information for each device is set in `response.perDeviceStatus.result.status`. |
| `response` | HashMap<String, String> | This field will contain a `DevicesLongRunningOperationResponse` object if the operation is created by `claimAsync`, `unclaimAsync`, or `updateMetadataAsync`. |
| `metadata` | HashMap<String, String> | This field will contain a `DevicesLongRunningOperationMetadata` object if the operation is created by `claimAsync`, `unclaimAsync`, or `updateMetadataAsync`. |
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

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_response = operation.response
operation_metadata = operation.metadata
operation_done = operation.done
operation_name = operation.name
```

---


### Configuration

Creates a new configuration. Once created, a customer can apply the configuration to devices.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `configuration_name` | String |  | Required. A short name that describes the configuration's purpose. For example, _Sales team_ or _Temporary employees_. The zero-touch enrollment portal displays this name to IT admins. |
| `is_default` | bool |  | Required. Whether this is the default configuration that zero-touch enrollment applies to any new devices the organization purchases in the future. Only one customer configuration can be the default. Setting this value to `true`, changes the previous default configuration's `isDefault` value to `false`. |
| `name` | String |  | Output only. The API resource name in the format `customers/[CUSTOMER_ID]/configurations/[CONFIGURATION_ID]`. Assigned by the server. |
| `company_name` | String |  | Required. The name of the organization. Zero-touch enrollment shows this organization name to device users during device provisioning. |
| `custom_message` | String |  | A message, containing one or two sentences, to help device users get help or give them more details about what’s happening to their device. Zero-touch enrollment shows this message before the device is provisioned. |
| `contact_phone` | String |  | Required. The telephone number that device users can call, using another device, to get help. Zero-touch enrollment shows this number to device users before device provisioning. Accepts numerals, spaces, the plus sign, hyphens, and parentheses. |
| `dpc_resource_path` | String |  | Required. The resource name of the selected DPC (device policy controller) in the format `customers/[CUSTOMER_ID]/dpcs/*`. To list the supported DPCs, call `customers.dpcs.list`. |
| `contact_email` | String |  | Required. The email address that device users can contact to get help. Zero-touch enrollment shows this email address to device users before device provisioning. The value is validated on input. |
| `forced_reset_time` | String |  | Optional. The timeout before forcing factory reset the device if the device doesn't go through provisioning in the setup wizard, usually due to lack of network connectivity during setup wizard. Ranges from 0-6 hours, with 2 hours being the default if unset. |
| `configuration_id` | String |  | Output only. The ID of the configuration. Assigned by the server. |
| `dpc_extras` | String |  | The JSON-formatted EMM provisioning extras that are passed to the DPC. |
| `parent` | String | ✅ | Required. The customer that manages the configuration. An API resource name in the format `customers/[CUSTOMER_ID]`. This field has custom validation in CreateConfigurationRequestValidator |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `configuration_name` | String | Required. A short name that describes the configuration's purpose. For example, _Sales team_ or _Temporary employees_. The zero-touch enrollment portal displays this name to IT admins. |
| `is_default` | bool | Required. Whether this is the default configuration that zero-touch enrollment applies to any new devices the organization purchases in the future. Only one customer configuration can be the default. Setting this value to `true`, changes the previous default configuration's `isDefault` value to `false`. |
| `name` | String | Output only. The API resource name in the format `customers/[CUSTOMER_ID]/configurations/[CONFIGURATION_ID]`. Assigned by the server. |
| `company_name` | String | Required. The name of the organization. Zero-touch enrollment shows this organization name to device users during device provisioning. |
| `custom_message` | String | A message, containing one or two sentences, to help device users get help or give them more details about what’s happening to their device. Zero-touch enrollment shows this message before the device is provisioned. |
| `contact_phone` | String | Required. The telephone number that device users can call, using another device, to get help. Zero-touch enrollment shows this number to device users before device provisioning. Accepts numerals, spaces, the plus sign, hyphens, and parentheses. |
| `dpc_resource_path` | String | Required. The resource name of the selected DPC (device policy controller) in the format `customers/[CUSTOMER_ID]/dpcs/*`. To list the supported DPCs, call `customers.dpcs.list`. |
| `contact_email` | String | Required. The email address that device users can contact to get help. Zero-touch enrollment shows this email address to device users before device provisioning. The value is validated on input. |
| `forced_reset_time` | String | Optional. The timeout before forcing factory reset the device if the device doesn't go through provisioning in the setup wizard, usually due to lack of network connectivity during setup wizard. Ranges from 0-6 hours, with 2 hours being the default if unset. |
| `configuration_id` | String | Output only. The ID of the configuration. Assigned by the server. |
| `dpc_extras` | String | The JSON-formatted EMM provisioning extras that are passed to the DPC. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create configuration
configuration = provider.androiddeviceprovisioning_api.Configuration {
    parent = "value"  # Required. The customer that manages the configuration. An API resource name in the format `customers/[CUSTOMER_ID]`. This field has custom validation in CreateConfigurationRequestValidator
}

# Access configuration outputs
configuration_id = configuration.id
configuration_configuration_name = configuration.configuration_name
configuration_is_default = configuration.is_default
configuration_name = configuration.name
configuration_company_name = configuration.company_name
configuration_custom_message = configuration.custom_message
configuration_contact_phone = configuration.contact_phone
configuration_dpc_resource_path = configuration.dpc_resource_path
configuration_contact_email = configuration.contact_email
configuration_forced_reset_time = configuration.forced_reset_time
configuration_configuration_id = configuration.configuration_id
configuration_dpc_extras = configuration.dpc_extras
```

---


### Vendor

Lists the vendors of the partner.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `total_size` | i64 | The total count of items in the list irrespective of pagination. |
| `next_page_token` | String | A token to retrieve the next page of results. Omitted if no further results are available. |
| `vendors` | Vec<String> | List of vendors of the reseller partner. Fields `name`, `companyId` and `companyName` are populated to the Company object. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access vendor outputs
vendor_id = vendor.id
vendor_total_size = vendor.total_size
vendor_next_page_token = vendor.next_page_token
vendor_vendors = vendor.vendors
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple dpc resources
dpc_0 = provider.androiddeviceprovisioning_api.Dpc {
}
dpc_1 = provider.androiddeviceprovisioning_api.Dpc {
}
dpc_2 = provider.androiddeviceprovisioning_api.Dpc {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    dpc = provider.androiddeviceprovisioning_api.Dpc {
    }
```

---

## Related Documentation

- [GCP Androiddeviceprovisioning_api Documentation](https://cloud.google.com/androiddeviceprovisioning_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
