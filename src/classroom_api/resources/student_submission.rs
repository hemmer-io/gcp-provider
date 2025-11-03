//! Student_submission resource
//!
//! Returns a student submission. Returning a student submission transfers ownership of attached Drive files to the student and may also update the submission state. Unlike the Classroom application, returning a student submission does not set assignedGrade to the draftGrade value. Only a teacher of the course that contains the requested student submission may call this method. This request must be made by the Developer Console project of the [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to create the corresponding course work item. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course or course work, return the requested student submission, or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course, course work, or student submission does not exist.

use crate::{ProviderError, Result};
use std::collections::HashMap;

/// Student_submission resource handler
pub struct Student_submission<'a> {
    provider: &'a crate::GcpProvider,
}

impl<'a> Student_submission<'a> {
    pub(crate) fn new(provider: &'a crate::GcpProvider) -> Self {
        Self { provider }
    }


    /// Create a new student_submission
    ///
    /// Note: Parameter types are simplified. SDK may require specific enums/types.
    /// TODO: Convert String parameters to appropriate SDK types as needed.
    #[allow(unused_variables)]
    pub async fn create(&self, course_work_id: String, id: String, course_id: String) -> Result<String> {

        todo!("Implement create for Gcp")

    }



    /// Read/describe a student_submission
    ///
    /// TODO: Map `id` parameter to appropriate SDK field(s)
    #[allow(unused_variables)]
    pub async fn read(&self, id: &str) -> Result<()> {

        todo!("Implement read for Gcp")

    }



    /// Update a student_submission
    ///
    /// TODO: Map `id` and update fields to appropriate SDK parameters
    #[allow(unused_variables)]
    pub async fn update(&self, id: &str) -> Result<()> {

        todo!("Implement update for Gcp")

    }



}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_student_submission_operations() {
        // Test student_submission CRUD operations
    }
}
