//! Aiplatform Service
//!
//! Auto-generated service module for aiplatform

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for aiplatform
pub struct AiplatformService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> AiplatformService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get annotation resource handler
    pub fn annotation(&self) -> resources::Annotation<'_> {
        resources::Annotation::new(self.provider)
    }
    /// Get extension resource handler
    pub fn extension(&self) -> resources::Extension<'_> {
        resources::Extension::new(self.provider)
    }
    /// Get batch_prediction_job resource handler
    pub fn batch_prediction_job(&self) -> resources::Batch_prediction_job<'_> {
        resources::Batch_prediction_job::new(self.provider)
    }
    /// Get sandbox_environment resource handler
    pub fn sandbox_environment(&self) -> resources::Sandbox_environment<'_> {
        resources::Sandbox_environment::new(self.provider)
    }
    /// Get migratable_resource resource handler
    pub fn migratable_resource(&self) -> resources::Migratable_resource<'_> {
        resources::Migratable_resource::new(self.provider)
    }
    /// Get operation resource handler
    pub fn operation(&self) -> resources::Operation<'_> {
        resources::Operation::new(self.provider)
    }
    /// Get model resource handler
    pub fn model(&self) -> resources::Model<'_> {
        resources::Model::new(self.provider)
    }
    /// Get event resource handler
    pub fn event(&self) -> resources::Event<'_> {
        resources::Event::new(self.provider)
    }
    /// Get feature resource handler
    pub fn feature(&self) -> resources::Feature<'_> {
        resources::Feature::new(self.provider)
    }
    /// Get context resource handler
    pub fn context(&self) -> resources::Context<'_> {
        resources::Context::new(self.provider)
    }
    /// Get chat resource handler
    pub fn chat(&self) -> resources::Chat<'_> {
        resources::Chat::new(self.provider)
    }
    /// Get model_monitor resource handler
    pub fn model_monitor(&self) -> resources::Model_monitor<'_> {
        resources::Model_monitor::new(self.provider)
    }
    /// Get annotation_spec resource handler
    pub fn annotation_spec(&self) -> resources::Annotation_spec<'_> {
        resources::Annotation_spec::new(self.provider)
    }
    /// Get studie resource handler
    pub fn studie(&self) -> resources::Studie<'_> {
        resources::Studie::new(self.provider)
    }
    /// Get example_store resource handler
    pub fn example_store(&self) -> resources::Example_store<'_> {
        resources::Example_store::new(self.provider)
    }
    /// Get metadata_schema resource handler
    pub fn metadata_schema(&self) -> resources::Metadata_schema<'_> {
        resources::Metadata_schema::new(self.provider)
    }
    /// Get training_pipeline resource handler
    pub fn training_pipeline(&self) -> resources::Training_pipeline<'_> {
        resources::Training_pipeline::new(self.provider)
    }
    /// Get data_item resource handler
    pub fn data_item(&self) -> resources::Data_item<'_> {
        resources::Data_item::new(self.provider)
    }
    /// Get notebook_runtime resource handler
    pub fn notebook_runtime(&self) -> resources::Notebook_runtime<'_> {
        resources::Notebook_runtime::new(self.provider)
    }
    /// Get experiment resource handler
    pub fn experiment(&self) -> resources::Experiment<'_> {
        resources::Experiment::new(self.provider)
    }
    /// Get run resource handler
    pub fn run(&self) -> resources::Run<'_> {
        resources::Run::new(self.provider)
    }
    /// Get metadata_store resource handler
    pub fn metadata_store(&self) -> resources::Metadata_store<'_> {
        resources::Metadata_store::new(self.provider)
    }
    /// Get persistent_resource resource handler
    pub fn persistent_resource(&self) -> resources::Persistent_resource<'_> {
        resources::Persistent_resource::new(self.provider)
    }
    /// Get data_labeling_job resource handler
    pub fn data_labeling_job(&self) -> resources::Data_labeling_job<'_> {
        resources::Data_labeling_job::new(self.provider)
    }
    /// Get schedule resource handler
    pub fn schedule(&self) -> resources::Schedule<'_> {
        resources::Schedule::new(self.provider)
    }
    /// Get specialist_pool resource handler
    pub fn specialist_pool(&self) -> resources::Specialist_pool<'_> {
        resources::Specialist_pool::new(self.provider)
    }
    /// Get hyperparameter_tuning_job resource handler
    pub fn hyperparameter_tuning_job(&self) -> resources::Hyperparameter_tuning_job<'_> {
        resources::Hyperparameter_tuning_job::new(self.provider)
    }
    /// Get endpoint resource handler
    pub fn endpoint(&self) -> resources::Endpoint<'_> {
        resources::Endpoint::new(self.provider)
    }
    /// Get feature_view resource handler
    pub fn feature_view(&self) -> resources::Feature_view<'_> {
        resources::Feature_view::new(self.provider)
    }
    /// Get feature_monitor resource handler
    pub fn feature_monitor(&self) -> resources::Feature_monitor<'_> {
        resources::Feature_monitor::new(self.provider)
    }
    /// Get rag_file resource handler
    pub fn rag_file(&self) -> resources::Rag_file<'_> {
        resources::Rag_file::new(self.provider)
    }
    /// Get time_serie resource handler
    pub fn time_serie(&self) -> resources::Time_serie<'_> {
        resources::Time_serie::new(self.provider)
    }
    /// Get dataset_version resource handler
    pub fn dataset_version(&self) -> resources::Dataset_version<'_> {
        resources::Dataset_version::new(self.provider)
    }
    /// Get project resource handler
    pub fn project(&self) -> resources::Project<'_> {
        resources::Project::new(self.provider)
    }
    /// Get saved_querie resource handler
    pub fn saved_querie(&self) -> resources::Saved_querie<'_> {
        resources::Saved_querie::new(self.provider)
    }
    /// Get indexe resource handler
    pub fn indexe(&self) -> resources::Indexe<'_> {
        resources::Indexe::new(self.provider)
    }
    /// Get dataset resource handler
    pub fn dataset(&self) -> resources::Dataset<'_> {
        resources::Dataset::new(self.provider)
    }
    /// Get feature_online_store resource handler
    pub fn feature_online_store(&self) -> resources::Feature_online_store<'_> {
        resources::Feature_online_store::new(self.provider)
    }
    /// Get deployment_resource_pool resource handler
    pub fn deployment_resource_pool(&self) -> resources::Deployment_resource_pool<'_> {
        resources::Deployment_resource_pool::new(self.provider)
    }
    /// Get index_endpoint resource handler
    pub fn index_endpoint(&self) -> resources::Index_endpoint<'_> {
        resources::Index_endpoint::new(self.provider)
    }
    /// Get reasoning_engine resource handler
    pub fn reasoning_engine(&self) -> resources::Reasoning_engine<'_> {
        resources::Reasoning_engine::new(self.provider)
    }
    /// Get trial resource handler
    pub fn trial(&self) -> resources::Trial<'_> {
        resources::Trial::new(self.provider)
    }
    /// Get evaluation_set resource handler
    pub fn evaluation_set(&self) -> resources::Evaluation_set<'_> {
        resources::Evaluation_set::new(self.provider)
    }
    /// Get nas_job resource handler
    pub fn nas_job(&self) -> resources::Nas_job<'_> {
        resources::Nas_job::new(self.provider)
    }
    /// Get feature_group resource handler
    pub fn feature_group(&self) -> resources::Feature_group<'_> {
        resources::Feature_group::new(self.provider)
    }
    /// Get media resource handler
    pub fn media(&self) -> resources::Media<'_> {
        resources::Media::new(self.provider)
    }
    /// Get feature_view_sync resource handler
    pub fn feature_view_sync(&self) -> resources::Feature_view_sync<'_> {
        resources::Feature_view_sync::new(self.provider)
    }
    /// Get evaluation_run resource handler
    pub fn evaluation_run(&self) -> resources::Evaluation_run<'_> {
        resources::Evaluation_run::new(self.provider)
    }
    /// Get execution resource handler
    pub fn execution(&self) -> resources::Execution<'_> {
        resources::Execution::new(self.provider)
    }
    /// Get memorie resource handler
    pub fn memorie(&self) -> resources::Memorie<'_> {
        resources::Memorie::new(self.provider)
    }
    /// Get location resource handler
    pub fn location(&self) -> resources::Location<'_> {
        resources::Location::new(self.provider)
    }
    /// Get evaluation resource handler
    pub fn evaluation(&self) -> resources::Evaluation<'_> {
        resources::Evaluation::new(self.provider)
    }
    /// Get pipeline_job resource handler
    pub fn pipeline_job(&self) -> resources::Pipeline_job<'_> {
        resources::Pipeline_job::new(self.provider)
    }
    /// Get session resource handler
    pub fn session(&self) -> resources::Session<'_> {
        resources::Session::new(self.provider)
    }
    /// Get custom_job resource handler
    pub fn custom_job(&self) -> resources::Custom_job<'_> {
        resources::Custom_job::new(self.provider)
    }
    /// Get tensorboard resource handler
    pub fn tensorboard(&self) -> resources::Tensorboard<'_> {
        resources::Tensorboard::new(self.provider)
    }
    /// Get model_deployment_monitoring_job resource handler
    pub fn model_deployment_monitoring_job(&self) -> resources::Model_deployment_monitoring_job<'_> {
        resources::Model_deployment_monitoring_job::new(self.provider)
    }
    /// Get model_monitoring_job resource handler
    pub fn model_monitoring_job(&self) -> resources::Model_monitoring_job<'_> {
        resources::Model_monitoring_job::new(self.provider)
    }
    /// Get notebook_execution_job resource handler
    pub fn notebook_execution_job(&self) -> resources::Notebook_execution_job<'_> {
        resources::Notebook_execution_job::new(self.provider)
    }
    /// Get entity_type resource handler
    pub fn entity_type(&self) -> resources::Entity_type<'_> {
        resources::Entity_type::new(self.provider)
    }
    /// Get cached_content resource handler
    pub fn cached_content(&self) -> resources::Cached_content<'_> {
        resources::Cached_content::new(self.provider)
    }
    /// Get artifact resource handler
    pub fn artifact(&self) -> resources::Artifact<'_> {
        resources::Artifact::new(self.provider)
    }
    /// Get notebook_runtime_template resource handler
    pub fn notebook_runtime_template(&self) -> resources::Notebook_runtime_template<'_> {
        resources::Notebook_runtime_template::new(self.provider)
    }
    /// Get featurestore resource handler
    pub fn featurestore(&self) -> resources::Featurestore<'_> {
        resources::Featurestore::new(self.provider)
    }
    /// Get feature_monitor_job resource handler
    pub fn feature_monitor_job(&self) -> resources::Feature_monitor_job<'_> {
        resources::Feature_monitor_job::new(self.provider)
    }
    /// Get tuning_job resource handler
    pub fn tuning_job(&self) -> resources::Tuning_job<'_> {
        resources::Tuning_job::new(self.provider)
    }
    /// Get model_garden_eula resource handler
    pub fn model_garden_eula(&self) -> resources::Model_garden_eula<'_> {
        resources::Model_garden_eula::new(self.provider)
    }
    /// Get evaluation_item resource handler
    pub fn evaluation_item(&self) -> resources::Evaluation_item<'_> {
        resources::Evaluation_item::new(self.provider)
    }
    /// Get nas_trial_detail resource handler
    pub fn nas_trial_detail(&self) -> resources::Nas_trial_detail<'_> {
        resources::Nas_trial_detail::new(self.provider)
    }
    /// Get slice resource handler
    pub fn slice(&self) -> resources::Slice<'_> {
        resources::Slice::new(self.provider)
    }
    /// Get rag_corpora resource handler
    pub fn rag_corpora(&self) -> resources::Rag_corpora<'_> {
        resources::Rag_corpora::new(self.provider)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        // Service creation test
    }
}
