# Classroom_api Service



**Resources**: 17

---

## Overview

The classroom_api service provides access to 17 resource types:

- [Course_work](#course_work) [CRUD]
- [Announcement](#announcement) [CRUD]
- [Add_on_attachment](#add_on_attachment) [CRUD]
- [Teacher](#teacher) [CRD]
- [Guardian_invitation](#guardian_invitation) [CRU]
- [Student](#student) [CRD]
- [Course](#course) [CRUD]
- [Registration](#registration) [CD]
- [Aliase](#aliase) [CRD]
- [User_profile](#user_profile) [R]
- [Topic](#topic) [CRUD]
- [Rubric](#rubric) [CRUD]
- [Course_work_material](#course_work_material) [CRUD]
- [Invitation](#invitation) [CRD]
- [Post](#post) [R]
- [Guardian](#guardian) [RD]
- [Student_submission](#student_submission) [CRU]

---

## Resources


### Course_work

Creates course work. The resulting course work (and corresponding student submissions) are associated with the Developer Console project of the [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to make the request. Classroom API requests to modify course work and student submissions must be made with an OAuth client ID from the associated Developer Console project. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course, create course work in the requested course, share a Drive attachment, or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course does not exist. * `FAILED_PRECONDITION` for the following request error: * AttachmentNotVisible

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `due_time` | String |  | Optional time of day, in UTC, that submissions for this course work are due. This must be specified if `due_date` is specified. |
| `assignee_mode` | String |  | Assignee mode of the coursework. If unspecified, the default value is `ALL_STUDENTS`. |
| `assignment` | String |  | Assignment details. This is populated only when `work_type` is `ASSIGNMENT`. Read-only. |
| `individual_students_options` | String |  | Identifiers of students with access to the coursework. This field is set only if `assigneeMode` is `INDIVIDUAL_STUDENTS`. If the `assigneeMode` is `INDIVIDUAL_STUDENTS`, then only students specified in this field are assigned the coursework. |
| `state` | String |  | Status of this course work. If unspecified, the default state is `DRAFT`. |
| `creator_user_id` | String |  | Identifier for the user that created the coursework. Read-only. |
| `creation_time` | String |  | Timestamp when this course work was created. Read-only. |
| `grade_category` | String |  | The category that this coursework's grade contributes to. Present only when a category has been chosen for the coursework. May be used in calculating the overall grade. Read-only. |
| `multiple_choice_question` | String |  | Multiple choice question details. For read operations, this field is populated only when `work_type` is `MULTIPLE_CHOICE_QUESTION`. For write operations, this field must be specified when creating course work with a `work_type` of `MULTIPLE_CHOICE_QUESTION`, and it must not be set otherwise. |
| `title` | String |  | Title of this course work. The title must be a valid UTF-8 string containing between 1 and 3000 characters. |
| `description` | String |  | Optional description of this course work. If set, the description must be a valid UTF-8 string containing no more than 30,000 characters. |
| `work_type` | String |  | Type of this course work. The type is set when the course work is created and cannot be changed. |
| `id` | String |  | Classroom-assigned identifier of this course work, unique per course. Read-only. |
| `course_id` | String |  | Identifier of the course. Read-only. |
| `scheduled_time` | String |  | Optional timestamp when this course work is scheduled to be published. |
| `due_date` | String |  | Optional date, in UTC, that submissions for this course work are due. This must be specified if `due_time` is specified. |
| `submission_modification_mode` | String |  | Setting to determine when students are allowed to modify submissions. If unspecified, the default value is `MODIFIABLE_UNTIL_TURNED_IN`. |
| `alternate_link` | String |  | Absolute link to this course work in the Classroom web UI. This is only populated if `state` is `PUBLISHED`. Read-only. |
| `materials` | Vec<String> |  | Additional materials. CourseWork must have no more than 20 material items. |
| `associated_with_developer` | bool |  | Whether this course work item is associated with the Developer Console project making the request. See CreateCourseWork for more details. Read-only. |
| `max_points` | f64 |  | Maximum grade for this course work. If zero or unspecified, this assignment is considered ungraded. This must be a non-negative integer value. |
| `update_time` | String |  | Timestamp of the most recent change to this course work. Read-only. |
| `topic_id` | String |  | Identifier for the topic that this coursework is associated with. Must match an existing topic in the course. |
| `grading_period_id` | String |  | Identifier of the grading period associated with the coursework. * At creation, if unspecified, the grading period ID will be set based on the `dueDate` (or `scheduledTime` if no `dueDate` is set). * To indicate no association to any grading period, set this field to an empty string (""). * If specified, it must match an existing grading period ID in the course. |
| `course_id` | String | ✅ | Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `due_time` | String | Optional time of day, in UTC, that submissions for this course work are due. This must be specified if `due_date` is specified. |
| `assignee_mode` | String | Assignee mode of the coursework. If unspecified, the default value is `ALL_STUDENTS`. |
| `assignment` | String | Assignment details. This is populated only when `work_type` is `ASSIGNMENT`. Read-only. |
| `individual_students_options` | String | Identifiers of students with access to the coursework. This field is set only if `assigneeMode` is `INDIVIDUAL_STUDENTS`. If the `assigneeMode` is `INDIVIDUAL_STUDENTS`, then only students specified in this field are assigned the coursework. |
| `state` | String | Status of this course work. If unspecified, the default state is `DRAFT`. |
| `creator_user_id` | String | Identifier for the user that created the coursework. Read-only. |
| `creation_time` | String | Timestamp when this course work was created. Read-only. |
| `grade_category` | String | The category that this coursework's grade contributes to. Present only when a category has been chosen for the coursework. May be used in calculating the overall grade. Read-only. |
| `multiple_choice_question` | String | Multiple choice question details. For read operations, this field is populated only when `work_type` is `MULTIPLE_CHOICE_QUESTION`. For write operations, this field must be specified when creating course work with a `work_type` of `MULTIPLE_CHOICE_QUESTION`, and it must not be set otherwise. |
| `title` | String | Title of this course work. The title must be a valid UTF-8 string containing between 1 and 3000 characters. |
| `description` | String | Optional description of this course work. If set, the description must be a valid UTF-8 string containing no more than 30,000 characters. |
| `work_type` | String | Type of this course work. The type is set when the course work is created and cannot be changed. |
| `id` | String | Classroom-assigned identifier of this course work, unique per course. Read-only. |
| `course_id` | String | Identifier of the course. Read-only. |
| `scheduled_time` | String | Optional timestamp when this course work is scheduled to be published. |
| `due_date` | String | Optional date, in UTC, that submissions for this course work are due. This must be specified if `due_time` is specified. |
| `submission_modification_mode` | String | Setting to determine when students are allowed to modify submissions. If unspecified, the default value is `MODIFIABLE_UNTIL_TURNED_IN`. |
| `alternate_link` | String | Absolute link to this course work in the Classroom web UI. This is only populated if `state` is `PUBLISHED`. Read-only. |
| `materials` | Vec<String> | Additional materials. CourseWork must have no more than 20 material items. |
| `associated_with_developer` | bool | Whether this course work item is associated with the Developer Console project making the request. See CreateCourseWork for more details. Read-only. |
| `max_points` | f64 | Maximum grade for this course work. If zero or unspecified, this assignment is considered ungraded. This must be a non-negative integer value. |
| `update_time` | String | Timestamp of the most recent change to this course work. Read-only. |
| `topic_id` | String | Identifier for the topic that this coursework is associated with. Must match an existing topic in the course. |
| `grading_period_id` | String | Identifier of the grading period associated with the coursework. * At creation, if unspecified, the grading period ID will be set based on the `dueDate` (or `scheduledTime` if no `dueDate` is set). * To indicate no association to any grading period, set this field to an empty string (""). * If specified, it must match an existing grading period ID in the course. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create course_work
course_work = provider.classroom_api.Course_work {
    course_id = "value"  # Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
}

# Access course_work outputs
course_work_id = course_work.id
course_work_due_time = course_work.due_time
course_work_assignee_mode = course_work.assignee_mode
course_work_assignment = course_work.assignment
course_work_individual_students_options = course_work.individual_students_options
course_work_state = course_work.state
course_work_creator_user_id = course_work.creator_user_id
course_work_creation_time = course_work.creation_time
course_work_grade_category = course_work.grade_category
course_work_multiple_choice_question = course_work.multiple_choice_question
course_work_title = course_work.title
course_work_description = course_work.description
course_work_work_type = course_work.work_type
course_work_id = course_work.id
course_work_course_id = course_work.course_id
course_work_scheduled_time = course_work.scheduled_time
course_work_due_date = course_work.due_date
course_work_submission_modification_mode = course_work.submission_modification_mode
course_work_alternate_link = course_work.alternate_link
course_work_materials = course_work.materials
course_work_associated_with_developer = course_work.associated_with_developer
course_work_max_points = course_work.max_points
course_work_update_time = course_work.update_time
course_work_topic_id = course_work.topic_id
course_work_grading_period_id = course_work.grading_period_id
```

---


### Announcement

Creates an announcement. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course, create announcements in the requested course, share a Drive attachment, or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course does not exist. * `FAILED_PRECONDITION` for the following request error: * AttachmentNotVisible

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `scheduled_time` | String |  | Optional timestamp when this announcement is scheduled to be published. |
| `update_time` | String |  | Timestamp of the most recent change to this announcement. Read-only. |
| `state` | String |  | Status of this announcement. If unspecified, the default state is `DRAFT`. |
| `alternate_link` | String |  | Absolute link to this announcement in the Classroom web UI. This is only populated if `state` is `PUBLISHED`. Read-only. |
| `individual_students_options` | String |  | Identifiers of students with access to the announcement. This field is set only if `assigneeMode` is `INDIVIDUAL_STUDENTS`. If the `assigneeMode` is `INDIVIDUAL_STUDENTS`, then only students specified in this field can see the announcement. |
| `assignee_mode` | String |  | Assignee mode of the announcement. If unspecified, the default value is `ALL_STUDENTS`. |
| `course_id` | String |  | Identifier of the course. Read-only. |
| `creator_user_id` | String |  | Identifier for the user that created the announcement. Read-only. |
| `text` | String |  | Description of this announcement. The text must be a valid UTF-8 string containing no more than 30,000 characters. |
| `creation_time` | String |  | Timestamp when this announcement was created. Read-only. |
| `id` | String |  | Classroom-assigned identifier of this announcement, unique per course. Read-only. |
| `materials` | Vec<String> |  | Additional materials. Announcements must have no more than 20 material items. |
| `course_id` | String | ✅ | Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `scheduled_time` | String | Optional timestamp when this announcement is scheduled to be published. |
| `update_time` | String | Timestamp of the most recent change to this announcement. Read-only. |
| `state` | String | Status of this announcement. If unspecified, the default state is `DRAFT`. |
| `alternate_link` | String | Absolute link to this announcement in the Classroom web UI. This is only populated if `state` is `PUBLISHED`. Read-only. |
| `individual_students_options` | String | Identifiers of students with access to the announcement. This field is set only if `assigneeMode` is `INDIVIDUAL_STUDENTS`. If the `assigneeMode` is `INDIVIDUAL_STUDENTS`, then only students specified in this field can see the announcement. |
| `assignee_mode` | String | Assignee mode of the announcement. If unspecified, the default value is `ALL_STUDENTS`. |
| `course_id` | String | Identifier of the course. Read-only. |
| `creator_user_id` | String | Identifier for the user that created the announcement. Read-only. |
| `text` | String | Description of this announcement. The text must be a valid UTF-8 string containing no more than 30,000 characters. |
| `creation_time` | String | Timestamp when this announcement was created. Read-only. |
| `id` | String | Classroom-assigned identifier of this announcement, unique per course. Read-only. |
| `materials` | Vec<String> | Additional materials. Announcements must have no more than 20 material items. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create announcement
announcement = provider.classroom_api.Announcement {
    course_id = "value"  # Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
}

# Access announcement outputs
announcement_id = announcement.id
announcement_scheduled_time = announcement.scheduled_time
announcement_update_time = announcement.update_time
announcement_state = announcement.state
announcement_alternate_link = announcement.alternate_link
announcement_individual_students_options = announcement.individual_students_options
announcement_assignee_mode = announcement.assignee_mode
announcement_course_id = announcement.course_id
announcement_creator_user_id = announcement.creator_user_id
announcement_text = announcement.text
announcement_creation_time = announcement.creation_time
announcement_id = announcement.id
announcement_materials = announcement.materials
```

---


### Add_on_attachment

Creates an add-on attachment under a post. Requires the add-on to have permission to create new attachments on the post. This method returns the following error codes: * `PERMISSION_DENIED` for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if one of the identified resources does not exist.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `due_date` | String |  | Date, in UTC, that work on this attachment is due. This must be specified if `due_time` is specified. |
| `copy_history` | Vec<String> |  | Output only. Identifiers of attachments that were previous copies of this attachment. If the attachment was previously copied by virtue of its parent post being copied, this enumerates the identifiers of attachments that were its previous copies in ascending chronological order of copy. |
| `id` | String |  | Immutable. Classroom-assigned identifier for this attachment, unique per post. |
| `item_id` | String |  | Immutable. Identifier of the `Announcement`, `CourseWork`, or `CourseWorkMaterial` under which the attachment is attached. Unique per course. |
| `student_view_uri` | String |  | Required. URI to show the student view of the attachment. The URI will be opened in an iframe with the `courseId`, `itemId`, `itemType`, and `attachmentId` query parameters set. |
| `due_time` | String |  | Time of day, in UTC, that work on this attachment is due. This must be specified if `due_date` is specified. |
| `post_id` | String |  | Immutable. Deprecated, use `item_id` instead. |
| `title` | String |  | Required. Title of this attachment. The title must be between 1 and 1000 characters. |
| `max_points` | f64 |  | Maximum grade for this attachment. Can only be set if `studentWorkReviewUri` is set. Set to a non-zero value to indicate that the attachment supports grade passback. If set, this must be a non-negative integer value. When set to zero, the attachment will not support grade passback. |
| `course_id` | String |  | Immutable. Identifier of the course. |
| `teacher_view_uri` | String |  | Required. URI to show the teacher view of the attachment. The URI will be opened in an iframe with the `courseId`, `itemId`, `itemType`, and `attachmentId` query parameters set. |
| `student_work_review_uri` | String |  | URI for the teacher to see student work on the attachment, if applicable. The URI will be opened in an iframe with the `courseId`, `itemId`, `itemType`, `attachmentId`, and `submissionId` query parameters set. This is the same `submissionId` returned in the [`AddOnContext.studentContext`](//devsite.google.com/classroom/reference/rest/v1/AddOnContext#StudentContext) field when a student views the attachment. If the URI is omitted or removed, `max_points` will also be discarded. |
| `item_id` | String | ✅ | Identifier of the `Announcement`, `CourseWork`, or `CourseWorkMaterial` under which to create the attachment. This field is required, but is not marked as such while we are migrating from post_id. |
| `course_id` | String | ✅ | Required. Identifier of the course. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `due_date` | String | Date, in UTC, that work on this attachment is due. This must be specified if `due_time` is specified. |
| `copy_history` | Vec<String> | Output only. Identifiers of attachments that were previous copies of this attachment. If the attachment was previously copied by virtue of its parent post being copied, this enumerates the identifiers of attachments that were its previous copies in ascending chronological order of copy. |
| `id` | String | Immutable. Classroom-assigned identifier for this attachment, unique per post. |
| `item_id` | String | Immutable. Identifier of the `Announcement`, `CourseWork`, or `CourseWorkMaterial` under which the attachment is attached. Unique per course. |
| `student_view_uri` | String | Required. URI to show the student view of the attachment. The URI will be opened in an iframe with the `courseId`, `itemId`, `itemType`, and `attachmentId` query parameters set. |
| `due_time` | String | Time of day, in UTC, that work on this attachment is due. This must be specified if `due_date` is specified. |
| `post_id` | String | Immutable. Deprecated, use `item_id` instead. |
| `title` | String | Required. Title of this attachment. The title must be between 1 and 1000 characters. |
| `max_points` | f64 | Maximum grade for this attachment. Can only be set if `studentWorkReviewUri` is set. Set to a non-zero value to indicate that the attachment supports grade passback. If set, this must be a non-negative integer value. When set to zero, the attachment will not support grade passback. |
| `course_id` | String | Immutable. Identifier of the course. |
| `teacher_view_uri` | String | Required. URI to show the teacher view of the attachment. The URI will be opened in an iframe with the `courseId`, `itemId`, `itemType`, and `attachmentId` query parameters set. |
| `student_work_review_uri` | String | URI for the teacher to see student work on the attachment, if applicable. The URI will be opened in an iframe with the `courseId`, `itemId`, `itemType`, `attachmentId`, and `submissionId` query parameters set. This is the same `submissionId` returned in the [`AddOnContext.studentContext`](//devsite.google.com/classroom/reference/rest/v1/AddOnContext#StudentContext) field when a student views the attachment. If the URI is omitted or removed, `max_points` will also be discarded. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create add_on_attachment
add_on_attachment = provider.classroom_api.Add_on_attachment {
    item_id = "value"  # Identifier of the `Announcement`, `CourseWork`, or `CourseWorkMaterial` under which to create the attachment. This field is required, but is not marked as such while we are migrating from post_id.
    course_id = "value"  # Required. Identifier of the course.
}

# Access add_on_attachment outputs
add_on_attachment_id = add_on_attachment.id
add_on_attachment_due_date = add_on_attachment.due_date
add_on_attachment_copy_history = add_on_attachment.copy_history
add_on_attachment_id = add_on_attachment.id
add_on_attachment_item_id = add_on_attachment.item_id
add_on_attachment_student_view_uri = add_on_attachment.student_view_uri
add_on_attachment_due_time = add_on_attachment.due_time
add_on_attachment_post_id = add_on_attachment.post_id
add_on_attachment_title = add_on_attachment.title
add_on_attachment_max_points = add_on_attachment.max_points
add_on_attachment_course_id = add_on_attachment.course_id
add_on_attachment_teacher_view_uri = add_on_attachment.teacher_view_uri
add_on_attachment_student_work_review_uri = add_on_attachment.student_work_review_uri
```

---


### Teacher

Creates a teacher of a course. Domain administrators are permitted to [directly add](https://developers.google.com/workspace/classroom/guides/manage-users) users within their domain as teachers to courses within their domain. Non-admin users should send an Invitation instead. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to create teachers in this course or for access errors. * `NOT_FOUND` if the requested course ID does not exist. * `FAILED_PRECONDITION` if the requested user's account is disabled, for the following request errors: * CourseMemberLimitReached * CourseNotModifiable * CourseTeacherLimitReached * UserGroupsMembershipLimitReached * InactiveCourseOwner * `ALREADY_EXISTS` if the user is already a teacher or student in the course.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `profile` | String |  | Global user information for the teacher. Read-only. |
| `course_id` | String |  | Identifier of the course. Read-only. |
| `user_id` | String |  | Identifier of the user. When specified as a parameter of a request, this identifier can be one of the following: * the numeric identifier for the user * the email address of the user * the string literal `"me"`, indicating the requesting user |
| `course_id` | String | ✅ | Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `profile` | String | Global user information for the teacher. Read-only. |
| `course_id` | String | Identifier of the course. Read-only. |
| `user_id` | String | Identifier of the user. When specified as a parameter of a request, this identifier can be one of the following: * the numeric identifier for the user * the email address of the user * the string literal `"me"`, indicating the requesting user |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create teacher
teacher = provider.classroom_api.Teacher {
    course_id = "value"  # Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
}

# Access teacher outputs
teacher_id = teacher.id
teacher_profile = teacher.profile
teacher_course_id = teacher.course_id
teacher_user_id = teacher.user_id
```

---


### Guardian_invitation

Creates a guardian invitation, and sends an email to the guardian asking them to confirm that they are the student's guardian. Once the guardian accepts the invitation, their `state` will change to `COMPLETED` and they will start receiving guardian notifications. A `Guardian` resource will also be created to represent the active guardian. The request object must have the `student_id` and `invited_email_address` fields set. Failing to set these fields, or setting any other fields in the request, will result in an error. This method returns the following error codes: * `PERMISSION_DENIED` if the current user does not have permission to manage guardians, if the guardian in question has already rejected too many requests for that student, if guardians are not enabled for the domain in question, or for other access errors. * `RESOURCE_EXHAUSTED` if the student or guardian has exceeded the guardian link limit. * `INVALID_ARGUMENT` if the guardian email address is not valid (for example, if it is too long), or if the format of the student ID provided cannot be recognized (it is not an email address, nor a `user_id` from this API). This error will also be returned if read-only fields are set, or if the `state` field is set to to a value other than `PENDING`. * `NOT_FOUND` if the student ID provided is a valid student ID, but Classroom has no record of that student. * `ALREADY_EXISTS` if there is already a pending guardian invitation for the student and `invited_email_address` provided, or if the provided `invited_email_address` matches the Google account of an existing `Guardian` for this user.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `creation_time` | String |  | The time that this invitation was created. Read-only. |
| `invited_email_address` | String |  | Email address that the invitation was sent to. This field is only visible to domain administrators. |
| `state` | String |  | The state that this invitation is in. |
| `student_id` | String |  | ID of the student (in standard format) |
| `invitation_id` | String |  | Unique identifier for this invitation. Read-only. |
| `student_id` | String | ✅ | ID of the student (in standard format) |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `creation_time` | String | The time that this invitation was created. Read-only. |
| `invited_email_address` | String | Email address that the invitation was sent to. This field is only visible to domain administrators. |
| `state` | String | The state that this invitation is in. |
| `student_id` | String | ID of the student (in standard format) |
| `invitation_id` | String | Unique identifier for this invitation. Read-only. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create guardian_invitation
guardian_invitation = provider.classroom_api.Guardian_invitation {
    student_id = "value"  # ID of the student (in standard format)
}

# Access guardian_invitation outputs
guardian_invitation_id = guardian_invitation.id
guardian_invitation_creation_time = guardian_invitation.creation_time
guardian_invitation_invited_email_address = guardian_invitation.invited_email_address
guardian_invitation_state = guardian_invitation.state
guardian_invitation_student_id = guardian_invitation.student_id
guardian_invitation_invitation_id = guardian_invitation.invitation_id
```

---


### Student

Adds a user as a student of a course. Domain administrators are permitted to [directly add](https://developers.google.com/workspace/classroom/guides/manage-users) users within their domain as students to courses within their domain. Students are permitted to add themselves to a course using an enrollment code. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to create students in this course or for access errors. * `NOT_FOUND` if the requested course ID does not exist. * `FAILED_PRECONDITION` if the requested user's account is disabled, for the following request errors: * CourseMemberLimitReached * CourseNotModifiable * UserGroupsMembershipLimitReached * InactiveCourseOwner * `ALREADY_EXISTS` if the user is already a student or teacher in the course.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `student_work_folder` | String |  | Information about a Drive Folder for this student's work in this course. Only visible to the student and domain administrators. Read-only. |
| `profile` | String |  | Global user information for the student. Read-only. |
| `user_id` | String |  | Identifier of the user. When specified as a parameter of a request, this identifier can be one of the following: * the numeric identifier for the user * the email address of the user * the string literal `"me"`, indicating the requesting user |
| `course_id` | String |  | Identifier of the course. Read-only. |
| `course_id` | String | ✅ | Identifier of the course to create the student in. This identifier can be either the Classroom-assigned identifier or an alias. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `student_work_folder` | String | Information about a Drive Folder for this student's work in this course. Only visible to the student and domain administrators. Read-only. |
| `profile` | String | Global user information for the student. Read-only. |
| `user_id` | String | Identifier of the user. When specified as a parameter of a request, this identifier can be one of the following: * the numeric identifier for the user * the email address of the user * the string literal `"me"`, indicating the requesting user |
| `course_id` | String | Identifier of the course. Read-only. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create student
student = provider.classroom_api.Student {
    course_id = "value"  # Identifier of the course to create the student in. This identifier can be either the Classroom-assigned identifier or an alias.
}

# Access student outputs
student_id = student.id
student_student_work_folder = student.student_work_folder
student_profile = student.profile
student_user_id = student.user_id
student_course_id = student.course_id
```

---


### Course

Creates a course. The user specified in `ownerId` is the owner of the created course and added as a teacher. A non-admin requesting user can only create a course with themselves as the owner. Domain admins can create courses owned by any user within their domain. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to create courses or for access errors. * `NOT_FOUND` if the primary teacher is not a valid user. * `FAILED_PRECONDITION` if the course owner's account is disabled or for the following request errors: * UserCannotOwnCourse * UserGroupsMembershipLimitReached * CourseTitleCannotContainUrl * `ALREADY_EXISTS` if an alias was specified in the `id` and already exists.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `guardians_enabled` | bool |  | Whether or not guardian notifications are enabled for this course. Read-only. |
| `description` | String |  | Optional description. For example, "We'll be learning about the structure of living creatures from a combination of textbooks, guest lectures, and lab work. Expect to be excited!" If set, this field must be a valid UTF-8 string and no longer than 30,000 characters. |
| `update_time` | String |  | Time of the most recent update to this course. Specifying this field in a course update mask results in an error. Read-only. |
| `course_state` | String |  | State of the course. If unspecified, the default state is `PROVISIONED`. |
| `enrollment_code` | String |  | Enrollment code to use when joining this course. Specifying this field in a course update mask results in an error. Read-only. |
| `section` | String |  | Section of the course. For example, "Period 2". If set, this field must be a valid UTF-8 string and no longer than 2800 characters. |
| `teacher_folder` | String |  | Information about a Drive Folder that is shared with all teachers of the course. This field will only be set for teachers of the course and domain administrators. Read-only. |
| `course_material_sets` | Vec<String> |  | Sets of materials that appear on the "about" page of this course. Read-only. |
| `gradebook_settings` | String |  | The gradebook settings that specify how a student's overall grade for the course will be calculated and who it will be displayed to. Read-only. |
| `creation_time` | String |  | Creation time of the course. Specifying this field in a course update mask results in an error. Read-only. |
| `description_heading` | String |  | Optional heading for the description. For example, "Welcome to 10th Grade Biology." If set, this field must be a valid UTF-8 string and no longer than 3600 characters. |
| `id` | String |  | Identifier for this course assigned by Classroom. When creating a course, you may optionally set this identifier to an alias string in the request to create a corresponding alias. The `id` is still assigned by Classroom and cannot be updated after the course is created. Specifying this field in a course update mask results in an error. |
| `owner_id` | String |  | The identifier of the owner of a course. When specified as a parameter of a create course request, this field is required. The identifier can be one of the following: * the numeric identifier for the user * the email address of the user * the string literal `"me"`, indicating the requesting user This must be set in a create request. Admins can also specify this field in a patch course request to transfer ownership. In other contexts, it is read-only. |
| `alternate_link` | String |  | Absolute link to this course in the Classroom web UI. Read-only. |
| `calendar_id` | String |  | The Calendar ID for a calendar that all course members can see, to which Classroom adds events for course work and announcements in the course. The Calendar for a course is created asynchronously when the course is set to `CourseState.ACTIVE` for the first time (at creation time or when it is updated to `ACTIVE` through the UI or the API). The Calendar ID will not be populated until the creation process is completed. Read-only. |
| `course_group_email` | String |  | The email address of a Google group containing all members of the course. This group does not accept email and can only be used for permissions. Read-only. |
| `name` | String |  | Name of the course. For example, "10th Grade Biology". The name is required. It must be between 1 and 750 characters and a valid UTF-8 string. |
| `room` | String |  | Optional room location. For example, "301". If set, this field must be a valid UTF-8 string and no longer than 650 characters. |
| `teacher_group_email` | String |  | The email address of a Google group containing all teachers of the course. This group does not accept email and can only be used for permissions. Read-only. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `guardians_enabled` | bool | Whether or not guardian notifications are enabled for this course. Read-only. |
| `description` | String | Optional description. For example, "We'll be learning about the structure of living creatures from a combination of textbooks, guest lectures, and lab work. Expect to be excited!" If set, this field must be a valid UTF-8 string and no longer than 30,000 characters. |
| `update_time` | String | Time of the most recent update to this course. Specifying this field in a course update mask results in an error. Read-only. |
| `course_state` | String | State of the course. If unspecified, the default state is `PROVISIONED`. |
| `enrollment_code` | String | Enrollment code to use when joining this course. Specifying this field in a course update mask results in an error. Read-only. |
| `section` | String | Section of the course. For example, "Period 2". If set, this field must be a valid UTF-8 string and no longer than 2800 characters. |
| `teacher_folder` | String | Information about a Drive Folder that is shared with all teachers of the course. This field will only be set for teachers of the course and domain administrators. Read-only. |
| `course_material_sets` | Vec<String> | Sets of materials that appear on the "about" page of this course. Read-only. |
| `gradebook_settings` | String | The gradebook settings that specify how a student's overall grade for the course will be calculated and who it will be displayed to. Read-only. |
| `creation_time` | String | Creation time of the course. Specifying this field in a course update mask results in an error. Read-only. |
| `description_heading` | String | Optional heading for the description. For example, "Welcome to 10th Grade Biology." If set, this field must be a valid UTF-8 string and no longer than 3600 characters. |
| `id` | String | Identifier for this course assigned by Classroom. When creating a course, you may optionally set this identifier to an alias string in the request to create a corresponding alias. The `id` is still assigned by Classroom and cannot be updated after the course is created. Specifying this field in a course update mask results in an error. |
| `owner_id` | String | The identifier of the owner of a course. When specified as a parameter of a create course request, this field is required. The identifier can be one of the following: * the numeric identifier for the user * the email address of the user * the string literal `"me"`, indicating the requesting user This must be set in a create request. Admins can also specify this field in a patch course request to transfer ownership. In other contexts, it is read-only. |
| `alternate_link` | String | Absolute link to this course in the Classroom web UI. Read-only. |
| `calendar_id` | String | The Calendar ID for a calendar that all course members can see, to which Classroom adds events for course work and announcements in the course. The Calendar for a course is created asynchronously when the course is set to `CourseState.ACTIVE` for the first time (at creation time or when it is updated to `ACTIVE` through the UI or the API). The Calendar ID will not be populated until the creation process is completed. Read-only. |
| `course_group_email` | String | The email address of a Google group containing all members of the course. This group does not accept email and can only be used for permissions. Read-only. |
| `name` | String | Name of the course. For example, "10th Grade Biology". The name is required. It must be between 1 and 750 characters and a valid UTF-8 string. |
| `room` | String | Optional room location. For example, "301". If set, this field must be a valid UTF-8 string and no longer than 650 characters. |
| `teacher_group_email` | String | The email address of a Google group containing all teachers of the course. This group does not accept email and can only be used for permissions. Read-only. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create course
course = provider.classroom_api.Course {
}

# Access course outputs
course_id = course.id
course_guardians_enabled = course.guardians_enabled
course_description = course.description
course_update_time = course.update_time
course_course_state = course.course_state
course_enrollment_code = course.enrollment_code
course_section = course.section
course_teacher_folder = course.teacher_folder
course_course_material_sets = course.course_material_sets
course_gradebook_settings = course.gradebook_settings
course_creation_time = course.creation_time
course_description_heading = course.description_heading
course_id = course.id
course_owner_id = course.owner_id
course_alternate_link = course.alternate_link
course_calendar_id = course.calendar_id
course_course_group_email = course.course_group_email
course_name = course.name
course_room = course.room
course_teacher_group_email = course.teacher_group_email
```

---


### Registration

Creates a `Registration`, causing Classroom to start sending notifications from the provided `feed` to the destination provided in `cloudPubSubTopic`. Returns the created `Registration`. Currently, this will be the same as the argument, but with server-assigned fields such as `expiry_time` and `id` filled in. Note that any value specified for the `expiry_time` or `id` fields will be ignored. While Classroom may validate the `cloudPubSubTopic` and return errors on a best effort basis, it is the caller's responsibility to ensure that it exists and that Classroom has permission to publish to it. This method may return the following error codes: * `PERMISSION_DENIED` if: * the authenticated user does not have permission to receive notifications from the requested field; or * the current user has not granted access to the current Cloud project with the appropriate scope for the requested feed. Note that domain-wide delegation of authority is not currently supported for this purpose. If the request has the appropriate scope, but no grant exists, a Request Errors is returned. * another access error is encountered. * `INVALID_ARGUMENT` if: * no `cloudPubsubTopic` is specified, or the specified `cloudPubsubTopic` is not valid; or * no `feed` is specified, or the specified `feed` is not valid. * `NOT_FOUND` if: * the specified `feed` cannot be located, or the requesting user does not have permission to determine whether or not it exists; or * the specified `cloudPubsubTopic` cannot be located, or Classroom has not been granted permission to publish to it.

**Operations**: ✅ Create ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `expiry_time` | String |  | The time until which the `Registration` is effective. This is a read-only field assigned by the server. |
| `cloud_pubsub_topic` | String |  | The Cloud Pub/Sub topic that notifications are to be sent to. |
| `registration_id` | String |  | A server-generated unique identifier for this `Registration`. Read-only. |
| `feed` | String |  | Specification for the class of notifications that Classroom should deliver to the destination. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create registration
registration = provider.classroom_api.Registration {
}

```

---


### Aliase

Creates an alias for a course. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to create the alias or for access errors. * `NOT_FOUND` if the course does not exist. * `ALREADY_EXISTS` if the alias already exists. * `FAILED_PRECONDITION` if the alias requested does not make sense for the requesting user or course (for example, if a user not in a domain attempts to access a domain-scoped alias).

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `alias` | String |  | Alias string. The format of the string indicates the desired alias scoping. * `d:` indicates a domain-scoped alias. Example: `d:math_101` * `p:` indicates a project-scoped alias. Example: `p:abc123` This field has a maximum length of 256 characters. |
| `course_id` | String | ✅ | Identifier of the course to alias. This identifier can be either the Classroom-assigned identifier or an alias. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `next_page_token` | String | Token identifying the next page of results to return. If empty, no further results are available. |
| `aliases` | Vec<String> | The course aliases. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create aliase
aliase = provider.classroom_api.Aliase {
    course_id = "value"  # Identifier of the course to alias. This identifier can be either the Classroom-assigned identifier or an alias.
}

# Access aliase outputs
aliase_id = aliase.id
aliase_next_page_token = aliase.next_page_token
aliase_aliases = aliase.aliases
```

---


### User_profile

Returns a user profile. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access this user profile, if no profile exists with the requested ID, or for access errors.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | Name of the user. Read-only. |
| `email_address` | String | Email address of the user. Must request `https://www.googleapis.com/auth/classroom.profile.emails` scope for this field to be populated in a response body. Read-only. |
| `id` | String | Identifier of the user. Read-only. |
| `permissions` | Vec<String> | Global permissions of the user. Read-only. |
| `photo_url` | String | URL of user's profile photo. Must request `https://www.googleapis.com/auth/classroom.profile.photos` scope for this field to be populated in a response body. Read-only. |
| `verified_teacher` | bool | Represents whether a Google Workspace for Education user's domain administrator has explicitly verified them as being a teacher. This field is always false if the user is not a member of a Google Workspace for Education domain. Read-only |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access user_profile outputs
user_profile_id = user_profile.id
user_profile_name = user_profile.name
user_profile_email_address = user_profile.email_address
user_profile_id = user_profile.id
user_profile_permissions = user_profile.permissions
user_profile_photo_url = user_profile.photo_url
user_profile_verified_teacher = user_profile.verified_teacher
```

---


### Topic

Creates a topic. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course, create a topic in the requested course, or for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `ALREADY_EXISTS` if there exists a topic in the course with the same name. * `FAILED_PRECONDITION` for the following request error: * CourseTopicLimitReached * `NOT_FOUND` if the requested course does not exist.

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | String |  | The name of the topic, generated by the user. Leading and trailing whitespaces, if any, are trimmed. Also, multiple consecutive whitespaces are collapsed into one inside the name. The result must be a non-empty string. Topic names are case sensitive, and must be no longer than 100 characters. |
| `topic_id` | String |  | Unique identifier for the topic. Read-only. |
| `course_id` | String |  | Identifier of the course. Read-only. |
| `update_time` | String |  | The time the topic was last updated by the system. Read-only. |
| `course_id` | String | ✅ | Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `name` | String | The name of the topic, generated by the user. Leading and trailing whitespaces, if any, are trimmed. Also, multiple consecutive whitespaces are collapsed into one inside the name. The result must be a non-empty string. Topic names are case sensitive, and must be no longer than 100 characters. |
| `topic_id` | String | Unique identifier for the topic. Read-only. |
| `course_id` | String | Identifier of the course. Read-only. |
| `update_time` | String | The time the topic was last updated by the system. Read-only. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create topic
topic = provider.classroom_api.Topic {
    course_id = "value"  # Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
}

# Access topic outputs
topic_id = topic.id
topic_name = topic.name
topic_topic_id = topic.topic_id
topic_course_id = topic.course_id
topic_update_time = topic.update_time
```

---


### Rubric

Creates a rubric. The requesting user and course owner must have rubrics creation capabilities. For details, see [licensing requirements](https://developers.google.com/workspace/classroom/rubrics/limitations#license-requirements). For further details, see [Rubrics structure and known limitations](/classroom/rubrics/limitations). This request must be made by the Google Cloud console of the [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to create the parent course work item. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user isn't permitted to create rubrics for course work in the requested course. * `INTERNAL` if the request has insufficient OAuth scopes. * `INVALID_ARGUMENT` if the request is malformed and for the following request error: * `RubricCriteriaInvalidFormat` * `NOT_FOUND` if the requested course or course work don't exist or the user doesn't have access to the course or course work. * `FAILED_PRECONDITION` for the following request error: * `AttachmentNotVisible`

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_time` | String |  | Output only. Timestamp of the most recent change to this rubric. Read-only. |
| `id` | String |  | Classroom-assigned identifier for the rubric. This is unique among rubrics for the relevant course work. Read-only. |
| `course_work_id` | String |  | Identifier for the course work this corresponds to. Read-only. |
| `course_id` | String |  | Identifier of the course. Read-only. |
| `criteria` | Vec<String> |  | List of criteria. Each criterion is a dimension on which performance is rated. |
| `creation_time` | String |  | Output only. Timestamp when this rubric was created. Read-only. |
| `source_spreadsheet_id` | String |  | Input only. Immutable. Google Sheets ID of the spreadsheet. This spreadsheet must contain formatted rubric settings. See [Create or reuse a rubric for an assignment](https://support.google.com/edu/classroom/answer/9335069). Use of this field requires the `https://www.googleapis.com/auth/spreadsheets.readonly` or `https://www.googleapis.com/auth/spreadsheets` scope. |
| `course_id` | String | ✅ | Required. Identifier of the course. |
| `course_work_id` | String | ✅ | Required. Identifier of the course work. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `update_time` | String | Output only. Timestamp of the most recent change to this rubric. Read-only. |
| `id` | String | Classroom-assigned identifier for the rubric. This is unique among rubrics for the relevant course work. Read-only. |
| `course_work_id` | String | Identifier for the course work this corresponds to. Read-only. |
| `course_id` | String | Identifier of the course. Read-only. |
| `criteria` | Vec<String> | List of criteria. Each criterion is a dimension on which performance is rated. |
| `creation_time` | String | Output only. Timestamp when this rubric was created. Read-only. |
| `source_spreadsheet_id` | String | Input only. Immutable. Google Sheets ID of the spreadsheet. This spreadsheet must contain formatted rubric settings. See [Create or reuse a rubric for an assignment](https://support.google.com/edu/classroom/answer/9335069). Use of this field requires the `https://www.googleapis.com/auth/spreadsheets.readonly` or `https://www.googleapis.com/auth/spreadsheets` scope. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create rubric
rubric = provider.classroom_api.Rubric {
    course_id = "value"  # Required. Identifier of the course.
    course_work_id = "value"  # Required. Identifier of the course work.
}

# Access rubric outputs
rubric_id = rubric.id
rubric_update_time = rubric.update_time
rubric_id = rubric.id
rubric_course_work_id = rubric.course_work_id
rubric_course_id = rubric.course_id
rubric_criteria = rubric.criteria
rubric_creation_time = rubric.creation_time
rubric_source_spreadsheet_id = rubric.source_spreadsheet_id
```

---


### Course_work_material

Creates a course work material. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course, create course work material in the requested course, share a Drive attachment, or for access errors. * `INVALID_ARGUMENT` if the request is malformed or if more than 20 * materials are provided. * `NOT_FOUND` if the requested course does not exist. * `FAILED_PRECONDITION` for the following request error: * AttachmentNotVisible

**Operations**: ✅ Create ✅ Read ✅ Update ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `description` | String |  | Optional description of this course work material. The text must be a valid UTF-8 string containing no more than 30,000 characters. |
| `scheduled_time` | String |  | Optional timestamp when this course work material is scheduled to be published. |
| `id` | String |  | Classroom-assigned identifier of this course work material, unique per course. Read-only. |
| `state` | String |  | Status of this course work material. If unspecified, the default state is `DRAFT`. |
| `individual_students_options` | String |  | Identifiers of students with access to the course work material. This field is set only if `assigneeMode` is `INDIVIDUAL_STUDENTS`. If the `assigneeMode` is `INDIVIDUAL_STUDENTS`, then only students specified in this field can see the course work material. |
| `materials` | Vec<String> |  | Additional materials. A course work material must have no more than 20 material items. |
| `title` | String |  | Title of this course work material. The title must be a valid UTF-8 string containing between 1 and 3000 characters. |
| `topic_id` | String |  | Identifier for the topic that this course work material is associated with. Must match an existing topic in the course. |
| `update_time` | String |  | Timestamp of the most recent change to this course work material. Read-only. |
| `assignee_mode` | String |  | Assignee mode of the course work material. If unspecified, the default value is `ALL_STUDENTS`. |
| `alternate_link` | String |  | Absolute link to this course work material in the Classroom web UI. This is only populated if `state` is `PUBLISHED`. Read-only. |
| `course_id` | String |  | Identifier of the course. Read-only. |
| `creation_time` | String |  | Timestamp when this course work material was created. Read-only. |
| `creator_user_id` | String |  | Identifier for the user that created the course work material. Read-only. |
| `course_id` | String | ✅ | Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `description` | String | Optional description of this course work material. The text must be a valid UTF-8 string containing no more than 30,000 characters. |
| `scheduled_time` | String | Optional timestamp when this course work material is scheduled to be published. |
| `id` | String | Classroom-assigned identifier of this course work material, unique per course. Read-only. |
| `state` | String | Status of this course work material. If unspecified, the default state is `DRAFT`. |
| `individual_students_options` | String | Identifiers of students with access to the course work material. This field is set only if `assigneeMode` is `INDIVIDUAL_STUDENTS`. If the `assigneeMode` is `INDIVIDUAL_STUDENTS`, then only students specified in this field can see the course work material. |
| `materials` | Vec<String> | Additional materials. A course work material must have no more than 20 material items. |
| `title` | String | Title of this course work material. The title must be a valid UTF-8 string containing between 1 and 3000 characters. |
| `topic_id` | String | Identifier for the topic that this course work material is associated with. Must match an existing topic in the course. |
| `update_time` | String | Timestamp of the most recent change to this course work material. Read-only. |
| `assignee_mode` | String | Assignee mode of the course work material. If unspecified, the default value is `ALL_STUDENTS`. |
| `alternate_link` | String | Absolute link to this course work material in the Classroom web UI. This is only populated if `state` is `PUBLISHED`. Read-only. |
| `course_id` | String | Identifier of the course. Read-only. |
| `creation_time` | String | Timestamp when this course work material was created. Read-only. |
| `creator_user_id` | String | Identifier for the user that created the course work material. Read-only. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create course_work_material
course_work_material = provider.classroom_api.Course_work_material {
    course_id = "value"  # Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
}

# Access course_work_material outputs
course_work_material_id = course_work_material.id
course_work_material_description = course_work_material.description
course_work_material_scheduled_time = course_work_material.scheduled_time
course_work_material_id = course_work_material.id
course_work_material_state = course_work_material.state
course_work_material_individual_students_options = course_work_material.individual_students_options
course_work_material_materials = course_work_material.materials
course_work_material_title = course_work_material.title
course_work_material_topic_id = course_work_material.topic_id
course_work_material_update_time = course_work_material.update_time
course_work_material_assignee_mode = course_work_material.assignee_mode
course_work_material_alternate_link = course_work_material.alternate_link
course_work_material_course_id = course_work_material.course_id
course_work_material_creation_time = course_work_material.creation_time
course_work_material_creator_user_id = course_work_material.creator_user_id
```

---


### Invitation

Creates an invitation. Only one invitation for a user and course may exist at a time. Delete and re-create an invitation to make changes. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to create invitations for this course or for access errors. * `NOT_FOUND` if the course or the user does not exist. * `FAILED_PRECONDITION`: * if the requested user's account is disabled. * if the user already has this role or a role with greater permissions. * for the following request errors: * IneligibleOwner * `ALREADY_EXISTS` if an invitation for the specified user and course already exists.

**Operations**: ✅ Create ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `role` | String |  | Role to invite the user to have. Must not be `COURSE_ROLE_UNSPECIFIED`. |
| `course_id` | String |  | Identifier of the course to invite the user to. |
| `id` | String |  | Identifier assigned by Classroom. Read-only. |
| `user_id` | String |  | Identifier of the invited user. When specified as a parameter of a request, this identifier can be set to one of the following: * the numeric identifier for the user * the email address of the user * the string literal `"me"`, indicating the requesting user |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `role` | String | Role to invite the user to have. Must not be `COURSE_ROLE_UNSPECIFIED`. |
| `course_id` | String | Identifier of the course to invite the user to. |
| `id` | String | Identifier assigned by Classroom. Read-only. |
| `user_id` | String | Identifier of the invited user. When specified as a parameter of a request, this identifier can be set to one of the following: * the numeric identifier for the user * the email address of the user * the string literal `"me"`, indicating the requesting user |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create invitation
invitation = provider.classroom_api.Invitation {
}

# Access invitation outputs
invitation_id = invitation.id
invitation_role = invitation.role
invitation_course_id = invitation.course_id
invitation_id = invitation.id
invitation_user_id = invitation.user_id
```

---


### Post

Gets metadata for Classroom add-ons in the context of a specific post. To maintain the integrity of its own data and permissions model, an add-on should call this to validate query parameters and the requesting user's role whenever the add-on is opened in an [iframe](https://developers.google.com/workspace/classroom/add-ons/get-started/iframes/iframes-overview). This method returns the following error codes: * `PERMISSION_DENIED` for access errors. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if one of the identified resources does not exist.

**Operations**: ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `teacher_context` | String | Add-on context corresponding to the requesting user's role as a teacher. Its presence implies that the requesting user is a teacher in the course. |
| `post_id` | String | Immutable. Deprecated, use `item_id` instead. |
| `supports_student_work` | bool | Optional. Whether the post allows the teacher to see student work and passback grades. |
| `course_id` | String | Immutable. Identifier of the course. |
| `item_id` | String | Immutable. Identifier of the `Announcement`, `CourseWork`, or `CourseWorkMaterial` under which the attachment is attached. |
| `student_context` | String | Add-on context corresponding to the requesting user's role as a student. Its presence implies that the requesting user is a student in the course. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access post outputs
post_id = post.id
post_teacher_context = post.teacher_context
post_post_id = post.post_id
post_supports_student_work = post.supports_student_work
post_course_id = post.course_id
post_item_id = post.item_id
post_student_context = post.student_context
```

---


### Guardian

Returns a specific guardian. This method returns the following error codes: * `PERMISSION_DENIED` if no user that matches the provided `student_id` is visible to the requesting user, if the requesting user is not permitted to view guardian information for the student identified by the `student_id`, if guardians are not enabled for the domain in question, or for other access errors. * `INVALID_ARGUMENT` if a `student_id` is specified, but its format cannot be recognized (it is not an email address, nor a `student_id` from the API, nor the literal string `me`). * `NOT_FOUND` if the requesting user is permitted to view guardians for the requested `student_id`, but no `Guardian` record exists for that student that matches the provided `guardian_id`.

**Operations**: ✅ Read ✅ Delete

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `student_id` | String | Identifier for the student to whom the guardian relationship applies. |
| `invited_email_address` | String | The email address to which the initial guardian invitation was sent. This field is only visible to domain administrators. |
| `guardian_id` | String | Identifier for the guardian. |
| `guardian_profile` | String | User profile for the guardian. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access guardian outputs
guardian_id = guardian.id
guardian_student_id = guardian.student_id
guardian_invited_email_address = guardian.invited_email_address
guardian_guardian_id = guardian.guardian_id
guardian_guardian_profile = guardian.guardian_profile
```

---


### Student_submission

Reclaims a student submission on behalf of the student that owns it. Reclaiming a student submission transfers ownership of attached Drive files to the student and updates the submission state. Only the student that owns the requested student submission may call this method, and only for a student submission that has been turned in. This request must be made by the Developer Console project of the [OAuth client ID](https://support.google.com/cloud/answer/6158849) used to create the corresponding course work item. This method returns the following error codes: * `PERMISSION_DENIED` if the requesting user is not permitted to access the requested course or course work, unsubmit the requested student submission, or for access errors. * `FAILED_PRECONDITION` if the student submission has not been turned in. * `INVALID_ARGUMENT` if the request is malformed. * `NOT_FOUND` if the requested course, course work, or student submission does not exist.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `course_work_id` | String | ✅ | Identifier of the course work. |
| `course_id` | String | ✅ | Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias. |
| `id` | String | ✅ | Identifier of the student submission. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `points_earned` | f64 | Student grade on this attachment. If unset, no grade was set. |
| `post_submission_state` | String | Submission state of add-on attachment's parent post (i.e. assignment). |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create student_submission
student_submission = provider.classroom_api.Student_submission {
    course_work_id = "value"  # Identifier of the course work.
    course_id = "value"  # Identifier of the course. This identifier can be either the Classroom-assigned identifier or an alias.
    id = "value"  # Identifier of the student submission.
}

# Access student_submission outputs
student_submission_id = student_submission.id
student_submission_points_earned = student_submission.points_earned
student_submission_post_submission_state = student_submission.post_submission_state
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple course_work resources
course_work_0 = provider.classroom_api.Course_work {
    course_id = "value-0"
}
course_work_1 = provider.classroom_api.Course_work {
    course_id = "value-1"
}
course_work_2 = provider.classroom_api.Course_work {
    course_id = "value-2"
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    course_work = provider.classroom_api.Course_work {
        course_id = "production-value"
    }
```

---

## Related Documentation

- [GCP Classroom_api Documentation](https://cloud.google.com/classroom_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
