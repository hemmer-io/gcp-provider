# Cloudcontrolspartner_api Service



**Resources**: 10

---

## Overview

The cloudcontrolspartner_api service provides access to 10 resource types:

- [Location](#location) [R]
- [Workload](#workload) [R]
- [Access_approval_request](#access_approval_request) [R]
- [Violation](#violation) [R]
- [Customer](#customer) [CRUD]
- [Violation](#violation) [R]
- [Access_approval_request](#access_approval_request) [R]
- [Customer](#customer) [CRUD]
- [Workload](#workload) [R]
- [Location](#location) [R]

---

## Resources


### Location

Get details of a Partner.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Time the resource was created |
| `operated_cloud_regions` | Vec<String> | List of Google Cloud regions that the partner sells services to customers. Valid Google Cloud regions found here: https://cloud.google.com/compute/docs/regions-zones |
| `skus` | Vec<String> | List of SKUs the partner is offering |
| `name` | String | Identifier. The resource name of the partner. Format: `organizations/{organization}/locations/{location}/partner` Example: "organizations/123456/locations/us-central1/partner" |
| `ekm_solutions` | Vec<String> | List of Google Cloud supported EKM partners supported by the partner |
| `partner_project_id` | String | Google Cloud project ID in the partner's Google Cloud organization for receiving enhanced Logs for Partners. |
| `update_time` | String | Output only. The last time the resource was updated |


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
location_create_time = location.create_time
location_operated_cloud_regions = location.operated_cloud_regions
location_skus = location.skus
location_name = location.name
location_ekm_solutions = location.ekm_solutions
location_partner_project_id = location.partner_project_id
location_update_time = location.update_time
```

---


### Workload

Gets details of a single workload

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `location` | String | The Google Cloud location of the workload |
| `name` | String | Identifier. Format: `organizations/{organization}/locations/{location}/customers/{customer}/workloads/{workload}` |
| `is_onboarded` | bool | Indicates whether a workload is fully onboarded. |
| `key_management_project_id` | String | The project id of the key management project for the workload |
| `folder` | String | Output only. The name of container folder of the assured workload |
| `folder_id` | String | Output only. Folder id this workload is associated with |
| `partner` | String | Partner associated with this workload. |
| `workload_onboarding_state` | String | Container for workload onboarding steps. |
| `create_time` | String | Output only. Time the resource was created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access workload outputs
workload_id = workload.id
workload_location = workload.location
workload_name = workload.name
workload_is_onboarded = workload.is_onboarded
workload_key_management_project_id = workload.key_management_project_id
workload_folder = workload.folder
workload_folder_id = workload.folder_id
workload_partner = workload.partner
workload_workload_onboarding_state = workload.workload_onboarding_state
workload_create_time = workload.create_time
```

---


### Access_approval_request

Deprecated: Only returns access approval requests directly associated with an assured workload folder.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token that can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `unreachable` | Vec<String> | Locations that could not be reached. |
| `access_approval_requests` | Vec<String> | List of access approval requests |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access access_approval_request outputs
access_approval_request_id = access_approval_request.id
access_approval_request_next_page_token = access_approval_request.next_page_token
access_approval_request_unreachable = access_approval_request.unreachable
access_approval_request_access_approval_requests = access_approval_request.access_approval_requests
```

---


### Violation

Gets details of a single Violation.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `folder_id` | String | The folder_id of the violation |
| `begin_time` | String | Output only. Time of the event which triggered the Violation. |
| `state` | String | Output only. State of the violation |
| `remediation` | String | Output only. Compliance violation remediation |
| `name` | String | Identifier. Format: `organizations/{organization}/locations/{location}/customers/{customer}/workloads/{workload}/violations/{violation}` |
| `resolve_time` | String | Output only. Time of the event which fixed the Violation. If the violation is ACTIVE this will be empty. |
| `update_time` | String | Output only. The last time when the Violation record was updated. |
| `description` | String | Output only. Description for the Violation. e.g. OrgPolicy gcp.resourceLocations has non compliant value. |
| `non_compliant_org_policy` | String | Output only. Immutable. Name of the OrgPolicy which was modified with non-compliant change and resulted this violation. Format: `projects/{project_number}/policies/{constraint_name}` `folders/{folder_id}/policies/{constraint_name}` `organizations/{organization_id}/policies/{constraint_name}` |
| `category` | String | Output only. Category under which this violation is mapped. e.g. Location, Service Usage, Access, Encryption, etc. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access violation outputs
violation_id = violation.id
violation_folder_id = violation.folder_id
violation_begin_time = violation.begin_time
violation_state = violation.state
violation_remediation = violation.remediation
violation_name = violation.name
violation_resolve_time = violation.resolve_time
violation_update_time = violation.update_time
violation_description = violation.description
violation_non_compliant_org_policy = violation.non_compliant_org_policy
violation_category = violation.category
```

---


### Customer

Creates a new customer.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. Format: `organizations/{organization}/locations/{location}/customers/{customer}` |
| `organization_domain` | String |  | Output only. The customer organization domain, extracted from CRM Organization’s display_name field. e.g. "google.com" |
| `customer_onboarding_state` | String |  | Output only. Container for customer onboarding steps |
| `display_name` | String |  | Required. Display name for the customer |
| `is_onboarded` | bool |  | Output only. Indicates whether a customer is fully onboarded |
| `parent` | String | ✅ | Required. Parent resource Format: `organizations/{organization}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. Format: `organizations/{organization}/locations/{location}/customers/{customer}` |
| `organization_domain` | String | Output only. The customer organization domain, extracted from CRM Organization’s display_name field. e.g. "google.com" |
| `customer_onboarding_state` | String | Output only. Container for customer onboarding steps |
| `display_name` | String | Required. Display name for the customer |
| `is_onboarded` | bool | Output only. Indicates whether a customer is fully onboarded |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create customer
customer = provider.cloudcontrolspartner_api.Customer {
    parent = "value"  # Required. Parent resource Format: `organizations/{organization}/locations/{location}`
}

# Access customer outputs
customer_id = customer.id
customer_name = customer.name
customer_organization_domain = customer.organization_domain
customer_customer_onboarding_state = customer.customer_onboarding_state
customer_display_name = customer.display_name
customer_is_onboarded = customer.is_onboarded
```

---


### Violation

Gets details of a single Violation.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `non_compliant_org_policy` | String | Output only. Immutable. Name of the OrgPolicy which was modified with non-compliant change and resulted this violation. Format: `projects/{project_number}/policies/{constraint_name}` `folders/{folder_id}/policies/{constraint_name}` `organizations/{organization_id}/policies/{constraint_name}` |
| `folder_id` | String | The folder_id of the violation |
| `state` | String | Output only. State of the violation |
| `description` | String | Output only. Description for the Violation. e.g. OrgPolicy gcp.resourceLocations has non compliant value. |
| `category` | String | Output only. Category under which this violation is mapped. e.g. Location, Service Usage, Access, Encryption, etc. |
| `remediation` | String | Output only. Compliance violation remediation |
| `begin_time` | String | Output only. Time of the event which triggered the Violation. |
| `name` | String | Identifier. Format: `organizations/{organization}/locations/{location}/customers/{customer}/workloads/{workload}/violations/{violation}` |
| `resolve_time` | String | Output only. Time of the event which fixed the Violation. If the violation is ACTIVE this will be empty. |
| `update_time` | String | Output only. The last time when the Violation record was updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access violation outputs
violation_id = violation.id
violation_non_compliant_org_policy = violation.non_compliant_org_policy
violation_folder_id = violation.folder_id
violation_state = violation.state
violation_description = violation.description
violation_category = violation.category
violation_remediation = violation.remediation
violation_begin_time = violation.begin_time
violation_name = violation.name
violation_resolve_time = violation.resolve_time
violation_update_time = violation.update_time
```

---


### Access_approval_request

Deprecated: Only returns access approval requests directly associated with an assured workload folder.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `access_approval_requests` | Vec<String> | List of access approval requests |
| `next_page_token` | String | A token that can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `unreachable` | Vec<String> | Locations that could not be reached. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access access_approval_request outputs
access_approval_request_id = access_approval_request.id
access_approval_request_access_approval_requests = access_approval_request.access_approval_requests
access_approval_request_next_page_token = access_approval_request.next_page_token
access_approval_request_unreachable = access_approval_request.unreachable
```

---


### Customer

Creates a new customer.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. Format: `organizations/{organization}/locations/{location}/customers/{customer}` |
| `organization_domain` | String |  | Output only. The customer organization domain, extracted from CRM Organization’s display_name field. e.g. "google.com" |
| `is_onboarded` | bool |  | Output only. Indicates whether a customer is fully onboarded |
| `customer_onboarding_state` | String |  | Output only. Container for customer onboarding steps |
| `display_name` | String |  | Required. Display name for the customer |
| `parent` | String | ✅ | Required. Parent resource Format: `organizations/{organization}/locations/{location}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. Format: `organizations/{organization}/locations/{location}/customers/{customer}` |
| `organization_domain` | String | Output only. The customer organization domain, extracted from CRM Organization’s display_name field. e.g. "google.com" |
| `is_onboarded` | bool | Output only. Indicates whether a customer is fully onboarded |
| `customer_onboarding_state` | String | Output only. Container for customer onboarding steps |
| `display_name` | String | Required. Display name for the customer |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create customer
customer = provider.cloudcontrolspartner_api.Customer {
    parent = "value"  # Required. Parent resource Format: `organizations/{organization}/locations/{location}`
}

# Access customer outputs
customer_id = customer.id
customer_name = customer.name
customer_organization_domain = customer.organization_domain
customer_is_onboarded = customer.is_onboarded
customer_customer_onboarding_state = customer.customer_onboarding_state
customer_display_name = customer.display_name
```

---


### Workload

Gets details of a single workload

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Time the resource was created. |
| `workload_onboarding_state` | String | Container for workload onboarding steps. |
| `name` | String | Identifier. Format: `organizations/{organization}/locations/{location}/customers/{customer}/workloads/{workload}` |
| `partner` | String | Partner associated with this workload. |
| `is_onboarded` | bool | Indicates whether a workload is fully onboarded. |
| `location` | String | The Google Cloud location of the workload |
| `key_management_project_id` | String | The project id of the key management project for the workload |
| `folder` | String | Output only. The name of container folder of the assured workload |
| `folder_id` | String | Output only. Folder id this workload is associated with |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access workload outputs
workload_id = workload.id
workload_create_time = workload.create_time
workload_workload_onboarding_state = workload.workload_onboarding_state
workload_name = workload.name
workload_partner = workload.partner
workload_is_onboarded = workload.is_onboarded
workload_location = workload.location
workload_key_management_project_id = workload.key_management_project_id
workload_folder = workload.folder
workload_folder_id = workload.folder_id
```

---


### Location

Get details of a Partner.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. The last time the resource was updated |
| `create_time` | String | Output only. Time the resource was created |
| `ekm_solutions` | Vec<String> | List of Google Cloud supported EKM partners supported by the partner |
| `operated_cloud_regions` | Vec<String> | List of Google Cloud regions that the partner sells services to customers. Valid Google Cloud regions found here: https://cloud.google.com/compute/docs/regions-zones |
| `name` | String | Identifier. The resource name of the partner. Format: `organizations/{organization}/locations/{location}/partner` Example: "organizations/123456/locations/us-central1/partner" |
| `partner_project_id` | String | Google Cloud project ID in the partner's Google Cloud organization for receiving enhanced Logs for Partners. |
| `skus` | Vec<String> | List of SKUs the partner is offering |


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
location_update_time = location.update_time
location_create_time = location.create_time
location_ekm_solutions = location.ekm_solutions
location_operated_cloud_regions = location.operated_cloud_regions
location_name = location.name
location_partner_project_id = location.partner_project_id
location_skus = location.skus
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
location_0 = provider.cloudcontrolspartner_api.Location {
}
location_1 = provider.cloudcontrolspartner_api.Location {
}
location_2 = provider.cloudcontrolspartner_api.Location {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    location = provider.cloudcontrolspartner_api.Location {
    }
```

---

## Related Documentation

- [GCP Cloudcontrolspartner_api Documentation](https://cloud.google.com/cloudcontrolspartner_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
