# Retail_api Service



**Resources**: 38

---

## Overview

The retail_api service provides access to 38 resource types:

- [User_event](#user_event) [C]
- [Attributes_config](#attributes_config) [C]
- [Control](#control) [CRUD]
- [Serving_config](#serving_config) [CRUD]
- [Model](#model) [CRUD]
- [Catalog](#catalog) [CRU]
- [Product](#product) [CRUD]
- [Placement](#placement) [C]
- [Project](#project) [RU]
- [Operation](#operation) [R]
- [Completion_data](#completion_data) [C]
- [Generative_question](#generative_question) [CR]
- [Serving_config](#serving_config) [CRUD]
- [Product](#product) [CRUD]
- [Completion_data](#completion_data) [C]
- [Attributes_config](#attributes_config) [C]
- [Placement](#placement) [C]
- [Generative_question](#generative_question) [CR]
- [Model](#model) [CRUD]
- [Operation](#operation) [R]
- [User_event](#user_event) [C]
- [Catalog](#catalog) [CRU]
- [Control](#control) [CRUD]
- [Catalog](#catalog) [CRU]
- [Control](#control) [CRUD]
- [User_event](#user_event) [C]
- [Model](#model) [CRUD]
- [Merchant_center_account_link](#merchant_center_account_link) [CRD]
- [Product](#product) [CRUD]
- [Branche](#branche) [R]
- [Serving_config](#serving_config) [CRUD]
- [Generative_question](#generative_question) [CR]
- [Project](#project) [CRU]
- [Placement](#placement) [C]
- [Retail_project](#retail_project) [C]
- [Attributes_config](#attributes_config) [C]
- [Completion_data](#completion_data) [C]
- [Operation](#operation) [R]

---

## Resources


### User_event

Bulk import of User events. Request processing might be synchronous. Events that already exist are skipped. Use this method for backfilling historical user events. `Operation.response` is of type `ImportResponse`. Note that it is possible for a subset of the items to be successfully inserted. `Operation.metadata` is of type `ImportMetadata`.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `errors_config` | String |  | The desired location of errors incurred during the Import. Cannot be set for inline user event imports. |
| `input_config` | String |  | Required. The desired input location of the data. |
| `parent` | String | ✅ | Required. `projects/1234/locations/global/catalogs/default_catalog` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user_event
user_event = provider.retail_api.User_event {
    parent = "value"  # Required. `projects/1234/locations/global/catalogs/default_catalog`
}

```

---


### Attributes_config

Adds the specified CatalogAttribute to the AttributesConfig. If the CatalogAttribute to add already exists, an ALREADY_EXISTS error is returned.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `catalog_attribute` | String |  | Required. The CatalogAttribute to add. |
| `attributes_config` | String | ✅ | Required. Full AttributesConfig resource name. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/attributesConfig` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create attributes_config
attributes_config = provider.retail_api.Attributes_config {
    attributes_config = "value"  # Required. Full AttributesConfig resource name. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/attributesConfig`
}

```

---


### Control

Creates a Control. If the Control to create already exists, an ALREADY_EXISTS error is returned.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `search_solution_use_case` | Vec<String> |  | Specifies the use case for the control. Affects what condition fields can be set. Only settable by search controls. Will default to SEARCH_SOLUTION_USE_CASE_SEARCH if not specified. Currently only allow one search_solution_use_case per control. |
| `name` | String |  | Immutable. Fully qualified name `projects/*/locations/global/catalogs/*/controls/*` |
| `rule` | String |  | A rule control - a condition-action pair. Enacts a set action when the condition is triggered. For example: Boost "gShoe" when query full matches "Running Shoes". |
| `associated_serving_config_ids` | Vec<String> |  | Output only. List of serving config ids that are associated with this control in the same Catalog. Note the association is managed via the ServingConfig, this is an output only denormalized view. |
| `display_name` | String |  | Required. The human readable control display name. Used in Retail UI. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is thrown. |
| `solution_types` | Vec<String> |  | Required. Immutable. The solution types that the control is used for. Currently we support setting only one type of solution at creation time. Only `SOLUTION_TYPE_SEARCH` value is supported at the moment. If no solution type is provided at creation time, will default to SOLUTION_TYPE_SEARCH. |
| `parent` | String | ✅ | Required. Full resource name of parent catalog. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `search_solution_use_case` | Vec<String> | Specifies the use case for the control. Affects what condition fields can be set. Only settable by search controls. Will default to SEARCH_SOLUTION_USE_CASE_SEARCH if not specified. Currently only allow one search_solution_use_case per control. |
| `name` | String | Immutable. Fully qualified name `projects/*/locations/global/catalogs/*/controls/*` |
| `rule` | String | A rule control - a condition-action pair. Enacts a set action when the condition is triggered. For example: Boost "gShoe" when query full matches "Running Shoes". |
| `associated_serving_config_ids` | Vec<String> | Output only. List of serving config ids that are associated with this control in the same Catalog. Note the association is managed via the ServingConfig, this is an output only denormalized view. |
| `display_name` | String | Required. The human readable control display name. Used in Retail UI. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is thrown. |
| `solution_types` | Vec<String> | Required. Immutable. The solution types that the control is used for. Currently we support setting only one type of solution at creation time. Only `SOLUTION_TYPE_SEARCH` value is supported at the moment. If no solution type is provided at creation time, will default to SOLUTION_TYPE_SEARCH. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create control
control = provider.retail_api.Control {
    parent = "value"  # Required. Full resource name of parent catalog. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}`
}

# Access control outputs
control_id = control.id
control_search_solution_use_case = control.search_solution_use_case
control_name = control.name
control_rule = control.rule
control_associated_serving_config_ids = control.associated_serving_config_ids
control_display_name = control.display_name
control_solution_types = control.solution_types
```

---


### Serving_config

Creates a ServingConfig. A maximum of 100 ServingConfigs are allowed in a Catalog, otherwise a FAILED_PRECONDITION error is returned.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `replacement_control_ids` | Vec<String> |  | Condition replacement specifications. - Applied according to the order in the list. - A previously replaced term can not be re-replaced. - Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `redirect_control_ids` | Vec<String> |  | Condition redirect specifications. Only the first triggered redirect action is applied, even if multiple apply. Maximum number of specifications is 1000. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `display_name` | String |  | Required. The human readable serving config display name. Used in Retail UI. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `do_not_associate_control_ids` | Vec<String> |  | Condition do not associate specifications. If multiple do not associate conditions match, all matching do not associate controls in the list will execute. - Order does not matter. - Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `diversity_type` | String |  | What kind of diversity to use - data driven or rule based. If unset, the server behavior defaults to RULE_BASED_DIVERSITY. |
| `enable_category_filter_level` | String |  | Whether to add additional category filters on the `similar-items` model. If not specified, we enable it by default. Allowed values are: * `no-category-match`: No additional filtering of original results from the model and the customer's filters. * `relaxed-category-match`: Only keep results with categories that match at least one item categories in the PredictRequests's context item. * If customer also sends filters in the PredictRequest, then the results will satisfy both conditions (user given and category match). Can only be set if solution_types is SOLUTION_TYPE_RECOMMENDATION. |
| `ignore_recs_denylist` | bool |  | When the flag is enabled, the products in the denylist will not be filtered out in the recommendation filtering results. |
| `name` | String |  | Immutable. Fully qualified name `projects/*/locations/global/catalogs/*/servingConfig/*` |
| `diversity_level` | String |  | How much diversity to use in recommendation model results e.g. `medium-diversity` or `high-diversity`. Currently supported values: * `no-diversity` * `low-diversity` * `medium-diversity` * `high-diversity` * `auto-diversity` If not specified, we choose default based on recommendation model type. Default value: `no-diversity`. Can only be set if solution_types is SOLUTION_TYPE_RECOMMENDATION. |
| `boost_control_ids` | Vec<String> |  | Condition boost specifications. If a product matches multiple conditions in the specifications, boost scores from these specifications are all applied and combined in a non-linear way. Maximum number of specifications is 100. Notice that if both ServingConfig.boost_control_ids and SearchRequest.boost_spec are set, the boost conditions from both places are evaluated. If a search request matches multiple boost conditions, the final boost score is equal to the sum of the boost scores from all matched boost conditions. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `dynamic_facet_spec` | String |  | The specification for dynamically generated facets. Notice that only textual facets can be dynamically generated. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `price_reranking_level` | String |  | How much price ranking we want in serving results. Price reranking causes product items with a similar recommendation probability to be ordered by price, with the highest-priced items first. This setting could result in a decrease in click-through and conversion rates. Allowed values are: * `no-price-reranking` * `low-price-reranking` * `medium-price-reranking` * `high-price-reranking` If not specified, we choose default based on model type. Default value: `no-price-reranking`. Can only be set if solution_types is SOLUTION_TYPE_RECOMMENDATION. |
| `personalization_spec` | String |  | The specification for personalization spec. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. Notice that if both ServingConfig.personalization_spec and SearchRequest.personalization_spec are set. SearchRequest.personalization_spec will override ServingConfig.personalization_spec. |
| `oneway_synonyms_control_ids` | Vec<String> |  | Condition oneway synonyms specifications. If multiple oneway synonyms conditions match, all matching oneway synonyms controls in the list will execute. Order of controls in the list will not matter. Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `filter_control_ids` | Vec<String> |  | Condition filter specifications. If a product matches multiple conditions in the specifications, filters from these specifications are all applied and combined via the AND operator. Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `ignore_control_ids` | Vec<String> |  | Condition ignore specifications. If multiple ignore conditions match, all matching ignore controls in the list will execute. - Order does not matter. - Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `model_id` | String |  | The id of the model in the same Catalog to use at serving time. Currently only RecommendationModels are supported: https://cloud.google.com/retail/recommendations-ai/docs/create-models Can be changed but only to a compatible model (e.g. others-you-may-like CTR to others-you-may-like CVR). Required when solution_types is SOLUTION_TYPE_RECOMMENDATION. |
| `solution_types` | Vec<String> |  | Required. Immutable. Specifies the solution types that a serving config can be associated with. Currently we support setting only one type of solution. |
| `facet_control_ids` | Vec<String> |  | Facet specifications for faceted search. If empty, no facets are returned. The ids refer to the ids of Control resources with only the Facet control set. These controls are assumed to be in the same Catalog as the ServingConfig. A maximum of 100 values are allowed. Otherwise, an INVALID_ARGUMENT error is returned. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `twoway_synonyms_control_ids` | Vec<String> |  | Condition synonyms specifications. If multiple syonyms conditions match, all matching synonyms control in the list will execute. Order of controls in the list will not matter. Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `parent` | String | ✅ | Required. Full resource name of parent. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `replacement_control_ids` | Vec<String> | Condition replacement specifications. - Applied according to the order in the list. - A previously replaced term can not be re-replaced. - Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `redirect_control_ids` | Vec<String> | Condition redirect specifications. Only the first triggered redirect action is applied, even if multiple apply. Maximum number of specifications is 1000. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `display_name` | String | Required. The human readable serving config display name. Used in Retail UI. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `do_not_associate_control_ids` | Vec<String> | Condition do not associate specifications. If multiple do not associate conditions match, all matching do not associate controls in the list will execute. - Order does not matter. - Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `diversity_type` | String | What kind of diversity to use - data driven or rule based. If unset, the server behavior defaults to RULE_BASED_DIVERSITY. |
| `enable_category_filter_level` | String | Whether to add additional category filters on the `similar-items` model. If not specified, we enable it by default. Allowed values are: * `no-category-match`: No additional filtering of original results from the model and the customer's filters. * `relaxed-category-match`: Only keep results with categories that match at least one item categories in the PredictRequests's context item. * If customer also sends filters in the PredictRequest, then the results will satisfy both conditions (user given and category match). Can only be set if solution_types is SOLUTION_TYPE_RECOMMENDATION. |
| `ignore_recs_denylist` | bool | When the flag is enabled, the products in the denylist will not be filtered out in the recommendation filtering results. |
| `name` | String | Immutable. Fully qualified name `projects/*/locations/global/catalogs/*/servingConfig/*` |
| `diversity_level` | String | How much diversity to use in recommendation model results e.g. `medium-diversity` or `high-diversity`. Currently supported values: * `no-diversity` * `low-diversity` * `medium-diversity` * `high-diversity` * `auto-diversity` If not specified, we choose default based on recommendation model type. Default value: `no-diversity`. Can only be set if solution_types is SOLUTION_TYPE_RECOMMENDATION. |
| `boost_control_ids` | Vec<String> | Condition boost specifications. If a product matches multiple conditions in the specifications, boost scores from these specifications are all applied and combined in a non-linear way. Maximum number of specifications is 100. Notice that if both ServingConfig.boost_control_ids and SearchRequest.boost_spec are set, the boost conditions from both places are evaluated. If a search request matches multiple boost conditions, the final boost score is equal to the sum of the boost scores from all matched boost conditions. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `dynamic_facet_spec` | String | The specification for dynamically generated facets. Notice that only textual facets can be dynamically generated. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `price_reranking_level` | String | How much price ranking we want in serving results. Price reranking causes product items with a similar recommendation probability to be ordered by price, with the highest-priced items first. This setting could result in a decrease in click-through and conversion rates. Allowed values are: * `no-price-reranking` * `low-price-reranking` * `medium-price-reranking` * `high-price-reranking` If not specified, we choose default based on model type. Default value: `no-price-reranking`. Can only be set if solution_types is SOLUTION_TYPE_RECOMMENDATION. |
| `personalization_spec` | String | The specification for personalization spec. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. Notice that if both ServingConfig.personalization_spec and SearchRequest.personalization_spec are set. SearchRequest.personalization_spec will override ServingConfig.personalization_spec. |
| `oneway_synonyms_control_ids` | Vec<String> | Condition oneway synonyms specifications. If multiple oneway synonyms conditions match, all matching oneway synonyms controls in the list will execute. Order of controls in the list will not matter. Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `filter_control_ids` | Vec<String> | Condition filter specifications. If a product matches multiple conditions in the specifications, filters from these specifications are all applied and combined via the AND operator. Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `ignore_control_ids` | Vec<String> | Condition ignore specifications. If multiple ignore conditions match, all matching ignore controls in the list will execute. - Order does not matter. - Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `model_id` | String | The id of the model in the same Catalog to use at serving time. Currently only RecommendationModels are supported: https://cloud.google.com/retail/recommendations-ai/docs/create-models Can be changed but only to a compatible model (e.g. others-you-may-like CTR to others-you-may-like CVR). Required when solution_types is SOLUTION_TYPE_RECOMMENDATION. |
| `solution_types` | Vec<String> | Required. Immutable. Specifies the solution types that a serving config can be associated with. Currently we support setting only one type of solution. |
| `facet_control_ids` | Vec<String> | Facet specifications for faceted search. If empty, no facets are returned. The ids refer to the ids of Control resources with only the Facet control set. These controls are assumed to be in the same Catalog as the ServingConfig. A maximum of 100 values are allowed. Otherwise, an INVALID_ARGUMENT error is returned. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `twoway_synonyms_control_ids` | Vec<String> | Condition synonyms specifications. If multiple syonyms conditions match, all matching synonyms control in the list will execute. Order of controls in the list will not matter. Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create serving_config
serving_config = provider.retail_api.Serving_config {
    parent = "value"  # Required. Full resource name of parent. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}`
}

# Access serving_config outputs
serving_config_id = serving_config.id
serving_config_replacement_control_ids = serving_config.replacement_control_ids
serving_config_redirect_control_ids = serving_config.redirect_control_ids
serving_config_display_name = serving_config.display_name
serving_config_do_not_associate_control_ids = serving_config.do_not_associate_control_ids
serving_config_diversity_type = serving_config.diversity_type
serving_config_enable_category_filter_level = serving_config.enable_category_filter_level
serving_config_ignore_recs_denylist = serving_config.ignore_recs_denylist
serving_config_name = serving_config.name
serving_config_diversity_level = serving_config.diversity_level
serving_config_boost_control_ids = serving_config.boost_control_ids
serving_config_dynamic_facet_spec = serving_config.dynamic_facet_spec
serving_config_price_reranking_level = serving_config.price_reranking_level
serving_config_personalization_spec = serving_config.personalization_spec
serving_config_oneway_synonyms_control_ids = serving_config.oneway_synonyms_control_ids
serving_config_filter_control_ids = serving_config.filter_control_ids
serving_config_ignore_control_ids = serving_config.ignore_control_ids
serving_config_model_id = serving_config.model_id
serving_config_solution_types = serving_config.solution_types
serving_config_facet_control_ids = serving_config.facet_control_ids
serving_config_twoway_synonyms_control_ids = serving_config.twoway_synonyms_control_ids
```

---


### Model

Creates a new model.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `periodic_tuning_state` | String |  | Optional. The state of periodic tuning. The period we use is 3 months - to do a one-off tune earlier use the `TuneModel` method. Default value is `PERIODIC_TUNING_ENABLED`. |
| `name` | String |  | Required. The fully qualified resource name of the model. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/models/{model_id}` catalog_id has char limit of 50. recommendation_model_id has char limit of 40. |
| `create_time` | String |  | Output only. Timestamp the Recommendation Model was created at. |
| `training_state` | String |  | Optional. The training state that the model is in (e.g. `TRAINING` or `PAUSED`). Since part of the cost of running the service is frequency of training - this can be used to determine when to train model in order to control cost. If not specified: the default value for `CreateModel` method is `TRAINING`. The default value for `UpdateModel` method is to keep the state the same as before. |
| `optimization_objective` | String |  | Optional. The optimization objective e.g. `cvr`. Currently supported values: `ctr`, `cvr`, `revenue-per-order`. If not specified, we choose default based on model type. Default depends on type of recommendation: `recommended-for-you` => `ctr` `others-you-may-like` => `ctr` `frequently-bought-together` => `revenue_per_order` This field together with optimization_objective describe model metadata to use to control model training and serving. See https://cloud.google.com/retail/docs/models for more details on what the model metadata control and which combination of parameters are valid. For invalid combinations of parameters (e.g. type = `frequently-bought-together` and optimization_objective = `ctr`), you receive an error 400 if you try to create/update a recommendation with this set of knobs. |
| `data_state` | String |  | Output only. The state of data requirements for this model: `DATA_OK` and `DATA_ERROR`. Recommendation model cannot be trained if the data is in `DATA_ERROR` state. Recommendation model can have `DATA_ERROR` state even if serving state is `ACTIVE`: models were trained successfully before, but cannot be refreshed because model no longer has sufficient data for training. |
| `model_features_config` | String |  | Optional. Additional model features config. |
| `serving_state` | String |  | Output only. The serving state of the model: `ACTIVE`, `NOT_ACTIVE`. |
| `display_name` | String |  | Required. The display name of the model. Should be human readable, used to display Recommendation Models in the Retail Cloud Console Dashboard. UTF-8 encoded string with limit of 1024 characters. |
| `update_time` | String |  | Output only. Timestamp the Recommendation Model was last updated. E.g. if a Recommendation Model was paused - this would be the time the pause was initiated. |
| `tuning_operation` | String |  | Output only. The tune operation associated with the model. Can be used to determine if there is an ongoing tune for this recommendation. Empty field implies no tune is goig on. |
| `last_tune_time` | String |  | Output only. The timestamp when the latest successful tune finished. |
| `serving_config_lists` | Vec<String> |  | Output only. The list of valid serving configs associated with the PageOptimizationConfig. |
| `type` | String |  | Required. The type of model e.g. `home-page`. Currently supported values: `recommended-for-you`, `others-you-may-like`, `frequently-bought-together`, `page-optimization`, `similar-items`, `buy-it-again`, `on-sale-items`, and `recently-viewed`(readonly value). This field together with optimization_objective describe model metadata to use to control model training and serving. See https://cloud.google.com/retail/docs/models for more details on what the model metadata control and which combination of parameters are valid. For invalid combinations of parameters (e.g. type = `frequently-bought-together` and optimization_objective = `ctr`), you receive an error 400 if you try to create/update a recommendation with this set of knobs. |
| `filtering_option` | String |  | Optional. If `RECOMMENDATIONS_FILTERING_ENABLED`, recommendation filtering by attributes is enabled for the model. |
| `parent` | String | ✅ | Required. The parent resource under which to create the model. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `periodic_tuning_state` | String | Optional. The state of periodic tuning. The period we use is 3 months - to do a one-off tune earlier use the `TuneModel` method. Default value is `PERIODIC_TUNING_ENABLED`. |
| `name` | String | Required. The fully qualified resource name of the model. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/models/{model_id}` catalog_id has char limit of 50. recommendation_model_id has char limit of 40. |
| `create_time` | String | Output only. Timestamp the Recommendation Model was created at. |
| `training_state` | String | Optional. The training state that the model is in (e.g. `TRAINING` or `PAUSED`). Since part of the cost of running the service is frequency of training - this can be used to determine when to train model in order to control cost. If not specified: the default value for `CreateModel` method is `TRAINING`. The default value for `UpdateModel` method is to keep the state the same as before. |
| `optimization_objective` | String | Optional. The optimization objective e.g. `cvr`. Currently supported values: `ctr`, `cvr`, `revenue-per-order`. If not specified, we choose default based on model type. Default depends on type of recommendation: `recommended-for-you` => `ctr` `others-you-may-like` => `ctr` `frequently-bought-together` => `revenue_per_order` This field together with optimization_objective describe model metadata to use to control model training and serving. See https://cloud.google.com/retail/docs/models for more details on what the model metadata control and which combination of parameters are valid. For invalid combinations of parameters (e.g. type = `frequently-bought-together` and optimization_objective = `ctr`), you receive an error 400 if you try to create/update a recommendation with this set of knobs. |
| `data_state` | String | Output only. The state of data requirements for this model: `DATA_OK` and `DATA_ERROR`. Recommendation model cannot be trained if the data is in `DATA_ERROR` state. Recommendation model can have `DATA_ERROR` state even if serving state is `ACTIVE`: models were trained successfully before, but cannot be refreshed because model no longer has sufficient data for training. |
| `model_features_config` | String | Optional. Additional model features config. |
| `serving_state` | String | Output only. The serving state of the model: `ACTIVE`, `NOT_ACTIVE`. |
| `display_name` | String | Required. The display name of the model. Should be human readable, used to display Recommendation Models in the Retail Cloud Console Dashboard. UTF-8 encoded string with limit of 1024 characters. |
| `update_time` | String | Output only. Timestamp the Recommendation Model was last updated. E.g. if a Recommendation Model was paused - this would be the time the pause was initiated. |
| `tuning_operation` | String | Output only. The tune operation associated with the model. Can be used to determine if there is an ongoing tune for this recommendation. Empty field implies no tune is goig on. |
| `last_tune_time` | String | Output only. The timestamp when the latest successful tune finished. |
| `serving_config_lists` | Vec<String> | Output only. The list of valid serving configs associated with the PageOptimizationConfig. |
| `type` | String | Required. The type of model e.g. `home-page`. Currently supported values: `recommended-for-you`, `others-you-may-like`, `frequently-bought-together`, `page-optimization`, `similar-items`, `buy-it-again`, `on-sale-items`, and `recently-viewed`(readonly value). This field together with optimization_objective describe model metadata to use to control model training and serving. See https://cloud.google.com/retail/docs/models for more details on what the model metadata control and which combination of parameters are valid. For invalid combinations of parameters (e.g. type = `frequently-bought-together` and optimization_objective = `ctr`), you receive an error 400 if you try to create/update a recommendation with this set of knobs. |
| `filtering_option` | String | Optional. If `RECOMMENDATIONS_FILTERING_ENABLED`, recommendation filtering by attributes is enabled for the model. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create model
model = provider.retail_api.Model {
    parent = "value"  # Required. The parent resource under which to create the model. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}`
}

# Access model outputs
model_id = model.id
model_periodic_tuning_state = model.periodic_tuning_state
model_name = model.name
model_create_time = model.create_time
model_training_state = model.training_state
model_optimization_objective = model.optimization_objective
model_data_state = model.data_state
model_model_features_config = model.model_features_config
model_serving_state = model.serving_state
model_display_name = model.display_name
model_update_time = model.update_time
model_tuning_operation = model.tuning_operation
model_last_tune_time = model.last_tune_time
model_serving_config_lists = model.serving_config_lists
model_type = model.type
model_filtering_option = model.filtering_option
```

---


### Catalog

Set a specified branch id as default branch. API methods such as SearchService.Search, ProductService.GetProduct, ProductService.ListProducts will treat requests using "default_branch" to the actual branch id set as default. For example, if `projects/*/locations/*/catalogs/*/branches/1` is set as default, setting SearchRequest.branch to `projects/*/locations/*/catalogs/*/branches/default_branch` is equivalent to setting SearchRequest.branch to `projects/*/locations/*/catalogs/*/branches/1`. Using multiple branches can be useful when developers would like to have a staging branch to test and verify for future usage. When it becomes ready, developers switch on the staging branch using this API while keeping using `projects/*/locations/*/catalogs/*/branches/default_branch` as SearchRequest.branch to route the traffic to this staging branch. CAUTION: If you have live predict/search traffic, switching the default branch could potentially cause outages if the ID space of the new branch is very different from the old one. More specifically: * PredictionService will only return product IDs from branch {newBranch}. * SearchService will only return product IDs from branch {newBranch} (if branch is not explicitly set). * UserEventService will only join events with products from branch {newBranch}.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `branch_id` | String |  | The final component of the resource name of a branch. This field must be one of "0", "1" or "2". Otherwise, an INVALID_ARGUMENT error is returned. If there are no sufficient active products in the targeted branch and force is not set, a FAILED_PRECONDITION error is returned. |
| `force` | bool |  | If set to true, it permits switching to a branch with branch_id even if it has no sufficient active products. |
| `note` | String |  | Some note on this request, this can be retrieved by CatalogService.GetDefaultBranch before next valid default branch set occurs. This field must be a UTF-8 encoded string with a length limit of 1,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `catalog` | String | ✅ | Full resource name of the catalog, such as `projects/*/locations/global/catalogs/default_catalog`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Required. Immutable. The fully qualified resource name of the attribute config. Format: `projects/*/locations/*/catalogs/*/attributesConfig` |
| `attribute_config_level` | String | Output only. The AttributeConfigLevel used for this catalog. |
| `catalog_attributes` | HashMap<String, String> | Enable attribute(s) config at catalog level. For example, indexable, dynamic_facetable, or searchable for each attribute. The key is catalog attribute's name. For example: `color`, `brands`, `attributes.custom_attribute`, such as `attributes.xyz`. The maximum number of catalog attributes allowed in a request is 1000. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create catalog
catalog = provider.retail_api.Catalog {
    catalog = "value"  # Full resource name of the catalog, such as `projects/*/locations/global/catalogs/default_catalog`.
}

# Access catalog outputs
catalog_id = catalog.id
catalog_name = catalog.name
catalog_attribute_config_level = catalog.attribute_config_level
catalog_catalog_attributes = catalog.catalog_attributes
```

---


### Product

Creates a Product.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | String |  | Immutable. The type of the product. Default to Catalog.product_level_config.ingestion_product_type if unset. |
| `gtin` | String |  | The Global Trade Item Number (GTIN) of the product. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. This field must be a Unigram. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [gtin](https://support.google.com/merchants/answer/6324461). Schema.org property [Product.isbn](https://schema.org/isbn), [Product.gtin8](https://schema.org/gtin8), [Product.gtin12](https://schema.org/gtin12), [Product.gtin13](https://schema.org/gtin13), or [Product.gtin14](https://schema.org/gtin14). If the value is not a valid GTIN, an INVALID_ARGUMENT error is returned. |
| `available_time` | String |  | The timestamp when this Product becomes available for SearchService.Search. Note that this is only applicable to Type.PRIMARY and Type.COLLECTION, and ignored for Type.VARIANT. |
| `images` | Vec<String> |  | Product images for the product. We highly recommend putting the main image first. A maximum of 300 images are allowed. Corresponding properties: Google Merchant Center property [image_link](https://support.google.com/merchants/answer/6324350). Schema.org property [Product.image](https://schema.org/image). |
| `local_inventories` | Vec<String> |  | Output only. A list of local inventories specific to different places. This field can be managed by ProductService.AddLocalInventories and ProductService.RemoveLocalInventories APIs if fine-grained, high-volume updates are necessary. |
| `rating` | String |  | The rating of this product. |
| `tags` | Vec<String> |  | Custom tags associated with the product. At most 250 values are allowed per Product. This value must be a UTF-8 encoded string with a length limit of 1,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. This tag can be used for filtering recommendation results by passing the tag as part of the PredictRequest.filter. Corresponding properties: Google Merchant Center property [custom_label_0–4](https://support.google.com/merchants/answer/6324473). |
| `sizes` | Vec<String> |  | The size of the product. To represent different size systems or size types, consider using this format: [[[size_system:]size_type:]size_value]. For example, in "US:MENS:M", "US" represents size system; "MENS" represents size type; "M" represents size value. In "GIRLS:27", size system is empty; "GIRLS" represents size type; "27" represents size value. In "32 inches", both size system and size type are empty, while size value is "32 inches". A maximum of 20 values are allowed per Product. Each value must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [size](https://support.google.com/merchants/answer/6324492), [size_type](https://support.google.com/merchants/answer/6324497), and [size_system](https://support.google.com/merchants/answer/6324502). Schema.org property [Product.size](https://schema.org/size). |
| `collection_member_ids` | Vec<String> |  | The id of the collection members when type is Type.COLLECTION. Non-existent product ids are allowed. The type of the members must be either Type.PRIMARY or Type.VARIANT otherwise an INVALID_ARGUMENT error is thrown. Should not set it for other types. A maximum of 1000 values are allowed. Otherwise, an INVALID_ARGUMENT error is return. |
| `patterns` | Vec<String> |  | The pattern or graphic print of the product. For example, "striped", "polka dot", "paisley". A maximum of 20 values are allowed per Product. Each value must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [pattern](https://support.google.com/merchants/answer/6324483). Schema.org property [Product.pattern](https://schema.org/pattern). |
| `primary_product_id` | String |  | Variant group identifier. Must be an id, with the same parent branch with this product. Otherwise, an error is thrown. For Type.PRIMARY Products, this field can only be empty or set to the same value as id. For VARIANT Products, this field cannot be empty. A maximum of 2,000 products are allowed to share the same Type.PRIMARY Product. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [item_group_id](https://support.google.com/merchants/answer/6324507). Schema.org property [Product.inProductGroupWithID](https://schema.org/inProductGroupWithID). |
| `ttl` | String |  | Input only. The TTL (time to live) of the product. Note that this is only applicable to Type.PRIMARY and Type.COLLECTION, and ignored for Type.VARIANT. In general, we suggest the users to delete the stale products explicitly, instead of using this field to determine staleness. If it is set, it must be a non-negative value, and expire_time is set as current timestamp plus ttl. The derived expire_time is returned in the output and ttl is left blank when retrieving the Product. If it is set, the product is not available for SearchService.Search after current timestamp plus ttl. However, the product can still be retrieved by ProductService.GetProduct and ProductService.ListProducts. |
| `retrievable_fields` | String |  | Indicates which fields in the Products are returned in SearchResponse. Supported fields for all types: * audience * availability * brands * color_info * conditions * gtin * materials * name * patterns * price_info * rating * sizes * title * uri Supported fields only for Type.PRIMARY and Type.COLLECTION: * categories * description * images Supported fields only for Type.VARIANT: * Only the first image in images To mark attributes as retrievable, include paths of the form "attributes.key" where "key" is the key of a custom attribute, as specified in attributes. For Type.PRIMARY and Type.COLLECTION, the following fields are always returned in SearchResponse by default: * name For Type.VARIANT, the following fields are always returned in by default: * name * color_info Note: Returning more fields in SearchResponse can increase response payload size and serving latency. This field is deprecated. Use the retrievable site-wide control instead. |
| `color_info` | String |  | The color of the product. Corresponding properties: Google Merchant Center property [color](https://support.google.com/merchants/answer/6324487). Schema.org property [Product.color](https://schema.org/color). |
| `available_quantity` | i64 |  | The available quantity of the item. |
| `categories` | Vec<String> |  | Product categories. This field is repeated for supporting one product belonging to several parallel categories. Strongly recommended using the full path for better search / recommendation quality. To represent full path of category, use '>' sign to separate different hierarchies. If '>' is part of the category name, replace it with other character(s). For example, if a shoes product belongs to both ["Shoes & Accessories" -> "Shoes"] and ["Sports & Fitness" -> "Athletic Clothing" -> "Shoes"], it could be represented as: "categories": [ "Shoes & Accessories > Shoes", "Sports & Fitness > Athletic Clothing > Shoes" ] Must be set for Type.PRIMARY Product otherwise an INVALID_ARGUMENT error is returned. At most 250 values are allowed per Product unless overridden through the Google Cloud console. Empty values are not allowed. Each value must be a UTF-8 encoded string with a length limit of 5,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property google_product_category. Schema.org property [Product.category] (https://schema.org/category). [mc_google_product_category]: https://support.google.com/merchants/answer/6324436 |
| `price_info` | String |  | Product price and cost information. Corresponding properties: Google Merchant Center property [price](https://support.google.com/merchants/answer/6324371). |
| `uri` | String |  | Canonical URL directly linking to the product detail page. It is strongly recommended to provide a valid uri for the product, otherwise the service performance could be significantly degraded. This field must be a UTF-8 encoded string with a length limit of 5,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [link](https://support.google.com/merchants/answer/6324416). Schema.org property [Offer.url](https://schema.org/url). |
| `expire_time` | String |  | Note that this field is applied in the following ways: * If the Product is already expired when it is uploaded, this product is not indexed for search. * If the Product is not expired when it is uploaded, only the Type.PRIMARY's and Type.COLLECTION's expireTime is respected, and Type.VARIANT's expireTime is not used. In general, we suggest the users to delete the stale products explicitly, instead of using this field to determine staleness. expire_time must be later than available_time and publish_time, otherwise an INVALID_ARGUMENT error is thrown. Corresponding properties: Google Merchant Center property [expiration_date](https://support.google.com/merchants/answer/6324499). |
| `publish_time` | String |  | The timestamp when the product is published by the retailer for the first time, which indicates the freshness of the products. Note that this field is different from available_time, given it purely describes product freshness regardless of when it is available on search and recommendation. |
| `description` | String |  | Product description. This field must be a UTF-8 encoded string with a length limit of 5,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [description](https://support.google.com/merchants/answer/6324468). Schema.org property [Product.description](https://schema.org/description). |
| `attributes` | HashMap<String, String> |  | Highly encouraged. Extra product attributes to be included. For example, for products, this could include the store name, vendor, style, color, etc. These are very strong signals for recommendation model, thus we highly recommend providing the attributes here. Features that can take on one of a limited number of possible values. Two types of features can be set are: Textual features. some examples would be the brand/maker of a product, or country of a customer. Numerical features. Some examples would be the height/weight of a product, or age of a customer. For example: `{ "vendor": {"text": ["vendor123", "vendor456"]}, "lengths_cm": {"numbers":[2.3, 15.4]}, "heights_cm": {"numbers":[8.1, 6.4]} }`. This field needs to pass all below criteria, otherwise an INVALID_ARGUMENT error is returned: * Max entries count: 200. * The key must be a UTF-8 encoded string with a length limit of 128 characters. * For indexable attribute, the key must match the pattern: `a-zA-Z0-9*`. For example, `key0LikeThis` or `KEY_1_LIKE_THIS`. * For text attributes, at most 400 values are allowed. Empty values are not allowed. Each value must be a non-empty UTF-8 encoded string with a length limit of 256 characters. * For number attributes, at most 400 values are allowed. |
| `audience` | String |  | The target group associated with a given audience (e.g. male, veterans, car owners, musicians, etc.) of the product. |
| `conditions` | Vec<String> |  | The condition of the product. Strongly encouraged to use the standard values: "new", "refurbished", "used". A maximum of 1 value is allowed per Product. Each value must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [condition](https://support.google.com/merchants/answer/6324469). Schema.org property [Offer.itemCondition](https://schema.org/itemCondition). |
| `brands` | Vec<String> |  | The brands of the product. A maximum of 30 brands are allowed unless overridden through the Google Cloud console. Each brand must be a UTF-8 encoded string with a length limit of 1,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [brand](https://support.google.com/merchants/answer/6324351). Schema.org property [Product.brand](https://schema.org/brand). |
| `fulfillment_info` | Vec<String> |  | Fulfillment information, such as the store IDs for in-store pickup or region IDs for different shipping methods. All the elements must have distinct FulfillmentInfo.type. Otherwise, an INVALID_ARGUMENT error is returned. |
| `language_code` | String |  | Language of the title/description and other string attributes. Use language tags defined by [BCP 47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt). For product prediction, this field is ignored and the model automatically detects the text language. The Product can include text in different languages, but duplicating Products to provide text in multiple languages can result in degraded model performance. For product search this field is in use. It defaults to "en-US" if unset. |
| `materials` | Vec<String> |  | The material of the product. For example, "leather", "wooden". A maximum of 20 values are allowed. Each value must be a UTF-8 encoded string with a length limit of 200 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [material](https://support.google.com/merchants/answer/6324410). Schema.org property [Product.material](https://schema.org/material). |
| `id` | String |  | Immutable. Product identifier, which is the final component of name. For example, this field is "id_1", if name is `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/id_1`. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [id](https://support.google.com/merchants/answer/6324405). Schema.org property [Product.sku](https://schema.org/sku). |
| `promotions` | Vec<String> |  | The promotions applied to the product. A maximum of 10 values are allowed per Product. Only Promotion.promotion_id will be used, other fields will be ignored if set. |
| `title` | String |  | Required. Product title. This field must be a UTF-8 encoded string with a length limit of 1,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [title](https://support.google.com/merchants/answer/6324415). Schema.org property [Product.name](https://schema.org/name). |
| `availability` | String |  | The online availability of the Product. Default to Availability.IN_STOCK. For primary products with variants set the availability of the primary as Availability.OUT_OF_STOCK and set the true availability at the variant level. This way the primary product will be considered "in stock" as long as it has at least one variant in stock. For primary products with no variants set the true availability at the primary level. Corresponding properties: Google Merchant Center property [availability](https://support.google.com/merchants/answer/6324448). Schema.org property [Offer.availability](https://schema.org/availability). |
| `variants` | Vec<String> |  | Output only. Product variants grouped together on primary product which share similar product attributes. It's automatically grouped by primary_product_id for all the product variants. Only populated for Type.PRIMARY Products. Note: This field is OUTPUT_ONLY for ProductService.GetProduct. Do not set this field in API requests. |
| `name` | String |  | Immutable. Full resource name of the product, such as `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/product_id`. |
| `parent` | String | ✅ | Required. The parent catalog resource name, such as `projects/*/locations/global/catalogs/default_catalog/branches/default_branch`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type` | String | Immutable. The type of the product. Default to Catalog.product_level_config.ingestion_product_type if unset. |
| `gtin` | String | The Global Trade Item Number (GTIN) of the product. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. This field must be a Unigram. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [gtin](https://support.google.com/merchants/answer/6324461). Schema.org property [Product.isbn](https://schema.org/isbn), [Product.gtin8](https://schema.org/gtin8), [Product.gtin12](https://schema.org/gtin12), [Product.gtin13](https://schema.org/gtin13), or [Product.gtin14](https://schema.org/gtin14). If the value is not a valid GTIN, an INVALID_ARGUMENT error is returned. |
| `available_time` | String | The timestamp when this Product becomes available for SearchService.Search. Note that this is only applicable to Type.PRIMARY and Type.COLLECTION, and ignored for Type.VARIANT. |
| `images` | Vec<String> | Product images for the product. We highly recommend putting the main image first. A maximum of 300 images are allowed. Corresponding properties: Google Merchant Center property [image_link](https://support.google.com/merchants/answer/6324350). Schema.org property [Product.image](https://schema.org/image). |
| `local_inventories` | Vec<String> | Output only. A list of local inventories specific to different places. This field can be managed by ProductService.AddLocalInventories and ProductService.RemoveLocalInventories APIs if fine-grained, high-volume updates are necessary. |
| `rating` | String | The rating of this product. |
| `tags` | Vec<String> | Custom tags associated with the product. At most 250 values are allowed per Product. This value must be a UTF-8 encoded string with a length limit of 1,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. This tag can be used for filtering recommendation results by passing the tag as part of the PredictRequest.filter. Corresponding properties: Google Merchant Center property [custom_label_0–4](https://support.google.com/merchants/answer/6324473). |
| `sizes` | Vec<String> | The size of the product. To represent different size systems or size types, consider using this format: [[[size_system:]size_type:]size_value]. For example, in "US:MENS:M", "US" represents size system; "MENS" represents size type; "M" represents size value. In "GIRLS:27", size system is empty; "GIRLS" represents size type; "27" represents size value. In "32 inches", both size system and size type are empty, while size value is "32 inches". A maximum of 20 values are allowed per Product. Each value must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [size](https://support.google.com/merchants/answer/6324492), [size_type](https://support.google.com/merchants/answer/6324497), and [size_system](https://support.google.com/merchants/answer/6324502). Schema.org property [Product.size](https://schema.org/size). |
| `collection_member_ids` | Vec<String> | The id of the collection members when type is Type.COLLECTION. Non-existent product ids are allowed. The type of the members must be either Type.PRIMARY or Type.VARIANT otherwise an INVALID_ARGUMENT error is thrown. Should not set it for other types. A maximum of 1000 values are allowed. Otherwise, an INVALID_ARGUMENT error is return. |
| `patterns` | Vec<String> | The pattern or graphic print of the product. For example, "striped", "polka dot", "paisley". A maximum of 20 values are allowed per Product. Each value must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [pattern](https://support.google.com/merchants/answer/6324483). Schema.org property [Product.pattern](https://schema.org/pattern). |
| `primary_product_id` | String | Variant group identifier. Must be an id, with the same parent branch with this product. Otherwise, an error is thrown. For Type.PRIMARY Products, this field can only be empty or set to the same value as id. For VARIANT Products, this field cannot be empty. A maximum of 2,000 products are allowed to share the same Type.PRIMARY Product. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [item_group_id](https://support.google.com/merchants/answer/6324507). Schema.org property [Product.inProductGroupWithID](https://schema.org/inProductGroupWithID). |
| `ttl` | String | Input only. The TTL (time to live) of the product. Note that this is only applicable to Type.PRIMARY and Type.COLLECTION, and ignored for Type.VARIANT. In general, we suggest the users to delete the stale products explicitly, instead of using this field to determine staleness. If it is set, it must be a non-negative value, and expire_time is set as current timestamp plus ttl. The derived expire_time is returned in the output and ttl is left blank when retrieving the Product. If it is set, the product is not available for SearchService.Search after current timestamp plus ttl. However, the product can still be retrieved by ProductService.GetProduct and ProductService.ListProducts. |
| `retrievable_fields` | String | Indicates which fields in the Products are returned in SearchResponse. Supported fields for all types: * audience * availability * brands * color_info * conditions * gtin * materials * name * patterns * price_info * rating * sizes * title * uri Supported fields only for Type.PRIMARY and Type.COLLECTION: * categories * description * images Supported fields only for Type.VARIANT: * Only the first image in images To mark attributes as retrievable, include paths of the form "attributes.key" where "key" is the key of a custom attribute, as specified in attributes. For Type.PRIMARY and Type.COLLECTION, the following fields are always returned in SearchResponse by default: * name For Type.VARIANT, the following fields are always returned in by default: * name * color_info Note: Returning more fields in SearchResponse can increase response payload size and serving latency. This field is deprecated. Use the retrievable site-wide control instead. |
| `color_info` | String | The color of the product. Corresponding properties: Google Merchant Center property [color](https://support.google.com/merchants/answer/6324487). Schema.org property [Product.color](https://schema.org/color). |
| `available_quantity` | i64 | The available quantity of the item. |
| `categories` | Vec<String> | Product categories. This field is repeated for supporting one product belonging to several parallel categories. Strongly recommended using the full path for better search / recommendation quality. To represent full path of category, use '>' sign to separate different hierarchies. If '>' is part of the category name, replace it with other character(s). For example, if a shoes product belongs to both ["Shoes & Accessories" -> "Shoes"] and ["Sports & Fitness" -> "Athletic Clothing" -> "Shoes"], it could be represented as: "categories": [ "Shoes & Accessories > Shoes", "Sports & Fitness > Athletic Clothing > Shoes" ] Must be set for Type.PRIMARY Product otherwise an INVALID_ARGUMENT error is returned. At most 250 values are allowed per Product unless overridden through the Google Cloud console. Empty values are not allowed. Each value must be a UTF-8 encoded string with a length limit of 5,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property google_product_category. Schema.org property [Product.category] (https://schema.org/category). [mc_google_product_category]: https://support.google.com/merchants/answer/6324436 |
| `price_info` | String | Product price and cost information. Corresponding properties: Google Merchant Center property [price](https://support.google.com/merchants/answer/6324371). |
| `uri` | String | Canonical URL directly linking to the product detail page. It is strongly recommended to provide a valid uri for the product, otherwise the service performance could be significantly degraded. This field must be a UTF-8 encoded string with a length limit of 5,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [link](https://support.google.com/merchants/answer/6324416). Schema.org property [Offer.url](https://schema.org/url). |
| `expire_time` | String | Note that this field is applied in the following ways: * If the Product is already expired when it is uploaded, this product is not indexed for search. * If the Product is not expired when it is uploaded, only the Type.PRIMARY's and Type.COLLECTION's expireTime is respected, and Type.VARIANT's expireTime is not used. In general, we suggest the users to delete the stale products explicitly, instead of using this field to determine staleness. expire_time must be later than available_time and publish_time, otherwise an INVALID_ARGUMENT error is thrown. Corresponding properties: Google Merchant Center property [expiration_date](https://support.google.com/merchants/answer/6324499). |
| `publish_time` | String | The timestamp when the product is published by the retailer for the first time, which indicates the freshness of the products. Note that this field is different from available_time, given it purely describes product freshness regardless of when it is available on search and recommendation. |
| `description` | String | Product description. This field must be a UTF-8 encoded string with a length limit of 5,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [description](https://support.google.com/merchants/answer/6324468). Schema.org property [Product.description](https://schema.org/description). |
| `attributes` | HashMap<String, String> | Highly encouraged. Extra product attributes to be included. For example, for products, this could include the store name, vendor, style, color, etc. These are very strong signals for recommendation model, thus we highly recommend providing the attributes here. Features that can take on one of a limited number of possible values. Two types of features can be set are: Textual features. some examples would be the brand/maker of a product, or country of a customer. Numerical features. Some examples would be the height/weight of a product, or age of a customer. For example: `{ "vendor": {"text": ["vendor123", "vendor456"]}, "lengths_cm": {"numbers":[2.3, 15.4]}, "heights_cm": {"numbers":[8.1, 6.4]} }`. This field needs to pass all below criteria, otherwise an INVALID_ARGUMENT error is returned: * Max entries count: 200. * The key must be a UTF-8 encoded string with a length limit of 128 characters. * For indexable attribute, the key must match the pattern: `a-zA-Z0-9*`. For example, `key0LikeThis` or `KEY_1_LIKE_THIS`. * For text attributes, at most 400 values are allowed. Empty values are not allowed. Each value must be a non-empty UTF-8 encoded string with a length limit of 256 characters. * For number attributes, at most 400 values are allowed. |
| `audience` | String | The target group associated with a given audience (e.g. male, veterans, car owners, musicians, etc.) of the product. |
| `conditions` | Vec<String> | The condition of the product. Strongly encouraged to use the standard values: "new", "refurbished", "used". A maximum of 1 value is allowed per Product. Each value must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [condition](https://support.google.com/merchants/answer/6324469). Schema.org property [Offer.itemCondition](https://schema.org/itemCondition). |
| `brands` | Vec<String> | The brands of the product. A maximum of 30 brands are allowed unless overridden through the Google Cloud console. Each brand must be a UTF-8 encoded string with a length limit of 1,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [brand](https://support.google.com/merchants/answer/6324351). Schema.org property [Product.brand](https://schema.org/brand). |
| `fulfillment_info` | Vec<String> | Fulfillment information, such as the store IDs for in-store pickup or region IDs for different shipping methods. All the elements must have distinct FulfillmentInfo.type. Otherwise, an INVALID_ARGUMENT error is returned. |
| `language_code` | String | Language of the title/description and other string attributes. Use language tags defined by [BCP 47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt). For product prediction, this field is ignored and the model automatically detects the text language. The Product can include text in different languages, but duplicating Products to provide text in multiple languages can result in degraded model performance. For product search this field is in use. It defaults to "en-US" if unset. |
| `materials` | Vec<String> | The material of the product. For example, "leather", "wooden". A maximum of 20 values are allowed. Each value must be a UTF-8 encoded string with a length limit of 200 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [material](https://support.google.com/merchants/answer/6324410). Schema.org property [Product.material](https://schema.org/material). |
| `id` | String | Immutable. Product identifier, which is the final component of name. For example, this field is "id_1", if name is `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/id_1`. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [id](https://support.google.com/merchants/answer/6324405). Schema.org property [Product.sku](https://schema.org/sku). |
| `promotions` | Vec<String> | The promotions applied to the product. A maximum of 10 values are allowed per Product. Only Promotion.promotion_id will be used, other fields will be ignored if set. |
| `title` | String | Required. Product title. This field must be a UTF-8 encoded string with a length limit of 1,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [title](https://support.google.com/merchants/answer/6324415). Schema.org property [Product.name](https://schema.org/name). |
| `availability` | String | The online availability of the Product. Default to Availability.IN_STOCK. For primary products with variants set the availability of the primary as Availability.OUT_OF_STOCK and set the true availability at the variant level. This way the primary product will be considered "in stock" as long as it has at least one variant in stock. For primary products with no variants set the true availability at the primary level. Corresponding properties: Google Merchant Center property [availability](https://support.google.com/merchants/answer/6324448). Schema.org property [Offer.availability](https://schema.org/availability). |
| `variants` | Vec<String> | Output only. Product variants grouped together on primary product which share similar product attributes. It's automatically grouped by primary_product_id for all the product variants. Only populated for Type.PRIMARY Products. Note: This field is OUTPUT_ONLY for ProductService.GetProduct. Do not set this field in API requests. |
| `name` | String | Immutable. Full resource name of the product, such as `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/product_id`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create product
product = provider.retail_api.Product {
    parent = "value"  # Required. The parent catalog resource name, such as `projects/*/locations/global/catalogs/default_catalog/branches/default_branch`.
}

# Access product outputs
product_id = product.id
product_type = product.type
product_gtin = product.gtin
product_available_time = product.available_time
product_images = product.images
product_local_inventories = product.local_inventories
product_rating = product.rating
product_tags = product.tags
product_sizes = product.sizes
product_collection_member_ids = product.collection_member_ids
product_patterns = product.patterns
product_primary_product_id = product.primary_product_id
product_ttl = product.ttl
product_retrievable_fields = product.retrievable_fields
product_color_info = product.color_info
product_available_quantity = product.available_quantity
product_categories = product.categories
product_price_info = product.price_info
product_uri = product.uri
product_expire_time = product.expire_time
product_publish_time = product.publish_time
product_description = product.description
product_attributes = product.attributes
product_audience = product.audience
product_conditions = product.conditions
product_brands = product.brands
product_fulfillment_info = product.fulfillment_info
product_language_code = product.language_code
product_materials = product.materials
product_id = product.id
product_promotions = product.promotions
product_title = product.title
product_availability = product.availability
product_variants = product.variants
product_name = product.name
```

---


### Placement

Makes a recommendation prediction.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `filter` | String |  | Filter for restricting prediction results with a length limit of 5,000 characters. Accepts values for tags and the `filterOutOfStockItems` flag. * Tag expressions. Restricts predictions to products that match all of the specified tags. Boolean operators `OR` and `NOT` are supported if the expression is enclosed in parentheses, and must be separated from the tag values by a space. `-"tagA"` is also supported and is equivalent to `NOT "tagA"`. Tag values must be double quoted UTF-8 encoded strings with a size limit of 1,000 characters. Note: "Recently viewed" models don't support tag filtering at the moment. * filterOutOfStockItems. Restricts predictions to products that do not have a stockState value of OUT_OF_STOCK. Examples: * tag=("Red" OR "Blue") tag="New-Arrival" tag=(NOT "promotional") * filterOutOfStockItems tag=(-"promotional") * filterOutOfStockItems If your filter blocks all prediction results, the API will return *no* results. If instead you want empty result sets to return generic (unfiltered) popular products, set `strictFiltering` to False in `PredictRequest.params`. Note that the API will never return items with storageStatus of "EXPIRED" or "DELETED" regardless of filter choices. If `filterSyntaxV2` is set to true under the `params` field, then attribute-based expressions are expected instead of the above described tag-based syntax. Examples: * (colors: ANY("Red", "Blue")) AND NOT (categories: ANY("Phones")) * (availability: ANY("IN_STOCK")) AND (colors: ANY("Red") OR categories: ANY("Phones")) For more information, see [Filter recommendations](https://cloud.google.com/retail/docs/filter-recs). |
| `page_token` | String |  | This field is not used; leave it unset. |
| `params` | HashMap<String, String> |  | Additional domain specific parameters for the predictions. Allowed values: * `returnProduct`: Boolean. If set to true, the associated product object will be returned in the `results.metadata` field in the prediction response. * `returnScore`: Boolean. If set to true, the prediction 'score' corresponding to each returned product will be set in the `results.metadata` field in the prediction response. The given 'score' indicates the probability of a product being clicked/purchased given the user's context and history. * `strictFiltering`: Boolean. True by default. If set to false, the service will return generic (unfiltered) popular products instead of empty if your filter blocks all prediction results. * `priceRerankLevel`: String. Default empty. If set to be non-empty, then it needs to be one of {'no-price-reranking', 'low-price-reranking', 'medium-price-reranking', 'high-price-reranking'}. This gives request-level control and adjusts prediction results based on product price. * `diversityLevel`: String. Default empty. If set to be non-empty, then it needs to be one of {'no-diversity', 'low-diversity', 'medium-diversity', 'high-diversity', 'auto-diversity'}. This gives request-level control and adjusts prediction results based on product category. * `filterSyntaxV2`: Boolean. False by default. If set to true, the `filter` field is interpreteted according to the new, attribute-based syntax. |
| `user_event` | String |  | Required. Context about the user, what they are looking at and what action they took to trigger the predict request. Note that this user event detail won't be ingested to userEvent logs. Thus, a separate userEvent write request is required for event logging. Don't set UserEvent.visitor_id or UserInfo.user_id to the same fixed ID for different users. If you are trying to receive non-personalized recommendations (not recommended; this can negatively impact model performance), instead set UserEvent.visitor_id to a random unique ID and leave UserInfo.user_id unset. |
| `validate_only` | bool |  | Use validate only mode for this prediction query. If set to true, a dummy model will be used that returns arbitrary products. Note that the validate only mode should only be used for testing the API, or if the model is not ready. |
| `page_size` | i64 |  | Maximum number of results to return. Set this property to the number of prediction results needed. If zero, the service will choose a reasonable default. The maximum allowed value is 100. Values above 100 will be coerced to 100. |
| `labels` | HashMap<String, String> |  | The labels applied to a resource must meet the following requirements: * Each resource can have multiple labels, up to a maximum of 64. * Each label must be a key-value pair. * Keys have a minimum length of 1 character and a maximum length of 63 characters and cannot be empty. Values can be empty and have a maximum length of 63 characters. * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. All characters must use UTF-8 encoding, and international characters are allowed. * The key portion of a label must be unique. However, you can use the same key with multiple resources. * Keys must start with a lowercase letter or international character. See [Google Cloud Document](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements) for more details. |
| `placement` | String | ✅ | Required. Full resource name of the format: `{placement=projects/*/locations/global/catalogs/default_catalog/servingConfigs/*}` or `{placement=projects/*/locations/global/catalogs/default_catalog/placements/*}`. We recommend using the `servingConfigs` resource. `placements` is a legacy resource. The ID of the Recommendations AI serving config or placement. Before you can request predictions from your model, you must create at least one serving config or placement for it. For more information, see [Manage serving configs] (https://cloud.google.com/retail/docs/manage-configs). The full list of available serving configs can be seen at https://console.cloud.google.com/ai/retail/catalogs/default_catalog/configs |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create placement
placement = provider.retail_api.Placement {
    placement = "value"  # Required. Full resource name of the format: `{placement=projects/*/locations/global/catalogs/default_catalog/servingConfigs/*}` or `{placement=projects/*/locations/global/catalogs/default_catalog/placements/*}`. We recommend using the `servingConfigs` resource. `placements` is a legacy resource. The ID of the Recommendations AI serving config or placement. Before you can request predictions from your model, you must create at least one serving config or placement for it. For more information, see [Manage serving configs] (https://cloud.google.com/retail/docs/manage-configs). The full list of available serving configs can be seen at https://console.cloud.google.com/ai/retail/catalogs/default_catalog/configs
}

```

---


### Project

Get the AlertConfig of the requested project.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Required. Immutable. The name of the AlertConfig singleton resource. Format: projects/*/alertConfig |
| `alert_policies` | Vec<String> |  | Alert policies for a customer. They must be unique by [AlertPolicy.alert_group] |
| `name` | String | ✅ | Required. Immutable. The name of the AlertConfig singleton resource. Format: projects/*/alertConfig |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Required. Immutable. The name of the AlertConfig singleton resource. Format: projects/*/alertConfig |
| `alert_policies` | Vec<String> | Alert policies for a customer. They must be unique by [AlertPolicy.alert_group] |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access project outputs
project_id = project.id
project_name = project.name
project_alert_policies = project.alert_policies
```

---


### Operation

Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access operation outputs
operation_id = operation.id
operation_error = operation.error
operation_metadata = operation.metadata
operation_name = operation.name
operation_response = operation.response
operation_done = operation.done
```

---


### Completion_data

Bulk import of processed completion dataset. Request processing is asynchronous. Partial updating is not supported. The operation is successfully finished only after the imported suggestions are indexed successfully and ready for serving. The process takes hours. This feature is only available for users who have Retail Search enabled. Enable Retail Search on Cloud Console before using this feature.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `notification_pubsub_topic` | String |  | Pub/Sub topic for receiving notification. If this field is set, when the import is finished, a notification is sent to specified Pub/Sub topic. The message data is JSON string of a Operation. Format of the Pub/Sub topic is `projects/{project}/topics/{topic}`. |
| `input_config` | String |  | Required. The desired input location of the data. |
| `parent` | String | ✅ | Required. The catalog which the suggestions dataset belongs to. Format: `projects/1234/locations/global/catalogs/default_catalog`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create completion_data
completion_data = provider.retail_api.Completion_data {
    parent = "value"  # Required. The catalog which the suggestions dataset belongs to. Format: `projects/1234/locations/global/catalogs/default_catalog`.
}

```

---


### Generative_question

Allows management of multiple questions.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `requests` | Vec<String> |  | Required. The updates question configs. |
| `parent` | String | ✅ | Optional. Resource name of the parent catalog. Format: projects/{project}/locations/{location}/catalogs/{catalog} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `generative_question_configs` | Vec<String> | All the questions for a given catalog. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create generative_question
generative_question = provider.retail_api.Generative_question {
    parent = "value"  # Optional. Resource name of the parent catalog. Format: projects/{project}/locations/{location}/catalogs/{catalog}
}

# Access generative_question outputs
generative_question_id = generative_question.id
generative_question_generative_question_configs = generative_question.generative_question_configs
```

---


### Serving_config

Creates a ServingConfig. A maximum of 100 ServingConfigs are allowed in a Catalog, otherwise a FAILED_PRECONDITION error is returned.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `diversity_type` | String |  | What kind of diversity to use - data driven or rule based. If unset, the server behavior defaults to RULE_BASED_DIVERSITY. |
| `filter_control_ids` | Vec<String> |  | Condition filter specifications. If a product matches multiple conditions in the specifications, filters from these specifications are all applied and combined via the AND operator. Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `ignore_control_ids` | Vec<String> |  | Condition ignore specifications. If multiple ignore conditions match, all matching ignore controls in the list will execute. - Order does not matter. - Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `solution_types` | Vec<String> |  | Required. Immutable. Specifies the solution types that a serving config can be associated with. Currently we support setting only one type of solution. |
| `do_not_associate_control_ids` | Vec<String> |  | Condition do not associate specifications. If multiple do not associate conditions match, all matching do not associate controls in the list will execute. - Order does not matter. - Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `diversity_level` | String |  | How much diversity to use in recommendation model results e.g. `medium-diversity` or `high-diversity`. Currently supported values: * `no-diversity` * `low-diversity` * `medium-diversity` * `high-diversity` * `auto-diversity` If not specified, we choose default based on recommendation model type. Default value: `no-diversity`. Can only be set if solution_types is SOLUTION_TYPE_RECOMMENDATION. |
| `replacement_control_ids` | Vec<String> |  | Condition replacement specifications. - Applied according to the order in the list. - A previously replaced term can not be re-replaced. - Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `price_reranking_level` | String |  | How much price ranking we want in serving results. Price reranking causes product items with a similar recommendation probability to be ordered by price, with the highest-priced items first. This setting could result in a decrease in click-through and conversion rates. Allowed values are: * `no-price-reranking` * `low-price-reranking` * `medium-price-reranking` * `high-price-reranking` If not specified, we choose default based on model type. Default value: `no-price-reranking`. Can only be set if solution_types is SOLUTION_TYPE_RECOMMENDATION. |
| `dynamic_facet_spec` | String |  | The specification for dynamically generated facets. Notice that only textual facets can be dynamically generated. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `enable_category_filter_level` | String |  | Whether to add additional category filters on the `similar-items` model. If not specified, we enable it by default. Allowed values are: * `no-category-match`: No additional filtering of original results from the model and the customer's filters. * `relaxed-category-match`: Only keep results with categories that match at least one item categories in the PredictRequests's context item. * If customer also sends filters in the PredictRequest, then the results will satisfy both conditions (user given and category match). Can only be set if solution_types is SOLUTION_TYPE_RECOMMENDATION. |
| `boost_control_ids` | Vec<String> |  | Condition boost specifications. If a product matches multiple conditions in the specifications, boost scores from these specifications are all applied and combined in a non-linear way. Maximum number of specifications is 100. Notice that if both ServingConfig.boost_control_ids and SearchRequest.boost_spec are set, the boost conditions from both places are evaluated. If a search request matches multiple boost conditions, the final boost score is equal to the sum of the boost scores from all matched boost conditions. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `ignore_recs_denylist` | bool |  | When the flag is enabled, the products in the denylist will not be filtered out in the recommendation filtering results. |
| `display_name` | String |  | Required. The human readable serving config display name. Used in Retail UI. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `model_id` | String |  | The id of the model in the same Catalog to use at serving time. Currently only RecommendationModels are supported: https://cloud.google.com/retail/recommendations-ai/docs/create-models Can be changed but only to a compatible model (e.g. others-you-may-like CTR to others-you-may-like CVR). Required when solution_types is SOLUTION_TYPE_RECOMMENDATION. |
| `oneway_synonyms_control_ids` | Vec<String> |  | Condition oneway synonyms specifications. If multiple oneway synonyms conditions match, all matching oneway synonyms controls in the list will execute. Order of controls in the list will not matter. Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `personalization_spec` | String |  | The specification for personalization spec. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. Notice that if both ServingConfig.personalization_spec and SearchRequest.personalization_spec are set. SearchRequest.personalization_spec will override ServingConfig.personalization_spec. |
| `name` | String |  | Immutable. Fully qualified name `projects/*/locations/global/catalogs/*/servingConfig/*` |
| `redirect_control_ids` | Vec<String> |  | Condition redirect specifications. Only the first triggered redirect action is applied, even if multiple apply. Maximum number of specifications is 1000. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `twoway_synonyms_control_ids` | Vec<String> |  | Condition synonyms specifications. If multiple syonyms conditions match, all matching synonyms control in the list will execute. Order of controls in the list will not matter. Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `facet_control_ids` | Vec<String> |  | Facet specifications for faceted search. If empty, no facets are returned. The ids refer to the ids of Control resources with only the Facet control set. These controls are assumed to be in the same Catalog as the ServingConfig. A maximum of 100 values are allowed. Otherwise, an INVALID_ARGUMENT error is returned. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `parent` | String | ✅ | Required. Full resource name of parent. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `diversity_type` | String | What kind of diversity to use - data driven or rule based. If unset, the server behavior defaults to RULE_BASED_DIVERSITY. |
| `filter_control_ids` | Vec<String> | Condition filter specifications. If a product matches multiple conditions in the specifications, filters from these specifications are all applied and combined via the AND operator. Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `ignore_control_ids` | Vec<String> | Condition ignore specifications. If multiple ignore conditions match, all matching ignore controls in the list will execute. - Order does not matter. - Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `solution_types` | Vec<String> | Required. Immutable. Specifies the solution types that a serving config can be associated with. Currently we support setting only one type of solution. |
| `do_not_associate_control_ids` | Vec<String> | Condition do not associate specifications. If multiple do not associate conditions match, all matching do not associate controls in the list will execute. - Order does not matter. - Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `diversity_level` | String | How much diversity to use in recommendation model results e.g. `medium-diversity` or `high-diversity`. Currently supported values: * `no-diversity` * `low-diversity` * `medium-diversity` * `high-diversity` * `auto-diversity` If not specified, we choose default based on recommendation model type. Default value: `no-diversity`. Can only be set if solution_types is SOLUTION_TYPE_RECOMMENDATION. |
| `replacement_control_ids` | Vec<String> | Condition replacement specifications. - Applied according to the order in the list. - A previously replaced term can not be re-replaced. - Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `price_reranking_level` | String | How much price ranking we want in serving results. Price reranking causes product items with a similar recommendation probability to be ordered by price, with the highest-priced items first. This setting could result in a decrease in click-through and conversion rates. Allowed values are: * `no-price-reranking` * `low-price-reranking` * `medium-price-reranking` * `high-price-reranking` If not specified, we choose default based on model type. Default value: `no-price-reranking`. Can only be set if solution_types is SOLUTION_TYPE_RECOMMENDATION. |
| `dynamic_facet_spec` | String | The specification for dynamically generated facets. Notice that only textual facets can be dynamically generated. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `enable_category_filter_level` | String | Whether to add additional category filters on the `similar-items` model. If not specified, we enable it by default. Allowed values are: * `no-category-match`: No additional filtering of original results from the model and the customer's filters. * `relaxed-category-match`: Only keep results with categories that match at least one item categories in the PredictRequests's context item. * If customer also sends filters in the PredictRequest, then the results will satisfy both conditions (user given and category match). Can only be set if solution_types is SOLUTION_TYPE_RECOMMENDATION. |
| `boost_control_ids` | Vec<String> | Condition boost specifications. If a product matches multiple conditions in the specifications, boost scores from these specifications are all applied and combined in a non-linear way. Maximum number of specifications is 100. Notice that if both ServingConfig.boost_control_ids and SearchRequest.boost_spec are set, the boost conditions from both places are evaluated. If a search request matches multiple boost conditions, the final boost score is equal to the sum of the boost scores from all matched boost conditions. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `ignore_recs_denylist` | bool | When the flag is enabled, the products in the denylist will not be filtered out in the recommendation filtering results. |
| `display_name` | String | Required. The human readable serving config display name. Used in Retail UI. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `model_id` | String | The id of the model in the same Catalog to use at serving time. Currently only RecommendationModels are supported: https://cloud.google.com/retail/recommendations-ai/docs/create-models Can be changed but only to a compatible model (e.g. others-you-may-like CTR to others-you-may-like CVR). Required when solution_types is SOLUTION_TYPE_RECOMMENDATION. |
| `oneway_synonyms_control_ids` | Vec<String> | Condition oneway synonyms specifications. If multiple oneway synonyms conditions match, all matching oneway synonyms controls in the list will execute. Order of controls in the list will not matter. Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `personalization_spec` | String | The specification for personalization spec. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. Notice that if both ServingConfig.personalization_spec and SearchRequest.personalization_spec are set. SearchRequest.personalization_spec will override ServingConfig.personalization_spec. |
| `name` | String | Immutable. Fully qualified name `projects/*/locations/global/catalogs/*/servingConfig/*` |
| `redirect_control_ids` | Vec<String> | Condition redirect specifications. Only the first triggered redirect action is applied, even if multiple apply. Maximum number of specifications is 1000. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `twoway_synonyms_control_ids` | Vec<String> | Condition synonyms specifications. If multiple syonyms conditions match, all matching synonyms control in the list will execute. Order of controls in the list will not matter. Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `facet_control_ids` | Vec<String> | Facet specifications for faceted search. If empty, no facets are returned. The ids refer to the ids of Control resources with only the Facet control set. These controls are assumed to be in the same Catalog as the ServingConfig. A maximum of 100 values are allowed. Otherwise, an INVALID_ARGUMENT error is returned. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create serving_config
serving_config = provider.retail_api.Serving_config {
    parent = "value"  # Required. Full resource name of parent. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}`
}

# Access serving_config outputs
serving_config_id = serving_config.id
serving_config_diversity_type = serving_config.diversity_type
serving_config_filter_control_ids = serving_config.filter_control_ids
serving_config_ignore_control_ids = serving_config.ignore_control_ids
serving_config_solution_types = serving_config.solution_types
serving_config_do_not_associate_control_ids = serving_config.do_not_associate_control_ids
serving_config_diversity_level = serving_config.diversity_level
serving_config_replacement_control_ids = serving_config.replacement_control_ids
serving_config_price_reranking_level = serving_config.price_reranking_level
serving_config_dynamic_facet_spec = serving_config.dynamic_facet_spec
serving_config_enable_category_filter_level = serving_config.enable_category_filter_level
serving_config_boost_control_ids = serving_config.boost_control_ids
serving_config_ignore_recs_denylist = serving_config.ignore_recs_denylist
serving_config_display_name = serving_config.display_name
serving_config_model_id = serving_config.model_id
serving_config_oneway_synonyms_control_ids = serving_config.oneway_synonyms_control_ids
serving_config_personalization_spec = serving_config.personalization_spec
serving_config_name = serving_config.name
serving_config_redirect_control_ids = serving_config.redirect_control_ids
serving_config_twoway_synonyms_control_ids = serving_config.twoway_synonyms_control_ids
serving_config_facet_control_ids = serving_config.facet_control_ids
```

---


### Product

Creates a Product.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | String |  | Immutable. The type of the product. Default to Catalog.product_level_config.ingestion_product_type if unset. |
| `retrievable_fields` | String |  | Indicates which fields in the Products are returned in SearchResponse. Supported fields for all types: * audience * availability * brands * color_info * conditions * gtin * materials * name * patterns * price_info * rating * sizes * title * uri Supported fields only for Type.PRIMARY and Type.COLLECTION: * categories * description * images Supported fields only for Type.VARIANT: * Only the first image in images To mark attributes as retrievable, include paths of the form "attributes.key" where "key" is the key of a custom attribute, as specified in attributes. For Type.PRIMARY and Type.COLLECTION, the following fields are always returned in SearchResponse by default: * name For Type.VARIANT, the following fields are always returned in by default: * name * color_info Note: Returning more fields in SearchResponse can increase response payload size and serving latency. This field is deprecated. Use the retrievable site-wide control instead. |
| `title` | String |  | Required. Product title. This field must be a UTF-8 encoded string with a length limit of 1,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [title](https://support.google.com/merchants/answer/6324415). Schema.org property [Product.name](https://schema.org/name). |
| `availability` | String |  | The online availability of the Product. Default to Availability.IN_STOCK. For primary products with variants set the availability of the primary as Availability.OUT_OF_STOCK and set the true availability at the variant level. This way the primary product will be considered "in stock" as long as it has at least one variant in stock. For primary products with no variants set the true availability at the primary level. Corresponding properties: Google Merchant Center property [availability](https://support.google.com/merchants/answer/6324448). Schema.org property [Offer.availability](https://schema.org/availability). |
| `images` | Vec<String> |  | Product images for the product. We highly recommend putting the main image first. A maximum of 300 images are allowed. Corresponding properties: Google Merchant Center property [image_link](https://support.google.com/merchants/answer/6324350). Schema.org property [Product.image](https://schema.org/image). |
| `description` | String |  | Product description. This field must be a UTF-8 encoded string with a length limit of 5,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [description](https://support.google.com/merchants/answer/6324468). Schema.org property [Product.description](https://schema.org/description). |
| `price_info` | String |  | Product price and cost information. Corresponding properties: Google Merchant Center property [price](https://support.google.com/merchants/answer/6324371). |
| `available_time` | String |  | The timestamp when this Product becomes available for SearchService.Search. Note that this is only applicable to Type.PRIMARY and Type.COLLECTION, and ignored for Type.VARIANT. |
| `conditions` | Vec<String> |  | The condition of the product. Strongly encouraged to use the standard values: "new", "refurbished", "used". A maximum of 1 value is allowed per Product. Each value must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [condition](https://support.google.com/merchants/answer/6324469). Schema.org property [Offer.itemCondition](https://schema.org/itemCondition). |
| `gtin` | String |  | The Global Trade Item Number (GTIN) of the product. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. This field must be a Unigram. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [gtin](https://support.google.com/merchants/answer/6324461). Schema.org property [Product.isbn](https://schema.org/isbn), [Product.gtin8](https://schema.org/gtin8), [Product.gtin12](https://schema.org/gtin12), [Product.gtin13](https://schema.org/gtin13), or [Product.gtin14](https://schema.org/gtin14). If the value is not a valid GTIN, an INVALID_ARGUMENT error is returned. |
| `ttl` | String |  | Input only. The TTL (time to live) of the product. Note that this is only applicable to Type.PRIMARY and Type.COLLECTION, and ignored for Type.VARIANT. In general, we suggest the users to delete the stale products explicitly, instead of using this field to determine staleness. If it is set, it must be a non-negative value, and expire_time is set as current timestamp plus ttl. The derived expire_time is returned in the output and ttl is left blank when retrieving the Product. If it is set, the product is not available for SearchService.Search after current timestamp plus ttl. However, the product can still be retrieved by ProductService.GetProduct and ProductService.ListProducts. |
| `tags` | Vec<String> |  | Custom tags associated with the product. At most 250 values are allowed per Product. This value must be a UTF-8 encoded string with a length limit of 1,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. This tag can be used for filtering recommendation results by passing the tag as part of the PredictRequest.filter. Corresponding properties: Google Merchant Center property [custom_label_0–4](https://support.google.com/merchants/answer/6324473). |
| `promotions` | Vec<String> |  | The promotions applied to the product. A maximum of 10 values are allowed per Product. Only Promotion.promotion_id will be used, other fields will be ignored if set. |
| `sizes` | Vec<String> |  | The size of the product. To represent different size systems or size types, consider using this format: [[[size_system:]size_type:]size_value]. For example, in "US:MENS:M", "US" represents size system; "MENS" represents size type; "M" represents size value. In "GIRLS:27", size system is empty; "GIRLS" represents size type; "27" represents size value. In "32 inches", both size system and size type are empty, while size value is "32 inches". A maximum of 20 values are allowed per Product. Each value must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [size](https://support.google.com/merchants/answer/6324492), [size_type](https://support.google.com/merchants/answer/6324497), and [size_system](https://support.google.com/merchants/answer/6324502). Schema.org property [Product.size](https://schema.org/size). |
| `primary_product_id` | String |  | Variant group identifier. Must be an id, with the same parent branch with this product. Otherwise, an error is thrown. For Type.PRIMARY Products, this field can only be empty or set to the same value as id. For VARIANT Products, this field cannot be empty. A maximum of 2,000 products are allowed to share the same Type.PRIMARY Product. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [item_group_id](https://support.google.com/merchants/answer/6324507). Schema.org property [Product.inProductGroupWithID](https://schema.org/inProductGroupWithID). |
| `audience` | String |  | The target group associated with a given audience (e.g. male, veterans, car owners, musicians, etc.) of the product. |
| `available_quantity` | i64 |  | The available quantity of the item. |
| `categories` | Vec<String> |  | Product categories. This field is repeated for supporting one product belonging to several parallel categories. Strongly recommended using the full path for better search / recommendation quality. To represent full path of category, use '>' sign to separate different hierarchies. If '>' is part of the category name, replace it with other character(s). For example, if a shoes product belongs to both ["Shoes & Accessories" -> "Shoes"] and ["Sports & Fitness" -> "Athletic Clothing" -> "Shoes"], it could be represented as: "categories": [ "Shoes & Accessories > Shoes", "Sports & Fitness > Athletic Clothing > Shoes" ] Must be set for Type.PRIMARY Product otherwise an INVALID_ARGUMENT error is returned. At most 250 values are allowed per Product unless overridden through the Google Cloud console. Empty values are not allowed. Each value must be a UTF-8 encoded string with a length limit of 5,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property google_product_category. Schema.org property [Product.category] (https://schema.org/category). [mc_google_product_category]: https://support.google.com/merchants/answer/6324436 |
| `color_info` | String |  | The color of the product. Corresponding properties: Google Merchant Center property [color](https://support.google.com/merchants/answer/6324487). Schema.org property [Product.color](https://schema.org/color). |
| `expire_time` | String |  | Note that this field is applied in the following ways: * If the Product is already expired when it is uploaded, this product is not indexed for search. * If the Product is not expired when it is uploaded, only the Type.PRIMARY's and Type.COLLECTION's expireTime is respected, and Type.VARIANT's expireTime is not used. In general, we suggest the users to delete the stale products explicitly, instead of using this field to determine staleness. expire_time must be later than available_time and publish_time, otherwise an INVALID_ARGUMENT error is thrown. Corresponding properties: Google Merchant Center property [expiration_date](https://support.google.com/merchants/answer/6324499). |
| `language_code` | String |  | Language of the title/description and other string attributes. Use language tags defined by [BCP 47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt). For product prediction, this field is ignored and the model automatically detects the text language. The Product can include text in different languages, but duplicating Products to provide text in multiple languages can result in degraded model performance. For product search this field is in use. It defaults to "en-US" if unset. |
| `publish_time` | String |  | The timestamp when the product is published by the retailer for the first time, which indicates the freshness of the products. Note that this field is different from available_time, given it purely describes product freshness regardless of when it is available on search and recommendation. |
| `brands` | Vec<String> |  | The brands of the product. A maximum of 30 brands are allowed unless overridden through the Google Cloud console. Each brand must be a UTF-8 encoded string with a length limit of 1,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [brand](https://support.google.com/merchants/answer/6324351). Schema.org property [Product.brand](https://schema.org/brand). |
| `id` | String |  | Immutable. Product identifier, which is the final component of name. For example, this field is "id_1", if name is `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/id_1`. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [id](https://support.google.com/merchants/answer/6324405). Schema.org property [Product.sku](https://schema.org/sku). |
| `materials` | Vec<String> |  | The material of the product. For example, "leather", "wooden". A maximum of 20 values are allowed. Each value must be a UTF-8 encoded string with a length limit of 200 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [material](https://support.google.com/merchants/answer/6324410). Schema.org property [Product.material](https://schema.org/material). |
| `rating` | String |  | The rating of this product. |
| `local_inventories` | Vec<String> |  | Output only. A list of local inventories specific to different places. This field can be managed by ProductService.AddLocalInventories and ProductService.RemoveLocalInventories APIs if fine-grained, high-volume updates are necessary. |
| `uri` | String |  | Canonical URL directly linking to the product detail page. It is strongly recommended to provide a valid uri for the product, otherwise the service performance could be significantly degraded. This field must be a UTF-8 encoded string with a length limit of 5,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [link](https://support.google.com/merchants/answer/6324416). Schema.org property [Offer.url](https://schema.org/url). |
| `variants` | Vec<String> |  | Output only. Product variants grouped together on primary product which share similar product attributes. It's automatically grouped by primary_product_id for all the product variants. Only populated for Type.PRIMARY Products. Note: This field is OUTPUT_ONLY for ProductService.GetProduct. Do not set this field in API requests. |
| `name` | String |  | Immutable. Full resource name of the product, such as `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/product_id`. |
| `fulfillment_info` | Vec<String> |  | Fulfillment information, such as the store IDs for in-store pickup or region IDs for different shipping methods. All the elements must have distinct FulfillmentInfo.type. Otherwise, an INVALID_ARGUMENT error is returned. |
| `patterns` | Vec<String> |  | The pattern or graphic print of the product. For example, "striped", "polka dot", "paisley". A maximum of 20 values are allowed per Product. Each value must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [pattern](https://support.google.com/merchants/answer/6324483). Schema.org property [Product.pattern](https://schema.org/pattern). |
| `collection_member_ids` | Vec<String> |  | The id of the collection members when type is Type.COLLECTION. Non-existent product ids are allowed. The type of the members must be either Type.PRIMARY or Type.VARIANT otherwise an INVALID_ARGUMENT error is thrown. Should not set it for other types. A maximum of 1000 values are allowed. Otherwise, an INVALID_ARGUMENT error is return. |
| `attributes` | HashMap<String, String> |  | Highly encouraged. Extra product attributes to be included. For example, for products, this could include the store name, vendor, style, color, etc. These are very strong signals for recommendation model, thus we highly recommend providing the attributes here. Features that can take on one of a limited number of possible values. Two types of features can be set are: Textual features. some examples would be the brand/maker of a product, or country of a customer. Numerical features. Some examples would be the height/weight of a product, or age of a customer. For example: `{ "vendor": {"text": ["vendor123", "vendor456"]}, "lengths_cm": {"numbers":[2.3, 15.4]}, "heights_cm": {"numbers":[8.1, 6.4]} }`. This field needs to pass all below criteria, otherwise an INVALID_ARGUMENT error is returned: * Max entries count: 200. * The key must be a UTF-8 encoded string with a length limit of 128 characters. * For indexable attribute, the key must match the pattern: `a-zA-Z0-9*`. For example, `key0LikeThis` or `KEY_1_LIKE_THIS`. * For text attributes, at most 400 values are allowed. Empty values are not allowed. Each value must be a non-empty UTF-8 encoded string with a length limit of 256 characters. * For number attributes, at most 400 values are allowed. |
| `parent` | String | ✅ | Required. The parent catalog resource name, such as `projects/*/locations/global/catalogs/default_catalog/branches/default_branch`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `type` | String | Immutable. The type of the product. Default to Catalog.product_level_config.ingestion_product_type if unset. |
| `retrievable_fields` | String | Indicates which fields in the Products are returned in SearchResponse. Supported fields for all types: * audience * availability * brands * color_info * conditions * gtin * materials * name * patterns * price_info * rating * sizes * title * uri Supported fields only for Type.PRIMARY and Type.COLLECTION: * categories * description * images Supported fields only for Type.VARIANT: * Only the first image in images To mark attributes as retrievable, include paths of the form "attributes.key" where "key" is the key of a custom attribute, as specified in attributes. For Type.PRIMARY and Type.COLLECTION, the following fields are always returned in SearchResponse by default: * name For Type.VARIANT, the following fields are always returned in by default: * name * color_info Note: Returning more fields in SearchResponse can increase response payload size and serving latency. This field is deprecated. Use the retrievable site-wide control instead. |
| `title` | String | Required. Product title. This field must be a UTF-8 encoded string with a length limit of 1,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [title](https://support.google.com/merchants/answer/6324415). Schema.org property [Product.name](https://schema.org/name). |
| `availability` | String | The online availability of the Product. Default to Availability.IN_STOCK. For primary products with variants set the availability of the primary as Availability.OUT_OF_STOCK and set the true availability at the variant level. This way the primary product will be considered "in stock" as long as it has at least one variant in stock. For primary products with no variants set the true availability at the primary level. Corresponding properties: Google Merchant Center property [availability](https://support.google.com/merchants/answer/6324448). Schema.org property [Offer.availability](https://schema.org/availability). |
| `images` | Vec<String> | Product images for the product. We highly recommend putting the main image first. A maximum of 300 images are allowed. Corresponding properties: Google Merchant Center property [image_link](https://support.google.com/merchants/answer/6324350). Schema.org property [Product.image](https://schema.org/image). |
| `description` | String | Product description. This field must be a UTF-8 encoded string with a length limit of 5,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [description](https://support.google.com/merchants/answer/6324468). Schema.org property [Product.description](https://schema.org/description). |
| `price_info` | String | Product price and cost information. Corresponding properties: Google Merchant Center property [price](https://support.google.com/merchants/answer/6324371). |
| `available_time` | String | The timestamp when this Product becomes available for SearchService.Search. Note that this is only applicable to Type.PRIMARY and Type.COLLECTION, and ignored for Type.VARIANT. |
| `conditions` | Vec<String> | The condition of the product. Strongly encouraged to use the standard values: "new", "refurbished", "used". A maximum of 1 value is allowed per Product. Each value must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [condition](https://support.google.com/merchants/answer/6324469). Schema.org property [Offer.itemCondition](https://schema.org/itemCondition). |
| `gtin` | String | The Global Trade Item Number (GTIN) of the product. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. This field must be a Unigram. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [gtin](https://support.google.com/merchants/answer/6324461). Schema.org property [Product.isbn](https://schema.org/isbn), [Product.gtin8](https://schema.org/gtin8), [Product.gtin12](https://schema.org/gtin12), [Product.gtin13](https://schema.org/gtin13), or [Product.gtin14](https://schema.org/gtin14). If the value is not a valid GTIN, an INVALID_ARGUMENT error is returned. |
| `ttl` | String | Input only. The TTL (time to live) of the product. Note that this is only applicable to Type.PRIMARY and Type.COLLECTION, and ignored for Type.VARIANT. In general, we suggest the users to delete the stale products explicitly, instead of using this field to determine staleness. If it is set, it must be a non-negative value, and expire_time is set as current timestamp plus ttl. The derived expire_time is returned in the output and ttl is left blank when retrieving the Product. If it is set, the product is not available for SearchService.Search after current timestamp plus ttl. However, the product can still be retrieved by ProductService.GetProduct and ProductService.ListProducts. |
| `tags` | Vec<String> | Custom tags associated with the product. At most 250 values are allowed per Product. This value must be a UTF-8 encoded string with a length limit of 1,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. This tag can be used for filtering recommendation results by passing the tag as part of the PredictRequest.filter. Corresponding properties: Google Merchant Center property [custom_label_0–4](https://support.google.com/merchants/answer/6324473). |
| `promotions` | Vec<String> | The promotions applied to the product. A maximum of 10 values are allowed per Product. Only Promotion.promotion_id will be used, other fields will be ignored if set. |
| `sizes` | Vec<String> | The size of the product. To represent different size systems or size types, consider using this format: [[[size_system:]size_type:]size_value]. For example, in "US:MENS:M", "US" represents size system; "MENS" represents size type; "M" represents size value. In "GIRLS:27", size system is empty; "GIRLS" represents size type; "27" represents size value. In "32 inches", both size system and size type are empty, while size value is "32 inches". A maximum of 20 values are allowed per Product. Each value must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [size](https://support.google.com/merchants/answer/6324492), [size_type](https://support.google.com/merchants/answer/6324497), and [size_system](https://support.google.com/merchants/answer/6324502). Schema.org property [Product.size](https://schema.org/size). |
| `primary_product_id` | String | Variant group identifier. Must be an id, with the same parent branch with this product. Otherwise, an error is thrown. For Type.PRIMARY Products, this field can only be empty or set to the same value as id. For VARIANT Products, this field cannot be empty. A maximum of 2,000 products are allowed to share the same Type.PRIMARY Product. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [item_group_id](https://support.google.com/merchants/answer/6324507). Schema.org property [Product.inProductGroupWithID](https://schema.org/inProductGroupWithID). |
| `audience` | String | The target group associated with a given audience (e.g. male, veterans, car owners, musicians, etc.) of the product. |
| `available_quantity` | i64 | The available quantity of the item. |
| `categories` | Vec<String> | Product categories. This field is repeated for supporting one product belonging to several parallel categories. Strongly recommended using the full path for better search / recommendation quality. To represent full path of category, use '>' sign to separate different hierarchies. If '>' is part of the category name, replace it with other character(s). For example, if a shoes product belongs to both ["Shoes & Accessories" -> "Shoes"] and ["Sports & Fitness" -> "Athletic Clothing" -> "Shoes"], it could be represented as: "categories": [ "Shoes & Accessories > Shoes", "Sports & Fitness > Athletic Clothing > Shoes" ] Must be set for Type.PRIMARY Product otherwise an INVALID_ARGUMENT error is returned. At most 250 values are allowed per Product unless overridden through the Google Cloud console. Empty values are not allowed. Each value must be a UTF-8 encoded string with a length limit of 5,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property google_product_category. Schema.org property [Product.category] (https://schema.org/category). [mc_google_product_category]: https://support.google.com/merchants/answer/6324436 |
| `color_info` | String | The color of the product. Corresponding properties: Google Merchant Center property [color](https://support.google.com/merchants/answer/6324487). Schema.org property [Product.color](https://schema.org/color). |
| `expire_time` | String | Note that this field is applied in the following ways: * If the Product is already expired when it is uploaded, this product is not indexed for search. * If the Product is not expired when it is uploaded, only the Type.PRIMARY's and Type.COLLECTION's expireTime is respected, and Type.VARIANT's expireTime is not used. In general, we suggest the users to delete the stale products explicitly, instead of using this field to determine staleness. expire_time must be later than available_time and publish_time, otherwise an INVALID_ARGUMENT error is thrown. Corresponding properties: Google Merchant Center property [expiration_date](https://support.google.com/merchants/answer/6324499). |
| `language_code` | String | Language of the title/description and other string attributes. Use language tags defined by [BCP 47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt). For product prediction, this field is ignored and the model automatically detects the text language. The Product can include text in different languages, but duplicating Products to provide text in multiple languages can result in degraded model performance. For product search this field is in use. It defaults to "en-US" if unset. |
| `publish_time` | String | The timestamp when the product is published by the retailer for the first time, which indicates the freshness of the products. Note that this field is different from available_time, given it purely describes product freshness regardless of when it is available on search and recommendation. |
| `brands` | Vec<String> | The brands of the product. A maximum of 30 brands are allowed unless overridden through the Google Cloud console. Each brand must be a UTF-8 encoded string with a length limit of 1,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [brand](https://support.google.com/merchants/answer/6324351). Schema.org property [Product.brand](https://schema.org/brand). |
| `id` | String | Immutable. Product identifier, which is the final component of name. For example, this field is "id_1", if name is `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/id_1`. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [id](https://support.google.com/merchants/answer/6324405). Schema.org property [Product.sku](https://schema.org/sku). |
| `materials` | Vec<String> | The material of the product. For example, "leather", "wooden". A maximum of 20 values are allowed. Each value must be a UTF-8 encoded string with a length limit of 200 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [material](https://support.google.com/merchants/answer/6324410). Schema.org property [Product.material](https://schema.org/material). |
| `rating` | String | The rating of this product. |
| `local_inventories` | Vec<String> | Output only. A list of local inventories specific to different places. This field can be managed by ProductService.AddLocalInventories and ProductService.RemoveLocalInventories APIs if fine-grained, high-volume updates are necessary. |
| `uri` | String | Canonical URL directly linking to the product detail page. It is strongly recommended to provide a valid uri for the product, otherwise the service performance could be significantly degraded. This field must be a UTF-8 encoded string with a length limit of 5,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [link](https://support.google.com/merchants/answer/6324416). Schema.org property [Offer.url](https://schema.org/url). |
| `variants` | Vec<String> | Output only. Product variants grouped together on primary product which share similar product attributes. It's automatically grouped by primary_product_id for all the product variants. Only populated for Type.PRIMARY Products. Note: This field is OUTPUT_ONLY for ProductService.GetProduct. Do not set this field in API requests. |
| `name` | String | Immutable. Full resource name of the product, such as `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/product_id`. |
| `fulfillment_info` | Vec<String> | Fulfillment information, such as the store IDs for in-store pickup or region IDs for different shipping methods. All the elements must have distinct FulfillmentInfo.type. Otherwise, an INVALID_ARGUMENT error is returned. |
| `patterns` | Vec<String> | The pattern or graphic print of the product. For example, "striped", "polka dot", "paisley". A maximum of 20 values are allowed per Product. Each value must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [pattern](https://support.google.com/merchants/answer/6324483). Schema.org property [Product.pattern](https://schema.org/pattern). |
| `collection_member_ids` | Vec<String> | The id of the collection members when type is Type.COLLECTION. Non-existent product ids are allowed. The type of the members must be either Type.PRIMARY or Type.VARIANT otherwise an INVALID_ARGUMENT error is thrown. Should not set it for other types. A maximum of 1000 values are allowed. Otherwise, an INVALID_ARGUMENT error is return. |
| `attributes` | HashMap<String, String> | Highly encouraged. Extra product attributes to be included. For example, for products, this could include the store name, vendor, style, color, etc. These are very strong signals for recommendation model, thus we highly recommend providing the attributes here. Features that can take on one of a limited number of possible values. Two types of features can be set are: Textual features. some examples would be the brand/maker of a product, or country of a customer. Numerical features. Some examples would be the height/weight of a product, or age of a customer. For example: `{ "vendor": {"text": ["vendor123", "vendor456"]}, "lengths_cm": {"numbers":[2.3, 15.4]}, "heights_cm": {"numbers":[8.1, 6.4]} }`. This field needs to pass all below criteria, otherwise an INVALID_ARGUMENT error is returned: * Max entries count: 200. * The key must be a UTF-8 encoded string with a length limit of 128 characters. * For indexable attribute, the key must match the pattern: `a-zA-Z0-9*`. For example, `key0LikeThis` or `KEY_1_LIKE_THIS`. * For text attributes, at most 400 values are allowed. Empty values are not allowed. Each value must be a non-empty UTF-8 encoded string with a length limit of 256 characters. * For number attributes, at most 400 values are allowed. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create product
product = provider.retail_api.Product {
    parent = "value"  # Required. The parent catalog resource name, such as `projects/*/locations/global/catalogs/default_catalog/branches/default_branch`.
}

# Access product outputs
product_id = product.id
product_type = product.type
product_retrievable_fields = product.retrievable_fields
product_title = product.title
product_availability = product.availability
product_images = product.images
product_description = product.description
product_price_info = product.price_info
product_available_time = product.available_time
product_conditions = product.conditions
product_gtin = product.gtin
product_ttl = product.ttl
product_tags = product.tags
product_promotions = product.promotions
product_sizes = product.sizes
product_primary_product_id = product.primary_product_id
product_audience = product.audience
product_available_quantity = product.available_quantity
product_categories = product.categories
product_color_info = product.color_info
product_expire_time = product.expire_time
product_language_code = product.language_code
product_publish_time = product.publish_time
product_brands = product.brands
product_id = product.id
product_materials = product.materials
product_rating = product.rating
product_local_inventories = product.local_inventories
product_uri = product.uri
product_variants = product.variants
product_name = product.name
product_fulfillment_info = product.fulfillment_info
product_patterns = product.patterns
product_collection_member_ids = product.collection_member_ids
product_attributes = product.attributes
```

---


### Completion_data

Bulk import of processed completion dataset. Request processing is asynchronous. Partial updating is not supported. The operation is successfully finished only after the imported suggestions are indexed successfully and ready for serving. The process takes hours. This feature is only available for users who have Retail Search enabled. Enable Retail Search on Cloud Console before using this feature.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `input_config` | String |  | Required. The desired input location of the data. |
| `notification_pubsub_topic` | String |  | Pub/Sub topic for receiving notification. If this field is set, when the import is finished, a notification is sent to specified Pub/Sub topic. The message data is JSON string of a Operation. Format of the Pub/Sub topic is `projects/{project}/topics/{topic}`. |
| `parent` | String | ✅ | Required. The catalog which the suggestions dataset belongs to. Format: `projects/1234/locations/global/catalogs/default_catalog`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create completion_data
completion_data = provider.retail_api.Completion_data {
    parent = "value"  # Required. The catalog which the suggestions dataset belongs to. Format: `projects/1234/locations/global/catalogs/default_catalog`.
}

```

---


### Attributes_config

Adds the specified CatalogAttribute to the AttributesConfig. If the CatalogAttribute to add already exists, an ALREADY_EXISTS error is returned.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `catalog_attribute` | String |  | Required. The CatalogAttribute to add. |
| `attributes_config` | String | ✅ | Required. Full AttributesConfig resource name. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/attributesConfig` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create attributes_config
attributes_config = provider.retail_api.Attributes_config {
    attributes_config = "value"  # Required. Full AttributesConfig resource name. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/attributesConfig`
}

```

---


### Placement

Performs a conversational search. This feature is only available for users who have Conversational Search enabled.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `branch` | String |  | Required. The branch resource name, such as `projects/*/locations/global/catalogs/default_catalog/branches/0`. Use "default_branch" as the branch ID or leave this field empty, to search products under the default branch. |
| `conversational_filtering_spec` | String |  | Optional. This field specifies all conversational filtering related parameters. |
| `page_categories` | Vec<String> |  | Optional. The categories associated with a category page. Must be set for category navigation queries to achieve good search quality. The format should be the same as UserEvent.page_categories; To represent full path of category, use '>' sign to separate different hierarchies. If '>' is part of the category name, replace it with other character(s). Category pages include special pages such as sales or promotions. For instance, a special sale page may have the category hierarchy: "pageCategories" : ["Sales > 2017 Black Friday Deals"]. |
| `conversation_id` | String |  | Optional. This field specifies the conversation id, which maintains the state of the conversation between client side and server side. Use the value from the previous ConversationalSearchResponse.conversation_id. For the initial request, this should be empty. |
| `user_labels` | HashMap<String, String> |  | Optional. The user labels applied to a resource must meet the following requirements: * Each resource can have multiple labels, up to a maximum of 64. * Each label must be a key-value pair. * Keys have a minimum length of 1 character and a maximum length of 63 characters and cannot be empty. Values can be empty and have a maximum length of 63 characters. * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. All characters must use UTF-8 encoding, and international characters are allowed. * The key portion of a label must be unique. However, you can use the same key with multiple resources. * Keys must start with a lowercase letter or international character. See [Google Cloud Document](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements) for more details. |
| `visitor_id` | String |  | Required. A unique identifier for tracking visitors. For example, this could be implemented with an HTTP cookie, which should be able to uniquely identify a visitor on a single device. This unique identifier should not change if the visitor logs in or out of the website. This should be the same identifier as UserEvent.visitor_id. The field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `query` | String |  | Optional. Raw search query to be searched for. If this field is empty, the request is considered a category browsing request. |
| `search_params` | String |  | Optional. Search parameters. |
| `safety_settings` | Vec<String> |  | Optional. The safety settings to be applied to the generated content. |
| `user_info` | String |  | Optional. User information. |
| `placement` | String | ✅ | Required. The resource name of the search engine placement, such as `projects/*/locations/global/catalogs/default_catalog/placements/default_search` or `projects/*/locations/global/catalogs/default_catalog/servingConfigs/default_serving_config` This field is used to identify the serving config name and the set of models that will be used to make the search. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create placement
placement = provider.retail_api.Placement {
    placement = "value"  # Required. The resource name of the search engine placement, such as `projects/*/locations/global/catalogs/default_catalog/placements/default_search` or `projects/*/locations/global/catalogs/default_catalog/servingConfigs/default_serving_config` This field is used to identify the serving config name and the set of models that will be used to make the search.
}

```

---


### Generative_question

Allows management of multiple questions.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `requests` | Vec<String> |  | Required. The updates question configs. |
| `parent` | String | ✅ | Optional. Resource name of the parent catalog. Format: projects/{project}/locations/{location}/catalogs/{catalog} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `generative_question_configs` | Vec<String> | All the questions for a given catalog. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create generative_question
generative_question = provider.retail_api.Generative_question {
    parent = "value"  # Optional. Resource name of the parent catalog. Format: projects/{project}/locations/{location}/catalogs/{catalog}
}

# Access generative_question outputs
generative_question_id = generative_question.id
generative_question_generative_question_configs = generative_question.generative_question_configs
```

---


### Model

Creates a new model.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `model_features_config` | String |  | Optional. Additional model features config. |
| `data_state` | String |  | Output only. The state of data requirements for this model: `DATA_OK` and `DATA_ERROR`. Recommendation model cannot be trained if the data is in `DATA_ERROR` state. Recommendation model can have `DATA_ERROR` state even if serving state is `ACTIVE`: models were trained successfully before, but cannot be refreshed because model no longer has sufficient data for training. |
| `training_state` | String |  | Optional. The training state that the model is in (e.g. `TRAINING` or `PAUSED`). Since part of the cost of running the service is frequency of training - this can be used to determine when to train model in order to control cost. If not specified: the default value for `CreateModel` method is `TRAINING`. The default value for `UpdateModel` method is to keep the state the same as before. |
| `create_time` | String |  | Output only. Timestamp the Recommendation Model was created at. |
| `filtering_option` | String |  | Optional. If `RECOMMENDATIONS_FILTERING_ENABLED`, recommendation filtering by attributes is enabled for the model. |
| `display_name` | String |  | Required. The display name of the model. Should be human readable, used to display Recommendation Models in the Retail Cloud Console Dashboard. UTF-8 encoded string with limit of 1024 characters. |
| `type` | String |  | Required. The type of model e.g. `home-page`. Currently supported values: `recommended-for-you`, `others-you-may-like`, `frequently-bought-together`, `page-optimization`, `similar-items`, `buy-it-again`, `on-sale-items`, and `recently-viewed`(readonly value). This field together with optimization_objective describe model metadata to use to control model training and serving. See https://cloud.google.com/retail/docs/models for more details on what the model metadata control and which combination of parameters are valid. For invalid combinations of parameters (e.g. type = `frequently-bought-together` and optimization_objective = `ctr`), you receive an error 400 if you try to create/update a recommendation with this set of knobs. |
| `last_tune_time` | String |  | Output only. The timestamp when the latest successful tune finished. |
| `serving_config_lists` | Vec<String> |  | Output only. The list of valid serving configs associated with the PageOptimizationConfig. |
| `update_time` | String |  | Output only. Timestamp the Recommendation Model was last updated. E.g. if a Recommendation Model was paused - this would be the time the pause was initiated. |
| `serving_state` | String |  | Output only. The serving state of the model: `ACTIVE`, `NOT_ACTIVE`. |
| `tuning_operation` | String |  | Output only. The tune operation associated with the model. Can be used to determine if there is an ongoing tune for this recommendation. Empty field implies no tune is goig on. |
| `periodic_tuning_state` | String |  | Optional. The state of periodic tuning. The period we use is 3 months - to do a one-off tune earlier use the `TuneModel` method. Default value is `PERIODIC_TUNING_ENABLED`. |
| `optimization_objective` | String |  | Optional. The optimization objective e.g. `cvr`. Currently supported values: `ctr`, `cvr`, `revenue-per-order`. If not specified, we choose default based on model type. Default depends on type of recommendation: `recommended-for-you` => `ctr` `others-you-may-like` => `ctr` `frequently-bought-together` => `revenue_per_order` This field together with optimization_objective describe model metadata to use to control model training and serving. See https://cloud.google.com/retail/docs/models for more details on what the model metadata control and which combination of parameters are valid. For invalid combinations of parameters (e.g. type = `frequently-bought-together` and optimization_objective = `ctr`), you receive an error 400 if you try to create/update a recommendation with this set of knobs. |
| `name` | String |  | Required. The fully qualified resource name of the model. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/models/{model_id}` catalog_id has char limit of 50. recommendation_model_id has char limit of 40. |
| `parent` | String | ✅ | Required. The parent resource under which to create the model. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `model_features_config` | String | Optional. Additional model features config. |
| `data_state` | String | Output only. The state of data requirements for this model: `DATA_OK` and `DATA_ERROR`. Recommendation model cannot be trained if the data is in `DATA_ERROR` state. Recommendation model can have `DATA_ERROR` state even if serving state is `ACTIVE`: models were trained successfully before, but cannot be refreshed because model no longer has sufficient data for training. |
| `training_state` | String | Optional. The training state that the model is in (e.g. `TRAINING` or `PAUSED`). Since part of the cost of running the service is frequency of training - this can be used to determine when to train model in order to control cost. If not specified: the default value for `CreateModel` method is `TRAINING`. The default value for `UpdateModel` method is to keep the state the same as before. |
| `create_time` | String | Output only. Timestamp the Recommendation Model was created at. |
| `filtering_option` | String | Optional. If `RECOMMENDATIONS_FILTERING_ENABLED`, recommendation filtering by attributes is enabled for the model. |
| `display_name` | String | Required. The display name of the model. Should be human readable, used to display Recommendation Models in the Retail Cloud Console Dashboard. UTF-8 encoded string with limit of 1024 characters. |
| `type` | String | Required. The type of model e.g. `home-page`. Currently supported values: `recommended-for-you`, `others-you-may-like`, `frequently-bought-together`, `page-optimization`, `similar-items`, `buy-it-again`, `on-sale-items`, and `recently-viewed`(readonly value). This field together with optimization_objective describe model metadata to use to control model training and serving. See https://cloud.google.com/retail/docs/models for more details on what the model metadata control and which combination of parameters are valid. For invalid combinations of parameters (e.g. type = `frequently-bought-together` and optimization_objective = `ctr`), you receive an error 400 if you try to create/update a recommendation with this set of knobs. |
| `last_tune_time` | String | Output only. The timestamp when the latest successful tune finished. |
| `serving_config_lists` | Vec<String> | Output only. The list of valid serving configs associated with the PageOptimizationConfig. |
| `update_time` | String | Output only. Timestamp the Recommendation Model was last updated. E.g. if a Recommendation Model was paused - this would be the time the pause was initiated. |
| `serving_state` | String | Output only. The serving state of the model: `ACTIVE`, `NOT_ACTIVE`. |
| `tuning_operation` | String | Output only. The tune operation associated with the model. Can be used to determine if there is an ongoing tune for this recommendation. Empty field implies no tune is goig on. |
| `periodic_tuning_state` | String | Optional. The state of periodic tuning. The period we use is 3 months - to do a one-off tune earlier use the `TuneModel` method. Default value is `PERIODIC_TUNING_ENABLED`. |
| `optimization_objective` | String | Optional. The optimization objective e.g. `cvr`. Currently supported values: `ctr`, `cvr`, `revenue-per-order`. If not specified, we choose default based on model type. Default depends on type of recommendation: `recommended-for-you` => `ctr` `others-you-may-like` => `ctr` `frequently-bought-together` => `revenue_per_order` This field together with optimization_objective describe model metadata to use to control model training and serving. See https://cloud.google.com/retail/docs/models for more details on what the model metadata control and which combination of parameters are valid. For invalid combinations of parameters (e.g. type = `frequently-bought-together` and optimization_objective = `ctr`), you receive an error 400 if you try to create/update a recommendation with this set of knobs. |
| `name` | String | Required. The fully qualified resource name of the model. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/models/{model_id}` catalog_id has char limit of 50. recommendation_model_id has char limit of 40. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create model
model = provider.retail_api.Model {
    parent = "value"  # Required. The parent resource under which to create the model. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}`
}

# Access model outputs
model_id = model.id
model_model_features_config = model.model_features_config
model_data_state = model.data_state
model_training_state = model.training_state
model_create_time = model.create_time
model_filtering_option = model.filtering_option
model_display_name = model.display_name
model_type = model.type
model_last_tune_time = model.last_tune_time
model_serving_config_lists = model.serving_config_lists
model_update_time = model.update_time
model_serving_state = model.serving_state
model_tuning_operation = model.tuning_operation
model_periodic_tuning_state = model.periodic_tuning_state
model_optimization_objective = model.optimization_objective
model_name = model.name
```

---


### Operation

Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `error` | String | The error result of the operation in case of failure or cancellation. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access operation outputs
operation_id = operation.id
operation_done = operation.done
operation_response = operation.response
operation_metadata = operation.metadata
operation_name = operation.name
operation_error = operation.error
```

---


### User_event

Writes a single user event from the browser. For larger user event payload over 16 KB, the POST method should be used instead, otherwise a 400 Bad Request error is returned. This method is used only by the Retail API JavaScript pixel and Google Tag Manager. Users should not call this method directly.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `prebuilt_rule` | String |  | The prebuilt rule name that can convert a specific type of raw_json. For example: "ga4_bq" rule for the GA4 user event schema. |
| `user_event` | String |  | Required. URL encoded UserEvent proto with a length limit of 2,000,000 characters. |
| `ets` | String |  | The event timestamp in milliseconds. This prevents browser caching of otherwise identical get requests. The name is abbreviated to reduce the payload bytes. |
| `uri` | String |  | The URL including cgi-parameters but excluding the hash fragment with a length limit of 5,000 characters. This is often more useful than the referer URL, because many browsers only send the domain for 3rd party requests. |
| `raw_json` | String |  | An arbitrary serialized JSON string that contains necessary information that can comprise a user event. When this field is specified, the user_event field will be ignored. Note: line-delimited JSON is not supported, a single JSON only. |
| `parent` | String | ✅ | Required. The parent catalog name, such as `projects/1234/locations/global/catalogs/default_catalog`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user_event
user_event = provider.retail_api.User_event {
    parent = "value"  # Required. The parent catalog name, such as `projects/1234/locations/global/catalogs/default_catalog`.
}

```

---


### Catalog

Exports analytics metrics. `Operation.response` is of type `ExportAnalyticsMetricsResponse`. `Operation.metadata` is of type `ExportMetadata`.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `output_config` | String |  | Required. The output location of the data. |
| `filter` | String |  | A filtering expression to specify restrictions on returned metrics. The expression is a sequence of terms. Each term applies a restriction to the returned metrics. Use this expression to restrict results to a specific time range. Currently we expect only one types of fields: * `timestamp`: This can be specified twice, once with a less than operator and once with a greater than operator. The `timestamp` restriction should result in one, contiguous, valid, `timestamp` range. Some examples of valid filters expressions: * Example 1: `timestamp > "2012-04-23T18:25:43.511Z" timestamp < "2012-04-23T18:30:43.511Z"` * Example 2: `timestamp > "2012-04-23T18:25:43.511Z"` |
| `catalog` | String | ✅ | Required. Full resource name of the parent catalog. Expected format: `projects/*/locations/*/catalogs/*` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Required. Immutable. The fully qualified resource name of the attribute config. Format: `projects/*/locations/*/catalogs/*/attributesConfig` |
| `attribute_config_level` | String | Output only. The AttributeConfigLevel used for this catalog. |
| `catalog_attributes` | HashMap<String, String> | Enable attribute(s) config at catalog level. For example, indexable, dynamic_facetable, or searchable for each attribute. The key is catalog attribute's name. For example: `color`, `brands`, `attributes.custom_attribute`, such as `attributes.xyz`. The maximum number of catalog attributes allowed in a request is 1000. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create catalog
catalog = provider.retail_api.Catalog {
    catalog = "value"  # Required. Full resource name of the parent catalog. Expected format: `projects/*/locations/*/catalogs/*`
}

# Access catalog outputs
catalog_id = catalog.id
catalog_name = catalog.name
catalog_attribute_config_level = catalog.attribute_config_level
catalog_catalog_attributes = catalog.catalog_attributes
```

---


### Control

Creates a Control. If the Control to create already exists, an ALREADY_EXISTS error is returned.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `search_solution_use_case` | Vec<String> |  | Specifies the use case for the control. Affects what condition fields can be set. Only settable by search controls. Will default to SEARCH_SOLUTION_USE_CASE_SEARCH if not specified. Currently only allow one search_solution_use_case per control. |
| `rule` | String |  | A rule control - a condition-action pair. Enacts a set action when the condition is triggered. For example: Boost "gShoe" when query full matches "Running Shoes". |
| `name` | String |  | Immutable. Fully qualified name `projects/*/locations/global/catalogs/*/controls/*` |
| `display_name` | String |  | Required. The human readable control display name. Used in Retail UI. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is thrown. |
| `associated_serving_config_ids` | Vec<String> |  | Output only. List of serving config ids that are associated with this control in the same Catalog. Note the association is managed via the ServingConfig, this is an output only denormalized view. |
| `solution_types` | Vec<String> |  | Required. Immutable. The solution types that the control is used for. Currently we support setting only one type of solution at creation time. Only `SOLUTION_TYPE_SEARCH` value is supported at the moment. If no solution type is provided at creation time, will default to SOLUTION_TYPE_SEARCH. |
| `parent` | String | ✅ | Required. Full resource name of parent catalog. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `search_solution_use_case` | Vec<String> | Specifies the use case for the control. Affects what condition fields can be set. Only settable by search controls. Will default to SEARCH_SOLUTION_USE_CASE_SEARCH if not specified. Currently only allow one search_solution_use_case per control. |
| `rule` | String | A rule control - a condition-action pair. Enacts a set action when the condition is triggered. For example: Boost "gShoe" when query full matches "Running Shoes". |
| `name` | String | Immutable. Fully qualified name `projects/*/locations/global/catalogs/*/controls/*` |
| `display_name` | String | Required. The human readable control display name. Used in Retail UI. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is thrown. |
| `associated_serving_config_ids` | Vec<String> | Output only. List of serving config ids that are associated with this control in the same Catalog. Note the association is managed via the ServingConfig, this is an output only denormalized view. |
| `solution_types` | Vec<String> | Required. Immutable. The solution types that the control is used for. Currently we support setting only one type of solution at creation time. Only `SOLUTION_TYPE_SEARCH` value is supported at the moment. If no solution type is provided at creation time, will default to SOLUTION_TYPE_SEARCH. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create control
control = provider.retail_api.Control {
    parent = "value"  # Required. Full resource name of parent catalog. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}`
}

# Access control outputs
control_id = control.id
control_search_solution_use_case = control.search_solution_use_case
control_rule = control.rule
control_name = control.name
control_display_name = control.display_name
control_associated_serving_config_ids = control.associated_serving_config_ids
control_solution_types = control.solution_types
```

---


### Catalog

Set a specified branch id as default branch. API methods such as SearchService.Search, ProductService.GetProduct, ProductService.ListProducts will treat requests using "default_branch" to the actual branch id set as default. For example, if `projects/*/locations/*/catalogs/*/branches/1` is set as default, setting SearchRequest.branch to `projects/*/locations/*/catalogs/*/branches/default_branch` is equivalent to setting SearchRequest.branch to `projects/*/locations/*/catalogs/*/branches/1`. Using multiple branches can be useful when developers would like to have a staging branch to test and verify for future usage. When it becomes ready, developers switch on the staging branch using this API while keeping using `projects/*/locations/*/catalogs/*/branches/default_branch` as SearchRequest.branch to route the traffic to this staging branch. CAUTION: If you have live predict/search traffic, switching the default branch could potentially cause outages if the ID space of the new branch is very different from the old one. More specifically: * PredictionService will only return product IDs from branch {newBranch}. * SearchService will only return product IDs from branch {newBranch} (if branch is not explicitly set). * UserEventService will only join events with products from branch {newBranch}.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `force` | bool |  | If set to true, it permits switching to a branch with branch_id even if it has no sufficient active products. |
| `branch_id` | String |  | The final component of the resource name of a branch. This field must be one of "0", "1" or "2". Otherwise, an INVALID_ARGUMENT error is returned. If there are no sufficient active products in the targeted branch and force is not set, a FAILED_PRECONDITION error is returned. |
| `note` | String |  | Some note on this request, this can be retrieved by CatalogService.GetDefaultBranch before next valid default branch set occurs. This field must be a UTF-8 encoded string with a length limit of 1,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `catalog` | String | ✅ | Full resource name of the catalog, such as `projects/*/locations/global/catalogs/default_catalog`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | A token that can be sent as ListCatalogsRequest.page_token to retrieve the next page. If this field is omitted, there are no subsequent pages. |
| `catalogs` | Vec<String> | All the customer's Catalogs. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create catalog
catalog = provider.retail_api.Catalog {
    catalog = "value"  # Full resource name of the catalog, such as `projects/*/locations/global/catalogs/default_catalog`.
}

# Access catalog outputs
catalog_id = catalog.id
catalog_next_page_token = catalog.next_page_token
catalog_catalogs = catalog.catalogs
```

---


### Control

Creates a Control. If the Control to create already exists, an ALREADY_EXISTS error is returned.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Immutable. Fully qualified name `projects/*/locations/global/catalogs/*/controls/*` |
| `search_solution_use_case` | Vec<String> |  | Specifies the use case for the control. Affects what condition fields can be set. Only settable by search controls. Will default to SEARCH_SOLUTION_USE_CASE_SEARCH if not specified. Currently only allow one search_solution_use_case per control. |
| `rule` | String |  | A rule control - a condition-action pair. Enacts a set action when the condition is triggered. For example: Boost "gShoe" when query full matches "Running Shoes". |
| `display_name` | String |  | Required. The human readable control display name. Used in Retail UI. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is thrown. |
| `solution_types` | Vec<String> |  | Required. Immutable. The solution types that the control is used for. Currently we support setting only one type of solution at creation time. Only `SOLUTION_TYPE_SEARCH` value is supported at the moment. If no solution type is provided at creation time, will default to SOLUTION_TYPE_SEARCH. |
| `associated_serving_config_ids` | Vec<String> |  | Output only. List of serving config ids that are associated with this control in the same Catalog. Note the association is managed via the ServingConfig, this is an output only denormalized view. |
| `parent` | String | ✅ | Required. Full resource name of parent catalog. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Immutable. Fully qualified name `projects/*/locations/global/catalogs/*/controls/*` |
| `search_solution_use_case` | Vec<String> | Specifies the use case for the control. Affects what condition fields can be set. Only settable by search controls. Will default to SEARCH_SOLUTION_USE_CASE_SEARCH if not specified. Currently only allow one search_solution_use_case per control. |
| `rule` | String | A rule control - a condition-action pair. Enacts a set action when the condition is triggered. For example: Boost "gShoe" when query full matches "Running Shoes". |
| `display_name` | String | Required. The human readable control display name. Used in Retail UI. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is thrown. |
| `solution_types` | Vec<String> | Required. Immutable. The solution types that the control is used for. Currently we support setting only one type of solution at creation time. Only `SOLUTION_TYPE_SEARCH` value is supported at the moment. If no solution type is provided at creation time, will default to SOLUTION_TYPE_SEARCH. |
| `associated_serving_config_ids` | Vec<String> | Output only. List of serving config ids that are associated with this control in the same Catalog. Note the association is managed via the ServingConfig, this is an output only denormalized view. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create control
control = provider.retail_api.Control {
    parent = "value"  # Required. Full resource name of parent catalog. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}`
}

# Access control outputs
control_id = control.id
control_name = control.name
control_search_solution_use_case = control.search_solution_use_case
control_rule = control.rule
control_display_name = control.display_name
control_solution_types = control.solution_types
control_associated_serving_config_ids = control.associated_serving_config_ids
```

---


### User_event

Exports user events. `Operation.response` is of type `ExportResponse`. `Operation.metadata` is of type `ExportMetadata`.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `filter` | String |  | A filtering expression to specify restrictions on returned events. The expression is a sequence of terms. Each term applies a restriction to the returned user events. Use this expression to restrict results to a specific time range or to filter events by eventType. For example, `eventTime > "2012-04-23T18:25:43.511Z" eventsMissingCatalogItems eventTime<"2012-04-23T18:25:43.511Z" eventType=search` We expect only three types of fields: * `eventTime`: This can be specified twice, once with a less than operator and once with a greater than operator. The `eventTime` restriction should result in one, contiguous, valid, `eventTime` range. * `eventType`: Boolean operators `OR` and `NOT` are supported if the expression is enclosed in parentheses and the operators are separated from the tag values by a space. * `eventsMissingCatalogItems`: This restricts results to events for which catalog items were not found in the catalog. The default behavior is to return only those events for which catalog items were found. Some examples of valid filters expressions: * Example 1: `eventTime > "2012-04-23T18:25:43.511Z" eventTime < "2012-04-23T18:30:43.511Z"` * Example 2: `eventTime > "2012-04-23T18:25:43.511Z" eventType = detail-page-view` * Example 3: `eventsMissingCatalogItems eventType = (NOT search) eventTime < "2018-04-23T18:30:43.511Z"` * Example 4: `eventTime > "2012-04-23T18:25:43.511Z"` * Example 5: `eventType = (detail-page-view OR search)` * Example 6: `eventsMissingCatalogItems` |
| `output_config` | String |  | Required. The output location of the data. |
| `parent` | String | ✅ | Required. Resource name of a Catalog. For example `projects/1234/locations/global/catalogs/default_catalog` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create user_event
user_event = provider.retail_api.User_event {
    parent = "value"  # Required. Resource name of a Catalog. For example `projects/1234/locations/global/catalogs/default_catalog`
}

```

---


### Model

Creates a new model.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tuning_operation` | String |  | Output only. The tune operation associated with the model. Can be used to determine if there is an ongoing tune for this recommendation. Empty field implies no tune is goig on. |
| `serving_state` | String |  | Output only. The serving state of the model: `ACTIVE`, `NOT_ACTIVE`. |
| `page_optimization_config` | String |  | Optional. The page optimization config. |
| `type` | String |  | Required. The type of model e.g. `home-page`. Currently supported values: `recommended-for-you`, `others-you-may-like`, `frequently-bought-together`, `page-optimization`, `similar-items`, `buy-it-again`, `on-sale-items`, and `recently-viewed`(readonly value). This field together with optimization_objective describe model metadata to use to control model training and serving. See https://cloud.google.com/retail/docs/models for more details on what the model metadata control and which combination of parameters are valid. For invalid combinations of parameters (e.g. type = `frequently-bought-together` and optimization_objective = `ctr`), you receive an error 400 if you try to create/update a recommendation with this set of knobs. |
| `last_tune_time` | String |  | Output only. The timestamp when the latest successful tune finished. |
| `optimization_objective` | String |  | Optional. The optimization objective e.g. `cvr`. Currently supported values: `ctr`, `cvr`, `revenue-per-order`. If not specified, we choose default based on model type. Default depends on type of recommendation: `recommended-for-you` => `ctr` `others-you-may-like` => `ctr` `frequently-bought-together` => `revenue_per_order` This field together with optimization_objective describe model metadata to use to control model training and serving. See https://cloud.google.com/retail/docs/models for more details on what the model metadata control and which combination of parameters are valid. For invalid combinations of parameters (e.g. type = `frequently-bought-together` and optimization_objective = `ctr`), you receive an error 400 if you try to create/update a recommendation with this set of knobs. |
| `training_state` | String |  | Optional. The training state that the model is in (e.g. `TRAINING` or `PAUSED`). Since part of the cost of running the service is frequency of training - this can be used to determine when to train model in order to control cost. If not specified: the default value for `CreateModel` method is `TRAINING`. The default value for `UpdateModel` method is to keep the state the same as before. |
| `create_time` | String |  | Output only. Timestamp the Recommendation Model was created at. |
| `model_features_config` | String |  | Optional. Additional model features config. |
| `periodic_tuning_state` | String |  | Optional. The state of periodic tuning. The period we use is 3 months - to do a one-off tune earlier use the `TuneModel` method. Default value is `PERIODIC_TUNING_ENABLED`. |
| `data_state` | String |  | Output only. The state of data requirements for this model: `DATA_OK` and `DATA_ERROR`. Recommendation model cannot be trained if the data is in `DATA_ERROR` state. Recommendation model can have `DATA_ERROR` state even if serving state is `ACTIVE`: models were trained successfully before, but cannot be refreshed because model no longer has sufficient data for training. |
| `display_name` | String |  | Required. The display name of the model. Should be human readable, used to display Recommendation Models in the Retail Cloud Console Dashboard. UTF-8 encoded string with limit of 1024 characters. |
| `filtering_option` | String |  | Optional. If `RECOMMENDATIONS_FILTERING_ENABLED`, recommendation filtering by attributes is enabled for the model. |
| `serving_config_lists` | Vec<String> |  | Output only. The list of valid serving configs associated with the PageOptimizationConfig. |
| `update_time` | String |  | Output only. Timestamp the Recommendation Model was last updated. E.g. if a Recommendation Model was paused - this would be the time the pause was initiated. |
| `name` | String |  | Required. The fully qualified resource name of the model. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/models/{model_id}` catalog_id has char limit of 50. recommendation_model_id has char limit of 40. |
| `parent` | String | ✅ | Required. The parent resource under which to create the model. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `tuning_operation` | String | Output only. The tune operation associated with the model. Can be used to determine if there is an ongoing tune for this recommendation. Empty field implies no tune is goig on. |
| `serving_state` | String | Output only. The serving state of the model: `ACTIVE`, `NOT_ACTIVE`. |
| `page_optimization_config` | String | Optional. The page optimization config. |
| `type` | String | Required. The type of model e.g. `home-page`. Currently supported values: `recommended-for-you`, `others-you-may-like`, `frequently-bought-together`, `page-optimization`, `similar-items`, `buy-it-again`, `on-sale-items`, and `recently-viewed`(readonly value). This field together with optimization_objective describe model metadata to use to control model training and serving. See https://cloud.google.com/retail/docs/models for more details on what the model metadata control and which combination of parameters are valid. For invalid combinations of parameters (e.g. type = `frequently-bought-together` and optimization_objective = `ctr`), you receive an error 400 if you try to create/update a recommendation with this set of knobs. |
| `last_tune_time` | String | Output only. The timestamp when the latest successful tune finished. |
| `optimization_objective` | String | Optional. The optimization objective e.g. `cvr`. Currently supported values: `ctr`, `cvr`, `revenue-per-order`. If not specified, we choose default based on model type. Default depends on type of recommendation: `recommended-for-you` => `ctr` `others-you-may-like` => `ctr` `frequently-bought-together` => `revenue_per_order` This field together with optimization_objective describe model metadata to use to control model training and serving. See https://cloud.google.com/retail/docs/models for more details on what the model metadata control and which combination of parameters are valid. For invalid combinations of parameters (e.g. type = `frequently-bought-together` and optimization_objective = `ctr`), you receive an error 400 if you try to create/update a recommendation with this set of knobs. |
| `training_state` | String | Optional. The training state that the model is in (e.g. `TRAINING` or `PAUSED`). Since part of the cost of running the service is frequency of training - this can be used to determine when to train model in order to control cost. If not specified: the default value for `CreateModel` method is `TRAINING`. The default value for `UpdateModel` method is to keep the state the same as before. |
| `create_time` | String | Output only. Timestamp the Recommendation Model was created at. |
| `model_features_config` | String | Optional. Additional model features config. |
| `periodic_tuning_state` | String | Optional. The state of periodic tuning. The period we use is 3 months - to do a one-off tune earlier use the `TuneModel` method. Default value is `PERIODIC_TUNING_ENABLED`. |
| `data_state` | String | Output only. The state of data requirements for this model: `DATA_OK` and `DATA_ERROR`. Recommendation model cannot be trained if the data is in `DATA_ERROR` state. Recommendation model can have `DATA_ERROR` state even if serving state is `ACTIVE`: models were trained successfully before, but cannot be refreshed because model no longer has sufficient data for training. |
| `display_name` | String | Required. The display name of the model. Should be human readable, used to display Recommendation Models in the Retail Cloud Console Dashboard. UTF-8 encoded string with limit of 1024 characters. |
| `filtering_option` | String | Optional. If `RECOMMENDATIONS_FILTERING_ENABLED`, recommendation filtering by attributes is enabled for the model. |
| `serving_config_lists` | Vec<String> | Output only. The list of valid serving configs associated with the PageOptimizationConfig. |
| `update_time` | String | Output only. Timestamp the Recommendation Model was last updated. E.g. if a Recommendation Model was paused - this would be the time the pause was initiated. |
| `name` | String | Required. The fully qualified resource name of the model. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/models/{model_id}` catalog_id has char limit of 50. recommendation_model_id has char limit of 40. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create model
model = provider.retail_api.Model {
    parent = "value"  # Required. The parent resource under which to create the model. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}`
}

# Access model outputs
model_id = model.id
model_tuning_operation = model.tuning_operation
model_serving_state = model.serving_state
model_page_optimization_config = model.page_optimization_config
model_type = model.type
model_last_tune_time = model.last_tune_time
model_optimization_objective = model.optimization_objective
model_training_state = model.training_state
model_create_time = model.create_time
model_model_features_config = model.model_features_config
model_periodic_tuning_state = model.periodic_tuning_state
model_data_state = model.data_state
model_display_name = model.display_name
model_filtering_option = model.filtering_option
model_serving_config_lists = model.serving_config_lists
model_update_time = model.update_time
model_name = model.name
```

---


### Merchant_center_account_link

Creates a MerchantCenterAccountLink.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | Output only. Immutable. Full resource name of the Merchant Center Account Link, such as `projects/*/locations/global/catalogs/default_catalog/merchantCenterAccountLinks/merchant_center_account_link`. |
| `feed_label` | String |  | The FeedLabel used to perform filtering. Note: this replaces [region_id](https://developers.google.com/shopping-content/reference/rest/v2.1/products#Product.FIELDS.feed_label). Example value: `US`. Example value: `FeedLabel1`. |
| `language_code` | String |  | Language of the title/description and other string attributes. Use language tags defined by [BCP 47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt). ISO 639-1. This specifies the language of offers in Merchant Center that will be accepted. If empty, no language filtering will be performed. Example value: `en`. |
| `state` | String |  | Output only. Represents the state of the link. |
| `project_id` | String |  | Output only. Google Cloud project ID. |
| `merchant_center_account_id` | String |  | Required. The linked [Merchant center account id](https://developers.google.com/shopping-content/guides/accountstatuses). The account must be a standalone account or a sub-account of a MCA. |
| `feed_filters` | Vec<String> |  | Criteria for the Merchant Center feeds to be ingested via the link. All offers will be ingested if the list is empty. Otherwise the offers will be ingested from selected feeds. |
| `source` | String |  | Optional. An optional arbitrary string that could be used as a tag for tracking link source. |
| `branch_id` | String |  | Required. The branch ID (e.g. 0/1/2) within the catalog that products from merchant_center_account_id are streamed to. When updating this field, an empty value will use the currently configured default branch. However, changing the default branch later on won't change the linked branch here. A single branch ID can only have one linked Merchant Center account ID. |
| `id` | String |  | Output only. Immutable. MerchantCenterAccountLink identifier, which is the final component of name. This field is auto generated and follows the convention: `BranchId_MerchantCenterAccountId`. `projects/*/locations/global/catalogs/default_catalog/merchantCenterAccountLinks/id_1`. |
| `parent` | String | ✅ | Required. The branch resource where this MerchantCenterAccountLink will be created. Format: `projects/{PROJECT_NUMBER}/locations/global/catalogs/{CATALOG_ID}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `merchant_center_account_links` | Vec<String> | The links. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create merchant_center_account_link
merchant_center_account_link = provider.retail_api.Merchant_center_account_link {
    parent = "value"  # Required. The branch resource where this MerchantCenterAccountLink will be created. Format: `projects/{PROJECT_NUMBER}/locations/global/catalogs/{CATALOG_ID}`
}

# Access merchant_center_account_link outputs
merchant_center_account_link_id = merchant_center_account_link.id
merchant_center_account_link_merchant_center_account_links = merchant_center_account_link.merchant_center_account_links
```

---


### Product

Creates a Product.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `expire_time` | String |  | Note that this field is applied in the following ways: * If the Product is already expired when it is uploaded, this product is not indexed for search. * If the Product is not expired when it is uploaded, only the Type.PRIMARY's and Type.COLLECTION's expireTime is respected, and Type.VARIANT's expireTime is not used. In general, we suggest the users to delete the stale products explicitly, instead of using this field to determine staleness. expire_time must be later than available_time and publish_time, otherwise an INVALID_ARGUMENT error is thrown. Corresponding properties: Google Merchant Center property [expiration_date](https://support.google.com/merchants/answer/6324499). |
| `patterns` | Vec<String> |  | The pattern or graphic print of the product. For example, "striped", "polka dot", "paisley". A maximum of 20 values are allowed per Product. Each value must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [pattern](https://support.google.com/merchants/answer/6324483). Schema.org property [Product.pattern](https://schema.org/pattern). |
| `available_quantity` | i64 |  | The available quantity of the item. |
| `fulfillment_info` | Vec<String> |  | Fulfillment information, such as the store IDs for in-store pickup or region IDs for different shipping methods. All the elements must have distinct FulfillmentInfo.type. Otherwise, an INVALID_ARGUMENT error is returned. |
| `price_info` | String |  | Product price and cost information. Corresponding properties: Google Merchant Center property [price](https://support.google.com/merchants/answer/6324371). |
| `categories` | Vec<String> |  | Product categories. This field is repeated for supporting one product belonging to several parallel categories. Strongly recommended using the full path for better search / recommendation quality. To represent full path of category, use '>' sign to separate different hierarchies. If '>' is part of the category name, replace it with other character(s). For example, if a shoes product belongs to both ["Shoes & Accessories" -> "Shoes"] and ["Sports & Fitness" -> "Athletic Clothing" -> "Shoes"], it could be represented as: "categories": [ "Shoes & Accessories > Shoes", "Sports & Fitness > Athletic Clothing > Shoes" ] Must be set for Type.PRIMARY Product otherwise an INVALID_ARGUMENT error is returned. At most 250 values are allowed per Product unless overridden through the Google Cloud console. Empty values are not allowed. Each value must be a UTF-8 encoded string with a length limit of 5,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property google_product_category. Schema.org property [Product.category] (https://schema.org/category). [mc_google_product_category]: https://support.google.com/merchants/answer/6324436 |
| `retrievable_fields` | String |  | Indicates which fields in the Products are returned in SearchResponse. Supported fields for all types: * audience * availability * brands * color_info * conditions * gtin * materials * name * patterns * price_info * rating * sizes * title * uri Supported fields only for Type.PRIMARY and Type.COLLECTION: * categories * description * images Supported fields only for Type.VARIANT: * Only the first image in images To mark attributes as retrievable, include paths of the form "attributes.key" where "key" is the key of a custom attribute, as specified in attributes. For Type.PRIMARY and Type.COLLECTION, the following fields are always returned in SearchResponse by default: * name For Type.VARIANT, the following fields are always returned in by default: * name * color_info Note: Returning more fields in SearchResponse can increase response payload size and serving latency. This field is deprecated. Use the retrievable site-wide control instead. |
| `attributes` | HashMap<String, String> |  | Highly encouraged. Extra product attributes to be included. For example, for products, this could include the store name, vendor, style, color, etc. These are very strong signals for recommendation model, thus we highly recommend providing the attributes here. Features that can take on one of a limited number of possible values. Two types of features can be set are: Textual features. some examples would be the brand/maker of a product, or country of a customer. Numerical features. Some examples would be the height/weight of a product, or age of a customer. For example: `{ "vendor": {"text": ["vendor123", "vendor456"]}, "lengths_cm": {"numbers":[2.3, 15.4]}, "heights_cm": {"numbers":[8.1, 6.4]} }`. This field needs to pass all below criteria, otherwise an INVALID_ARGUMENT error is returned: * Max entries count: 200. * The key must be a UTF-8 encoded string with a length limit of 128 characters. * For indexable attribute, the key must match the pattern: `a-zA-Z0-9*`. For example, `key0LikeThis` or `KEY_1_LIKE_THIS`. * For text attributes, at most 400 values are allowed. Empty values are not allowed. Each value must be a non-empty UTF-8 encoded string with a length limit of 256 characters. * For number attributes, at most 400 values are allowed. |
| `type` | String |  | Immutable. The type of the product. Default to Catalog.product_level_config.ingestion_product_type if unset. |
| `promotions` | Vec<String> |  | The promotions applied to the product. A maximum of 10 values are allowed per Product. Only Promotion.promotion_id will be used, other fields will be ignored if set. |
| `uri` | String |  | Canonical URL directly linking to the product detail page. It is strongly recommended to provide a valid uri for the product, otherwise the service performance could be significantly degraded. This field must be a UTF-8 encoded string with a length limit of 5,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [link](https://support.google.com/merchants/answer/6324416). Schema.org property [Offer.url](https://schema.org/url). |
| `id` | String |  | Immutable. Product identifier, which is the final component of name. For example, this field is "id_1", if name is `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/id_1`. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [id](https://support.google.com/merchants/answer/6324405). Schema.org property [Product.sku](https://schema.org/sku). |
| `primary_product_id` | String |  | Variant group identifier. Must be an id, with the same parent branch with this product. Otherwise, an error is thrown. For Type.PRIMARY Products, this field can only be empty or set to the same value as id. For VARIANT Products, this field cannot be empty. A maximum of 2,000 products are allowed to share the same Type.PRIMARY Product. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [item_group_id](https://support.google.com/merchants/answer/6324507). Schema.org property [Product.inProductGroupWithID](https://schema.org/inProductGroupWithID). |
| `tags` | Vec<String> |  | Custom tags associated with the product. At most 250 values are allowed per Product. This value must be a UTF-8 encoded string with a length limit of 1,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. This tag can be used for filtering recommendation results by passing the tag as part of the PredictRequest.filter. Corresponding properties: Google Merchant Center property [custom_label_0–4](https://support.google.com/merchants/answer/6324473). |
| `title` | String |  | Required. Product title. This field must be a UTF-8 encoded string with a length limit of 1,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [title](https://support.google.com/merchants/answer/6324415). Schema.org property [Product.name](https://schema.org/name). |
| `materials` | Vec<String> |  | The material of the product. For example, "leather", "wooden". A maximum of 20 values are allowed. Each value must be a UTF-8 encoded string with a length limit of 200 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [material](https://support.google.com/merchants/answer/6324410). Schema.org property [Product.material](https://schema.org/material). |
| `local_inventories` | Vec<String> |  | Output only. A list of local inventories specific to different places. This field can be managed by ProductService.AddLocalInventories and ProductService.RemoveLocalInventories APIs if fine-grained, high-volume updates are necessary. |
| `name` | String |  | Immutable. Full resource name of the product, such as `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/product_id`. |
| `variants` | Vec<String> |  | Output only. Product variants grouped together on primary product which share similar product attributes. It's automatically grouped by primary_product_id for all the product variants. Only populated for Type.PRIMARY Products. Note: This field is OUTPUT_ONLY for ProductService.GetProduct. Do not set this field in API requests. |
| `available_time` | String |  | The timestamp when this Product becomes available for SearchService.Search. Note that this is only applicable to Type.PRIMARY and Type.COLLECTION, and ignored for Type.VARIANT. |
| `ttl` | String |  | Input only. The TTL (time to live) of the product. Note that this is only applicable to Type.PRIMARY and Type.COLLECTION, and ignored for Type.VARIANT. In general, we suggest the users to delete the stale products explicitly, instead of using this field to determine staleness. If it is set, it must be a non-negative value, and expire_time is set as current timestamp plus ttl. The derived expire_time is returned in the output and ttl is left blank when retrieving the Product. If it is set, the product is not available for SearchService.Search after current timestamp plus ttl. However, the product can still be retrieved by ProductService.GetProduct and ProductService.ListProducts. |
| `collection_member_ids` | Vec<String> |  | The id of the collection members when type is Type.COLLECTION. Non-existent product ids are allowed. The type of the members must be either Type.PRIMARY or Type.VARIANT otherwise an INVALID_ARGUMENT error is thrown. Should not set it for other types. A maximum of 1000 values are allowed. Otherwise, an INVALID_ARGUMENT error is return. |
| `language_code` | String |  | Language of the title/description and other string attributes. Use language tags defined by [BCP 47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt). For product prediction, this field is ignored and the model automatically detects the text language. The Product can include text in different languages, but duplicating Products to provide text in multiple languages can result in degraded model performance. For product search this field is in use. It defaults to "en-US" if unset. |
| `publish_time` | String |  | The timestamp when the product is published by the retailer for the first time, which indicates the freshness of the products. Note that this field is different from available_time, given it purely describes product freshness regardless of when it is available on search and recommendation. |
| `gtin` | String |  | The Global Trade Item Number (GTIN) of the product. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. This field must be a Unigram. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [gtin](https://support.google.com/merchants/answer/6324461). Schema.org property [Product.isbn](https://schema.org/isbn), [Product.gtin8](https://schema.org/gtin8), [Product.gtin12](https://schema.org/gtin12), [Product.gtin13](https://schema.org/gtin13), or [Product.gtin14](https://schema.org/gtin14). If the value is not a valid GTIN, an INVALID_ARGUMENT error is returned. |
| `color_info` | String |  | The color of the product. Corresponding properties: Google Merchant Center property [color](https://support.google.com/merchants/answer/6324487). Schema.org property [Product.color](https://schema.org/color). |
| `brands` | Vec<String> |  | The brands of the product. A maximum of 30 brands are allowed unless overridden through the Google Cloud console. Each brand must be a UTF-8 encoded string with a length limit of 1,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [brand](https://support.google.com/merchants/answer/6324351). Schema.org property [Product.brand](https://schema.org/brand). |
| `conditions` | Vec<String> |  | The condition of the product. Strongly encouraged to use the standard values: "new", "refurbished", "used". A maximum of 1 value is allowed per Product. Each value must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [condition](https://support.google.com/merchants/answer/6324469). Schema.org property [Offer.itemCondition](https://schema.org/itemCondition). |
| `sizes` | Vec<String> |  | The size of the product. To represent different size systems or size types, consider using this format: [[[size_system:]size_type:]size_value]. For example, in "US:MENS:M", "US" represents size system; "MENS" represents size type; "M" represents size value. In "GIRLS:27", size system is empty; "GIRLS" represents size type; "27" represents size value. In "32 inches", both size system and size type are empty, while size value is "32 inches". A maximum of 20 values are allowed per Product. Each value must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [size](https://support.google.com/merchants/answer/6324492), [size_type](https://support.google.com/merchants/answer/6324497), and [size_system](https://support.google.com/merchants/answer/6324502). Schema.org property [Product.size](https://schema.org/size). |
| `description` | String |  | Product description. This field must be a UTF-8 encoded string with a length limit of 5,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [description](https://support.google.com/merchants/answer/6324468). Schema.org property [Product.description](https://schema.org/description). |
| `rating` | String |  | The rating of this product. |
| `images` | Vec<String> |  | Product images for the product. We highly recommend putting the main image first. A maximum of 300 images are allowed. Corresponding properties: Google Merchant Center property [image_link](https://support.google.com/merchants/answer/6324350). Schema.org property [Product.image](https://schema.org/image). |
| `availability` | String |  | The online availability of the Product. Default to Availability.IN_STOCK. For primary products with variants set the availability of the primary as Availability.OUT_OF_STOCK and set the true availability at the variant level. This way the primary product will be considered "in stock" as long as it has at least one variant in stock. For primary products with no variants set the true availability at the primary level. Corresponding properties: Google Merchant Center property [availability](https://support.google.com/merchants/answer/6324448). Schema.org property [Offer.availability](https://schema.org/availability). |
| `audience` | String |  | The target group associated with a given audience (e.g. male, veterans, car owners, musicians, etc.) of the product. |
| `parent` | String | ✅ | Required. The parent catalog resource name, such as `projects/*/locations/global/catalogs/default_catalog/branches/default_branch`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `expire_time` | String | Note that this field is applied in the following ways: * If the Product is already expired when it is uploaded, this product is not indexed for search. * If the Product is not expired when it is uploaded, only the Type.PRIMARY's and Type.COLLECTION's expireTime is respected, and Type.VARIANT's expireTime is not used. In general, we suggest the users to delete the stale products explicitly, instead of using this field to determine staleness. expire_time must be later than available_time and publish_time, otherwise an INVALID_ARGUMENT error is thrown. Corresponding properties: Google Merchant Center property [expiration_date](https://support.google.com/merchants/answer/6324499). |
| `patterns` | Vec<String> | The pattern or graphic print of the product. For example, "striped", "polka dot", "paisley". A maximum of 20 values are allowed per Product. Each value must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [pattern](https://support.google.com/merchants/answer/6324483). Schema.org property [Product.pattern](https://schema.org/pattern). |
| `available_quantity` | i64 | The available quantity of the item. |
| `fulfillment_info` | Vec<String> | Fulfillment information, such as the store IDs for in-store pickup or region IDs for different shipping methods. All the elements must have distinct FulfillmentInfo.type. Otherwise, an INVALID_ARGUMENT error is returned. |
| `price_info` | String | Product price and cost information. Corresponding properties: Google Merchant Center property [price](https://support.google.com/merchants/answer/6324371). |
| `categories` | Vec<String> | Product categories. This field is repeated for supporting one product belonging to several parallel categories. Strongly recommended using the full path for better search / recommendation quality. To represent full path of category, use '>' sign to separate different hierarchies. If '>' is part of the category name, replace it with other character(s). For example, if a shoes product belongs to both ["Shoes & Accessories" -> "Shoes"] and ["Sports & Fitness" -> "Athletic Clothing" -> "Shoes"], it could be represented as: "categories": [ "Shoes & Accessories > Shoes", "Sports & Fitness > Athletic Clothing > Shoes" ] Must be set for Type.PRIMARY Product otherwise an INVALID_ARGUMENT error is returned. At most 250 values are allowed per Product unless overridden through the Google Cloud console. Empty values are not allowed. Each value must be a UTF-8 encoded string with a length limit of 5,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property google_product_category. Schema.org property [Product.category] (https://schema.org/category). [mc_google_product_category]: https://support.google.com/merchants/answer/6324436 |
| `retrievable_fields` | String | Indicates which fields in the Products are returned in SearchResponse. Supported fields for all types: * audience * availability * brands * color_info * conditions * gtin * materials * name * patterns * price_info * rating * sizes * title * uri Supported fields only for Type.PRIMARY and Type.COLLECTION: * categories * description * images Supported fields only for Type.VARIANT: * Only the first image in images To mark attributes as retrievable, include paths of the form "attributes.key" where "key" is the key of a custom attribute, as specified in attributes. For Type.PRIMARY and Type.COLLECTION, the following fields are always returned in SearchResponse by default: * name For Type.VARIANT, the following fields are always returned in by default: * name * color_info Note: Returning more fields in SearchResponse can increase response payload size and serving latency. This field is deprecated. Use the retrievable site-wide control instead. |
| `attributes` | HashMap<String, String> | Highly encouraged. Extra product attributes to be included. For example, for products, this could include the store name, vendor, style, color, etc. These are very strong signals for recommendation model, thus we highly recommend providing the attributes here. Features that can take on one of a limited number of possible values. Two types of features can be set are: Textual features. some examples would be the brand/maker of a product, or country of a customer. Numerical features. Some examples would be the height/weight of a product, or age of a customer. For example: `{ "vendor": {"text": ["vendor123", "vendor456"]}, "lengths_cm": {"numbers":[2.3, 15.4]}, "heights_cm": {"numbers":[8.1, 6.4]} }`. This field needs to pass all below criteria, otherwise an INVALID_ARGUMENT error is returned: * Max entries count: 200. * The key must be a UTF-8 encoded string with a length limit of 128 characters. * For indexable attribute, the key must match the pattern: `a-zA-Z0-9*`. For example, `key0LikeThis` or `KEY_1_LIKE_THIS`. * For text attributes, at most 400 values are allowed. Empty values are not allowed. Each value must be a non-empty UTF-8 encoded string with a length limit of 256 characters. * For number attributes, at most 400 values are allowed. |
| `type` | String | Immutable. The type of the product. Default to Catalog.product_level_config.ingestion_product_type if unset. |
| `promotions` | Vec<String> | The promotions applied to the product. A maximum of 10 values are allowed per Product. Only Promotion.promotion_id will be used, other fields will be ignored if set. |
| `uri` | String | Canonical URL directly linking to the product detail page. It is strongly recommended to provide a valid uri for the product, otherwise the service performance could be significantly degraded. This field must be a UTF-8 encoded string with a length limit of 5,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [link](https://support.google.com/merchants/answer/6324416). Schema.org property [Offer.url](https://schema.org/url). |
| `id` | String | Immutable. Product identifier, which is the final component of name. For example, this field is "id_1", if name is `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/id_1`. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [id](https://support.google.com/merchants/answer/6324405). Schema.org property [Product.sku](https://schema.org/sku). |
| `primary_product_id` | String | Variant group identifier. Must be an id, with the same parent branch with this product. Otherwise, an error is thrown. For Type.PRIMARY Products, this field can only be empty or set to the same value as id. For VARIANT Products, this field cannot be empty. A maximum of 2,000 products are allowed to share the same Type.PRIMARY Product. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [item_group_id](https://support.google.com/merchants/answer/6324507). Schema.org property [Product.inProductGroupWithID](https://schema.org/inProductGroupWithID). |
| `tags` | Vec<String> | Custom tags associated with the product. At most 250 values are allowed per Product. This value must be a UTF-8 encoded string with a length limit of 1,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. This tag can be used for filtering recommendation results by passing the tag as part of the PredictRequest.filter. Corresponding properties: Google Merchant Center property [custom_label_0–4](https://support.google.com/merchants/answer/6324473). |
| `title` | String | Required. Product title. This field must be a UTF-8 encoded string with a length limit of 1,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [title](https://support.google.com/merchants/answer/6324415). Schema.org property [Product.name](https://schema.org/name). |
| `materials` | Vec<String> | The material of the product. For example, "leather", "wooden". A maximum of 20 values are allowed. Each value must be a UTF-8 encoded string with a length limit of 200 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [material](https://support.google.com/merchants/answer/6324410). Schema.org property [Product.material](https://schema.org/material). |
| `local_inventories` | Vec<String> | Output only. A list of local inventories specific to different places. This field can be managed by ProductService.AddLocalInventories and ProductService.RemoveLocalInventories APIs if fine-grained, high-volume updates are necessary. |
| `name` | String | Immutable. Full resource name of the product, such as `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/product_id`. |
| `variants` | Vec<String> | Output only. Product variants grouped together on primary product which share similar product attributes. It's automatically grouped by primary_product_id for all the product variants. Only populated for Type.PRIMARY Products. Note: This field is OUTPUT_ONLY for ProductService.GetProduct. Do not set this field in API requests. |
| `available_time` | String | The timestamp when this Product becomes available for SearchService.Search. Note that this is only applicable to Type.PRIMARY and Type.COLLECTION, and ignored for Type.VARIANT. |
| `ttl` | String | Input only. The TTL (time to live) of the product. Note that this is only applicable to Type.PRIMARY and Type.COLLECTION, and ignored for Type.VARIANT. In general, we suggest the users to delete the stale products explicitly, instead of using this field to determine staleness. If it is set, it must be a non-negative value, and expire_time is set as current timestamp plus ttl. The derived expire_time is returned in the output and ttl is left blank when retrieving the Product. If it is set, the product is not available for SearchService.Search after current timestamp plus ttl. However, the product can still be retrieved by ProductService.GetProduct and ProductService.ListProducts. |
| `collection_member_ids` | Vec<String> | The id of the collection members when type is Type.COLLECTION. Non-existent product ids are allowed. The type of the members must be either Type.PRIMARY or Type.VARIANT otherwise an INVALID_ARGUMENT error is thrown. Should not set it for other types. A maximum of 1000 values are allowed. Otherwise, an INVALID_ARGUMENT error is return. |
| `language_code` | String | Language of the title/description and other string attributes. Use language tags defined by [BCP 47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt). For product prediction, this field is ignored and the model automatically detects the text language. The Product can include text in different languages, but duplicating Products to provide text in multiple languages can result in degraded model performance. For product search this field is in use. It defaults to "en-US" if unset. |
| `publish_time` | String | The timestamp when the product is published by the retailer for the first time, which indicates the freshness of the products. Note that this field is different from available_time, given it purely describes product freshness regardless of when it is available on search and recommendation. |
| `gtin` | String | The Global Trade Item Number (GTIN) of the product. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. This field must be a Unigram. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [gtin](https://support.google.com/merchants/answer/6324461). Schema.org property [Product.isbn](https://schema.org/isbn), [Product.gtin8](https://schema.org/gtin8), [Product.gtin12](https://schema.org/gtin12), [Product.gtin13](https://schema.org/gtin13), or [Product.gtin14](https://schema.org/gtin14). If the value is not a valid GTIN, an INVALID_ARGUMENT error is returned. |
| `color_info` | String | The color of the product. Corresponding properties: Google Merchant Center property [color](https://support.google.com/merchants/answer/6324487). Schema.org property [Product.color](https://schema.org/color). |
| `brands` | Vec<String> | The brands of the product. A maximum of 30 brands are allowed unless overridden through the Google Cloud console. Each brand must be a UTF-8 encoded string with a length limit of 1,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [brand](https://support.google.com/merchants/answer/6324351). Schema.org property [Product.brand](https://schema.org/brand). |
| `conditions` | Vec<String> | The condition of the product. Strongly encouraged to use the standard values: "new", "refurbished", "used". A maximum of 1 value is allowed per Product. Each value must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [condition](https://support.google.com/merchants/answer/6324469). Schema.org property [Offer.itemCondition](https://schema.org/itemCondition). |
| `sizes` | Vec<String> | The size of the product. To represent different size systems or size types, consider using this format: [[[size_system:]size_type:]size_value]. For example, in "US:MENS:M", "US" represents size system; "MENS" represents size type; "M" represents size value. In "GIRLS:27", size system is empty; "GIRLS" represents size type; "27" represents size value. In "32 inches", both size system and size type are empty, while size value is "32 inches". A maximum of 20 values are allowed per Product. Each value must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [size](https://support.google.com/merchants/answer/6324492), [size_type](https://support.google.com/merchants/answer/6324497), and [size_system](https://support.google.com/merchants/answer/6324502). Schema.org property [Product.size](https://schema.org/size). |
| `description` | String | Product description. This field must be a UTF-8 encoded string with a length limit of 5,000 characters. Otherwise, an INVALID_ARGUMENT error is returned. Corresponding properties: Google Merchant Center property [description](https://support.google.com/merchants/answer/6324468). Schema.org property [Product.description](https://schema.org/description). |
| `rating` | String | The rating of this product. |
| `images` | Vec<String> | Product images for the product. We highly recommend putting the main image first. A maximum of 300 images are allowed. Corresponding properties: Google Merchant Center property [image_link](https://support.google.com/merchants/answer/6324350). Schema.org property [Product.image](https://schema.org/image). |
| `availability` | String | The online availability of the Product. Default to Availability.IN_STOCK. For primary products with variants set the availability of the primary as Availability.OUT_OF_STOCK and set the true availability at the variant level. This way the primary product will be considered "in stock" as long as it has at least one variant in stock. For primary products with no variants set the true availability at the primary level. Corresponding properties: Google Merchant Center property [availability](https://support.google.com/merchants/answer/6324448). Schema.org property [Offer.availability](https://schema.org/availability). |
| `audience` | String | The target group associated with a given audience (e.g. male, veterans, car owners, musicians, etc.) of the product. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create product
product = provider.retail_api.Product {
    parent = "value"  # Required. The parent catalog resource name, such as `projects/*/locations/global/catalogs/default_catalog/branches/default_branch`.
}

# Access product outputs
product_id = product.id
product_expire_time = product.expire_time
product_patterns = product.patterns
product_available_quantity = product.available_quantity
product_fulfillment_info = product.fulfillment_info
product_price_info = product.price_info
product_categories = product.categories
product_retrievable_fields = product.retrievable_fields
product_attributes = product.attributes
product_type = product.type
product_promotions = product.promotions
product_uri = product.uri
product_id = product.id
product_primary_product_id = product.primary_product_id
product_tags = product.tags
product_title = product.title
product_materials = product.materials
product_local_inventories = product.local_inventories
product_name = product.name
product_variants = product.variants
product_available_time = product.available_time
product_ttl = product.ttl
product_collection_member_ids = product.collection_member_ids
product_language_code = product.language_code
product_publish_time = product.publish_time
product_gtin = product.gtin
product_color_info = product.color_info
product_brands = product.brands
product_conditions = product.conditions
product_sizes = product.sizes
product_description = product.description
product_rating = product.rating
product_images = product.images
product_availability = product.availability
product_audience = product.audience
```

---


### Branche

Retrieves a Branch.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `is_default` | bool | Output only. Indicates whether this branch is set as the default branch of its parent catalog. |
| `last_product_import_time` | String | Output only. Timestamp of last import through ProductService.ImportProducts. Empty value means no import has been made to this branch. |
| `product_count_stats` | Vec<String> | Output only. Statistics for number of products in the branch, provided for different scopes. This field is not populated in BranchView.BASIC view. |
| `quality_metrics` | Vec<String> | Output only. The quality metrics measured among products of this branch. See QualityMetric.requirement_key for supported metrics. Metrics could be missing if failed to retrieve. This field is not populated in BranchView.BASIC view. |
| `display_name` | String | Output only. Human readable name of the branch to display in the UI. |
| `name` | String | Immutable. Full resource name of the branch, such as `projects/*/locations/global/catalogs/default_catalog/branches/branch_id`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access branche outputs
branche_id = branche.id
branche_is_default = branche.is_default
branche_last_product_import_time = branche.last_product_import_time
branche_product_count_stats = branche.product_count_stats
branche_quality_metrics = branche.quality_metrics
branche_display_name = branche.display_name
branche_name = branche.name
```

---


### Serving_config

Creates a ServingConfig. A maximum of 100 ServingConfigs are allowed in a Catalog, otherwise a FAILED_PRECONDITION error is returned.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `diversity_type` | String |  | What kind of diversity to use - data driven or rule based. If unset, the server behavior defaults to RULE_BASED_DIVERSITY. |
| `model_id` | String |  | The id of the model in the same Catalog to use at serving time. Currently only RecommendationModels are supported: https://cloud.google.com/retail/recommendations-ai/docs/create-models Can be changed but only to a compatible model (e.g. others-you-may-like CTR to others-you-may-like CVR). Required when solution_types is SOLUTION_TYPE_RECOMMENDATION. |
| `ignore_recs_denylist` | bool |  | When the flag is enabled, the products in the denylist will not be filtered out in the recommendation filtering results. |
| `enable_category_filter_level` | String |  | Whether to add additional category filters on the `similar-items` model. If not specified, we enable it by default. Allowed values are: * `no-category-match`: No additional filtering of original results from the model and the customer's filters. * `relaxed-category-match`: Only keep results with categories that match at least one item categories in the PredictRequests's context item. * If customer also sends filters in the PredictRequest, then the results will satisfy both conditions (user given and category match). Can only be set if solution_types is SOLUTION_TYPE_RECOMMENDATION. |
| `ignore_control_ids` | Vec<String> |  | Condition ignore specifications. If multiple ignore conditions match, all matching ignore controls in the list will execute. - Order does not matter. - Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `personalization_spec` | String |  | The specification for personalization spec. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. Notice that if both ServingConfig.personalization_spec and SearchRequest.personalization_spec are set. SearchRequest.personalization_spec will override ServingConfig.personalization_spec. |
| `filter_control_ids` | Vec<String> |  | Condition filter specifications. If a product matches multiple conditions in the specifications, filters from these specifications are all applied and combined via the AND operator. Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `do_not_associate_control_ids` | Vec<String> |  | Condition do not associate specifications. If multiple do not associate conditions match, all matching do not associate controls in the list will execute. - Order does not matter. - Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `diversity_level` | String |  | How much diversity to use in recommendation model results e.g. `medium-diversity` or `high-diversity`. Currently supported values: * `no-diversity` * `low-diversity` * `medium-diversity` * `high-diversity` * `auto-diversity` If not specified, we choose default based on recommendation model type. Default value: `no-diversity`. Can only be set if solution_types is SOLUTION_TYPE_RECOMMENDATION. |
| `name` | String |  | Immutable. Fully qualified name `projects/*/locations/global/catalogs/*/servingConfig/*` |
| `price_reranking_level` | String |  | How much price ranking we want in serving results. Price reranking causes product items with a similar recommendation probability to be ordered by price, with the highest-priced items first. This setting could result in a decrease in click-through and conversion rates. Allowed values are: * `no-price-reranking` * `low-price-reranking` * `medium-price-reranking` * `high-price-reranking` If not specified, we choose default based on model type. Default value: `no-price-reranking`. Can only be set if solution_types is SOLUTION_TYPE_RECOMMENDATION. |
| `twoway_synonyms_control_ids` | Vec<String> |  | Condition synonyms specifications. If multiple syonyms conditions match, all matching synonyms control in the list will execute. Order of controls in the list will not matter. Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `redirect_control_ids` | Vec<String> |  | Condition redirect specifications. Only the first triggered redirect action is applied, even if multiple apply. Maximum number of specifications is 1000. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `solution_types` | Vec<String> |  | Required. Immutable. Specifies the solution types that a serving config can be associated with. Currently we support setting only one type of solution. |
| `replacement_control_ids` | Vec<String> |  | Condition replacement specifications. - Applied according to the order in the list. - A previously replaced term can not be re-replaced. - Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `display_name` | String |  | Required. The human readable serving config display name. Used in Retail UI. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `boost_control_ids` | Vec<String> |  | Condition boost specifications. If a product matches multiple conditions in the specifications, boost scores from these specifications are all applied and combined in a non-linear way. Maximum number of specifications is 100. Notice that if both ServingConfig.boost_control_ids and SearchRequest.boost_spec are set, the boost conditions from both places are evaluated. If a search request matches multiple boost conditions, the final boost score is equal to the sum of the boost scores from all matched boost conditions. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `facet_control_ids` | Vec<String> |  | Facet specifications for faceted search. If empty, no facets are returned. The ids refer to the ids of Control resources with only the Facet control set. These controls are assumed to be in the same Catalog as the ServingConfig. A maximum of 100 values are allowed. Otherwise, an INVALID_ARGUMENT error is returned. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `oneway_synonyms_control_ids` | Vec<String> |  | Condition oneway synonyms specifications. If multiple oneway synonyms conditions match, all matching oneway synonyms controls in the list will execute. Order of controls in the list will not matter. Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `dynamic_facet_spec` | String |  | The specification for dynamically generated facets. Notice that only textual facets can be dynamically generated. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `parent` | String | ✅ | Required. Full resource name of parent. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `diversity_type` | String | What kind of diversity to use - data driven or rule based. If unset, the server behavior defaults to RULE_BASED_DIVERSITY. |
| `model_id` | String | The id of the model in the same Catalog to use at serving time. Currently only RecommendationModels are supported: https://cloud.google.com/retail/recommendations-ai/docs/create-models Can be changed but only to a compatible model (e.g. others-you-may-like CTR to others-you-may-like CVR). Required when solution_types is SOLUTION_TYPE_RECOMMENDATION. |
| `ignore_recs_denylist` | bool | When the flag is enabled, the products in the denylist will not be filtered out in the recommendation filtering results. |
| `enable_category_filter_level` | String | Whether to add additional category filters on the `similar-items` model. If not specified, we enable it by default. Allowed values are: * `no-category-match`: No additional filtering of original results from the model and the customer's filters. * `relaxed-category-match`: Only keep results with categories that match at least one item categories in the PredictRequests's context item. * If customer also sends filters in the PredictRequest, then the results will satisfy both conditions (user given and category match). Can only be set if solution_types is SOLUTION_TYPE_RECOMMENDATION. |
| `ignore_control_ids` | Vec<String> | Condition ignore specifications. If multiple ignore conditions match, all matching ignore controls in the list will execute. - Order does not matter. - Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `personalization_spec` | String | The specification for personalization spec. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. Notice that if both ServingConfig.personalization_spec and SearchRequest.personalization_spec are set. SearchRequest.personalization_spec will override ServingConfig.personalization_spec. |
| `filter_control_ids` | Vec<String> | Condition filter specifications. If a product matches multiple conditions in the specifications, filters from these specifications are all applied and combined via the AND operator. Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `do_not_associate_control_ids` | Vec<String> | Condition do not associate specifications. If multiple do not associate conditions match, all matching do not associate controls in the list will execute. - Order does not matter. - Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `diversity_level` | String | How much diversity to use in recommendation model results e.g. `medium-diversity` or `high-diversity`. Currently supported values: * `no-diversity` * `low-diversity` * `medium-diversity` * `high-diversity` * `auto-diversity` If not specified, we choose default based on recommendation model type. Default value: `no-diversity`. Can only be set if solution_types is SOLUTION_TYPE_RECOMMENDATION. |
| `name` | String | Immutable. Fully qualified name `projects/*/locations/global/catalogs/*/servingConfig/*` |
| `price_reranking_level` | String | How much price ranking we want in serving results. Price reranking causes product items with a similar recommendation probability to be ordered by price, with the highest-priced items first. This setting could result in a decrease in click-through and conversion rates. Allowed values are: * `no-price-reranking` * `low-price-reranking` * `medium-price-reranking` * `high-price-reranking` If not specified, we choose default based on model type. Default value: `no-price-reranking`. Can only be set if solution_types is SOLUTION_TYPE_RECOMMENDATION. |
| `twoway_synonyms_control_ids` | Vec<String> | Condition synonyms specifications. If multiple syonyms conditions match, all matching synonyms control in the list will execute. Order of controls in the list will not matter. Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `redirect_control_ids` | Vec<String> | Condition redirect specifications. Only the first triggered redirect action is applied, even if multiple apply. Maximum number of specifications is 1000. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `solution_types` | Vec<String> | Required. Immutable. Specifies the solution types that a serving config can be associated with. Currently we support setting only one type of solution. |
| `replacement_control_ids` | Vec<String> | Condition replacement specifications. - Applied according to the order in the list. - A previously replaced term can not be re-replaced. - Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `display_name` | String | Required. The human readable serving config display name. Used in Retail UI. This field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `boost_control_ids` | Vec<String> | Condition boost specifications. If a product matches multiple conditions in the specifications, boost scores from these specifications are all applied and combined in a non-linear way. Maximum number of specifications is 100. Notice that if both ServingConfig.boost_control_ids and SearchRequest.boost_spec are set, the boost conditions from both places are evaluated. If a search request matches multiple boost conditions, the final boost score is equal to the sum of the boost scores from all matched boost conditions. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `facet_control_ids` | Vec<String> | Facet specifications for faceted search. If empty, no facets are returned. The ids refer to the ids of Control resources with only the Facet control set. These controls are assumed to be in the same Catalog as the ServingConfig. A maximum of 100 values are allowed. Otherwise, an INVALID_ARGUMENT error is returned. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `oneway_synonyms_control_ids` | Vec<String> | Condition oneway synonyms specifications. If multiple oneway synonyms conditions match, all matching oneway synonyms controls in the list will execute. Order of controls in the list will not matter. Maximum number of specifications is 100. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |
| `dynamic_facet_spec` | String | The specification for dynamically generated facets. Notice that only textual facets can be dynamically generated. Can only be set if solution_types is SOLUTION_TYPE_SEARCH. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create serving_config
serving_config = provider.retail_api.Serving_config {
    parent = "value"  # Required. Full resource name of parent. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}`
}

# Access serving_config outputs
serving_config_id = serving_config.id
serving_config_diversity_type = serving_config.diversity_type
serving_config_model_id = serving_config.model_id
serving_config_ignore_recs_denylist = serving_config.ignore_recs_denylist
serving_config_enable_category_filter_level = serving_config.enable_category_filter_level
serving_config_ignore_control_ids = serving_config.ignore_control_ids
serving_config_personalization_spec = serving_config.personalization_spec
serving_config_filter_control_ids = serving_config.filter_control_ids
serving_config_do_not_associate_control_ids = serving_config.do_not_associate_control_ids
serving_config_diversity_level = serving_config.diversity_level
serving_config_name = serving_config.name
serving_config_price_reranking_level = serving_config.price_reranking_level
serving_config_twoway_synonyms_control_ids = serving_config.twoway_synonyms_control_ids
serving_config_redirect_control_ids = serving_config.redirect_control_ids
serving_config_solution_types = serving_config.solution_types
serving_config_replacement_control_ids = serving_config.replacement_control_ids
serving_config_display_name = serving_config.display_name
serving_config_boost_control_ids = serving_config.boost_control_ids
serving_config_facet_control_ids = serving_config.facet_control_ids
serving_config_oneway_synonyms_control_ids = serving_config.oneway_synonyms_control_ids
serving_config_dynamic_facet_spec = serving_config.dynamic_facet_spec
```

---


### Generative_question

Allows management of multiple questions.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `requests` | Vec<String> |  | Required. The updates question configs. |
| `parent` | String | ✅ | Optional. Resource name of the parent catalog. Format: projects/{project}/locations/{location}/catalogs/{catalog} |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `generative_question_configs` | Vec<String> | All the questions for a given catalog. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create generative_question
generative_question = provider.retail_api.Generative_question {
    parent = "value"  # Optional. Resource name of the parent catalog. Format: projects/{project}/locations/{location}/catalogs/{catalog}
}

# Access generative_question outputs
generative_question_id = generative_question.id
generative_question_generative_question_configs = generative_question.generative_question_configs
```

---


### Project

The method enrolls a solution of type Retail Search into a project. The Recommendations AI solution type is enrolled by default when your project enables Retail API, so you don't need to call the enrollSolution method for recommendations.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `solution` | String |  | Required. Solution to enroll. |
| `project` | String | ✅ | Required. Full resource name of parent. Format: `projects/{project_number_or_id}` |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `alert_policies` | Vec<String> | Alert policies for a customer. They must be unique by [AlertPolicy.alert_group] |
| `name` | String | Required. Immutable. The name of the AlertConfig singleton resource. Format: projects/*/alertConfig |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create project
project = provider.retail_api.Project {
    project = "value"  # Required. Full resource name of parent. Format: `projects/{project_number_or_id}`
}

# Access project outputs
project_id = project.id
project_alert_policies = project.alert_policies
project_name = project.name
```

---


### Placement

Performs a conversational search. This feature is only available for users who have Conversational Search enabled.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `user_labels` | HashMap<String, String> |  | Optional. The user labels applied to a resource must meet the following requirements: * Each resource can have multiple labels, up to a maximum of 64. * Each label must be a key-value pair. * Keys have a minimum length of 1 character and a maximum length of 63 characters and cannot be empty. Values can be empty and have a maximum length of 63 characters. * Keys and values can contain only lowercase letters, numeric characters, underscores, and dashes. All characters must use UTF-8 encoding, and international characters are allowed. * The key portion of a label must be unique. However, you can use the same key with multiple resources. * Keys must start with a lowercase letter or international character. See [Google Cloud Document](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements) for more details. |
| `visitor_id` | String |  | Required. A unique identifier for tracking visitors. For example, this could be implemented with an HTTP cookie, which should be able to uniquely identify a visitor on a single device. This unique identifier should not change if the visitor logs in or out of the website. This should be the same identifier as UserEvent.visitor_id. The field must be a UTF-8 encoded string with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error is returned. |
| `query` | String |  | Optional. Raw search query to be searched for. If this field is empty, the request is considered a category browsing request. |
| `page_categories` | Vec<String> |  | Optional. The categories associated with a category page. Must be set for category navigation queries to achieve good search quality. The format should be the same as UserEvent.page_categories; To represent full path of category, use '>' sign to separate different hierarchies. If '>' is part of the category name, replace it with other character(s). Category pages include special pages such as sales or promotions. For instance, a special sale page may have the category hierarchy: "pageCategories" : ["Sales > 2017 Black Friday Deals"]. |
| `conversational_filtering_spec` | String |  | Optional. This field specifies all conversational filtering related parameters. |
| `conversation_id` | String |  | Optional. This field specifies the conversation id, which maintains the state of the conversation between client side and server side. Use the value from the previous ConversationalSearchResponse.conversation_id. For the initial request, this should be empty. |
| `branch` | String |  | Required. The branch resource name, such as `projects/*/locations/global/catalogs/default_catalog/branches/0`. Use "default_branch" as the branch ID or leave this field empty, to search products under the default branch. |
| `safety_settings` | Vec<String> |  | Optional. The safety settings to be applied to the generated content. |
| `user_info` | String |  | Optional. User information. |
| `search_params` | String |  | Optional. Search parameters. |
| `placement` | String | ✅ | Required. The resource name of the search engine placement, such as `projects/*/locations/global/catalogs/default_catalog/placements/default_search` or `projects/*/locations/global/catalogs/default_catalog/servingConfigs/default_serving_config` This field is used to identify the serving config name and the set of models that will be used to make the search. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create placement
placement = provider.retail_api.Placement {
    placement = "value"  # Required. The resource name of the search engine placement, such as `projects/*/locations/global/catalogs/default_catalog/placements/default_search` or `projects/*/locations/global/catalogs/default_catalog/servingConfigs/default_serving_config` This field is used to identify the serving config name and the set of models that will be used to make the search.
}

```

---


### Retail_project

Accepts service terms for this project. By making requests to this API, you agree to the terms of service linked below. https://cloud.google.com/retail/data-use-terms

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `project` | String | ✅ | Required. Full resource name of the project. Format: `projects/{project_number_or_id}/retailProject` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create retail_project
retail_project = provider.retail_api.Retail_project {
    project = "value"  # Required. Full resource name of the project. Format: `projects/{project_number_or_id}/retailProject`
}

```

---


### Attributes_config

Adds the specified CatalogAttribute to the AttributesConfig. If the CatalogAttribute to add already exists, an ALREADY_EXISTS error is returned.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `catalog_attribute` | String |  | Required. The CatalogAttribute to add. |
| `attributes_config` | String | ✅ | Required. Full AttributesConfig resource name. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/attributesConfig` |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create attributes_config
attributes_config = provider.retail_api.Attributes_config {
    attributes_config = "value"  # Required. Full AttributesConfig resource name. Format: `projects/{project_number}/locations/{location_id}/catalogs/{catalog_id}/attributesConfig`
}

```

---


### Completion_data

Bulk import of processed completion dataset. Request processing is asynchronous. Partial updating is not supported. The operation is successfully finished only after the imported suggestions are indexed successfully and ready for serving. The process takes hours. This feature is only available for users who have Retail Search enabled. Enable Retail Search on Cloud Console before using this feature.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `input_config` | String |  | Required. The desired input location of the data. |
| `notification_pubsub_topic` | String |  | Pub/Sub topic for receiving notification. If this field is set, when the import is finished, a notification is sent to specified Pub/Sub topic. The message data is JSON string of a Operation. Format of the Pub/Sub topic is `projects/{project}/topics/{topic}`. |
| `parent` | String | ✅ | Required. The catalog which the suggestions dataset belongs to. Format: `projects/1234/locations/global/catalogs/default_catalog`. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create completion_data
completion_data = provider.retail_api.Completion_data {
    parent = "value"  # Required. The catalog which the suggestions dataset belongs to. Format: `projects/1234/locations/global/catalogs/default_catalog`.
}

```

---


### Operation

Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `metadata` | HashMap<String, String> | Service-specific metadata associated with the operation. It typically contains progress information and common metadata such as create time. Some services might not provide such metadata. Any method that returns a long-running operation should document the metadata type, if any. |
| `name` | String | The server-assigned name, which is only unique within the same service that originally returns it. If you use the default HTTP mapping, the `name` should be a resource name ending with `operations/{unique_id}`. |
| `done` | bool | If the value is `false`, it means the operation is still in progress. If `true`, the operation is completed, and either `error` or `response` is available. |
| `error` | String | The error result of the operation in case of failure or cancellation. |
| `response` | HashMap<String, String> | The normal, successful response of the operation. If the original method returns no data on success, such as `Delete`, the response is `google.protobuf.Empty`. If the original method is standard `Get`/`Create`/`Update`, the response should be the resource. For other methods, the response should have the type `XxxResponse`, where `Xxx` is the original method name. For example, if the original method name is `TakeSnapshot()`, the inferred response type is `TakeSnapshotResponse`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access operation outputs
operation_id = operation.id
operation_metadata = operation.metadata
operation_name = operation.name
operation_done = operation.done
operation_error = operation.error
operation_response = operation.response
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple user_event resources
user_event_0 = provider.retail_api.User_event {
    parent = "value-0"
}
user_event_1 = provider.retail_api.User_event {
    parent = "value-1"
}
user_event_2 = provider.retail_api.User_event {
    parent = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    user_event = provider.retail_api.User_event {
        parent = "production-value"
    }
```

---

## Related Documentation

- [GCP Retail_api Documentation](https://cloud.google.com/retail_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
