# Consumersurveys_api Service



**Resources**: 3

---

## Overview

The consumersurveys_api service provides access to 3 resource types:

- [Result](#result) [R]
- [Survey](#survey) [CRUD]
- [Mobileapppanel](#mobileapppanel) [RU]

---

## Resources


### Result

Retrieves any survey results that have been produced so far. Results are formatted as an Excel file. You must add "?alt=media" to the URL as an argument to get results.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `survey_url_id` | String |  |
| `status` | String |  |


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
result_survey_url_id = result.survey_url_id
result_status = result.status
```

---


### Survey

Creates a survey.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  |  |
| `survey_url_id` | String |  |  |
| `questions` | Vec<String> |  |  |
| `title` | String |  |  |
| `audience` | String |  |  |
| `rejection_reason` | String |  |  |
| `owners` | Vec<String> |  |  |
| `description` | String |  |  |
| `cost` | String |  |  |
| `customer_data` | String |  |  |
| `wanted_response_count` | i64 |  |  |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state` | String |  |
| `survey_url_id` | String |  |
| `questions` | Vec<String> |  |
| `title` | String |  |
| `audience` | String |  |
| `rejection_reason` | String |  |
| `owners` | Vec<String> |  |
| `description` | String |  |
| `cost` | String |  |
| `customer_data` | String |  |
| `wanted_response_count` | i64 |  |


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
survey_state = survey.state
survey_survey_url_id = survey.survey_url_id
survey_questions = survey.questions
survey_title = survey.title
survey_audience = survey.audience
survey_rejection_reason = survey.rejection_reason
survey_owners = survey.owners
survey_description = survey.description
survey_cost = survey.cost
survey_customer_data = survey.customer_data
survey_wanted_response_count = survey.wanted_response_count
```

---


### Mobileapppanel

Retrieves a MobileAppPanel that is available to the authenticated user.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  |  |
| `is_public_panel` | bool |  |  |
| `language` | String |  |  |
| `mobile_app_panel_id` | String |  |  |
| `owners` | Vec<String> |  |  |
| `country` | String |  |  |
| `panel_id` | String | ✅ | External URL ID for the panel. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String |  |
| `is_public_panel` | bool |  |
| `language` | String |  |
| `mobile_app_panel_id` | String |  |
| `owners` | Vec<String> |  |
| `country` | String |  |


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
mobileapppanel_name = mobileapppanel.name
mobileapppanel_is_public_panel = mobileapppanel.is_public_panel
mobileapppanel_language = mobileapppanel.language
mobileapppanel_mobile_app_panel_id = mobileapppanel.mobile_app_panel_id
mobileapppanel_owners = mobileapppanel.owners
mobileapppanel_country = mobileapppanel.country
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple result resources
result_0 = provider.consumersurveys_api.Result {
}
result_1 = provider.consumersurveys_api.Result {
}
result_2 = provider.consumersurveys_api.Result {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    result = provider.consumersurveys_api.Result {
    }
```

---

## Related Documentation

- [GCP Consumersurveys_api Documentation](https://cloud.google.com/consumersurveys_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
