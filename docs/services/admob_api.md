# Admob_api Service



**Resources**: 16

---

## Overview

The admob_api service provides access to 16 resource types:

- [App](#app) [R]
- [Account](#account) [R]
- [Network_report](#network_report) [C]
- [Mediation_report](#mediation_report) [C]
- [Ad_unit](#ad_unit) [R]
- [Adapter](#adapter) [R]
- [Network_report](#network_report) [C]
- [Ad_unit_mapping](#ad_unit_mapping) [CR]
- [App](#app) [CR]
- [Ad_source](#ad_source) [R]
- [Mediation_ab_experiment](#mediation_ab_experiment) [C]
- [Campaign_report](#campaign_report) [C]
- [Mediation_report](#mediation_report) [C]
- [Account](#account) [R]
- [Mediation_group](#mediation_group) [CRU]
- [Ad_unit](#ad_unit) [CR]

---

## Resources


### App

List the apps under the specified AdMob account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | If not empty, indicates that there may be more apps for the request; this value should be passed in a new `ListAppsRequest`. |
| `apps` | Vec<String> | The resulting apps for the requested account. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access app outputs
app_id = app.id
app_next_page_token = app.next_page_token
app_apps = app.apps
```

---


### Account

Gets information about the specified AdMob publisher account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `publisher_id` | String | The unique ID by which this publisher account can be identified in the API requests (for example, pub-1234567890). |
| `currency_code` | String | Currency code of the earning-related metrics, which is the 3-letter code defined in ISO 4217. The daily average rate is used for the currency conversion. |
| `reporting_time_zone` | String | The time zone that is used in reports that are generated for this account. The value is a time-zone ID as specified by the CLDR project, for example, "America/Los_Angeles". |
| `name` | String | Resource name of this account. Format is accounts/{publisher_id}. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access account outputs
account_id = account.id
account_publisher_id = account.publisher_id
account_currency_code = account.currency_code
account_reporting_time_zone = account.reporting_time_zone
account_name = account.name
```

---


### Network_report

Generates an AdMob Network report based on the provided report specification. Returns result of a server-side streaming RPC. The result is returned in a sequence of responses.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `report_spec` | String |  | Network report specification. |
| `parent` | String | ✅ | Resource name of the account to generate the report for. Example: accounts/pub-9876543210987654 |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create network_report
network_report = provider.admob_api.Network_report {
    parent = "value"  # Resource name of the account to generate the report for. Example: accounts/pub-9876543210987654
}

```

---


### Mediation_report

Generates an AdMob Mediation report based on the provided report specification. Returns result of a server-side streaming RPC. The result is returned in a sequence of responses.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `report_spec` | String |  | Network report specification. |
| `parent` | String | ✅ | Resource name of the account to generate the report for. Example: accounts/pub-9876543210987654 |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create mediation_report
mediation_report = provider.admob_api.Mediation_report {
    parent = "value"  # Resource name of the account to generate the report for. Example: accounts/pub-9876543210987654
}

```

---


### Ad_unit

List the ad units under the specified AdMob account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ad_units` | Vec<String> | The resulting ad units for the requested account. |
| `next_page_token` | String | If not empty, indicates that there may be more ad units for the request; this value should be passed in a new `ListAdUnitsRequest`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access ad_unit outputs
ad_unit_id = ad_unit.id
ad_unit_ad_units = ad_unit.ad_units
ad_unit_next_page_token = ad_unit.next_page_token
```

---


### Adapter

List the adapters of the ad source.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Used to set the `page_token` in the `ListAdapterRequest` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `adapters` | Vec<String> | The adapter. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access adapter outputs
adapter_id = adapter.id
adapter_next_page_token = adapter.next_page_token
adapter_adapters = adapter.adapters
```

---


### Network_report

Generates an AdMob Network report based on the provided report specification. Returns result of a server-side streaming RPC. The result is returned in a sequence of responses.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `report_spec` | String |  | Network report specification. |
| `parent` | String | ✅ | Resource name of the account to generate the report for. Example: accounts/pub-9876543210987654 |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create network_report
network_report = provider.admob_api.Network_report {
    parent = "value"  # Resource name of the account to generate the report for. Example: accounts/pub-9876543210987654
}

```

---


### Ad_unit_mapping

Create an ad unit mapping under the specific AdMob account and ad unit. This method has limited access. If you see a 403 permission denied error, please reach out to your account manager for access.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `adapter_id` | String |  | The ID of mediation ad source adapter used by this ad unit mapping. The adapter determines the information needed in the ad_network_settings. |
| `name` | String |  | Resource name of this ad unit mapping. Format is: accounts/{publisher_id}/adUnits/{ad_unit_id_fragment}/adUnitMappings/{ad_unit_mapping_id} Example: accounts/pub-1234567890123456/adUnits/0123456789/adUnitMappings/987654321 |
| `state` | String |  | Output only. The status of this ad unit mapping. |
| `ad_unit_configurations` | HashMap<String, String> |  | Settings for the specified ad unit to make an ad request to 3rd party ad network. Key-value pairs with values set by the user for the keys requested by the ad network. Please see https://support.google.com/admob/answer/3245073 for details on how to configure the network settings. |
| `display_name` | String |  | Optional. The display name of this ad unit mapping instance. |
| `parent` | String | ✅ | Required. The parent which owns the ad unit mapping. Format: accounts/{publisher_id}/adUnits/{ad_unit_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Used to set the `page_token` in the `ListAdUnitMappingsRequest` to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `ad_unit_mappings` | Vec<String> | The ad unit mappings from the specified account and ad unit. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create ad_unit_mapping
ad_unit_mapping = provider.admob_api.Ad_unit_mapping {
    parent = "value"  # Required. The parent which owns the ad unit mapping. Format: accounts/{publisher_id}/adUnits/{ad_unit_id}
}

# Access ad_unit_mapping outputs
ad_unit_mapping_id = ad_unit_mapping.id
ad_unit_mapping_next_page_token = ad_unit_mapping.next_page_token
ad_unit_mapping_ad_unit_mappings = ad_unit_mapping.ad_unit_mappings
```

---


### App

Creates an app under the specified AdMob account. This method has limited access. If you see a 403 permission denied error, please reach out to your account manager for access.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `app_approval_state` | String |  | Output only. The approval state for the app. The field is read-only. |
| `platform` | String |  | Describes the platform of the app. Limited to "IOS" and "ANDROID". |
| `app_id` | String |  | The externally visible ID of the app which can be used to integrate with the AdMob SDK. This is a read only property. Example: ca-app-pub-9876543210987654~0123456789 |
| `linked_app_info` | String |  | Immutable. The information for an app that is linked to an app store. This field is present if and only if the app is linked to an app store. |
| `manual_app_info` | String |  | The information for an app that is not linked to any app store. After an app is linked, this information is still retrivable. If no name is provided for the app upon creation, a placeholder name will be used. |
| `name` | String |  | Resource name for this app. Format is accounts/{publisher_id}/apps/{app_id_fragment} Example: accounts/pub-9876543210987654/apps/0123456789 |
| `parent` | String | ✅ | Required. Resource name of the account for which the app is being created. Example: accounts/pub-9876543210987654 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | If not empty, indicates that there may be more apps for the request; this value should be passed in a new `ListAppsRequest`. |
| `apps` | Vec<String> | The resulting apps for the requested account. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create app
app = provider.admob_api.App {
    parent = "value"  # Required. Resource name of the account for which the app is being created. Example: accounts/pub-9876543210987654
}

# Access app outputs
app_id = app.id
app_next_page_token = app.next_page_token
app_apps = app.apps
```

---


### Ad_source

List the ad sources.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ad_sources` | Vec<String> | The ad sources. |
| `next_page_token` | String | Used to set the `page_token` in the `ListAdSourcesRequest` to retrieve the next page. If this field is omitted, there are no subsequent pages. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access ad_source outputs
ad_source_id = ad_source.id
ad_source_ad_sources = ad_source.ad_sources
ad_source_next_page_token = ad_source.next_page_token
```

---


### Mediation_ab_experiment

Create an A/B testing experiment for a specified AdMob account and a mediation group. This method has limited access. If you see a 403 permission denied error, please reach out to your account manager for access.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `treatment_traffic_percentage` | String |  | The percentage of the mediation A/B experiment traffic that will be send to the treatment (variant B). The remainder is sent to the control (variant A). The percentage is expressed as an integer in the inclusive range of [1,99]. See https://support.google.com/admob/answer/9572326 for details. |
| `name` | String |  | Resource name for this experiment. The format is accounts/{publisher_id}/ mediationGroups/{mediation_group_id}/mediationAbExperiment/ {mediation_group_experiment_id}. For example: accounts/pub-9876543210987654/mediationGroups/0123456789/ mediationAbExperiment/12345 |
| `variant_leader` | String |  | Output only. The variant leader for the experiment according to some key metrics. |
| `treatment_mediation_lines` | Vec<String> |  | The experiment mediation lines created for the treatment. They will be used for serving when the experiment status is RUNNING. |
| `end_time` | String |  | Output only. The time at which the experiment was ended or target to end (in UTC). |
| `experiment_id` | String |  | Output only. Unique identifier for the mediation A/B experiment. It is an output only property. |
| `mediation_group_id` | String |  | Output only. The mediation group id this experiment belongs to. This can be used for filtering the experiments in the list experiments API. |
| `start_time` | String |  | Output only. The time at which the experiment was started (in UTC). |
| `display_name` | String |  | The display name for the mediation A/B experiment. |
| `state` | String |  | Output only. The state of the experiment. It is an output only field. |
| `control_mediation_lines` | Vec<String> |  | Output only. The experiment mediation lines for control. They are inherited from the parent mediation group. It is an output only field. |
| `parent` | String | ✅ | Required. The parent which owns the mediation group. Format: accounts/{publisher_id}/mediationGroups/{mediation_group_id} |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create mediation_ab_experiment
mediation_ab_experiment = provider.admob_api.Mediation_ab_experiment {
    parent = "value"  # Required. The parent which owns the mediation group. Format: accounts/{publisher_id}/mediationGroups/{mediation_group_id}
}

```

---


### Campaign_report

Generates Campaign Report based on provided specifications.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `report_spec` | String |  | Campaign report specification. |
| `parent` | String | ✅ | Resource name of the account to generate the report for. Example: accounts/pub-9876543210987654 |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create campaign_report
campaign_report = provider.admob_api.Campaign_report {
    parent = "value"  # Resource name of the account to generate the report for. Example: accounts/pub-9876543210987654
}

```

---


### Mediation_report

Generates an AdMob Mediation report based on the provided report specification. Returns result of a server-side streaming RPC. The result is returned in a sequence of responses.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `report_spec` | String |  | Network report specification. |
| `parent` | String | ✅ | Resource name of the account to generate the report for. Example: accounts/pub-9876543210987654 |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create mediation_report
mediation_report = provider.admob_api.Mediation_report {
    parent = "value"  # Resource name of the account to generate the report for. Example: accounts/pub-9876543210987654
}

```

---


### Account

Gets information about the specified AdMob publisher account.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `currency_code` | String | Currency code of the earning-related metrics, which is the 3-letter code defined in ISO 4217. The daily average rate is used for the currency conversion. |
| `name` | String | Resource name of this account. Format is accounts/{publisher_id}. |
| `publisher_id` | String | The unique ID by which this publisher account can be identified in the API requests (for example, pub-1234567890). |
| `reporting_time_zone` | String | The time zone that is used in reports that are generated for this account. The value is a time-zone ID as specified by the CLDR project, for example, "America/Los_Angeles". |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access account outputs
account_id = account.id
account_currency_code = account.currency_code
account_name = account.name
account_publisher_id = account.publisher_id
account_reporting_time_zone = account.reporting_time_zone
```

---


### Mediation_group

Create a mediation group under the specific AdMob account. This method has limited access. If you see a 403 permission denied error, please reach out to your account manager for access.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state` | String |  | The status of the mediation group. Only enabled mediation groups will be served. |
| `targeting` | String |  | Set of criteria targeted by this mediation group, such as ad units and geo locations. |
| `mediation_group_id` | String |  | The ID of the mediation group. Example: "0123456789". This is a read only property. |
| `display_name` | String |  | User provided name for the mediation group. The maximum length allowed is 120 characters. |
| `mediation_group_lines` | HashMap<String, String> |  | The mediation lines used for serving for this mediation group. Key is the ID of the mediation group line. For creation, use distinct negative values as placeholder. |
| `mediation_ab_experiment_state` | String |  | Output only. The state of the mediation a/b experiment that belongs to this mediation group. |
| `name` | String |  | Resource name for this mediation group. Format is: accounts/{publisher_id}/mediationGroups/{mediation_group_id} Example: accounts/pub-9876543210987654/mediationGroups/0123456789 |
| `parent` | String | ✅ | Required. The parent which owns the mediation group. Format: accounts/{publisher_id} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | If not empty, indicates that there may be more mediation groups for the request; this value should be passed in a new `ListMediationGroupsRequest`. |
| `mediation_groups` | Vec<String> | The resulting mediation groups for the requested account. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create mediation_group
mediation_group = provider.admob_api.Mediation_group {
    parent = "value"  # Required. The parent which owns the mediation group. Format: accounts/{publisher_id}
}

# Access mediation_group outputs
mediation_group_id = mediation_group.id
mediation_group_next_page_token = mediation_group.next_page_token
mediation_group_mediation_groups = mediation_group.mediation_groups
```

---


### Ad_unit

Creates an ad unit under the specified AdMob account. This method has limited access. If you see a 403 permission denied error, please reach out to your account manager for access.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ad_format` | String |  | AdFormat of the ad unit. Possible values are as follows: "APP_OPEN" - App Open ad format. "BANNER" - Banner ad format. "BANNER_INTERSTITIAL" - Legacy format that can be used as either banner or interstitial. This format can no longer be created but can be targeted by mediation groups. "INTERSTITIAL" - A full screen ad. Supported ad types are "RICH_MEDIA" and "VIDEO". "NATIVE" - Native ad format. "REWARDED" - An ad that, once viewed, gets a callback verifying the view so that a reward can be given to the user. Supported ad types are "RICH_MEDIA" (interactive) and video where video can not be excluded. "REWARDED_INTERSTITIAL" - Rewarded Interstitial ad format. Only supports video ad type. See https://support.google.com/admob/answer/9884467. |
| `name` | String |  | Resource name for this ad unit. Format is accounts/{publisher_id}/adUnits/{ad_unit_id_fragment} Example: accounts/pub-9876543210987654/adUnits/0123456789 |
| `app_id` | String |  | The externally visible ID of the app this ad unit is associated with. Example: ca-app-pub-9876543210987654~0123456789 |
| `ad_unit_id` | String |  | The externally visible ID of the ad unit which can be used to integrate with the AdMob SDK. This is a read only property. Example: ca-app-pub-9876543210987654/0123456789 |
| `ad_types` | Vec<String> |  | Ad media type supported by this ad unit. Possible values as follows: "RICH_MEDIA" - Text, image, and other non-video media. "VIDEO" - Video media. |
| `display_name` | String |  | The display name of the ad unit as shown in the AdMob UI, which is provided by the user. The maximum length allowed is 80 characters. |
| `reward_settings` | String |  | Optional. Settings for a rewarded ad unit. This can be set or unset only when the ad_format is "REWARDED". |
| `parent` | String | ✅ | Required. Resource name of the account to create the specified ad unit for. Example: accounts/pub-9876543210987654 |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ad_units` | Vec<String> | The resulting ad units for the requested account. |
| `next_page_token` | String | If not empty, indicates that there may be more ad units for the request; this value should be passed in a new `ListAdUnitsRequest`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create ad_unit
ad_unit = provider.admob_api.Ad_unit {
    parent = "value"  # Required. Resource name of the account to create the specified ad unit for. Example: accounts/pub-9876543210987654
}

# Access ad_unit outputs
ad_unit_id = ad_unit.id
ad_unit_ad_units = ad_unit.ad_units
ad_unit_next_page_token = ad_unit.next_page_token
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple app resources
app_0 = provider.admob_api.App {
}
app_1 = provider.admob_api.App {
}
app_2 = provider.admob_api.App {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    app = provider.admob_api.App {
    }
```

---

## Related Documentation

- [GCP Admob_api Documentation](https://cloud.google.com/admob_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
