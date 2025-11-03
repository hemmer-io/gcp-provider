//! Healthcare_api service for Gcp provider
//!
//! This module handles all healthcare_api resources and their CRUD operations.

use hemmer_core::Result;
use hemmer_provider::{ResourceInput, ResourceOutput, ResourcePlan};

/// Healthcare_api service handler
pub struct Healthcare_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Healthcare_apiService<'a> {
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
            "consent" => {
                self.plan_consent(current_state, desired_input).await
            }
            "dicom_store" => {
                self.plan_dicom_store(current_state, desired_input).await
            }
            "message" => {
                self.plan_message(current_state, desired_input).await
            }
            "nlp" => {
                self.plan_nlp(current_state, desired_input).await
            }
            "fhir" => {
                self.plan_fhir(current_state, desired_input).await
            }
            "user_data_mapping" => {
                self.plan_user_data_mapping(current_state, desired_input).await
            }
            "instance" => {
                self.plan_instance(current_state, desired_input).await
            }
            "fhir_store" => {
                self.plan_fhir_store(current_state, desired_input).await
            }
            "attribute_definition" => {
                self.plan_attribute_definition(current_state, desired_input).await
            }
            "operation" => {
                self.plan_operation(current_state, desired_input).await
            }
            "bulkdata" => {
                self.plan_bulkdata(current_state, desired_input).await
            }
            "data_mapper_workspace" => {
                self.plan_data_mapper_workspace(current_state, desired_input).await
            }
            "studie" => {
                self.plan_studie(current_state, desired_input).await
            }
            "location" => {
                self.plan_location(current_state, desired_input).await
            }
            "serie" => {
                self.plan_serie(current_state, desired_input).await
            }
            "consent_artifact" => {
                self.plan_consent_artifact(current_state, desired_input).await
            }
            "hl7_v2_store" => {
                self.plan_hl7_v2_store(current_state, desired_input).await
            }
            "dataset" => {
                self.plan_dataset(current_state, desired_input).await
            }
            "frame" => {
                self.plan_frame(current_state, desired_input).await
            }
            "consent_store" => {
                self.plan_consent_store(current_state, desired_input).await
            }
            "operation" => {
                self.plan_operation(current_state, desired_input).await
            }
            "hl7_v2_store" => {
                self.plan_hl7_v2_store(current_state, desired_input).await
            }
            "dataset" => {
                self.plan_dataset(current_state, desired_input).await
            }
            "location" => {
                self.plan_location(current_state, desired_input).await
            }
            "dicom_store" => {
                self.plan_dicom_store(current_state, desired_input).await
            }
            "annotation" => {
                self.plan_annotation(current_state, desired_input).await
            }
            "operation" => {
                self.plan_operation(current_state, desired_input).await
            }
            "hl7_v2_store" => {
                self.plan_hl7_v2_store(current_state, desired_input).await
            }
            "serie" => {
                self.plan_serie(current_state, desired_input).await
            }
            "dataset" => {
                self.plan_dataset(current_state, desired_input).await
            }
            "dicom_store" => {
                self.plan_dicom_store(current_state, desired_input).await
            }
            "fhir_store" => {
                self.plan_fhir_store(current_state, desired_input).await
            }
            "fhir" => {
                self.plan_fhir(current_state, desired_input).await
            }
            "location" => {
                self.plan_location(current_state, desired_input).await
            }
            "studie" => {
                self.plan_studie(current_state, desired_input).await
            }
            "instance" => {
                self.plan_instance(current_state, desired_input).await
            }
            "annotation_store" => {
                self.plan_annotation_store(current_state, desired_input).await
            }
            "dicom_web" => {
                self.plan_dicom_web(current_state, desired_input).await
            }
            "message" => {
                self.plan_message(current_state, desired_input).await
            }
            "frame" => {
                self.plan_frame(current_state, desired_input).await
            }
            "attribute_definition" => {
                self.plan_attribute_definition(current_state, desired_input).await
            }
            "studie" => {
                self.plan_studie(current_state, desired_input).await
            }
            "user_data_mapping" => {
                self.plan_user_data_mapping(current_state, desired_input).await
            }
            "consent_artifact" => {
                self.plan_consent_artifact(current_state, desired_input).await
            }
            "message" => {
                self.plan_message(current_state, desired_input).await
            }
            "serie" => {
                self.plan_serie(current_state, desired_input).await
            }
            "instance" => {
                self.plan_instance(current_state, desired_input).await
            }
            "frame" => {
                self.plan_frame(current_state, desired_input).await
            }
            "consent_store" => {
                self.plan_consent_store(current_state, desired_input).await
            }
            "location" => {
                self.plan_location(current_state, desired_input).await
            }
            "data_mapper_workspace" => {
                self.plan_data_mapper_workspace(current_state, desired_input).await
            }
            "fhir_store" => {
                self.plan_fhir_store(current_state, desired_input).await
            }
            "hl7_v2_store" => {
                self.plan_hl7_v2_store(current_state, desired_input).await
            }
            "dataset" => {
                self.plan_dataset(current_state, desired_input).await
            }
            "consent" => {
                self.plan_consent(current_state, desired_input).await
            }
            "dicom_store" => {
                self.plan_dicom_store(current_state, desired_input).await
            }
            "fhir" => {
                self.plan_fhir(current_state, desired_input).await
            }
            "bulkdata" => {
                self.plan_bulkdata(current_state, desired_input).await
            }
            "nlp" => {
                self.plan_nlp(current_state, desired_input).await
            }
            "operation" => {
                self.plan_operation(current_state, desired_input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "healthcare_api",
                resource_name
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
            "consent" => {
                self.create_consent(input).await
            }
            "dicom_store" => {
                self.create_dicom_store(input).await
            }
            "message" => {
                self.create_message(input).await
            }
            "nlp" => {
                self.create_nlp(input).await
            }
            "fhir" => {
                self.create_fhir(input).await
            }
            "user_data_mapping" => {
                self.create_user_data_mapping(input).await
            }
            "instance" => {
                self.create_instance(input).await
            }
            "fhir_store" => {
                self.create_fhir_store(input).await
            }
            "attribute_definition" => {
                self.create_attribute_definition(input).await
            }
            "operation" => {
                self.create_operation(input).await
            }
            "bulkdata" => {
                self.create_bulkdata(input).await
            }
            "data_mapper_workspace" => {
                self.create_data_mapper_workspace(input).await
            }
            "studie" => {
                self.create_studie(input).await
            }
            "location" => {
                self.create_location(input).await
            }
            "serie" => {
                self.create_serie(input).await
            }
            "consent_artifact" => {
                self.create_consent_artifact(input).await
            }
            "hl7_v2_store" => {
                self.create_hl7_v2_store(input).await
            }
            "dataset" => {
                self.create_dataset(input).await
            }
            "frame" => {
                self.create_frame(input).await
            }
            "consent_store" => {
                self.create_consent_store(input).await
            }
            "operation" => {
                self.create_operation(input).await
            }
            "hl7_v2_store" => {
                self.create_hl7_v2_store(input).await
            }
            "dataset" => {
                self.create_dataset(input).await
            }
            "location" => {
                self.create_location(input).await
            }
            "dicom_store" => {
                self.create_dicom_store(input).await
            }
            "annotation" => {
                self.create_annotation(input).await
            }
            "operation" => {
                self.create_operation(input).await
            }
            "hl7_v2_store" => {
                self.create_hl7_v2_store(input).await
            }
            "serie" => {
                self.create_serie(input).await
            }
            "dataset" => {
                self.create_dataset(input).await
            }
            "dicom_store" => {
                self.create_dicom_store(input).await
            }
            "fhir_store" => {
                self.create_fhir_store(input).await
            }
            "fhir" => {
                self.create_fhir(input).await
            }
            "location" => {
                self.create_location(input).await
            }
            "studie" => {
                self.create_studie(input).await
            }
            "instance" => {
                self.create_instance(input).await
            }
            "annotation_store" => {
                self.create_annotation_store(input).await
            }
            "dicom_web" => {
                self.create_dicom_web(input).await
            }
            "message" => {
                self.create_message(input).await
            }
            "frame" => {
                self.create_frame(input).await
            }
            "attribute_definition" => {
                self.create_attribute_definition(input).await
            }
            "studie" => {
                self.create_studie(input).await
            }
            "user_data_mapping" => {
                self.create_user_data_mapping(input).await
            }
            "consent_artifact" => {
                self.create_consent_artifact(input).await
            }
            "message" => {
                self.create_message(input).await
            }
            "serie" => {
                self.create_serie(input).await
            }
            "instance" => {
                self.create_instance(input).await
            }
            "frame" => {
                self.create_frame(input).await
            }
            "consent_store" => {
                self.create_consent_store(input).await
            }
            "location" => {
                self.create_location(input).await
            }
            "data_mapper_workspace" => {
                self.create_data_mapper_workspace(input).await
            }
            "fhir_store" => {
                self.create_fhir_store(input).await
            }
            "hl7_v2_store" => {
                self.create_hl7_v2_store(input).await
            }
            "dataset" => {
                self.create_dataset(input).await
            }
            "consent" => {
                self.create_consent(input).await
            }
            "dicom_store" => {
                self.create_dicom_store(input).await
            }
            "fhir" => {
                self.create_fhir(input).await
            }
            "bulkdata" => {
                self.create_bulkdata(input).await
            }
            "nlp" => {
                self.create_nlp(input).await
            }
            "operation" => {
                self.create_operation(input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "healthcare_api",
                resource_name
            ))),
        }
    }

    /// Read resource state
    pub async fn read_resource(
        &self,
        resource_name: &str,
        id: &str,
    ) -> Result<ResourceOutput> {
        match resource_name {
            "consent" => {
                self.read_consent(id).await
            }
            "dicom_store" => {
                self.read_dicom_store(id).await
            }
            "message" => {
                self.read_message(id).await
            }
            "nlp" => {
                self.read_nlp(id).await
            }
            "fhir" => {
                self.read_fhir(id).await
            }
            "user_data_mapping" => {
                self.read_user_data_mapping(id).await
            }
            "instance" => {
                self.read_instance(id).await
            }
            "fhir_store" => {
                self.read_fhir_store(id).await
            }
            "attribute_definition" => {
                self.read_attribute_definition(id).await
            }
            "operation" => {
                self.read_operation(id).await
            }
            "bulkdata" => {
                self.read_bulkdata(id).await
            }
            "data_mapper_workspace" => {
                self.read_data_mapper_workspace(id).await
            }
            "studie" => {
                self.read_studie(id).await
            }
            "location" => {
                self.read_location(id).await
            }
            "serie" => {
                self.read_serie(id).await
            }
            "consent_artifact" => {
                self.read_consent_artifact(id).await
            }
            "hl7_v2_store" => {
                self.read_hl7_v2_store(id).await
            }
            "dataset" => {
                self.read_dataset(id).await
            }
            "frame" => {
                self.read_frame(id).await
            }
            "consent_store" => {
                self.read_consent_store(id).await
            }
            "operation" => {
                self.read_operation(id).await
            }
            "hl7_v2_store" => {
                self.read_hl7_v2_store(id).await
            }
            "dataset" => {
                self.read_dataset(id).await
            }
            "location" => {
                self.read_location(id).await
            }
            "dicom_store" => {
                self.read_dicom_store(id).await
            }
            "annotation" => {
                self.read_annotation(id).await
            }
            "operation" => {
                self.read_operation(id).await
            }
            "hl7_v2_store" => {
                self.read_hl7_v2_store(id).await
            }
            "serie" => {
                self.read_serie(id).await
            }
            "dataset" => {
                self.read_dataset(id).await
            }
            "dicom_store" => {
                self.read_dicom_store(id).await
            }
            "fhir_store" => {
                self.read_fhir_store(id).await
            }
            "fhir" => {
                self.read_fhir(id).await
            }
            "location" => {
                self.read_location(id).await
            }
            "studie" => {
                self.read_studie(id).await
            }
            "instance" => {
                self.read_instance(id).await
            }
            "annotation_store" => {
                self.read_annotation_store(id).await
            }
            "dicom_web" => {
                self.read_dicom_web(id).await
            }
            "message" => {
                self.read_message(id).await
            }
            "frame" => {
                self.read_frame(id).await
            }
            "attribute_definition" => {
                self.read_attribute_definition(id).await
            }
            "studie" => {
                self.read_studie(id).await
            }
            "user_data_mapping" => {
                self.read_user_data_mapping(id).await
            }
            "consent_artifact" => {
                self.read_consent_artifact(id).await
            }
            "message" => {
                self.read_message(id).await
            }
            "serie" => {
                self.read_serie(id).await
            }
            "instance" => {
                self.read_instance(id).await
            }
            "frame" => {
                self.read_frame(id).await
            }
            "consent_store" => {
                self.read_consent_store(id).await
            }
            "location" => {
                self.read_location(id).await
            }
            "data_mapper_workspace" => {
                self.read_data_mapper_workspace(id).await
            }
            "fhir_store" => {
                self.read_fhir_store(id).await
            }
            "hl7_v2_store" => {
                self.read_hl7_v2_store(id).await
            }
            "dataset" => {
                self.read_dataset(id).await
            }
            "consent" => {
                self.read_consent(id).await
            }
            "dicom_store" => {
                self.read_dicom_store(id).await
            }
            "fhir" => {
                self.read_fhir(id).await
            }
            "bulkdata" => {
                self.read_bulkdata(id).await
            }
            "nlp" => {
                self.read_nlp(id).await
            }
            "operation" => {
                self.read_operation(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "healthcare_api",
                resource_name
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
            "consent" => {
                self.update_consent(id, input).await
            }
            "dicom_store" => {
                self.update_dicom_store(id, input).await
            }
            "message" => {
                self.update_message(id, input).await
            }
            "nlp" => {
                self.update_nlp(id, input).await
            }
            "fhir" => {
                self.update_fhir(id, input).await
            }
            "user_data_mapping" => {
                self.update_user_data_mapping(id, input).await
            }
            "instance" => {
                self.update_instance(id, input).await
            }
            "fhir_store" => {
                self.update_fhir_store(id, input).await
            }
            "attribute_definition" => {
                self.update_attribute_definition(id, input).await
            }
            "operation" => {
                self.update_operation(id, input).await
            }
            "bulkdata" => {
                self.update_bulkdata(id, input).await
            }
            "data_mapper_workspace" => {
                self.update_data_mapper_workspace(id, input).await
            }
            "studie" => {
                self.update_studie(id, input).await
            }
            "location" => {
                self.update_location(id, input).await
            }
            "serie" => {
                self.update_serie(id, input).await
            }
            "consent_artifact" => {
                self.update_consent_artifact(id, input).await
            }
            "hl7_v2_store" => {
                self.update_hl7_v2_store(id, input).await
            }
            "dataset" => {
                self.update_dataset(id, input).await
            }
            "frame" => {
                self.update_frame(id, input).await
            }
            "consent_store" => {
                self.update_consent_store(id, input).await
            }
            "operation" => {
                self.update_operation(id, input).await
            }
            "hl7_v2_store" => {
                self.update_hl7_v2_store(id, input).await
            }
            "dataset" => {
                self.update_dataset(id, input).await
            }
            "location" => {
                self.update_location(id, input).await
            }
            "dicom_store" => {
                self.update_dicom_store(id, input).await
            }
            "annotation" => {
                self.update_annotation(id, input).await
            }
            "operation" => {
                self.update_operation(id, input).await
            }
            "hl7_v2_store" => {
                self.update_hl7_v2_store(id, input).await
            }
            "serie" => {
                self.update_serie(id, input).await
            }
            "dataset" => {
                self.update_dataset(id, input).await
            }
            "dicom_store" => {
                self.update_dicom_store(id, input).await
            }
            "fhir_store" => {
                self.update_fhir_store(id, input).await
            }
            "fhir" => {
                self.update_fhir(id, input).await
            }
            "location" => {
                self.update_location(id, input).await
            }
            "studie" => {
                self.update_studie(id, input).await
            }
            "instance" => {
                self.update_instance(id, input).await
            }
            "annotation_store" => {
                self.update_annotation_store(id, input).await
            }
            "dicom_web" => {
                self.update_dicom_web(id, input).await
            }
            "message" => {
                self.update_message(id, input).await
            }
            "frame" => {
                self.update_frame(id, input).await
            }
            "attribute_definition" => {
                self.update_attribute_definition(id, input).await
            }
            "studie" => {
                self.update_studie(id, input).await
            }
            "user_data_mapping" => {
                self.update_user_data_mapping(id, input).await
            }
            "consent_artifact" => {
                self.update_consent_artifact(id, input).await
            }
            "message" => {
                self.update_message(id, input).await
            }
            "serie" => {
                self.update_serie(id, input).await
            }
            "instance" => {
                self.update_instance(id, input).await
            }
            "frame" => {
                self.update_frame(id, input).await
            }
            "consent_store" => {
                self.update_consent_store(id, input).await
            }
            "location" => {
                self.update_location(id, input).await
            }
            "data_mapper_workspace" => {
                self.update_data_mapper_workspace(id, input).await
            }
            "fhir_store" => {
                self.update_fhir_store(id, input).await
            }
            "hl7_v2_store" => {
                self.update_hl7_v2_store(id, input).await
            }
            "dataset" => {
                self.update_dataset(id, input).await
            }
            "consent" => {
                self.update_consent(id, input).await
            }
            "dicom_store" => {
                self.update_dicom_store(id, input).await
            }
            "fhir" => {
                self.update_fhir(id, input).await
            }
            "bulkdata" => {
                self.update_bulkdata(id, input).await
            }
            "nlp" => {
                self.update_nlp(id, input).await
            }
            "operation" => {
                self.update_operation(id, input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "healthcare_api",
                resource_name
            ))),
        }
    }

    /// Delete a resource
    pub async fn delete_resource(
        &self,
        resource_name: &str,
        id: &str,
    ) -> Result<()> {
        match resource_name {
            "consent" => {
                self.delete_consent(id).await
            }
            "dicom_store" => {
                self.delete_dicom_store(id).await
            }
            "message" => {
                self.delete_message(id).await
            }
            "nlp" => {
                self.delete_nlp(id).await
            }
            "fhir" => {
                self.delete_fhir(id).await
            }
            "user_data_mapping" => {
                self.delete_user_data_mapping(id).await
            }
            "instance" => {
                self.delete_instance(id).await
            }
            "fhir_store" => {
                self.delete_fhir_store(id).await
            }
            "attribute_definition" => {
                self.delete_attribute_definition(id).await
            }
            "operation" => {
                self.delete_operation(id).await
            }
            "bulkdata" => {
                self.delete_bulkdata(id).await
            }
            "data_mapper_workspace" => {
                self.delete_data_mapper_workspace(id).await
            }
            "studie" => {
                self.delete_studie(id).await
            }
            "location" => {
                self.delete_location(id).await
            }
            "serie" => {
                self.delete_serie(id).await
            }
            "consent_artifact" => {
                self.delete_consent_artifact(id).await
            }
            "hl7_v2_store" => {
                self.delete_hl7_v2_store(id).await
            }
            "dataset" => {
                self.delete_dataset(id).await
            }
            "frame" => {
                self.delete_frame(id).await
            }
            "consent_store" => {
                self.delete_consent_store(id).await
            }
            "operation" => {
                self.delete_operation(id).await
            }
            "hl7_v2_store" => {
                self.delete_hl7_v2_store(id).await
            }
            "dataset" => {
                self.delete_dataset(id).await
            }
            "location" => {
                self.delete_location(id).await
            }
            "dicom_store" => {
                self.delete_dicom_store(id).await
            }
            "annotation" => {
                self.delete_annotation(id).await
            }
            "operation" => {
                self.delete_operation(id).await
            }
            "hl7_v2_store" => {
                self.delete_hl7_v2_store(id).await
            }
            "serie" => {
                self.delete_serie(id).await
            }
            "dataset" => {
                self.delete_dataset(id).await
            }
            "dicom_store" => {
                self.delete_dicom_store(id).await
            }
            "fhir_store" => {
                self.delete_fhir_store(id).await
            }
            "fhir" => {
                self.delete_fhir(id).await
            }
            "location" => {
                self.delete_location(id).await
            }
            "studie" => {
                self.delete_studie(id).await
            }
            "instance" => {
                self.delete_instance(id).await
            }
            "annotation_store" => {
                self.delete_annotation_store(id).await
            }
            "dicom_web" => {
                self.delete_dicom_web(id).await
            }
            "message" => {
                self.delete_message(id).await
            }
            "frame" => {
                self.delete_frame(id).await
            }
            "attribute_definition" => {
                self.delete_attribute_definition(id).await
            }
            "studie" => {
                self.delete_studie(id).await
            }
            "user_data_mapping" => {
                self.delete_user_data_mapping(id).await
            }
            "consent_artifact" => {
                self.delete_consent_artifact(id).await
            }
            "message" => {
                self.delete_message(id).await
            }
            "serie" => {
                self.delete_serie(id).await
            }
            "instance" => {
                self.delete_instance(id).await
            }
            "frame" => {
                self.delete_frame(id).await
            }
            "consent_store" => {
                self.delete_consent_store(id).await
            }
            "location" => {
                self.delete_location(id).await
            }
            "data_mapper_workspace" => {
                self.delete_data_mapper_workspace(id).await
            }
            "fhir_store" => {
                self.delete_fhir_store(id).await
            }
            "hl7_v2_store" => {
                self.delete_hl7_v2_store(id).await
            }
            "dataset" => {
                self.delete_dataset(id).await
            }
            "consent" => {
                self.delete_consent(id).await
            }
            "dicom_store" => {
                self.delete_dicom_store(id).await
            }
            "fhir" => {
                self.delete_fhir(id).await
            }
            "bulkdata" => {
                self.delete_bulkdata(id).await
            }
            "nlp" => {
                self.delete_nlp(id).await
            }
            "operation" => {
                self.delete_operation(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "healthcare_api",
                resource_name
            ))),
        }
    }

    // ========================================================================
    // Resource-specific CRUD implementations
    // ========================================================================


    // ------------------------------------------------------------------------
    // Consent resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a consent resource
    async fn plan_consent(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new consent resource
    async fn create_consent(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a consent resource
    async fn read_consent(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a consent resource
    async fn update_consent(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a consent resource
    async fn delete_consent(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Dicom_store resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a dicom_store resource
    async fn plan_dicom_store(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new dicom_store resource
    async fn create_dicom_store(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a dicom_store resource
    async fn read_dicom_store(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a dicom_store resource
    async fn update_dicom_store(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a dicom_store resource
    async fn delete_dicom_store(
        &self,
        id: &str,
    ) -> Result<()> {
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
    async fn create_message(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a message resource
    async fn read_message(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a message resource
    async fn update_message(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a message resource
    async fn delete_message(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Nlp resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a nlp resource
    async fn plan_nlp(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new nlp resource
    async fn create_nlp(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a nlp resource
    async fn read_nlp(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a nlp resource
    async fn update_nlp(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a nlp resource
    async fn delete_nlp(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Fhir resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a fhir resource
    async fn plan_fhir(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new fhir resource
    async fn create_fhir(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a fhir resource
    async fn read_fhir(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a fhir resource
    async fn update_fhir(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a fhir resource
    async fn delete_fhir(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // User_data_mapping resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a user_data_mapping resource
    async fn plan_user_data_mapping(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new user_data_mapping resource
    async fn create_user_data_mapping(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a user_data_mapping resource
    async fn read_user_data_mapping(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a user_data_mapping resource
    async fn update_user_data_mapping(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a user_data_mapping resource
    async fn delete_user_data_mapping(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Instance resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a instance resource
    async fn plan_instance(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new instance resource
    async fn create_instance(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a instance resource
    async fn read_instance(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a instance resource
    async fn update_instance(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a instance resource
    async fn delete_instance(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Fhir_store resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a fhir_store resource
    async fn plan_fhir_store(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new fhir_store resource
    async fn create_fhir_store(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a fhir_store resource
    async fn read_fhir_store(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a fhir_store resource
    async fn update_fhir_store(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a fhir_store resource
    async fn delete_fhir_store(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Attribute_definition resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a attribute_definition resource
    async fn plan_attribute_definition(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new attribute_definition resource
    async fn create_attribute_definition(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a attribute_definition resource
    async fn read_attribute_definition(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a attribute_definition resource
    async fn update_attribute_definition(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a attribute_definition resource
    async fn delete_attribute_definition(
        &self,
        id: &str,
    ) -> Result<()> {
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
    async fn create_operation(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a operation resource
    async fn read_operation(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a operation resource
    async fn update_operation(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a operation resource
    async fn delete_operation(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Bulkdata resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a bulkdata resource
    async fn plan_bulkdata(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new bulkdata resource
    async fn create_bulkdata(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a bulkdata resource
    async fn read_bulkdata(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a bulkdata resource
    async fn update_bulkdata(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a bulkdata resource
    async fn delete_bulkdata(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Data_mapper_workspace resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a data_mapper_workspace resource
    async fn plan_data_mapper_workspace(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new data_mapper_workspace resource
    async fn create_data_mapper_workspace(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a data_mapper_workspace resource
    async fn read_data_mapper_workspace(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a data_mapper_workspace resource
    async fn update_data_mapper_workspace(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a data_mapper_workspace resource
    async fn delete_data_mapper_workspace(
        &self,
        id: &str,
    ) -> Result<()> {
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
    async fn create_studie(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a studie resource
    async fn read_studie(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a studie resource
    async fn update_studie(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a studie resource
    async fn delete_studie(
        &self,
        id: &str,
    ) -> Result<()> {
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
    async fn create_location(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a location resource
    async fn read_location(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a location resource
    async fn update_location(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a location resource
    async fn delete_location(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Serie resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a serie resource
    async fn plan_serie(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new serie resource
    async fn create_serie(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a serie resource
    async fn read_serie(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a serie resource
    async fn update_serie(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a serie resource
    async fn delete_serie(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Consent_artifact resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a consent_artifact resource
    async fn plan_consent_artifact(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new consent_artifact resource
    async fn create_consent_artifact(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a consent_artifact resource
    async fn read_consent_artifact(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a consent_artifact resource
    async fn update_consent_artifact(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a consent_artifact resource
    async fn delete_consent_artifact(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Hl7_v2_store resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a hl7_v2_store resource
    async fn plan_hl7_v2_store(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new hl7_v2_store resource
    async fn create_hl7_v2_store(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a hl7_v2_store resource
    async fn read_hl7_v2_store(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a hl7_v2_store resource
    async fn update_hl7_v2_store(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a hl7_v2_store resource
    async fn delete_hl7_v2_store(
        &self,
        id: &str,
    ) -> Result<()> {
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
    async fn create_dataset(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a dataset resource
    async fn read_dataset(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a dataset resource
    async fn update_dataset(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a dataset resource
    async fn delete_dataset(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Frame resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a frame resource
    async fn plan_frame(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new frame resource
    async fn create_frame(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a frame resource
    async fn read_frame(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a frame resource
    async fn update_frame(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a frame resource
    async fn delete_frame(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Consent_store resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a consent_store resource
    async fn plan_consent_store(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new consent_store resource
    async fn create_consent_store(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a consent_store resource
    async fn read_consent_store(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a consent_store resource
    async fn update_consent_store(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a consent_store resource
    async fn delete_consent_store(
        &self,
        id: &str,
    ) -> Result<()> {
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
    async fn create_operation(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a operation resource
    async fn read_operation(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a operation resource
    async fn update_operation(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a operation resource
    async fn delete_operation(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Hl7_v2_store resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a hl7_v2_store resource
    async fn plan_hl7_v2_store(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new hl7_v2_store resource
    async fn create_hl7_v2_store(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a hl7_v2_store resource
    async fn read_hl7_v2_store(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a hl7_v2_store resource
    async fn update_hl7_v2_store(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a hl7_v2_store resource
    async fn delete_hl7_v2_store(
        &self,
        id: &str,
    ) -> Result<()> {
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
    async fn create_dataset(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a dataset resource
    async fn read_dataset(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a dataset resource
    async fn update_dataset(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a dataset resource
    async fn delete_dataset(
        &self,
        id: &str,
    ) -> Result<()> {
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
    async fn create_location(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a location resource
    async fn read_location(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a location resource
    async fn update_location(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a location resource
    async fn delete_location(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Dicom_store resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a dicom_store resource
    async fn plan_dicom_store(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new dicom_store resource
    async fn create_dicom_store(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a dicom_store resource
    async fn read_dicom_store(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a dicom_store resource
    async fn update_dicom_store(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a dicom_store resource
    async fn delete_dicom_store(
        &self,
        id: &str,
    ) -> Result<()> {
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
    async fn create_annotation(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a annotation resource
    async fn read_annotation(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a annotation resource
    async fn update_annotation(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a annotation resource
    async fn delete_annotation(
        &self,
        id: &str,
    ) -> Result<()> {
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
    async fn create_operation(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a operation resource
    async fn read_operation(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a operation resource
    async fn update_operation(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a operation resource
    async fn delete_operation(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Hl7_v2_store resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a hl7_v2_store resource
    async fn plan_hl7_v2_store(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new hl7_v2_store resource
    async fn create_hl7_v2_store(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a hl7_v2_store resource
    async fn read_hl7_v2_store(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a hl7_v2_store resource
    async fn update_hl7_v2_store(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a hl7_v2_store resource
    async fn delete_hl7_v2_store(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Serie resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a serie resource
    async fn plan_serie(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new serie resource
    async fn create_serie(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a serie resource
    async fn read_serie(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a serie resource
    async fn update_serie(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a serie resource
    async fn delete_serie(
        &self,
        id: &str,
    ) -> Result<()> {
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
    async fn create_dataset(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a dataset resource
    async fn read_dataset(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a dataset resource
    async fn update_dataset(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a dataset resource
    async fn delete_dataset(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Dicom_store resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a dicom_store resource
    async fn plan_dicom_store(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new dicom_store resource
    async fn create_dicom_store(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a dicom_store resource
    async fn read_dicom_store(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a dicom_store resource
    async fn update_dicom_store(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a dicom_store resource
    async fn delete_dicom_store(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Fhir_store resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a fhir_store resource
    async fn plan_fhir_store(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new fhir_store resource
    async fn create_fhir_store(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a fhir_store resource
    async fn read_fhir_store(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a fhir_store resource
    async fn update_fhir_store(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a fhir_store resource
    async fn delete_fhir_store(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Fhir resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a fhir resource
    async fn plan_fhir(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new fhir resource
    async fn create_fhir(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a fhir resource
    async fn read_fhir(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a fhir resource
    async fn update_fhir(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a fhir resource
    async fn delete_fhir(
        &self,
        id: &str,
    ) -> Result<()> {
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
    async fn create_location(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a location resource
    async fn read_location(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a location resource
    async fn update_location(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a location resource
    async fn delete_location(
        &self,
        id: &str,
    ) -> Result<()> {
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
    async fn create_studie(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a studie resource
    async fn read_studie(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a studie resource
    async fn update_studie(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a studie resource
    async fn delete_studie(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Instance resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a instance resource
    async fn plan_instance(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new instance resource
    async fn create_instance(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a instance resource
    async fn read_instance(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a instance resource
    async fn update_instance(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a instance resource
    async fn delete_instance(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Annotation_store resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a annotation_store resource
    async fn plan_annotation_store(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new annotation_store resource
    async fn create_annotation_store(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a annotation_store resource
    async fn read_annotation_store(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a annotation_store resource
    async fn update_annotation_store(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a annotation_store resource
    async fn delete_annotation_store(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Dicom_web resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a dicom_web resource
    async fn plan_dicom_web(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new dicom_web resource
    async fn create_dicom_web(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a dicom_web resource
    async fn read_dicom_web(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a dicom_web resource
    async fn update_dicom_web(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a dicom_web resource
    async fn delete_dicom_web(
        &self,
        id: &str,
    ) -> Result<()> {
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
    async fn create_message(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a message resource
    async fn read_message(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a message resource
    async fn update_message(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a message resource
    async fn delete_message(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Frame resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a frame resource
    async fn plan_frame(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new frame resource
    async fn create_frame(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a frame resource
    async fn read_frame(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a frame resource
    async fn update_frame(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a frame resource
    async fn delete_frame(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Attribute_definition resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a attribute_definition resource
    async fn plan_attribute_definition(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new attribute_definition resource
    async fn create_attribute_definition(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a attribute_definition resource
    async fn read_attribute_definition(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a attribute_definition resource
    async fn update_attribute_definition(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a attribute_definition resource
    async fn delete_attribute_definition(
        &self,
        id: &str,
    ) -> Result<()> {
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
    async fn create_studie(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a studie resource
    async fn read_studie(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a studie resource
    async fn update_studie(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a studie resource
    async fn delete_studie(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // User_data_mapping resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a user_data_mapping resource
    async fn plan_user_data_mapping(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new user_data_mapping resource
    async fn create_user_data_mapping(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a user_data_mapping resource
    async fn read_user_data_mapping(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a user_data_mapping resource
    async fn update_user_data_mapping(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a user_data_mapping resource
    async fn delete_user_data_mapping(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Consent_artifact resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a consent_artifact resource
    async fn plan_consent_artifact(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new consent_artifact resource
    async fn create_consent_artifact(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a consent_artifact resource
    async fn read_consent_artifact(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a consent_artifact resource
    async fn update_consent_artifact(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a consent_artifact resource
    async fn delete_consent_artifact(
        &self,
        id: &str,
    ) -> Result<()> {
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
    async fn create_message(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a message resource
    async fn read_message(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a message resource
    async fn update_message(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a message resource
    async fn delete_message(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Serie resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a serie resource
    async fn plan_serie(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new serie resource
    async fn create_serie(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a serie resource
    async fn read_serie(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a serie resource
    async fn update_serie(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a serie resource
    async fn delete_serie(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Instance resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a instance resource
    async fn plan_instance(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new instance resource
    async fn create_instance(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a instance resource
    async fn read_instance(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a instance resource
    async fn update_instance(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a instance resource
    async fn delete_instance(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Frame resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a frame resource
    async fn plan_frame(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new frame resource
    async fn create_frame(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a frame resource
    async fn read_frame(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a frame resource
    async fn update_frame(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a frame resource
    async fn delete_frame(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Consent_store resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a consent_store resource
    async fn plan_consent_store(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new consent_store resource
    async fn create_consent_store(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a consent_store resource
    async fn read_consent_store(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a consent_store resource
    async fn update_consent_store(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a consent_store resource
    async fn delete_consent_store(
        &self,
        id: &str,
    ) -> Result<()> {
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
    async fn create_location(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a location resource
    async fn read_location(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a location resource
    async fn update_location(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a location resource
    async fn delete_location(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Data_mapper_workspace resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a data_mapper_workspace resource
    async fn plan_data_mapper_workspace(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new data_mapper_workspace resource
    async fn create_data_mapper_workspace(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a data_mapper_workspace resource
    async fn read_data_mapper_workspace(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a data_mapper_workspace resource
    async fn update_data_mapper_workspace(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a data_mapper_workspace resource
    async fn delete_data_mapper_workspace(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Fhir_store resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a fhir_store resource
    async fn plan_fhir_store(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new fhir_store resource
    async fn create_fhir_store(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a fhir_store resource
    async fn read_fhir_store(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a fhir_store resource
    async fn update_fhir_store(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a fhir_store resource
    async fn delete_fhir_store(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Hl7_v2_store resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a hl7_v2_store resource
    async fn plan_hl7_v2_store(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new hl7_v2_store resource
    async fn create_hl7_v2_store(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a hl7_v2_store resource
    async fn read_hl7_v2_store(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a hl7_v2_store resource
    async fn update_hl7_v2_store(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a hl7_v2_store resource
    async fn delete_hl7_v2_store(
        &self,
        id: &str,
    ) -> Result<()> {
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
    async fn create_dataset(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a dataset resource
    async fn read_dataset(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a dataset resource
    async fn update_dataset(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a dataset resource
    async fn delete_dataset(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Consent resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a consent resource
    async fn plan_consent(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new consent resource
    async fn create_consent(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a consent resource
    async fn read_consent(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a consent resource
    async fn update_consent(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a consent resource
    async fn delete_consent(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Dicom_store resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a dicom_store resource
    async fn plan_dicom_store(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new dicom_store resource
    async fn create_dicom_store(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a dicom_store resource
    async fn read_dicom_store(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a dicom_store resource
    async fn update_dicom_store(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a dicom_store resource
    async fn delete_dicom_store(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Fhir resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a fhir resource
    async fn plan_fhir(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new fhir resource
    async fn create_fhir(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a fhir resource
    async fn read_fhir(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a fhir resource
    async fn update_fhir(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a fhir resource
    async fn delete_fhir(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Bulkdata resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a bulkdata resource
    async fn plan_bulkdata(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new bulkdata resource
    async fn create_bulkdata(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a bulkdata resource
    async fn read_bulkdata(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a bulkdata resource
    async fn update_bulkdata(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a bulkdata resource
    async fn delete_bulkdata(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Nlp resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a nlp resource
    async fn plan_nlp(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new nlp resource
    async fn create_nlp(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a nlp resource
    async fn read_nlp(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a nlp resource
    async fn update_nlp(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a nlp resource
    async fn delete_nlp(
        &self,
        id: &str,
    ) -> Result<()> {
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
    async fn create_operation(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a operation resource
    async fn read_operation(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a operation resource
    async fn update_operation(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a operation resource
    async fn delete_operation(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


}
