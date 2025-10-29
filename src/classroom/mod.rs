//! Classroom Service
//!
//! Auto-generated service module for classroom

pub mod resources;

use crate::{ProviderError, Result};

/// Service handler for classroom
pub struct ClassroomService<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> ClassroomService<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }

    /// Get teacher resource handler
    pub fn teacher(&self) -> resources::Teacher<'_> {
        resources::Teacher::new(self.provider)
    }
    /// Get guardian_invitation resource handler
    pub fn guardian_invitation(&self) -> resources::Guardian_invitation<'_> {
        resources::Guardian_invitation::new(self.provider)
    }
    /// Get announcement resource handler
    pub fn announcement(&self) -> resources::Announcement<'_> {
        resources::Announcement::new(self.provider)
    }
    /// Get add_on_attachment resource handler
    pub fn add_on_attachment(&self) -> resources::Add_on_attachment<'_> {
        resources::Add_on_attachment::new(self.provider)
    }
    /// Get registration resource handler
    pub fn registration(&self) -> resources::Registration<'_> {
        resources::Registration::new(self.provider)
    }
    /// Get course resource handler
    pub fn course(&self) -> resources::Course<'_> {
        resources::Course::new(self.provider)
    }
    /// Get user_profile resource handler
    pub fn user_profile(&self) -> resources::User_profile<'_> {
        resources::User_profile::new(self.provider)
    }
    /// Get rubric resource handler
    pub fn rubric(&self) -> resources::Rubric<'_> {
        resources::Rubric::new(self.provider)
    }
    /// Get topic resource handler
    pub fn topic(&self) -> resources::Topic<'_> {
        resources::Topic::new(self.provider)
    }
    /// Get invitation resource handler
    pub fn invitation(&self) -> resources::Invitation<'_> {
        resources::Invitation::new(self.provider)
    }
    /// Get guardian resource handler
    pub fn guardian(&self) -> resources::Guardian<'_> {
        resources::Guardian::new(self.provider)
    }
    /// Get student_submission resource handler
    pub fn student_submission(&self) -> resources::Student_submission<'_> {
        resources::Student_submission::new(self.provider)
    }
    /// Get course_work_material resource handler
    pub fn course_work_material(&self) -> resources::Course_work_material<'_> {
        resources::Course_work_material::new(self.provider)
    }
    /// Get post resource handler
    pub fn post(&self) -> resources::Post<'_> {
        resources::Post::new(self.provider)
    }
    /// Get course_work resource handler
    pub fn course_work(&self) -> resources::Course_work<'_> {
        resources::Course_work::new(self.provider)
    }
    /// Get aliase resource handler
    pub fn aliase(&self) -> resources::Aliase<'_> {
        resources::Aliase::new(self.provider)
    }
    /// Get student resource handler
    pub fn student(&self) -> resources::Student<'_> {
        resources::Student::new(self.provider)
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
