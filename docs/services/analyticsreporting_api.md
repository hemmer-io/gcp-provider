# Analyticsreporting_api Service



**Resources**: 2

---

## Overview

The analyticsreporting_api service provides access to 2 resource types:

- [User_activity](#user_activity) [C]
- [Report](#report) [C]

---

## Resources


### User_activity

Returns User Activity data.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `date_range` | String |  | Date range for which to retrieve the user activity. If a date range is not provided, the default date range is (startDate: current date - 7 days, endDate: current date - 1 day). |
| `activity_types` | Vec<String> |  | Set of all activity types being requested. Only acvities matching these types will be returned in the response. If empty, all activies will be returned. |
| `page_size` | i64 |  | Page size is for paging and specifies the maximum number of returned rows. Page size should be > 0. If the value is 0 or if the field isn't specified, the request returns the default of 1000 rows per page. |
| `user` | String |  | Required. Unique user Id to query for. Every [SearchUserActivityRequest](#SearchUserActivityRequest) must contain this field. |
| `view_id` | String |  | Required. The Analytics [view ID](https://support.google.com/analytics/answer/1009618) from which to retrieve data. Every [SearchUserActivityRequest](#SearchUserActivityRequest) must contain the `viewId`. |
| `page_token` | String |  | A continuation token to get the next page of the results. Adding this to the request will return the rows after the pageToken. The pageToken should be the value returned in the nextPageToken parameter in the response to the [SearchUserActivityRequest](#SearchUserActivityRequest) request. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user_activity
user_activity = provider.analyticsreporting_api.User_activity {
}

```

---


### Report

Returns the Analytics data.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `use_resource_quotas` | bool |  | Enables [resource based quotas](/analytics/devguides/reporting/core/v4/limits-quotas#analytics_reporting_api_v4), (defaults to `False`). If this field is set to `True` the per view (profile) quotas are governed by the computational cost of the request. Note that using cost based quotas will higher enable sampling rates. (10 Million for `SMALL`, 100M for `LARGE`. See the [limits and quotas documentation](/analytics/devguides/reporting/core/v4/limits-quotas#analytics_reporting_api_v4) for details. |
| `report_requests` | Vec<String> |  | Requests, each request will have a separate response. There can be a maximum of 5 requests. All requests should have the same `dateRanges`, `viewId`, `segments`, `samplingLevel`, and `cohortGroup`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create report
report = provider.analyticsreporting_api.Report {
}

```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple user_activity resources
user_activity_0 = provider.analyticsreporting_api.User_activity {
}
user_activity_1 = provider.analyticsreporting_api.User_activity {
}
user_activity_2 = provider.analyticsreporting_api.User_activity {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    user_activity = provider.analyticsreporting_api.User_activity {
    }
```

---

## Related Documentation

- [GCP Analyticsreporting_api Documentation](https://cloud.google.com/analyticsreporting_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
