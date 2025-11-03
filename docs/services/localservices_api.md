# Localservices_api Service



**Resources**: 2

---

## Overview

The localservices_api service provides access to 2 resource types:

- [Account_report](#account_report) [R]
- [Detailed_lead_report](#detailed_lead_report) [R]

---

## Resources


### Account_report

Get account reports containing aggregate account data of all linked GLS accounts. Caller needs to provide their manager customer id and the associated auth credential that allows them read permissions on their linked accounts.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Pagination token to retrieve the next page of results. When `next_page_token` is not filled in, there is no next page and the list returned is the last page in the result set. |
| `account_reports` | Vec<String> | List of account reports which maps 1:1 to a particular linked GLS account. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access account_report outputs
account_report_id = account_report.id
account_report_next_page_token = account_report.next_page_token
account_report_account_reports = account_report.account_reports
```

---


### Detailed_lead_report

Get detailed lead reports containing leads that have been received by all linked GLS accounts. Caller needs to provide their manager customer id and the associated auth credential that allows them read permissions on their linked accounts.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `detailed_lead_reports` | Vec<String> | List of detailed lead reports uniquely identified by external lead id. |
| `next_page_token` | String | Pagination token to retrieve the next page of results. When `next_page_token` is not filled in, there is no next page and the list returned is the last page in the result set. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access detailed_lead_report outputs
detailed_lead_report_id = detailed_lead_report.id
detailed_lead_report_detailed_lead_reports = detailed_lead_report.detailed_lead_reports
detailed_lead_report_next_page_token = detailed_lead_report.next_page_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple account_report resources
account_report_0 = provider.localservices_api.Account_report {
}
account_report_1 = provider.localservices_api.Account_report {
}
account_report_2 = provider.localservices_api.Account_report {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    account_report = provider.localservices_api.Account_report {
    }
```

---

## Related Documentation

- [GCP Localservices_api Documentation](https://cloud.google.com/localservices_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
