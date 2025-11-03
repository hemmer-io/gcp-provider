# Partners_api Service



**Resources**: 10

---

## Overview

The partners_api service provides access to 10 resource types:

- [Offer](#offer) [R]
- [History](#history) [R]
- [User_event](#user_event) [C]
- [User](#user) [RUD]
- [User_state](#user_state) [R]
- [Analytic](#analytic) [R]
- [Lead](#lead) [CR]
- [Client_message](#client_message) [C]
- [Partner](#partner) [RU]
- [Companie](#companie) [R]

---

## Resources


### Offer

Lists the Offers available for the current user

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `no_offer_reason` | String | Reason why no Offers are available. |
| `available_offers` | Vec<String> | Available Offers to be distributed. |
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
offer_no_offer_reason = offer.no_offer_reason
offer_available_offers = offer.available_offers
offer_response_metadata = offer.response_metadata
```

---


### History

Lists the Historical Offers for the current user (or user's entire company)

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `response_metadata` | String | Current response metadata. |
| `can_show_entire_company` | bool | True if the user has the option to show entire company history. |
| `showing_entire_company` | bool | True if this response is showing entire company history. |
| `offers` | Vec<String> | Historical offers meeting request. |
| `total_results` | i64 | Number of results across all pages. |
| `next_page_token` | String | Supply this token in a ListOffersHistoryRequest to retrieve the next page. |


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
history_response_metadata = history.response_metadata
history_can_show_entire_company = history.can_show_entire_company
history_showing_entire_company = history.showing_entire_company
history_offers = history.offers
history_total_results = history.total_results
history_next_page_token = history.next_page_token
```

---


### User_event

Logs a user event.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `event_category` | String |  | The category the action belongs to. |
| `event_action` | String |  | The action that occurred. |
| `event_scope` | String |  | The scope of the event. |
| `url` | String |  | The URL where the event occurred. |
| `lead` | String |  | Advertiser lead information. |
| `request_metadata` | String |  | Current request metadata. |
| `event_datas` | Vec<String> |  | List of event data for the event. |



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


### User

Gets a user.

**Operations**: ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `address` | String |  | The primary address for this company. |
| `resolved_timestamp` | String |  | The timestamp when the user was approved.
@OutputOnly |
| `company_id` | String |  | The ID of the company. There may be no id if this is a
pending company.5 |
| `logo_url` | String |  | A URL to a profile photo, e.g. a G+ profile photo. |
| `state` | String |  | The state of relationship, in terms of approvals. |
| `badge_tier` | String |  | Whether the company is a Partner. |
| `primary_language_code` | String |  | The primary language code of the company. |
| `website` | String |  | The website URL for this company. |
| `is_pending` | bool |  | The flag that indicates if the company is pending verification. |
| `specialization_status` | Vec<String> |  | The list of Google Partners specialization statuses for the company. |
| `creation_time` | String |  | The timestamp of when affiliation was requested.
@OutputOnly |
| `company_admin` | bool |  | Indicates if the user is an admin for this company. |
| `phone_number` | String |  | The phone number for the company's primary address. |
| `primary_country_code` | String |  | The primary country code of the company. |
| `primary_address` | String |  | The primary location of the company. |
| `internal_company_id` | String |  | The internal company ID.
Only available for a whitelisted set of api clients. |
| `manager_account` | String |  | The AdWords manager account # associated this company. |
| `segment` | Vec<String> |  | The segment the company is classified as. |
| `name` | String |  | The name (in the company's primary language) for the company. |
| `user_id` | String | ✅ | The ID of the user. Can be set to <code>me</code> to mean
the currently authenticated user. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `afa_info_shared` | bool | Whether or not the user has opted to share their Academy for Ads info with
Google Partners. |
| `available_adwords_manager_accounts` | Vec<String> | This is the list of AdWords Manager Accounts the user has edit access to.
If the user has edit access to multiple accounts, the user can choose the
preferred account and we use this when a personal account is needed. Can
be empty meaning the user has access to no accounts.
@OutputOnly |
| `internal_id` | String | The internal user ID.
Only available for a whitelisted set of api clients. |
| `primary_emails` | Vec<String> | The list of emails the user has access to/can select as primary.
@OutputOnly |
| `company_verification_email` | String | The email address used by the user used for company verification.
@OutputOnly |
| `profile` | String | The profile information of a Partners user, contains all the directly
editable user information. |
| `id` | String | The ID of the user. |
| `certification_status` | Vec<String> | The list of achieved certifications. These are calculated based on exam
results and other requirements.
@OutputOnly |
| `public_profile` | String | Information about a user's external public profile outside Google Partners. |
| `last_access_time` | String | The most recent time the user interacted with the Partners site.
@OutputOnly |
| `exam_status` | Vec<String> | The list of exams the user ever taken. For each type of exam, only one
entry is listed. |
| `company` | String | The company that the user is associated with.
If not present, the user is not associated with any company. |


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
user_afa_info_shared = user.afa_info_shared
user_available_adwords_manager_accounts = user.available_adwords_manager_accounts
user_internal_id = user.internal_id
user_primary_emails = user.primary_emails
user_company_verification_email = user.company_verification_email
user_profile = user.profile
user_id = user.id
user_certification_status = user.certification_status
user_public_profile = user.public_profile
user_last_access_time = user.last_access_time
user_exam_status = user.exam_status
user_company = user.company
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
| `response_metadata` | String | Current response metadata. |
| `user_states` | Vec<String> | User's states. |


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
user_state_response_metadata = user_state.response_metadata
user_state_user_states = user_state.user_states
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


### Lead

Creates an advertiser lead for the given company ID.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `recaptcha_challenge` | String |  | <a href="https://www.google.com/recaptcha/">reCaptcha</a> challenge info. |
| `request_metadata` | String |  | Current request metadata. |
| `lead` | String |  | The lead resource. The `LeadType` must not be `LEAD_TYPE_UNSPECIFIED`
and either `email` or `phone_number` must be provided. |
| `company_id` | String | ✅ | The ID of the company to contact. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `leads` | Vec<String> | The list of leads. |
| `next_page_token` | String | A token to retrieve next page of results.
Pass this value in the `ListLeadsRequest.page_token` field in the
subsequent call to
ListLeads to retrieve the
next page of results. |
| `total_size` | i64 | The total count of leads for the given company. |
| `response_metadata` | String | Current response metadata. |


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
lead_leads = lead.leads
lead_next_page_token = lead.next_page_token
lead_total_size = lead.total_size
lead_response_metadata = lead.response_metadata
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
| `details` | String |  | Details about the client message. |
| `request_metadata` | String |  | Current request metadata. |
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


### Partner

Gets Partners Status of the logged in user's agency.
Should only be called if the logged in user is the admin of the agency.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String |  | The ID of the company. |
| `profile_status` | String |  | The public viewability status of the company's profile. |
| `specialization_status` | Vec<String> |  | The list of Google Partners specialization statuses for the company. |
| `website_url` | String |  | URL of the company's website. |
| `original_min_monthly_budget` | String |  | The unconverted minimum monthly budget that the company accepts for partner
business. |
| `certification_statuses` | Vec<String> |  | The list of Google Partners certification statuses for the company. |
| `ranks` | Vec<String> |  | Information related to the ranking of the company within the list of
companies. |
| `primary_adwords_manager_account_id` | String |  | The Primary AdWords Manager Account id. |
| `converted_min_monthly_budget` | String |  | The minimum monthly budget that the company accepts for partner business,
converted to the requested currency code. |
| `badge_tier` | String |  | Partner badge tier |
| `industries` | Vec<String> |  | Industries the company can help with. |
| `company_types` | Vec<String> |  | Company type labels listed on the company's profile. |
| `additional_websites` | Vec<String> |  | URL of the company's additional websites used to verify the dynamic badges.
These are stored as full URLs as entered by the user, but only the TLD will
be used for the actual verification. |
| `primary_location` | String |  | The primary location of the company. |
| `primary_language_code` | String |  | The primary language code of the company, as defined by
<a href="https://tools.ietf.org/html/bcp47">BCP 47</a>
(IETF BCP 47, "Tags for Identifying Languages"). |
| `badge_authority_in_awn` | bool |  | Whether the company's badge authority is in AWN |
| `localized_infos` | Vec<String> |  | The list of localized info for the company. |
| `auto_approval_email_domains` | Vec<String> |  | Email domains that allow users with a matching email address to get
auto-approved for associating with this company. |
| `services` | Vec<String> |  | Services the company can help with. |
| `public_profile` | String |  | Basic information from the company's public profile. |
| `locations` | Vec<String> |  | The list of all company locations.
If set, must include the
primary_location
in the list. |
| `name` | String |  | The name of the company. |


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


### Companie

Gets a company.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `response_metadata` | String | Current response metadata. |
| `company` | String | The company. |


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
companie_response_metadata = companie.response_metadata
companie_company = companie.company
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple offer resources
offer_0 = provider.partners_api.Offer {
}
offer_1 = provider.partners_api.Offer {
}
offer_2 = provider.partners_api.Offer {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    offer = provider.partners_api.Offer {
    }
```

---

## Related Documentation

- [GCP Partners_api Documentation](https://cloud.google.com/partners_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
