# Consumersurveys_api Service



**Resources**: 3

---

## Overview

The consumersurveys_api service provides access to 3 resource types:

- [Mobileapppanel](#mobileapppanel) [RU]
- [Survey](#survey) [CRUD]
- [Result](#result) [R]

---

## Resources


### Mobileapppanel

Retrieves a MobileAppPanel that is available to the authenticated user.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `is_public_panel` | bool |  |  |
| `country` | String |  |  |
| `owners` | Vec<String> |  |  |
| `name` | String |  |  |
| `mobile_app_panel_id` | String |  |  |
| `language` | String |  |  |
| `panel_id` | String | ✅ | External URL ID for the panel. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `is_public_panel` | bool |  |
| `country` | String |  |
| `owners` | Vec<String> |  |
| `name` | String |  |
| `mobile_app_panel_id` | String |  |
| `language` | String |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access mobileapppanel outputs
mobileapppanel_id = mobileapppanel.id
mobileapppanel_is_public_panel = mobileapppanel.is_public_panel
mobileapppanel_country = mobileapppanel.country
mobileapppanel_owners = mobileapppanel.owners
mobileapppanel_name = mobileapppanel.name
mobileapppanel_mobile_app_panel_id = mobileapppanel.mobile_app_panel_id
mobileapppanel_language = mobileapppanel.language
```

---


### Survey

Creates a survey.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `title` | String |  |  |
| `state` | String |  |  |
| `audience` | String |  |  |
| `cost` | String |  |  |
| `owners` | Vec<String> |  |  |
| `customer_data` | String |  |  |
| `questions` | Vec<String> |  |  |
| `survey_url_id` | String |  |  |
| `wanted_response_count` | i64 |  |  |
| `rejection_reason` | String |  |  |
| `description` | String |  |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `title` | String |  |
| `state` | String |  |
| `audience` | String |  |
| `cost` | String |  |
| `owners` | Vec<String> |  |
| `customer_data` | String |  |
| `questions` | Vec<String> |  |
| `survey_url_id` | String |  |
| `wanted_response_count` | i64 |  |
| `rejection_reason` | String |  |
| `description` | String |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create survey
survey = provider.consumersurveys_api.Survey {
}

# Access survey outputs
survey_id = survey.id
survey_title = survey.title
survey_state = survey.state
survey_audience = survey.audience
survey_cost = survey.cost
survey_owners = survey.owners
survey_customer_data = survey.customer_data
survey_questions = survey.questions
survey_survey_url_id = survey.survey_url_id
survey_wanted_response_count = survey.wanted_response_count
survey_rejection_reason = survey.rejection_reason
survey_description = survey.description
```

---


### Result

Retrieves any survey results that have been produced so far. Results are formatted as an Excel file. You must add "?alt=media" to the URL as an argument to get results.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `status` | String |  |
| `survey_url_id` | String |  |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access result outputs
result_id = result.id
result_status = result.status
result_survey_url_id = result.survey_url_id
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple mobileapppanel resources
mobileapppanel_0 = provider.consumersurveys_api.Mobileapppanel {
    panel_id = "value-0"
}
mobileapppanel_1 = provider.consumersurveys_api.Mobileapppanel {
    panel_id = "value-1"
}
mobileapppanel_2 = provider.consumersurveys_api.Mobileapppanel {
    panel_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    mobileapppanel = provider.consumersurveys_api.Mobileapppanel {
        panel_id = "production-value"
    }
```

---

## Related Documentation

- [GCP Consumersurveys_api Documentation](https://cloud.google.com/consumersurveys_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
