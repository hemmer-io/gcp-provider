# Cloudchannel_api Service



**Resources**: 15

---

## Overview

The cloudchannel_api service provides access to 15 resource types:

- [Entitlement](#entitlement) [CR]
- [Report_job](#report_job) [C]
- [Operation](#operation) [CRD]
- [Customer](#customer) [CRUD]
- [Offer](#offer) [R]
- [Channel_partner_link](#channel_partner_link) [CRU]
- [Integrator](#integrator) [CR]
- [Account](#account) [CR]
- [Report](#report) [CR]
- [Channel_partner_repricing_config](#channel_partner_repricing_config) [CRUD]
- [Sku_group](#sku_group) [R]
- [Sku](#sku) [R]
- [Product](#product) [R]
- [Customer_repricing_config](#customer_repricing_config) [CRUD]
- [Billable_sku](#billable_sku) [R]

---

## Resources


### Entitlement

Creates an entitlement for a customer. Possible error codes: * PERMISSION_DENIED: * The customer doesn't belong to the reseller. * The reseller is not authorized to transact on this Product. See https://support.google.com/channelservices/answer/9759265 * INVALID_ARGUMENT: * Required request parameters are missing or invalid. * There is already a customer entitlement for a SKU from the same product family. * INVALID_VALUE: Make sure the OfferId is valid. If it is, contact Google Channel support for further troubleshooting. * NOT_FOUND: The customer or offer resource was not found. * ALREADY_EXISTS: * The SKU was already purchased for the customer. * The customer's primary email already exists. Retry after changing the customer's primary contact email. * CONDITION_NOT_MET or FAILED_PRECONDITION: * The domain required for purchasing a SKU has not been verified. * A pre-requisite SKU required to purchase an Add-On SKU is missing. For example, Google Workspace Business Starter is required to purchase Vault or Drive. * (Developer accounts only) Reseller and resold domain must meet the following naming requirements: * Domain names must start with goog-test. * Domain names must include the reseller domain. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The Operation metadata will contain an instance of OperationMetadata.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `request_id` | String |  | Optional. You can specify an optional unique request ID, and if you need to retry your request, the server will know to ignore the request if it's complete. For example, you make an initial request and the request times out. If you make the request again with the same request ID, the server can check if it received the original operation with the same request ID. If it did, it will ignore the second request. The request ID must be a valid [UUID](https://tools.ietf.org/html/rfc4122) with the exception that zero UUID is not supported (`00000000-0000-0000-0000-000000000000`). |
| `entitlement` | String |  | Required. The entitlement to create. |
| `parent` | String | ✅ | Required. The resource name of the reseller's customer account in which to create the entitlement. Parent uses the format: accounts/{account_id}/customers/{customer_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `commitment_settings` | String | Commitment settings for a commitment-based Offer. Required for commitment based offers. |
| `association_info` | String | Association information to other entitlements. |
| `suspension_reasons` | Vec<String> | Output only. Enumerable of all current suspension reasons for an entitlement. |
| `trial_settings` | String | Output only. Settings for trial offers. |
| `purchase_order_id` | String | Optional. This purchase order (PO) information is for resellers to use for their company tracking usage. If a purchaseOrderId value is given, it appears in the API responses and shows up in the invoice. The property accepts up to 80 plain text characters. This is only supported for Google Workspace entitlements. |
| `provisioned_service` | String | Output only. Service provisioning details for the entitlement. |
| `parameters` | Vec<String> | Extended entitlement parameters. When creating an entitlement, valid parameter names and values are defined in the Offer.parameter_definitions. For Google Workspace, the following Parameters may be accepted as input: - max_units: The maximum assignable units for a flexible offer OR - num_units: The total commitment for commitment-based offers The response may additionally include the following output-only Parameters: - assigned_units: The number of licenses assigned to users. For Google Cloud billing subaccounts, the following Parameter may be accepted as input: - display_name: The display name of the billing subaccount. |
| `provisioning_state` | String | Output only. Current provisioning state of the entitlement. |
| `price_reference_id` | String | Optional. Price reference ID for the offer. Only for offers that require additional price information. Used to guarantee that the pricing is consistent between quoting the offer and placing the order. |
| `update_time` | String | Output only. The time at which the entitlement is updated. |
| `name` | String | Output only. Resource name of an entitlement in the form: accounts/{account_id}/customers/{customer_id}/entitlements/{entitlement_id}. |
| `billing_account` | String | Optional. The billing account resource name that is used to pay for this entitlement. |
| `offer` | String | Required. The offer resource name for which the entitlement is to be created. Takes the form: accounts/{account_id}/offers/{offer_id}. |
| `create_time` | String | Output only. The time at which the entitlement is created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create entitlement
entitlement = provider.cloudchannel_api.Entitlement {
    parent = "value"  # Required. The resource name of the reseller's customer account in which to create the entitlement. Parent uses the format: accounts/{account_id}/customers/{customer_id}
}

# Access entitlement outputs
entitlement_id = entitlement.id
entitlement_commitment_settings = entitlement.commitment_settings
entitlement_association_info = entitlement.association_info
entitlement_suspension_reasons = entitlement.suspension_reasons
entitlement_trial_settings = entitlement.trial_settings
entitlement_purchase_order_id = entitlement.purchase_order_id
entitlement_provisioned_service = entitlement.provisioned_service
entitlement_parameters = entitlement.parameters
entitlement_provisioning_state = entitlement.provisioning_state
entitlement_price_reference_id = entitlement.price_reference_id
entitlement_update_time = entitlement.update_time
entitlement_name = entitlement.name
entitlement_billing_account = entitlement.billing_account
entitlement_offer = entitlement.offer
entitlement_create_time = entitlement.create_time
```

---


### Report_job

Retrieves data generated by CloudChannelReportsService.RunReportJob. Deprecated: Please use [Export Channel Services data to BigQuery](https://cloud.google.com/channel/docs/rebilling/export-data-to-bigquery) instead.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `partition_keys` | Vec<String> |  | Optional. List of keys specifying which report partitions to return. If empty, returns all partitions. |
| `page_size` | i64 |  | Optional. Requested page size of the report. The server may return fewer results than requested. If you don't specify a page size, the server uses a sensible default (may change over time). The maximum value is 30,000; the server will change larger values to 30,000. |
| `page_token` | String |  | Optional. A token that specifies a page of results beyond the first page. Obtained through FetchReportResultsResponse.next_page_token of the previous CloudChannelReportsService.FetchReportResults call. |
| `report_job` | String | ✅ | Required. The report job created by CloudChannelReportsService.RunReportJob. Report_job uses the format: accounts/{account_id}/reportJobs/{report_job_id} |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create report_job
report_job = provider.cloudchannel_api.Report_job {
    report_job = "value"  # Required. The report job created by CloudChannelReportsService.RunReportJob. Report_job uses the format: accounts/{account_id}/reportJobs/{report_job_id}
}

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
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
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
operation = provider.cloudchannel_api.Operation {
    name = "value"  # The name of the operation resource to be cancelled.
}

# Access operation outputs
operation_id = operation.id
operation_response = operation.response
operation_done = operation.done
operation_metadata = operation.metadata
operation_error = operation.error
operation_name = operation.name
```

---


### Customer

Creates a new Customer resource under the reseller or distributor account. Possible error codes: * PERMISSION_DENIED: * The reseller account making the request is different from the reseller account in the API request. * You are not authorized to create a customer. See https://support.google.com/channelservices/answer/9759265 * INVALID_ARGUMENT: * Required request parameters are missing or invalid. * Domain field value doesn't match the primary email domain. Return value: The newly created Customer resource.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `create_time` | String |  | Output only. Time when the customer was created. |
| `domain` | String |  | Required. The customer's primary domain. Must match the primary contact email's domain. |
| `cloud_identity_id` | String |  | Output only. The customer's Cloud Identity ID if the customer has a Cloud Identity resource. |
| `primary_contact_info` | String |  | Primary contact info. |
| `alternate_email` | String |  | Secondary contact email. You need to provide an alternate email to create different domains if a primary contact email already exists. Users will receive a notification with credentials when you create an admin.google.com account. Secondary emails are also recovery email addresses. Alternate emails are optional when you create Team customers. |
| `channel_partner_id` | String |  | Cloud Identity ID of the customer's channel partner. Populated only if a channel partner exists for this customer. |
| `customer_attestation_state` | String |  | Optional. Indicate if a customer is attesting about the correctness of provided information. Only required if creating a GCP Entitlement. |
| `name` | String |  | Output only. Resource name of the customer. Format: accounts/{account_id}/customers/{customer_id} |
| `org_postal_address` | String |  | Required. The organization address for the customer. To enforce US laws and embargoes, we require a region, postal code, and address lines. You must provide valid addresses for every customer. To set the customer's language, use the Customer-level language code. |
| `update_time` | String |  | Output only. Time when the customer was updated. |
| `correlation_id` | String |  | Optional. External CRM ID for the customer. Populated only if a CRM ID exists for this customer. |
| `cloud_identity_info` | String |  | Output only. Cloud Identity information for the customer. Populated only if a Cloud Identity account exists for this customer. |
| `language_code` | String |  | Optional. The BCP-47 language code, such as "en-US" or "sr-Latn". For more information, see https://www.unicode.org/reports/tr35/#Unicode_locale_identifier. |
| `org_display_name` | String |  | Required. Name of the organization that the customer entity represents. |
| `parent` | String | ✅ | Required. The resource name of reseller account in which to create the customer. Parent uses the format: accounts/{account_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `create_time` | String | Output only. Time when the customer was created. |
| `domain` | String | Required. The customer's primary domain. Must match the primary contact email's domain. |
| `cloud_identity_id` | String | Output only. The customer's Cloud Identity ID if the customer has a Cloud Identity resource. |
| `primary_contact_info` | String | Primary contact info. |
| `alternate_email` | String | Secondary contact email. You need to provide an alternate email to create different domains if a primary contact email already exists. Users will receive a notification with credentials when you create an admin.google.com account. Secondary emails are also recovery email addresses. Alternate emails are optional when you create Team customers. |
| `channel_partner_id` | String | Cloud Identity ID of the customer's channel partner. Populated only if a channel partner exists for this customer. |
| `customer_attestation_state` | String | Optional. Indicate if a customer is attesting about the correctness of provided information. Only required if creating a GCP Entitlement. |
| `name` | String | Output only. Resource name of the customer. Format: accounts/{account_id}/customers/{customer_id} |
| `org_postal_address` | String | Required. The organization address for the customer. To enforce US laws and embargoes, we require a region, postal code, and address lines. You must provide valid addresses for every customer. To set the customer's language, use the Customer-level language code. |
| `update_time` | String | Output only. Time when the customer was updated. |
| `correlation_id` | String | Optional. External CRM ID for the customer. Populated only if a CRM ID exists for this customer. |
| `cloud_identity_info` | String | Output only. Cloud Identity information for the customer. Populated only if a Cloud Identity account exists for this customer. |
| `language_code` | String | Optional. The BCP-47 language code, such as "en-US" or "sr-Latn". For more information, see https://www.unicode.org/reports/tr35/#Unicode_locale_identifier. |
| `org_display_name` | String | Required. Name of the organization that the customer entity represents. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create customer
customer = provider.cloudchannel_api.Customer {
    parent = "value"  # Required. The resource name of reseller account in which to create the customer. Parent uses the format: accounts/{account_id}
}

# Access customer outputs
customer_id = customer.id
customer_create_time = customer.create_time
customer_domain = customer.domain
customer_cloud_identity_id = customer.cloud_identity_id
customer_primary_contact_info = customer.primary_contact_info
customer_alternate_email = customer.alternate_email
customer_channel_partner_id = customer.channel_partner_id
customer_customer_attestation_state = customer.customer_attestation_state
customer_name = customer.name
customer_org_postal_address = customer.org_postal_address
customer_update_time = customer.update_time
customer_correlation_id = customer.correlation_id
customer_cloud_identity_info = customer.cloud_identity_info
customer_language_code = customer.language_code
customer_org_display_name = customer.org_display_name
```

---


### Offer

Lists the Offers the reseller can sell. Possible error codes: * INVALID_ARGUMENT: Required request parameters are missing or invalid.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `offers` | Vec<String> | The list of Offers requested. The pricing information for each Offer only includes the base price. Effective prices and discounts aren't populated. |
| `next_page_token` | String | A token to retrieve the next page of results. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access offer outputs
offer_id = offer.id
offer_offers = offer.offers
offer_next_page_token = offer.next_page_token
```

---


### Channel_partner_link

Initiates a channel partner link between a distributor and a reseller, or between resellers in an n-tier reseller channel. Invited partners need to follow the invite_link_uri provided in the response to accept. After accepting the invitation, a link is set up between the two parties. You must be a distributor to call this method. Possible error codes: * PERMISSION_DENIED: The reseller account making the request is different from the reseller account in the API request. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * ALREADY_EXISTS: The ChannelPartnerLink sent in the request already exists. * NOT_FOUND: No Cloud Identity customer exists for provided domain. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The new ChannelPartnerLink resource.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. Timestamp of when the channel partner link is updated. |
| `link_state` | String |  | Required. State of the channel partner link. |
| `invite_link_uri` | String |  | Output only. URI of the web page where partner accepts the link invitation. |
| `name` | String |  | Output only. Resource name for the channel partner link, in the format accounts/{account_id}/channelPartnerLinks/{id}. |
| `public_id` | String |  | Output only. Public identifier that a customer must use to generate a transfer token to move to this distributor-reseller combination. |
| `reseller_cloud_identity_id` | String |  | Required. Cloud Identity ID of the linked reseller. |
| `channel_partner_cloud_identity_info` | String |  | Output only. Cloud Identity info of the channel partner (IR). |
| `create_time` | String |  | Output only. Timestamp of when the channel partner link is created. |
| `parent` | String | ✅ | Required. Create a channel partner link for the provided reseller account's resource name. Parent uses the format: accounts/{account_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. Timestamp of when the channel partner link is updated. |
| `link_state` | String | Required. State of the channel partner link. |
| `invite_link_uri` | String | Output only. URI of the web page where partner accepts the link invitation. |
| `name` | String | Output only. Resource name for the channel partner link, in the format accounts/{account_id}/channelPartnerLinks/{id}. |
| `public_id` | String | Output only. Public identifier that a customer must use to generate a transfer token to move to this distributor-reseller combination. |
| `reseller_cloud_identity_id` | String | Required. Cloud Identity ID of the linked reseller. |
| `channel_partner_cloud_identity_info` | String | Output only. Cloud Identity info of the channel partner (IR). |
| `create_time` | String | Output only. Timestamp of when the channel partner link is created. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create channel_partner_link
channel_partner_link = provider.cloudchannel_api.Channel_partner_link {
    parent = "value"  # Required. Create a channel partner link for the provided reseller account's resource name. Parent uses the format: accounts/{account_id}
}

# Access channel_partner_link outputs
channel_partner_link_id = channel_partner_link.id
channel_partner_link_update_time = channel_partner_link.update_time
channel_partner_link_link_state = channel_partner_link.link_state
channel_partner_link_invite_link_uri = channel_partner_link.invite_link_uri
channel_partner_link_name = channel_partner_link.name
channel_partner_link_public_id = channel_partner_link.public_id
channel_partner_link_reseller_cloud_identity_id = channel_partner_link.reseller_cloud_identity_id
channel_partner_link_channel_partner_cloud_identity_info = channel_partner_link.channel_partner_cloud_identity_info
channel_partner_link_create_time = channel_partner_link.create_time
```

---


### Integrator

Registers a service account with subscriber privileges on the Pub/Sub topic for this Channel Services account or integrator. After you create a subscriber, you get the events through SubscriberEvent Possible error codes: * PERMISSION_DENIED: The reseller account making the request and the provided reseller account are different, or the impersonated user is not a super admin. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The topic name with the registered service email address.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `integrator` | String |  | Optional. Resource name of the integrator. Required if account is not provided. Otherwise, leave this field empty/unset. |
| `account` | String |  | Optional. Resource name of the account. Required if integrator is not provided. Otherwise, leave this field empty/unset. |
| `service_account` | String |  | Required. Service account that provides subscriber access to the registered topic. |
| `integrator` | String | ✅ | Optional. Resource name of the integrator. Required if account is not provided. Otherwise, leave this field empty/unset. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `topic` | String | Name of the topic registered with the reseller. |
| `service_accounts` | Vec<String> | List of service accounts which have subscriber access to the topic. |
| `next_page_token` | String | A token that can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create integrator
integrator = provider.cloudchannel_api.Integrator {
    integrator = "value"  # Optional. Resource name of the integrator. Required if account is not provided. Otherwise, leave this field empty/unset.
}

# Access integrator outputs
integrator_id = integrator.id
integrator_topic = integrator.topic
integrator_service_accounts = integrator.service_accounts
integrator_next_page_token = integrator.next_page_token
```

---


### Account

List TransferableOffers of a customer based on Cloud Identity ID or Customer Name in the request. Use this method when a reseller gets the entitlement information of an unowned customer. The reseller should provide the customer's Cloud Identity ID or Customer Name. Possible error codes: * PERMISSION_DENIED: * The customer doesn't belong to the reseller and has no auth token. * The customer provided incorrect reseller information when generating auth token. * The reseller account making the request is different from the reseller account in the query. * The reseller is not authorized to transact on this Product. See https://support.google.com/channelservices/answer/9759265 * INVALID_ARGUMENT: Required request parameters are missing or invalid. Return value: List of TransferableOffer for the given customer and SKU.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `billing_account` | String |  | Optional. The Billing Account to look up Offers for. Format: accounts/{account_id}/billingAccounts/{billing_account_id}. This field is only relevant for multi-currency accounts. It should be left empty for single currency accounts. |
| `language_code` | String |  | Optional. The BCP-47 language code. For example, "en-US". The response will localize in the corresponding language code, if specified. The default value is "en-US". |
| `cloud_identity_id` | String |  | Customer's Cloud Identity ID |
| `page_token` | String |  | A token for a page of results other than the first page. Obtained using ListTransferableOffersResponse.next_page_token of the previous CloudChannelService.ListTransferableOffers call. |
| `customer_name` | String |  | A reseller should create a customer and use the resource name of that customer here. |
| `sku` | String |  | Required. The SKU to look up Offers for. |
| `page_size` | i64 |  | Requested page size. Server might return fewer results than requested. If unspecified, returns at most 100 offers. The maximum value is 1000; the server will coerce values above 1000. |
| `parent` | String | ✅ | Required. The resource name of the reseller's account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `topic` | String | Name of the topic registered with the reseller. |
| `service_accounts` | Vec<String> | List of service accounts which have subscriber access to the topic. |
| `next_page_token` | String | A token that can be sent as `page_token` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create account
account = provider.cloudchannel_api.Account {
    parent = "value"  # Required. The resource name of the reseller's account.
}

# Access account outputs
account_id = account.id
account_topic = account.topic
account_service_accounts = account.service_accounts
account_next_page_token = account.next_page_token
```

---


### Report

Begins generation of data for a given report. The report identifier is a UID (for example, `613bf59q`). Possible error codes: * PERMISSION_DENIED: The user doesn't have access to this report. * INVALID_ARGUMENT: Required request parameters are missing or invalid. * NOT_FOUND: The report identifier was not found. * INTERNAL: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. * UNKNOWN: Any non-user error related to a technical issue in the backend. Contact Cloud Channel support. Return value: The ID of a long-running operation. To get the results of the operation, call the GetOperation method of CloudChannelOperationsService. The Operation metadata contains an instance of OperationMetadata. To get the results of report generation, call CloudChannelReportsService.FetchReportResults with the RunReportJobResponse.report_job. Deprecated: Please use [Export Channel Services data to BigQuery](https://cloud.google.com/channel/docs/rebilling/export-data-to-bigquery) instead.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `language_code` | String |  | Optional. The BCP-47 language code, such as "en-US". If specified, the response is localized to the corresponding language code if the original data sources support it. Default is "en-US". |
| `date_range` | String |  | Optional. The range of usage or invoice dates to include in the result. |
| `filter` | String |  | Optional. A structured string that defines conditions on dimension columns to restrict the report output. Filters support logical operators (AND, OR, NOT) and conditional operators (=, !=, <, >, <=, and >=) using `column_id` as keys. For example: `(customer:"accounts/C123abc/customers/S456def" OR customer:"accounts/C123abc/customers/S789ghi") AND invoice_start_date.year >= 2022` |
| `name` | String | ✅ | Required. The report's resource name. Specifies the account and report used to generate report data. The report_id identifier is a UID (for example, `613bf59q`). Name uses the format: accounts/{account_id}/reports/{report_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Pass this token to FetchReportResultsRequest.page_token to retrieve the next page of results. |
| `reports` | Vec<String> | The reports available to the partner. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create report
report = provider.cloudchannel_api.Report {
    name = "value"  # Required. The report's resource name. Specifies the account and report used to generate report data. The report_id identifier is a UID (for example, `613bf59q`). Name uses the format: accounts/{account_id}/reports/{report_id}
}

# Access report outputs
report_id = report.id
report_next_page_token = report.next_page_token
report_reports = report.reports
```

---


### Channel_partner_repricing_config

Creates a ChannelPartnerRepricingConfig. Call this method to set modifications for a specific ChannelPartner's bill. You can only create configs if the RepricingConfig.effective_invoice_month is a future month. If needed, you can create a config for the current month, with some restrictions. When creating a config for a future month, make sure there are no existing configs for that RepricingConfig.effective_invoice_month. The following restrictions are for creating configs in the current month. * This functionality is reserved for recovering from an erroneous config, and should not be used for regular business cases. * The new config will not modify exports used with other configs. Changes to the config may be immediate, but may take up to 24 hours. * There is a limit of ten configs for any ChannelPartner or RepricingConfig.EntitlementGranularity.entitlement, for any RepricingConfig.effective_invoice_month. * The contained ChannelPartnerRepricingConfig.repricing_config value must be different from the value used in the current config for a ChannelPartner. Possible Error Codes: * PERMISSION_DENIED: If the account making the request and the account being queried are different. * INVALID_ARGUMENT: Missing or invalid required parameters in the request. Also displays if the updated config is for the current month or past months. * NOT_FOUND: The ChannelPartnerRepricingConfig specified does not exist or is not associated with the given account. * INTERNAL: Any non-user error related to technical issues in the backend. In this case, contact Cloud Channel support. Return Value: If successful, the updated ChannelPartnerRepricingConfig resource, otherwise returns an error.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. Timestamp of an update to the repricing rule. If `update_time` is after RepricingConfig.effective_invoice_month then it indicates this was set mid-month. |
| `name` | String |  | Output only. Resource name of the ChannelPartnerRepricingConfig. Format: accounts/{account_id}/channelPartnerLinks/{channel_partner_id}/channelPartnerRepricingConfigs/{id}. |
| `repricing_config` | String |  | Required. The configuration for bill modifications made by a reseller before sending it to ChannelPartner. |
| `parent` | String | ✅ | Required. The resource name of the ChannelPartner that will receive the repricing config. Parent uses the format: accounts/{account_id}/channelPartnerLinks/{channel_partner_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. Timestamp of an update to the repricing rule. If `update_time` is after RepricingConfig.effective_invoice_month then it indicates this was set mid-month. |
| `name` | String | Output only. Resource name of the ChannelPartnerRepricingConfig. Format: accounts/{account_id}/channelPartnerLinks/{channel_partner_id}/channelPartnerRepricingConfigs/{id}. |
| `repricing_config` | String | Required. The configuration for bill modifications made by a reseller before sending it to ChannelPartner. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create channel_partner_repricing_config
channel_partner_repricing_config = provider.cloudchannel_api.Channel_partner_repricing_config {
    parent = "value"  # Required. The resource name of the ChannelPartner that will receive the repricing config. Parent uses the format: accounts/{account_id}/channelPartnerLinks/{channel_partner_id}
}

# Access channel_partner_repricing_config outputs
channel_partner_repricing_config_id = channel_partner_repricing_config.id
channel_partner_repricing_config_update_time = channel_partner_repricing_config.update_time
channel_partner_repricing_config_name = channel_partner_repricing_config.name
channel_partner_repricing_config_repricing_config = channel_partner_repricing_config.repricing_config
```

---


### Sku_group

Lists the Rebilling supported SKU groups the account is authorized to sell. Reference: https://cloud.google.com/skus/sku-groups Possible Error Codes: * PERMISSION_DENIED: If the account making the request and the account being queried are different, or the account doesn't exist. * INTERNAL: Any non-user error related to technical issues in the backend. In this case, contact Cloud Channel support. Return Value: If successful, the SkuGroup resources. The data for each resource is displayed in the alphabetical order of SKU group display name. The data for each resource is displayed in the ascending order of SkuGroup.display_name If unsuccessful, returns an error.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token to retrieve the next page of results. Pass to ListSkuGroupsRequest.page_token to obtain that page. |
| `sku_groups` | Vec<String> | The list of SKU groups requested. |


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
sku_group_next_page_token = sku_group.next_page_token
sku_group_sku_groups = sku_group.sku_groups
```

---


### Sku

Lists the SKUs for a product the reseller is authorized to sell. Possible error codes: * INVALID_ARGUMENT: Required request parameters are missing or invalid.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `skus` | Vec<String> | The list of SKUs requested. |
| `next_page_token` | String | A token to retrieve the next page of results. |


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


### Product

Lists the Products the reseller is authorized to sell. Possible error codes: * INVALID_ARGUMENT: Required request parameters are missing or invalid.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token to retrieve the next page of results. |
| `products` | Vec<String> | List of Products requested. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access product outputs
product_id = product.id
product_next_page_token = product.next_page_token
product_products = product.products
```

---


### Customer_repricing_config

Creates a CustomerRepricingConfig. Call this method to set modifications for a specific customer's bill. You can only create configs if the RepricingConfig.effective_invoice_month is a future month. If needed, you can create a config for the current month, with some restrictions. When creating a config for a future month, make sure there are no existing configs for that RepricingConfig.effective_invoice_month. The following restrictions are for creating configs in the current month. * This functionality is reserved for recovering from an erroneous config, and should not be used for regular business cases. * The new config will not modify exports used with other configs. Changes to the config may be immediate, but may take up to 24 hours. * There is a limit of ten configs for any RepricingConfig.EntitlementGranularity.entitlement, for any RepricingConfig.effective_invoice_month. * The contained CustomerRepricingConfig.repricing_config value must be different from the value used in the current config for a RepricingConfig.EntitlementGranularity.entitlement. Possible Error Codes: * PERMISSION_DENIED: If the account making the request and the account being queried are different. * INVALID_ARGUMENT: Missing or invalid required parameters in the request. Also displays if the updated config is for the current month or past months. * NOT_FOUND: The CustomerRepricingConfig specified does not exist or is not associated with the given account. * INTERNAL: Any non-user error related to technical issues in the backend. In this case, contact Cloud Channel support. Return Value: If successful, the updated CustomerRepricingConfig resource, otherwise returns an error.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Resource name of the CustomerRepricingConfig. Format: accounts/{account_id}/customers/{customer_id}/customerRepricingConfigs/{id}. |
| `repricing_config` | String |  | Required. The configuration for bill modifications made by a reseller before sending it to customers. |
| `update_time` | String |  | Output only. Timestamp of an update to the repricing rule. If `update_time` is after RepricingConfig.effective_invoice_month then it indicates this was set mid-month. |
| `parent` | String | ✅ | Required. The resource name of the customer that will receive this repricing config. Parent uses the format: accounts/{account_id}/customers/{customer_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Resource name of the CustomerRepricingConfig. Format: accounts/{account_id}/customers/{customer_id}/customerRepricingConfigs/{id}. |
| `repricing_config` | String | Required. The configuration for bill modifications made by a reseller before sending it to customers. |
| `update_time` | String | Output only. Timestamp of an update to the repricing rule. If `update_time` is after RepricingConfig.effective_invoice_month then it indicates this was set mid-month. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create customer_repricing_config
customer_repricing_config = provider.cloudchannel_api.Customer_repricing_config {
    parent = "value"  # Required. The resource name of the customer that will receive this repricing config. Parent uses the format: accounts/{account_id}/customers/{customer_id}
}

# Access customer_repricing_config outputs
customer_repricing_config_id = customer_repricing_config.id
customer_repricing_config_name = customer_repricing_config.name
customer_repricing_config_repricing_config = customer_repricing_config.repricing_config
customer_repricing_config_update_time = customer_repricing_config.update_time
```

---


### Billable_sku

Lists the Billable SKUs in a given SKU group. Possible error codes: PERMISSION_DENIED: If the account making the request and the account being queried for are different, or the account doesn't exist. INVALID_ARGUMENT: Missing or invalid required parameters in the request. INTERNAL: Any non-user error related to technical issue in the backend. In this case, contact cloud channel support. Return Value: If successful, the BillableSku resources. The data for each resource is displayed in the ascending order of: * BillableSku.service_display_name * BillableSku.sku_display_name If unsuccessful, returns an error.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `billable_skus` | Vec<String> | The list of billable SKUs in the requested SKU group. |
| `next_page_token` | String | A token to retrieve the next page of results. Pass to ListSkuGroupBillableSkusRequest.page_token to obtain that page. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access billable_sku outputs
billable_sku_id = billable_sku.id
billable_sku_billable_skus = billable_sku.billable_skus
billable_sku_next_page_token = billable_sku.next_page_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple entitlement resources
entitlement_0 = provider.cloudchannel_api.Entitlement {
    parent = "value-0"
}
entitlement_1 = provider.cloudchannel_api.Entitlement {
    parent = "value-1"
}
entitlement_2 = provider.cloudchannel_api.Entitlement {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    entitlement = provider.cloudchannel_api.Entitlement {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Cloudchannel_api Documentation](https://cloud.google.com/cloudchannel_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
