# Cloudbilling_api Service



**Resources**: 9

---

## Overview

The cloudbilling_api service provides access to 9 resource types:

- [Project](#project) [RU]
- [Sub_account](#sub_account) [CR]
- [Service](#service) [R]
- [Sku](#sku) [R]
- [Billing_account](#billing_account) [CRU]
- [Sku_group](#sku_group) [R]
- [Price](#price) [R]
- [Sku](#sku) [R]
- [Service](#service) [R]

---

## Resources


### Project

Gets the billing information for a project. The current authenticated user must have the `resourcemanager.projects.get` permission for the project, which can be granted by assigning the [Project Viewer](https://cloud.google.com/iam/docs/understanding-roles#predefined_roles) role.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. The resource name for the `ProjectBillingInfo`; has the form `projects/{project_id}/billingInfo`. For example, the resource name for the billing information for project `tokyo-rain-123` would be `projects/tokyo-rain-123/billingInfo`. |
| `billing_enabled` | bool |  | Output only. True if the project is associated with an open billing account, to which usage on the project is charged. False if the project is associated with a closed billing account, or no billing account at all, and therefore cannot use paid services. |
| `billing_account_name` | String |  | The resource name of the billing account associated with the project, if any. For example, `billingAccounts/012345-567890-ABCDEF`. |
| `project_id` | String |  | Output only. The ID of the project that this `ProjectBillingInfo` represents, such as `tokyo-rain-123`. This is a convenience field so that you don't need to parse the `name` field to obtain a project ID. |
| `name` | String | ✅ | Required. The resource name of the project associated with the billing information that you want to update. For example, `projects/tokyo-rain-123`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. The resource name for the `ProjectBillingInfo`; has the form `projects/{project_id}/billingInfo`. For example, the resource name for the billing information for project `tokyo-rain-123` would be `projects/tokyo-rain-123/billingInfo`. |
| `billing_enabled` | bool | Output only. True if the project is associated with an open billing account, to which usage on the project is charged. False if the project is associated with a closed billing account, or no billing account at all, and therefore cannot use paid services. |
| `billing_account_name` | String | The resource name of the billing account associated with the project, if any. For example, `billingAccounts/012345-567890-ABCDEF`. |
| `project_id` | String | Output only. The ID of the project that this `ProjectBillingInfo` represents, such as `tokyo-rain-123`. This is a convenience field so that you don't need to parse the `name` field to obtain a project ID. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access project outputs
project_id = project.id
project_name = project.name
project_billing_enabled = project.billing_enabled
project_billing_account_name = project.billing_account_name
project_project_id = project.project_id
```

---


### Sub_account

This method creates [billing subaccounts](https://cloud.google.com/billing/docs/concepts#subaccounts). Google Cloud resellers should use the Channel Services APIs, [accounts.customers.create](https://cloud.google.com/channel/docs/reference/rest/v1/accounts.customers/create) and [accounts.customers.entitlements.create](https://cloud.google.com/channel/docs/reference/rest/v1/accounts.customers.entitlements/create). When creating a subaccount, the current authenticated user must have the `billing.accounts.update` IAM permission on the parent account, which is typically given to billing account [administrators](https://cloud.google.com/billing/docs/how-to/billing-access). This method will return an error if the parent account has not been provisioned for subaccounts.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. The resource name of the billing account. The resource name has the form `billingAccounts/{billing_account_id}`. For example, `billingAccounts/012345-567890-ABCDEF` would be the resource name for billing account `012345-567890-ABCDEF`. |
| `parent` | String |  | Output only. The billing account's parent resource identifier. Use the `MoveBillingAccount` method to update the account's parent resource if it is a organization. Format: - `organizations/{organization_id}`, for example, `organizations/12345678` - `billingAccounts/{billing_account_id}`, for example, `billingAccounts/012345-567890-ABCDEF` |
| `open` | bool |  | Output only. True if the billing account is open, and will therefore be charged for any usage on associated projects. False if the billing account is closed, and therefore projects associated with it are unable to use paid services. |
| `display_name` | String |  | The display name given to the billing account, such as `My Billing Account`. This name is displayed in the Google Cloud Console. |
| `currency_code` | String |  | Optional. The currency in which the billing account is billed and charged, represented as an ISO 4217 code such as `USD`. Billing account currency is determined at the time of billing account creation and cannot be updated subsequently, so this field should not be set on update requests. In addition, a subaccount always matches the currency of its parent billing account, so this field should not be set on subaccount creation requests. Clients can read this field to determine the currency of an existing billing account. |
| `master_billing_account` | String |  | If this account is a [subaccount](https://cloud.google.com/billing/docs/concepts), then this will be the resource name of the parent billing account that it is being resold through. Otherwise this will be empty. |
| `parent` | String | ✅ | Optional. The parent to create a billing account from. Format: - `billingAccounts/{billing_account_id}`, for example, `billingAccounts/012345-567890-ABCDEF` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `billing_accounts` | Vec<String> | A list of billing accounts. |
| `next_page_token` | String | A token to retrieve the next page of results. To retrieve the next page, call `ListBillingAccounts` again with the `page_token` field set to this value. This field is empty if there are no more results to retrieve. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create sub_account
sub_account = provider.cloudbilling_api.Sub_account {
    parent = "value"  # Optional. The parent to create a billing account from. Format: - `billingAccounts/{billing_account_id}`, for example, `billingAccounts/012345-567890-ABCDEF`
}

# Access sub_account outputs
sub_account_id = sub_account.id
sub_account_billing_accounts = sub_account.billing_accounts
sub_account_next_page_token = sub_account.next_page_token
```

---


### Service

Lists all public cloud services.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `services` | Vec<String> | A list of services. |
| `next_page_token` | String | A token to retrieve the next page of results. To retrieve the next page, call `ListServices` again with the `page_token` field set to this value. This field is empty if there are no more results to retrieve. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access service outputs
service_id = service.id
service_services = service.services
service_next_page_token = service.next_page_token
```

---


### Sku

Lists all publicly available SKUs for a given cloud service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `skus` | Vec<String> | The list of public SKUs of the given service. |
| `next_page_token` | String | A token to retrieve the next page of results. To retrieve the next page, call `ListSkus` again with the `page_token` field set to this value. This field is empty if there are no more results to retrieve. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access sku outputs
sku_id = sku.id
sku_skus = sku.skus
sku_next_page_token = sku.next_page_token
```

---


### Billing_account

This method creates [billing subaccounts](https://cloud.google.com/billing/docs/concepts#subaccounts). Google Cloud resellers should use the Channel Services APIs, [accounts.customers.create](https://cloud.google.com/channel/docs/reference/rest/v1/accounts.customers/create) and [accounts.customers.entitlements.create](https://cloud.google.com/channel/docs/reference/rest/v1/accounts.customers.entitlements/create). When creating a subaccount, the current authenticated user must have the `billing.accounts.update` IAM permission on the parent account, which is typically given to billing account [administrators](https://cloud.google.com/billing/docs/how-to/billing-access). This method will return an error if the parent account has not been provisioned for subaccounts.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. The resource name of the billing account. The resource name has the form `billingAccounts/{billing_account_id}`. For example, `billingAccounts/012345-567890-ABCDEF` would be the resource name for billing account `012345-567890-ABCDEF`. |
| `parent` | String |  | Output only. The billing account's parent resource identifier. Use the `MoveBillingAccount` method to update the account's parent resource if it is a organization. Format: - `organizations/{organization_id}`, for example, `organizations/12345678` - `billingAccounts/{billing_account_id}`, for example, `billingAccounts/012345-567890-ABCDEF` |
| `open` | bool |  | Output only. True if the billing account is open, and will therefore be charged for any usage on associated projects. False if the billing account is closed, and therefore projects associated with it are unable to use paid services. |
| `display_name` | String |  | The display name given to the billing account, such as `My Billing Account`. This name is displayed in the Google Cloud Console. |
| `currency_code` | String |  | Optional. The currency in which the billing account is billed and charged, represented as an ISO 4217 code such as `USD`. Billing account currency is determined at the time of billing account creation and cannot be updated subsequently, so this field should not be set on update requests. In addition, a subaccount always matches the currency of its parent billing account, so this field should not be set on subaccount creation requests. Clients can read this field to determine the currency of an existing billing account. |
| `master_billing_account` | String |  | If this account is a [subaccount](https://cloud.google.com/billing/docs/concepts), then this will be the resource name of the parent billing account that it is being resold through. Otherwise this will be empty. |
| `parent` | String | ✅ | Optional. The parent to create a billing account from. Format: - `billingAccounts/{billing_account_id}`, for example, `billingAccounts/012345-567890-ABCDEF` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. The resource name of the billing account. The resource name has the form `billingAccounts/{billing_account_id}`. For example, `billingAccounts/012345-567890-ABCDEF` would be the resource name for billing account `012345-567890-ABCDEF`. |
| `parent` | String | Output only. The billing account's parent resource identifier. Use the `MoveBillingAccount` method to update the account's parent resource if it is a organization. Format: - `organizations/{organization_id}`, for example, `organizations/12345678` - `billingAccounts/{billing_account_id}`, for example, `billingAccounts/012345-567890-ABCDEF` |
| `open` | bool | Output only. True if the billing account is open, and will therefore be charged for any usage on associated projects. False if the billing account is closed, and therefore projects associated with it are unable to use paid services. |
| `display_name` | String | The display name given to the billing account, such as `My Billing Account`. This name is displayed in the Google Cloud Console. |
| `currency_code` | String | Optional. The currency in which the billing account is billed and charged, represented as an ISO 4217 code such as `USD`. Billing account currency is determined at the time of billing account creation and cannot be updated subsequently, so this field should not be set on update requests. In addition, a subaccount always matches the currency of its parent billing account, so this field should not be set on subaccount creation requests. Clients can read this field to determine the currency of an existing billing account. |
| `master_billing_account` | String | If this account is a [subaccount](https://cloud.google.com/billing/docs/concepts), then this will be the resource name of the parent billing account that it is being resold through. Otherwise this will be empty. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create billing_account
billing_account = provider.cloudbilling_api.Billing_account {
    parent = "value"  # Optional. The parent to create a billing account from. Format: - `billingAccounts/{billing_account_id}`, for example, `billingAccounts/012345-567890-ABCDEF`
}

# Access billing_account outputs
billing_account_id = billing_account.id
billing_account_name = billing_account.name
billing_account_parent = billing_account.parent
billing_account_open = billing_account.open
billing_account_display_name = billing_account.display_name
billing_account_currency_code = billing_account.currency_code
billing_account_master_billing_account = billing_account.master_billing_account
```

---


### Sku_group

Gets a publicly listed SKU group.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Resource name for the SKU group. Example: "skuGroups/0e6403d1-4694-44d2-a696-7a78b1a69301". |
| `display_name` | String | Description of the SKU group. Example: "A2 VMs (1 Year CUD)". |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access sku_group outputs
sku_group_id = sku_group.id
sku_group_name = sku_group.name
sku_group_display_name = sku_group.display_name
```

---


### Price

Gets the latest price for the given SKU.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Resource name for the latest price. |
| `value_type` | String | Type of the price. It can have values: ["unspecified", "rate"]. |
| `currency_code` | String | ISO-4217 currency code for the price. |
| `rate` | String | Rate price metadata. SKUs with `Rate` price are offered by pricing tiers. The price can have 1 or more rate pricing tiers. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access price outputs
price_id = price.id
price_name = price.name
price_value_type = price.value_type
price_currency_code = price.currency_code
price_rate = price.rate
```

---


### Sku

Gets a publicly listed SKU that is part of a publicly listed SKU group.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Resource name for the SkuGroupSku. Example: "skuGroups/0e6403d1-4694-44d2-a696-7a78b1a69301/skus/AA95-CD31-42FE". |
| `sku_id` | String | Unique identifier for the SKU. It is the string after the collection identifier "skus/" Example: "AA95-CD31-42FE". |
| `geo_taxonomy` | String | Geographic metadata that applies to the SkuGroupSku. |
| `display_name` | String | Description of the SkuGroupSku. Example: "A2 Instance Core running in Hong Kong". |
| `product_taxonomy` | String | List of product categories that apply to the SkuGroupSku. |
| `service` | String | Service that the SkuGroupSku belongs to. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access sku outputs
sku_id = sku.id
sku_name = sku.name
sku_sku_id = sku.sku_id
sku_geo_taxonomy = sku.geo_taxonomy
sku_display_name = sku.display_name
sku_product_taxonomy = sku.product_taxonomy
sku_service = sku.service
```

---


### Service

Gets a Google Cloud service visible to a billing account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `service_id` | String | Identifier for the service. It is the string after the collection identifier "services/". Example: "DA34-426B-A397". |
| `display_name` | String | Description of the BillingAccountService. Example: "BigQuery", "Compute Engine". |
| `name` | String | Resource name for the BillingAccountService. Example: "billingAccounts/012345-567890-ABCDEF/services/DA34-426B-A397". |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access service outputs
service_id = service.id
service_service_id = service.service_id
service_display_name = service.display_name
service_name = service.name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple project resources
project_0 = provider.cloudbilling_api.Project {
    name = "value-0"
}
project_1 = provider.cloudbilling_api.Project {
    name = "value-1"
}
project_2 = provider.cloudbilling_api.Project {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    project = provider.cloudbilling_api.Project {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Cloudbilling_api Documentation](https://cloud.google.com/cloudbilling_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
