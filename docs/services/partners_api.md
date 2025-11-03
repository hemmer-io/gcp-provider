# Partners_api Service



**Resources**: 10

---

## Overview

The partners_api service provides access to 10 resource types:

- [History](#history) [R]
- [User](#user) [RUD]
- [Companie](#companie) [R]
- [Offer](#offer) [R]
- [Partner](#partner) [RU]
- [Lead](#lead) [CR]
- [User_event](#user_event) [C]
- [User_state](#user_state) [R]
- [Client_message](#client_message) [C]
- [Analytic](#analytic) [R]

---

## Resources


### History

Lists the Historical Offers for the current user (or user's entire company)

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `showing_entire_company` | bool | True if this response is showing entire company history. |
| `can_show_entire_company` | bool | True if the user has the option to show entire company history. |
| `next_page_token` | String | Supply this token in a ListOffersHistoryRequest to retrieve the next page. |
| `offers` | Vec<String> | Historical offers meeting request. |
| `total_results` | i64 | Number of results across all pages. |
| `response_metadata` | String | Current response metadata. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access history outputs
history_id = history.id
history_showing_entire_company = history.showing_entire_company
history_can_show_entire_company = history.can_show_entire_company
history_next_page_token = history.next_page_token
history_offers = history.offers
history_total_results = history.total_results
history_response_metadata = history.response_metadata
```

---


### User

Gets a user.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `markets` | Vec<String> |  | A list of ids representing which markets the user was interested in. |
| `job_functions` | Vec<String> |  | A list of ids represnting which job categories the user selected. |
| `migrate_to_afa` | bool |  | Whether or not to migrate the user's exam data to Academy for Ads. |
| `email_opt_ins` | String |  | The list of opt-ins for the user, related to communication preferences. |
| `languages` | Vec<String> |  | The list of languages this user understands. |
| `given_name` | String |  | The user's given name. |
| `adwords_manager_account` | String |  | If the user has edit access to multiple accounts, the user can choose the
preferred account and it is used when a personal account is needed. Can
be empty. |
| `phone_number` | String |  | The user's phone number. |
| `email_address` | String |  | The email address the user has selected on the Partners site as primary. |
| `family_name` | String |  | The user's family name. |
| `profile_public` | bool |  | Whether the user's public profile is visible to anyone with the URL. |
| `primary_country_code` | String |  | The user's primary country, an ISO 2-character code. |
| `address` | String |  | The user's mailing address, contains multiple fields. |
| `industries` | Vec<String> |  | A list of ids representing which industries the user selected. |
| `channels` | Vec<String> |  | A list of ids representing which channels the user selected they were in. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `profile` | String | The profile information of a Partners user, contains all the directly
editable user information. |
| `available_adwords_manager_accounts` | Vec<String> | This is the list of AdWords Manager Accounts the user has edit access to.
If the user has edit access to multiple accounts, the user can choose the
preferred account and we use this when a personal account is needed. Can
be empty meaning the user has access to no accounts.
@OutputOnly |
| `company` | String | The company that the user is associated with.
If not present, the user is not associated with any company. |
| `company_verification_email` | String | The email address used by the user used for company verification.
@OutputOnly |
| `primary_emails` | Vec<String> | The list of emails the user has access to/can select as primary.
@OutputOnly |
| `last_access_time` | String | The most recent time the user interacted with the Partners site.
@OutputOnly |
| `afa_info_shared` | bool | Whether or not the user has opted to share their Academy for Ads info with
Google Partners. |
| `public_profile` | String | Information about a user's external public profile outside Google Partners. |
| `id` | String | The ID of the user. |
| `exam_status` | Vec<String> | The list of exams the user ever taken. For each type of exam, only one
entry is listed. |
| `internal_id` | String | The internal user ID.
Only available for a whitelisted set of api clients. |
| `certification_status` | Vec<String> | The list of achieved certifications. These are calculated based on exam
results and other requirements.
@OutputOnly |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access user outputs
user_id = user.id
user_profile = user.profile
user_available_adwords_manager_accounts = user.available_adwords_manager_accounts
user_company = user.company
user_company_verification_email = user.company_verification_email
user_primary_emails = user.primary_emails
user_last_access_time = user.last_access_time
user_afa_info_shared = user.afa_info_shared
user_public_profile = user.public_profile
user_id = user.id
user_exam_status = user.exam_status
user_internal_id = user.internal_id
user_certification_status = user.certification_status
```

---


### Companie

Gets a company.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `company` | String | The company. |
| `response_metadata` | String | Current response metadata. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access companie outputs
companie_id = companie.id
companie_company = companie.company
companie_response_metadata = companie.response_metadata
```

---


### Offer

Lists the Offers available for the current user

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `available_offers` | Vec<String> | Available Offers to be distributed. |
| `no_offer_reason` | String | Reason why no Offers are available. |
| `response_metadata` | String | Current response metadata. |


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
offer_available_offers = offer.available_offers
offer_no_offer_reason = offer.no_offer_reason
offer_response_metadata = offer.response_metadata
```

---


### Partner

Gets Partners Status of the logged in user's agency.
Should only be called if the logged in user is the admin of the agency.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `phone_number` | String |  | Phone number of lead source. |
| `min_monthly_budget` | String |  | The minimum monthly budget lead source is willing to spend. |
| `comments` | String |  | Comments lead source gave. |
| `type` | String |  | Type of lead. |
| `family_name` | String |  | Last name of lead source. |
| `state` | String |  | The lead's state in relation to the company. |
| `id` | String |  | ID of the lead. |
| `email` | String |  | Email address of lead source. |
| `website_url` | String |  | Website URL of lead source. |
| `adwords_customer_id` | String |  | The AdWords Customer ID of the lead. |
| `marketing_opt_in` | bool |  | Whether or not the lead signed up for marketing emails |
| `create_time` | String |  | Timestamp of when this lead was created. |
| `gps_motivations` | Vec<String> |  | List of reasons for using Google Partner Search and creating a lead. |
| `given_name` | String |  | First name of lead source. |
| `language_code` | String |  | Language code of the lead's language preference, as defined by
<a href="https://tools.ietf.org/html/bcp47">BCP 47</a>
(IETF BCP 47, "Tags for Identifying Languages"). |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `response_metadata` | String | Current response metadata. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access partner outputs
partner_id = partner.id
partner_response_metadata = partner.response_metadata
```

---


### Lead

Creates an advertiser lead for the given company ID.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `request_metadata` | String |  | Current request metadata. |
| `recaptcha_challenge` | String |  | <a href="https://www.google.com/recaptcha/">reCaptcha</a> challenge info. |
| `lead` | String |  | The lead resource. The `LeadType` must not be `LEAD_TYPE_UNSPECIFIED`
and either `email` or `phone_number` must be provided. |
| `company_id` | String | ✅ | The ID of the company to contact. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token to retrieve next page of results.
Pass this value in the `ListLeadsRequest.page_token` field in the
subsequent call to
ListLeads to retrieve the
next page of results. |
| `total_size` | i64 | The total count of leads for the given company. |
| `response_metadata` | String | Current response metadata. |
| `leads` | Vec<String> | The list of leads. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create lead
lead = provider.partners_api.Lead {
    company_id = "value"  # The ID of the company to contact.
}

# Access lead outputs
lead_id = lead.id
lead_next_page_token = lead.next_page_token
lead_total_size = lead.total_size
lead_response_metadata = lead.response_metadata
lead_leads = lead.leads
```

---


### User_event

Logs a user event.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `event_action` | String |  | The action that occurred. |
| `lead` | String |  | Advertiser lead information. |
| `request_metadata` | String |  | Current request metadata. |
| `url` | String |  | The URL where the event occurred. |
| `event_datas` | Vec<String> |  | List of event data for the event. |
| `event_category` | String |  | The category the action belongs to. |
| `event_scope` | String |  | The scope of the event. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user_event
user_event = provider.partners_api.User_event {
}

```

---


### User_state

Lists states for current user.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `user_states` | Vec<String> | User's states. |
| `response_metadata` | String | Current response metadata. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access user_state outputs
user_state_id = user_state.id
user_state_user_states = user_state.user_states
user_state_response_metadata = user_state.response_metadata
```

---


### Client_message

Logs a generic message from the client, such as
`Failed to render component`, `Profile page is running slow`,
`More than 500 users have accessed this result.`, etc.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `request_metadata` | String |  | Current request metadata. |
| `details` | String |  | Details about the client message. |
| `client_info` | HashMap<String, String> |  | Map of client info, such as URL, browser navigator, browser platform, etc. |
| `level` | String |  | Message level of client message. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create client_message
client_message = provider.partners_api.Client_message {
}

```

---


### Analytic

Lists analytics data for a user's associated company.
Should only be called within the context of an authorized logged in user.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `response_metadata` | String | Current response metadata. |
| `next_page_token` | String | A token to retrieve next page of results.
Pass this value in the `ListAnalyticsRequest.page_token` field in the
subsequent call to
ListAnalytics to retrieve the
next page of results. |
| `analytics_summary` | String | Aggregated information across the response's
analytics. |
| `analytics` | Vec<String> | The list of analytics.
Sorted in ascending order of
Analytics.event_date. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access analytic outputs
analytic_id = analytic.id
analytic_response_metadata = analytic.response_metadata
analytic_next_page_token = analytic.next_page_token
analytic_analytics_summary = analytic.analytics_summary
analytic_analytics = analytic.analytics
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple history resources
history_0 = provider.partners_api.History {
}
history_1 = provider.partners_api.History {
}
history_2 = provider.partners_api.History {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    history = provider.partners_api.History {
    }
```

---

## Related Documentation

- [GCP Partners_api Documentation](https://cloud.google.com/partners_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
