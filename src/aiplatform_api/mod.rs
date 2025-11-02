//! Aiplatform_api service for Gcp provider
//!
//! This module handles all aiplatform_api resources and their CRUD operations.

use hemmer_core::Result;
use hemmer_provider::{ResourceInput, ResourceOutput, ResourcePlan};

/// Aiplatform_api service handler
pub struct Aiplatform_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Aiplatform_apiService<'a> {
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
            "location" => self.plan_location(current_state, desired_input).await,
            "evaluation_item" => {
                self.plan_evaluation_item(current_state, desired_input)
                    .await
            }
            "entity_type" => self.plan_entity_type(current_state, desired_input).await,
            "metadata_schema" => {
                self.plan_metadata_schema(current_state, desired_input)
                    .await
            }
            "indexe" => self.plan_indexe(current_state, desired_input).await,
            "model" => self.plan_model(current_state, desired_input).await,
            "migratable_resource" => {
                self.plan_migratable_resource(current_state, desired_input)
                    .await
            }
            "rag_file" => self.plan_rag_file(current_state, desired_input).await,
            "notebook_runtime" => {
                self.plan_notebook_runtime(current_state, desired_input)
                    .await
            }
            "feature_group" => self.plan_feature_group(current_state, desired_input).await,
            "data_item" => self.plan_data_item(current_state, desired_input).await,
            "feature_online_store" => {
                self.plan_feature_online_store(current_state, desired_input)
                    .await
            }
            "experiment" => self.plan_experiment(current_state, desired_input).await,
            "context" => self.plan_context(current_state, desired_input).await,
            "media" => self.plan_media(current_state, desired_input).await,
            "invoke" => self.plan_invoke(current_state, desired_input).await,
            "annotation_spec" => {
                self.plan_annotation_spec(current_state, desired_input)
                    .await
            }
            "endpoint" => self.plan_endpoint(current_state, desired_input).await,
            "model_deployment_monitoring_job" => {
                self.plan_model_deployment_monitoring_job(current_state, desired_input)
                    .await
            }
            "featurestore" => self.plan_featurestore(current_state, desired_input).await,
            "dataset" => self.plan_dataset(current_state, desired_input).await,
            "training_pipeline" => {
                self.plan_training_pipeline(current_state, desired_input)
                    .await
            }
            "feature" => self.plan_feature(current_state, desired_input).await,
            "dataset_version" => {
                self.plan_dataset_version(current_state, desired_input)
                    .await
            }
            "tensorboard" => self.plan_tensorboard(current_state, desired_input).await,
            "index_endpoint" => self.plan_index_endpoint(current_state, desired_input).await,
            "artifact" => self.plan_artifact(current_state, desired_input).await,
            "custom_job" => self.plan_custom_job(current_state, desired_input).await,
            "execution" => self.plan_execution(current_state, desired_input).await,
            "reasoning_engine" => {
                self.plan_reasoning_engine(current_state, desired_input)
                    .await
            }
            "evaluation_run" => self.plan_evaluation_run(current_state, desired_input).await,
            "pipeline_job" => self.plan_pipeline_job(current_state, desired_input).await,
            "evaluation_set" => self.plan_evaluation_set(current_state, desired_input).await,
            "nas_trial_detail" => {
                self.plan_nas_trial_detail(current_state, desired_input)
                    .await
            }
            "tuning_job" => self.plan_tuning_job(current_state, desired_input).await,
            "run" => self.plan_run(current_state, desired_input).await,
            "metadata_store" => self.plan_metadata_store(current_state, desired_input).await,
            "slice" => self.plan_slice(current_state, desired_input).await,
            "time_serie" => self.plan_time_serie(current_state, desired_input).await,
            "persistent_resource" => {
                self.plan_persistent_resource(current_state, desired_input)
                    .await
            }
            "annotation" => self.plan_annotation(current_state, desired_input).await,
            "hyperparameter_tuning_job" => {
                self.plan_hyperparameter_tuning_job(current_state, desired_input)
                    .await
            }
            "notebook_runtime_template" => {
                self.plan_notebook_runtime_template(current_state, desired_input)
                    .await
            }
            "project" => self.plan_project(current_state, desired_input).await,
            "notebook_execution_job" => {
                self.plan_notebook_execution_job(current_state, desired_input)
                    .await
            }
            "chat" => self.plan_chat(current_state, desired_input).await,
            "cached_content" => self.plan_cached_content(current_state, desired_input).await,
            "nas_job" => self.plan_nas_job(current_state, desired_input).await,
            "operation" => self.plan_operation(current_state, desired_input).await,
            "rag_corpora" => self.plan_rag_corpora(current_state, desired_input).await,
            "data_labeling_job" => {
                self.plan_data_labeling_job(current_state, desired_input)
                    .await
            }
            "deployment_resource_pool" => {
                self.plan_deployment_resource_pool(current_state, desired_input)
                    .await
            }
            "feature_view" => self.plan_feature_view(current_state, desired_input).await,
            "evaluation" => self.plan_evaluation(current_state, desired_input).await,
            "schedule" => self.plan_schedule(current_state, desired_input).await,
            "batch_prediction_job" => {
                self.plan_batch_prediction_job(current_state, desired_input)
                    .await
            }
            "feature_view_sync" => {
                self.plan_feature_view_sync(current_state, desired_input)
                    .await
            }
            "studie" => self.plan_studie(current_state, desired_input).await,
            "trial" => self.plan_trial(current_state, desired_input).await,
            "specialist_pool" => {
                self.plan_specialist_pool(current_state, desired_input)
                    .await
            }
            "openapi" => self.plan_openapi(current_state, desired_input).await,
            "saved_querie" => self.plan_saved_querie(current_state, desired_input).await,
            "feature_view" => self.plan_feature_view(current_state, desired_input).await,
            "artifact" => self.plan_artifact(current_state, desired_input).await,
            "notebook_runtime" => {
                self.plan_notebook_runtime(current_state, desired_input)
                    .await
            }
            "annotation" => self.plan_annotation(current_state, desired_input).await,
            "notebook_runtime_template" => {
                self.plan_notebook_runtime_template(current_state, desired_input)
                    .await
            }
            "saved_querie" => self.plan_saved_querie(current_state, desired_input).await,
            "tuning_job" => self.plan_tuning_job(current_state, desired_input).await,
            "indexe" => self.plan_indexe(current_state, desired_input).await,
            "index_endpoint" => self.plan_index_endpoint(current_state, desired_input).await,
            "batch_prediction_job" => {
                self.plan_batch_prediction_job(current_state, desired_input)
                    .await
            }
            "location" => self.plan_location(current_state, desired_input).await,
            "featurestore" => self.plan_featurestore(current_state, desired_input).await,
            "chat" => self.plan_chat(current_state, desired_input).await,
            "run" => self.plan_run(current_state, desired_input).await,
            "rag_corpora" => self.plan_rag_corpora(current_state, desired_input).await,
            "persistent_resource" => {
                self.plan_persistent_resource(current_state, desired_input)
                    .await
            }
            "feature" => self.plan_feature(current_state, desired_input).await,
            "hyperparameter_tuning_job" => {
                self.plan_hyperparameter_tuning_job(current_state, desired_input)
                    .await
            }
            "specialist_pool" => {
                self.plan_specialist_pool(current_state, desired_input)
                    .await
            }
            "trial" => self.plan_trial(current_state, desired_input).await,
            "session" => self.plan_session(current_state, desired_input).await,
            "dataset" => self.plan_dataset(current_state, desired_input).await,
            "reasoning_engine" => {
                self.plan_reasoning_engine(current_state, desired_input)
                    .await
            }
            "feature_view_sync" => {
                self.plan_feature_view_sync(current_state, desired_input)
                    .await
            }
            "execution" => self.plan_execution(current_state, desired_input).await,
            "memorie" => self.plan_memorie(current_state, desired_input).await,
            "annotation_spec" => {
                self.plan_annotation_spec(current_state, desired_input)
                    .await
            }
            "feature_online_store" => {
                self.plan_feature_online_store(current_state, desired_input)
                    .await
            }
            "migratable_resource" => {
                self.plan_migratable_resource(current_state, desired_input)
                    .await
            }
            "feature_monitor" => {
                self.plan_feature_monitor(current_state, desired_input)
                    .await
            }
            "event" => self.plan_event(current_state, desired_input).await,
            "nas_job" => self.plan_nas_job(current_state, desired_input).await,
            "context" => self.plan_context(current_state, desired_input).await,
            "media" => self.plan_media(current_state, desired_input).await,
            "experiment" => self.plan_experiment(current_state, desired_input).await,
            "example_store" => self.plan_example_store(current_state, desired_input).await,
            "evaluation" => self.plan_evaluation(current_state, desired_input).await,
            "dataset_version" => {
                self.plan_dataset_version(current_state, desired_input)
                    .await
            }
            "model" => self.plan_model(current_state, desired_input).await,
            "feature_group" => self.plan_feature_group(current_state, desired_input).await,
            "operation" => self.plan_operation(current_state, desired_input).await,
            "studie" => self.plan_studie(current_state, desired_input).await,
            "metadata_schema" => {
                self.plan_metadata_schema(current_state, desired_input)
                    .await
            }
            "model_monitor" => self.plan_model_monitor(current_state, desired_input).await,
            "deployment_resource_pool" => {
                self.plan_deployment_resource_pool(current_state, desired_input)
                    .await
            }
            "model_monitoring_job" => {
                self.plan_model_monitoring_job(current_state, desired_input)
                    .await
            }
            "model_deployment_monitoring_job" => {
                self.plan_model_deployment_monitoring_job(current_state, desired_input)
                    .await
            }
            "endpoint" => self.plan_endpoint(current_state, desired_input).await,
            "schedule" => self.plan_schedule(current_state, desired_input).await,
            "model_garden_eula" => {
                self.plan_model_garden_eula(current_state, desired_input)
                    .await
            }
            "feature_monitor_job" => {
                self.plan_feature_monitor_job(current_state, desired_input)
                    .await
            }
            "notebook_execution_job" => {
                self.plan_notebook_execution_job(current_state, desired_input)
                    .await
            }
            "tensorboard" => self.plan_tensorboard(current_state, desired_input).await,
            "rag_file" => self.plan_rag_file(current_state, desired_input).await,
            "cached_content" => self.plan_cached_content(current_state, desired_input).await,
            "evaluation_item" => {
                self.plan_evaluation_item(current_state, desired_input)
                    .await
            }
            "slice" => self.plan_slice(current_state, desired_input).await,
            "extension" => self.plan_extension(current_state, desired_input).await,
            "custom_job" => self.plan_custom_job(current_state, desired_input).await,
            "data_item" => self.plan_data_item(current_state, desired_input).await,
            "evaluation_run" => self.plan_evaluation_run(current_state, desired_input).await,
            "time_serie" => self.plan_time_serie(current_state, desired_input).await,
            "sandbox_environment" => {
                self.plan_sandbox_environment(current_state, desired_input)
                    .await
            }
            "project" => self.plan_project(current_state, desired_input).await,
            "evaluation_set" => self.plan_evaluation_set(current_state, desired_input).await,
            "entity_type" => self.plan_entity_type(current_state, desired_input).await,
            "data_labeling_job" => {
                self.plan_data_labeling_job(current_state, desired_input)
                    .await
            }
            "pipeline_job" => self.plan_pipeline_job(current_state, desired_input).await,
            "training_pipeline" => {
                self.plan_training_pipeline(current_state, desired_input)
                    .await
            }
            "nas_trial_detail" => {
                self.plan_nas_trial_detail(current_state, desired_input)
                    .await
            }
            "metadata_store" => self.plan_metadata_store(current_state, desired_input).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "aiplatform_api", resource_name
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
            "location" => self.create_location(input).await,
            "evaluation_item" => self.create_evaluation_item(input).await,
            "entity_type" => self.create_entity_type(input).await,
            "metadata_schema" => self.create_metadata_schema(input).await,
            "indexe" => self.create_indexe(input).await,
            "model" => self.create_model(input).await,
            "migratable_resource" => self.create_migratable_resource(input).await,
            "rag_file" => self.create_rag_file(input).await,
            "notebook_runtime" => self.create_notebook_runtime(input).await,
            "feature_group" => self.create_feature_group(input).await,
            "data_item" => self.create_data_item(input).await,
            "feature_online_store" => self.create_feature_online_store(input).await,
            "experiment" => self.create_experiment(input).await,
            "context" => self.create_context(input).await,
            "media" => self.create_media(input).await,
            "invoke" => self.create_invoke(input).await,
            "annotation_spec" => self.create_annotation_spec(input).await,
            "endpoint" => self.create_endpoint(input).await,
            "model_deployment_monitoring_job" => {
                self.create_model_deployment_monitoring_job(input).await
            }
            "featurestore" => self.create_featurestore(input).await,
            "dataset" => self.create_dataset(input).await,
            "training_pipeline" => self.create_training_pipeline(input).await,
            "feature" => self.create_feature(input).await,
            "dataset_version" => self.create_dataset_version(input).await,
            "tensorboard" => self.create_tensorboard(input).await,
            "index_endpoint" => self.create_index_endpoint(input).await,
            "artifact" => self.create_artifact(input).await,
            "custom_job" => self.create_custom_job(input).await,
            "execution" => self.create_execution(input).await,
            "reasoning_engine" => self.create_reasoning_engine(input).await,
            "evaluation_run" => self.create_evaluation_run(input).await,
            "pipeline_job" => self.create_pipeline_job(input).await,
            "evaluation_set" => self.create_evaluation_set(input).await,
            "nas_trial_detail" => self.create_nas_trial_detail(input).await,
            "tuning_job" => self.create_tuning_job(input).await,
            "run" => self.create_run(input).await,
            "metadata_store" => self.create_metadata_store(input).await,
            "slice" => self.create_slice(input).await,
            "time_serie" => self.create_time_serie(input).await,
            "persistent_resource" => self.create_persistent_resource(input).await,
            "annotation" => self.create_annotation(input).await,
            "hyperparameter_tuning_job" => self.create_hyperparameter_tuning_job(input).await,
            "notebook_runtime_template" => self.create_notebook_runtime_template(input).await,
            "project" => self.create_project(input).await,
            "notebook_execution_job" => self.create_notebook_execution_job(input).await,
            "chat" => self.create_chat(input).await,
            "cached_content" => self.create_cached_content(input).await,
            "nas_job" => self.create_nas_job(input).await,
            "operation" => self.create_operation(input).await,
            "rag_corpora" => self.create_rag_corpora(input).await,
            "data_labeling_job" => self.create_data_labeling_job(input).await,
            "deployment_resource_pool" => self.create_deployment_resource_pool(input).await,
            "feature_view" => self.create_feature_view(input).await,
            "evaluation" => self.create_evaluation(input).await,
            "schedule" => self.create_schedule(input).await,
            "batch_prediction_job" => self.create_batch_prediction_job(input).await,
            "feature_view_sync" => self.create_feature_view_sync(input).await,
            "studie" => self.create_studie(input).await,
            "trial" => self.create_trial(input).await,
            "specialist_pool" => self.create_specialist_pool(input).await,
            "openapi" => self.create_openapi(input).await,
            "saved_querie" => self.create_saved_querie(input).await,
            "feature_view" => self.create_feature_view(input).await,
            "artifact" => self.create_artifact(input).await,
            "notebook_runtime" => self.create_notebook_runtime(input).await,
            "annotation" => self.create_annotation(input).await,
            "notebook_runtime_template" => self.create_notebook_runtime_template(input).await,
            "saved_querie" => self.create_saved_querie(input).await,
            "tuning_job" => self.create_tuning_job(input).await,
            "indexe" => self.create_indexe(input).await,
            "index_endpoint" => self.create_index_endpoint(input).await,
            "batch_prediction_job" => self.create_batch_prediction_job(input).await,
            "location" => self.create_location(input).await,
            "featurestore" => self.create_featurestore(input).await,
            "chat" => self.create_chat(input).await,
            "run" => self.create_run(input).await,
            "rag_corpora" => self.create_rag_corpora(input).await,
            "persistent_resource" => self.create_persistent_resource(input).await,
            "feature" => self.create_feature(input).await,
            "hyperparameter_tuning_job" => self.create_hyperparameter_tuning_job(input).await,
            "specialist_pool" => self.create_specialist_pool(input).await,
            "trial" => self.create_trial(input).await,
            "session" => self.create_session(input).await,
            "dataset" => self.create_dataset(input).await,
            "reasoning_engine" => self.create_reasoning_engine(input).await,
            "feature_view_sync" => self.create_feature_view_sync(input).await,
            "execution" => self.create_execution(input).await,
            "memorie" => self.create_memorie(input).await,
            "annotation_spec" => self.create_annotation_spec(input).await,
            "feature_online_store" => self.create_feature_online_store(input).await,
            "migratable_resource" => self.create_migratable_resource(input).await,
            "feature_monitor" => self.create_feature_monitor(input).await,
            "event" => self.create_event(input).await,
            "nas_job" => self.create_nas_job(input).await,
            "context" => self.create_context(input).await,
            "media" => self.create_media(input).await,
            "experiment" => self.create_experiment(input).await,
            "example_store" => self.create_example_store(input).await,
            "evaluation" => self.create_evaluation(input).await,
            "dataset_version" => self.create_dataset_version(input).await,
            "model" => self.create_model(input).await,
            "feature_group" => self.create_feature_group(input).await,
            "operation" => self.create_operation(input).await,
            "studie" => self.create_studie(input).await,
            "metadata_schema" => self.create_metadata_schema(input).await,
            "model_monitor" => self.create_model_monitor(input).await,
            "deployment_resource_pool" => self.create_deployment_resource_pool(input).await,
            "model_monitoring_job" => self.create_model_monitoring_job(input).await,
            "model_deployment_monitoring_job" => {
                self.create_model_deployment_monitoring_job(input).await
            }
            "endpoint" => self.create_endpoint(input).await,
            "schedule" => self.create_schedule(input).await,
            "model_garden_eula" => self.create_model_garden_eula(input).await,
            "feature_monitor_job" => self.create_feature_monitor_job(input).await,
            "notebook_execution_job" => self.create_notebook_execution_job(input).await,
            "tensorboard" => self.create_tensorboard(input).await,
            "rag_file" => self.create_rag_file(input).await,
            "cached_content" => self.create_cached_content(input).await,
            "evaluation_item" => self.create_evaluation_item(input).await,
            "slice" => self.create_slice(input).await,
            "extension" => self.create_extension(input).await,
            "custom_job" => self.create_custom_job(input).await,
            "data_item" => self.create_data_item(input).await,
            "evaluation_run" => self.create_evaluation_run(input).await,
            "time_serie" => self.create_time_serie(input).await,
            "sandbox_environment" => self.create_sandbox_environment(input).await,
            "project" => self.create_project(input).await,
            "evaluation_set" => self.create_evaluation_set(input).await,
            "entity_type" => self.create_entity_type(input).await,
            "data_labeling_job" => self.create_data_labeling_job(input).await,
            "pipeline_job" => self.create_pipeline_job(input).await,
            "training_pipeline" => self.create_training_pipeline(input).await,
            "nas_trial_detail" => self.create_nas_trial_detail(input).await,
            "metadata_store" => self.create_metadata_store(input).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "aiplatform_api", resource_name
            ))),
        }
    }

    /// Read resource state
    pub async fn read_resource(&self, resource_name: &str, id: &str) -> Result<ResourceOutput> {
        match resource_name {
            "location" => self.read_location(id).await,
            "evaluation_item" => self.read_evaluation_item(id).await,
            "entity_type" => self.read_entity_type(id).await,
            "metadata_schema" => self.read_metadata_schema(id).await,
            "indexe" => self.read_indexe(id).await,
            "model" => self.read_model(id).await,
            "migratable_resource" => self.read_migratable_resource(id).await,
            "rag_file" => self.read_rag_file(id).await,
            "notebook_runtime" => self.read_notebook_runtime(id).await,
            "feature_group" => self.read_feature_group(id).await,
            "data_item" => self.read_data_item(id).await,
            "feature_online_store" => self.read_feature_online_store(id).await,
            "experiment" => self.read_experiment(id).await,
            "context" => self.read_context(id).await,
            "media" => self.read_media(id).await,
            "invoke" => self.read_invoke(id).await,
            "annotation_spec" => self.read_annotation_spec(id).await,
            "endpoint" => self.read_endpoint(id).await,
            "model_deployment_monitoring_job" => {
                self.read_model_deployment_monitoring_job(id).await
            }
            "featurestore" => self.read_featurestore(id).await,
            "dataset" => self.read_dataset(id).await,
            "training_pipeline" => self.read_training_pipeline(id).await,
            "feature" => self.read_feature(id).await,
            "dataset_version" => self.read_dataset_version(id).await,
            "tensorboard" => self.read_tensorboard(id).await,
            "index_endpoint" => self.read_index_endpoint(id).await,
            "artifact" => self.read_artifact(id).await,
            "custom_job" => self.read_custom_job(id).await,
            "execution" => self.read_execution(id).await,
            "reasoning_engine" => self.read_reasoning_engine(id).await,
            "evaluation_run" => self.read_evaluation_run(id).await,
            "pipeline_job" => self.read_pipeline_job(id).await,
            "evaluation_set" => self.read_evaluation_set(id).await,
            "nas_trial_detail" => self.read_nas_trial_detail(id).await,
            "tuning_job" => self.read_tuning_job(id).await,
            "run" => self.read_run(id).await,
            "metadata_store" => self.read_metadata_store(id).await,
            "slice" => self.read_slice(id).await,
            "time_serie" => self.read_time_serie(id).await,
            "persistent_resource" => self.read_persistent_resource(id).await,
            "annotation" => self.read_annotation(id).await,
            "hyperparameter_tuning_job" => self.read_hyperparameter_tuning_job(id).await,
            "notebook_runtime_template" => self.read_notebook_runtime_template(id).await,
            "project" => self.read_project(id).await,
            "notebook_execution_job" => self.read_notebook_execution_job(id).await,
            "chat" => self.read_chat(id).await,
            "cached_content" => self.read_cached_content(id).await,
            "nas_job" => self.read_nas_job(id).await,
            "operation" => self.read_operation(id).await,
            "rag_corpora" => self.read_rag_corpora(id).await,
            "data_labeling_job" => self.read_data_labeling_job(id).await,
            "deployment_resource_pool" => self.read_deployment_resource_pool(id).await,
            "feature_view" => self.read_feature_view(id).await,
            "evaluation" => self.read_evaluation(id).await,
            "schedule" => self.read_schedule(id).await,
            "batch_prediction_job" => self.read_batch_prediction_job(id).await,
            "feature_view_sync" => self.read_feature_view_sync(id).await,
            "studie" => self.read_studie(id).await,
            "trial" => self.read_trial(id).await,
            "specialist_pool" => self.read_specialist_pool(id).await,
            "openapi" => self.read_openapi(id).await,
            "saved_querie" => self.read_saved_querie(id).await,
            "feature_view" => self.read_feature_view(id).await,
            "artifact" => self.read_artifact(id).await,
            "notebook_runtime" => self.read_notebook_runtime(id).await,
            "annotation" => self.read_annotation(id).await,
            "notebook_runtime_template" => self.read_notebook_runtime_template(id).await,
            "saved_querie" => self.read_saved_querie(id).await,
            "tuning_job" => self.read_tuning_job(id).await,
            "indexe" => self.read_indexe(id).await,
            "index_endpoint" => self.read_index_endpoint(id).await,
            "batch_prediction_job" => self.read_batch_prediction_job(id).await,
            "location" => self.read_location(id).await,
            "featurestore" => self.read_featurestore(id).await,
            "chat" => self.read_chat(id).await,
            "run" => self.read_run(id).await,
            "rag_corpora" => self.read_rag_corpora(id).await,
            "persistent_resource" => self.read_persistent_resource(id).await,
            "feature" => self.read_feature(id).await,
            "hyperparameter_tuning_job" => self.read_hyperparameter_tuning_job(id).await,
            "specialist_pool" => self.read_specialist_pool(id).await,
            "trial" => self.read_trial(id).await,
            "session" => self.read_session(id).await,
            "dataset" => self.read_dataset(id).await,
            "reasoning_engine" => self.read_reasoning_engine(id).await,
            "feature_view_sync" => self.read_feature_view_sync(id).await,
            "execution" => self.read_execution(id).await,
            "memorie" => self.read_memorie(id).await,
            "annotation_spec" => self.read_annotation_spec(id).await,
            "feature_online_store" => self.read_feature_online_store(id).await,
            "migratable_resource" => self.read_migratable_resource(id).await,
            "feature_monitor" => self.read_feature_monitor(id).await,
            "event" => self.read_event(id).await,
            "nas_job" => self.read_nas_job(id).await,
            "context" => self.read_context(id).await,
            "media" => self.read_media(id).await,
            "experiment" => self.read_experiment(id).await,
            "example_store" => self.read_example_store(id).await,
            "evaluation" => self.read_evaluation(id).await,
            "dataset_version" => self.read_dataset_version(id).await,
            "model" => self.read_model(id).await,
            "feature_group" => self.read_feature_group(id).await,
            "operation" => self.read_operation(id).await,
            "studie" => self.read_studie(id).await,
            "metadata_schema" => self.read_metadata_schema(id).await,
            "model_monitor" => self.read_model_monitor(id).await,
            "deployment_resource_pool" => self.read_deployment_resource_pool(id).await,
            "model_monitoring_job" => self.read_model_monitoring_job(id).await,
            "model_deployment_monitoring_job" => {
                self.read_model_deployment_monitoring_job(id).await
            }
            "endpoint" => self.read_endpoint(id).await,
            "schedule" => self.read_schedule(id).await,
            "model_garden_eula" => self.read_model_garden_eula(id).await,
            "feature_monitor_job" => self.read_feature_monitor_job(id).await,
            "notebook_execution_job" => self.read_notebook_execution_job(id).await,
            "tensorboard" => self.read_tensorboard(id).await,
            "rag_file" => self.read_rag_file(id).await,
            "cached_content" => self.read_cached_content(id).await,
            "evaluation_item" => self.read_evaluation_item(id).await,
            "slice" => self.read_slice(id).await,
            "extension" => self.read_extension(id).await,
            "custom_job" => self.read_custom_job(id).await,
            "data_item" => self.read_data_item(id).await,
            "evaluation_run" => self.read_evaluation_run(id).await,
            "time_serie" => self.read_time_serie(id).await,
            "sandbox_environment" => self.read_sandbox_environment(id).await,
            "project" => self.read_project(id).await,
            "evaluation_set" => self.read_evaluation_set(id).await,
            "entity_type" => self.read_entity_type(id).await,
            "data_labeling_job" => self.read_data_labeling_job(id).await,
            "pipeline_job" => self.read_pipeline_job(id).await,
            "training_pipeline" => self.read_training_pipeline(id).await,
            "nas_trial_detail" => self.read_nas_trial_detail(id).await,
            "metadata_store" => self.read_metadata_store(id).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "aiplatform_api", resource_name
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
            "location" => self.update_location(id, input).await,
            "evaluation_item" => self.update_evaluation_item(id, input).await,
            "entity_type" => self.update_entity_type(id, input).await,
            "metadata_schema" => self.update_metadata_schema(id, input).await,
            "indexe" => self.update_indexe(id, input).await,
            "model" => self.update_model(id, input).await,
            "migratable_resource" => self.update_migratable_resource(id, input).await,
            "rag_file" => self.update_rag_file(id, input).await,
            "notebook_runtime" => self.update_notebook_runtime(id, input).await,
            "feature_group" => self.update_feature_group(id, input).await,
            "data_item" => self.update_data_item(id, input).await,
            "feature_online_store" => self.update_feature_online_store(id, input).await,
            "experiment" => self.update_experiment(id, input).await,
            "context" => self.update_context(id, input).await,
            "media" => self.update_media(id, input).await,
            "invoke" => self.update_invoke(id, input).await,
            "annotation_spec" => self.update_annotation_spec(id, input).await,
            "endpoint" => self.update_endpoint(id, input).await,
            "model_deployment_monitoring_job" => {
                self.update_model_deployment_monitoring_job(id, input).await
            }
            "featurestore" => self.update_featurestore(id, input).await,
            "dataset" => self.update_dataset(id, input).await,
            "training_pipeline" => self.update_training_pipeline(id, input).await,
            "feature" => self.update_feature(id, input).await,
            "dataset_version" => self.update_dataset_version(id, input).await,
            "tensorboard" => self.update_tensorboard(id, input).await,
            "index_endpoint" => self.update_index_endpoint(id, input).await,
            "artifact" => self.update_artifact(id, input).await,
            "custom_job" => self.update_custom_job(id, input).await,
            "execution" => self.update_execution(id, input).await,
            "reasoning_engine" => self.update_reasoning_engine(id, input).await,
            "evaluation_run" => self.update_evaluation_run(id, input).await,
            "pipeline_job" => self.update_pipeline_job(id, input).await,
            "evaluation_set" => self.update_evaluation_set(id, input).await,
            "nas_trial_detail" => self.update_nas_trial_detail(id, input).await,
            "tuning_job" => self.update_tuning_job(id, input).await,
            "run" => self.update_run(id, input).await,
            "metadata_store" => self.update_metadata_store(id, input).await,
            "slice" => self.update_slice(id, input).await,
            "time_serie" => self.update_time_serie(id, input).await,
            "persistent_resource" => self.update_persistent_resource(id, input).await,
            "annotation" => self.update_annotation(id, input).await,
            "hyperparameter_tuning_job" => self.update_hyperparameter_tuning_job(id, input).await,
            "notebook_runtime_template" => self.update_notebook_runtime_template(id, input).await,
            "project" => self.update_project(id, input).await,
            "notebook_execution_job" => self.update_notebook_execution_job(id, input).await,
            "chat" => self.update_chat(id, input).await,
            "cached_content" => self.update_cached_content(id, input).await,
            "nas_job" => self.update_nas_job(id, input).await,
            "operation" => self.update_operation(id, input).await,
            "rag_corpora" => self.update_rag_corpora(id, input).await,
            "data_labeling_job" => self.update_data_labeling_job(id, input).await,
            "deployment_resource_pool" => self.update_deployment_resource_pool(id, input).await,
            "feature_view" => self.update_feature_view(id, input).await,
            "evaluation" => self.update_evaluation(id, input).await,
            "schedule" => self.update_schedule(id, input).await,
            "batch_prediction_job" => self.update_batch_prediction_job(id, input).await,
            "feature_view_sync" => self.update_feature_view_sync(id, input).await,
            "studie" => self.update_studie(id, input).await,
            "trial" => self.update_trial(id, input).await,
            "specialist_pool" => self.update_specialist_pool(id, input).await,
            "openapi" => self.update_openapi(id, input).await,
            "saved_querie" => self.update_saved_querie(id, input).await,
            "feature_view" => self.update_feature_view(id, input).await,
            "artifact" => self.update_artifact(id, input).await,
            "notebook_runtime" => self.update_notebook_runtime(id, input).await,
            "annotation" => self.update_annotation(id, input).await,
            "notebook_runtime_template" => self.update_notebook_runtime_template(id, input).await,
            "saved_querie" => self.update_saved_querie(id, input).await,
            "tuning_job" => self.update_tuning_job(id, input).await,
            "indexe" => self.update_indexe(id, input).await,
            "index_endpoint" => self.update_index_endpoint(id, input).await,
            "batch_prediction_job" => self.update_batch_prediction_job(id, input).await,
            "location" => self.update_location(id, input).await,
            "featurestore" => self.update_featurestore(id, input).await,
            "chat" => self.update_chat(id, input).await,
            "run" => self.update_run(id, input).await,
            "rag_corpora" => self.update_rag_corpora(id, input).await,
            "persistent_resource" => self.update_persistent_resource(id, input).await,
            "feature" => self.update_feature(id, input).await,
            "hyperparameter_tuning_job" => self.update_hyperparameter_tuning_job(id, input).await,
            "specialist_pool" => self.update_specialist_pool(id, input).await,
            "trial" => self.update_trial(id, input).await,
            "session" => self.update_session(id, input).await,
            "dataset" => self.update_dataset(id, input).await,
            "reasoning_engine" => self.update_reasoning_engine(id, input).await,
            "feature_view_sync" => self.update_feature_view_sync(id, input).await,
            "execution" => self.update_execution(id, input).await,
            "memorie" => self.update_memorie(id, input).await,
            "annotation_spec" => self.update_annotation_spec(id, input).await,
            "feature_online_store" => self.update_feature_online_store(id, input).await,
            "migratable_resource" => self.update_migratable_resource(id, input).await,
            "feature_monitor" => self.update_feature_monitor(id, input).await,
            "event" => self.update_event(id, input).await,
            "nas_job" => self.update_nas_job(id, input).await,
            "context" => self.update_context(id, input).await,
            "media" => self.update_media(id, input).await,
            "experiment" => self.update_experiment(id, input).await,
            "example_store" => self.update_example_store(id, input).await,
            "evaluation" => self.update_evaluation(id, input).await,
            "dataset_version" => self.update_dataset_version(id, input).await,
            "model" => self.update_model(id, input).await,
            "feature_group" => self.update_feature_group(id, input).await,
            "operation" => self.update_operation(id, input).await,
            "studie" => self.update_studie(id, input).await,
            "metadata_schema" => self.update_metadata_schema(id, input).await,
            "model_monitor" => self.update_model_monitor(id, input).await,
            "deployment_resource_pool" => self.update_deployment_resource_pool(id, input).await,
            "model_monitoring_job" => self.update_model_monitoring_job(id, input).await,
            "model_deployment_monitoring_job" => {
                self.update_model_deployment_monitoring_job(id, input).await
            }
            "endpoint" => self.update_endpoint(id, input).await,
            "schedule" => self.update_schedule(id, input).await,
            "model_garden_eula" => self.update_model_garden_eula(id, input).await,
            "feature_monitor_job" => self.update_feature_monitor_job(id, input).await,
            "notebook_execution_job" => self.update_notebook_execution_job(id, input).await,
            "tensorboard" => self.update_tensorboard(id, input).await,
            "rag_file" => self.update_rag_file(id, input).await,
            "cached_content" => self.update_cached_content(id, input).await,
            "evaluation_item" => self.update_evaluation_item(id, input).await,
            "slice" => self.update_slice(id, input).await,
            "extension" => self.update_extension(id, input).await,
            "custom_job" => self.update_custom_job(id, input).await,
            "data_item" => self.update_data_item(id, input).await,
            "evaluation_run" => self.update_evaluation_run(id, input).await,
            "time_serie" => self.update_time_serie(id, input).await,
            "sandbox_environment" => self.update_sandbox_environment(id, input).await,
            "project" => self.update_project(id, input).await,
            "evaluation_set" => self.update_evaluation_set(id, input).await,
            "entity_type" => self.update_entity_type(id, input).await,
            "data_labeling_job" => self.update_data_labeling_job(id, input).await,
            "pipeline_job" => self.update_pipeline_job(id, input).await,
            "training_pipeline" => self.update_training_pipeline(id, input).await,
            "nas_trial_detail" => self.update_nas_trial_detail(id, input).await,
            "metadata_store" => self.update_metadata_store(id, input).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "aiplatform_api", resource_name
            ))),
        }
    }

    /// Delete a resource
    pub async fn delete_resource(&self, resource_name: &str, id: &str) -> Result<()> {
        match resource_name {
            "location" => self.delete_location(id).await,
            "evaluation_item" => self.delete_evaluation_item(id).await,
            "entity_type" => self.delete_entity_type(id).await,
            "metadata_schema" => self.delete_metadata_schema(id).await,
            "indexe" => self.delete_indexe(id).await,
            "model" => self.delete_model(id).await,
            "migratable_resource" => self.delete_migratable_resource(id).await,
            "rag_file" => self.delete_rag_file(id).await,
            "notebook_runtime" => self.delete_notebook_runtime(id).await,
            "feature_group" => self.delete_feature_group(id).await,
            "data_item" => self.delete_data_item(id).await,
            "feature_online_store" => self.delete_feature_online_store(id).await,
            "experiment" => self.delete_experiment(id).await,
            "context" => self.delete_context(id).await,
            "media" => self.delete_media(id).await,
            "invoke" => self.delete_invoke(id).await,
            "annotation_spec" => self.delete_annotation_spec(id).await,
            "endpoint" => self.delete_endpoint(id).await,
            "model_deployment_monitoring_job" => {
                self.delete_model_deployment_monitoring_job(id).await
            }
            "featurestore" => self.delete_featurestore(id).await,
            "dataset" => self.delete_dataset(id).await,
            "training_pipeline" => self.delete_training_pipeline(id).await,
            "feature" => self.delete_feature(id).await,
            "dataset_version" => self.delete_dataset_version(id).await,
            "tensorboard" => self.delete_tensorboard(id).await,
            "index_endpoint" => self.delete_index_endpoint(id).await,
            "artifact" => self.delete_artifact(id).await,
            "custom_job" => self.delete_custom_job(id).await,
            "execution" => self.delete_execution(id).await,
            "reasoning_engine" => self.delete_reasoning_engine(id).await,
            "evaluation_run" => self.delete_evaluation_run(id).await,
            "pipeline_job" => self.delete_pipeline_job(id).await,
            "evaluation_set" => self.delete_evaluation_set(id).await,
            "nas_trial_detail" => self.delete_nas_trial_detail(id).await,
            "tuning_job" => self.delete_tuning_job(id).await,
            "run" => self.delete_run(id).await,
            "metadata_store" => self.delete_metadata_store(id).await,
            "slice" => self.delete_slice(id).await,
            "time_serie" => self.delete_time_serie(id).await,
            "persistent_resource" => self.delete_persistent_resource(id).await,
            "annotation" => self.delete_annotation(id).await,
            "hyperparameter_tuning_job" => self.delete_hyperparameter_tuning_job(id).await,
            "notebook_runtime_template" => self.delete_notebook_runtime_template(id).await,
            "project" => self.delete_project(id).await,
            "notebook_execution_job" => self.delete_notebook_execution_job(id).await,
            "chat" => self.delete_chat(id).await,
            "cached_content" => self.delete_cached_content(id).await,
            "nas_job" => self.delete_nas_job(id).await,
            "operation" => self.delete_operation(id).await,
            "rag_corpora" => self.delete_rag_corpora(id).await,
            "data_labeling_job" => self.delete_data_labeling_job(id).await,
            "deployment_resource_pool" => self.delete_deployment_resource_pool(id).await,
            "feature_view" => self.delete_feature_view(id).await,
            "evaluation" => self.delete_evaluation(id).await,
            "schedule" => self.delete_schedule(id).await,
            "batch_prediction_job" => self.delete_batch_prediction_job(id).await,
            "feature_view_sync" => self.delete_feature_view_sync(id).await,
            "studie" => self.delete_studie(id).await,
            "trial" => self.delete_trial(id).await,
            "specialist_pool" => self.delete_specialist_pool(id).await,
            "openapi" => self.delete_openapi(id).await,
            "saved_querie" => self.delete_saved_querie(id).await,
            "feature_view" => self.delete_feature_view(id).await,
            "artifact" => self.delete_artifact(id).await,
            "notebook_runtime" => self.delete_notebook_runtime(id).await,
            "annotation" => self.delete_annotation(id).await,
            "notebook_runtime_template" => self.delete_notebook_runtime_template(id).await,
            "saved_querie" => self.delete_saved_querie(id).await,
            "tuning_job" => self.delete_tuning_job(id).await,
            "indexe" => self.delete_indexe(id).await,
            "index_endpoint" => self.delete_index_endpoint(id).await,
            "batch_prediction_job" => self.delete_batch_prediction_job(id).await,
            "location" => self.delete_location(id).await,
            "featurestore" => self.delete_featurestore(id).await,
            "chat" => self.delete_chat(id).await,
            "run" => self.delete_run(id).await,
            "rag_corpora" => self.delete_rag_corpora(id).await,
            "persistent_resource" => self.delete_persistent_resource(id).await,
            "feature" => self.delete_feature(id).await,
            "hyperparameter_tuning_job" => self.delete_hyperparameter_tuning_job(id).await,
            "specialist_pool" => self.delete_specialist_pool(id).await,
            "trial" => self.delete_trial(id).await,
            "session" => self.delete_session(id).await,
            "dataset" => self.delete_dataset(id).await,
            "reasoning_engine" => self.delete_reasoning_engine(id).await,
            "feature_view_sync" => self.delete_feature_view_sync(id).await,
            "execution" => self.delete_execution(id).await,
            "memorie" => self.delete_memorie(id).await,
            "annotation_spec" => self.delete_annotation_spec(id).await,
            "feature_online_store" => self.delete_feature_online_store(id).await,
            "migratable_resource" => self.delete_migratable_resource(id).await,
            "feature_monitor" => self.delete_feature_monitor(id).await,
            "event" => self.delete_event(id).await,
            "nas_job" => self.delete_nas_job(id).await,
            "context" => self.delete_context(id).await,
            "media" => self.delete_media(id).await,
            "experiment" => self.delete_experiment(id).await,
            "example_store" => self.delete_example_store(id).await,
            "evaluation" => self.delete_evaluation(id).await,
            "dataset_version" => self.delete_dataset_version(id).await,
            "model" => self.delete_model(id).await,
            "feature_group" => self.delete_feature_group(id).await,
            "operation" => self.delete_operation(id).await,
            "studie" => self.delete_studie(id).await,
            "metadata_schema" => self.delete_metadata_schema(id).await,
            "model_monitor" => self.delete_model_monitor(id).await,
            "deployment_resource_pool" => self.delete_deployment_resource_pool(id).await,
            "model_monitoring_job" => self.delete_model_monitoring_job(id).await,
            "model_deployment_monitoring_job" => {
                self.delete_model_deployment_monitoring_job(id).await
            }
            "endpoint" => self.delete_endpoint(id).await,
            "schedule" => self.delete_schedule(id).await,
            "model_garden_eula" => self.delete_model_garden_eula(id).await,
            "feature_monitor_job" => self.delete_feature_monitor_job(id).await,
            "notebook_execution_job" => self.delete_notebook_execution_job(id).await,
            "tensorboard" => self.delete_tensorboard(id).await,
            "rag_file" => self.delete_rag_file(id).await,
            "cached_content" => self.delete_cached_content(id).await,
            "evaluation_item" => self.delete_evaluation_item(id).await,
            "slice" => self.delete_slice(id).await,
            "extension" => self.delete_extension(id).await,
            "custom_job" => self.delete_custom_job(id).await,
            "data_item" => self.delete_data_item(id).await,
            "evaluation_run" => self.delete_evaluation_run(id).await,
            "time_serie" => self.delete_time_serie(id).await,
            "sandbox_environment" => self.delete_sandbox_environment(id).await,
            "project" => self.delete_project(id).await,
            "evaluation_set" => self.delete_evaluation_set(id).await,
            "entity_type" => self.delete_entity_type(id).await,
            "data_labeling_job" => self.delete_data_labeling_job(id).await,
            "pipeline_job" => self.delete_pipeline_job(id).await,
            "training_pipeline" => self.delete_training_pipeline(id).await,
            "nas_trial_detail" => self.delete_nas_trial_detail(id).await,
            "metadata_store" => self.delete_metadata_store(id).await,
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "aiplatform_api", resource_name
            ))),
        }
    }

    // ========================================================================
    // Resource-specific CRUD implementations
    // ========================================================================

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
    // Evaluation_item resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a evaluation_item resource
    async fn plan_evaluation_item(
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

    /// Create a new evaluation_item resource
    async fn create_evaluation_item(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a evaluation_item resource
    async fn read_evaluation_item(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a evaluation_item resource
    async fn update_evaluation_item(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a evaluation_item resource
    async fn delete_evaluation_item(&self, id: &str) -> Result<()> {
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
    // Metadata_schema resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a metadata_schema resource
    async fn plan_metadata_schema(
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

    /// Create a new metadata_schema resource
    async fn create_metadata_schema(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a metadata_schema resource
    async fn read_metadata_schema(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a metadata_schema resource
    async fn update_metadata_schema(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a metadata_schema resource
    async fn delete_metadata_schema(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Indexe resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a indexe resource
    async fn plan_indexe(
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

    /// Create a new indexe resource
    async fn create_indexe(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a indexe resource
    async fn read_indexe(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a indexe resource
    async fn update_indexe(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a indexe resource
    async fn delete_indexe(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Model resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a model resource
    async fn plan_model(
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

    /// Create a new model resource
    async fn create_model(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a model resource
    async fn read_model(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a model resource
    async fn update_model(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a model resource
    async fn delete_model(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Migratable_resource resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a migratable_resource resource
    async fn plan_migratable_resource(
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

    /// Create a new migratable_resource resource
    async fn create_migratable_resource(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a migratable_resource resource
    async fn read_migratable_resource(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a migratable_resource resource
    async fn update_migratable_resource(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a migratable_resource resource
    async fn delete_migratable_resource(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Rag_file resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a rag_file resource
    async fn plan_rag_file(
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

    /// Create a new rag_file resource
    async fn create_rag_file(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a rag_file resource
    async fn read_rag_file(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a rag_file resource
    async fn update_rag_file(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a rag_file resource
    async fn delete_rag_file(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Notebook_runtime resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a notebook_runtime resource
    async fn plan_notebook_runtime(
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

    /// Create a new notebook_runtime resource
    async fn create_notebook_runtime(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a notebook_runtime resource
    async fn read_notebook_runtime(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a notebook_runtime resource
    async fn update_notebook_runtime(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a notebook_runtime resource
    async fn delete_notebook_runtime(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Feature_group resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a feature_group resource
    async fn plan_feature_group(
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

    /// Create a new feature_group resource
    async fn create_feature_group(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a feature_group resource
    async fn read_feature_group(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a feature_group resource
    async fn update_feature_group(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a feature_group resource
    async fn delete_feature_group(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Data_item resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a data_item resource
    async fn plan_data_item(
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

    /// Create a new data_item resource
    async fn create_data_item(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a data_item resource
    async fn read_data_item(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a data_item resource
    async fn update_data_item(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a data_item resource
    async fn delete_data_item(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Feature_online_store resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a feature_online_store resource
    async fn plan_feature_online_store(
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

    /// Create a new feature_online_store resource
    async fn create_feature_online_store(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a feature_online_store resource
    async fn read_feature_online_store(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a feature_online_store resource
    async fn update_feature_online_store(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a feature_online_store resource
    async fn delete_feature_online_store(&self, id: &str) -> Result<()> {
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
    // Invoke resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a invoke resource
    async fn plan_invoke(
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

    /// Create a new invoke resource
    async fn create_invoke(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a invoke resource
    async fn read_invoke(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a invoke resource
    async fn update_invoke(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a invoke resource
    async fn delete_invoke(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Annotation_spec resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a annotation_spec resource
    async fn plan_annotation_spec(
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

    /// Create a new annotation_spec resource
    async fn create_annotation_spec(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a annotation_spec resource
    async fn read_annotation_spec(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a annotation_spec resource
    async fn update_annotation_spec(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a annotation_spec resource
    async fn delete_annotation_spec(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Endpoint resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a endpoint resource
    async fn plan_endpoint(
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

    /// Create a new endpoint resource
    async fn create_endpoint(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a endpoint resource
    async fn read_endpoint(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a endpoint resource
    async fn update_endpoint(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a endpoint resource
    async fn delete_endpoint(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Model_deployment_monitoring_job resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a model_deployment_monitoring_job resource
    async fn plan_model_deployment_monitoring_job(
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

    /// Create a new model_deployment_monitoring_job resource
    async fn create_model_deployment_monitoring_job(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a model_deployment_monitoring_job resource
    async fn read_model_deployment_monitoring_job(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a model_deployment_monitoring_job resource
    async fn update_model_deployment_monitoring_job(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a model_deployment_monitoring_job resource
    async fn delete_model_deployment_monitoring_job(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Featurestore resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a featurestore resource
    async fn plan_featurestore(
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

    /// Create a new featurestore resource
    async fn create_featurestore(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a featurestore resource
    async fn read_featurestore(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a featurestore resource
    async fn update_featurestore(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a featurestore resource
    async fn delete_featurestore(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Dataset resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a dataset resource
    async fn plan_dataset(
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

    /// Create a new dataset resource
    async fn create_dataset(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a dataset resource
    async fn read_dataset(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a dataset resource
    async fn update_dataset(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a dataset resource
    async fn delete_dataset(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Training_pipeline resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a training_pipeline resource
    async fn plan_training_pipeline(
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

    /// Create a new training_pipeline resource
    async fn create_training_pipeline(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a training_pipeline resource
    async fn read_training_pipeline(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a training_pipeline resource
    async fn update_training_pipeline(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a training_pipeline resource
    async fn delete_training_pipeline(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Feature resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a feature resource
    async fn plan_feature(
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

    /// Create a new feature resource
    async fn create_feature(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a feature resource
    async fn read_feature(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a feature resource
    async fn update_feature(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a feature resource
    async fn delete_feature(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Dataset_version resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a dataset_version resource
    async fn plan_dataset_version(
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

    /// Create a new dataset_version resource
    async fn create_dataset_version(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a dataset_version resource
    async fn read_dataset_version(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a dataset_version resource
    async fn update_dataset_version(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a dataset_version resource
    async fn delete_dataset_version(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Tensorboard resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a tensorboard resource
    async fn plan_tensorboard(
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

    /// Create a new tensorboard resource
    async fn create_tensorboard(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a tensorboard resource
    async fn read_tensorboard(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a tensorboard resource
    async fn update_tensorboard(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a tensorboard resource
    async fn delete_tensorboard(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Index_endpoint resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a index_endpoint resource
    async fn plan_index_endpoint(
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

    /// Create a new index_endpoint resource
    async fn create_index_endpoint(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a index_endpoint resource
    async fn read_index_endpoint(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a index_endpoint resource
    async fn update_index_endpoint(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a index_endpoint resource
    async fn delete_index_endpoint(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Artifact resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a artifact resource
    async fn plan_artifact(
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

    /// Create a new artifact resource
    async fn create_artifact(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a artifact resource
    async fn read_artifact(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a artifact resource
    async fn update_artifact(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a artifact resource
    async fn delete_artifact(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Custom_job resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a custom_job resource
    async fn plan_custom_job(
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

    /// Create a new custom_job resource
    async fn create_custom_job(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a custom_job resource
    async fn read_custom_job(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a custom_job resource
    async fn update_custom_job(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a custom_job resource
    async fn delete_custom_job(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Execution resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a execution resource
    async fn plan_execution(
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

    /// Create a new execution resource
    async fn create_execution(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a execution resource
    async fn read_execution(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a execution resource
    async fn update_execution(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a execution resource
    async fn delete_execution(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Reasoning_engine resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a reasoning_engine resource
    async fn plan_reasoning_engine(
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

    /// Create a new reasoning_engine resource
    async fn create_reasoning_engine(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a reasoning_engine resource
    async fn read_reasoning_engine(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a reasoning_engine resource
    async fn update_reasoning_engine(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a reasoning_engine resource
    async fn delete_reasoning_engine(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Evaluation_run resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a evaluation_run resource
    async fn plan_evaluation_run(
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

    /// Create a new evaluation_run resource
    async fn create_evaluation_run(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a evaluation_run resource
    async fn read_evaluation_run(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a evaluation_run resource
    async fn update_evaluation_run(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a evaluation_run resource
    async fn delete_evaluation_run(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Pipeline_job resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a pipeline_job resource
    async fn plan_pipeline_job(
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

    /// Create a new pipeline_job resource
    async fn create_pipeline_job(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a pipeline_job resource
    async fn read_pipeline_job(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a pipeline_job resource
    async fn update_pipeline_job(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a pipeline_job resource
    async fn delete_pipeline_job(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Evaluation_set resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a evaluation_set resource
    async fn plan_evaluation_set(
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

    /// Create a new evaluation_set resource
    async fn create_evaluation_set(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a evaluation_set resource
    async fn read_evaluation_set(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a evaluation_set resource
    async fn update_evaluation_set(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a evaluation_set resource
    async fn delete_evaluation_set(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Nas_trial_detail resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a nas_trial_detail resource
    async fn plan_nas_trial_detail(
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

    /// Create a new nas_trial_detail resource
    async fn create_nas_trial_detail(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a nas_trial_detail resource
    async fn read_nas_trial_detail(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a nas_trial_detail resource
    async fn update_nas_trial_detail(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a nas_trial_detail resource
    async fn delete_nas_trial_detail(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Tuning_job resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a tuning_job resource
    async fn plan_tuning_job(
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

    /// Create a new tuning_job resource
    async fn create_tuning_job(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a tuning_job resource
    async fn read_tuning_job(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a tuning_job resource
    async fn update_tuning_job(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a tuning_job resource
    async fn delete_tuning_job(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Run resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a run resource
    async fn plan_run(
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

    /// Create a new run resource
    async fn create_run(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a run resource
    async fn read_run(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a run resource
    async fn update_run(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a run resource
    async fn delete_run(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Metadata_store resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a metadata_store resource
    async fn plan_metadata_store(
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

    /// Create a new metadata_store resource
    async fn create_metadata_store(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a metadata_store resource
    async fn read_metadata_store(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a metadata_store resource
    async fn update_metadata_store(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a metadata_store resource
    async fn delete_metadata_store(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Slice resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a slice resource
    async fn plan_slice(
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

    /// Create a new slice resource
    async fn create_slice(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a slice resource
    async fn read_slice(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a slice resource
    async fn update_slice(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a slice resource
    async fn delete_slice(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Time_serie resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a time_serie resource
    async fn plan_time_serie(
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

    /// Create a new time_serie resource
    async fn create_time_serie(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a time_serie resource
    async fn read_time_serie(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a time_serie resource
    async fn update_time_serie(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a time_serie resource
    async fn delete_time_serie(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Persistent_resource resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a persistent_resource resource
    async fn plan_persistent_resource(
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

    /// Create a new persistent_resource resource
    async fn create_persistent_resource(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a persistent_resource resource
    async fn read_persistent_resource(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a persistent_resource resource
    async fn update_persistent_resource(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a persistent_resource resource
    async fn delete_persistent_resource(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Annotation resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a annotation resource
    async fn plan_annotation(
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

    /// Create a new annotation resource
    async fn create_annotation(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a annotation resource
    async fn read_annotation(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a annotation resource
    async fn update_annotation(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a annotation resource
    async fn delete_annotation(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Hyperparameter_tuning_job resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a hyperparameter_tuning_job resource
    async fn plan_hyperparameter_tuning_job(
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

    /// Create a new hyperparameter_tuning_job resource
    async fn create_hyperparameter_tuning_job(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a hyperparameter_tuning_job resource
    async fn read_hyperparameter_tuning_job(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a hyperparameter_tuning_job resource
    async fn update_hyperparameter_tuning_job(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a hyperparameter_tuning_job resource
    async fn delete_hyperparameter_tuning_job(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Notebook_runtime_template resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a notebook_runtime_template resource
    async fn plan_notebook_runtime_template(
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

    /// Create a new notebook_runtime_template resource
    async fn create_notebook_runtime_template(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a notebook_runtime_template resource
    async fn read_notebook_runtime_template(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a notebook_runtime_template resource
    async fn update_notebook_runtime_template(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a notebook_runtime_template resource
    async fn delete_notebook_runtime_template(&self, id: &str) -> Result<()> {
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
    // Notebook_execution_job resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a notebook_execution_job resource
    async fn plan_notebook_execution_job(
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

    /// Create a new notebook_execution_job resource
    async fn create_notebook_execution_job(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a notebook_execution_job resource
    async fn read_notebook_execution_job(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a notebook_execution_job resource
    async fn update_notebook_execution_job(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a notebook_execution_job resource
    async fn delete_notebook_execution_job(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Chat resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a chat resource
    async fn plan_chat(
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

    /// Create a new chat resource
    async fn create_chat(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a chat resource
    async fn read_chat(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a chat resource
    async fn update_chat(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a chat resource
    async fn delete_chat(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Cached_content resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a cached_content resource
    async fn plan_cached_content(
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

    /// Create a new cached_content resource
    async fn create_cached_content(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a cached_content resource
    async fn read_cached_content(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a cached_content resource
    async fn update_cached_content(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a cached_content resource
    async fn delete_cached_content(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Nas_job resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a nas_job resource
    async fn plan_nas_job(
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

    /// Create a new nas_job resource
    async fn create_nas_job(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a nas_job resource
    async fn read_nas_job(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a nas_job resource
    async fn update_nas_job(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a nas_job resource
    async fn delete_nas_job(&self, id: &str) -> Result<()> {
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
    // Rag_corpora resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a rag_corpora resource
    async fn plan_rag_corpora(
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

    /// Create a new rag_corpora resource
    async fn create_rag_corpora(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a rag_corpora resource
    async fn read_rag_corpora(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a rag_corpora resource
    async fn update_rag_corpora(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a rag_corpora resource
    async fn delete_rag_corpora(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Data_labeling_job resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a data_labeling_job resource
    async fn plan_data_labeling_job(
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

    /// Create a new data_labeling_job resource
    async fn create_data_labeling_job(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a data_labeling_job resource
    async fn read_data_labeling_job(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a data_labeling_job resource
    async fn update_data_labeling_job(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a data_labeling_job resource
    async fn delete_data_labeling_job(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Deployment_resource_pool resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a deployment_resource_pool resource
    async fn plan_deployment_resource_pool(
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

    /// Create a new deployment_resource_pool resource
    async fn create_deployment_resource_pool(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a deployment_resource_pool resource
    async fn read_deployment_resource_pool(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a deployment_resource_pool resource
    async fn update_deployment_resource_pool(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a deployment_resource_pool resource
    async fn delete_deployment_resource_pool(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Feature_view resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a feature_view resource
    async fn plan_feature_view(
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

    /// Create a new feature_view resource
    async fn create_feature_view(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a feature_view resource
    async fn read_feature_view(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a feature_view resource
    async fn update_feature_view(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a feature_view resource
    async fn delete_feature_view(&self, id: &str) -> Result<()> {
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
    // Schedule resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a schedule resource
    async fn plan_schedule(
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

    /// Create a new schedule resource
    async fn create_schedule(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a schedule resource
    async fn read_schedule(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a schedule resource
    async fn update_schedule(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a schedule resource
    async fn delete_schedule(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Batch_prediction_job resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a batch_prediction_job resource
    async fn plan_batch_prediction_job(
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

    /// Create a new batch_prediction_job resource
    async fn create_batch_prediction_job(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a batch_prediction_job resource
    async fn read_batch_prediction_job(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a batch_prediction_job resource
    async fn update_batch_prediction_job(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a batch_prediction_job resource
    async fn delete_batch_prediction_job(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Feature_view_sync resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a feature_view_sync resource
    async fn plan_feature_view_sync(
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

    /// Create a new feature_view_sync resource
    async fn create_feature_view_sync(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a feature_view_sync resource
    async fn read_feature_view_sync(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a feature_view_sync resource
    async fn update_feature_view_sync(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a feature_view_sync resource
    async fn delete_feature_view_sync(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Studie resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a studie resource
    async fn plan_studie(
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

    /// Create a new studie resource
    async fn create_studie(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a studie resource
    async fn read_studie(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a studie resource
    async fn update_studie(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a studie resource
    async fn delete_studie(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Trial resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a trial resource
    async fn plan_trial(
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

    /// Create a new trial resource
    async fn create_trial(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a trial resource
    async fn read_trial(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a trial resource
    async fn update_trial(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a trial resource
    async fn delete_trial(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Specialist_pool resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a specialist_pool resource
    async fn plan_specialist_pool(
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

    /// Create a new specialist_pool resource
    async fn create_specialist_pool(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a specialist_pool resource
    async fn read_specialist_pool(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a specialist_pool resource
    async fn update_specialist_pool(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a specialist_pool resource
    async fn delete_specialist_pool(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Openapi resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a openapi resource
    async fn plan_openapi(
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

    /// Create a new openapi resource
    async fn create_openapi(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a openapi resource
    async fn read_openapi(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a openapi resource
    async fn update_openapi(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a openapi resource
    async fn delete_openapi(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Saved_querie resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a saved_querie resource
    async fn plan_saved_querie(
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

    /// Create a new saved_querie resource
    async fn create_saved_querie(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a saved_querie resource
    async fn read_saved_querie(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a saved_querie resource
    async fn update_saved_querie(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a saved_querie resource
    async fn delete_saved_querie(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Feature_view resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a feature_view resource
    async fn plan_feature_view(
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

    /// Create a new feature_view resource
    async fn create_feature_view(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a feature_view resource
    async fn read_feature_view(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a feature_view resource
    async fn update_feature_view(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a feature_view resource
    async fn delete_feature_view(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Artifact resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a artifact resource
    async fn plan_artifact(
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

    /// Create a new artifact resource
    async fn create_artifact(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a artifact resource
    async fn read_artifact(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a artifact resource
    async fn update_artifact(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a artifact resource
    async fn delete_artifact(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Notebook_runtime resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a notebook_runtime resource
    async fn plan_notebook_runtime(
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

    /// Create a new notebook_runtime resource
    async fn create_notebook_runtime(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a notebook_runtime resource
    async fn read_notebook_runtime(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a notebook_runtime resource
    async fn update_notebook_runtime(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a notebook_runtime resource
    async fn delete_notebook_runtime(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Annotation resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a annotation resource
    async fn plan_annotation(
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

    /// Create a new annotation resource
    async fn create_annotation(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a annotation resource
    async fn read_annotation(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a annotation resource
    async fn update_annotation(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a annotation resource
    async fn delete_annotation(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Notebook_runtime_template resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a notebook_runtime_template resource
    async fn plan_notebook_runtime_template(
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

    /// Create a new notebook_runtime_template resource
    async fn create_notebook_runtime_template(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a notebook_runtime_template resource
    async fn read_notebook_runtime_template(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a notebook_runtime_template resource
    async fn update_notebook_runtime_template(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a notebook_runtime_template resource
    async fn delete_notebook_runtime_template(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Saved_querie resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a saved_querie resource
    async fn plan_saved_querie(
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

    /// Create a new saved_querie resource
    async fn create_saved_querie(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a saved_querie resource
    async fn read_saved_querie(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a saved_querie resource
    async fn update_saved_querie(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a saved_querie resource
    async fn delete_saved_querie(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Tuning_job resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a tuning_job resource
    async fn plan_tuning_job(
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

    /// Create a new tuning_job resource
    async fn create_tuning_job(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a tuning_job resource
    async fn read_tuning_job(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a tuning_job resource
    async fn update_tuning_job(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a tuning_job resource
    async fn delete_tuning_job(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Indexe resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a indexe resource
    async fn plan_indexe(
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

    /// Create a new indexe resource
    async fn create_indexe(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a indexe resource
    async fn read_indexe(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a indexe resource
    async fn update_indexe(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a indexe resource
    async fn delete_indexe(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Index_endpoint resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a index_endpoint resource
    async fn plan_index_endpoint(
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

    /// Create a new index_endpoint resource
    async fn create_index_endpoint(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a index_endpoint resource
    async fn read_index_endpoint(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a index_endpoint resource
    async fn update_index_endpoint(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a index_endpoint resource
    async fn delete_index_endpoint(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Batch_prediction_job resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a batch_prediction_job resource
    async fn plan_batch_prediction_job(
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

    /// Create a new batch_prediction_job resource
    async fn create_batch_prediction_job(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a batch_prediction_job resource
    async fn read_batch_prediction_job(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a batch_prediction_job resource
    async fn update_batch_prediction_job(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a batch_prediction_job resource
    async fn delete_batch_prediction_job(&self, id: &str) -> Result<()> {
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
    // Featurestore resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a featurestore resource
    async fn plan_featurestore(
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

    /// Create a new featurestore resource
    async fn create_featurestore(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a featurestore resource
    async fn read_featurestore(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a featurestore resource
    async fn update_featurestore(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a featurestore resource
    async fn delete_featurestore(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Chat resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a chat resource
    async fn plan_chat(
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

    /// Create a new chat resource
    async fn create_chat(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a chat resource
    async fn read_chat(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a chat resource
    async fn update_chat(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a chat resource
    async fn delete_chat(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Run resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a run resource
    async fn plan_run(
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

    /// Create a new run resource
    async fn create_run(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a run resource
    async fn read_run(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a run resource
    async fn update_run(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a run resource
    async fn delete_run(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Rag_corpora resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a rag_corpora resource
    async fn plan_rag_corpora(
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

    /// Create a new rag_corpora resource
    async fn create_rag_corpora(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a rag_corpora resource
    async fn read_rag_corpora(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a rag_corpora resource
    async fn update_rag_corpora(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a rag_corpora resource
    async fn delete_rag_corpora(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Persistent_resource resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a persistent_resource resource
    async fn plan_persistent_resource(
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

    /// Create a new persistent_resource resource
    async fn create_persistent_resource(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a persistent_resource resource
    async fn read_persistent_resource(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a persistent_resource resource
    async fn update_persistent_resource(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a persistent_resource resource
    async fn delete_persistent_resource(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Feature resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a feature resource
    async fn plan_feature(
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

    /// Create a new feature resource
    async fn create_feature(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a feature resource
    async fn read_feature(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a feature resource
    async fn update_feature(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a feature resource
    async fn delete_feature(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Hyperparameter_tuning_job resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a hyperparameter_tuning_job resource
    async fn plan_hyperparameter_tuning_job(
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

    /// Create a new hyperparameter_tuning_job resource
    async fn create_hyperparameter_tuning_job(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a hyperparameter_tuning_job resource
    async fn read_hyperparameter_tuning_job(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a hyperparameter_tuning_job resource
    async fn update_hyperparameter_tuning_job(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a hyperparameter_tuning_job resource
    async fn delete_hyperparameter_tuning_job(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Specialist_pool resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a specialist_pool resource
    async fn plan_specialist_pool(
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

    /// Create a new specialist_pool resource
    async fn create_specialist_pool(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a specialist_pool resource
    async fn read_specialist_pool(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a specialist_pool resource
    async fn update_specialist_pool(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a specialist_pool resource
    async fn delete_specialist_pool(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Trial resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a trial resource
    async fn plan_trial(
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

    /// Create a new trial resource
    async fn create_trial(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a trial resource
    async fn read_trial(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a trial resource
    async fn update_trial(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a trial resource
    async fn delete_trial(&self, id: &str) -> Result<()> {
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
    // Dataset resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a dataset resource
    async fn plan_dataset(
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

    /// Create a new dataset resource
    async fn create_dataset(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a dataset resource
    async fn read_dataset(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a dataset resource
    async fn update_dataset(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a dataset resource
    async fn delete_dataset(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Reasoning_engine resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a reasoning_engine resource
    async fn plan_reasoning_engine(
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

    /// Create a new reasoning_engine resource
    async fn create_reasoning_engine(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a reasoning_engine resource
    async fn read_reasoning_engine(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a reasoning_engine resource
    async fn update_reasoning_engine(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a reasoning_engine resource
    async fn delete_reasoning_engine(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Feature_view_sync resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a feature_view_sync resource
    async fn plan_feature_view_sync(
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

    /// Create a new feature_view_sync resource
    async fn create_feature_view_sync(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a feature_view_sync resource
    async fn read_feature_view_sync(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a feature_view_sync resource
    async fn update_feature_view_sync(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a feature_view_sync resource
    async fn delete_feature_view_sync(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Execution resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a execution resource
    async fn plan_execution(
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

    /// Create a new execution resource
    async fn create_execution(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a execution resource
    async fn read_execution(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a execution resource
    async fn update_execution(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a execution resource
    async fn delete_execution(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Memorie resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a memorie resource
    async fn plan_memorie(
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

    /// Create a new memorie resource
    async fn create_memorie(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a memorie resource
    async fn read_memorie(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a memorie resource
    async fn update_memorie(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a memorie resource
    async fn delete_memorie(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Annotation_spec resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a annotation_spec resource
    async fn plan_annotation_spec(
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

    /// Create a new annotation_spec resource
    async fn create_annotation_spec(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a annotation_spec resource
    async fn read_annotation_spec(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a annotation_spec resource
    async fn update_annotation_spec(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a annotation_spec resource
    async fn delete_annotation_spec(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Feature_online_store resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a feature_online_store resource
    async fn plan_feature_online_store(
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

    /// Create a new feature_online_store resource
    async fn create_feature_online_store(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a feature_online_store resource
    async fn read_feature_online_store(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a feature_online_store resource
    async fn update_feature_online_store(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a feature_online_store resource
    async fn delete_feature_online_store(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Migratable_resource resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a migratable_resource resource
    async fn plan_migratable_resource(
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

    /// Create a new migratable_resource resource
    async fn create_migratable_resource(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a migratable_resource resource
    async fn read_migratable_resource(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a migratable_resource resource
    async fn update_migratable_resource(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a migratable_resource resource
    async fn delete_migratable_resource(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Feature_monitor resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a feature_monitor resource
    async fn plan_feature_monitor(
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

    /// Create a new feature_monitor resource
    async fn create_feature_monitor(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a feature_monitor resource
    async fn read_feature_monitor(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a feature_monitor resource
    async fn update_feature_monitor(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a feature_monitor resource
    async fn delete_feature_monitor(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Event resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a event resource
    async fn plan_event(
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

    /// Create a new event resource
    async fn create_event(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a event resource
    async fn read_event(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a event resource
    async fn update_event(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a event resource
    async fn delete_event(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Nas_job resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a nas_job resource
    async fn plan_nas_job(
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

    /// Create a new nas_job resource
    async fn create_nas_job(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a nas_job resource
    async fn read_nas_job(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a nas_job resource
    async fn update_nas_job(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a nas_job resource
    async fn delete_nas_job(&self, id: &str) -> Result<()> {
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
    // Example_store resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a example_store resource
    async fn plan_example_store(
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

    /// Create a new example_store resource
    async fn create_example_store(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a example_store resource
    async fn read_example_store(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a example_store resource
    async fn update_example_store(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a example_store resource
    async fn delete_example_store(&self, id: &str) -> Result<()> {
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
    // Dataset_version resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a dataset_version resource
    async fn plan_dataset_version(
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

    /// Create a new dataset_version resource
    async fn create_dataset_version(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a dataset_version resource
    async fn read_dataset_version(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a dataset_version resource
    async fn update_dataset_version(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a dataset_version resource
    async fn delete_dataset_version(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Model resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a model resource
    async fn plan_model(
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

    /// Create a new model resource
    async fn create_model(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a model resource
    async fn read_model(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a model resource
    async fn update_model(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a model resource
    async fn delete_model(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Feature_group resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a feature_group resource
    async fn plan_feature_group(
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

    /// Create a new feature_group resource
    async fn create_feature_group(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a feature_group resource
    async fn read_feature_group(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a feature_group resource
    async fn update_feature_group(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a feature_group resource
    async fn delete_feature_group(&self, id: &str) -> Result<()> {
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
    // Studie resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a studie resource
    async fn plan_studie(
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

    /// Create a new studie resource
    async fn create_studie(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a studie resource
    async fn read_studie(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a studie resource
    async fn update_studie(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a studie resource
    async fn delete_studie(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Metadata_schema resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a metadata_schema resource
    async fn plan_metadata_schema(
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

    /// Create a new metadata_schema resource
    async fn create_metadata_schema(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a metadata_schema resource
    async fn read_metadata_schema(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a metadata_schema resource
    async fn update_metadata_schema(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a metadata_schema resource
    async fn delete_metadata_schema(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Model_monitor resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a model_monitor resource
    async fn plan_model_monitor(
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

    /// Create a new model_monitor resource
    async fn create_model_monitor(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a model_monitor resource
    async fn read_model_monitor(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a model_monitor resource
    async fn update_model_monitor(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a model_monitor resource
    async fn delete_model_monitor(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Deployment_resource_pool resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a deployment_resource_pool resource
    async fn plan_deployment_resource_pool(
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

    /// Create a new deployment_resource_pool resource
    async fn create_deployment_resource_pool(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a deployment_resource_pool resource
    async fn read_deployment_resource_pool(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a deployment_resource_pool resource
    async fn update_deployment_resource_pool(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a deployment_resource_pool resource
    async fn delete_deployment_resource_pool(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Model_monitoring_job resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a model_monitoring_job resource
    async fn plan_model_monitoring_job(
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

    /// Create a new model_monitoring_job resource
    async fn create_model_monitoring_job(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a model_monitoring_job resource
    async fn read_model_monitoring_job(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a model_monitoring_job resource
    async fn update_model_monitoring_job(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a model_monitoring_job resource
    async fn delete_model_monitoring_job(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Model_deployment_monitoring_job resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a model_deployment_monitoring_job resource
    async fn plan_model_deployment_monitoring_job(
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

    /// Create a new model_deployment_monitoring_job resource
    async fn create_model_deployment_monitoring_job(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a model_deployment_monitoring_job resource
    async fn read_model_deployment_monitoring_job(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a model_deployment_monitoring_job resource
    async fn update_model_deployment_monitoring_job(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a model_deployment_monitoring_job resource
    async fn delete_model_deployment_monitoring_job(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Endpoint resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a endpoint resource
    async fn plan_endpoint(
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

    /// Create a new endpoint resource
    async fn create_endpoint(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a endpoint resource
    async fn read_endpoint(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a endpoint resource
    async fn update_endpoint(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a endpoint resource
    async fn delete_endpoint(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Schedule resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a schedule resource
    async fn plan_schedule(
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

    /// Create a new schedule resource
    async fn create_schedule(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a schedule resource
    async fn read_schedule(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a schedule resource
    async fn update_schedule(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a schedule resource
    async fn delete_schedule(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Model_garden_eula resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a model_garden_eula resource
    async fn plan_model_garden_eula(
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

    /// Create a new model_garden_eula resource
    async fn create_model_garden_eula(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a model_garden_eula resource
    async fn read_model_garden_eula(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a model_garden_eula resource
    async fn update_model_garden_eula(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a model_garden_eula resource
    async fn delete_model_garden_eula(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Feature_monitor_job resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a feature_monitor_job resource
    async fn plan_feature_monitor_job(
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

    /// Create a new feature_monitor_job resource
    async fn create_feature_monitor_job(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a feature_monitor_job resource
    async fn read_feature_monitor_job(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a feature_monitor_job resource
    async fn update_feature_monitor_job(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a feature_monitor_job resource
    async fn delete_feature_monitor_job(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Notebook_execution_job resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a notebook_execution_job resource
    async fn plan_notebook_execution_job(
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

    /// Create a new notebook_execution_job resource
    async fn create_notebook_execution_job(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a notebook_execution_job resource
    async fn read_notebook_execution_job(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a notebook_execution_job resource
    async fn update_notebook_execution_job(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a notebook_execution_job resource
    async fn delete_notebook_execution_job(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Tensorboard resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a tensorboard resource
    async fn plan_tensorboard(
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

    /// Create a new tensorboard resource
    async fn create_tensorboard(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a tensorboard resource
    async fn read_tensorboard(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a tensorboard resource
    async fn update_tensorboard(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a tensorboard resource
    async fn delete_tensorboard(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Rag_file resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a rag_file resource
    async fn plan_rag_file(
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

    /// Create a new rag_file resource
    async fn create_rag_file(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a rag_file resource
    async fn read_rag_file(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a rag_file resource
    async fn update_rag_file(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a rag_file resource
    async fn delete_rag_file(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Cached_content resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a cached_content resource
    async fn plan_cached_content(
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

    /// Create a new cached_content resource
    async fn create_cached_content(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a cached_content resource
    async fn read_cached_content(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a cached_content resource
    async fn update_cached_content(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a cached_content resource
    async fn delete_cached_content(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Evaluation_item resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a evaluation_item resource
    async fn plan_evaluation_item(
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

    /// Create a new evaluation_item resource
    async fn create_evaluation_item(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a evaluation_item resource
    async fn read_evaluation_item(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a evaluation_item resource
    async fn update_evaluation_item(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a evaluation_item resource
    async fn delete_evaluation_item(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Slice resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a slice resource
    async fn plan_slice(
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

    /// Create a new slice resource
    async fn create_slice(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a slice resource
    async fn read_slice(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a slice resource
    async fn update_slice(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a slice resource
    async fn delete_slice(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Extension resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a extension resource
    async fn plan_extension(
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

    /// Create a new extension resource
    async fn create_extension(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a extension resource
    async fn read_extension(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a extension resource
    async fn update_extension(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a extension resource
    async fn delete_extension(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Custom_job resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a custom_job resource
    async fn plan_custom_job(
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

    /// Create a new custom_job resource
    async fn create_custom_job(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a custom_job resource
    async fn read_custom_job(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a custom_job resource
    async fn update_custom_job(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a custom_job resource
    async fn delete_custom_job(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Data_item resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a data_item resource
    async fn plan_data_item(
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

    /// Create a new data_item resource
    async fn create_data_item(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a data_item resource
    async fn read_data_item(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a data_item resource
    async fn update_data_item(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a data_item resource
    async fn delete_data_item(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Evaluation_run resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a evaluation_run resource
    async fn plan_evaluation_run(
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

    /// Create a new evaluation_run resource
    async fn create_evaluation_run(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a evaluation_run resource
    async fn read_evaluation_run(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a evaluation_run resource
    async fn update_evaluation_run(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a evaluation_run resource
    async fn delete_evaluation_run(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Time_serie resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a time_serie resource
    async fn plan_time_serie(
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

    /// Create a new time_serie resource
    async fn create_time_serie(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a time_serie resource
    async fn read_time_serie(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a time_serie resource
    async fn update_time_serie(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a time_serie resource
    async fn delete_time_serie(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Sandbox_environment resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a sandbox_environment resource
    async fn plan_sandbox_environment(
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

    /// Create a new sandbox_environment resource
    async fn create_sandbox_environment(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a sandbox_environment resource
    async fn read_sandbox_environment(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a sandbox_environment resource
    async fn update_sandbox_environment(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a sandbox_environment resource
    async fn delete_sandbox_environment(&self, id: &str) -> Result<()> {
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
    // Evaluation_set resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a evaluation_set resource
    async fn plan_evaluation_set(
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

    /// Create a new evaluation_set resource
    async fn create_evaluation_set(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a evaluation_set resource
    async fn read_evaluation_set(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a evaluation_set resource
    async fn update_evaluation_set(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a evaluation_set resource
    async fn delete_evaluation_set(&self, id: &str) -> Result<()> {
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
    // Data_labeling_job resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a data_labeling_job resource
    async fn plan_data_labeling_job(
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

    /// Create a new data_labeling_job resource
    async fn create_data_labeling_job(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a data_labeling_job resource
    async fn read_data_labeling_job(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a data_labeling_job resource
    async fn update_data_labeling_job(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a data_labeling_job resource
    async fn delete_data_labeling_job(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Pipeline_job resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a pipeline_job resource
    async fn plan_pipeline_job(
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

    /// Create a new pipeline_job resource
    async fn create_pipeline_job(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a pipeline_job resource
    async fn read_pipeline_job(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a pipeline_job resource
    async fn update_pipeline_job(&self, id: &str, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a pipeline_job resource
    async fn delete_pipeline_job(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Training_pipeline resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a training_pipeline resource
    async fn plan_training_pipeline(
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

    /// Create a new training_pipeline resource
    async fn create_training_pipeline(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a training_pipeline resource
    async fn read_training_pipeline(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a training_pipeline resource
    async fn update_training_pipeline(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a training_pipeline resource
    async fn delete_training_pipeline(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Nas_trial_detail resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a nas_trial_detail resource
    async fn plan_nas_trial_detail(
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

    /// Create a new nas_trial_detail resource
    async fn create_nas_trial_detail(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a nas_trial_detail resource
    async fn read_nas_trial_detail(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a nas_trial_detail resource
    async fn update_nas_trial_detail(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a nas_trial_detail resource
    async fn delete_nas_trial_detail(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }

    // ------------------------------------------------------------------------
    // Metadata_store resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a metadata_store resource
    async fn plan_metadata_store(
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

    /// Create a new metadata_store resource
    async fn create_metadata_store(&self, input: ResourceInput) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id("placeholder-id"))
    }

    /// Read a metadata_store resource
    async fn read_metadata_store(&self, id: &str) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Update a metadata_store resource
    async fn update_metadata_store(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new().with_id(id))
    }

    /// Delete a metadata_store resource
    async fn delete_metadata_store(&self, id: &str) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }
}
