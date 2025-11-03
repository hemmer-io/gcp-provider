# Billingbudgets_api Service



**Resources**: 2

---

## Overview

The billingbudgets_api service provides access to 2 resource types:

- [Budget](#budget) [CRUD]
- [Budget](#budget) [CRUD]

---

## Resources


### Budget

Creates a new budget. See [Quotas and limits](https://cloud.google.com/billing/quotas) for more information on the limits of the number of budgets you can create.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Resource name of the budget. The resource name implies the scope of a budget. Values are of the form `billingAccounts/{billingAccountId}/budgets/{budgetId}`. |
| `threshold_rules` | Vec<String> |  | Optional. Rules that trigger alerts (notifications of thresholds being crossed) when spend exceeds the specified percentages of the budget. Optional for `pubsubTopic` notifications. Required if using email notifications. |
| `display_name` | String |  | User data for display name in UI. The name must be less than or equal to 60 characters. |
| `notifications_rule` | String |  | Optional. Rules to apply to notifications sent based on budget spend and thresholds. |
| `ownership_scope` | String |  |  |
| `budget_filter` | String |  | Optional. Filters that define which resources are used to compute the actual spend against the budget amount, such as projects, services, and the budget's time period, as well as other filters. |
| `etag` | String |  | Optional. Etag to validate that the object is unchanged for a read-modify-write operation. An empty etag causes an update to overwrite other changes. |
| `amount` | String |  | Required. Budgeted amount. |
| `parent` | String | ✅ | Required. The name of the billing account to create the budget in. Values are of the form `billingAccounts/{billingAccountId}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Output only. Resource name of the budget. The resource name implies the scope of a budget. Values are of the form `billingAccounts/{billingAccountId}/budgets/{budgetId}`. |
| `threshold_rules` | Vec<String> | Optional. Rules that trigger alerts (notifications of thresholds being crossed) when spend exceeds the specified percentages of the budget. Optional for `pubsubTopic` notifications. Required if using email notifications. |
| `display_name` | String | User data for display name in UI. The name must be less than or equal to 60 characters. |
| `notifications_rule` | String | Optional. Rules to apply to notifications sent based on budget spend and thresholds. |
| `ownership_scope` | String |  |
| `budget_filter` | String | Optional. Filters that define which resources are used to compute the actual spend against the budget amount, such as projects, services, and the budget's time period, as well as other filters. |
| `etag` | String | Optional. Etag to validate that the object is unchanged for a read-modify-write operation. An empty etag causes an update to overwrite other changes. |
| `amount` | String | Required. Budgeted amount. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create budget
budget = provider.billingbudgets_api.Budget {
    parent = "value"  # Required. The name of the billing account to create the budget in. Values are of the form `billingAccounts/{billingAccountId}`.
}

# Access budget outputs
budget_id = budget.id
budget_name = budget.name
budget_threshold_rules = budget.threshold_rules
budget_display_name = budget.display_name
budget_notifications_rule = budget.notifications_rule
budget_ownership_scope = budget.ownership_scope
budget_budget_filter = budget.budget_filter
budget_etag = budget.etag
budget_amount = budget.amount
```

---


### Budget

Creates a new budget. See [Quotas and limits](https://cloud.google.com/billing/quotas) for more information on the limits of the number of budgets you can create.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `budget` | String |  | Required. Budget to create. |
| `parent` | String | ✅ | Required. The name of the billing account to create the budget in. Values are of the form `billingAccounts/{billingAccountId}`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ownership_scope` | String |  |
| `etag` | String | Optional. Etag to validate that the object is unchanged for a read-modify-write operation. An empty etag will cause an update to overwrite other changes. |
| `threshold_rules` | Vec<String> | Optional. Rules that trigger alerts (notifications of thresholds being crossed) when spend exceeds the specified percentages of the budget. Optional for `pubsubTopic` notifications. Required if using email notifications. |
| `all_updates_rule` | String | Optional. Rules to apply to notifications sent based on budget spend and thresholds. |
| `name` | String | Output only. Resource name of the budget. The resource name implies the scope of a budget. Values are of the form `billingAccounts/{billingAccountId}/budgets/{budgetId}`. |
| `budget_filter` | String | Optional. Filters that define which resources are used to compute the actual spend against the budget amount, such as projects, services, and the budget's time period, as well as other filters. |
| `amount` | String | Required. Budgeted amount. |
| `display_name` | String | User data for display name in UI. Validation: <= 60 chars. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create budget
budget = provider.billingbudgets_api.Budget {
    parent = "value"  # Required. The name of the billing account to create the budget in. Values are of the form `billingAccounts/{billingAccountId}`.
}

# Access budget outputs
budget_id = budget.id
budget_ownership_scope = budget.ownership_scope
budget_etag = budget.etag
budget_threshold_rules = budget.threshold_rules
budget_all_updates_rule = budget.all_updates_rule
budget_name = budget.name
budget_budget_filter = budget.budget_filter
budget_amount = budget.amount
budget_display_name = budget.display_name
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple budget resources
budget_0 = provider.billingbudgets_api.Budget {
    parent = "value-0"
}
budget_1 = provider.billingbudgets_api.Budget {
    parent = "value-1"
}
budget_2 = provider.billingbudgets_api.Budget {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    budget = provider.billingbudgets_api.Budget {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Billingbudgets_api Documentation](https://cloud.google.com/billingbudgets_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
