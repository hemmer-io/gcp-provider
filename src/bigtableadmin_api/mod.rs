//! Bigtableadmin_api service for Gcp provider
//!
//! This module handles all bigtableadmin_api resources and their CRUD operations.

use hemmer_core::Result;
use hemmer_provider::{ResourceInput, ResourceOutput, ResourcePlan};

/// Bigtableadmin_api service handler
pub struct Bigtableadmin_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Bigtableadmin_apiService<'a> {
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
            "app_profile" => {
                self.plan_app_profile(current_state, desired_input).await
            }
            "location" => {
                self.plan_location(current_state, desired_input).await
            }
            "authorized_view" => {
                self.plan_authorized_view(current_state, desired_input).await
            }
            "hot_tablet" => {
                self.plan_hot_tablet(current_state, desired_input).await
            }
            "logical_view" => {
                self.plan_logical_view(current_state, desired_input).await
            }
            "table" => {
                self.plan_table(current_state, desired_input).await
            }
            "schema_bundle" => {
                self.plan_schema_bundle(current_state, desired_input).await
            }
            "materialized_view" => {
                self.plan_materialized_view(current_state, desired_input).await
            }
            "cluster" => {
                self.plan_cluster(current_state, desired_input).await
            }
            "instance" => {
                self.plan_instance(current_state, desired_input).await
            }
            "operation" => {
                self.plan_operation(current_state, desired_input).await
            }
            "backup" => {
                self.plan_backup(current_state, desired_input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "bigtableadmin_api",
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
            "app_profile" => {
                self.create_app_profile(input).await
            }
            "location" => {
                self.create_location(input).await
            }
            "authorized_view" => {
                self.create_authorized_view(input).await
            }
            "hot_tablet" => {
                self.create_hot_tablet(input).await
            }
            "logical_view" => {
                self.create_logical_view(input).await
            }
            "table" => {
                self.create_table(input).await
            }
            "schema_bundle" => {
                self.create_schema_bundle(input).await
            }
            "materialized_view" => {
                self.create_materialized_view(input).await
            }
            "cluster" => {
                self.create_cluster(input).await
            }
            "instance" => {
                self.create_instance(input).await
            }
            "operation" => {
                self.create_operation(input).await
            }
            "backup" => {
                self.create_backup(input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "bigtableadmin_api",
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
            "app_profile" => {
                self.read_app_profile(id).await
            }
            "location" => {
                self.read_location(id).await
            }
            "authorized_view" => {
                self.read_authorized_view(id).await
            }
            "hot_tablet" => {
                self.read_hot_tablet(id).await
            }
            "logical_view" => {
                self.read_logical_view(id).await
            }
            "table" => {
                self.read_table(id).await
            }
            "schema_bundle" => {
                self.read_schema_bundle(id).await
            }
            "materialized_view" => {
                self.read_materialized_view(id).await
            }
            "cluster" => {
                self.read_cluster(id).await
            }
            "instance" => {
                self.read_instance(id).await
            }
            "operation" => {
                self.read_operation(id).await
            }
            "backup" => {
                self.read_backup(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "bigtableadmin_api",
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
            "app_profile" => {
                self.update_app_profile(id, input).await
            }
            "location" => {
                self.update_location(id, input).await
            }
            "authorized_view" => {
                self.update_authorized_view(id, input).await
            }
            "hot_tablet" => {
                self.update_hot_tablet(id, input).await
            }
            "logical_view" => {
                self.update_logical_view(id, input).await
            }
            "table" => {
                self.update_table(id, input).await
            }
            "schema_bundle" => {
                self.update_schema_bundle(id, input).await
            }
            "materialized_view" => {
                self.update_materialized_view(id, input).await
            }
            "cluster" => {
                self.update_cluster(id, input).await
            }
            "instance" => {
                self.update_instance(id, input).await
            }
            "operation" => {
                self.update_operation(id, input).await
            }
            "backup" => {
                self.update_backup(id, input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "bigtableadmin_api",
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
            "app_profile" => {
                self.delete_app_profile(id).await
            }
            "location" => {
                self.delete_location(id).await
            }
            "authorized_view" => {
                self.delete_authorized_view(id).await
            }
            "hot_tablet" => {
                self.delete_hot_tablet(id).await
            }
            "logical_view" => {
                self.delete_logical_view(id).await
            }
            "table" => {
                self.delete_table(id).await
            }
            "schema_bundle" => {
                self.delete_schema_bundle(id).await
            }
            "materialized_view" => {
                self.delete_materialized_view(id).await
            }
            "cluster" => {
                self.delete_cluster(id).await
            }
            "instance" => {
                self.delete_instance(id).await
            }
            "operation" => {
                self.delete_operation(id).await
            }
            "backup" => {
                self.delete_backup(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "bigtableadmin_api",
                resource_name
            ))),
        }
    }

    // ========================================================================
    // Resource-specific CRUD implementations
    // ========================================================================


    // ------------------------------------------------------------------------
    // App_profile resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a app_profile resource
    async fn plan_app_profile(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new app_profile resource
    async fn create_app_profile(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a app_profile resource
    async fn read_app_profile(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a app_profile resource
    async fn update_app_profile(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a app_profile resource
    async fn delete_app_profile(
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
    // Authorized_view resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a authorized_view resource
    async fn plan_authorized_view(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new authorized_view resource
    async fn create_authorized_view(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a authorized_view resource
    async fn read_authorized_view(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a authorized_view resource
    async fn update_authorized_view(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a authorized_view resource
    async fn delete_authorized_view(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Hot_tablet resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a hot_tablet resource
    async fn plan_hot_tablet(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new hot_tablet resource
    async fn create_hot_tablet(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a hot_tablet resource
    async fn read_hot_tablet(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a hot_tablet resource
    async fn update_hot_tablet(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a hot_tablet resource
    async fn delete_hot_tablet(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Logical_view resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a logical_view resource
    async fn plan_logical_view(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new logical_view resource
    async fn create_logical_view(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a logical_view resource
    async fn read_logical_view(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a logical_view resource
    async fn update_logical_view(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a logical_view resource
    async fn delete_logical_view(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Table resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a table resource
    async fn plan_table(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new table resource
    async fn create_table(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a table resource
    async fn read_table(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a table resource
    async fn update_table(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a table resource
    async fn delete_table(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Schema_bundle resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a schema_bundle resource
    async fn plan_schema_bundle(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new schema_bundle resource
    async fn create_schema_bundle(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a schema_bundle resource
    async fn read_schema_bundle(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a schema_bundle resource
    async fn update_schema_bundle(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a schema_bundle resource
    async fn delete_schema_bundle(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Materialized_view resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a materialized_view resource
    async fn plan_materialized_view(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new materialized_view resource
    async fn create_materialized_view(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a materialized_view resource
    async fn read_materialized_view(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a materialized_view resource
    async fn update_materialized_view(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a materialized_view resource
    async fn delete_materialized_view(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Cluster resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a cluster resource
    async fn plan_cluster(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new cluster resource
    async fn create_cluster(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a cluster resource
    async fn read_cluster(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a cluster resource
    async fn update_cluster(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a cluster resource
    async fn delete_cluster(
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
    // Backup resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a backup resource
    async fn plan_backup(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new backup resource
    async fn create_backup(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a backup resource
    async fn read_backup(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a backup resource
    async fn update_backup(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a backup resource
    async fn delete_backup(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


}
