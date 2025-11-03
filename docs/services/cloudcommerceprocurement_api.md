# Cloudcommerceprocurement_api Service



**Resources**: 2

---

## Overview

The cloudcommerceprocurement_api service provides access to 2 resource types:

- [Entitlement](#entitlement) [CRU]
- [Account](#account) [CR]

---

## Resources


### Entitlement

Requests suspension of an active Entitlement. This is not yet supported.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `reason` | String |  | A free-form reason string, explaining the reason for suspension request. |
| `name` | String | ✅ | Required. The name of the entitlement to suspend. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `entitlement_benefit_ids` | Vec<String> | Output only. The entitlement benefit IDs associated with the purchase. |
| `offer_duration` | String | Output only. The offer duration of the current offer, in ISO 8601 duration format. This is empty if the entitlement wasn't made using an offer, or if the offer has a specified end date instead of a duration. * If the entitlement is in the state ENTITLEMENT_ACTIVATION_REQUESTED, and the upcoming offer doesn't have a specified end date, then this field is populated with the duration of the upcoming offer. Otherwise, this field is empty. * If the entitlement is in the state ENTITLEMENT_ACTIVE, ENTITLEMENT_PENDING_CANCELLATION, ENTITLEMENT_PENDING_PLAN_CHANGE, or ENTITLEMENT_PENDING_PLAN_CHANGE_APPROVAL, and the current offer doesn't have a specified end date, then this field contains the duration of the current offer. Otherwise, this field is empty. * If the entitlement is in the state ENTITLEMENT_CANCELLED, and the offer doesn't have a specified end date, then this field is populated with the duration of the latest offer that the order was associated with. Otherwise, this field is empty. |
| `order_id` | String | Output only. The order ID of this entitlement, without any `orders/` resource name prefix. |
| `state` | String | Output only. The state of the entitlement. |
| `new_pending_offer` | String | Output only. Upon a pending plan change, the name of the offer that the entitlement is switching to. Only exists if the pending plan change is moving to an offer. This field isn't populated for entitlements which aren't active yet. Format: 'projects/{project}/services/{service}/privateOffers/{offer}' OR 'projects/{project}/services/{service}/standardOffers/{offer}', depending on whether the offer is private or public. The {service} in the name is the listing service of the offer. It could be either the product service that the offer is referencing, or a generic private offer parent service. We recommend that you don't build your integration to rely on the meaning of this {service} part. * If the entitlement is in the state ENTITLEMENT_ACTIVATION_REQUESTED, ENTITLEMENT_ACTIVE or ENTITLEMENT_PENDING_CANCELLATION, then this field is empty. * If the entitlement is in the state ENTITLEMENT_PENDING_PLAN_CHANGE_APPROVAL or ENTITLEMENT_PENDING_PLAN_CHANGE, then this field is populated with the upcoming offer. * If the entitlement is in the state ENTITLEMENT_CANCELLED, then this is empty. |
| `create_time` | String | Output only. The creation timestamp. |
| `name` | String | Output only. The resource name of the entitlement. Entitlement names have the form `providers/{provider_id}/entitlements/{entitlement_id}`. |
| `new_offer_end_time` | String | Output only. The end time of the new offer, determined from the offer's specified end date. If the offer des not have a specified end date then this field is not set. This field is populated even if the entitlement isn't active yet. If there's no upcoming offer, the field is empty. * If the entitlement is in the state ENTITLEMENT_ACTIVATION_REQUESTED, ENTITLEMENT_ACTIVE, or ENTITLEMENT_PENDING_CANCELLATION, then this field is empty. * If the entitlement is in the state ENTITLEMENT_PENDING_PLAN_CHANGE_APPROVAL or ENTITLEMENT_PENDING_PLAN_CHANGE, and the upcoming offer has a specified end date, then this field is populated with the expected end time of the upcoming offer, in the future. Otherwise, this field is empty. * If the entitlement is in the state ENTITLEMENT_CANCELLED, then this field is empty. |
| `input_properties` | HashMap<String, String> | Output only. The custom properties that were collected from the user to create this entitlement. |
| `offer_end_time` | String | Output only. End time for the current term of the Offer associated with this entitlement. The value of this field can change naturally over time due to auto-renewal, even if the offer isn't changed. * If the entitlement is in the state ENTITLEMENT_ACTIVATION_REQUESTED, then: * If the entitlement isn't approved yet approved, and the offer has a specified end date, then this field is populated with the expected end time of the upcoming offer, in the future. Otherwise, this field is empty. * If the entitlement is approved, then this field is populated with the expected end time of the upcoming offer, in the future. This means that this field and the field offer_duration can both exist. * If the entitlement is in the state ENTITLEMENT_ACTIVE or ENTITLEMENT_PENDING_CANCELLATION, then this field is populated with the expected end time of the current offer, in the future. This field's value is set regardless of whether the offer has a specific end date or a duration. This means that this field and the field offer_duration can both exist. * If the entitlement is in the state ENTITLEMENT_PENDING_PLAN_CHANGE_APPROVAL or ENTITLEMENT_PENDING_PLAN_CHANGE: * If the entitlement's pricing model is usage based and the associated offer is a private offer whose term has ended, then this field reflects the ACTUAL end time of the entitlement's associated offer (in the past), even though the entitlement associated with this private offer does not terminate at the end of that private offer's term. * Otherwise, this is the expected end date of the current offer, in the future. * If the entitlement is in the state ENTITLEMENT_CANCELLED, then this field is populated with the end time, in the past, of the latest offer that the order was associated with. If the entitlement was cancelled before any offer started, then this field is empty. |
| `provider` | String | Output only. The identifier of the service provider that this entitlement was created against. Each service provider is assigned a unique provider value when they onboard with Cloud Commerce platform. |
| `subscription_end_time` | String | Output only. End time for the subscription corresponding to this entitlement. |
| `message_to_user` | String | Provider-supplied message that is displayed to the end user. Currently this is used to communicate progress and ETA for provisioning. This field can be updated only when a user is waiting for an action from the provider, i.e. entitlement state is EntitlementState.ENTITLEMENT_ACTIVATION_REQUESTED or EntitlementState.ENTITLEMENT_PENDING_PLAN_CHANGE_APPROVAL. This field is cleared automatically when the entitlement state changes. |
| `new_pending_plan` | String | Output only. The identifier of the pending new plan. Required if the product has plans and the entitlement has a pending plan change. |
| `consumers` | Vec<String> | Output only. The resources using this entitlement, if applicable. |
| `new_offer_start_time` | String | Output only. The timestamp when the new offer becomes effective. This field is populated even if the entitlement isn't active yet. If there's no upcoming offer, the field is empty. * If the entitlement is in the state ENTITLEMENT_ACTIVATION_REQUESTED, this field isn't populated when the entitlement isn't yet approved. After the entitlement is approved, this field is populated with the effective time of the upcoming offer. * If the entitlement is in the state ENTITLEMENT_ACTIVE or ENTITLEMENT_PENDING_CANCELLATION, this field isn't populated. * If the entitlement is in the state ENTITLEMENT_PENDING_PLAN_CHANGE_APPROVAL, this field isn't populated, because the entitlement change is waiting on approval. * If the entitlement is in the state ENTITLEMENT_PENDING_PLAN_CHANGE, this field is populated with the expected effective time of the upcoming offer, which is in the future. * If the entitlement is in the state ENTITLEMENT_CANCELLED, then this field is empty. |
| `cancellation_reason` | String | Output only. The reason the entitlement was cancelled. If this entitlement wasn't cancelled, this field is empty. Possible values include "unknown", "expired", "user-cancelled", "account-closed", "billing-disabled" (if the customer has manually disabled billing to their resources), "user-aborted", and "migrated" (if the entitlement has migrated across products). Values of this field are subject to change, and we recommend that you don't build your technical integration to rely on these fields. |
| `product_external_name` | String | Output only. The identifier of the product that was procured. |
| `update_time` | String | Output only. The last update timestamp. |
| `new_pending_offer_duration` | String | Output only. The duration of the new offer, in ISO 8601 duration format. This field is populated for pending offer changes. It isn't populated for entitlements which aren't active yet. If the offer has a specified end date instead of a duration, this field is empty. * If the entitlement is in the state ENTITLEMENT_ACTIVATION_REQUESTED, ENTITLEMENT_ACTIVE, or ENTITLEMENT_PENDING_CANCELLATION, this field is empty. * If the entitlement is in the state ENTITLEMENT_PENDING_PLAN_CHANGE_APPROVAL or ENTITLEMENT_PENDING_PLAN_CHANGE, and the upcoming offer doesn't have a specified end date, then this field is populated with the duration of the upcoming offer. Otherwise, this field is empty. * If the entitlement is in the state ENTITLEMENT_CANCELLED, then this field is empty. |
| `usage_reporting_id` | String | Output only. The consumerId to use when reporting usage through the Service Control API. See the consumerId field at [Reporting Metrics](https://cloud.google.com/service-control/reporting-metrics) for more details. This field is present only if the product has usage-based billing configured. |
| `offer` | String | Output only. The name of the offer that was procured. Field is empty if order wasn't made using an offer. Format: 'projects/{project}/services/{service}/privateOffers/{offer}' OR 'projects/{project}/services/{service}/standardOffers/{offer}', depending on whether the offer is private or public. The {service} in the name is the listing service of the offer. It could be either the product service that the offer is referencing, or a generic private offer parent service. We recommend that you don't build your integration to rely on the meaning of this {service} part. * If the entitlement is in the state ENTITLEMENT_ACTIVATION_REQUESTED, this field is populated with the upcoming offer. * If the entitlement is in the state ENTITLEMENT_ACTIVE, ENTITLEMENT_PENDING_CANCELLATION, ENTITLEMENT_PENDING_PLAN_CHANGE, or ENTITLEMENT_PENDING_PLAN_CHANGE_APPROVAL, this field is populated with the current offer. * If the entitlement is in the state ENTITLEMENT_CANCELLED, then this field is populated with the latest offer that the order was associated with. |
| `account` | String | Output only. The resource name of the account that this entitlement is based on, if any. |
| `product` | String | Output only. The identifier of the entity that was purchased. This may actually represent a product, quote, or offer. We strongly recommend that you use the following more explicit fields: productExternalName, quoteExternalName, or offer. |
| `quote_external_name` | String | Output only. The identifier of the quote that was used to procure. Empty if the order is not purchased using a quote. |
| `plan` | String | Output only. The identifier of the plan that was procured. Required if the product has plans. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create entitlement
entitlement = provider.cloudcommerceprocurement_api.Entitlement {
    name = "value"  # Required. The name of the entitlement to suspend.
}

# Access entitlement outputs
entitlement_id = entitlement.id
entitlement_entitlement_benefit_ids = entitlement.entitlement_benefit_ids
entitlement_offer_duration = entitlement.offer_duration
entitlement_order_id = entitlement.order_id
entitlement_state = entitlement.state
entitlement_new_pending_offer = entitlement.new_pending_offer
entitlement_create_time = entitlement.create_time
entitlement_name = entitlement.name
entitlement_new_offer_end_time = entitlement.new_offer_end_time
entitlement_input_properties = entitlement.input_properties
entitlement_offer_end_time = entitlement.offer_end_time
entitlement_provider = entitlement.provider
entitlement_subscription_end_time = entitlement.subscription_end_time
entitlement_message_to_user = entitlement.message_to_user
entitlement_new_pending_plan = entitlement.new_pending_plan
entitlement_consumers = entitlement.consumers
entitlement_new_offer_start_time = entitlement.new_offer_start_time
entitlement_cancellation_reason = entitlement.cancellation_reason
entitlement_product_external_name = entitlement.product_external_name
entitlement_update_time = entitlement.update_time
entitlement_new_pending_offer_duration = entitlement.new_pending_offer_duration
entitlement_usage_reporting_id = entitlement.usage_reporting_id
entitlement_offer = entitlement.offer
entitlement_account = entitlement.account
entitlement_product = entitlement.product
entitlement_quote_external_name = entitlement.quote_external_name
entitlement_plan = entitlement.plan
```

---


### Account

Rejects an approval on an Account.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `approval_name` | String |  | The name of the approval being rejected. If absent and there is only one approval possible, that approval will be rejected. If absent and there are many approvals possible, the request will fail with a 400 Bad Request. Optional. |
| `reason` | String |  | Free form text string explaining the rejection reason. Max allowed length: 256 bytes. Longer strings will be truncated. |
| `name` | String | ✅ | Required. The resource name of the account. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `input_properties` | HashMap<String, String> | Output only. The custom properties that were collected from the user to create this account. |
| `approvals` | Vec<String> | Output only. The approvals for this account. These approvals are used to track actions that are permitted or have been completed by a customer within the context of the provider. This might include a sign up flow or a provisioning step, for example, that the provider can admit to having happened. |
| `create_time` | String | Output only. The creation timestamp. |
| `state` | String | Output only. The state of the account. This is used to decide whether the customer is in good standing with the provider and is able to make purchases. An account might not be able to make a purchase if the billing account is suspended, for example. |
| `name` | String | Output only. The resource name of the account. Account names have the form `accounts/{account_id}`. |
| `provider` | String | Output only. The identifier of the service provider that this account was created against. Each service provider is assigned a unique provider value when they onboard with Cloud Commerce platform. |
| `reseller_parent_billing_account` | String | Output only. The reseller parent billing account of the account's corresponding billing account, applicable only when the corresponding billing account is a subaccount of a reseller. Included in responses only for view: ACCOUNT_VIEW_FULL. Format: billingAccounts/{billing_account_id} |
| `update_time` | String | Output only. The last update timestamp. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create account
account = provider.cloudcommerceprocurement_api.Account {
    name = "value"  # Required. The resource name of the account.
}

# Access account outputs
account_id = account.id
account_input_properties = account.input_properties
account_approvals = account.approvals
account_create_time = account.create_time
account_state = account.state
account_name = account.name
account_provider = account.provider
account_reseller_parent_billing_account = account.reseller_parent_billing_account
account_update_time = account.update_time
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
entitlement_0 = provider.cloudcommerceprocurement_api.Entitlement {
    name = "value-0"
}
entitlement_1 = provider.cloudcommerceprocurement_api.Entitlement {
    name = "value-1"
}
entitlement_2 = provider.cloudcommerceprocurement_api.Entitlement {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    entitlement = provider.cloudcommerceprocurement_api.Entitlement {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Cloudcommerceprocurement_api Documentation](https://cloud.google.com/cloudcommerceprocurement_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
