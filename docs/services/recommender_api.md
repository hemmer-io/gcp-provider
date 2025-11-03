# Recommender_api Service



**Resources**: 9

---

## Overview

The recommender_api service provides access to 9 resource types:

- [Insight_type](#insight_type) [RU]
- [Insight](#insight) [CR]
- [Recommendation](#recommendation) [CR]
- [Recommender](#recommender) [RU]
- [Location](#location) [R]
- [Recommender](#recommender) [RU]
- [Insight_type](#insight_type) [RU]
- [Insight](#insight) [CR]
- [Recommendation](#recommendation) [CR]

---

## Resources


### Insight_type

Gets the requested InsightTypeConfig. There is only one instance of the config for each InsightType.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Identifier. Name of insight type config. Eg, projects/[PROJECT_NUMBER]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]/config |
| `update_time` | String |  | Last time when the config was updated. |
| `annotations` | HashMap<String, String> |  | Allows clients to store small amounts of arbitrary data. Annotations must follow the Kubernetes syntax. The total size of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. |
| `revision_id` | String |  | Output only. Immutable. The revision ID of the config. A new revision is committed whenever the config is changed in any way. The format is an 8-character hexadecimal string. |
| `etag` | String |  | Fingerprint of the InsightTypeConfig. Provides optimistic locking when updating. |
| `insight_type_generation_config` | String |  | InsightTypeGenerationConfig which configures the generation of insights for this insight type. |
| `display_name` | String |  | A user-settable field to provide a human-readable name to be used in user interfaces. |
| `name` | String | ✅ | Identifier. Name of insight type config. Eg, projects/[PROJECT_NUMBER]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]/config |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Identifier. Name of insight type config. Eg, projects/[PROJECT_NUMBER]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]/config |
| `update_time` | String | Last time when the config was updated. |
| `annotations` | HashMap<String, String> | Allows clients to store small amounts of arbitrary data. Annotations must follow the Kubernetes syntax. The total size of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. |
| `revision_id` | String | Output only. Immutable. The revision ID of the config. A new revision is committed whenever the config is changed in any way. The format is an 8-character hexadecimal string. |
| `etag` | String | Fingerprint of the InsightTypeConfig. Provides optimistic locking when updating. |
| `insight_type_generation_config` | String | InsightTypeGenerationConfig which configures the generation of insights for this insight type. |
| `display_name` | String | A user-settable field to provide a human-readable name to be used in user interfaces. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access insight_type outputs
insight_type_id = insight_type.id
insight_type_name = insight_type.name
insight_type_update_time = insight_type.update_time
insight_type_annotations = insight_type.annotations
insight_type_revision_id = insight_type.revision_id
insight_type_etag = insight_type.etag
insight_type_insight_type_generation_config = insight_type.insight_type_generation_config
insight_type_display_name = insight_type.display_name
```

---


### Insight

Marks the Insight State as Accepted. Users can use this method to indicate to the Recommender API that they have applied some action based on the insight. This stops the insight content from being updated. MarkInsightAccepted can be applied to insights in ACTIVE state. Requires the recommender.*.update IAM permission for the specified insight.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state_metadata` | HashMap<String, String> |  | Optional. State properties user wish to include with this state. Full replace of the current state_metadata. |
| `etag` | String |  | Required. Fingerprint of the Insight. Provides optimistic locking. |
| `name` | String | ✅ | Required. Name of the insight. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `content` | HashMap<String, String> | A struct of custom fields to explain the insight. Example: "grantedPermissionsCount": "1000" |
| `insight_subtype` | String | Insight subtype. Insight content schema will be stable for a given subtype. |
| `last_refresh_time` | String | Timestamp of the latest data used to generate the insight. |
| `severity` | String | Insight's severity. |
| `description` | String | Free-form human readable summary in English. The maximum length is 500 characters. |
| `category` | String | Category being targeted by the insight. |
| `associated_recommendations` | Vec<String> | Recommendations derived from this insight. |
| `etag` | String | Fingerprint of the Insight. Provides optimistic locking when updating states. |
| `state_info` | String | Information state and metadata. |
| `name` | String | Identifier. Name of the insight. |
| `target_resources` | Vec<String> | Fully qualified resource names that this insight is targeting. |
| `observation_period` | String | Observation period that led to the insight. The source data used to generate the insight ends at last_refresh_time and begins at (last_refresh_time - observation_period). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create insight
insight = provider.recommender_api.Insight {
    name = "value"  # Required. Name of the insight.
}

# Access insight outputs
insight_id = insight.id
insight_content = insight.content
insight_insight_subtype = insight.insight_subtype
insight_last_refresh_time = insight.last_refresh_time
insight_severity = insight.severity
insight_description = insight.description
insight_category = insight.category
insight_associated_recommendations = insight.associated_recommendations
insight_etag = insight.etag
insight_state_info = insight.state_info
insight_name = insight.name
insight_target_resources = insight.target_resources
insight_observation_period = insight.observation_period
```

---


### Recommendation

Marks the Recommendation State as Claimed. Users can use this method to indicate to the Recommender API that they are starting to apply the recommendation themselves. This stops the recommendation content from being updated. Associated insights are frozen and placed in the ACCEPTED state. MarkRecommendationClaimed can be applied to recommendations in CLAIMED, SUCCEEDED, FAILED, or ACTIVE state. Requires the recommender.*.update IAM permission for the specified recommender.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state_metadata` | HashMap<String, String> |  | State properties to include with this state. Overwrites any existing `state_metadata`. Keys must match the regex `/^a-z0-9{0,62}$/`. Values must match the regex `/^[a-zA-Z0-9_./-]{0,255}$/`. |
| `etag` | String |  | Required. Fingerprint of the Recommendation. Provides optimistic locking. |
| `name` | String | ✅ | Required. Name of the recommendation. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `primary_impact` | String | The primary impact that this recommendation can have while trying to optimize for one category. |
| `target_resources` | Vec<String> | Fully qualified resource names that this recommendation is targeting. |
| `xor_group_id` | String | Corresponds to a mutually exclusive group ID within a recommender. A non-empty ID indicates that the recommendation belongs to a mutually exclusive group. This means that only one recommendation within the group is suggested to be applied. |
| `name` | String | Identifier. Name of recommendation. |
| `additional_impact` | Vec<String> | Optional set of additional impact that this recommendation may have when trying to optimize for the primary category. These may be positive or negative. |
| `last_refresh_time` | String | Last time this recommendation was refreshed by the system that created it in the first place. |
| `priority` | String | Recommendation's priority. |
| `content` | String | Content of the recommendation describing recommended changes to resources. |
| `recommender_subtype` | String | Contains an identifier for a subtype of recommendations produced for the same recommender. Subtype is a function of content and impact, meaning a new subtype might be added when significant changes to `content` or `primary_impact.category` are introduced. See the Recommenders section to see a list of subtypes for a given Recommender. Examples: For recommender = "google.iam.policy.Recommender", recommender_subtype can be one of "REMOVE_ROLE"/"REPLACE_ROLE" |
| `associated_insights` | Vec<String> | Insights that led to this recommendation. |
| `description` | String | Free-form human readable summary in English. The maximum length is 500 characters. |
| `etag` | String | Fingerprint of the Recommendation. Provides optimistic locking when updating states. |
| `state_info` | String | Information for state. Contains state and metadata. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create recommendation
recommendation = provider.recommender_api.Recommendation {
    name = "value"  # Required. Name of the recommendation.
}

# Access recommendation outputs
recommendation_id = recommendation.id
recommendation_primary_impact = recommendation.primary_impact
recommendation_target_resources = recommendation.target_resources
recommendation_xor_group_id = recommendation.xor_group_id
recommendation_name = recommendation.name
recommendation_additional_impact = recommendation.additional_impact
recommendation_last_refresh_time = recommendation.last_refresh_time
recommendation_priority = recommendation.priority
recommendation_content = recommendation.content
recommendation_recommender_subtype = recommendation.recommender_subtype
recommendation_associated_insights = recommendation.associated_insights
recommendation_description = recommendation.description
recommendation_etag = recommendation.etag
recommendation_state_info = recommendation.state_info
```

---


### Recommender

Gets the requested Recommender Config. There is only one instance of the config for each Recommender.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `etag` | String |  | Fingerprint of the RecommenderConfig. Provides optimistic locking when updating. |
| `name` | String |  | Identifier. Name of recommender config. Eg, projects/[PROJECT_NUMBER]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]/config |
| `update_time` | String |  | Last time when the config was updated. |
| `display_name` | String |  | A user-settable field to provide a human-readable name to be used in user interfaces. |
| `annotations` | HashMap<String, String> |  | Allows clients to store small amounts of arbitrary data. Annotations must follow the Kubernetes syntax. The total size of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. |
| `revision_id` | String |  | Output only. Immutable. The revision ID of the config. A new revision is committed whenever the config is changed in any way. The format is an 8-character hexadecimal string. |
| `recommender_generation_config` | String |  | RecommenderGenerationConfig which configures the Generation of recommendations for this recommender. |
| `name` | String | ✅ | Identifier. Name of recommender config. Eg, projects/[PROJECT_NUMBER]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]/config |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `etag` | String | Fingerprint of the RecommenderConfig. Provides optimistic locking when updating. |
| `name` | String | Identifier. Name of recommender config. Eg, projects/[PROJECT_NUMBER]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]/config |
| `update_time` | String | Last time when the config was updated. |
| `display_name` | String | A user-settable field to provide a human-readable name to be used in user interfaces. |
| `annotations` | HashMap<String, String> | Allows clients to store small amounts of arbitrary data. Annotations must follow the Kubernetes syntax. The total size of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. |
| `revision_id` | String | Output only. Immutable. The revision ID of the config. A new revision is committed whenever the config is changed in any way. The format is an 8-character hexadecimal string. |
| `recommender_generation_config` | String | RecommenderGenerationConfig which configures the Generation of recommendations for this recommender. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access recommender outputs
recommender_id = recommender.id
recommender_etag = recommender.etag
recommender_name = recommender.name
recommender_update_time = recommender.update_time
recommender_display_name = recommender.display_name
recommender_annotations = recommender.annotations
recommender_revision_id = recommender.revision_id
recommender_recommender_generation_config = recommender.recommender_generation_config
```

---


### Location

Lists locations with recommendations or insights.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | The standard List next-page token. |
| `locations` | Vec<String> | A list of locations that matches the specified filter in the request. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access location outputs
location_id = location.id
location_next_page_token = location.next_page_token
location_locations = location.locations
```

---


### Recommender

Gets the requested Recommender Config. There is only one instance of the config for each Recommender.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `display_name` | String |  | A user-settable field to provide a human-readable name to be used in user interfaces. |
| `annotations` | HashMap<String, String> |  | Allows clients to store small amounts of arbitrary data. Annotations must follow the Kubernetes syntax. The total size of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. |
| `name` | String |  | Identifier. Name of recommender config. Eg, projects/[PROJECT_NUMBER]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]/config |
| `recommender_generation_config` | String |  | RecommenderGenerationConfig which configures the Generation of recommendations for this recommender. |
| `etag` | String |  | Fingerprint of the RecommenderConfig. Provides optimistic locking when updating. |
| `revision_id` | String |  | Output only. Immutable. The revision ID of the config. A new revision is committed whenever the config is changed in any way. The format is an 8-character hexadecimal string. |
| `update_time` | String |  | Last time when the config was updated. |
| `name` | String | ✅ | Identifier. Name of recommender config. Eg, projects/[PROJECT_NUMBER]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]/config |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `display_name` | String | A user-settable field to provide a human-readable name to be used in user interfaces. |
| `annotations` | HashMap<String, String> | Allows clients to store small amounts of arbitrary data. Annotations must follow the Kubernetes syntax. The total size of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. |
| `name` | String | Identifier. Name of recommender config. Eg, projects/[PROJECT_NUMBER]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]/config |
| `recommender_generation_config` | String | RecommenderGenerationConfig which configures the Generation of recommendations for this recommender. |
| `etag` | String | Fingerprint of the RecommenderConfig. Provides optimistic locking when updating. |
| `revision_id` | String | Output only. Immutable. The revision ID of the config. A new revision is committed whenever the config is changed in any way. The format is an 8-character hexadecimal string. |
| `update_time` | String | Last time when the config was updated. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access recommender outputs
recommender_id = recommender.id
recommender_display_name = recommender.display_name
recommender_annotations = recommender.annotations
recommender_name = recommender.name
recommender_recommender_generation_config = recommender.recommender_generation_config
recommender_etag = recommender.etag
recommender_revision_id = recommender.revision_id
recommender_update_time = recommender.update_time
```

---


### Insight_type

Gets the requested InsightTypeConfig. There is only one instance of the config for each InsightType.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `annotations` | HashMap<String, String> |  | Allows clients to store small amounts of arbitrary data. Annotations must follow the Kubernetes syntax. The total size of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. |
| `revision_id` | String |  | Output only. Immutable. The revision ID of the config. A new revision is committed whenever the config is changed in any way. The format is an 8-character hexadecimal string. |
| `update_time` | String |  | Last time when the config was updated. |
| `etag` | String |  | Fingerprint of the InsightTypeConfig. Provides optimistic locking when updating. |
| `name` | String |  | Identifier. Name of insight type config. Eg, projects/[PROJECT_NUMBER]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]/config |
| `insight_type_generation_config` | String |  | InsightTypeGenerationConfig which configures the generation of insights for this insight type. |
| `display_name` | String |  | A user-settable field to provide a human-readable name to be used in user interfaces. |
| `name` | String | ✅ | Identifier. Name of insight type config. Eg, projects/[PROJECT_NUMBER]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]/config |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `annotations` | HashMap<String, String> | Allows clients to store small amounts of arbitrary data. Annotations must follow the Kubernetes syntax. The total size of all keys and values combined is limited to 256k. Key can have 2 segments: prefix (optional) and name (required), separated by a slash (/). Prefix must be a DNS subdomain. Name must be 63 characters or less, begin and end with alphanumerics, with dashes (-), underscores (_), dots (.), and alphanumerics between. |
| `revision_id` | String | Output only. Immutable. The revision ID of the config. A new revision is committed whenever the config is changed in any way. The format is an 8-character hexadecimal string. |
| `update_time` | String | Last time when the config was updated. |
| `etag` | String | Fingerprint of the InsightTypeConfig. Provides optimistic locking when updating. |
| `name` | String | Identifier. Name of insight type config. Eg, projects/[PROJECT_NUMBER]/locations/[LOCATION]/insightTypes/[INSIGHT_TYPE_ID]/config |
| `insight_type_generation_config` | String | InsightTypeGenerationConfig which configures the generation of insights for this insight type. |
| `display_name` | String | A user-settable field to provide a human-readable name to be used in user interfaces. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access insight_type outputs
insight_type_id = insight_type.id
insight_type_annotations = insight_type.annotations
insight_type_revision_id = insight_type.revision_id
insight_type_update_time = insight_type.update_time
insight_type_etag = insight_type.etag
insight_type_name = insight_type.name
insight_type_insight_type_generation_config = insight_type.insight_type_generation_config
insight_type_display_name = insight_type.display_name
```

---


### Insight

Marks the Insight State as Accepted. Users can use this method to indicate to the Recommender API that they have applied some action based on the insight. This stops the insight content from being updated. MarkInsightAccepted can be applied to insights in ACTIVE state. Requires the recommender.*.update IAM permission for the specified insight.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state_metadata` | HashMap<String, String> |  | Optional. State properties user wish to include with this state. Full replace of the current state_metadata. |
| `etag` | String |  | Required. Fingerprint of the Insight. Provides optimistic locking. |
| `name` | String | ✅ | Required. Name of the insight. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `associated_recommendations` | Vec<String> | Recommendations derived from this insight. |
| `etag` | String | Fingerprint of the Insight. Provides optimistic locking when updating states. |
| `category` | String | Category being targeted by the insight. |
| `insight_subtype` | String | Insight subtype. Insight content schema will be stable for a given subtype. |
| `severity` | String | Insight's severity. |
| `target_resources` | Vec<String> | Fully qualified resource names that this insight is targeting. |
| `last_refresh_time` | String | Timestamp of the latest data used to generate the insight. |
| `content` | HashMap<String, String> | A struct of custom fields to explain the insight. Example: "grantedPermissionsCount": "1000" |
| `observation_period` | String | Observation period that led to the insight. The source data used to generate the insight ends at last_refresh_time and begins at (last_refresh_time - observation_period). |
| `name` | String | Identifier. Name of the insight. |
| `description` | String | Free-form human readable summary in English. The maximum length is 500 characters. |
| `state_info` | String | Information state and metadata. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create insight
insight = provider.recommender_api.Insight {
    name = "value"  # Required. Name of the insight.
}

# Access insight outputs
insight_id = insight.id
insight_associated_recommendations = insight.associated_recommendations
insight_etag = insight.etag
insight_category = insight.category
insight_insight_subtype = insight.insight_subtype
insight_severity = insight.severity
insight_target_resources = insight.target_resources
insight_last_refresh_time = insight.last_refresh_time
insight_content = insight.content
insight_observation_period = insight.observation_period
insight_name = insight.name
insight_description = insight.description
insight_state_info = insight.state_info
```

---


### Recommendation

Marks the Recommendation State as Failed. Users can use this method to indicate to the Recommender API that they have applied the recommendation themselves, and the operation failed. This stops the recommendation content from being updated. Associated insights are frozen and placed in the ACCEPTED state. MarkRecommendationFailed can be applied to recommendations in ACTIVE, CLAIMED, SUCCEEDED, or FAILED state. Requires the recommender.*.update IAM permission for the specified recommender.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `state_metadata` | HashMap<String, String> |  | State properties to include with this state. Overwrites any existing `state_metadata`. Keys must match the regex `/^a-z0-9{0,62}$/`. Values must match the regex `/^[a-zA-Z0-9_./-]{0,255}$/`. |
| `etag` | String |  | Required. Fingerprint of the Recommendation. Provides optimistic locking. |
| `name` | String | ✅ | Required. Name of the recommendation. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `state_info` | String | Information for state. Contains state and metadata. |
| `associated_insights` | Vec<String> | Insights that led to this recommendation. |
| `target_resources` | Vec<String> | Fully qualified resource names that this recommendation is targeting. |
| `description` | String | Free-form human readable summary in English. The maximum length is 500 characters. |
| `name` | String | Identifier. Name of recommendation. |
| `recommender_subtype` | String | Contains an identifier for a subtype of recommendations produced for the same recommender. Subtype is a function of content and impact, meaning a new subtype might be added when significant changes to `content` or `primary_impact.category` are introduced. See the Recommenders section to see a list of subtypes for a given Recommender. Examples: For recommender = "google.iam.policy.Recommender", recommender_subtype can be one of "REMOVE_ROLE"/"REPLACE_ROLE" |
| `content` | String | Content of the recommendation describing recommended changes to resources. |
| `priority` | String | Recommendation's priority. |
| `primary_impact` | String | The primary impact that this recommendation can have while trying to optimize for one category. |
| `last_refresh_time` | String | Last time this recommendation was refreshed by the system that created it in the first place. |
| `additional_impact` | Vec<String> | Optional set of additional impact that this recommendation may have when trying to optimize for the primary category. These may be positive or negative. |
| `xor_group_id` | String | Corresponds to a mutually exclusive group ID within a recommender. A non-empty ID indicates that the recommendation belongs to a mutually exclusive group. This means that only one recommendation within the group is suggested to be applied. |
| `etag` | String | Fingerprint of the Recommendation. Provides optimistic locking when updating states. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create recommendation
recommendation = provider.recommender_api.Recommendation {
    name = "value"  # Required. Name of the recommendation.
}

# Access recommendation outputs
recommendation_id = recommendation.id
recommendation_state_info = recommendation.state_info
recommendation_associated_insights = recommendation.associated_insights
recommendation_target_resources = recommendation.target_resources
recommendation_description = recommendation.description
recommendation_name = recommendation.name
recommendation_recommender_subtype = recommendation.recommender_subtype
recommendation_content = recommendation.content
recommendation_priority = recommendation.priority
recommendation_primary_impact = recommendation.primary_impact
recommendation_last_refresh_time = recommendation.last_refresh_time
recommendation_additional_impact = recommendation.additional_impact
recommendation_xor_group_id = recommendation.xor_group_id
recommendation_etag = recommendation.etag
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple insight_type resources
insight_type_0 = provider.recommender_api.Insight_type {
    name = "value-0"
}
insight_type_1 = provider.recommender_api.Insight_type {
    name = "value-1"
}
insight_type_2 = provider.recommender_api.Insight_type {
    name = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    insight_type = provider.recommender_api.Insight_type {
        name = "production-value"
    }
```

---

## Related Documentation

- [GCP Recommender_api Documentation](https://cloud.google.com/recommender_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
