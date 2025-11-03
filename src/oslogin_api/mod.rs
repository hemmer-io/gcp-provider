//! Oslogin_api service for Gcp provider
//!
//! This module handles all oslogin_api resources and their CRUD operations.

use hemmer_core::Result;
use hemmer_provider::{ResourceInput, ResourceOutput, ResourcePlan};

/// Oslogin_api service handler
pub struct Oslogin_apiService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Oslogin_apiService<'a> {
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
            "ssh_public_key" => {
                self.plan_ssh_public_key(current_state, desired_input).await
            }
            "project" => {
                self.plan_project(current_state, desired_input).await
            }
            "location" => {
                self.plan_location(current_state, desired_input).await
            }
            "user" => {
                self.plan_user(current_state, desired_input).await
            }
            "zone" => {
                self.plan_zone(current_state, desired_input).await
            }
            "user" => {
                self.plan_user(current_state, desired_input).await
            }
            "location" => {
                self.plan_location(current_state, desired_input).await
            }
            "project" => {
                self.plan_project(current_state, desired_input).await
            }
            "ssh_public_key" => {
                self.plan_ssh_public_key(current_state, desired_input).await
            }
            "project" => {
                self.plan_project(current_state, desired_input).await
            }
            "zone" => {
                self.plan_zone(current_state, desired_input).await
            }
            "user" => {
                self.plan_user(current_state, desired_input).await
            }
            "location" => {
                self.plan_location(current_state, desired_input).await
            }
            "ssh_public_key" => {
                self.plan_ssh_public_key(current_state, desired_input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "oslogin_api",
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
            "ssh_public_key" => {
                self.create_ssh_public_key(input).await
            }
            "project" => {
                self.create_project(input).await
            }
            "location" => {
                self.create_location(input).await
            }
            "user" => {
                self.create_user(input).await
            }
            "zone" => {
                self.create_zone(input).await
            }
            "user" => {
                self.create_user(input).await
            }
            "location" => {
                self.create_location(input).await
            }
            "project" => {
                self.create_project(input).await
            }
            "ssh_public_key" => {
                self.create_ssh_public_key(input).await
            }
            "project" => {
                self.create_project(input).await
            }
            "zone" => {
                self.create_zone(input).await
            }
            "user" => {
                self.create_user(input).await
            }
            "location" => {
                self.create_location(input).await
            }
            "ssh_public_key" => {
                self.create_ssh_public_key(input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "oslogin_api",
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
            "ssh_public_key" => {
                self.read_ssh_public_key(id).await
            }
            "project" => {
                self.read_project(id).await
            }
            "location" => {
                self.read_location(id).await
            }
            "user" => {
                self.read_user(id).await
            }
            "zone" => {
                self.read_zone(id).await
            }
            "user" => {
                self.read_user(id).await
            }
            "location" => {
                self.read_location(id).await
            }
            "project" => {
                self.read_project(id).await
            }
            "ssh_public_key" => {
                self.read_ssh_public_key(id).await
            }
            "project" => {
                self.read_project(id).await
            }
            "zone" => {
                self.read_zone(id).await
            }
            "user" => {
                self.read_user(id).await
            }
            "location" => {
                self.read_location(id).await
            }
            "ssh_public_key" => {
                self.read_ssh_public_key(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "oslogin_api",
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
            "ssh_public_key" => {
                self.update_ssh_public_key(id, input).await
            }
            "project" => {
                self.update_project(id, input).await
            }
            "location" => {
                self.update_location(id, input).await
            }
            "user" => {
                self.update_user(id, input).await
            }
            "zone" => {
                self.update_zone(id, input).await
            }
            "user" => {
                self.update_user(id, input).await
            }
            "location" => {
                self.update_location(id, input).await
            }
            "project" => {
                self.update_project(id, input).await
            }
            "ssh_public_key" => {
                self.update_ssh_public_key(id, input).await
            }
            "project" => {
                self.update_project(id, input).await
            }
            "zone" => {
                self.update_zone(id, input).await
            }
            "user" => {
                self.update_user(id, input).await
            }
            "location" => {
                self.update_location(id, input).await
            }
            "ssh_public_key" => {
                self.update_ssh_public_key(id, input).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "oslogin_api",
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
            "ssh_public_key" => {
                self.delete_ssh_public_key(id).await
            }
            "project" => {
                self.delete_project(id).await
            }
            "location" => {
                self.delete_location(id).await
            }
            "user" => {
                self.delete_user(id).await
            }
            "zone" => {
                self.delete_zone(id).await
            }
            "user" => {
                self.delete_user(id).await
            }
            "location" => {
                self.delete_location(id).await
            }
            "project" => {
                self.delete_project(id).await
            }
            "ssh_public_key" => {
                self.delete_ssh_public_key(id).await
            }
            "project" => {
                self.delete_project(id).await
            }
            "zone" => {
                self.delete_zone(id).await
            }
            "user" => {
                self.delete_user(id).await
            }
            "location" => {
                self.delete_location(id).await
            }
            "ssh_public_key" => {
                self.delete_ssh_public_key(id).await
            }
            _ => Err(hemmer_core::HemmerError::Provider(format!(
                "Unknown resource type: {}.{}",
                "oslogin_api",
                resource_name
            ))),
        }
    }

    // ========================================================================
    // Resource-specific CRUD implementations
    // ========================================================================


    // ------------------------------------------------------------------------
    // Ssh_public_key resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a ssh_public_key resource
    async fn plan_ssh_public_key(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new ssh_public_key resource
    async fn create_ssh_public_key(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a ssh_public_key resource
    async fn read_ssh_public_key(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a ssh_public_key resource
    async fn update_ssh_public_key(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a ssh_public_key resource
    async fn delete_ssh_public_key(
        &self,
        id: &str,
    ) -> Result<()> {
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
    async fn create_project(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a project resource
    async fn read_project(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a project resource
    async fn update_project(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a project resource
    async fn delete_project(
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
    // User resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a user resource
    async fn plan_user(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new user resource
    async fn create_user(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a user resource
    async fn read_user(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a user resource
    async fn update_user(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a user resource
    async fn delete_user(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Zone resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a zone resource
    async fn plan_zone(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new zone resource
    async fn create_zone(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a zone resource
    async fn read_zone(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a zone resource
    async fn update_zone(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a zone resource
    async fn delete_zone(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // User resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a user resource
    async fn plan_user(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new user resource
    async fn create_user(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a user resource
    async fn read_user(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a user resource
    async fn update_user(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a user resource
    async fn delete_user(
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
    async fn create_project(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a project resource
    async fn read_project(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a project resource
    async fn update_project(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a project resource
    async fn delete_project(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Ssh_public_key resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a ssh_public_key resource
    async fn plan_ssh_public_key(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new ssh_public_key resource
    async fn create_ssh_public_key(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a ssh_public_key resource
    async fn read_ssh_public_key(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a ssh_public_key resource
    async fn update_ssh_public_key(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a ssh_public_key resource
    async fn delete_ssh_public_key(
        &self,
        id: &str,
    ) -> Result<()> {
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
    async fn create_project(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a project resource
    async fn read_project(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a project resource
    async fn update_project(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a project resource
    async fn delete_project(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // Zone resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a zone resource
    async fn plan_zone(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new zone resource
    async fn create_zone(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a zone resource
    async fn read_zone(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a zone resource
    async fn update_zone(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a zone resource
    async fn delete_zone(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


    // ------------------------------------------------------------------------
    // User resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a user resource
    async fn plan_user(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new user resource
    async fn create_user(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a user resource
    async fn read_user(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a user resource
    async fn update_user(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a user resource
    async fn delete_user(
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
    // Ssh_public_key resource operations
    // ------------------------------------------------------------------------

    /// Plan changes to a ssh_public_key resource
    async fn plan_ssh_public_key(
        &self,
        current_state: Option<&ResourceOutput>,
        desired_input: &ResourceInput,
    ) -> Result<ResourcePlan> {
        // If no current state exists, this is a create operation
        if current_state.is_none() {
            return Ok(ResourcePlan::create());
        }

        // TODO: Implement proper diff logic
        // For now, return NoOp if resource exists
        Ok(ResourcePlan::no_op())
    }

    /// Create a new ssh_public_key resource
    async fn create_ssh_public_key(
        &self,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id("placeholder-id"))
    }

    /// Read a ssh_public_key resource
    async fn read_ssh_public_key(
        &self,
        id: &str,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Update a ssh_public_key resource
    async fn update_ssh_public_key(
        &self,
        id: &str,
        input: ResourceInput,
    ) -> Result<ResourceOutput> {
        // TODO: Implement Gcp SDK calls
        Ok(ResourceOutput::new()
            .with_id(id))
    }

    /// Delete a ssh_public_key resource
    async fn delete_ssh_public_key(
        &self,
        id: &str,
    ) -> Result<()> {
        // TODO: Implement Gcp SDK calls
        Ok(())
    }


}
