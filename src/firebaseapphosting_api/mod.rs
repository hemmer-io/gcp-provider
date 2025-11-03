//! Firebaseapphosting_api service for Gcp provider
//!
//! This module handles all firebaseapphosting_api resources and their CRUD operations.

use hemmer_core::Result;
use hemmer_provider::{ResourceInput, ResourceOutput, ResourcePlan};

/// Firebaseapphosting_api service handler
pub struct Firebaseapphosting_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Firebaseapphosting_apiService<'a> {
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
            "backend" => {
                self.plan_backend(current_state, desired_input).await
            }
            "domain" => {
                self.plan_domain(current_state, desired_input).await
            }
            "location" => {
                self.plan_location(current_state, desired_input).await
            }
            "build" => {
                self.plan_build(current_state, desired_input).await
            }
            "rollout" => {
                self.plan_rollout(current_state, desired_input).await
            }
            "traffic" => {
                self.plan_traffic(current_state, desired_input).await
            }
            "operation" => {
                self.plan_operation(current_state, desired_input).await
            }
            "backend" => {
                self.plan_backend(current_state, desired_input).await
            }
            "build" => {
                self.plan_build(current_state, desired_input).await
            }
            "domain" => {
                self.plan_domain(current_state, desired_input).await
            }
            "location" => {
                self.plan_location(current_state, desired_input).await
            }
            "rollout" => {
                self.plan_rollout(current_state, desired_input).await
            }
            "operation" => {
                self.plan_operation(current_state, desired_input).await
            }
            "traffic" => {
                self.plan_traffic(current_state, desired_input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "firebaseapphosting_api",
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
            "backend" => {
                self.create_backend(input).await
            }
            "domain" => {
                self.create_domain(input).await
            }
            "location" => {
                self.create_location(input).await
            }
            "build" => {
                self.create_build(input).await
            }
            "rollout" => {
                self.create_rollout(input).await
            }
            "traffic" => {
                self.create_traffic(input).await
            }
            "operation" => {
                self.create_operation(input).await
            }
            "backend" => {
                self.create_backend(input).await
            }
            "build" => {
                self.create_build(input).await
            }
            "domain" => {
                self.create_domain(input).await
            }
            "location" => {
                self.create_location(input).await
            }
            "rollout" => {
                self.create_rollout(input).await
            }
            "operation" => {
                self.create_operation(input).await
            }
            "traffic" => {
                self.create_traffic(input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "firebaseapphosting_api",
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
            "backend" => {
                self.read_backend(id).await
            }
            "domain" => {
                self.read_domain(id).await
            }
            "location" => {
                self.read_location(id).await
            }
            "build" => {
                self.read_build(id).await
            }
            "rollout" => {
                self.read_rollout(id).await
            }
            "traffic" => {
                self.read_traffic(id).await
            }
            "operation" => {
                self.read_operation(id).await
            }
            "backend" => {
                self.read_backend(id).await
            }
            "build" => {
                self.read_build(id).await
            }
            "domain" => {
                self.read_domain(id).await
            }
            "location" => {
                self.read_location(id).await
            }
            "rollout" => {
                self.read_rollout(id).await
            }
            "operation" => {
                self.read_operation(id).await
            }
            "traffic" => {
                self.read_traffic(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "firebaseapphosting_api",
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
            "backend" => {
                self.update_backend(id, input).await
            }
            "domain" => {
                self.update_domain(id, input).await
            }
            "location" => {
                self.update_location(id, input).await
            }
            "build" => {
                self.update_build(id, input).await
            }
            "rollout" => {
                self.update_rollout(id, input).await
            }
            "traffic" => {
                self.update_traffic(id, input).await
            }
            "operation" => {
                self.update_operation(id, input).await
            }
            "backend" => {
                self.update_backend(id, input).await
            }
            "build" => {
                self.update_build(id, input).await
            }
            "domain" => {
                self.update_domain(id, input).await
            }
            "location" => {
                self.update_location(id, input).await
            }
            "rollout" => {
                self.update_rollout(id, input).await
            }
            "operation" => {
                self.update_operation(id, input).await
            }
            "traffic" => {
                self.update_traffic(id, input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "firebaseapphosting_api",
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
            "backend" => {
                self.delete_backend(id).await
            }
            "domain" => {
                self.delete_domain(id).await
            }
            "location" => {
                self.delete_location(id).await
            }
            "build" => {
                self.delete_build(id).await
            }
            "rollout" => {
                self.delete_rollout(id).await
            }
            "traffic" => {
                self.delete_traffic(id).await
            }
            "operation" => {
                self.delete_operation(id).await
            }
            "backend" => {
                self.delete_backend(id).await
            }
            "build" => {
                self.delete_build(id).await
            }
            "domain" => {
                self.delete_domain(id).await
            }
            "location" => {
                self.delete_location(id).await
            }
            "rollout" => {
                self.delete_rollout(id).await
            }
            "operation" => {
                self.delete_operation(id).await
            }
            "traffic" => {
                self.delete_traffic(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "firebaseapphosting_api",
                resource_name
            ))),
        }
    }

    // ========================================================================
    // Resource-specific CRUD implementations
    // ========================================================================


    // ------------------------------------------------------------------------
    // Backend resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a backend resource
    async fn plan_backend(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new backend resource
    async fn create_backend(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a backend resource
    async fn read_backend(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a backend resource
    async fn update_backend(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a backend resource
    async fn delete_backend(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Domain resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a domain resource
    async fn plan_domain(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new domain resource
    async fn create_domain(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a domain resource
    async fn read_domain(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a domain resource
    async fn update_domain(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a domain resource
    async fn delete_domain(
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
    // Build resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a build resource
    async fn plan_build(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new build resource
    async fn create_build(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a build resource
    async fn read_build(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a build resource
    async fn update_build(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a build resource
    async fn delete_build(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Rollout resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a rollout resource
    async fn plan_rollout(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new rollout resource
    async fn create_rollout(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a rollout resource
    async fn read_rollout(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a rollout resource
    async fn update_rollout(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a rollout resource
    async fn delete_rollout(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Traffic resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a traffic resource
    async fn plan_traffic(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new traffic resource
    async fn create_traffic(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a traffic resource
    async fn read_traffic(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a traffic resource
    async fn update_traffic(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a traffic resource
    async fn delete_traffic(
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
    // Backend resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a backend resource
    async fn plan_backend(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new backend resource
    async fn create_backend(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a backend resource
    async fn read_backend(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a backend resource
    async fn update_backend(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a backend resource
    async fn delete_backend(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Build resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a build resource
    async fn plan_build(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new build resource
    async fn create_build(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a build resource
    async fn read_build(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a build resource
    async fn update_build(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a build resource
    async fn delete_build(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Domain resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a domain resource
    async fn plan_domain(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new domain resource
    async fn create_domain(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a domain resource
    async fn read_domain(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a domain resource
    async fn update_domain(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a domain resource
    async fn delete_domain(
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
    // Rollout resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a rollout resource
    async fn plan_rollout(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new rollout resource
    async fn create_rollout(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a rollout resource
    async fn read_rollout(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a rollout resource
    async fn update_rollout(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a rollout resource
    async fn delete_rollout(
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
    // Traffic resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a traffic resource
    async fn plan_traffic(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new traffic resource
    async fn create_traffic(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a traffic resource
    async fn read_traffic(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a traffic resource
    async fn update_traffic(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a traffic resource
    async fn delete_traffic(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


}
