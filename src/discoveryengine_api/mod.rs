//! Discoveryengine_api service for Gcp provider
//!
//! This module handles all discoveryengine_api resources and their CRUD operations.

use hemmer_core::Result;
use hemmer_provider::{ResourceInput, ResourceOutput, ResourcePlan};

/// Discoveryengine_api service handler
pub struct Discoveryengine_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Discoveryengine_apiService<'a> {
    /// Create a new service handler
    pub fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Plan changes to a resource
    pub async fn plan_resource(
        &self,
        resource_name: &str,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        match resource_name {
            "custom_model" => self.plan_custom_model(current_state, desired_input).await,
            "identity_mapping_store" => {
                self.plan_identity_mapping_store(current_state, desired_input)
                    .await
            }
            "location" => self.plan_location(current_state, desired_input).await,
            "answer" => self.plan_answer(current_state, desired_input).await,
            "operation" => self.plan_operation(current_state, desired_input).await,
            "sitemap" => self.plan_sitemap(current_state, desired_input).await,
            "user_store" => self.plan_user_store(current_state, desired_input).await,
            "completion_config" => {
                self.plan_completion_config(current_state, desired_input)
                    .await
            }
            "session" => self.plan_session(current_state, desired_input).await,
            "user_license" => self.plan_user_license(current_state, desired_input).await,
            "document" => self.plan_document(current_state, desired_input).await,
            "control" => self.plan_control(current_state, desired_input).await,
            "data_store" => self.plan_data_store(current_state, desired_input).await,
            "media" => self.plan_media(current_state, desired_input).await,
            "assistant" => self.plan_assistant(current_state, desired_input).await,
            "ranking_config" => self.plan_ranking_config(current_state, desired_input).await,
            "schema" => self.plan_schema(current_state, desired_input).await,
            "license_config" => self.plan_license_config(current_state, desired_input).await,
            "conversation" => self.plan_conversation(current_state, desired_input).await,
            "engine" => self.plan_engine(current_state, desired_input).await,
            "branche" => self.plan_branche(current_state, desired_input).await,
            "collection" => self.plan_collection(current_state, desired_input).await,
            "target_site" => self.plan_target_site(current_state, desired_input).await,
            "suggestion_deny_list_entrie" => {
                self.plan_suggestion_deny_list_entrie(current_state, desired_input)
                    .await
            }
            "serving_config" => self.plan_serving_config(current_state, desired_input).await,
            "grounding_config" => {
                self.plan_grounding_config(current_state, desired_input)
                    .await
            }
            "cmek_config" => self.plan_cmek_config(current_state, desired_input).await,
            "user_event" => self.plan_user_event(current_state, desired_input).await,
            "completion_suggestion" => {
                self.plan_completion_suggestion(current_state, desired_input)
                    .await
            }
            "site_search_engine" => {
                self.plan_site_search_engine(current_state, desired_input)
                    .await
            }
            "project" => self.plan_project(current_state, desired_input).await,
            "branche" => self.plan_branche(current_state, desired_input).await,
            "sample_querie" => self.plan_sample_querie(current_state, desired_input).await,
            "evaluation" => self.plan_evaluation(current_state, desired_input).await,
            "grounding_config" => {
                self.plan_grounding_config(current_state, desired_input)
                    .await
            }
            "media" => self.plan_media(current_state, desired_input).await,
            "file" => self.plan_file(current_state, desired_input).await,
            "target_site" => self.plan_target_site(current_state, desired_input).await,
            "operation" => self.plan_operation(current_state, desired_input).await,
            "schema" => self.plan_schema(current_state, desired_input).await,
            "document" => self.plan_document(current_state, desired_input).await,
            "user_event" => self.plan_user_event(current_state, desired_input).await,
            "site_search_engine" => {
                self.plan_site_search_engine(current_state, desired_input)
                    .await
            }
            "identity_mapping_store" => {
                self.plan_identity_mapping_store(current_state, desired_input)
                    .await
            }
            "agent" => self.plan_agent(current_state, desired_input).await,
            "sitemap" => self.plan_sitemap(current_state, desired_input).await,
            "completion_config" => {
                self.plan_completion_config(current_state, desired_input)
                    .await
            }
            "answer" => self.plan_answer(current_state, desired_input).await,
            "assistant" => self.plan_assistant(current_state, desired_input).await,
            "user_license" => self.plan_user_license(current_state, desired_input).await,
            "session" => self.plan_session(current_state, desired_input).await,
            "source" => self.plan_source(current_state, desired_input).await,
            "canned_querie" => self.plan_canned_querie(current_state, desired_input).await,
            "chunk" => self.plan_chunk(current_state, desired_input).await,
            "billing_account_license_config" => {
                self.plan_billing_account_license_config(current_state, desired_input)
                    .await
            }
            "data_connector" => self.plan_data_connector(current_state, desired_input).await,
            "data_store" => self.plan_data_store(current_state, desired_input).await,
            "audio_overview" => self.plan_audio_overview(current_state, desired_input).await,
            "requirement" => self.plan_requirement(current_state, desired_input).await,
            "authorization" => self.plan_authorization(current_state, desired_input).await,
            "license_config" => self.plan_license_config(current_state, desired_input).await,
            "analytic" => self.plan_analytic(current_state, desired_input).await,
            "control" => self.plan_control(current_state, desired_input).await,
            "custom_model" => self.plan_custom_model(current_state, desired_input).await,
            "serving_config" => self.plan_serving_config(current_state, desired_input).await,
            "collection" => self.plan_collection(current_state, desired_input).await,
            "engine" => self.plan_engine(current_state, desired_input).await,
            "connector_run" => self.plan_connector_run(current_state, desired_input).await,
            "widget_config" => self.plan_widget_config(current_state, desired_input).await,
            "project" => self.plan_project(current_state, desired_input).await,
            "conversation" => self.plan_conversation(current_state, desired_input).await,
            "completion_suggestion" => {
                self.plan_completion_suggestion(current_state, desired_input)
                    .await
            }
            "notebook" => self.plan_notebook(current_state, desired_input).await,
            "ranking_config" => self.plan_ranking_config(current_state, desired_input).await,
            "suggestion_deny_list_entrie" => {
                self.plan_suggestion_deny_list_entrie(current_state, desired_input)
                    .await
            }
            "cmek_config" => self.plan_cmek_config(current_state, desired_input).await,
            "location" => self.plan_location(current_state, desired_input).await,
            "user_store" => self.plan_user_store(current_state, desired_input).await,
            "sample_query_set" => {
                self.plan_sample_query_set(current_state, desired_input)
                    .await
            }
            "evaluation" => self.plan_evaluation(current_state, desired_input).await,
            "ranking_config" => self.plan_ranking_config(current_state, desired_input).await,
            "answer" => self.plan_answer(current_state, desired_input).await,
            "completion_suggestion" => {
                self.plan_completion_suggestion(current_state, desired_input)
                    .await
            }
            "target_site" => self.plan_target_site(current_state, desired_input).await,
            "suggestion_deny_list_entrie" => {
                self.plan_suggestion_deny_list_entrie(current_state, desired_input)
                    .await
            }
            "identity_mapping_store" => {
                self.plan_identity_mapping_store(current_state, desired_input)
                    .await
            }
            "user_store" => self.plan_user_store(current_state, desired_input).await,
            "custom_model" => self.plan_custom_model(current_state, desired_input).await,
            "data_store" => self.plan_data_store(current_state, desired_input).await,
            "branche" => self.plan_branche(current_state, desired_input).await,
            "sample_query_set" => {
                self.plan_sample_query_set(current_state, desired_input)
                    .await
            }
            "media" => self.plan_media(current_state, desired_input).await,
            "site_search_engine" => {
                self.plan_site_search_engine(current_state, desired_input)
                    .await
            }
            "schema" => self.plan_schema(current_state, desired_input).await,
            "serving_config" => self.plan_serving_config(current_state, desired_input).await,
            "sample_querie" => self.plan_sample_querie(current_state, desired_input).await,
            "grounding_config" => {
                self.plan_grounding_config(current_state, desired_input)
                    .await
            }
            "cmek_config" => self.plan_cmek_config(current_state, desired_input).await,
            "license_config" => self.plan_license_config(current_state, desired_input).await,
            "engine" => self.plan_engine(current_state, desired_input).await,
            "sitemap" => self.plan_sitemap(current_state, desired_input).await,
            "operation" => self.plan_operation(current_state, desired_input).await,
            "user_event" => self.plan_user_event(current_state, desired_input).await,
            "user_license" => self.plan_user_license(current_state, desired_input).await,
            "control" => self.plan_control(current_state, desired_input).await,
            "session" => self.plan_session(current_state, desired_input).await,
            "location" => self.plan_location(current_state, desired_input).await,
            "assistant" => self.plan_assistant(current_state, desired_input).await,
            "completion_config" => {
                self.plan_completion_config(current_state, desired_input)
                    .await
            }
            "conversation" => self.plan_conversation(current_state, desired_input).await,
            "document" => self.plan_document(current_state, desired_input).await,
            "project" => self.plan_project(current_state, desired_input).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "discoveryengine_api", resource_name
            ))),
        }
    }

    /// Create a new resource
    pub async fn create_resource(
        &self,
        resource_name: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        match resource_name {
            "custom_model" => self.create_custom_model(input).await,
            "identity_mapping_store" => self.create_identity_mapping_store(input).await,
            "location" => self.create_location(input).await,
            "answer" => self.create_answer(input).await,
            "operation" => self.create_operation(input).await,
            "sitemap" => self.create_sitemap(input).await,
            "user_store" => self.create_user_store(input).await,
            "completion_config" => self.create_completion_config(input).await,
            "session" => self.create_session(input).await,
            "user_license" => self.create_user_license(input).await,
            "document" => self.create_document(input).await,
            "control" => self.create_control(input).await,
            "data_store" => self.create_data_store(input).await,
            "media" => self.create_media(input).await,
            "assistant" => self.create_assistant(input).await,
            "ranking_config" => self.create_ranking_config(input).await,
            "schema" => self.create_schema(input).await,
            "license_config" => self.create_license_config(input).await,
            "conversation" => self.create_conversation(input).await,
            "engine" => self.create_engine(input).await,
            "branche" => self.create_branche(input).await,
            "collection" => self.create_collection(input).await,
            "target_site" => self.create_target_site(input).await,
            "suggestion_deny_list_entrie" => self.create_suggestion_deny_list_entrie(input).await,
            "serving_config" => self.create_serving_config(input).await,
            "grounding_config" => self.create_grounding_config(input).await,
            "cmek_config" => self.create_cmek_config(input).await,
            "user_event" => self.create_user_event(input).await,
            "completion_suggestion" => self.create_completion_suggestion(input).await,
            "site_search_engine" => self.create_site_search_engine(input).await,
            "project" => self.create_project(input).await,
            "branche" => self.create_branche(input).await,
            "sample_querie" => self.create_sample_querie(input).await,
            "evaluation" => self.create_evaluation(input).await,
            "grounding_config" => self.create_grounding_config(input).await,
            "media" => self.create_media(input).await,
            "file" => self.create_file(input).await,
            "target_site" => self.create_target_site(input).await,
            "operation" => self.create_operation(input).await,
            "schema" => self.create_schema(input).await,
            "document" => self.create_document(input).await,
            "user_event" => self.create_user_event(input).await,
            "site_search_engine" => self.create_site_search_engine(input).await,
            "identity_mapping_store" => self.create_identity_mapping_store(input).await,
            "agent" => self.create_agent(input).await,
            "sitemap" => self.create_sitemap(input).await,
            "completion_config" => self.create_completion_config(input).await,
            "answer" => self.create_answer(input).await,
            "assistant" => self.create_assistant(input).await,
            "user_license" => self.create_user_license(input).await,
            "session" => self.create_session(input).await,
            "source" => self.create_source(input).await,
            "canned_querie" => self.create_canned_querie(input).await,
            "chunk" => self.create_chunk(input).await,
            "billing_account_license_config" => {
                self.create_billing_account_license_config(input).await
            }
            "data_connector" => self.create_data_connector(input).await,
            "data_store" => self.create_data_store(input).await,
            "audio_overview" => self.create_audio_overview(input).await,
            "requirement" => self.create_requirement(input).await,
            "authorization" => self.create_authorization(input).await,
            "license_config" => self.create_license_config(input).await,
            "analytic" => self.create_analytic(input).await,
            "control" => self.create_control(input).await,
            "custom_model" => self.create_custom_model(input).await,
            "serving_config" => self.create_serving_config(input).await,
            "collection" => self.create_collection(input).await,
            "engine" => self.create_engine(input).await,
            "connector_run" => self.create_connector_run(input).await,
            "widget_config" => self.create_widget_config(input).await,
            "project" => self.create_project(input).await,
            "conversation" => self.create_conversation(input).await,
            "completion_suggestion" => self.create_completion_suggestion(input).await,
            "notebook" => self.create_notebook(input).await,
            "ranking_config" => self.create_ranking_config(input).await,
            "suggestion_deny_list_entrie" => self.create_suggestion_deny_list_entrie(input).await,
            "cmek_config" => self.create_cmek_config(input).await,
            "location" => self.create_location(input).await,
            "user_store" => self.create_user_store(input).await,
            "sample_query_set" => self.create_sample_query_set(input).await,
            "evaluation" => self.create_evaluation(input).await,
            "ranking_config" => self.create_ranking_config(input).await,
            "answer" => self.create_answer(input).await,
            "completion_suggestion" => self.create_completion_suggestion(input).await,
            "target_site" => self.create_target_site(input).await,
            "suggestion_deny_list_entrie" => self.create_suggestion_deny_list_entrie(input).await,
            "identity_mapping_store" => self.create_identity_mapping_store(input).await,
            "user_store" => self.create_user_store(input).await,
            "custom_model" => self.create_custom_model(input).await,
            "data_store" => self.create_data_store(input).await,
            "branche" => self.create_branche(input).await,
            "sample_query_set" => self.create_sample_query_set(input).await,
            "media" => self.create_media(input).await,
            "site_search_engine" => self.create_site_search_engine(input).await,
            "schema" => self.create_schema(input).await,
            "serving_config" => self.create_serving_config(input).await,
            "sample_querie" => self.create_sample_querie(input).await,
            "grounding_config" => self.create_grounding_config(input).await,
            "cmek_config" => self.create_cmek_config(input).await,
            "license_config" => self.create_license_config(input).await,
            "engine" => self.create_engine(input).await,
            "sitemap" => self.create_sitemap(input).await,
            "operation" => self.create_operation(input).await,
            "user_event" => self.create_user_event(input).await,
            "user_license" => self.create_user_license(input).await,
            "control" => self.create_control(input).await,
            "session" => self.create_session(input).await,
            "location" => self.create_location(input).await,
            "assistant" => self.create_assistant(input).await,
            "completion_config" => self.create_completion_config(input).await,
            "conversation" => self.create_conversation(input).await,
            "document" => self.create_document(input).await,
            "project" => self.create_project(input).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "discoveryengine_api", resource_name
            ))),
        }
    }

    /// Read resource state
    pub async fn read_resource(&self, resource_name: &str, id: &str) -> Result<ResourceOutput> {
        match resource_name {
            "custom_model" => self.read_custom_model(id).await,
            "identity_mapping_store" => self.read_identity_mapping_store(id).await,
            "location" => self.read_location(id).await,
            "answer" => self.read_answer(id).await,
            "operation" => self.read_operation(id).await,
            "sitemap" => self.read_sitemap(id).await,
            "user_store" => self.read_user_store(id).await,
            "completion_config" => self.read_completion_config(id).await,
            "session" => self.read_session(id).await,
            "user_license" => self.read_user_license(id).await,
            "document" => self.read_document(id).await,
            "control" => self.read_control(id).await,
            "data_store" => self.read_data_store(id).await,
            "media" => self.read_media(id).await,
            "assistant" => self.read_assistant(id).await,
            "ranking_config" => self.read_ranking_config(id).await,
            "schema" => self.read_schema(id).await,
            "license_config" => self.read_license_config(id).await,
            "conversation" => self.read_conversation(id).await,
            "engine" => self.read_engine(id).await,
            "branche" => self.read_branche(id).await,
            "collection" => self.read_collection(id).await,
            "target_site" => self.read_target_site(id).await,
            "suggestion_deny_list_entrie" => self.read_suggestion_deny_list_entrie(id).await,
            "serving_config" => self.read_serving_config(id).await,
            "grounding_config" => self.read_grounding_config(id).await,
            "cmek_config" => self.read_cmek_config(id).await,
            "user_event" => self.read_user_event(id).await,
            "completion_suggestion" => self.read_completion_suggestion(id).await,
            "site_search_engine" => self.read_site_search_engine(id).await,
            "project" => self.read_project(id).await,
            "branche" => self.read_branche(id).await,
            "sample_querie" => self.read_sample_querie(id).await,
            "evaluation" => self.read_evaluation(id).await,
            "grounding_config" => self.read_grounding_config(id).await,
            "media" => self.read_media(id).await,
            "file" => self.read_file(id).await,
            "target_site" => self.read_target_site(id).await,
            "operation" => self.read_operation(id).await,
            "schema" => self.read_schema(id).await,
            "document" => self.read_document(id).await,
            "user_event" => self.read_user_event(id).await,
            "site_search_engine" => self.read_site_search_engine(id).await,
            "identity_mapping_store" => self.read_identity_mapping_store(id).await,
            "agent" => self.read_agent(id).await,
            "sitemap" => self.read_sitemap(id).await,
            "completion_config" => self.read_completion_config(id).await,
            "answer" => self.read_answer(id).await,
            "assistant" => self.read_assistant(id).await,
            "user_license" => self.read_user_license(id).await,
            "session" => self.read_session(id).await,
            "source" => self.read_source(id).await,
            "canned_querie" => self.read_canned_querie(id).await,
            "chunk" => self.read_chunk(id).await,
            "billing_account_license_config" => self.read_billing_account_license_config(id).await,
            "data_connector" => self.read_data_connector(id).await,
            "data_store" => self.read_data_store(id).await,
            "audio_overview" => self.read_audio_overview(id).await,
            "requirement" => self.read_requirement(id).await,
            "authorization" => self.read_authorization(id).await,
            "license_config" => self.read_license_config(id).await,
            "analytic" => self.read_analytic(id).await,
            "control" => self.read_control(id).await,
            "custom_model" => self.read_custom_model(id).await,
            "serving_config" => self.read_serving_config(id).await,
            "collection" => self.read_collection(id).await,
            "engine" => self.read_engine(id).await,
            "connector_run" => self.read_connector_run(id).await,
            "widget_config" => self.read_widget_config(id).await,
            "project" => self.read_project(id).await,
            "conversation" => self.read_conversation(id).await,
            "completion_suggestion" => self.read_completion_suggestion(id).await,
            "notebook" => self.read_notebook(id).await,
            "ranking_config" => self.read_ranking_config(id).await,
            "suggestion_deny_list_entrie" => self.read_suggestion_deny_list_entrie(id).await,
            "cmek_config" => self.read_cmek_config(id).await,
            "location" => self.read_location(id).await,
            "user_store" => self.read_user_store(id).await,
            "sample_query_set" => self.read_sample_query_set(id).await,
            "evaluation" => self.read_evaluation(id).await,
            "ranking_config" => self.read_ranking_config(id).await,
            "answer" => self.read_answer(id).await,
            "completion_suggestion" => self.read_completion_suggestion(id).await,
            "target_site" => self.read_target_site(id).await,
            "suggestion_deny_list_entrie" => self.read_suggestion_deny_list_entrie(id).await,
            "identity_mapping_store" => self.read_identity_mapping_store(id).await,
            "user_store" => self.read_user_store(id).await,
            "custom_model" => self.read_custom_model(id).await,
            "data_store" => self.read_data_store(id).await,
            "branche" => self.read_branche(id).await,
            "sample_query_set" => self.read_sample_query_set(id).await,
            "media" => self.read_media(id).await,
            "site_search_engine" => self.read_site_search_engine(id).await,
            "schema" => self.read_schema(id).await,
            "serving_config" => self.read_serving_config(id).await,
            "sample_querie" => self.read_sample_querie(id).await,
            "grounding_config" => self.read_grounding_config(id).await,
            "cmek_config" => self.read_cmek_config(id).await,
            "license_config" => self.read_license_config(id).await,
            "engine" => self.read_engine(id).await,
            "sitemap" => self.read_sitemap(id).await,
            "operation" => self.read_operation(id).await,
            "user_event" => self.read_user_event(id).await,
            "user_license" => self.read_user_license(id).await,
            "control" => self.read_control(id).await,
            "session" => self.read_session(id).await,
            "location" => self.read_location(id).await,
            "assistant" => self.read_assistant(id).await,
            "completion_config" => self.read_completion_config(id).await,
            "conversation" => self.read_conversation(id).await,
            "document" => self.read_document(id).await,
            "project" => self.read_project(id).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "discoveryengine_api", resource_name
            ))),
        }
    }

    /// Update an existing resource
    pub async fn update_resource(
        &self,
        resource_name: &str,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        match resource_name {
            "custom_model" => self.update_custom_model(id, input).await,
            "identity_mapping_store" => self.update_identity_mapping_store(id, input).await,
            "location" => self.update_location(id, input).await,
            "answer" => self.update_answer(id, input).await,
            "operation" => self.update_operation(id, input).await,
            "sitemap" => self.update_sitemap(id, input).await,
            "user_store" => self.update_user_store(id, input).await,
            "completion_config" => self.update_completion_config(id, input).await,
            "session" => self.update_session(id, input).await,
            "user_license" => self.update_user_license(id, input).await,
            "document" => self.update_document(id, input).await,
            "control" => self.update_control(id, input).await,
            "data_store" => self.update_data_store(id, input).await,
            "media" => self.update_media(id, input).await,
            "assistant" => self.update_assistant(id, input).await,
            "ranking_config" => self.update_ranking_config(id, input).await,
            "schema" => self.update_schema(id, input).await,
            "license_config" => self.update_license_config(id, input).await,
            "conversation" => self.update_conversation(id, input).await,
            "engine" => self.update_engine(id, input).await,
            "branche" => self.update_branche(id, input).await,
            "collection" => self.update_collection(id, input).await,
            "target_site" => self.update_target_site(id, input).await,
            "suggestion_deny_list_entrie" => {
                self.update_suggestion_deny_list_entrie(id, input).await
            }
            "serving_config" => self.update_serving_config(id, input).await,
            "grounding_config" => self.update_grounding_config(id, input).await,
            "cmek_config" => self.update_cmek_config(id, input).await,
            "user_event" => self.update_user_event(id, input).await,
            "completion_suggestion" => self.update_completion_suggestion(id, input).await,
            "site_search_engine" => self.update_site_search_engine(id, input).await,
            "project" => self.update_project(id, input).await,
            "branche" => self.update_branche(id, input).await,
            "sample_querie" => self.update_sample_querie(id, input).await,
            "evaluation" => self.update_evaluation(id, input).await,
            "grounding_config" => self.update_grounding_config(id, input).await,
            "media" => self.update_media(id, input).await,
            "file" => self.update_file(id, input).await,
            "target_site" => self.update_target_site(id, input).await,
            "operation" => self.update_operation(id, input).await,
            "schema" => self.update_schema(id, input).await,
            "document" => self.update_document(id, input).await,
            "user_event" => self.update_user_event(id, input).await,
            "site_search_engine" => self.update_site_search_engine(id, input).await,
            "identity_mapping_store" => self.update_identity_mapping_store(id, input).await,
            "agent" => self.update_agent(id, input).await,
            "sitemap" => self.update_sitemap(id, input).await,
            "completion_config" => self.update_completion_config(id, input).await,
            "answer" => self.update_answer(id, input).await,
            "assistant" => self.update_assistant(id, input).await,
            "user_license" => self.update_user_license(id, input).await,
            "session" => self.update_session(id, input).await,
            "source" => self.update_source(id, input).await,
            "canned_querie" => self.update_canned_querie(id, input).await,
            "chunk" => self.update_chunk(id, input).await,
            "billing_account_license_config" => {
                self.update_billing_account_license_config(id, input).await
            }
            "data_connector" => self.update_data_connector(id, input).await,
            "data_store" => self.update_data_store(id, input).await,
            "audio_overview" => self.update_audio_overview(id, input).await,
            "requirement" => self.update_requirement(id, input).await,
            "authorization" => self.update_authorization(id, input).await,
            "license_config" => self.update_license_config(id, input).await,
            "analytic" => self.update_analytic(id, input).await,
            "control" => self.update_control(id, input).await,
            "custom_model" => self.update_custom_model(id, input).await,
            "serving_config" => self.update_serving_config(id, input).await,
            "collection" => self.update_collection(id, input).await,
            "engine" => self.update_engine(id, input).await,
            "connector_run" => self.update_connector_run(id, input).await,
            "widget_config" => self.update_widget_config(id, input).await,
            "project" => self.update_project(id, input).await,
            "conversation" => self.update_conversation(id, input).await,
            "completion_suggestion" => self.update_completion_suggestion(id, input).await,
            "notebook" => self.update_notebook(id, input).await,
            "ranking_config" => self.update_ranking_config(id, input).await,
            "suggestion_deny_list_entrie" => {
                self.update_suggestion_deny_list_entrie(id, input).await
            }
            "cmek_config" => self.update_cmek_config(id, input).await,
            "location" => self.update_location(id, input).await,
            "user_store" => self.update_user_store(id, input).await,
            "sample_query_set" => self.update_sample_query_set(id, input).await,
            "evaluation" => self.update_evaluation(id, input).await,
            "ranking_config" => self.update_ranking_config(id, input).await,
            "answer" => self.update_answer(id, input).await,
            "completion_suggestion" => self.update_completion_suggestion(id, input).await,
            "target_site" => self.update_target_site(id, input).await,
            "suggestion_deny_list_entrie" => {
                self.update_suggestion_deny_list_entrie(id, input).await
            }
            "identity_mapping_store" => self.update_identity_mapping_store(id, input).await,
            "user_store" => self.update_user_store(id, input).await,
            "custom_model" => self.update_custom_model(id, input).await,
            "data_store" => self.update_data_store(id, input).await,
            "branche" => self.update_branche(id, input).await,
            "sample_query_set" => self.update_sample_query_set(id, input).await,
            "media" => self.update_media(id, input).await,
            "site_search_engine" => self.update_site_search_engine(id, input).await,
            "schema" => self.update_schema(id, input).await,
            "serving_config" => self.update_serving_config(id, input).await,
            "sample_querie" => self.update_sample_querie(id, input).await,
            "grounding_config" => self.update_grounding_config(id, input).await,
            "cmek_config" => self.update_cmek_config(id, input).await,
            "license_config" => self.update_license_config(id, input).await,
            "engine" => self.update_engine(id, input).await,
            "sitemap" => self.update_sitemap(id, input).await,
            "operation" => self.update_operation(id, input).await,
            "user_event" => self.update_user_event(id, input).await,
            "user_license" => self.update_user_license(id, input).await,
            "control" => self.update_control(id, input).await,
            "session" => self.update_session(id, input).await,
            "location" => self.update_location(id, input).await,
            "assistant" => self.update_assistant(id, input).await,
            "completion_config" => self.update_completion_config(id, input).await,
            "conversation" => self.update_conversation(id, input).await,
            "document" => self.update_document(id, input).await,
            "project" => self.update_project(id, input).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "discoveryengine_api", resource_name
            ))),
        }
    }

    /// Delete a resource
    pub async fn delete_resource(&self, resource_name: &str, id: &str) -> Result<()> {
        match resource_name {
            "custom_model" => self.delete_custom_model(id).await,
            "identity_mapping_store" => self.delete_identity_mapping_store(id).await,
            "location" => self.delete_location(id).await,
            "answer" => self.delete_answer(id).await,
            "operation" => self.delete_operation(id).await,
            "sitemap" => self.delete_sitemap(id).await,
            "user_store" => self.delete_user_store(id).await,
            "completion_config" => self.delete_completion_config(id).await,
            "session" => self.delete_session(id).await,
            "user_license" => self.delete_user_license(id).await,
            "document" => self.delete_document(id).await,
            "control" => self.delete_control(id).await,
            "data_store" => self.delete_data_store(id).await,
            "media" => self.delete_media(id).await,
            "assistant" => self.delete_assistant(id).await,
            "ranking_config" => self.delete_ranking_config(id).await,
            "schema" => self.delete_schema(id).await,
            "license_config" => self.delete_license_config(id).await,
            "conversation" => self.delete_conversation(id).await,
            "engine" => self.delete_engine(id).await,
            "branche" => self.delete_branche(id).await,
            "collection" => self.delete_collection(id).await,
            "target_site" => self.delete_target_site(id).await,
            "suggestion_deny_list_entrie" => self.delete_suggestion_deny_list_entrie(id).await,
            "serving_config" => self.delete_serving_config(id).await,
            "grounding_config" => self.delete_grounding_config(id).await,
            "cmek_config" => self.delete_cmek_config(id).await,
            "user_event" => self.delete_user_event(id).await,
            "completion_suggestion" => self.delete_completion_suggestion(id).await,
            "site_search_engine" => self.delete_site_search_engine(id).await,
            "project" => self.delete_project(id).await,
            "branche" => self.delete_branche(id).await,
            "sample_querie" => self.delete_sample_querie(id).await,
            "evaluation" => self.delete_evaluation(id).await,
            "grounding_config" => self.delete_grounding_config(id).await,
            "media" => self.delete_media(id).await,
            "file" => self.delete_file(id).await,
            "target_site" => self.delete_target_site(id).await,
            "operation" => self.delete_operation(id).await,
            "schema" => self.delete_schema(id).await,
            "document" => self.delete_document(id).await,
            "user_event" => self.delete_user_event(id).await,
            "site_search_engine" => self.delete_site_search_engine(id).await,
            "identity_mapping_store" => self.delete_identity_mapping_store(id).await,
            "agent" => self.delete_agent(id).await,
            "sitemap" => self.delete_sitemap(id).await,
            "completion_config" => self.delete_completion_config(id).await,
            "answer" => self.delete_answer(id).await,
            "assistant" => self.delete_assistant(id).await,
            "user_license" => self.delete_user_license(id).await,
            "session" => self.delete_session(id).await,
            "source" => self.delete_source(id).await,
            "canned_querie" => self.delete_canned_querie(id).await,
            "chunk" => self.delete_chunk(id).await,
            "billing_account_license_config" => {
                self.delete_billing_account_license_config(id).await
            }
            "data_connector" => self.delete_data_connector(id).await,
            "data_store" => self.delete_data_store(id).await,
            "audio_overview" => self.delete_audio_overview(id).await,
            "requirement" => self.delete_requirement(id).await,
            "authorization" => self.delete_authorization(id).await,
            "license_config" => self.delete_license_config(id).await,
            "analytic" => self.delete_analytic(id).await,
            "control" => self.delete_control(id).await,
            "custom_model" => self.delete_custom_model(id).await,
            "serving_config" => self.delete_serving_config(id).await,
            "collection" => self.delete_collection(id).await,
            "engine" => self.delete_engine(id).await,
            "connector_run" => self.delete_connector_run(id).await,
            "widget_config" => self.delete_widget_config(id).await,
            "project" => self.delete_project(id).await,
            "conversation" => self.delete_conversation(id).await,
            "completion_suggestion" => self.delete_completion_suggestion(id).await,
            "notebook" => self.delete_notebook(id).await,
            "ranking_config" => self.delete_ranking_config(id).await,
            "suggestion_deny_list_entrie" => self.delete_suggestion_deny_list_entrie(id).await,
            "cmek_config" => self.delete_cmek_config(id).await,
            "location" => self.delete_location(id).await,
            "user_store" => self.delete_user_store(id).await,
            "sample_query_set" => self.delete_sample_query_set(id).await,
            "evaluation" => self.delete_evaluation(id).await,
            "ranking_config" => self.delete_ranking_config(id).await,
            "answer" => self.delete_answer(id).await,
            "completion_suggestion" => self.delete_completion_suggestion(id).await,
            "target_site" => self.delete_target_site(id).await,
            "suggestion_deny_list_entrie" => self.delete_suggestion_deny_list_entrie(id).await,
            "identity_mapping_store" => self.delete_identity_mapping_store(id).await,
            "user_store" => self.delete_user_store(id).await,
            "custom_model" => self.delete_custom_model(id).await,
            "data_store" => self.delete_data_store(id).await,
            "branche" => self.delete_branche(id).await,
            "sample_query_set" => self.delete_sample_query_set(id).await,
            "media" => self.delete_media(id).await,
            "site_search_engine" => self.delete_site_search_engine(id).await,
            "schema" => self.delete_schema(id).await,
            "serving_config" => self.delete_serving_config(id).await,
            "sample_querie" => self.delete_sample_querie(id).await,
            "grounding_config" => self.delete_grounding_config(id).await,
            "cmek_config" => self.delete_cmek_config(id).await,
            "license_config" => self.delete_license_config(id).await,
            "engine" => self.delete_engine(id).await,
            "sitemap" => self.delete_sitemap(id).await,
            "operation" => self.delete_operation(id).await,
            "user_event" => self.delete_user_event(id).await,
            "user_license" => self.delete_user_license(id).await,
            "control" => self.delete_control(id).await,
            "session" => self.delete_session(id).await,
            "location" => self.delete_location(id).await,
            "assistant" => self.delete_assistant(id).await,
            "completion_config" => self.delete_completion_config(id).await,
            "conversation" => self.delete_conversation(id).await,
            "document" => self.delete_document(id).await,
            "project" => self.delete_project(id).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "discoveryengine_api", resource_name
            ))),
        }
    }

    // ========================================================================
    // Resource-specific CRUD implementations
    // ========================================================================

    // ------------------------------------------------------------------------
    // Custom_model resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a custom_model resource
    async fn plan_custom_model(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new custom_model resource
    async fn create_custom_model(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a custom_model resource
    async fn read_custom_model(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a custom_model resource
    async fn update_custom_model(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a custom_model resource
    async fn delete_custom_model(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Identity_mapping_store resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a identity_mapping_store resource
    async fn plan_identity_mapping_store(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new identity_mapping_store resource
    async fn create_identity_mapping_store(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a identity_mapping_store resource
    async fn read_identity_mapping_store(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a identity_mapping_store resource
    async fn update_identity_mapping_store(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a identity_mapping_store resource
    async fn delete_identity_mapping_store(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Location resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a location resource
    async fn plan_location(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new location resource
    async fn create_location(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a location resource
    async fn read_location(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a location resource
    async fn update_location(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a location resource
    async fn delete_location(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Answer resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a answer resource
    async fn plan_answer(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new answer resource
    async fn create_answer(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a answer resource
    async fn read_answer(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a answer resource
    async fn update_answer(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a answer resource
    async fn delete_answer(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Operation resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a operation resource
    async fn plan_operation(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new operation resource
    async fn create_operation(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a operation resource
    async fn read_operation(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a operation resource
    async fn update_operation(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a operation resource
    async fn delete_operation(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Sitemap resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a sitemap resource
    async fn plan_sitemap(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new sitemap resource
    async fn create_sitemap(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a sitemap resource
    async fn read_sitemap(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a sitemap resource
    async fn update_sitemap(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a sitemap resource
    async fn delete_sitemap(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // User_store resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a user_store resource
    async fn plan_user_store(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new user_store resource
    async fn create_user_store(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a user_store resource
    async fn read_user_store(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a user_store resource
    async fn update_user_store(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a user_store resource
    async fn delete_user_store(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Completion_config resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a completion_config resource
    async fn plan_completion_config(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new completion_config resource
    async fn create_completion_config(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a completion_config resource
    async fn read_completion_config(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a completion_config resource
    async fn update_completion_config(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a completion_config resource
    async fn delete_completion_config(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Session resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a session resource
    async fn plan_session(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new session resource
    async fn create_session(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a session resource
    async fn read_session(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a session resource
    async fn update_session(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a session resource
    async fn delete_session(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // User_license resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a user_license resource
    async fn plan_user_license(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new user_license resource
    async fn create_user_license(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a user_license resource
    async fn read_user_license(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a user_license resource
    async fn update_user_license(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a user_license resource
    async fn delete_user_license(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Document resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a document resource
    async fn plan_document(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new document resource
    async fn create_document(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a document resource
    async fn read_document(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a document resource
    async fn update_document(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a document resource
    async fn delete_document(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Control resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a control resource
    async fn plan_control(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new control resource
    async fn create_control(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a control resource
    async fn read_control(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a control resource
    async fn update_control(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a control resource
    async fn delete_control(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Data_store resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a data_store resource
    async fn plan_data_store(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new data_store resource
    async fn create_data_store(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a data_store resource
    async fn read_data_store(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a data_store resource
    async fn update_data_store(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a data_store resource
    async fn delete_data_store(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Media resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a media resource
    async fn plan_media(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new media resource
    async fn create_media(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a media resource
    async fn read_media(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a media resource
    async fn update_media(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a media resource
    async fn delete_media(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Assistant resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a assistant resource
    async fn plan_assistant(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new assistant resource
    async fn create_assistant(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a assistant resource
    async fn read_assistant(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a assistant resource
    async fn update_assistant(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a assistant resource
    async fn delete_assistant(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Ranking_config resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a ranking_config resource
    async fn plan_ranking_config(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new ranking_config resource
    async fn create_ranking_config(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a ranking_config resource
    async fn read_ranking_config(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a ranking_config resource
    async fn update_ranking_config(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a ranking_config resource
    async fn delete_ranking_config(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Schema resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a schema resource
    async fn plan_schema(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new schema resource
    async fn create_schema(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a schema resource
    async fn read_schema(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a schema resource
    async fn update_schema(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a schema resource
    async fn delete_schema(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // License_config resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a license_config resource
    async fn plan_license_config(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new license_config resource
    async fn create_license_config(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a license_config resource
    async fn read_license_config(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a license_config resource
    async fn update_license_config(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a license_config resource
    async fn delete_license_config(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Conversation resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a conversation resource
    async fn plan_conversation(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new conversation resource
    async fn create_conversation(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a conversation resource
    async fn read_conversation(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a conversation resource
    async fn update_conversation(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a conversation resource
    async fn delete_conversation(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Engine resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a engine resource
    async fn plan_engine(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new engine resource
    async fn create_engine(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a engine resource
    async fn read_engine(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a engine resource
    async fn update_engine(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a engine resource
    async fn delete_engine(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Branche resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a branche resource
    async fn plan_branche(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new branche resource
    async fn create_branche(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a branche resource
    async fn read_branche(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a branche resource
    async fn update_branche(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a branche resource
    async fn delete_branche(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Collection resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a collection resource
    async fn plan_collection(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new collection resource
    async fn create_collection(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a collection resource
    async fn read_collection(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a collection resource
    async fn update_collection(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a collection resource
    async fn delete_collection(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Target_site resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a target_site resource
    async fn plan_target_site(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new target_site resource
    async fn create_target_site(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a target_site resource
    async fn read_target_site(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a target_site resource
    async fn update_target_site(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a target_site resource
    async fn delete_target_site(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Suggestion_deny_list_entrie resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a suggestion_deny_list_entrie resource
    async fn plan_suggestion_deny_list_entrie(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new suggestion_deny_list_entrie resource
    async fn create_suggestion_deny_list_entrie(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a suggestion_deny_list_entrie resource
    async fn read_suggestion_deny_list_entrie(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a suggestion_deny_list_entrie resource
    async fn update_suggestion_deny_list_entrie(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a suggestion_deny_list_entrie resource
    async fn delete_suggestion_deny_list_entrie(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Serving_config resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a serving_config resource
    async fn plan_serving_config(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new serving_config resource
    async fn create_serving_config(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a serving_config resource
    async fn read_serving_config(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a serving_config resource
    async fn update_serving_config(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a serving_config resource
    async fn delete_serving_config(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Grounding_config resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a grounding_config resource
    async fn plan_grounding_config(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new grounding_config resource
    async fn create_grounding_config(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a grounding_config resource
    async fn read_grounding_config(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a grounding_config resource
    async fn update_grounding_config(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a grounding_config resource
    async fn delete_grounding_config(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Cmek_config resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a cmek_config resource
    async fn plan_cmek_config(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new cmek_config resource
    async fn create_cmek_config(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a cmek_config resource
    async fn read_cmek_config(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a cmek_config resource
    async fn update_cmek_config(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a cmek_config resource
    async fn delete_cmek_config(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // User_event resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a user_event resource
    async fn plan_user_event(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new user_event resource
    async fn create_user_event(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a user_event resource
    async fn read_user_event(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a user_event resource
    async fn update_user_event(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a user_event resource
    async fn delete_user_event(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Completion_suggestion resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a completion_suggestion resource
    async fn plan_completion_suggestion(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new completion_suggestion resource
    async fn create_completion_suggestion(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a completion_suggestion resource
    async fn read_completion_suggestion(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a completion_suggestion resource
    async fn update_completion_suggestion(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a completion_suggestion resource
    async fn delete_completion_suggestion(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Site_search_engine resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a site_search_engine resource
    async fn plan_site_search_engine(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new site_search_engine resource
    async fn create_site_search_engine(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a site_search_engine resource
    async fn read_site_search_engine(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a site_search_engine resource
    async fn update_site_search_engine(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a site_search_engine resource
    async fn delete_site_search_engine(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Project resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a project resource
    async fn plan_project(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new project resource
    async fn create_project(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a project resource
    async fn read_project(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a project resource
    async fn update_project(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a project resource
    async fn delete_project(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Branche resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a branche resource
    async fn plan_branche(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new branche resource
    async fn create_branche(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a branche resource
    async fn read_branche(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a branche resource
    async fn update_branche(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a branche resource
    async fn delete_branche(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Sample_querie resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a sample_querie resource
    async fn plan_sample_querie(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new sample_querie resource
    async fn create_sample_querie(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a sample_querie resource
    async fn read_sample_querie(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a sample_querie resource
    async fn update_sample_querie(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a sample_querie resource
    async fn delete_sample_querie(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Evaluation resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a evaluation resource
    async fn plan_evaluation(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new evaluation resource
    async fn create_evaluation(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a evaluation resource
    async fn read_evaluation(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a evaluation resource
    async fn update_evaluation(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a evaluation resource
    async fn delete_evaluation(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Grounding_config resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a grounding_config resource
    async fn plan_grounding_config(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new grounding_config resource
    async fn create_grounding_config(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a grounding_config resource
    async fn read_grounding_config(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a grounding_config resource
    async fn update_grounding_config(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a grounding_config resource
    async fn delete_grounding_config(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Media resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a media resource
    async fn plan_media(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new media resource
    async fn create_media(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a media resource
    async fn read_media(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a media resource
    async fn update_media(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a media resource
    async fn delete_media(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // File resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a file resource
    async fn plan_file(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new file resource
    async fn create_file(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a file resource
    async fn read_file(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a file resource
    async fn update_file(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a file resource
    async fn delete_file(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Target_site resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a target_site resource
    async fn plan_target_site(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new target_site resource
    async fn create_target_site(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a target_site resource
    async fn read_target_site(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a target_site resource
    async fn update_target_site(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a target_site resource
    async fn delete_target_site(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Operation resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a operation resource
    async fn plan_operation(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new operation resource
    async fn create_operation(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a operation resource
    async fn read_operation(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a operation resource
    async fn update_operation(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a operation resource
    async fn delete_operation(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Schema resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a schema resource
    async fn plan_schema(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new schema resource
    async fn create_schema(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a schema resource
    async fn read_schema(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a schema resource
    async fn update_schema(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a schema resource
    async fn delete_schema(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Document resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a document resource
    async fn plan_document(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new document resource
    async fn create_document(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a document resource
    async fn read_document(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a document resource
    async fn update_document(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a document resource
    async fn delete_document(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // User_event resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a user_event resource
    async fn plan_user_event(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new user_event resource
    async fn create_user_event(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a user_event resource
    async fn read_user_event(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a user_event resource
    async fn update_user_event(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a user_event resource
    async fn delete_user_event(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Site_search_engine resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a site_search_engine resource
    async fn plan_site_search_engine(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new site_search_engine resource
    async fn create_site_search_engine(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a site_search_engine resource
    async fn read_site_search_engine(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a site_search_engine resource
    async fn update_site_search_engine(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a site_search_engine resource
    async fn delete_site_search_engine(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Identity_mapping_store resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a identity_mapping_store resource
    async fn plan_identity_mapping_store(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new identity_mapping_store resource
    async fn create_identity_mapping_store(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a identity_mapping_store resource
    async fn read_identity_mapping_store(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a identity_mapping_store resource
    async fn update_identity_mapping_store(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a identity_mapping_store resource
    async fn delete_identity_mapping_store(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Agent resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a agent resource
    async fn plan_agent(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new agent resource
    async fn create_agent(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a agent resource
    async fn read_agent(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a agent resource
    async fn update_agent(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a agent resource
    async fn delete_agent(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Sitemap resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a sitemap resource
    async fn plan_sitemap(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new sitemap resource
    async fn create_sitemap(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a sitemap resource
    async fn read_sitemap(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a sitemap resource
    async fn update_sitemap(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a sitemap resource
    async fn delete_sitemap(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Completion_config resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a completion_config resource
    async fn plan_completion_config(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new completion_config resource
    async fn create_completion_config(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a completion_config resource
    async fn read_completion_config(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a completion_config resource
    async fn update_completion_config(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a completion_config resource
    async fn delete_completion_config(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Answer resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a answer resource
    async fn plan_answer(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new answer resource
    async fn create_answer(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a answer resource
    async fn read_answer(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a answer resource
    async fn update_answer(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a answer resource
    async fn delete_answer(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Assistant resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a assistant resource
    async fn plan_assistant(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new assistant resource
    async fn create_assistant(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a assistant resource
    async fn read_assistant(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a assistant resource
    async fn update_assistant(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a assistant resource
    async fn delete_assistant(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // User_license resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a user_license resource
    async fn plan_user_license(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new user_license resource
    async fn create_user_license(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a user_license resource
    async fn read_user_license(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a user_license resource
    async fn update_user_license(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a user_license resource
    async fn delete_user_license(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Session resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a session resource
    async fn plan_session(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new session resource
    async fn create_session(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a session resource
    async fn read_session(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a session resource
    async fn update_session(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a session resource
    async fn delete_session(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Source resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a source resource
    async fn plan_source(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new source resource
    async fn create_source(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a source resource
    async fn read_source(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a source resource
    async fn update_source(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a source resource
    async fn delete_source(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Canned_querie resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a canned_querie resource
    async fn plan_canned_querie(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new canned_querie resource
    async fn create_canned_querie(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a canned_querie resource
    async fn read_canned_querie(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a canned_querie resource
    async fn update_canned_querie(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a canned_querie resource
    async fn delete_canned_querie(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Chunk resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a chunk resource
    async fn plan_chunk(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new chunk resource
    async fn create_chunk(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a chunk resource
    async fn read_chunk(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a chunk resource
    async fn update_chunk(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a chunk resource
    async fn delete_chunk(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Billing_account_license_config resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a billing_account_license_config resource
    async fn plan_billing_account_license_config(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new billing_account_license_config resource
    async fn create_billing_account_license_config(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a billing_account_license_config resource
    async fn read_billing_account_license_config(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a billing_account_license_config resource
    async fn update_billing_account_license_config(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a billing_account_license_config resource
    async fn delete_billing_account_license_config(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Data_connector resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a data_connector resource
    async fn plan_data_connector(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new data_connector resource
    async fn create_data_connector(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a data_connector resource
    async fn read_data_connector(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a data_connector resource
    async fn update_data_connector(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a data_connector resource
    async fn delete_data_connector(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Data_store resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a data_store resource
    async fn plan_data_store(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new data_store resource
    async fn create_data_store(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a data_store resource
    async fn read_data_store(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a data_store resource
    async fn update_data_store(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a data_store resource
    async fn delete_data_store(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Audio_overview resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a audio_overview resource
    async fn plan_audio_overview(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new audio_overview resource
    async fn create_audio_overview(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a audio_overview resource
    async fn read_audio_overview(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a audio_overview resource
    async fn update_audio_overview(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a audio_overview resource
    async fn delete_audio_overview(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Requirement resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a requirement resource
    async fn plan_requirement(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new requirement resource
    async fn create_requirement(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a requirement resource
    async fn read_requirement(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a requirement resource
    async fn update_requirement(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a requirement resource
    async fn delete_requirement(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Authorization resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a authorization resource
    async fn plan_authorization(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new authorization resource
    async fn create_authorization(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a authorization resource
    async fn read_authorization(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a authorization resource
    async fn update_authorization(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a authorization resource
    async fn delete_authorization(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // License_config resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a license_config resource
    async fn plan_license_config(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new license_config resource
    async fn create_license_config(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a license_config resource
    async fn read_license_config(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a license_config resource
    async fn update_license_config(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a license_config resource
    async fn delete_license_config(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Analytic resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a analytic resource
    async fn plan_analytic(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new analytic resource
    async fn create_analytic(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a analytic resource
    async fn read_analytic(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a analytic resource
    async fn update_analytic(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a analytic resource
    async fn delete_analytic(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Control resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a control resource
    async fn plan_control(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new control resource
    async fn create_control(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a control resource
    async fn read_control(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a control resource
    async fn update_control(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a control resource
    async fn delete_control(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Custom_model resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a custom_model resource
    async fn plan_custom_model(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new custom_model resource
    async fn create_custom_model(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a custom_model resource
    async fn read_custom_model(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a custom_model resource
    async fn update_custom_model(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a custom_model resource
    async fn delete_custom_model(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Serving_config resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a serving_config resource
    async fn plan_serving_config(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new serving_config resource
    async fn create_serving_config(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a serving_config resource
    async fn read_serving_config(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a serving_config resource
    async fn update_serving_config(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a serving_config resource
    async fn delete_serving_config(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Collection resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a collection resource
    async fn plan_collection(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new collection resource
    async fn create_collection(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a collection resource
    async fn read_collection(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a collection resource
    async fn update_collection(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a collection resource
    async fn delete_collection(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Engine resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a engine resource
    async fn plan_engine(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new engine resource
    async fn create_engine(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a engine resource
    async fn read_engine(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a engine resource
    async fn update_engine(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a engine resource
    async fn delete_engine(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Connector_run resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a connector_run resource
    async fn plan_connector_run(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new connector_run resource
    async fn create_connector_run(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a connector_run resource
    async fn read_connector_run(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a connector_run resource
    async fn update_connector_run(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a connector_run resource
    async fn delete_connector_run(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Widget_config resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a widget_config resource
    async fn plan_widget_config(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new widget_config resource
    async fn create_widget_config(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a widget_config resource
    async fn read_widget_config(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a widget_config resource
    async fn update_widget_config(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a widget_config resource
    async fn delete_widget_config(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Project resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a project resource
    async fn plan_project(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new project resource
    async fn create_project(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a project resource
    async fn read_project(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a project resource
    async fn update_project(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a project resource
    async fn delete_project(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Conversation resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a conversation resource
    async fn plan_conversation(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new conversation resource
    async fn create_conversation(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a conversation resource
    async fn read_conversation(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a conversation resource
    async fn update_conversation(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a conversation resource
    async fn delete_conversation(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Completion_suggestion resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a completion_suggestion resource
    async fn plan_completion_suggestion(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new completion_suggestion resource
    async fn create_completion_suggestion(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a completion_suggestion resource
    async fn read_completion_suggestion(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a completion_suggestion resource
    async fn update_completion_suggestion(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a completion_suggestion resource
    async fn delete_completion_suggestion(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Notebook resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a notebook resource
    async fn plan_notebook(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new notebook resource
    async fn create_notebook(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a notebook resource
    async fn read_notebook(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a notebook resource
    async fn update_notebook(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a notebook resource
    async fn delete_notebook(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Ranking_config resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a ranking_config resource
    async fn plan_ranking_config(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new ranking_config resource
    async fn create_ranking_config(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a ranking_config resource
    async fn read_ranking_config(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a ranking_config resource
    async fn update_ranking_config(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a ranking_config resource
    async fn delete_ranking_config(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Suggestion_deny_list_entrie resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a suggestion_deny_list_entrie resource
    async fn plan_suggestion_deny_list_entrie(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new suggestion_deny_list_entrie resource
    async fn create_suggestion_deny_list_entrie(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a suggestion_deny_list_entrie resource
    async fn read_suggestion_deny_list_entrie(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a suggestion_deny_list_entrie resource
    async fn update_suggestion_deny_list_entrie(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a suggestion_deny_list_entrie resource
    async fn delete_suggestion_deny_list_entrie(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Cmek_config resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a cmek_config resource
    async fn plan_cmek_config(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new cmek_config resource
    async fn create_cmek_config(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a cmek_config resource
    async fn read_cmek_config(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a cmek_config resource
    async fn update_cmek_config(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a cmek_config resource
    async fn delete_cmek_config(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Location resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a location resource
    async fn plan_location(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new location resource
    async fn create_location(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a location resource
    async fn read_location(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a location resource
    async fn update_location(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a location resource
    async fn delete_location(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // User_store resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a user_store resource
    async fn plan_user_store(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new user_store resource
    async fn create_user_store(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a user_store resource
    async fn read_user_store(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a user_store resource
    async fn update_user_store(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a user_store resource
    async fn delete_user_store(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Sample_query_set resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a sample_query_set resource
    async fn plan_sample_query_set(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new sample_query_set resource
    async fn create_sample_query_set(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a sample_query_set resource
    async fn read_sample_query_set(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a sample_query_set resource
    async fn update_sample_query_set(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a sample_query_set resource
    async fn delete_sample_query_set(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Evaluation resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a evaluation resource
    async fn plan_evaluation(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new evaluation resource
    async fn create_evaluation(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a evaluation resource
    async fn read_evaluation(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a evaluation resource
    async fn update_evaluation(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a evaluation resource
    async fn delete_evaluation(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Ranking_config resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a ranking_config resource
    async fn plan_ranking_config(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new ranking_config resource
    async fn create_ranking_config(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a ranking_config resource
    async fn read_ranking_config(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a ranking_config resource
    async fn update_ranking_config(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a ranking_config resource
    async fn delete_ranking_config(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Answer resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a answer resource
    async fn plan_answer(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new answer resource
    async fn create_answer(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a answer resource
    async fn read_answer(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a answer resource
    async fn update_answer(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a answer resource
    async fn delete_answer(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Completion_suggestion resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a completion_suggestion resource
    async fn plan_completion_suggestion(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new completion_suggestion resource
    async fn create_completion_suggestion(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a completion_suggestion resource
    async fn read_completion_suggestion(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a completion_suggestion resource
    async fn update_completion_suggestion(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a completion_suggestion resource
    async fn delete_completion_suggestion(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Target_site resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a target_site resource
    async fn plan_target_site(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new target_site resource
    async fn create_target_site(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a target_site resource
    async fn read_target_site(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a target_site resource
    async fn update_target_site(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a target_site resource
    async fn delete_target_site(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Suggestion_deny_list_entrie resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a suggestion_deny_list_entrie resource
    async fn plan_suggestion_deny_list_entrie(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new suggestion_deny_list_entrie resource
    async fn create_suggestion_deny_list_entrie(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a suggestion_deny_list_entrie resource
    async fn read_suggestion_deny_list_entrie(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a suggestion_deny_list_entrie resource
    async fn update_suggestion_deny_list_entrie(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a suggestion_deny_list_entrie resource
    async fn delete_suggestion_deny_list_entrie(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Identity_mapping_store resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a identity_mapping_store resource
    async fn plan_identity_mapping_store(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new identity_mapping_store resource
    async fn create_identity_mapping_store(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a identity_mapping_store resource
    async fn read_identity_mapping_store(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a identity_mapping_store resource
    async fn update_identity_mapping_store(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a identity_mapping_store resource
    async fn delete_identity_mapping_store(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // User_store resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a user_store resource
    async fn plan_user_store(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new user_store resource
    async fn create_user_store(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a user_store resource
    async fn read_user_store(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a user_store resource
    async fn update_user_store(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a user_store resource
    async fn delete_user_store(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Custom_model resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a custom_model resource
    async fn plan_custom_model(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new custom_model resource
    async fn create_custom_model(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a custom_model resource
    async fn read_custom_model(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a custom_model resource
    async fn update_custom_model(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a custom_model resource
    async fn delete_custom_model(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Data_store resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a data_store resource
    async fn plan_data_store(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new data_store resource
    async fn create_data_store(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a data_store resource
    async fn read_data_store(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a data_store resource
    async fn update_data_store(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a data_store resource
    async fn delete_data_store(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Branche resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a branche resource
    async fn plan_branche(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new branche resource
    async fn create_branche(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a branche resource
    async fn read_branche(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a branche resource
    async fn update_branche(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a branche resource
    async fn delete_branche(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Sample_query_set resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a sample_query_set resource
    async fn plan_sample_query_set(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new sample_query_set resource
    async fn create_sample_query_set(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a sample_query_set resource
    async fn read_sample_query_set(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a sample_query_set resource
    async fn update_sample_query_set(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a sample_query_set resource
    async fn delete_sample_query_set(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Media resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a media resource
    async fn plan_media(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new media resource
    async fn create_media(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a media resource
    async fn read_media(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a media resource
    async fn update_media(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a media resource
    async fn delete_media(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Site_search_engine resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a site_search_engine resource
    async fn plan_site_search_engine(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new site_search_engine resource
    async fn create_site_search_engine(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a site_search_engine resource
    async fn read_site_search_engine(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a site_search_engine resource
    async fn update_site_search_engine(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a site_search_engine resource
    async fn delete_site_search_engine(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Schema resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a schema resource
    async fn plan_schema(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new schema resource
    async fn create_schema(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a schema resource
    async fn read_schema(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a schema resource
    async fn update_schema(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a schema resource
    async fn delete_schema(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Serving_config resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a serving_config resource
    async fn plan_serving_config(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new serving_config resource
    async fn create_serving_config(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a serving_config resource
    async fn read_serving_config(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a serving_config resource
    async fn update_serving_config(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a serving_config resource
    async fn delete_serving_config(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Sample_querie resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a sample_querie resource
    async fn plan_sample_querie(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new sample_querie resource
    async fn create_sample_querie(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a sample_querie resource
    async fn read_sample_querie(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a sample_querie resource
    async fn update_sample_querie(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a sample_querie resource
    async fn delete_sample_querie(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Grounding_config resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a grounding_config resource
    async fn plan_grounding_config(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new grounding_config resource
    async fn create_grounding_config(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a grounding_config resource
    async fn read_grounding_config(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a grounding_config resource
    async fn update_grounding_config(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a grounding_config resource
    async fn delete_grounding_config(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Cmek_config resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a cmek_config resource
    async fn plan_cmek_config(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new cmek_config resource
    async fn create_cmek_config(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a cmek_config resource
    async fn read_cmek_config(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a cmek_config resource
    async fn update_cmek_config(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a cmek_config resource
    async fn delete_cmek_config(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // License_config resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a license_config resource
    async fn plan_license_config(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new license_config resource
    async fn create_license_config(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a license_config resource
    async fn read_license_config(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a license_config resource
    async fn update_license_config(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a license_config resource
    async fn delete_license_config(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Engine resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a engine resource
    async fn plan_engine(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new engine resource
    async fn create_engine(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a engine resource
    async fn read_engine(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a engine resource
    async fn update_engine(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a engine resource
    async fn delete_engine(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Sitemap resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a sitemap resource
    async fn plan_sitemap(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new sitemap resource
    async fn create_sitemap(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a sitemap resource
    async fn read_sitemap(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a sitemap resource
    async fn update_sitemap(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a sitemap resource
    async fn delete_sitemap(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Operation resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a operation resource
    async fn plan_operation(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new operation resource
    async fn create_operation(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a operation resource
    async fn read_operation(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a operation resource
    async fn update_operation(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a operation resource
    async fn delete_operation(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // User_event resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a user_event resource
    async fn plan_user_event(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new user_event resource
    async fn create_user_event(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a user_event resource
    async fn read_user_event(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a user_event resource
    async fn update_user_event(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a user_event resource
    async fn delete_user_event(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // User_license resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a user_license resource
    async fn plan_user_license(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new user_license resource
    async fn create_user_license(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a user_license resource
    async fn read_user_license(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a user_license resource
    async fn update_user_license(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a user_license resource
    async fn delete_user_license(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Control resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a control resource
    async fn plan_control(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new control resource
    async fn create_control(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a control resource
    async fn read_control(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a control resource
    async fn update_control(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a control resource
    async fn delete_control(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Session resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a session resource
    async fn plan_session(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new session resource
    async fn create_session(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a session resource
    async fn read_session(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a session resource
    async fn update_session(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a session resource
    async fn delete_session(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Location resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a location resource
    async fn plan_location(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new location resource
    async fn create_location(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a location resource
    async fn read_location(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a location resource
    async fn update_location(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a location resource
    async fn delete_location(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Assistant resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a assistant resource
    async fn plan_assistant(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new assistant resource
    async fn create_assistant(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a assistant resource
    async fn read_assistant(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a assistant resource
    async fn update_assistant(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a assistant resource
    async fn delete_assistant(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Completion_config resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a completion_config resource
    async fn plan_completion_config(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new completion_config resource
    async fn create_completion_config(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a completion_config resource
    async fn read_completion_config(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a completion_config resource
    async fn update_completion_config(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a completion_config resource
    async fn delete_completion_config(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Conversation resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a conversation resource
    async fn plan_conversation(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new conversation resource
    async fn create_conversation(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a conversation resource
    async fn read_conversation(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a conversation resource
    async fn update_conversation(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a conversation resource
    async fn delete_conversation(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Document resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a document resource
    async fn plan_document(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new document resource
    async fn create_document(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a document resource
    async fn read_document(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a document resource
    async fn update_document(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a document resource
    async fn delete_document(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Project resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a project resource
    async fn plan_project(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new project resource
    async fn create_project(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a project resource
    async fn read_project(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a project resource
    async fn update_project(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a project resource
    async fn delete_project(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }
}
