//! Dialogflow_api service for Gcp provider
//!
//! This module handles all dialogflow_api resources and their CRUD operations.

use hemmer_core::Result;
use hemmer_provider::{ResourceInput, ResourceOutput, ResourcePlan};

/// Dialogflow_api service handler
pub struct Dialogflow_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Dialogflow_apiService<'a> {
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
            "test_case" => self.plan_test_case(current_state, desired_input).await,
            "result" => self.plan_result(current_state, desired_input).await,
            "environment" => self.plan_environment(current_state, desired_input).await,
            "page" => self.plan_page(current_state, desired_input).await,
            "intent" => self.plan_intent(current_state, desired_input).await,
            "session" => self.plan_session(current_state, desired_input).await,
            "deployment" => self.plan_deployment(current_state, desired_input).await,
            "security_setting" => {
                self.plan_security_setting(current_state, desired_input)
                    .await
            }
            "webhook" => self.plan_webhook(current_state, desired_input).await,
            "location" => self.plan_location(current_state, desired_input).await,
            "changelog" => self.plan_changelog(current_state, desired_input).await,
            "entity_type" => self.plan_entity_type(current_state, desired_input).await,
            "transition_route_group" => {
                self.plan_transition_route_group(current_state, desired_input)
                    .await
            }
            "playbook" => self.plan_playbook(current_state, desired_input).await,
            "generator" => self.plan_generator(current_state, desired_input).await,
            "tool" => self.plan_tool(current_state, desired_input).await,
            "experiment" => self.plan_experiment(current_state, desired_input).await,
            "example" => self.plan_example(current_state, desired_input).await,
            "continuous_test_result" => {
                self.plan_continuous_test_result(current_state, desired_input)
                    .await
            }
            "flow" => self.plan_flow(current_state, desired_input).await,
            "agent" => self.plan_agent(current_state, desired_input).await,
            "version" => self.plan_version(current_state, desired_input).await,
            "operation" => self.plan_operation(current_state, desired_input).await,
            "conversation" => self.plan_conversation(current_state, desired_input).await,
            "environment" => self.plan_environment(current_state, desired_input).await,
            "conversation_profile" => {
                self.plan_conversation_profile(current_state, desired_input)
                    .await
            }
            "version" => self.plan_version(current_state, desired_input).await,
            "context" => self.plan_context(current_state, desired_input).await,
            "project" => self.plan_project(current_state, desired_input).await,
            "intent" => self.plan_intent(current_state, desired_input).await,
            "session" => self.plan_session(current_state, desired_input).await,
            "conversation" => self.plan_conversation(current_state, desired_input).await,
            "entity_type" => self.plan_entity_type(current_state, desired_input).await,
            "location" => self.plan_location(current_state, desired_input).await,
            "agent" => self.plan_agent(current_state, desired_input).await,
            "tool" => self.plan_tool(current_state, desired_input).await,
            "encryption_spec" => {
                self.plan_encryption_spec(current_state, desired_input)
                    .await
            }
            "suggestion" => self.plan_suggestion(current_state, desired_input).await,
            "entitie" => self.plan_entitie(current_state, desired_input).await,
            "knowledge_base" => self.plan_knowledge_base(current_state, desired_input).await,
            "message" => self.plan_message(current_state, desired_input).await,
            "phone_number" => self.plan_phone_number(current_state, desired_input).await,
            "evaluation" => self.plan_evaluation(current_state, desired_input).await,
            "document" => self.plan_document(current_state, desired_input).await,
            "participant" => self.plan_participant(current_state, desired_input).await,
            "generator" => self.plan_generator(current_state, desired_input).await,
            "operation" => self.plan_operation(current_state, desired_input).await,
            "stateless_suggestion" => {
                self.plan_stateless_suggestion(current_state, desired_input)
                    .await
            }
            "sip_trunk" => self.plan_sip_trunk(current_state, desired_input).await,
            "answer_record" => self.plan_answer_record(current_state, desired_input).await,
            "stateless_suggestion" => {
                self.plan_stateless_suggestion(current_state, desired_input)
                    .await
            }
            "sip_trunk" => self.plan_sip_trunk(current_state, desired_input).await,
            "entity_type" => self.plan_entity_type(current_state, desired_input).await,
            "conversation_model" => {
                self.plan_conversation_model(current_state, desired_input)
                    .await
            }
            "knowledge_base" => self.plan_knowledge_base(current_state, desired_input).await,
            "conversation_dataset" => {
                self.plan_conversation_dataset(current_state, desired_input)
                    .await
            }
            "generator" => self.plan_generator(current_state, desired_input).await,
            "encryption_spec" => {
                self.plan_encryption_spec(current_state, desired_input)
                    .await
            }
            "document" => self.plan_document(current_state, desired_input).await,
            "session" => self.plan_session(current_state, desired_input).await,
            "location" => self.plan_location(current_state, desired_input).await,
            "evaluation" => self.plan_evaluation(current_state, desired_input).await,
            "entitie" => self.plan_entitie(current_state, desired_input).await,
            "participant" => self.plan_participant(current_state, desired_input).await,
            "project" => self.plan_project(current_state, desired_input).await,
            "operation" => self.plan_operation(current_state, desired_input).await,
            "conversation" => self.plan_conversation(current_state, desired_input).await,
            "message" => self.plan_message(current_state, desired_input).await,
            "agent" => self.plan_agent(current_state, desired_input).await,
            "intent" => self.plan_intent(current_state, desired_input).await,
            "suggestion" => self.plan_suggestion(current_state, desired_input).await,
            "environment" => self.plan_environment(current_state, desired_input).await,
            "context" => self.plan_context(current_state, desired_input).await,
            "version" => self.plan_version(current_state, desired_input).await,
            "answer_record" => self.plan_answer_record(current_state, desired_input).await,
            "tool" => self.plan_tool(current_state, desired_input).await,
            "conversation_profile" => {
                self.plan_conversation_profile(current_state, desired_input)
                    .await
            }
            "deployment" => self.plan_deployment(current_state, desired_input).await,
            "version" => self.plan_version(current_state, desired_input).await,
            "security_setting" => {
                self.plan_security_setting(current_state, desired_input)
                    .await
            }
            "test_case" => self.plan_test_case(current_state, desired_input).await,
            "continuous_test_result" => {
                self.plan_continuous_test_result(current_state, desired_input)
                    .await
            }
            "session" => self.plan_session(current_state, desired_input).await,
            "page" => self.plan_page(current_state, desired_input).await,
            "webhook" => self.plan_webhook(current_state, desired_input).await,
            "environment" => self.plan_environment(current_state, desired_input).await,
            "changelog" => self.plan_changelog(current_state, desired_input).await,
            "agent" => self.plan_agent(current_state, desired_input).await,
            "playbook" => self.plan_playbook(current_state, desired_input).await,
            "intent" => self.plan_intent(current_state, desired_input).await,
            "entity_type" => self.plan_entity_type(current_state, desired_input).await,
            "operation" => self.plan_operation(current_state, desired_input).await,
            "flow" => self.plan_flow(current_state, desired_input).await,
            "example" => self.plan_example(current_state, desired_input).await,
            "tool" => self.plan_tool(current_state, desired_input).await,
            "location" => self.plan_location(current_state, desired_input).await,
            "generator" => self.plan_generator(current_state, desired_input).await,
            "transition_route_group" => {
                self.plan_transition_route_group(current_state, desired_input)
                    .await
            }
            "result" => self.plan_result(current_state, desired_input).await,
            "experiment" => self.plan_experiment(current_state, desired_input).await,
            "operation" => self.plan_operation(current_state, desired_input).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "dialogflow_api", resource_name
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
            "test_case" => self.create_test_case(input).await,
            "result" => self.create_result(input).await,
            "environment" => self.create_environment(input).await,
            "page" => self.create_page(input).await,
            "intent" => self.create_intent(input).await,
            "session" => self.create_session(input).await,
            "deployment" => self.create_deployment(input).await,
            "security_setting" => self.create_security_setting(input).await,
            "webhook" => self.create_webhook(input).await,
            "location" => self.create_location(input).await,
            "changelog" => self.create_changelog(input).await,
            "entity_type" => self.create_entity_type(input).await,
            "transition_route_group" => self.create_transition_route_group(input).await,
            "playbook" => self.create_playbook(input).await,
            "generator" => self.create_generator(input).await,
            "tool" => self.create_tool(input).await,
            "experiment" => self.create_experiment(input).await,
            "example" => self.create_example(input).await,
            "continuous_test_result" => self.create_continuous_test_result(input).await,
            "flow" => self.create_flow(input).await,
            "agent" => self.create_agent(input).await,
            "version" => self.create_version(input).await,
            "operation" => self.create_operation(input).await,
            "conversation" => self.create_conversation(input).await,
            "environment" => self.create_environment(input).await,
            "conversation_profile" => self.create_conversation_profile(input).await,
            "version" => self.create_version(input).await,
            "context" => self.create_context(input).await,
            "project" => self.create_project(input).await,
            "intent" => self.create_intent(input).await,
            "session" => self.create_session(input).await,
            "conversation" => self.create_conversation(input).await,
            "entity_type" => self.create_entity_type(input).await,
            "location" => self.create_location(input).await,
            "agent" => self.create_agent(input).await,
            "tool" => self.create_tool(input).await,
            "encryption_spec" => self.create_encryption_spec(input).await,
            "suggestion" => self.create_suggestion(input).await,
            "entitie" => self.create_entitie(input).await,
            "knowledge_base" => self.create_knowledge_base(input).await,
            "message" => self.create_message(input).await,
            "phone_number" => self.create_phone_number(input).await,
            "evaluation" => self.create_evaluation(input).await,
            "document" => self.create_document(input).await,
            "participant" => self.create_participant(input).await,
            "generator" => self.create_generator(input).await,
            "operation" => self.create_operation(input).await,
            "stateless_suggestion" => self.create_stateless_suggestion(input).await,
            "sip_trunk" => self.create_sip_trunk(input).await,
            "answer_record" => self.create_answer_record(input).await,
            "stateless_suggestion" => self.create_stateless_suggestion(input).await,
            "sip_trunk" => self.create_sip_trunk(input).await,
            "entity_type" => self.create_entity_type(input).await,
            "conversation_model" => self.create_conversation_model(input).await,
            "knowledge_base" => self.create_knowledge_base(input).await,
            "conversation_dataset" => self.create_conversation_dataset(input).await,
            "generator" => self.create_generator(input).await,
            "encryption_spec" => self.create_encryption_spec(input).await,
            "document" => self.create_document(input).await,
            "session" => self.create_session(input).await,
            "location" => self.create_location(input).await,
            "evaluation" => self.create_evaluation(input).await,
            "entitie" => self.create_entitie(input).await,
            "participant" => self.create_participant(input).await,
            "project" => self.create_project(input).await,
            "operation" => self.create_operation(input).await,
            "conversation" => self.create_conversation(input).await,
            "message" => self.create_message(input).await,
            "agent" => self.create_agent(input).await,
            "intent" => self.create_intent(input).await,
            "suggestion" => self.create_suggestion(input).await,
            "environment" => self.create_environment(input).await,
            "context" => self.create_context(input).await,
            "version" => self.create_version(input).await,
            "answer_record" => self.create_answer_record(input).await,
            "tool" => self.create_tool(input).await,
            "conversation_profile" => self.create_conversation_profile(input).await,
            "deployment" => self.create_deployment(input).await,
            "version" => self.create_version(input).await,
            "security_setting" => self.create_security_setting(input).await,
            "test_case" => self.create_test_case(input).await,
            "continuous_test_result" => self.create_continuous_test_result(input).await,
            "session" => self.create_session(input).await,
            "page" => self.create_page(input).await,
            "webhook" => self.create_webhook(input).await,
            "environment" => self.create_environment(input).await,
            "changelog" => self.create_changelog(input).await,
            "agent" => self.create_agent(input).await,
            "playbook" => self.create_playbook(input).await,
            "intent" => self.create_intent(input).await,
            "entity_type" => self.create_entity_type(input).await,
            "operation" => self.create_operation(input).await,
            "flow" => self.create_flow(input).await,
            "example" => self.create_example(input).await,
            "tool" => self.create_tool(input).await,
            "location" => self.create_location(input).await,
            "generator" => self.create_generator(input).await,
            "transition_route_group" => self.create_transition_route_group(input).await,
            "result" => self.create_result(input).await,
            "experiment" => self.create_experiment(input).await,
            "operation" => self.create_operation(input).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "dialogflow_api", resource_name
            ))),
        }
    }

    /// Read resource state
    pub async fn read_resource(&self, resource_name: &str, id: &str) -> Result<ResourceOutput> {
        match resource_name {
            "test_case" => self.read_test_case(id).await,
            "result" => self.read_result(id).await,
            "environment" => self.read_environment(id).await,
            "page" => self.read_page(id).await,
            "intent" => self.read_intent(id).await,
            "session" => self.read_session(id).await,
            "deployment" => self.read_deployment(id).await,
            "security_setting" => self.read_security_setting(id).await,
            "webhook" => self.read_webhook(id).await,
            "location" => self.read_location(id).await,
            "changelog" => self.read_changelog(id).await,
            "entity_type" => self.read_entity_type(id).await,
            "transition_route_group" => self.read_transition_route_group(id).await,
            "playbook" => self.read_playbook(id).await,
            "generator" => self.read_generator(id).await,
            "tool" => self.read_tool(id).await,
            "experiment" => self.read_experiment(id).await,
            "example" => self.read_example(id).await,
            "continuous_test_result" => self.read_continuous_test_result(id).await,
            "flow" => self.read_flow(id).await,
            "agent" => self.read_agent(id).await,
            "version" => self.read_version(id).await,
            "operation" => self.read_operation(id).await,
            "conversation" => self.read_conversation(id).await,
            "environment" => self.read_environment(id).await,
            "conversation_profile" => self.read_conversation_profile(id).await,
            "version" => self.read_version(id).await,
            "context" => self.read_context(id).await,
            "project" => self.read_project(id).await,
            "intent" => self.read_intent(id).await,
            "session" => self.read_session(id).await,
            "conversation" => self.read_conversation(id).await,
            "entity_type" => self.read_entity_type(id).await,
            "location" => self.read_location(id).await,
            "agent" => self.read_agent(id).await,
            "tool" => self.read_tool(id).await,
            "encryption_spec" => self.read_encryption_spec(id).await,
            "suggestion" => self.read_suggestion(id).await,
            "entitie" => self.read_entitie(id).await,
            "knowledge_base" => self.read_knowledge_base(id).await,
            "message" => self.read_message(id).await,
            "phone_number" => self.read_phone_number(id).await,
            "evaluation" => self.read_evaluation(id).await,
            "document" => self.read_document(id).await,
            "participant" => self.read_participant(id).await,
            "generator" => self.read_generator(id).await,
            "operation" => self.read_operation(id).await,
            "stateless_suggestion" => self.read_stateless_suggestion(id).await,
            "sip_trunk" => self.read_sip_trunk(id).await,
            "answer_record" => self.read_answer_record(id).await,
            "stateless_suggestion" => self.read_stateless_suggestion(id).await,
            "sip_trunk" => self.read_sip_trunk(id).await,
            "entity_type" => self.read_entity_type(id).await,
            "conversation_model" => self.read_conversation_model(id).await,
            "knowledge_base" => self.read_knowledge_base(id).await,
            "conversation_dataset" => self.read_conversation_dataset(id).await,
            "generator" => self.read_generator(id).await,
            "encryption_spec" => self.read_encryption_spec(id).await,
            "document" => self.read_document(id).await,
            "session" => self.read_session(id).await,
            "location" => self.read_location(id).await,
            "evaluation" => self.read_evaluation(id).await,
            "entitie" => self.read_entitie(id).await,
            "participant" => self.read_participant(id).await,
            "project" => self.read_project(id).await,
            "operation" => self.read_operation(id).await,
            "conversation" => self.read_conversation(id).await,
            "message" => self.read_message(id).await,
            "agent" => self.read_agent(id).await,
            "intent" => self.read_intent(id).await,
            "suggestion" => self.read_suggestion(id).await,
            "environment" => self.read_environment(id).await,
            "context" => self.read_context(id).await,
            "version" => self.read_version(id).await,
            "answer_record" => self.read_answer_record(id).await,
            "tool" => self.read_tool(id).await,
            "conversation_profile" => self.read_conversation_profile(id).await,
            "deployment" => self.read_deployment(id).await,
            "version" => self.read_version(id).await,
            "security_setting" => self.read_security_setting(id).await,
            "test_case" => self.read_test_case(id).await,
            "continuous_test_result" => self.read_continuous_test_result(id).await,
            "session" => self.read_session(id).await,
            "page" => self.read_page(id).await,
            "webhook" => self.read_webhook(id).await,
            "environment" => self.read_environment(id).await,
            "changelog" => self.read_changelog(id).await,
            "agent" => self.read_agent(id).await,
            "playbook" => self.read_playbook(id).await,
            "intent" => self.read_intent(id).await,
            "entity_type" => self.read_entity_type(id).await,
            "operation" => self.read_operation(id).await,
            "flow" => self.read_flow(id).await,
            "example" => self.read_example(id).await,
            "tool" => self.read_tool(id).await,
            "location" => self.read_location(id).await,
            "generator" => self.read_generator(id).await,
            "transition_route_group" => self.read_transition_route_group(id).await,
            "result" => self.read_result(id).await,
            "experiment" => self.read_experiment(id).await,
            "operation" => self.read_operation(id).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "dialogflow_api", resource_name
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
            "test_case" => self.update_test_case(id, input).await,
            "result" => self.update_result(id, input).await,
            "environment" => self.update_environment(id, input).await,
            "page" => self.update_page(id, input).await,
            "intent" => self.update_intent(id, input).await,
            "session" => self.update_session(id, input).await,
            "deployment" => self.update_deployment(id, input).await,
            "security_setting" => self.update_security_setting(id, input).await,
            "webhook" => self.update_webhook(id, input).await,
            "location" => self.update_location(id, input).await,
            "changelog" => self.update_changelog(id, input).await,
            "entity_type" => self.update_entity_type(id, input).await,
            "transition_route_group" => self.update_transition_route_group(id, input).await,
            "playbook" => self.update_playbook(id, input).await,
            "generator" => self.update_generator(id, input).await,
            "tool" => self.update_tool(id, input).await,
            "experiment" => self.update_experiment(id, input).await,
            "example" => self.update_example(id, input).await,
            "continuous_test_result" => self.update_continuous_test_result(id, input).await,
            "flow" => self.update_flow(id, input).await,
            "agent" => self.update_agent(id, input).await,
            "version" => self.update_version(id, input).await,
            "operation" => self.update_operation(id, input).await,
            "conversation" => self.update_conversation(id, input).await,
            "environment" => self.update_environment(id, input).await,
            "conversation_profile" => self.update_conversation_profile(id, input).await,
            "version" => self.update_version(id, input).await,
            "context" => self.update_context(id, input).await,
            "project" => self.update_project(id, input).await,
            "intent" => self.update_intent(id, input).await,
            "session" => self.update_session(id, input).await,
            "conversation" => self.update_conversation(id, input).await,
            "entity_type" => self.update_entity_type(id, input).await,
            "location" => self.update_location(id, input).await,
            "agent" => self.update_agent(id, input).await,
            "tool" => self.update_tool(id, input).await,
            "encryption_spec" => self.update_encryption_spec(id, input).await,
            "suggestion" => self.update_suggestion(id, input).await,
            "entitie" => self.update_entitie(id, input).await,
            "knowledge_base" => self.update_knowledge_base(id, input).await,
            "message" => self.update_message(id, input).await,
            "phone_number" => self.update_phone_number(id, input).await,
            "evaluation" => self.update_evaluation(id, input).await,
            "document" => self.update_document(id, input).await,
            "participant" => self.update_participant(id, input).await,
            "generator" => self.update_generator(id, input).await,
            "operation" => self.update_operation(id, input).await,
            "stateless_suggestion" => self.update_stateless_suggestion(id, input).await,
            "sip_trunk" => self.update_sip_trunk(id, input).await,
            "answer_record" => self.update_answer_record(id, input).await,
            "stateless_suggestion" => self.update_stateless_suggestion(id, input).await,
            "sip_trunk" => self.update_sip_trunk(id, input).await,
            "entity_type" => self.update_entity_type(id, input).await,
            "conversation_model" => self.update_conversation_model(id, input).await,
            "knowledge_base" => self.update_knowledge_base(id, input).await,
            "conversation_dataset" => self.update_conversation_dataset(id, input).await,
            "generator" => self.update_generator(id, input).await,
            "encryption_spec" => self.update_encryption_spec(id, input).await,
            "document" => self.update_document(id, input).await,
            "session" => self.update_session(id, input).await,
            "location" => self.update_location(id, input).await,
            "evaluation" => self.update_evaluation(id, input).await,
            "entitie" => self.update_entitie(id, input).await,
            "participant" => self.update_participant(id, input).await,
            "project" => self.update_project(id, input).await,
            "operation" => self.update_operation(id, input).await,
            "conversation" => self.update_conversation(id, input).await,
            "message" => self.update_message(id, input).await,
            "agent" => self.update_agent(id, input).await,
            "intent" => self.update_intent(id, input).await,
            "suggestion" => self.update_suggestion(id, input).await,
            "environment" => self.update_environment(id, input).await,
            "context" => self.update_context(id, input).await,
            "version" => self.update_version(id, input).await,
            "answer_record" => self.update_answer_record(id, input).await,
            "tool" => self.update_tool(id, input).await,
            "conversation_profile" => self.update_conversation_profile(id, input).await,
            "deployment" => self.update_deployment(id, input).await,
            "version" => self.update_version(id, input).await,
            "security_setting" => self.update_security_setting(id, input).await,
            "test_case" => self.update_test_case(id, input).await,
            "continuous_test_result" => self.update_continuous_test_result(id, input).await,
            "session" => self.update_session(id, input).await,
            "page" => self.update_page(id, input).await,
            "webhook" => self.update_webhook(id, input).await,
            "environment" => self.update_environment(id, input).await,
            "changelog" => self.update_changelog(id, input).await,
            "agent" => self.update_agent(id, input).await,
            "playbook" => self.update_playbook(id, input).await,
            "intent" => self.update_intent(id, input).await,
            "entity_type" => self.update_entity_type(id, input).await,
            "operation" => self.update_operation(id, input).await,
            "flow" => self.update_flow(id, input).await,
            "example" => self.update_example(id, input).await,
            "tool" => self.update_tool(id, input).await,
            "location" => self.update_location(id, input).await,
            "generator" => self.update_generator(id, input).await,
            "transition_route_group" => self.update_transition_route_group(id, input).await,
            "result" => self.update_result(id, input).await,
            "experiment" => self.update_experiment(id, input).await,
            "operation" => self.update_operation(id, input).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "dialogflow_api", resource_name
            ))),
        }
    }

    /// Delete a resource
    pub async fn delete_resource(&self, resource_name: &str, id: &str) -> Result<()> {
        match resource_name {
            "test_case" => self.delete_test_case(id).await,
            "result" => self.delete_result(id).await,
            "environment" => self.delete_environment(id).await,
            "page" => self.delete_page(id).await,
            "intent" => self.delete_intent(id).await,
            "session" => self.delete_session(id).await,
            "deployment" => self.delete_deployment(id).await,
            "security_setting" => self.delete_security_setting(id).await,
            "webhook" => self.delete_webhook(id).await,
            "location" => self.delete_location(id).await,
            "changelog" => self.delete_changelog(id).await,
            "entity_type" => self.delete_entity_type(id).await,
            "transition_route_group" => self.delete_transition_route_group(id).await,
            "playbook" => self.delete_playbook(id).await,
            "generator" => self.delete_generator(id).await,
            "tool" => self.delete_tool(id).await,
            "experiment" => self.delete_experiment(id).await,
            "example" => self.delete_example(id).await,
            "continuous_test_result" => self.delete_continuous_test_result(id).await,
            "flow" => self.delete_flow(id).await,
            "agent" => self.delete_agent(id).await,
            "version" => self.delete_version(id).await,
            "operation" => self.delete_operation(id).await,
            "conversation" => self.delete_conversation(id).await,
            "environment" => self.delete_environment(id).await,
            "conversation_profile" => self.delete_conversation_profile(id).await,
            "version" => self.delete_version(id).await,
            "context" => self.delete_context(id).await,
            "project" => self.delete_project(id).await,
            "intent" => self.delete_intent(id).await,
            "session" => self.delete_session(id).await,
            "conversation" => self.delete_conversation(id).await,
            "entity_type" => self.delete_entity_type(id).await,
            "location" => self.delete_location(id).await,
            "agent" => self.delete_agent(id).await,
            "tool" => self.delete_tool(id).await,
            "encryption_spec" => self.delete_encryption_spec(id).await,
            "suggestion" => self.delete_suggestion(id).await,
            "entitie" => self.delete_entitie(id).await,
            "knowledge_base" => self.delete_knowledge_base(id).await,
            "message" => self.delete_message(id).await,
            "phone_number" => self.delete_phone_number(id).await,
            "evaluation" => self.delete_evaluation(id).await,
            "document" => self.delete_document(id).await,
            "participant" => self.delete_participant(id).await,
            "generator" => self.delete_generator(id).await,
            "operation" => self.delete_operation(id).await,
            "stateless_suggestion" => self.delete_stateless_suggestion(id).await,
            "sip_trunk" => self.delete_sip_trunk(id).await,
            "answer_record" => self.delete_answer_record(id).await,
            "stateless_suggestion" => self.delete_stateless_suggestion(id).await,
            "sip_trunk" => self.delete_sip_trunk(id).await,
            "entity_type" => self.delete_entity_type(id).await,
            "conversation_model" => self.delete_conversation_model(id).await,
            "knowledge_base" => self.delete_knowledge_base(id).await,
            "conversation_dataset" => self.delete_conversation_dataset(id).await,
            "generator" => self.delete_generator(id).await,
            "encryption_spec" => self.delete_encryption_spec(id).await,
            "document" => self.delete_document(id).await,
            "session" => self.delete_session(id).await,
            "location" => self.delete_location(id).await,
            "evaluation" => self.delete_evaluation(id).await,
            "entitie" => self.delete_entitie(id).await,
            "participant" => self.delete_participant(id).await,
            "project" => self.delete_project(id).await,
            "operation" => self.delete_operation(id).await,
            "conversation" => self.delete_conversation(id).await,
            "message" => self.delete_message(id).await,
            "agent" => self.delete_agent(id).await,
            "intent" => self.delete_intent(id).await,
            "suggestion" => self.delete_suggestion(id).await,
            "environment" => self.delete_environment(id).await,
            "context" => self.delete_context(id).await,
            "version" => self.delete_version(id).await,
            "answer_record" => self.delete_answer_record(id).await,
            "tool" => self.delete_tool(id).await,
            "conversation_profile" => self.delete_conversation_profile(id).await,
            "deployment" => self.delete_deployment(id).await,
            "version" => self.delete_version(id).await,
            "security_setting" => self.delete_security_setting(id).await,
            "test_case" => self.delete_test_case(id).await,
            "continuous_test_result" => self.delete_continuous_test_result(id).await,
            "session" => self.delete_session(id).await,
            "page" => self.delete_page(id).await,
            "webhook" => self.delete_webhook(id).await,
            "environment" => self.delete_environment(id).await,
            "changelog" => self.delete_changelog(id).await,
            "agent" => self.delete_agent(id).await,
            "playbook" => self.delete_playbook(id).await,
            "intent" => self.delete_intent(id).await,
            "entity_type" => self.delete_entity_type(id).await,
            "operation" => self.delete_operation(id).await,
            "flow" => self.delete_flow(id).await,
            "example" => self.delete_example(id).await,
            "tool" => self.delete_tool(id).await,
            "location" => self.delete_location(id).await,
            "generator" => self.delete_generator(id).await,
            "transition_route_group" => self.delete_transition_route_group(id).await,
            "result" => self.delete_result(id).await,
            "experiment" => self.delete_experiment(id).await,
            "operation" => self.delete_operation(id).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "dialogflow_api", resource_name
            ))),
        }
    }

    // ========================================================================
    // Resource-specific CRUD implementations
    // ========================================================================

    // ------------------------------------------------------------------------
    // Test_case resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a test_case resource
    async fn plan_test_case(
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

    /// Create a new test_case resource
    async fn create_test_case(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a test_case resource
    async fn read_test_case(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a test_case resource
    async fn update_test_case(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a test_case resource
    async fn delete_test_case(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Result resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a result resource
    async fn plan_result(
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

    /// Create a new result resource
    async fn create_result(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a result resource
    async fn read_result(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a result resource
    async fn update_result(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a result resource
    async fn delete_result(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Environment resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a environment resource
    async fn plan_environment(
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

    /// Create a new environment resource
    async fn create_environment(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a environment resource
    async fn read_environment(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a environment resource
    async fn update_environment(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a environment resource
    async fn delete_environment(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Page resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a page resource
    async fn plan_page(
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

    /// Create a new page resource
    async fn create_page(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a page resource
    async fn read_page(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a page resource
    async fn update_page(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a page resource
    async fn delete_page(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Intent resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a intent resource
    async fn plan_intent(
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

    /// Create a new intent resource
    async fn create_intent(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a intent resource
    async fn read_intent(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a intent resource
    async fn update_intent(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a intent resource
    async fn delete_intent(&self, id: &str) -> Result<()> {
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
    // Deployment resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a deployment resource
    async fn plan_deployment(
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

    /// Create a new deployment resource
    async fn create_deployment(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a deployment resource
    async fn read_deployment(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a deployment resource
    async fn update_deployment(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a deployment resource
    async fn delete_deployment(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Security_setting resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a security_setting resource
    async fn plan_security_setting(
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

    /// Create a new security_setting resource
    async fn create_security_setting(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a security_setting resource
    async fn read_security_setting(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a security_setting resource
    async fn update_security_setting(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a security_setting resource
    async fn delete_security_setting(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Webhook resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a webhook resource
    async fn plan_webhook(
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

    /// Create a new webhook resource
    async fn create_webhook(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a webhook resource
    async fn read_webhook(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a webhook resource
    async fn update_webhook(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a webhook resource
    async fn delete_webhook(&self, id: &str) -> Result<()> {
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
    // Changelog resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a changelog resource
    async fn plan_changelog(
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

    /// Create a new changelog resource
    async fn create_changelog(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a changelog resource
    async fn read_changelog(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a changelog resource
    async fn update_changelog(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a changelog resource
    async fn delete_changelog(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Entity_type resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a entity_type resource
    async fn plan_entity_type(
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

    /// Create a new entity_type resource
    async fn create_entity_type(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a entity_type resource
    async fn read_entity_type(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a entity_type resource
    async fn update_entity_type(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a entity_type resource
    async fn delete_entity_type(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Transition_route_group resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a transition_route_group resource
    async fn plan_transition_route_group(
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

    /// Create a new transition_route_group resource
    async fn create_transition_route_group(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a transition_route_group resource
    async fn read_transition_route_group(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a transition_route_group resource
    async fn update_transition_route_group(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a transition_route_group resource
    async fn delete_transition_route_group(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Playbook resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a playbook resource
    async fn plan_playbook(
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

    /// Create a new playbook resource
    async fn create_playbook(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a playbook resource
    async fn read_playbook(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a playbook resource
    async fn update_playbook(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a playbook resource
    async fn delete_playbook(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Generator resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a generator resource
    async fn plan_generator(
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

    /// Create a new generator resource
    async fn create_generator(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a generator resource
    async fn read_generator(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a generator resource
    async fn update_generator(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a generator resource
    async fn delete_generator(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Tool resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a tool resource
    async fn plan_tool(
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

    /// Create a new tool resource
    async fn create_tool(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a tool resource
    async fn read_tool(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a tool resource
    async fn update_tool(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a tool resource
    async fn delete_tool(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Experiment resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a experiment resource
    async fn plan_experiment(
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

    /// Create a new experiment resource
    async fn create_experiment(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a experiment resource
    async fn read_experiment(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a experiment resource
    async fn update_experiment(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a experiment resource
    async fn delete_experiment(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Example resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a example resource
    async fn plan_example(
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

    /// Create a new example resource
    async fn create_example(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a example resource
    async fn read_example(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a example resource
    async fn update_example(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a example resource
    async fn delete_example(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Continuous_test_result resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a continuous_test_result resource
    async fn plan_continuous_test_result(
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

    /// Create a new continuous_test_result resource
    async fn create_continuous_test_result(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a continuous_test_result resource
    async fn read_continuous_test_result(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a continuous_test_result resource
    async fn update_continuous_test_result(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a continuous_test_result resource
    async fn delete_continuous_test_result(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Flow resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a flow resource
    async fn plan_flow(
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

    /// Create a new flow resource
    async fn create_flow(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a flow resource
    async fn read_flow(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a flow resource
    async fn update_flow(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a flow resource
    async fn delete_flow(&self, id: &str) -> Result<()> {
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
    // Version resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a version resource
    async fn plan_version(
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

    /// Create a new version resource
    async fn create_version(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a version resource
    async fn read_version(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a version resource
    async fn update_version(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a version resource
    async fn delete_version(&self, id: &str) -> Result<()> {
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
    // Environment resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a environment resource
    async fn plan_environment(
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

    /// Create a new environment resource
    async fn create_environment(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a environment resource
    async fn read_environment(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a environment resource
    async fn update_environment(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a environment resource
    async fn delete_environment(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Conversation_profile resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a conversation_profile resource
    async fn plan_conversation_profile(
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

    /// Create a new conversation_profile resource
    async fn create_conversation_profile(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a conversation_profile resource
    async fn read_conversation_profile(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a conversation_profile resource
    async fn update_conversation_profile(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a conversation_profile resource
    async fn delete_conversation_profile(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Version resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a version resource
    async fn plan_version(
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

    /// Create a new version resource
    async fn create_version(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a version resource
    async fn read_version(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a version resource
    async fn update_version(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a version resource
    async fn delete_version(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Context resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a context resource
    async fn plan_context(
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

    /// Create a new context resource
    async fn create_context(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a context resource
    async fn read_context(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a context resource
    async fn update_context(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a context resource
    async fn delete_context(&self, id: &str) -> Result<()> {
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
    // Intent resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a intent resource
    async fn plan_intent(
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

    /// Create a new intent resource
    async fn create_intent(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a intent resource
    async fn read_intent(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a intent resource
    async fn update_intent(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a intent resource
    async fn delete_intent(&self, id: &str) -> Result<()> {
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
    // Entity_type resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a entity_type resource
    async fn plan_entity_type(
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

    /// Create a new entity_type resource
    async fn create_entity_type(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a entity_type resource
    async fn read_entity_type(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a entity_type resource
    async fn update_entity_type(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a entity_type resource
    async fn delete_entity_type(&self, id: &str) -> Result<()> {
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
    // Tool resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a tool resource
    async fn plan_tool(
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

    /// Create a new tool resource
    async fn create_tool(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a tool resource
    async fn read_tool(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a tool resource
    async fn update_tool(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a tool resource
    async fn delete_tool(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Encryption_spec resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a encryption_spec resource
    async fn plan_encryption_spec(
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

    /// Create a new encryption_spec resource
    async fn create_encryption_spec(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a encryption_spec resource
    async fn read_encryption_spec(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a encryption_spec resource
    async fn update_encryption_spec(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a encryption_spec resource
    async fn delete_encryption_spec(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Suggestion resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a suggestion resource
    async fn plan_suggestion(
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

    /// Create a new suggestion resource
    async fn create_suggestion(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a suggestion resource
    async fn read_suggestion(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a suggestion resource
    async fn update_suggestion(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a suggestion resource
    async fn delete_suggestion(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Entitie resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a entitie resource
    async fn plan_entitie(
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

    /// Create a new entitie resource
    async fn create_entitie(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a entitie resource
    async fn read_entitie(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a entitie resource
    async fn update_entitie(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a entitie resource
    async fn delete_entitie(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Knowledge_base resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a knowledge_base resource
    async fn plan_knowledge_base(
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

    /// Create a new knowledge_base resource
    async fn create_knowledge_base(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a knowledge_base resource
    async fn read_knowledge_base(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a knowledge_base resource
    async fn update_knowledge_base(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a knowledge_base resource
    async fn delete_knowledge_base(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Message resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a message resource
    async fn plan_message(
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

    /// Create a new message resource
    async fn create_message(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a message resource
    async fn read_message(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a message resource
    async fn update_message(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a message resource
    async fn delete_message(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Phone_number resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a phone_number resource
    async fn plan_phone_number(
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

    /// Create a new phone_number resource
    async fn create_phone_number(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a phone_number resource
    async fn read_phone_number(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a phone_number resource
    async fn update_phone_number(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a phone_number resource
    async fn delete_phone_number(&self, id: &str) -> Result<()> {
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
    // Participant resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a participant resource
    async fn plan_participant(
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

    /// Create a new participant resource
    async fn create_participant(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a participant resource
    async fn read_participant(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a participant resource
    async fn update_participant(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a participant resource
    async fn delete_participant(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Generator resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a generator resource
    async fn plan_generator(
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

    /// Create a new generator resource
    async fn create_generator(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a generator resource
    async fn read_generator(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a generator resource
    async fn update_generator(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a generator resource
    async fn delete_generator(&self, id: &str) -> Result<()> {
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
    // Stateless_suggestion resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a stateless_suggestion resource
    async fn plan_stateless_suggestion(
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

    /// Create a new stateless_suggestion resource
    async fn create_stateless_suggestion(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a stateless_suggestion resource
    async fn read_stateless_suggestion(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a stateless_suggestion resource
    async fn update_stateless_suggestion(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a stateless_suggestion resource
    async fn delete_stateless_suggestion(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Sip_trunk resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a sip_trunk resource
    async fn plan_sip_trunk(
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

    /// Create a new sip_trunk resource
    async fn create_sip_trunk(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a sip_trunk resource
    async fn read_sip_trunk(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a sip_trunk resource
    async fn update_sip_trunk(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a sip_trunk resource
    async fn delete_sip_trunk(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Answer_record resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a answer_record resource
    async fn plan_answer_record(
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

    /// Create a new answer_record resource
    async fn create_answer_record(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a answer_record resource
    async fn read_answer_record(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a answer_record resource
    async fn update_answer_record(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a answer_record resource
    async fn delete_answer_record(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Stateless_suggestion resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a stateless_suggestion resource
    async fn plan_stateless_suggestion(
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

    /// Create a new stateless_suggestion resource
    async fn create_stateless_suggestion(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a stateless_suggestion resource
    async fn read_stateless_suggestion(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a stateless_suggestion resource
    async fn update_stateless_suggestion(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a stateless_suggestion resource
    async fn delete_stateless_suggestion(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Sip_trunk resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a sip_trunk resource
    async fn plan_sip_trunk(
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

    /// Create a new sip_trunk resource
    async fn create_sip_trunk(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a sip_trunk resource
    async fn read_sip_trunk(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a sip_trunk resource
    async fn update_sip_trunk(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a sip_trunk resource
    async fn delete_sip_trunk(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Entity_type resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a entity_type resource
    async fn plan_entity_type(
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

    /// Create a new entity_type resource
    async fn create_entity_type(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a entity_type resource
    async fn read_entity_type(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a entity_type resource
    async fn update_entity_type(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a entity_type resource
    async fn delete_entity_type(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Conversation_model resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a conversation_model resource
    async fn plan_conversation_model(
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

    /// Create a new conversation_model resource
    async fn create_conversation_model(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a conversation_model resource
    async fn read_conversation_model(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a conversation_model resource
    async fn update_conversation_model(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a conversation_model resource
    async fn delete_conversation_model(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Knowledge_base resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a knowledge_base resource
    async fn plan_knowledge_base(
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

    /// Create a new knowledge_base resource
    async fn create_knowledge_base(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a knowledge_base resource
    async fn read_knowledge_base(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a knowledge_base resource
    async fn update_knowledge_base(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a knowledge_base resource
    async fn delete_knowledge_base(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Conversation_dataset resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a conversation_dataset resource
    async fn plan_conversation_dataset(
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

    /// Create a new conversation_dataset resource
    async fn create_conversation_dataset(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a conversation_dataset resource
    async fn read_conversation_dataset(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a conversation_dataset resource
    async fn update_conversation_dataset(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a conversation_dataset resource
    async fn delete_conversation_dataset(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Generator resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a generator resource
    async fn plan_generator(
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

    /// Create a new generator resource
    async fn create_generator(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a generator resource
    async fn read_generator(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a generator resource
    async fn update_generator(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a generator resource
    async fn delete_generator(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Encryption_spec resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a encryption_spec resource
    async fn plan_encryption_spec(
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

    /// Create a new encryption_spec resource
    async fn create_encryption_spec(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a encryption_spec resource
    async fn read_encryption_spec(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a encryption_spec resource
    async fn update_encryption_spec(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a encryption_spec resource
    async fn delete_encryption_spec(&self, id: &str) -> Result<()> {
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
    // Entitie resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a entitie resource
    async fn plan_entitie(
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

    /// Create a new entitie resource
    async fn create_entitie(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a entitie resource
    async fn read_entitie(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a entitie resource
    async fn update_entitie(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a entitie resource
    async fn delete_entitie(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Participant resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a participant resource
    async fn plan_participant(
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

    /// Create a new participant resource
    async fn create_participant(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a participant resource
    async fn read_participant(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a participant resource
    async fn update_participant(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a participant resource
    async fn delete_participant(&self, id: &str) -> Result<()> {
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
    // Message resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a message resource
    async fn plan_message(
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

    /// Create a new message resource
    async fn create_message(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a message resource
    async fn read_message(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a message resource
    async fn update_message(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a message resource
    async fn delete_message(&self, id: &str) -> Result<()> {
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
    // Intent resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a intent resource
    async fn plan_intent(
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

    /// Create a new intent resource
    async fn create_intent(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a intent resource
    async fn read_intent(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a intent resource
    async fn update_intent(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a intent resource
    async fn delete_intent(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Suggestion resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a suggestion resource
    async fn plan_suggestion(
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

    /// Create a new suggestion resource
    async fn create_suggestion(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a suggestion resource
    async fn read_suggestion(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a suggestion resource
    async fn update_suggestion(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a suggestion resource
    async fn delete_suggestion(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Environment resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a environment resource
    async fn plan_environment(
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

    /// Create a new environment resource
    async fn create_environment(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a environment resource
    async fn read_environment(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a environment resource
    async fn update_environment(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a environment resource
    async fn delete_environment(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Context resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a context resource
    async fn plan_context(
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

    /// Create a new context resource
    async fn create_context(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a context resource
    async fn read_context(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a context resource
    async fn update_context(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a context resource
    async fn delete_context(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Version resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a version resource
    async fn plan_version(
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

    /// Create a new version resource
    async fn create_version(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a version resource
    async fn read_version(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a version resource
    async fn update_version(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a version resource
    async fn delete_version(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Answer_record resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a answer_record resource
    async fn plan_answer_record(
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

    /// Create a new answer_record resource
    async fn create_answer_record(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a answer_record resource
    async fn read_answer_record(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a answer_record resource
    async fn update_answer_record(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a answer_record resource
    async fn delete_answer_record(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Tool resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a tool resource
    async fn plan_tool(
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

    /// Create a new tool resource
    async fn create_tool(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a tool resource
    async fn read_tool(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a tool resource
    async fn update_tool(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a tool resource
    async fn delete_tool(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Conversation_profile resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a conversation_profile resource
    async fn plan_conversation_profile(
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

    /// Create a new conversation_profile resource
    async fn create_conversation_profile(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a conversation_profile resource
    async fn read_conversation_profile(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a conversation_profile resource
    async fn update_conversation_profile(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a conversation_profile resource
    async fn delete_conversation_profile(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Deployment resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a deployment resource
    async fn plan_deployment(
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

    /// Create a new deployment resource
    async fn create_deployment(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a deployment resource
    async fn read_deployment(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a deployment resource
    async fn update_deployment(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a deployment resource
    async fn delete_deployment(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Version resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a version resource
    async fn plan_version(
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

    /// Create a new version resource
    async fn create_version(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a version resource
    async fn read_version(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a version resource
    async fn update_version(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a version resource
    async fn delete_version(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Security_setting resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a security_setting resource
    async fn plan_security_setting(
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

    /// Create a new security_setting resource
    async fn create_security_setting(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a security_setting resource
    async fn read_security_setting(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a security_setting resource
    async fn update_security_setting(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a security_setting resource
    async fn delete_security_setting(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Test_case resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a test_case resource
    async fn plan_test_case(
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

    /// Create a new test_case resource
    async fn create_test_case(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a test_case resource
    async fn read_test_case(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a test_case resource
    async fn update_test_case(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a test_case resource
    async fn delete_test_case(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Continuous_test_result resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a continuous_test_result resource
    async fn plan_continuous_test_result(
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

    /// Create a new continuous_test_result resource
    async fn create_continuous_test_result(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a continuous_test_result resource
    async fn read_continuous_test_result(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a continuous_test_result resource
    async fn update_continuous_test_result(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a continuous_test_result resource
    async fn delete_continuous_test_result(&self, id: &str) -> Result<()> {
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
    // Page resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a page resource
    async fn plan_page(
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

    /// Create a new page resource
    async fn create_page(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a page resource
    async fn read_page(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a page resource
    async fn update_page(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a page resource
    async fn delete_page(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Webhook resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a webhook resource
    async fn plan_webhook(
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

    /// Create a new webhook resource
    async fn create_webhook(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a webhook resource
    async fn read_webhook(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a webhook resource
    async fn update_webhook(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a webhook resource
    async fn delete_webhook(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Environment resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a environment resource
    async fn plan_environment(
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

    /// Create a new environment resource
    async fn create_environment(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a environment resource
    async fn read_environment(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a environment resource
    async fn update_environment(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a environment resource
    async fn delete_environment(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Changelog resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a changelog resource
    async fn plan_changelog(
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

    /// Create a new changelog resource
    async fn create_changelog(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a changelog resource
    async fn read_changelog(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a changelog resource
    async fn update_changelog(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a changelog resource
    async fn delete_changelog(&self, id: &str) -> Result<()> {
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
    // Playbook resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a playbook resource
    async fn plan_playbook(
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

    /// Create a new playbook resource
    async fn create_playbook(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a playbook resource
    async fn read_playbook(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a playbook resource
    async fn update_playbook(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a playbook resource
    async fn delete_playbook(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Intent resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a intent resource
    async fn plan_intent(
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

    /// Create a new intent resource
    async fn create_intent(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a intent resource
    async fn read_intent(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a intent resource
    async fn update_intent(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a intent resource
    async fn delete_intent(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Entity_type resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a entity_type resource
    async fn plan_entity_type(
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

    /// Create a new entity_type resource
    async fn create_entity_type(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a entity_type resource
    async fn read_entity_type(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a entity_type resource
    async fn update_entity_type(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a entity_type resource
    async fn delete_entity_type(&self, id: &str) -> Result<()> {
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
    // Flow resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a flow resource
    async fn plan_flow(
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

    /// Create a new flow resource
    async fn create_flow(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a flow resource
    async fn read_flow(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a flow resource
    async fn update_flow(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a flow resource
    async fn delete_flow(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Example resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a example resource
    async fn plan_example(
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

    /// Create a new example resource
    async fn create_example(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a example resource
    async fn read_example(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a example resource
    async fn update_example(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a example resource
    async fn delete_example(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Tool resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a tool resource
    async fn plan_tool(
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

    /// Create a new tool resource
    async fn create_tool(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a tool resource
    async fn read_tool(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a tool resource
    async fn update_tool(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a tool resource
    async fn delete_tool(&self, id: &str) -> Result<()> {
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
    // Generator resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a generator resource
    async fn plan_generator(
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

    /// Create a new generator resource
    async fn create_generator(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a generator resource
    async fn read_generator(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a generator resource
    async fn update_generator(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a generator resource
    async fn delete_generator(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Transition_route_group resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a transition_route_group resource
    async fn plan_transition_route_group(
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

    /// Create a new transition_route_group resource
    async fn create_transition_route_group(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a transition_route_group resource
    async fn read_transition_route_group(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a transition_route_group resource
    async fn update_transition_route_group(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a transition_route_group resource
    async fn delete_transition_route_group(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Result resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a result resource
    async fn plan_result(
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

    /// Create a new result resource
    async fn create_result(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a result resource
    async fn read_result(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a result resource
    async fn update_result(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a result resource
    async fn delete_result(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Experiment resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a experiment resource
    async fn plan_experiment(
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

    /// Create a new experiment resource
    async fn create_experiment(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a experiment resource
    async fn read_experiment(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a experiment resource
    async fn update_experiment(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a experiment resource
    async fn delete_experiment(&self, id: &str) -> Result<()> {
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
}
