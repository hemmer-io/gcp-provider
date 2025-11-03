# Walletobjects_api Service



**Resources**: 20

---

## Overview

The walletobjects_api service provides access to 20 resource types:

- [Genericclas](#genericclas) [CRU]
- [Eventticketobject](#eventticketobject) [CRU]
- [Transitobject](#transitobject) [CRU]
- [Loyaltyobject](#loyaltyobject) [CRU]
- [Offerobject](#offerobject) [CRU]
- [Genericobject](#genericobject) [CRU]
- [Giftcardobject](#giftcardobject) [CRU]
- [Media](#media) [CR]
- [Flightobject](#flightobject) [CRU]
- [Smarttap](#smarttap) [C]
- [Offerclas](#offerclas) [CRU]
- [Private_content](#private_content) [C]
- [Transitclas](#transitclas) [CRU]
- [Permission](#permission) [RU]
- [Giftcardclas](#giftcardclas) [CRU]
- [Loyaltyclas](#loyaltyclas) [CRU]
- [Issuer](#issuer) [CRU]
- [Eventticketclas](#eventticketclas) [CRU]
- [Jwt](#jwt) [C]
- [Flightclas](#flightclas) [CRU]

---

## Resources


### Genericclas

Inserts a generic class with the given ID and properties.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `callback_options` | String |  | Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback. |
| `image_modules_data` | Vec<String> |  | Image module data. If `imageModulesData` is also defined on the object, both will be displayed. Only one of the image from class and one from object level will be rendered when both set. |
| `links_module_data` | String |  | Links module data. If `linksModuleData` is also defined on the object, both will be displayed. The maximum number of these fields displayed is 10 from class and 10 from object. |
| `text_modules_data` | Vec<String> |  | Text module data. If `textModulesData` is also defined on the object, both will be displayed. The maximum number of these fields displayed is 10 from class and 10 from object. |
| `merchant_locations` | Vec<String> |  | Merchant locations. There is a maximum of ten on the class. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `value_added_module_data` | Vec<String> |  | Optional value added module data. Maximum of ten on the class. For a pass only ten will be displayed, prioritizing those from the object. |
| `id` | String |  | Required. The unique identifier for the class. This ID must be unique across all from an issuer. This value needs to follow the format `issuerID.identifier` where `issuerID` is issued by Google and `identifier` is chosen by you. The unique identifier can only include alphanumeric characters, `.`, `_`, or `-`. |
| `messages` | Vec<String> |  | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `class_template_info` | String |  | Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display. |
| `multiple_devices_and_holders_allowed_status` | String |  | Identifies whether multiple users and devices will save the same object referencing this class. |
| `redemption_issuers` | Vec<String> |  | Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `view_unlock_requirement` | String |  | View Unlock Requirement options for the generic pass. |
| `security_animation` | String |  | Optional information about the security animation. If this is set a security animation will be rendered on pass details. |
| `app_link_data` | String |  | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding object that will be used instead. |
| `enable_smart_tap` | bool |  | Available only to Smart Tap enabled partners. Contact support for additional guidance. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `callback_options` | String | Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback. |
| `image_modules_data` | Vec<String> | Image module data. If `imageModulesData` is also defined on the object, both will be displayed. Only one of the image from class and one from object level will be rendered when both set. |
| `links_module_data` | String | Links module data. If `linksModuleData` is also defined on the object, both will be displayed. The maximum number of these fields displayed is 10 from class and 10 from object. |
| `text_modules_data` | Vec<String> | Text module data. If `textModulesData` is also defined on the object, both will be displayed. The maximum number of these fields displayed is 10 from class and 10 from object. |
| `merchant_locations` | Vec<String> | Merchant locations. There is a maximum of ten on the class. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `value_added_module_data` | Vec<String> | Optional value added module data. Maximum of ten on the class. For a pass only ten will be displayed, prioritizing those from the object. |
| `id` | String | Required. The unique identifier for the class. This ID must be unique across all from an issuer. This value needs to follow the format `issuerID.identifier` where `issuerID` is issued by Google and `identifier` is chosen by you. The unique identifier can only include alphanumeric characters, `.`, `_`, or `-`. |
| `messages` | Vec<String> | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `class_template_info` | String | Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display. |
| `multiple_devices_and_holders_allowed_status` | String | Identifies whether multiple users and devices will save the same object referencing this class. |
| `redemption_issuers` | Vec<String> | Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `view_unlock_requirement` | String | View Unlock Requirement options for the generic pass. |
| `security_animation` | String | Optional information about the security animation. If this is set a security animation will be rendered on pass details. |
| `app_link_data` | String | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding object that will be used instead. |
| `enable_smart_tap` | bool | Available only to Smart Tap enabled partners. Contact support for additional guidance. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create genericclas
genericclas = provider.walletobjects_api.Genericclas {
}

# Access genericclas outputs
genericclas_id = genericclas.id
genericclas_callback_options = genericclas.callback_options
genericclas_image_modules_data = genericclas.image_modules_data
genericclas_links_module_data = genericclas.links_module_data
genericclas_text_modules_data = genericclas.text_modules_data
genericclas_merchant_locations = genericclas.merchant_locations
genericclas_value_added_module_data = genericclas.value_added_module_data
genericclas_id = genericclas.id
genericclas_messages = genericclas.messages
genericclas_class_template_info = genericclas.class_template_info
genericclas_multiple_devices_and_holders_allowed_status = genericclas.multiple_devices_and_holders_allowed_status
genericclas_redemption_issuers = genericclas.redemption_issuers
genericclas_view_unlock_requirement = genericclas.view_unlock_requirement
genericclas_security_animation = genericclas.security_animation
genericclas_app_link_data = genericclas.app_link_data
genericclas_enable_smart_tap = genericclas.enable_smart_tap
```

---


### Eventticketobject

Inserts an event ticket object with the given ID and properties.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `class_id` | String |  | Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. |
| `notify_preference` | String |  | Whether or not field updates to this object should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If set to DO_NOT_NOTIFY or NOTIFICATION_SETTINGS_UNSPECIFIED, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `hero_image` | String |  | Optional banner image displayed on the front of the card. If none is present, hero image of the class, if present, will be displayed. If hero image of the class is also not present, nothing will be displayed. |
| `links_module_data` | String |  | Links module data. If links module data is also defined on the class, both will be displayed. |
| `value_added_module_data` | Vec<String> |  | Optional value added module data. Maximum of ten on the object. |
| `has_users` | bool |  | Indicates if the object has users. This field is set by the platform. |
| `linked_object_ids` | Vec<String> |  | linked_object_ids are a list of other objects such as event ticket, loyalty, offer, generic, giftcard, transit and boarding pass that should be automatically attached to this event ticket object. If a user had saved this event ticket, then these linked_object_ids would be automatically pushed to the user's wallet (unless they turned off the setting to receive such linked passes). Make sure that objects present in linked_object_ids are already inserted - if not, calls would fail. Once linked, the linked objects cannot be unlinked. You cannot link objects belonging to another issuer. There is a limit to the number of objects that can be linked to a single object. After the limit is reached, new linked objects in the call will be ignored silently. Object IDs should follow the format issuer ID. identifier where the former is issued by Google and the latter is chosen by you. |
| `save_restrictions` | String |  | Restrictions on the object that needs to be verified before the user tries to save the pass. Note that this restrictions will only be applied during save time. If the restrictions changed after a user saves the pass, the new restrictions will not be applied to an already saved pass. |
| `app_link_data` | String |  | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding class only object AppLinkData will be displayed. |
| `info_module_data` | String |  | Deprecated. Use textModulesData instead. |
| `rotating_barcode` | String |  | The rotating barcode type and value. |
| `ticket_number` | String |  | The number of the ticket. This can be a unique identifier across all tickets in an issuer's system, all tickets for the event (e.g. XYZ1234512345), or all tickets in the order (1, 2, 3, etc.). |
| `ticket_holder_name` | String |  | Name of the ticket holder, if the ticket is assigned to a person. E.g. "John Doe" or "Jane Doe". |
| `linked_offer_ids` | Vec<String> |  | A list of offer objects linked to this event ticket. The offer objects must already exist. Offer object IDs should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. |
| `hex_background_color` | String |  | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `disable_expiration_notification` | bool |  | Indicates if notifications should explicitly be suppressed. If this field is set to true, regardless of the `messages` field, expiration notifications to the user will be suppressed. By default, this field is set to false. Currently, this can only be set for offers. |
| `text_modules_data` | Vec<String> |  | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `smart_tap_redemption_value` | String |  | The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported. |
| `version` | String |  | Deprecated |
| `class_reference` | String |  | A copy of the inherited fields of the parent class. These fields are retrieved during a GET. |
| `seat_info` | String |  | Seating details for this ticket. |
| `barcode` | String |  | The barcode type and value. |
| `pass_constraints` | String |  | Pass constraints for the object. Includes limiting NFC and screenshot behaviors. |
| `grouping_info` | String |  | Information that controls how passes are grouped together. |
| `reservation_info` | String |  | Reservation details for this ticket. This is expected to be shared amongst all tickets that were purchased in the same order. |
| `messages` | Vec<String> |  | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `has_linked_device` | bool |  | Whether this object is currently linked to a single device. This field is set by the platform when a user saves the object, linking it to their device. Intended for use by select partners. Contact support for additional information. |
| `locations` | Vec<String> |  | Note: This field is currently not supported to trigger geo notifications. |
| `state` | String |  | Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section. |
| `face_value` | String |  | The face value of the ticket, matching what would be printed on a physical version of the ticket. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#eventTicketObject"`. |
| `ticket_type` | String |  | The type of the ticket, such as "Adult" or "Child", or "VIP" or "Standard". |
| `image_modules_data` | Vec<String> |  | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `id` | String |  | Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. The unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `valid_time_interval` | String |  | The time period this object will be `active` and object can be used. An object's state will be changed to `expired` when this time period has passed. |
| `merchant_locations` | Vec<String> |  | Merchant locations. There is a maximum of ten on the object. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `class_id` | String | Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. |
| `notify_preference` | String | Whether or not field updates to this object should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If set to DO_NOT_NOTIFY or NOTIFICATION_SETTINGS_UNSPECIFIED, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `hero_image` | String | Optional banner image displayed on the front of the card. If none is present, hero image of the class, if present, will be displayed. If hero image of the class is also not present, nothing will be displayed. |
| `links_module_data` | String | Links module data. If links module data is also defined on the class, both will be displayed. |
| `value_added_module_data` | Vec<String> | Optional value added module data. Maximum of ten on the object. |
| `has_users` | bool | Indicates if the object has users. This field is set by the platform. |
| `linked_object_ids` | Vec<String> | linked_object_ids are a list of other objects such as event ticket, loyalty, offer, generic, giftcard, transit and boarding pass that should be automatically attached to this event ticket object. If a user had saved this event ticket, then these linked_object_ids would be automatically pushed to the user's wallet (unless they turned off the setting to receive such linked passes). Make sure that objects present in linked_object_ids are already inserted - if not, calls would fail. Once linked, the linked objects cannot be unlinked. You cannot link objects belonging to another issuer. There is a limit to the number of objects that can be linked to a single object. After the limit is reached, new linked objects in the call will be ignored silently. Object IDs should follow the format issuer ID. identifier where the former is issued by Google and the latter is chosen by you. |
| `save_restrictions` | String | Restrictions on the object that needs to be verified before the user tries to save the pass. Note that this restrictions will only be applied during save time. If the restrictions changed after a user saves the pass, the new restrictions will not be applied to an already saved pass. |
| `app_link_data` | String | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding class only object AppLinkData will be displayed. |
| `info_module_data` | String | Deprecated. Use textModulesData instead. |
| `rotating_barcode` | String | The rotating barcode type and value. |
| `ticket_number` | String | The number of the ticket. This can be a unique identifier across all tickets in an issuer's system, all tickets for the event (e.g. XYZ1234512345), or all tickets in the order (1, 2, 3, etc.). |
| `ticket_holder_name` | String | Name of the ticket holder, if the ticket is assigned to a person. E.g. "John Doe" or "Jane Doe". |
| `linked_offer_ids` | Vec<String> | A list of offer objects linked to this event ticket. The offer objects must already exist. Offer object IDs should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. |
| `hex_background_color` | String | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `disable_expiration_notification` | bool | Indicates if notifications should explicitly be suppressed. If this field is set to true, regardless of the `messages` field, expiration notifications to the user will be suppressed. By default, this field is set to false. Currently, this can only be set for offers. |
| `text_modules_data` | Vec<String> | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `smart_tap_redemption_value` | String | The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported. |
| `version` | String | Deprecated |
| `class_reference` | String | A copy of the inherited fields of the parent class. These fields are retrieved during a GET. |
| `seat_info` | String | Seating details for this ticket. |
| `barcode` | String | The barcode type and value. |
| `pass_constraints` | String | Pass constraints for the object. Includes limiting NFC and screenshot behaviors. |
| `grouping_info` | String | Information that controls how passes are grouped together. |
| `reservation_info` | String | Reservation details for this ticket. This is expected to be shared amongst all tickets that were purchased in the same order. |
| `messages` | Vec<String> | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `has_linked_device` | bool | Whether this object is currently linked to a single device. This field is set by the platform when a user saves the object, linking it to their device. Intended for use by select partners. Contact support for additional information. |
| `locations` | Vec<String> | Note: This field is currently not supported to trigger geo notifications. |
| `state` | String | Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section. |
| `face_value` | String | The face value of the ticket, matching what would be printed on a physical version of the ticket. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#eventTicketObject"`. |
| `ticket_type` | String | The type of the ticket, such as "Adult" or "Child", or "VIP" or "Standard". |
| `image_modules_data` | Vec<String> | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `id` | String | Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. The unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `valid_time_interval` | String | The time period this object will be `active` and object can be used. An object's state will be changed to `expired` when this time period has passed. |
| `merchant_locations` | Vec<String> | Merchant locations. There is a maximum of ten on the object. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create eventticketobject
eventticketobject = provider.walletobjects_api.Eventticketobject {
}

# Access eventticketobject outputs
eventticketobject_id = eventticketobject.id
eventticketobject_class_id = eventticketobject.class_id
eventticketobject_notify_preference = eventticketobject.notify_preference
eventticketobject_hero_image = eventticketobject.hero_image
eventticketobject_links_module_data = eventticketobject.links_module_data
eventticketobject_value_added_module_data = eventticketobject.value_added_module_data
eventticketobject_has_users = eventticketobject.has_users
eventticketobject_linked_object_ids = eventticketobject.linked_object_ids
eventticketobject_save_restrictions = eventticketobject.save_restrictions
eventticketobject_app_link_data = eventticketobject.app_link_data
eventticketobject_info_module_data = eventticketobject.info_module_data
eventticketobject_rotating_barcode = eventticketobject.rotating_barcode
eventticketobject_ticket_number = eventticketobject.ticket_number
eventticketobject_ticket_holder_name = eventticketobject.ticket_holder_name
eventticketobject_linked_offer_ids = eventticketobject.linked_offer_ids
eventticketobject_hex_background_color = eventticketobject.hex_background_color
eventticketobject_disable_expiration_notification = eventticketobject.disable_expiration_notification
eventticketobject_text_modules_data = eventticketobject.text_modules_data
eventticketobject_smart_tap_redemption_value = eventticketobject.smart_tap_redemption_value
eventticketobject_version = eventticketobject.version
eventticketobject_class_reference = eventticketobject.class_reference
eventticketobject_seat_info = eventticketobject.seat_info
eventticketobject_barcode = eventticketobject.barcode
eventticketobject_pass_constraints = eventticketobject.pass_constraints
eventticketobject_grouping_info = eventticketobject.grouping_info
eventticketobject_reservation_info = eventticketobject.reservation_info
eventticketobject_messages = eventticketobject.messages
eventticketobject_has_linked_device = eventticketobject.has_linked_device
eventticketobject_locations = eventticketobject.locations
eventticketobject_state = eventticketobject.state
eventticketobject_face_value = eventticketobject.face_value
eventticketobject_kind = eventticketobject.kind
eventticketobject_ticket_type = eventticketobject.ticket_type
eventticketobject_image_modules_data = eventticketobject.image_modules_data
eventticketobject_id = eventticketobject.id
eventticketobject_valid_time_interval = eventticketobject.valid_time_interval
eventticketobject_merchant_locations = eventticketobject.merchant_locations
```

---


### Transitobject

Inserts an transit object with the given ID and properties.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `ticket_number` | String |  | The number of the ticket. This is a unique identifier for the ticket in the transit operator's system. |
| `save_restrictions` | String |  | Restrictions on the object that needs to be verified before the user tries to save the pass. Note that this restrictions will only be applied during save time. If the restrictions changed after a user saves the pass, the new restrictions will not be applied to an already saved pass. |
| `device_context` | String |  | Device context associated with the object. |
| `hero_image` | String |  | Optional banner image displayed on the front of the card. If none is present, hero image of the class, if present, will be displayed. If hero image of the class is also not present, nothing will be displayed. |
| `value_added_module_data` | Vec<String> |  | Optional value added module data. Maximum of ten on the object. |
| `concession_category` | String |  | The concession category for the ticket. |
| `class_id` | String |  | Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. |
| `image_modules_data` | Vec<String> |  | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `valid_time_interval` | String |  | The time period this object will be `active` and object can be used. An object's state will be changed to `expired` when this time period has passed. |
| `activation_status` | String |  | The activation status for the object. Required if the class has `activationOptions` set. |
| `has_linked_device` | bool |  | Whether this object is currently linked to a single device. This field is set by the platform when a user saves the object, linking it to their device. Intended for use by select partners. Contact support for additional information. |
| `id` | String |  | Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. The unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `text_modules_data` | Vec<String> |  | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `grouping_info` | String |  | Information that controls how passes are grouped together. |
| `locations` | Vec<String> |  | Note: This field is currently not supported to trigger geo notifications. |
| `barcode` | String |  | The barcode type and value. |
| `app_link_data` | String |  | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding class only object AppLinkData will be displayed. |
| `passenger_type` | String |  | The number of passengers. |
| `ticket_leg` | String |  | A single ticket leg contains departure and arrival information along with boarding and seating information. If more than one leg is to be specified then use the `ticketLegs` field instead. Both `ticketLeg` and `ticketLegs` may not be set. |
| `class_reference` | String |  | A copy of the inherited fields of the parent class. These fields are retrieved during a GET. |
| `custom_ticket_status` | String |  | A custom status to use for the ticket status value when `ticketStatus` does not provide the right option. Both `ticketStatus` and `customTicketStatus` may not be set. |
| `trip_type` | String |  | Required. The type of trip this transit object represents. Used to determine the pass title and/or which symbol to use between the origin and destination. |
| `info_module_data` | String |  | Deprecated. Use textModulesData instead. |
| `purchase_details` | String |  | Purchase details for this ticket. |
| `merchant_locations` | Vec<String> |  | Merchant locations. There is a maximum of ten on the object. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `links_module_data` | String |  | Links module data. If links module data is also defined on the class, both will be displayed. |
| `messages` | Vec<String> |  | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `pass_constraints` | String |  | Pass constraints for the object. Includes limiting NFC and screenshot behaviors. |
| `rotating_barcode` | String |  | The rotating barcode type and value. |
| `passenger_names` | String |  | The name(s) of the passengers the ticket is assigned to. The above `passengerType` field is meant to give Google context on this field. |
| `smart_tap_redemption_value` | String |  | The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported. |
| `ticket_status` | String |  | The status of the ticket. For states which affect display, use the `state` field instead. |
| `trip_id` | String |  | This id is used to group tickets together if the user has saved multiple tickets for the same trip. |
| `version` | String |  | Deprecated |
| `hex_background_color` | String |  | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `disable_expiration_notification` | bool |  | Indicates if notifications should explicitly be suppressed. If this field is set to true, regardless of the `messages` field, expiration notifications to the user will be suppressed. By default, this field is set to false. Currently, this can only be set for offers. |
| `linked_object_ids` | Vec<String> |  | linked_object_ids are a list of other objects such as event ticket, loyalty, offer, generic, giftcard, transit and boarding pass that should be automatically attached to this transit object. If a user had saved this transit card, then these linked_object_ids would be automatically pushed to the user's wallet (unless they turned off the setting to receive such linked passes). Make sure that objects present in linked_object_ids are already inserted - if not, calls would fail. Once linked, the linked objects cannot be unlinked. You cannot link objects belonging to another issuer. There is a limit to the number of objects that can be linked to a single object. After the limit is reached, new linked objects in the call will be ignored silently. Object IDs should follow the format issuer ID. identifier where the former is issued by Google and the latter is chosen by you. |
| `state` | String |  | Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section. |
| `has_users` | bool |  | Indicates if the object has users. This field is set by the platform. |
| `custom_concession_category` | String |  | A custom concession category to use when `concessionCategory` does not provide the right option. Both `concessionCategory` and `customConcessionCategory` may not be set. |
| `ticket_legs` | Vec<String> |  | Each ticket may contain one or more legs. Each leg contains departure and arrival information along with boarding and seating information. If only one leg is to be specified then use the `ticketLeg` field instead. Both `ticketLeg` and `ticketLegs` may not be set. |
| `ticket_restrictions` | String |  | Information about what kind of restrictions there are on using this ticket. For example, which days of the week it must be used, or which routes are allowed to be taken. |
| `notify_preference` | String |  | Whether or not field updates to this object should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If set to DO_NOT_NOTIFY or NOTIFICATION_SETTINGS_UNSPECIFIED, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `ticket_number` | String | The number of the ticket. This is a unique identifier for the ticket in the transit operator's system. |
| `save_restrictions` | String | Restrictions on the object that needs to be verified before the user tries to save the pass. Note that this restrictions will only be applied during save time. If the restrictions changed after a user saves the pass, the new restrictions will not be applied to an already saved pass. |
| `device_context` | String | Device context associated with the object. |
| `hero_image` | String | Optional banner image displayed on the front of the card. If none is present, hero image of the class, if present, will be displayed. If hero image of the class is also not present, nothing will be displayed. |
| `value_added_module_data` | Vec<String> | Optional value added module data. Maximum of ten on the object. |
| `concession_category` | String | The concession category for the ticket. |
| `class_id` | String | Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. |
| `image_modules_data` | Vec<String> | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `valid_time_interval` | String | The time period this object will be `active` and object can be used. An object's state will be changed to `expired` when this time period has passed. |
| `activation_status` | String | The activation status for the object. Required if the class has `activationOptions` set. |
| `has_linked_device` | bool | Whether this object is currently linked to a single device. This field is set by the platform when a user saves the object, linking it to their device. Intended for use by select partners. Contact support for additional information. |
| `id` | String | Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. The unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `text_modules_data` | Vec<String> | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `grouping_info` | String | Information that controls how passes are grouped together. |
| `locations` | Vec<String> | Note: This field is currently not supported to trigger geo notifications. |
| `barcode` | String | The barcode type and value. |
| `app_link_data` | String | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding class only object AppLinkData will be displayed. |
| `passenger_type` | String | The number of passengers. |
| `ticket_leg` | String | A single ticket leg contains departure and arrival information along with boarding and seating information. If more than one leg is to be specified then use the `ticketLegs` field instead. Both `ticketLeg` and `ticketLegs` may not be set. |
| `class_reference` | String | A copy of the inherited fields of the parent class. These fields are retrieved during a GET. |
| `custom_ticket_status` | String | A custom status to use for the ticket status value when `ticketStatus` does not provide the right option. Both `ticketStatus` and `customTicketStatus` may not be set. |
| `trip_type` | String | Required. The type of trip this transit object represents. Used to determine the pass title and/or which symbol to use between the origin and destination. |
| `info_module_data` | String | Deprecated. Use textModulesData instead. |
| `purchase_details` | String | Purchase details for this ticket. |
| `merchant_locations` | Vec<String> | Merchant locations. There is a maximum of ten on the object. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `links_module_data` | String | Links module data. If links module data is also defined on the class, both will be displayed. |
| `messages` | Vec<String> | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `pass_constraints` | String | Pass constraints for the object. Includes limiting NFC and screenshot behaviors. |
| `rotating_barcode` | String | The rotating barcode type and value. |
| `passenger_names` | String | The name(s) of the passengers the ticket is assigned to. The above `passengerType` field is meant to give Google context on this field. |
| `smart_tap_redemption_value` | String | The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported. |
| `ticket_status` | String | The status of the ticket. For states which affect display, use the `state` field instead. |
| `trip_id` | String | This id is used to group tickets together if the user has saved multiple tickets for the same trip. |
| `version` | String | Deprecated |
| `hex_background_color` | String | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `disable_expiration_notification` | bool | Indicates if notifications should explicitly be suppressed. If this field is set to true, regardless of the `messages` field, expiration notifications to the user will be suppressed. By default, this field is set to false. Currently, this can only be set for offers. |
| `linked_object_ids` | Vec<String> | linked_object_ids are a list of other objects such as event ticket, loyalty, offer, generic, giftcard, transit and boarding pass that should be automatically attached to this transit object. If a user had saved this transit card, then these linked_object_ids would be automatically pushed to the user's wallet (unless they turned off the setting to receive such linked passes). Make sure that objects present in linked_object_ids are already inserted - if not, calls would fail. Once linked, the linked objects cannot be unlinked. You cannot link objects belonging to another issuer. There is a limit to the number of objects that can be linked to a single object. After the limit is reached, new linked objects in the call will be ignored silently. Object IDs should follow the format issuer ID. identifier where the former is issued by Google and the latter is chosen by you. |
| `state` | String | Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section. |
| `has_users` | bool | Indicates if the object has users. This field is set by the platform. |
| `custom_concession_category` | String | A custom concession category to use when `concessionCategory` does not provide the right option. Both `concessionCategory` and `customConcessionCategory` may not be set. |
| `ticket_legs` | Vec<String> | Each ticket may contain one or more legs. Each leg contains departure and arrival information along with boarding and seating information. If only one leg is to be specified then use the `ticketLeg` field instead. Both `ticketLeg` and `ticketLegs` may not be set. |
| `ticket_restrictions` | String | Information about what kind of restrictions there are on using this ticket. For example, which days of the week it must be used, or which routes are allowed to be taken. |
| `notify_preference` | String | Whether or not field updates to this object should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If set to DO_NOT_NOTIFY or NOTIFICATION_SETTINGS_UNSPECIFIED, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create transitobject
transitobject = provider.walletobjects_api.Transitobject {
}

# Access transitobject outputs
transitobject_id = transitobject.id
transitobject_ticket_number = transitobject.ticket_number
transitobject_save_restrictions = transitobject.save_restrictions
transitobject_device_context = transitobject.device_context
transitobject_hero_image = transitobject.hero_image
transitobject_value_added_module_data = transitobject.value_added_module_data
transitobject_concession_category = transitobject.concession_category
transitobject_class_id = transitobject.class_id
transitobject_image_modules_data = transitobject.image_modules_data
transitobject_valid_time_interval = transitobject.valid_time_interval
transitobject_activation_status = transitobject.activation_status
transitobject_has_linked_device = transitobject.has_linked_device
transitobject_id = transitobject.id
transitobject_text_modules_data = transitobject.text_modules_data
transitobject_grouping_info = transitobject.grouping_info
transitobject_locations = transitobject.locations
transitobject_barcode = transitobject.barcode
transitobject_app_link_data = transitobject.app_link_data
transitobject_passenger_type = transitobject.passenger_type
transitobject_ticket_leg = transitobject.ticket_leg
transitobject_class_reference = transitobject.class_reference
transitobject_custom_ticket_status = transitobject.custom_ticket_status
transitobject_trip_type = transitobject.trip_type
transitobject_info_module_data = transitobject.info_module_data
transitobject_purchase_details = transitobject.purchase_details
transitobject_merchant_locations = transitobject.merchant_locations
transitobject_links_module_data = transitobject.links_module_data
transitobject_messages = transitobject.messages
transitobject_pass_constraints = transitobject.pass_constraints
transitobject_rotating_barcode = transitobject.rotating_barcode
transitobject_passenger_names = transitobject.passenger_names
transitobject_smart_tap_redemption_value = transitobject.smart_tap_redemption_value
transitobject_ticket_status = transitobject.ticket_status
transitobject_trip_id = transitobject.trip_id
transitobject_version = transitobject.version
transitobject_hex_background_color = transitobject.hex_background_color
transitobject_disable_expiration_notification = transitobject.disable_expiration_notification
transitobject_linked_object_ids = transitobject.linked_object_ids
transitobject_state = transitobject.state
transitobject_has_users = transitobject.has_users
transitobject_custom_concession_category = transitobject.custom_concession_category
transitobject_ticket_legs = transitobject.ticket_legs
transitobject_ticket_restrictions = transitobject.ticket_restrictions
transitobject_notify_preference = transitobject.notify_preference
```

---


### Loyaltyobject

Inserts an loyalty object with the given ID and properties.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `has_users` | bool |  | Indicates if the object has users. This field is set by the platform. |
| `smart_tap_redemption_value` | String |  | The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported. If this value is not set but the class level fields `enableSmartTap` and `redemptionIssuers` are set up correctly, the `barcode.value` or the `accountId` fields are used as fallback if present. |
| `state` | String |  | Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#loyaltyObject"`. |
| `save_restrictions` | String |  | Restrictions on the object that needs to be verified before the user tries to save the pass. Note that this restrictions will only be applied during save time. If the restrictions changed after a user saves the pass, the new restrictions will not be applied to an already saved pass. |
| `hero_image` | String |  | Optional banner image displayed on the front of the card. If none is present, hero image of the class, if present, will be displayed. If hero image of the class is also not present, nothing will be displayed. |
| `notify_preference` | String |  | Whether or not field updates to this object should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If set to DO_NOT_NOTIFY or NOTIFICATION_SETTINGS_UNSPECIFIED, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `pass_constraints` | String |  | Pass constraints for the object. Includes limiting NFC and screenshot behaviors. |
| `disable_expiration_notification` | bool |  | Indicates if notifications should explicitly be suppressed. If this field is set to true, regardless of the `messages` field, expiration notifications to the user will be suppressed. By default, this field is set to false. Currently, this can only be set for offers. |
| `secondary_loyalty_points` | String |  | The secondary loyalty reward points label, balance, and type. Shown in addition to the primary loyalty points. |
| `valid_time_interval` | String |  | The time period this object will be `active` and object can be used. An object's state will be changed to `expired` when this time period has passed. |
| `rotating_barcode` | String |  | The rotating barcode type and value. |
| `id` | String |  | Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. The unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `merchant_locations` | Vec<String> |  | Merchant locations. There is a maximum of ten on the object. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `loyalty_points` | String |  | The loyalty reward points label, balance, and type. |
| `messages` | Vec<String> |  | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `links_module_data` | String |  | Links module data. If links module data is also defined on the class, both will be displayed. |
| `account_name` | String |  | The loyalty account holder name, such as "John Smith." Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `account_id` | String |  | The loyalty account identifier. Recommended maximum length is 20 characters. |
| `value_added_module_data` | Vec<String> |  | Optional value added module data. Maximum of ten on the object. |
| `has_linked_device` | bool |  | Whether this object is currently linked to a single device. This field is set by the platform when a user saves the object, linking it to their device. Intended for use by select partners. Contact support for additional information. |
| `app_link_data` | String |  | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding class only object AppLinkData will be displayed. |
| `info_module_data` | String |  | Deprecated. Use textModulesData instead. |
| `class_id` | String |  | Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. |
| `grouping_info` | String |  | Information that controls how passes are grouped together. |
| `linked_offer_ids` | Vec<String> |  | A list of offer objects linked to this loyalty card. The offer objects must already exist. Offer object IDs should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. |
| `locations` | Vec<String> |  | Note: This field is currently not supported to trigger geo notifications. |
| `version` | String |  | Deprecated |
| `image_modules_data` | Vec<String> |  | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `linked_object_ids` | Vec<String> |  | linked_object_ids are a list of other objects such as event ticket, loyalty, offer, generic, giftcard, transit and boarding pass that should be automatically attached to this loyalty object. If a user had saved this loyalty card, then these linked_object_ids would be automatically pushed to the user's wallet (unless they turned off the setting to receive such linked passes). Make sure that objects present in linked_object_ids are already inserted - if not, calls would fail. Once linked, the linked objects cannot be unlinked. You cannot link objects belonging to another issuer. There is a limit to the number of objects that can be linked to a single object. After the limit is reached, new linked objects in the call will be ignored silently. Object IDs should follow the format issuer ID. identifier where the former is issued by Google and the latter is chosen by you. |
| `barcode` | String |  | The barcode type and value. |
| `text_modules_data` | Vec<String> |  | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `class_reference` | String |  | A copy of the inherited fields of the parent class. These fields are retrieved during a GET. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `has_users` | bool | Indicates if the object has users. This field is set by the platform. |
| `smart_tap_redemption_value` | String | The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported. If this value is not set but the class level fields `enableSmartTap` and `redemptionIssuers` are set up correctly, the `barcode.value` or the `accountId` fields are used as fallback if present. |
| `state` | String | Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#loyaltyObject"`. |
| `save_restrictions` | String | Restrictions on the object that needs to be verified before the user tries to save the pass. Note that this restrictions will only be applied during save time. If the restrictions changed after a user saves the pass, the new restrictions will not be applied to an already saved pass. |
| `hero_image` | String | Optional banner image displayed on the front of the card. If none is present, hero image of the class, if present, will be displayed. If hero image of the class is also not present, nothing will be displayed. |
| `notify_preference` | String | Whether or not field updates to this object should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If set to DO_NOT_NOTIFY or NOTIFICATION_SETTINGS_UNSPECIFIED, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `pass_constraints` | String | Pass constraints for the object. Includes limiting NFC and screenshot behaviors. |
| `disable_expiration_notification` | bool | Indicates if notifications should explicitly be suppressed. If this field is set to true, regardless of the `messages` field, expiration notifications to the user will be suppressed. By default, this field is set to false. Currently, this can only be set for offers. |
| `secondary_loyalty_points` | String | The secondary loyalty reward points label, balance, and type. Shown in addition to the primary loyalty points. |
| `valid_time_interval` | String | The time period this object will be `active` and object can be used. An object's state will be changed to `expired` when this time period has passed. |
| `rotating_barcode` | String | The rotating barcode type and value. |
| `id` | String | Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. The unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `merchant_locations` | Vec<String> | Merchant locations. There is a maximum of ten on the object. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `loyalty_points` | String | The loyalty reward points label, balance, and type. |
| `messages` | Vec<String> | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `links_module_data` | String | Links module data. If links module data is also defined on the class, both will be displayed. |
| `account_name` | String | The loyalty account holder name, such as "John Smith." Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `account_id` | String | The loyalty account identifier. Recommended maximum length is 20 characters. |
| `value_added_module_data` | Vec<String> | Optional value added module data. Maximum of ten on the object. |
| `has_linked_device` | bool | Whether this object is currently linked to a single device. This field is set by the platform when a user saves the object, linking it to their device. Intended for use by select partners. Contact support for additional information. |
| `app_link_data` | String | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding class only object AppLinkData will be displayed. |
| `info_module_data` | String | Deprecated. Use textModulesData instead. |
| `class_id` | String | Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. |
| `grouping_info` | String | Information that controls how passes are grouped together. |
| `linked_offer_ids` | Vec<String> | A list of offer objects linked to this loyalty card. The offer objects must already exist. Offer object IDs should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. |
| `locations` | Vec<String> | Note: This field is currently not supported to trigger geo notifications. |
| `version` | String | Deprecated |
| `image_modules_data` | Vec<String> | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `linked_object_ids` | Vec<String> | linked_object_ids are a list of other objects such as event ticket, loyalty, offer, generic, giftcard, transit and boarding pass that should be automatically attached to this loyalty object. If a user had saved this loyalty card, then these linked_object_ids would be automatically pushed to the user's wallet (unless they turned off the setting to receive such linked passes). Make sure that objects present in linked_object_ids are already inserted - if not, calls would fail. Once linked, the linked objects cannot be unlinked. You cannot link objects belonging to another issuer. There is a limit to the number of objects that can be linked to a single object. After the limit is reached, new linked objects in the call will be ignored silently. Object IDs should follow the format issuer ID. identifier where the former is issued by Google and the latter is chosen by you. |
| `barcode` | String | The barcode type and value. |
| `text_modules_data` | Vec<String> | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `class_reference` | String | A copy of the inherited fields of the parent class. These fields are retrieved during a GET. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create loyaltyobject
loyaltyobject = provider.walletobjects_api.Loyaltyobject {
}

# Access loyaltyobject outputs
loyaltyobject_id = loyaltyobject.id
loyaltyobject_has_users = loyaltyobject.has_users
loyaltyobject_smart_tap_redemption_value = loyaltyobject.smart_tap_redemption_value
loyaltyobject_state = loyaltyobject.state
loyaltyobject_kind = loyaltyobject.kind
loyaltyobject_save_restrictions = loyaltyobject.save_restrictions
loyaltyobject_hero_image = loyaltyobject.hero_image
loyaltyobject_notify_preference = loyaltyobject.notify_preference
loyaltyobject_pass_constraints = loyaltyobject.pass_constraints
loyaltyobject_disable_expiration_notification = loyaltyobject.disable_expiration_notification
loyaltyobject_secondary_loyalty_points = loyaltyobject.secondary_loyalty_points
loyaltyobject_valid_time_interval = loyaltyobject.valid_time_interval
loyaltyobject_rotating_barcode = loyaltyobject.rotating_barcode
loyaltyobject_id = loyaltyobject.id
loyaltyobject_merchant_locations = loyaltyobject.merchant_locations
loyaltyobject_loyalty_points = loyaltyobject.loyalty_points
loyaltyobject_messages = loyaltyobject.messages
loyaltyobject_links_module_data = loyaltyobject.links_module_data
loyaltyobject_account_name = loyaltyobject.account_name
loyaltyobject_account_id = loyaltyobject.account_id
loyaltyobject_value_added_module_data = loyaltyobject.value_added_module_data
loyaltyobject_has_linked_device = loyaltyobject.has_linked_device
loyaltyobject_app_link_data = loyaltyobject.app_link_data
loyaltyobject_info_module_data = loyaltyobject.info_module_data
loyaltyobject_class_id = loyaltyobject.class_id
loyaltyobject_grouping_info = loyaltyobject.grouping_info
loyaltyobject_linked_offer_ids = loyaltyobject.linked_offer_ids
loyaltyobject_locations = loyaltyobject.locations
loyaltyobject_version = loyaltyobject.version
loyaltyobject_image_modules_data = loyaltyobject.image_modules_data
loyaltyobject_linked_object_ids = loyaltyobject.linked_object_ids
loyaltyobject_barcode = loyaltyobject.barcode
loyaltyobject_text_modules_data = loyaltyobject.text_modules_data
loyaltyobject_class_reference = loyaltyobject.class_reference
```

---


### Offerobject

Inserts an offer object with the given ID and properties.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `version` | String |  | Deprecated |
| `grouping_info` | String |  | Information that controls how passes are grouped together. |
| `rotating_barcode` | String |  | The rotating barcode type and value. |
| `state` | String |  | Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section. |
| `app_link_data` | String |  | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding class only object AppLinkData will be displayed. |
| `id` | String |  | Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. The unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `barcode` | String |  | The barcode type and value. |
| `class_id` | String |  | Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. |
| `image_modules_data` | Vec<String> |  | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `smart_tap_redemption_value` | String |  | The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported. |
| `value_added_module_data` | Vec<String> |  | Optional value added module data. Maximum of ten on the object. |
| `linked_object_ids` | Vec<String> |  | linked_object_ids are a list of other objects such as event ticket, loyalty, offer, generic, giftcard, transit and boarding pass that should be automatically attached to this offer object. If a user had saved this offer, then these linked_object_ids would be automatically pushed to the user's wallet (unless they turned off the setting to receive such linked passes). Make sure that objects present in linked_object_ids are already inserted - if not, calls would fail. Once linked, the linked objects cannot be unlinked. You cannot link objects belonging to another issuer. There is a limit to the number of objects that can be linked to a single object. After the limit is reached, new linked objects in the call will be ignored silently. Object IDs should follow the format issuer ID.identifier where the former is issued by Google and the latter is chosen by you. |
| `merchant_locations` | Vec<String> |  | Merchant locations. There is a maximum of ten on the object. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `messages` | Vec<String> |  | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `text_modules_data` | Vec<String> |  | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `has_linked_device` | bool |  | Whether this object is currently linked to a single device. This field is set by the platform when a user saves the object, linking it to their device. Intended for use by select partners. Contact support for additional information. |
| `links_module_data` | String |  | Links module data. If links module data is also defined on the class, both will be displayed. |
| `notify_preference` | String |  | Whether or not field updates to this object should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If set to DO_NOT_NOTIFY or NOTIFICATION_SETTINGS_UNSPECIFIED, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `valid_time_interval` | String |  | The time period this object will be `active` and object can be used. An object's state will be changed to `expired` when this time period has passed. |
| `class_reference` | String |  | A copy of the inherited fields of the parent class. These fields are retrieved during a GET. |
| `info_module_data` | String |  | Deprecated. Use textModulesData instead. |
| `disable_expiration_notification` | bool |  | Indicates if notifications should explicitly be suppressed. If this field is set to true, regardless of the `messages` field, expiration notifications to the user will be suppressed. By default, this field is set to false. Currently, this can only be set for offers. |
| `has_users` | bool |  | Indicates if the object has users. This field is set by the platform. |
| `pass_constraints` | String |  | Pass constraints for the object. Includes limiting NFC and screenshot behaviors. |
| `hero_image` | String |  | Optional banner image displayed on the front of the card. If none is present, hero image of the class, if present, will be displayed. If hero image of the class is also not present, nothing will be displayed. |
| `locations` | Vec<String> |  | Note: This field is currently not supported to trigger geo notifications. |
| `save_restrictions` | String |  | Restrictions on the object that needs to be verified before the user tries to save the pass. Note that this restrictions will only be applied during save time. If the restrictions changed after a user saves the pass, the new restrictions will not be applied to an already saved pass. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#offerObject"`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `version` | String | Deprecated |
| `grouping_info` | String | Information that controls how passes are grouped together. |
| `rotating_barcode` | String | The rotating barcode type and value. |
| `state` | String | Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section. |
| `app_link_data` | String | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding class only object AppLinkData will be displayed. |
| `id` | String | Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. The unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `barcode` | String | The barcode type and value. |
| `class_id` | String | Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. |
| `image_modules_data` | Vec<String> | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `smart_tap_redemption_value` | String | The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported. |
| `value_added_module_data` | Vec<String> | Optional value added module data. Maximum of ten on the object. |
| `linked_object_ids` | Vec<String> | linked_object_ids are a list of other objects such as event ticket, loyalty, offer, generic, giftcard, transit and boarding pass that should be automatically attached to this offer object. If a user had saved this offer, then these linked_object_ids would be automatically pushed to the user's wallet (unless they turned off the setting to receive such linked passes). Make sure that objects present in linked_object_ids are already inserted - if not, calls would fail. Once linked, the linked objects cannot be unlinked. You cannot link objects belonging to another issuer. There is a limit to the number of objects that can be linked to a single object. After the limit is reached, new linked objects in the call will be ignored silently. Object IDs should follow the format issuer ID.identifier where the former is issued by Google and the latter is chosen by you. |
| `merchant_locations` | Vec<String> | Merchant locations. There is a maximum of ten on the object. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `messages` | Vec<String> | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `text_modules_data` | Vec<String> | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `has_linked_device` | bool | Whether this object is currently linked to a single device. This field is set by the platform when a user saves the object, linking it to their device. Intended for use by select partners. Contact support for additional information. |
| `links_module_data` | String | Links module data. If links module data is also defined on the class, both will be displayed. |
| `notify_preference` | String | Whether or not field updates to this object should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If set to DO_NOT_NOTIFY or NOTIFICATION_SETTINGS_UNSPECIFIED, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `valid_time_interval` | String | The time period this object will be `active` and object can be used. An object's state will be changed to `expired` when this time period has passed. |
| `class_reference` | String | A copy of the inherited fields of the parent class. These fields are retrieved during a GET. |
| `info_module_data` | String | Deprecated. Use textModulesData instead. |
| `disable_expiration_notification` | bool | Indicates if notifications should explicitly be suppressed. If this field is set to true, regardless of the `messages` field, expiration notifications to the user will be suppressed. By default, this field is set to false. Currently, this can only be set for offers. |
| `has_users` | bool | Indicates if the object has users. This field is set by the platform. |
| `pass_constraints` | String | Pass constraints for the object. Includes limiting NFC and screenshot behaviors. |
| `hero_image` | String | Optional banner image displayed on the front of the card. If none is present, hero image of the class, if present, will be displayed. If hero image of the class is also not present, nothing will be displayed. |
| `locations` | Vec<String> | Note: This field is currently not supported to trigger geo notifications. |
| `save_restrictions` | String | Restrictions on the object that needs to be verified before the user tries to save the pass. Note that this restrictions will only be applied during save time. If the restrictions changed after a user saves the pass, the new restrictions will not be applied to an already saved pass. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#offerObject"`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create offerobject
offerobject = provider.walletobjects_api.Offerobject {
}

# Access offerobject outputs
offerobject_id = offerobject.id
offerobject_version = offerobject.version
offerobject_grouping_info = offerobject.grouping_info
offerobject_rotating_barcode = offerobject.rotating_barcode
offerobject_state = offerobject.state
offerobject_app_link_data = offerobject.app_link_data
offerobject_id = offerobject.id
offerobject_barcode = offerobject.barcode
offerobject_class_id = offerobject.class_id
offerobject_image_modules_data = offerobject.image_modules_data
offerobject_smart_tap_redemption_value = offerobject.smart_tap_redemption_value
offerobject_value_added_module_data = offerobject.value_added_module_data
offerobject_linked_object_ids = offerobject.linked_object_ids
offerobject_merchant_locations = offerobject.merchant_locations
offerobject_messages = offerobject.messages
offerobject_text_modules_data = offerobject.text_modules_data
offerobject_has_linked_device = offerobject.has_linked_device
offerobject_links_module_data = offerobject.links_module_data
offerobject_notify_preference = offerobject.notify_preference
offerobject_valid_time_interval = offerobject.valid_time_interval
offerobject_class_reference = offerobject.class_reference
offerobject_info_module_data = offerobject.info_module_data
offerobject_disable_expiration_notification = offerobject.disable_expiration_notification
offerobject_has_users = offerobject.has_users
offerobject_pass_constraints = offerobject.pass_constraints
offerobject_hero_image = offerobject.hero_image
offerobject_locations = offerobject.locations
offerobject_save_restrictions = offerobject.save_restrictions
offerobject_kind = offerobject.kind
```

---


### Genericobject

Inserts a generic object with the given ID and properties.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `messages` | Vec<String> |  | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `rotating_barcode` | String |  | The rotating barcode settings/details. |
| `merchant_locations` | Vec<String> |  | Merchant locations. There is a maximum of ten on the object. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `wide_logo` | String |  | The wide logo of the pass. When provided, this will be used in place of the logo in the top left of the card view. |
| `valid_time_interval` | String |  | The time period this object will be considered valid or usable. When the time period is passed, the object will be considered expired, which will affect the rendering on user's devices. |
| `has_users` | bool |  | Indicates if the object has users. This field is set by the platform. |
| `pass_constraints` | String |  | Pass constraints for the object. Includes limiting NFC and screenshot behaviors. |
| `text_modules_data` | Vec<String> |  | Text module data. If `textModulesData` is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from class and 10 from object. |
| `hero_image` | String |  | Banner image displayed on the front of the card if present. The image will be displayed at 100% width. |
| `image_modules_data` | Vec<String> |  | Image module data. Only one of the image from class and one from object level will be rendered when both set. |
| `card_title` | String |  | Required. The header of the pass. This is usually the Business name such as "XXX Gym", "AAA Insurance". This field is required and appears in the header row at the very top of the pass. |
| `generic_type` | String |  | Specify which `GenericType` the card belongs to. |
| `save_restrictions` | String |  | Restrictions on the object that needs to be verified before the user tries to save the pass. Note that this restrictions will only be applied during save time. If the restrictions changed after a user saves the pass, the new restrictions will not be applied to an already saved pass. |
| `subheader` | String |  | The title label of the pass, such as location where this pass can be used. Appears right above the title in the title row in the pass detail view. |
| `class_id` | String |  | Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format `issuerID.identifier` where `issuerID` is issued by Google and `identifier` is chosen by you. |
| `logo` | String |  | The logo image of the pass. This image is displayed in the card detail view in upper left, and also on the list/thumbnail view. If the logo is not present, the first letter of `cardTitle` would be shown as logo. |
| `smart_tap_redemption_value` | String |  | The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported. |
| `linked_object_ids` | Vec<String> |  | linked_object_ids are a list of other objects such as event ticket, loyalty, offer, generic, giftcard, transit and boarding pass that should be automatically attached to this generic object. If a user had saved this generic card, then these linked_object_ids would be automatically pushed to the user's wallet (unless they turned off the setting to receive such linked passes). Make sure that objects present in linked_object_ids are already inserted - if not, calls would fail. Once linked, the linked objects cannot be unlinked. You cannot link objects belonging to another issuer. There is a limit to the number of objects that can be linked to a single object. After the limit is reached, new linked objects in the call will be ignored silently. Object IDs should follow the format issuer ID. identifier where the former is issued by Google and the latter is chosen by you. |
| `app_link_data` | String |  | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding class only object AppLinkData will be displayed. |
| `hex_background_color` | String |  | The background color for the card. If not set, the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used and if logo is not set, a color would be chosen by Google. |
| `barcode` | String |  | The barcode type and value. If pass does not have a barcode, we can allow the issuer to set Barcode.alternate_text and display just that. |
| `id` | String |  | Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value needs to follow the format `issuerID.identifier` where `issuerID` is issued by Google and `identifier` is chosen by you. The unique identifier can only include alphanumeric characters, `.`, `_`, or `-`. |
| `notifications` | String |  | The notification settings that are enabled for this object. |
| `state` | String |  | The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section. If this is not provided, the object would be considered `ACTIVE`. |
| `value_added_module_data` | Vec<String> |  | Optional value added module data. Maximum of ten on the object. |
| `grouping_info` | String |  | Information that controls how passes are grouped together. |
| `links_module_data` | String |  | Links module data. If `linksModuleData` is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from class and 10 from object. |
| `header` | String |  | Required. The title of the pass, such as "50% off coupon" or "Library card" or "Voucher". This field is required and appears in the title row of the pass detail view. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `messages` | Vec<String> | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `rotating_barcode` | String | The rotating barcode settings/details. |
| `merchant_locations` | Vec<String> | Merchant locations. There is a maximum of ten on the object. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `wide_logo` | String | The wide logo of the pass. When provided, this will be used in place of the logo in the top left of the card view. |
| `valid_time_interval` | String | The time period this object will be considered valid or usable. When the time period is passed, the object will be considered expired, which will affect the rendering on user's devices. |
| `has_users` | bool | Indicates if the object has users. This field is set by the platform. |
| `pass_constraints` | String | Pass constraints for the object. Includes limiting NFC and screenshot behaviors. |
| `text_modules_data` | Vec<String> | Text module data. If `textModulesData` is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from class and 10 from object. |
| `hero_image` | String | Banner image displayed on the front of the card if present. The image will be displayed at 100% width. |
| `image_modules_data` | Vec<String> | Image module data. Only one of the image from class and one from object level will be rendered when both set. |
| `card_title` | String | Required. The header of the pass. This is usually the Business name such as "XXX Gym", "AAA Insurance". This field is required and appears in the header row at the very top of the pass. |
| `generic_type` | String | Specify which `GenericType` the card belongs to. |
| `save_restrictions` | String | Restrictions on the object that needs to be verified before the user tries to save the pass. Note that this restrictions will only be applied during save time. If the restrictions changed after a user saves the pass, the new restrictions will not be applied to an already saved pass. |
| `subheader` | String | The title label of the pass, such as location where this pass can be used. Appears right above the title in the title row in the pass detail view. |
| `class_id` | String | Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format `issuerID.identifier` where `issuerID` is issued by Google and `identifier` is chosen by you. |
| `logo` | String | The logo image of the pass. This image is displayed in the card detail view in upper left, and also on the list/thumbnail view. If the logo is not present, the first letter of `cardTitle` would be shown as logo. |
| `smart_tap_redemption_value` | String | The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported. |
| `linked_object_ids` | Vec<String> | linked_object_ids are a list of other objects such as event ticket, loyalty, offer, generic, giftcard, transit and boarding pass that should be automatically attached to this generic object. If a user had saved this generic card, then these linked_object_ids would be automatically pushed to the user's wallet (unless they turned off the setting to receive such linked passes). Make sure that objects present in linked_object_ids are already inserted - if not, calls would fail. Once linked, the linked objects cannot be unlinked. You cannot link objects belonging to another issuer. There is a limit to the number of objects that can be linked to a single object. After the limit is reached, new linked objects in the call will be ignored silently. Object IDs should follow the format issuer ID. identifier where the former is issued by Google and the latter is chosen by you. |
| `app_link_data` | String | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding class only object AppLinkData will be displayed. |
| `hex_background_color` | String | The background color for the card. If not set, the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used and if logo is not set, a color would be chosen by Google. |
| `barcode` | String | The barcode type and value. If pass does not have a barcode, we can allow the issuer to set Barcode.alternate_text and display just that. |
| `id` | String | Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value needs to follow the format `issuerID.identifier` where `issuerID` is issued by Google and `identifier` is chosen by you. The unique identifier can only include alphanumeric characters, `.`, `_`, or `-`. |
| `notifications` | String | The notification settings that are enabled for this object. |
| `state` | String | The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section. If this is not provided, the object would be considered `ACTIVE`. |
| `value_added_module_data` | Vec<String> | Optional value added module data. Maximum of ten on the object. |
| `grouping_info` | String | Information that controls how passes are grouped together. |
| `links_module_data` | String | Links module data. If `linksModuleData` is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from class and 10 from object. |
| `header` | String | Required. The title of the pass, such as "50% off coupon" or "Library card" or "Voucher". This field is required and appears in the title row of the pass detail view. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create genericobject
genericobject = provider.walletobjects_api.Genericobject {
}

# Access genericobject outputs
genericobject_id = genericobject.id
genericobject_messages = genericobject.messages
genericobject_rotating_barcode = genericobject.rotating_barcode
genericobject_merchant_locations = genericobject.merchant_locations
genericobject_wide_logo = genericobject.wide_logo
genericobject_valid_time_interval = genericobject.valid_time_interval
genericobject_has_users = genericobject.has_users
genericobject_pass_constraints = genericobject.pass_constraints
genericobject_text_modules_data = genericobject.text_modules_data
genericobject_hero_image = genericobject.hero_image
genericobject_image_modules_data = genericobject.image_modules_data
genericobject_card_title = genericobject.card_title
genericobject_generic_type = genericobject.generic_type
genericobject_save_restrictions = genericobject.save_restrictions
genericobject_subheader = genericobject.subheader
genericobject_class_id = genericobject.class_id
genericobject_logo = genericobject.logo
genericobject_smart_tap_redemption_value = genericobject.smart_tap_redemption_value
genericobject_linked_object_ids = genericobject.linked_object_ids
genericobject_app_link_data = genericobject.app_link_data
genericobject_hex_background_color = genericobject.hex_background_color
genericobject_barcode = genericobject.barcode
genericobject_id = genericobject.id
genericobject_notifications = genericobject.notifications
genericobject_state = genericobject.state
genericobject_value_added_module_data = genericobject.value_added_module_data
genericobject_grouping_info = genericobject.grouping_info
genericobject_links_module_data = genericobject.links_module_data
genericobject_header = genericobject.header
```

---


### Giftcardobject

Inserts an gift card object with the given ID and properties.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `messages` | Vec<String> |  | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `linked_object_ids` | Vec<String> |  | linked_object_ids are a list of other objects such as event ticket, loyalty, offer, generic, giftcard, transit and boarding pass that should be automatically attached to this giftcard object. If a user had saved this gift card, then these linked_object_ids would be automatically pushed to the user's wallet (unless they turned off the setting to receive such linked passes). Make sure that objects present in linked_object_ids are already inserted - if not, calls would fail. Once linked, the linked objects cannot be unlinked. You cannot link objects belonging to another issuer. There is a limit to the number of objects that can be linked to a single object. After the limit is reached, new linked objects in the call will be ignored silently. Object IDs should follow the format issuer ID. identifier where the former is issued by Google and the latter is chosen by you. |
| `version` | String |  | Deprecated |
| `class_id` | String |  | Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. |
| `state` | String |  | Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section. |
| `value_added_module_data` | Vec<String> |  | Optional value added module data. Maximum of ten on the object. |
| `text_modules_data` | Vec<String> |  | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `grouping_info` | String |  | Information that controls how passes are grouped together. |
| `pass_constraints` | String |  | Pass constraints for the object. Includes limiting NFC and screenshot behaviors. |
| `app_link_data` | String |  | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding class only object AppLinkData will be displayed. |
| `barcode` | String |  | The barcode type and value. |
| `links_module_data` | String |  | Links module data. If links module data is also defined on the class, both will be displayed. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#giftCardObject"`. |
| `locations` | Vec<String> |  | Note: This field is currently not supported to trigger geo notifications. |
| `card_number` | String |  | Required. The card's number. |
| `merchant_locations` | Vec<String> |  | Merchant locations. There is a maximum of ten on the object. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `pin` | String |  | The card's PIN. |
| `smart_tap_redemption_value` | String |  | The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported. |
| `has_linked_device` | bool |  | Whether this object is currently linked to a single device. This field is set by the platform when a user saves the object, linking it to their device. Intended for use by select partners. Contact support for additional information. |
| `balance` | String |  | The card's monetary balance. |
| `notify_preference` | String |  | Whether or not field updates to this object should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If set to DO_NOT_NOTIFY or NOTIFICATION_SETTINGS_UNSPECIFIED, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `has_users` | bool |  | Indicates if the object has users. This field is set by the platform. |
| `image_modules_data` | Vec<String> |  | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `info_module_data` | String |  | Deprecated. Use textModulesData instead. |
| `rotating_barcode` | String |  | The rotating barcode type and value. |
| `hero_image` | String |  | Optional banner image displayed on the front of the card. If none is present, hero image of the class, if present, will be displayed. If hero image of the class is also not present, nothing will be displayed. |
| `balance_update_time` | String |  | The date and time when the balance was last updated. Offset is required. If balance is updated and this property is not provided, system will default to the current time. |
| `event_number` | String |  | The card's event number, an optional field used by some gift cards. |
| `id` | String |  | Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. The unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `valid_time_interval` | String |  | The time period this object will be `active` and object can be used. An object's state will be changed to `expired` when this time period has passed. |
| `save_restrictions` | String |  | Restrictions on the object that needs to be verified before the user tries to save the pass. Note that this restrictions will only be applied during save time. If the restrictions changed after a user saves the pass, the new restrictions will not be applied to an already saved pass. |
| `class_reference` | String |  | A copy of the inherited fields of the parent class. These fields are retrieved during a GET. |
| `disable_expiration_notification` | bool |  | Indicates if notifications should explicitly be suppressed. If this field is set to true, regardless of the `messages` field, expiration notifications to the user will be suppressed. By default, this field is set to false. Currently, this can only be set for offers. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `messages` | Vec<String> | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `linked_object_ids` | Vec<String> | linked_object_ids are a list of other objects such as event ticket, loyalty, offer, generic, giftcard, transit and boarding pass that should be automatically attached to this giftcard object. If a user had saved this gift card, then these linked_object_ids would be automatically pushed to the user's wallet (unless they turned off the setting to receive such linked passes). Make sure that objects present in linked_object_ids are already inserted - if not, calls would fail. Once linked, the linked objects cannot be unlinked. You cannot link objects belonging to another issuer. There is a limit to the number of objects that can be linked to a single object. After the limit is reached, new linked objects in the call will be ignored silently. Object IDs should follow the format issuer ID. identifier where the former is issued by Google and the latter is chosen by you. |
| `version` | String | Deprecated |
| `class_id` | String | Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. |
| `state` | String | Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section. |
| `value_added_module_data` | Vec<String> | Optional value added module data. Maximum of ten on the object. |
| `text_modules_data` | Vec<String> | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `grouping_info` | String | Information that controls how passes are grouped together. |
| `pass_constraints` | String | Pass constraints for the object. Includes limiting NFC and screenshot behaviors. |
| `app_link_data` | String | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding class only object AppLinkData will be displayed. |
| `barcode` | String | The barcode type and value. |
| `links_module_data` | String | Links module data. If links module data is also defined on the class, both will be displayed. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#giftCardObject"`. |
| `locations` | Vec<String> | Note: This field is currently not supported to trigger geo notifications. |
| `card_number` | String | Required. The card's number. |
| `merchant_locations` | Vec<String> | Merchant locations. There is a maximum of ten on the object. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `pin` | String | The card's PIN. |
| `smart_tap_redemption_value` | String | The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported. |
| `has_linked_device` | bool | Whether this object is currently linked to a single device. This field is set by the platform when a user saves the object, linking it to their device. Intended for use by select partners. Contact support for additional information. |
| `balance` | String | The card's monetary balance. |
| `notify_preference` | String | Whether or not field updates to this object should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If set to DO_NOT_NOTIFY or NOTIFICATION_SETTINGS_UNSPECIFIED, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `has_users` | bool | Indicates if the object has users. This field is set by the platform. |
| `image_modules_data` | Vec<String> | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `info_module_data` | String | Deprecated. Use textModulesData instead. |
| `rotating_barcode` | String | The rotating barcode type and value. |
| `hero_image` | String | Optional banner image displayed on the front of the card. If none is present, hero image of the class, if present, will be displayed. If hero image of the class is also not present, nothing will be displayed. |
| `balance_update_time` | String | The date and time when the balance was last updated. Offset is required. If balance is updated and this property is not provided, system will default to the current time. |
| `event_number` | String | The card's event number, an optional field used by some gift cards. |
| `id` | String | Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. The unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `valid_time_interval` | String | The time period this object will be `active` and object can be used. An object's state will be changed to `expired` when this time period has passed. |
| `save_restrictions` | String | Restrictions on the object that needs to be verified before the user tries to save the pass. Note that this restrictions will only be applied during save time. If the restrictions changed after a user saves the pass, the new restrictions will not be applied to an already saved pass. |
| `class_reference` | String | A copy of the inherited fields of the parent class. These fields are retrieved during a GET. |
| `disable_expiration_notification` | bool | Indicates if notifications should explicitly be suppressed. If this field is set to true, regardless of the `messages` field, expiration notifications to the user will be suppressed. By default, this field is set to false. Currently, this can only be set for offers. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create giftcardobject
giftcardobject = provider.walletobjects_api.Giftcardobject {
}

# Access giftcardobject outputs
giftcardobject_id = giftcardobject.id
giftcardobject_messages = giftcardobject.messages
giftcardobject_linked_object_ids = giftcardobject.linked_object_ids
giftcardobject_version = giftcardobject.version
giftcardobject_class_id = giftcardobject.class_id
giftcardobject_state = giftcardobject.state
giftcardobject_value_added_module_data = giftcardobject.value_added_module_data
giftcardobject_text_modules_data = giftcardobject.text_modules_data
giftcardobject_grouping_info = giftcardobject.grouping_info
giftcardobject_pass_constraints = giftcardobject.pass_constraints
giftcardobject_app_link_data = giftcardobject.app_link_data
giftcardobject_barcode = giftcardobject.barcode
giftcardobject_links_module_data = giftcardobject.links_module_data
giftcardobject_kind = giftcardobject.kind
giftcardobject_locations = giftcardobject.locations
giftcardobject_card_number = giftcardobject.card_number
giftcardobject_merchant_locations = giftcardobject.merchant_locations
giftcardobject_pin = giftcardobject.pin
giftcardobject_smart_tap_redemption_value = giftcardobject.smart_tap_redemption_value
giftcardobject_has_linked_device = giftcardobject.has_linked_device
giftcardobject_balance = giftcardobject.balance
giftcardobject_notify_preference = giftcardobject.notify_preference
giftcardobject_has_users = giftcardobject.has_users
giftcardobject_image_modules_data = giftcardobject.image_modules_data
giftcardobject_info_module_data = giftcardobject.info_module_data
giftcardobject_rotating_barcode = giftcardobject.rotating_barcode
giftcardobject_hero_image = giftcardobject.hero_image
giftcardobject_balance_update_time = giftcardobject.balance_update_time
giftcardobject_event_number = giftcardobject.event_number
giftcardobject_id = giftcardobject.id
giftcardobject_valid_time_interval = giftcardobject.valid_time_interval
giftcardobject_save_restrictions = giftcardobject.save_restrictions
giftcardobject_class_reference = giftcardobject.class_reference
giftcardobject_disable_expiration_notification = giftcardobject.disable_expiration_notification
```

---


### Media

Uploads rotating barcode values for the transit object referenced by the given object ID. Note the max upload size is specified in google3/production/config/cdd/apps-upload/customers/payments-consumer-passes/config.gcl and enforced by Scotty.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `blob` | String |  | A reference to the rotating barcode values payload that was uploaded. |
| `media_request_info` | String |  | Extra information about the uploaded media. |
| `resource_id` | String | ✅ | The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `content_type_info` | String | Extended content type information provided for Scotty uploads. |
| `diff_upload_response` | String | Set if reference_type is DIFF_UPLOAD_RESPONSE. |
| `token` | String | A unique fingerprint/version id for the media data |
| `is_potential_retry` | bool | |is_potential_retry| is set false only when Scotty is certain that it has not sent the request before. When a client resumes an upload, this field must be set true in agent calls, because Scotty cannot be certain that it has never sent the request before due to potential failure in the session state persistence. |
| `hash_verified` | bool | For Scotty uploads only. If a user sends a hash code and the backend has requested that Scotty verify the upload against the client hash, Scotty will perform the check on behalf of the backend and will reject it if the hashes don't match. This is set to true if Scotty performed this verification. |
| `hash` | String | Deprecated, use one of explicit hash type fields instead. These two hash related fields will only be populated on Scotty based media uploads and will contain the content of the hash group in the NotificationRequest: http://cs/#google3/blobstore2/api/scotty/service/proto/upload_listener.proto&q=class:Hash Hex encoded hash value of the uploaded media. |
| `filename` | String | Original file name |
| `sha1_hash` | String | Scotty-provided SHA1 hash for an upload. |
| `length` | String | Size of the data, in bytes |
| `composite_media` | Vec<String> | A composite media composed of one or more media objects, set if reference_type is COMPOSITE_MEDIA. The media length field must be set to the sum of the lengths of all composite media objects. Note: All composite media must have length specified. |
| `crc32c_hash` | i64 | For Scotty Uploads: Scotty-provided hashes for uploads For Scotty Downloads: (WARNING: DO NOT USE WITHOUT PERMISSION FROM THE SCOTTY TEAM.) A Hash provided by the agent to be used to verify the data being downloaded. Currently only supported for inline payloads. Further, only crc32c_hash is currently supported. |
| `inline` | String | Media data, set if reference_type is INLINE |
| `object_id` | String | Reference to a TI Blob, set if reference_type is BIGSTORE_REF. |
| `diff_upload_request` | String | Set if reference_type is DIFF_UPLOAD_REQUEST. |
| `diff_checksums_response` | String | Set if reference_type is DIFF_CHECKSUMS_RESPONSE. |
| `cosmo_binary_reference` | String | A binary data reference for a media download. Serves as a technology-agnostic binary reference in some Google infrastructure. This value is a serialized storage_cosmo.BinaryReference proto. Storing it as bytes is a hack to get around the fact that the cosmo proto (as well as others it includes) doesn't support JavaScript. This prevents us from including the actual type of this field. |
| `content_type` | String | MIME type of the data |
| `diff_download_response` | String | Set if reference_type is DIFF_DOWNLOAD_RESPONSE. |
| `reference_type` | String | Describes what the field reference contains. |
| `diff_version_response` | String | Set if reference_type is DIFF_VERSION_RESPONSE. |
| `blob_ref` | String | Blobstore v1 reference, set if reference_type is BLOBSTORE_REF This should be the byte representation of a blobstore.BlobRef. Since Blobstore is deprecating v1, use blobstore2_info instead. For now, any v2 blob will also be represented in this field as v1 BlobRef. |
| `sha256_hash` | String | Scotty-provided SHA256 hash for an upload. |
| `media_id` | String | Media id to forward to the operation GetMedia. Can be set if reference_type is GET_MEDIA. |
| `algorithm` | String | Deprecated, use one of explicit hash type fields instead. Algorithm used for calculating the hash. As of 2011/01/21, "MD5" is the only possible value for this field. New values may be added at any time. |
| `path` | String | Path to the data, set if reference_type is PATH |
| `bigstore_object_ref` | String | Use object_id instead. |
| `blobstore2_info` | String | Blobstore v2 info, set if reference_type is BLOBSTORE_REF and it refers to a v2 blob. |
| `download_parameters` | String | Parameters for a media download. |
| `md5_hash` | String | Scotty-provided MD5 hash for an upload. |
| `timestamp` | String | Time at which the media data was last updated, in milliseconds since UNIX epoch |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create media
media = provider.walletobjects_api.Media {
    resource_id = "value"  # The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'.
}

# Access media outputs
media_id = media.id
media_content_type_info = media.content_type_info
media_diff_upload_response = media.diff_upload_response
media_token = media.token
media_is_potential_retry = media.is_potential_retry
media_hash_verified = media.hash_verified
media_hash = media.hash
media_filename = media.filename
media_sha1_hash = media.sha1_hash
media_length = media.length
media_composite_media = media.composite_media
media_crc32c_hash = media.crc32c_hash
media_inline = media.inline
media_object_id = media.object_id
media_diff_upload_request = media.diff_upload_request
media_diff_checksums_response = media.diff_checksums_response
media_cosmo_binary_reference = media.cosmo_binary_reference
media_content_type = media.content_type
media_diff_download_response = media.diff_download_response
media_reference_type = media.reference_type
media_diff_version_response = media.diff_version_response
media_blob_ref = media.blob_ref
media_sha256_hash = media.sha256_hash
media_media_id = media.media_id
media_algorithm = media.algorithm
media_path = media.path
media_bigstore_object_ref = media.bigstore_object_ref
media_blobstore2_info = media.blobstore2_info
media_download_parameters = media.download_parameters
media_md5_hash = media.md5_hash
media_timestamp = media.timestamp
```

---


### Flightobject

Inserts an flight object with the given ID and properties.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `class_id` | String |  | Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. |
| `info_module_data` | String |  | Deprecated. Use textModulesData instead. |
| `boarding_and_seating_info` | String |  | Passenger specific information about boarding and seating. |
| `has_linked_device` | bool |  | Whether this object is currently linked to a single device. This field is set by the platform when a user saves the object, linking it to their device. Intended for use by select partners. Contact support for additional information. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#flightObject"`. |
| `version` | String |  | Deprecated |
| `state` | String |  | Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section. |
| `has_users` | bool |  | Indicates if the object has users. This field is set by the platform. |
| `pass_constraints` | String |  | Pass constraints for the object. Includes limiting NFC and screenshot behaviors. |
| `messages` | Vec<String> |  | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `notify_preference` | String |  | Whether or not field updates to this object should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If set to DO_NOT_NOTIFY or NOTIFICATION_SETTINGS_UNSPECIFIED, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `passenger_name` | String |  | Required. Passenger name as it would appear on the boarding pass. eg: "Dave M Gahan" or "Gahan/Dave" or "GAHAN/DAVEM" |
| `class_reference` | String |  | A copy of the inherited fields of the parent class. These fields are retrieved during a GET. |
| `rotating_barcode` | String |  | The rotating barcode type and value. |
| `hex_background_color` | String |  | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `barcode` | String |  | The barcode type and value. |
| `security_program_logo` | String |  | An image for the security program that applies to the passenger. |
| `reservation_info` | String |  | Required. Information about flight reservation. |
| `disable_expiration_notification` | bool |  | Indicates if notifications should explicitly be suppressed. If this field is set to true, regardless of the `messages` field, expiration notifications to the user will be suppressed. By default, this field is set to false. Currently, this can only be set for Flights. |
| `image_modules_data` | Vec<String> |  | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `locations` | Vec<String> |  | Note: This field is currently not supported to trigger geo notifications. |
| `id` | String |  | Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. The unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `links_module_data` | String |  | Links module data. If links module data is also defined on the class, both will be displayed. |
| `hero_image` | String |  | Optional banner image displayed on the front of the card. If none is present, hero image of the class, if present, will be displayed. If hero image of the class is also not present, nothing will be displayed. |
| `valid_time_interval` | String |  | The time period this object will be `active` and object can be used. An object's state will be changed to `expired` when this time period has passed. |
| `app_link_data` | String |  | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding class only object AppLinkData will be displayed. |
| `merchant_locations` | Vec<String> |  | Merchant locations. There is a maximum of ten on the object. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `value_added_module_data` | Vec<String> |  | Optional value added module data. Maximum of ten on the object. |
| `linked_object_ids` | Vec<String> |  | linked_object_ids are a list of other objects such as event ticket, loyalty, offer, generic, giftcard, transit and boarding pass that should be automatically attached to this flight object. If a user had saved this boarding pass, then these linked_object_ids would be automatically pushed to the user's wallet (unless they turned off the setting to receive such linked passes). Make sure that objects present in linked_object_ids are already inserted - if not, calls would fail. Once linked, the linked objects cannot be unlinked. You cannot link objects belonging to another issuer. There is a limit to the number of objects that can be linked to a single object. After the limit is reached, new linked objects in the call will be ignored silently. Object IDs should follow the format issuer ID. identifier where the former is issued by Google and the latter is chosen by you. |
| `grouping_info` | String |  | Information that controls how passes are grouped together. |
| `save_restrictions` | String |  | Restrictions on the object that needs to be verified before the user tries to save the pass. Note that this restrictions will only be applied during save time. If the restrictions changed after a user saves the pass, the new restrictions will not be applied to an already saved pass. |
| `smart_tap_redemption_value` | String |  | The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported. |
| `text_modules_data` | Vec<String> |  | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `class_id` | String | Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. |
| `info_module_data` | String | Deprecated. Use textModulesData instead. |
| `boarding_and_seating_info` | String | Passenger specific information about boarding and seating. |
| `has_linked_device` | bool | Whether this object is currently linked to a single device. This field is set by the platform when a user saves the object, linking it to their device. Intended for use by select partners. Contact support for additional information. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#flightObject"`. |
| `version` | String | Deprecated |
| `state` | String | Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section. |
| `has_users` | bool | Indicates if the object has users. This field is set by the platform. |
| `pass_constraints` | String | Pass constraints for the object. Includes limiting NFC and screenshot behaviors. |
| `messages` | Vec<String> | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `notify_preference` | String | Whether or not field updates to this object should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If set to DO_NOT_NOTIFY or NOTIFICATION_SETTINGS_UNSPECIFIED, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `passenger_name` | String | Required. Passenger name as it would appear on the boarding pass. eg: "Dave M Gahan" or "Gahan/Dave" or "GAHAN/DAVEM" |
| `class_reference` | String | A copy of the inherited fields of the parent class. These fields are retrieved during a GET. |
| `rotating_barcode` | String | The rotating barcode type and value. |
| `hex_background_color` | String | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `barcode` | String | The barcode type and value. |
| `security_program_logo` | String | An image for the security program that applies to the passenger. |
| `reservation_info` | String | Required. Information about flight reservation. |
| `disable_expiration_notification` | bool | Indicates if notifications should explicitly be suppressed. If this field is set to true, regardless of the `messages` field, expiration notifications to the user will be suppressed. By default, this field is set to false. Currently, this can only be set for Flights. |
| `image_modules_data` | Vec<String> | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `locations` | Vec<String> | Note: This field is currently not supported to trigger geo notifications. |
| `id` | String | Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. The unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `links_module_data` | String | Links module data. If links module data is also defined on the class, both will be displayed. |
| `hero_image` | String | Optional banner image displayed on the front of the card. If none is present, hero image of the class, if present, will be displayed. If hero image of the class is also not present, nothing will be displayed. |
| `valid_time_interval` | String | The time period this object will be `active` and object can be used. An object's state will be changed to `expired` when this time period has passed. |
| `app_link_data` | String | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding class only object AppLinkData will be displayed. |
| `merchant_locations` | Vec<String> | Merchant locations. There is a maximum of ten on the object. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `value_added_module_data` | Vec<String> | Optional value added module data. Maximum of ten on the object. |
| `linked_object_ids` | Vec<String> | linked_object_ids are a list of other objects such as event ticket, loyalty, offer, generic, giftcard, transit and boarding pass that should be automatically attached to this flight object. If a user had saved this boarding pass, then these linked_object_ids would be automatically pushed to the user's wallet (unless they turned off the setting to receive such linked passes). Make sure that objects present in linked_object_ids are already inserted - if not, calls would fail. Once linked, the linked objects cannot be unlinked. You cannot link objects belonging to another issuer. There is a limit to the number of objects that can be linked to a single object. After the limit is reached, new linked objects in the call will be ignored silently. Object IDs should follow the format issuer ID. identifier where the former is issued by Google and the latter is chosen by you. |
| `grouping_info` | String | Information that controls how passes are grouped together. |
| `save_restrictions` | String | Restrictions on the object that needs to be verified before the user tries to save the pass. Note that this restrictions will only be applied during save time. If the restrictions changed after a user saves the pass, the new restrictions will not be applied to an already saved pass. |
| `smart_tap_redemption_value` | String | The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported. |
| `text_modules_data` | Vec<String> | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create flightobject
flightobject = provider.walletobjects_api.Flightobject {
}

# Access flightobject outputs
flightobject_id = flightobject.id
flightobject_class_id = flightobject.class_id
flightobject_info_module_data = flightobject.info_module_data
flightobject_boarding_and_seating_info = flightobject.boarding_and_seating_info
flightobject_has_linked_device = flightobject.has_linked_device
flightobject_kind = flightobject.kind
flightobject_version = flightobject.version
flightobject_state = flightobject.state
flightobject_has_users = flightobject.has_users
flightobject_pass_constraints = flightobject.pass_constraints
flightobject_messages = flightobject.messages
flightobject_notify_preference = flightobject.notify_preference
flightobject_passenger_name = flightobject.passenger_name
flightobject_class_reference = flightobject.class_reference
flightobject_rotating_barcode = flightobject.rotating_barcode
flightobject_hex_background_color = flightobject.hex_background_color
flightobject_barcode = flightobject.barcode
flightobject_security_program_logo = flightobject.security_program_logo
flightobject_reservation_info = flightobject.reservation_info
flightobject_disable_expiration_notification = flightobject.disable_expiration_notification
flightobject_image_modules_data = flightobject.image_modules_data
flightobject_locations = flightobject.locations
flightobject_id = flightobject.id
flightobject_links_module_data = flightobject.links_module_data
flightobject_hero_image = flightobject.hero_image
flightobject_valid_time_interval = flightobject.valid_time_interval
flightobject_app_link_data = flightobject.app_link_data
flightobject_merchant_locations = flightobject.merchant_locations
flightobject_value_added_module_data = flightobject.value_added_module_data
flightobject_linked_object_ids = flightobject.linked_object_ids
flightobject_grouping_info = flightobject.grouping_info
flightobject_save_restrictions = flightobject.save_restrictions
flightobject_smart_tap_redemption_value = flightobject.smart_tap_redemption_value
flightobject_text_modules_data = flightobject.text_modules_data
```

---


### Smarttap

Inserts the smart tap.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#smartTap"`. |
| `id` | String |  | The unique identifier for a smart tap. This value should follow the format issuer ID.identifier where the former is issued by Google and latter is the Smart Tap id. The Smart Tap id is a Base64 encoded string which represents the id which was generated by the Google Pay app. |
| `infos` | Vec<String> |  | Communication from merchant to user. |
| `merchant_id` | String |  | Smart Tap merchant ID of who engaged in the Smart Tap interaction. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create smarttap
smarttap = provider.walletobjects_api.Smarttap {
}

```

---


### Offerclas

Inserts an offer class with the given ID and properties.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `localized_title` | String |  | Translated strings for the title. Recommended maximum length is 60 characters to ensure full string is displayed on smaller screens. |
| `localized_provider` | String |  | Translated strings for the provider. Recommended maximum length is 12 characters to ensure full string is displayed on smaller screens. |
| `hex_background_color` | String |  | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `app_link_data` | String |  | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding object that will be used instead. |
| `image_modules_data` | Vec<String> |  | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `redemption_channel` | String |  | Required. The redemption channels applicable to this offer. |
| `view_unlock_requirement` | String |  | View Unlock Requirement options for the offer. |
| `localized_fine_print` | String |  | Translated strings for the fine_print. |
| `notify_preference` | String |  | Whether or not field updates to this class should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If not specified, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `callback_options` | String |  | Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback. |
| `homepage_uri` | String |  | The URI of your application's home page. Populating the URI in this field results in the exact same behavior as populating an URI in linksModuleData (when an object is rendered, a link to the homepage is shown in what would usually be thought of as the linksModuleData section of the object). |
| `locations` | Vec<String> |  | Note: This field is currently not supported to trigger geo notifications. |
| `class_template_info` | String |  | Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display. |
| `title_image` | String |  | The title image of the offer. This image is displayed in both the details and list views of the app. |
| `issuer_name` | String |  | Required. The issuer name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `value_added_module_data` | Vec<String> |  | Optional value added module data. Maximum of ten on the class. For a pass only ten will be displayed, prioritizing those from the object. |
| `localized_details` | String |  | Translated strings for the details. |
| `country_code` | String |  | Country code used to display the card's country (when the user is not in that country), as well as to display localized content when content is not available in the user's locale. |
| `title` | String |  | Required. The title of the offer, such as "20% off any t-shirt." Recommended maximum length is 60 characters to ensure full string is displayed on smaller screens. |
| `id` | String |  | Required. The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `merchant_locations` | Vec<String> |  | Merchant locations. There is a maximum of ten on the class. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `enable_smart_tap` | bool |  | Identifies whether this class supports Smart Tap. The `redemptionIssuers` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `localized_issuer_name` | String |  | Translated strings for the issuer_name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `provider` | String |  | Required. The offer provider (either the aggregator name or merchant name). Recommended maximum length is 12 characters to ensure full string is displayed on smaller screens. |
| `fine_print` | String |  | The fine print or terms of the offer, such as "20% off any t-shirt at Adam's Apparel." |
| `help_uri` | String |  | The help link for the offer, such as `http://myownpersonaldomain.com/help` |
| `details` | String |  | The details of the offer. |
| `review_status` | String |  | Required. The status of the class. This field can be set to `draft` or The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`. |
| `info_module_data` | String |  | Deprecated. Use textModulesData instead. |
| `hero_image` | String |  | Optional banner image displayed on the front of the card. If none is present, nothing will be displayed. The image will display at 100% width. |
| `links_module_data` | String |  | Links module data. If links module data is also defined on the object, both will be displayed. |
| `review` | String |  | The review comments set by the platform when a class is marked `approved` or `rejected`. |
| `messages` | Vec<String> |  | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `text_modules_data` | Vec<String> |  | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#offerClass"`. |
| `localized_short_title` | String |  | Translated strings for the short title. Recommended maximum length is 20 characters. |
| `multiple_devices_and_holders_allowed_status` | String |  | Identifies whether multiple users and devices will save the same object referencing this class. |
| `redemption_issuers` | Vec<String> |  | Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `allow_multiple_users_per_object` | bool |  | Deprecated. Use `multipleDevicesAndHoldersAllowedStatus` instead. |
| `version` | String |  | Deprecated |
| `wide_title_image` | String |  | The wide title image of the offer. When provided, this will be used in place of the title image in the top left of the card view. |
| `word_mark` | String |  | Deprecated. |
| `security_animation` | String |  | Optional information about the security animation. If this is set a security animation will be rendered on pass details. |
| `short_title` | String |  | A shortened version of the title of the offer, such as "20% off," shown to users as a quick reference to the offer contents. Recommended maximum length is 20 characters. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `localized_title` | String | Translated strings for the title. Recommended maximum length is 60 characters to ensure full string is displayed on smaller screens. |
| `localized_provider` | String | Translated strings for the provider. Recommended maximum length is 12 characters to ensure full string is displayed on smaller screens. |
| `hex_background_color` | String | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `app_link_data` | String | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding object that will be used instead. |
| `image_modules_data` | Vec<String> | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `redemption_channel` | String | Required. The redemption channels applicable to this offer. |
| `view_unlock_requirement` | String | View Unlock Requirement options for the offer. |
| `localized_fine_print` | String | Translated strings for the fine_print. |
| `notify_preference` | String | Whether or not field updates to this class should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If not specified, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `callback_options` | String | Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback. |
| `homepage_uri` | String | The URI of your application's home page. Populating the URI in this field results in the exact same behavior as populating an URI in linksModuleData (when an object is rendered, a link to the homepage is shown in what would usually be thought of as the linksModuleData section of the object). |
| `locations` | Vec<String> | Note: This field is currently not supported to trigger geo notifications. |
| `class_template_info` | String | Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display. |
| `title_image` | String | The title image of the offer. This image is displayed in both the details and list views of the app. |
| `issuer_name` | String | Required. The issuer name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `value_added_module_data` | Vec<String> | Optional value added module data. Maximum of ten on the class. For a pass only ten will be displayed, prioritizing those from the object. |
| `localized_details` | String | Translated strings for the details. |
| `country_code` | String | Country code used to display the card's country (when the user is not in that country), as well as to display localized content when content is not available in the user's locale. |
| `title` | String | Required. The title of the offer, such as "20% off any t-shirt." Recommended maximum length is 60 characters to ensure full string is displayed on smaller screens. |
| `id` | String | Required. The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `merchant_locations` | Vec<String> | Merchant locations. There is a maximum of ten on the class. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `enable_smart_tap` | bool | Identifies whether this class supports Smart Tap. The `redemptionIssuers` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `localized_issuer_name` | String | Translated strings for the issuer_name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `provider` | String | Required. The offer provider (either the aggregator name or merchant name). Recommended maximum length is 12 characters to ensure full string is displayed on smaller screens. |
| `fine_print` | String | The fine print or terms of the offer, such as "20% off any t-shirt at Adam's Apparel." |
| `help_uri` | String | The help link for the offer, such as `http://myownpersonaldomain.com/help` |
| `details` | String | The details of the offer. |
| `review_status` | String | Required. The status of the class. This field can be set to `draft` or The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`. |
| `info_module_data` | String | Deprecated. Use textModulesData instead. |
| `hero_image` | String | Optional banner image displayed on the front of the card. If none is present, nothing will be displayed. The image will display at 100% width. |
| `links_module_data` | String | Links module data. If links module data is also defined on the object, both will be displayed. |
| `review` | String | The review comments set by the platform when a class is marked `approved` or `rejected`. |
| `messages` | Vec<String> | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `text_modules_data` | Vec<String> | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#offerClass"`. |
| `localized_short_title` | String | Translated strings for the short title. Recommended maximum length is 20 characters. |
| `multiple_devices_and_holders_allowed_status` | String | Identifies whether multiple users and devices will save the same object referencing this class. |
| `redemption_issuers` | Vec<String> | Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `allow_multiple_users_per_object` | bool | Deprecated. Use `multipleDevicesAndHoldersAllowedStatus` instead. |
| `version` | String | Deprecated |
| `wide_title_image` | String | The wide title image of the offer. When provided, this will be used in place of the title image in the top left of the card view. |
| `word_mark` | String | Deprecated. |
| `security_animation` | String | Optional information about the security animation. If this is set a security animation will be rendered on pass details. |
| `short_title` | String | A shortened version of the title of the offer, such as "20% off," shown to users as a quick reference to the offer contents. Recommended maximum length is 20 characters. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create offerclas
offerclas = provider.walletobjects_api.Offerclas {
}

# Access offerclas outputs
offerclas_id = offerclas.id
offerclas_localized_title = offerclas.localized_title
offerclas_localized_provider = offerclas.localized_provider
offerclas_hex_background_color = offerclas.hex_background_color
offerclas_app_link_data = offerclas.app_link_data
offerclas_image_modules_data = offerclas.image_modules_data
offerclas_redemption_channel = offerclas.redemption_channel
offerclas_view_unlock_requirement = offerclas.view_unlock_requirement
offerclas_localized_fine_print = offerclas.localized_fine_print
offerclas_notify_preference = offerclas.notify_preference
offerclas_callback_options = offerclas.callback_options
offerclas_homepage_uri = offerclas.homepage_uri
offerclas_locations = offerclas.locations
offerclas_class_template_info = offerclas.class_template_info
offerclas_title_image = offerclas.title_image
offerclas_issuer_name = offerclas.issuer_name
offerclas_value_added_module_data = offerclas.value_added_module_data
offerclas_localized_details = offerclas.localized_details
offerclas_country_code = offerclas.country_code
offerclas_title = offerclas.title
offerclas_id = offerclas.id
offerclas_merchant_locations = offerclas.merchant_locations
offerclas_enable_smart_tap = offerclas.enable_smart_tap
offerclas_localized_issuer_name = offerclas.localized_issuer_name
offerclas_provider = offerclas.provider
offerclas_fine_print = offerclas.fine_print
offerclas_help_uri = offerclas.help_uri
offerclas_details = offerclas.details
offerclas_review_status = offerclas.review_status
offerclas_info_module_data = offerclas.info_module_data
offerclas_hero_image = offerclas.hero_image
offerclas_links_module_data = offerclas.links_module_data
offerclas_review = offerclas.review
offerclas_messages = offerclas.messages
offerclas_text_modules_data = offerclas.text_modules_data
offerclas_kind = offerclas.kind
offerclas_localized_short_title = offerclas.localized_short_title
offerclas_multiple_devices_and_holders_allowed_status = offerclas.multiple_devices_and_holders_allowed_status
offerclas_redemption_issuers = offerclas.redemption_issuers
offerclas_allow_multiple_users_per_object = offerclas.allow_multiple_users_per_object
offerclas_version = offerclas.version
offerclas_wide_title_image = offerclas.wide_title_image
offerclas_word_mark = offerclas.word_mark
offerclas_security_animation = offerclas.security_animation
offerclas_short_title = offerclas.short_title
```

---


### Private_content

Provide Google with information about awaiting private pass update. This will allow Google to provide the update notification to the device that currently holds this pass.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `external_pass_id` | String |  | Required. A fully qualified identifier of the pass that the issuer wants to notify the pass holder(s) about. Formatted as . |
| `update_uri` | String |  | Required. The issuer endpoint URI the pass holder needs to follow in order to receive an updated pass JWT. It can not contain any sensitive information. The endpoint needs to authenticate the user before giving the user the updated JWT. Example update URI https://someissuer.com/update/passId=someExternalPassId |
| `updated_pass_jwt_signature` | String |  | Required. The JWT signature of the updated pass that the issuer wants to notify Google about. Only devices that report a different JWT signature than this JWT signature will receive the update notification. |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create private_content
private_content = provider.walletobjects_api.Private_content {
}

```

---


### Transitclas

Inserts a transit class with the given ID and properties.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `merchant_locations` | Vec<String> |  | Merchant locations. There is a maximum of ten on the class. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `version` | String |  | Deprecated |
| `notify_preference` | String |  | Whether or not field updates to this class should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If set to DO_NOT_NOTIFY or NOTIFICATION_SETTINGS_UNSPECIFIED, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `value_added_module_data` | Vec<String> |  | Optional value added module data. Maximum of ten on the class. For a pass only ten will be displayed, prioritizing those from the object. |
| `custom_fare_name_label` | String |  | A custom label to use for the transit fare name value (`transitObject.ticketLeg.fareName`). |
| `custom_transit_terminus_name_label` | String |  | A custom label to use for the transit terminus name value (`transitObject.ticketLeg.transitTerminusName`). |
| `country_code` | String |  | Country code used to display the card's country (when the user is not in that country), as well as to display localized content when content is not available in the user's locale. |
| `allow_multiple_users_per_object` | bool |  | Deprecated. Use `multipleDevicesAndHoldersAllowedStatus` instead. |
| `custom_route_restrictions_details_label` | String |  | A custom label to use for the route restrictions details value (`transitObject.ticketRestrictions.routeRestrictionsDetails`). |
| `custom_route_restrictions_label` | String |  | A custom label to use for the route restrictions value (`transitObject.ticketRestrictions.routeRestrictions`). |
| `custom_fare_class_label` | String |  | A custom label to use for the fare class value (`transitObject.ticketLeg.ticketSeat.fareClass`). |
| `activation_options` | String |  | Activation options for an activatable ticket. |
| `custom_purchase_receipt_number_label` | String |  | A custom label to use for the purchase receipt number value (`transitObject.purchaseDetails.purchaseReceiptNumber`). |
| `id` | String |  | Required. The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `info_module_data` | String |  | Deprecated. Use textModulesData instead. |
| `custom_time_restrictions_label` | String |  | A custom label to use for the time restrictions details value (`transitObject.ticketRestrictions.timeRestrictions`). |
| `hex_background_color` | String |  | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `hero_image` | String |  | Optional banner image displayed on the front of the card. If none is present, nothing will be displayed. The image will display at 100% width. |
| `custom_coach_label` | String |  | A custom label to use for the coach value (`transitObject.ticketLeg.ticketSeat.coach`). |
| `callback_options` | String |  | Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback. |
| `image_modules_data` | Vec<String> |  | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `review_status` | String |  | Required. The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`. |
| `watermark` | String |  | Watermark image to display on the user's device. |
| `wide_logo` | String |  | The wide logo of the ticket. When provided, this will be used in place of the logo in the top left of the card view. |
| `logo` | String |  | Required. The logo image of the ticket. This image is displayed in the card detail view of the app. |
| `custom_ticket_number_label` | String |  | A custom label to use for the ticket number value (`transitObject.ticketNumber`). |
| `custom_discount_message_label` | String |  | A custom label to use for the transit discount message value (`transitObject.purchaseDetails.ticketCost.discountMessage`). |
| `links_module_data` | String |  | Links module data. If links module data is also defined on the object, both will be displayed. |
| `locations` | Vec<String> |  | Note: This field is currently not supported to trigger geo notifications. |
| `multiple_devices_and_holders_allowed_status` | String |  | Identifies whether multiple users and devices will save the same object referencing this class. |
| `review` | String |  | The review comments set by the platform when a class is marked `approved` or `rejected`. |
| `transit_operator_name` | String |  | The name of the transit operator. |
| `text_modules_data` | Vec<String> |  | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `view_unlock_requirement` | String |  | View Unlock Requirement options for the transit ticket. |
| `messages` | Vec<String> |  | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `word_mark` | String |  | Deprecated. |
| `redemption_issuers` | Vec<String> |  | Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `security_animation` | String |  | Optional information about the security animation. If this is set a security animation will be rendered on pass details. |
| `transit_type` | String |  | Required. The type of transit this class represents, such as "bus". |
| `issuer_name` | String |  | Required. The issuer name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `custom_carriage_label` | String |  | A custom label to use for the carriage value (`transitObject.ticketLeg.carriage`). |
| `custom_concession_category_label` | String |  | A custom label to use for the transit concession category value (`transitObject.concessionCategory`). |
| `custom_purchase_price_label` | String |  | A custom label to use for the purchase price value (`transitObject.purchaseDetails.ticketCost.purchasePrice`). |
| `custom_seat_label` | String |  | A custom label to use for the seat location value (`transitObject.ticketLeg.ticketSeat.seat`). |
| `app_link_data` | String |  | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding object that will be used instead. |
| `class_template_info` | String |  | Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display. |
| `enable_smart_tap` | bool |  | Identifies whether this class supports Smart Tap. The `redemptionIssuers` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `custom_confirmation_code_label` | String |  | A custom label to use for the confirmation code value (`transitObject.purchaseDetails.confirmationCode`). |
| `custom_other_restrictions_label` | String |  | A custom label to use for the other restrictions value (`transitObject.ticketRestrictions.otherRestrictions`). |
| `enable_single_leg_itinerary` | bool |  | Controls the display of the single-leg itinerary for this class. By default, an itinerary will only display for multi-leg trips. |
| `custom_platform_label` | String |  | A custom label to use for the boarding platform value (`transitObject.ticketLeg.platform`). |
| `custom_purchase_face_value_label` | String |  | A custom label to use for the purchase face value (`transitObject.purchaseDetails.ticketCost.faceValue`). |
| `homepage_uri` | String |  | The URI of your application's home page. Populating the URI in this field results in the exact same behavior as populating an URI in linksModuleData (when an object is rendered, a link to the homepage is shown in what would usually be thought of as the linksModuleData section of the object). |
| `language_override` | String |  | If this field is present, transit tickets served to a user's device will always be in this language. Represents the BCP 47 language tag. Example values are "en-US", "en-GB", "de", or "de-AT". |
| `custom_zone_label` | String |  | A custom label to use for the boarding zone value (`transitObject.ticketLeg.zone`). |
| `localized_issuer_name` | String |  | Translated strings for the issuer_name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `merchant_locations` | Vec<String> | Merchant locations. There is a maximum of ten on the class. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `version` | String | Deprecated |
| `notify_preference` | String | Whether or not field updates to this class should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If set to DO_NOT_NOTIFY or NOTIFICATION_SETTINGS_UNSPECIFIED, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `value_added_module_data` | Vec<String> | Optional value added module data. Maximum of ten on the class. For a pass only ten will be displayed, prioritizing those from the object. |
| `custom_fare_name_label` | String | A custom label to use for the transit fare name value (`transitObject.ticketLeg.fareName`). |
| `custom_transit_terminus_name_label` | String | A custom label to use for the transit terminus name value (`transitObject.ticketLeg.transitTerminusName`). |
| `country_code` | String | Country code used to display the card's country (when the user is not in that country), as well as to display localized content when content is not available in the user's locale. |
| `allow_multiple_users_per_object` | bool | Deprecated. Use `multipleDevicesAndHoldersAllowedStatus` instead. |
| `custom_route_restrictions_details_label` | String | A custom label to use for the route restrictions details value (`transitObject.ticketRestrictions.routeRestrictionsDetails`). |
| `custom_route_restrictions_label` | String | A custom label to use for the route restrictions value (`transitObject.ticketRestrictions.routeRestrictions`). |
| `custom_fare_class_label` | String | A custom label to use for the fare class value (`transitObject.ticketLeg.ticketSeat.fareClass`). |
| `activation_options` | String | Activation options for an activatable ticket. |
| `custom_purchase_receipt_number_label` | String | A custom label to use for the purchase receipt number value (`transitObject.purchaseDetails.purchaseReceiptNumber`). |
| `id` | String | Required. The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `info_module_data` | String | Deprecated. Use textModulesData instead. |
| `custom_time_restrictions_label` | String | A custom label to use for the time restrictions details value (`transitObject.ticketRestrictions.timeRestrictions`). |
| `hex_background_color` | String | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `hero_image` | String | Optional banner image displayed on the front of the card. If none is present, nothing will be displayed. The image will display at 100% width. |
| `custom_coach_label` | String | A custom label to use for the coach value (`transitObject.ticketLeg.ticketSeat.coach`). |
| `callback_options` | String | Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback. |
| `image_modules_data` | Vec<String> | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `review_status` | String | Required. The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`. |
| `watermark` | String | Watermark image to display on the user's device. |
| `wide_logo` | String | The wide logo of the ticket. When provided, this will be used in place of the logo in the top left of the card view. |
| `logo` | String | Required. The logo image of the ticket. This image is displayed in the card detail view of the app. |
| `custom_ticket_number_label` | String | A custom label to use for the ticket number value (`transitObject.ticketNumber`). |
| `custom_discount_message_label` | String | A custom label to use for the transit discount message value (`transitObject.purchaseDetails.ticketCost.discountMessage`). |
| `links_module_data` | String | Links module data. If links module data is also defined on the object, both will be displayed. |
| `locations` | Vec<String> | Note: This field is currently not supported to trigger geo notifications. |
| `multiple_devices_and_holders_allowed_status` | String | Identifies whether multiple users and devices will save the same object referencing this class. |
| `review` | String | The review comments set by the platform when a class is marked `approved` or `rejected`. |
| `transit_operator_name` | String | The name of the transit operator. |
| `text_modules_data` | Vec<String> | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `view_unlock_requirement` | String | View Unlock Requirement options for the transit ticket. |
| `messages` | Vec<String> | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `word_mark` | String | Deprecated. |
| `redemption_issuers` | Vec<String> | Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `security_animation` | String | Optional information about the security animation. If this is set a security animation will be rendered on pass details. |
| `transit_type` | String | Required. The type of transit this class represents, such as "bus". |
| `issuer_name` | String | Required. The issuer name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `custom_carriage_label` | String | A custom label to use for the carriage value (`transitObject.ticketLeg.carriage`). |
| `custom_concession_category_label` | String | A custom label to use for the transit concession category value (`transitObject.concessionCategory`). |
| `custom_purchase_price_label` | String | A custom label to use for the purchase price value (`transitObject.purchaseDetails.ticketCost.purchasePrice`). |
| `custom_seat_label` | String | A custom label to use for the seat location value (`transitObject.ticketLeg.ticketSeat.seat`). |
| `app_link_data` | String | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding object that will be used instead. |
| `class_template_info` | String | Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display. |
| `enable_smart_tap` | bool | Identifies whether this class supports Smart Tap. The `redemptionIssuers` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `custom_confirmation_code_label` | String | A custom label to use for the confirmation code value (`transitObject.purchaseDetails.confirmationCode`). |
| `custom_other_restrictions_label` | String | A custom label to use for the other restrictions value (`transitObject.ticketRestrictions.otherRestrictions`). |
| `enable_single_leg_itinerary` | bool | Controls the display of the single-leg itinerary for this class. By default, an itinerary will only display for multi-leg trips. |
| `custom_platform_label` | String | A custom label to use for the boarding platform value (`transitObject.ticketLeg.platform`). |
| `custom_purchase_face_value_label` | String | A custom label to use for the purchase face value (`transitObject.purchaseDetails.ticketCost.faceValue`). |
| `homepage_uri` | String | The URI of your application's home page. Populating the URI in this field results in the exact same behavior as populating an URI in linksModuleData (when an object is rendered, a link to the homepage is shown in what would usually be thought of as the linksModuleData section of the object). |
| `language_override` | String | If this field is present, transit tickets served to a user's device will always be in this language. Represents the BCP 47 language tag. Example values are "en-US", "en-GB", "de", or "de-AT". |
| `custom_zone_label` | String | A custom label to use for the boarding zone value (`transitObject.ticketLeg.zone`). |
| `localized_issuer_name` | String | Translated strings for the issuer_name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create transitclas
transitclas = provider.walletobjects_api.Transitclas {
}

# Access transitclas outputs
transitclas_id = transitclas.id
transitclas_merchant_locations = transitclas.merchant_locations
transitclas_version = transitclas.version
transitclas_notify_preference = transitclas.notify_preference
transitclas_value_added_module_data = transitclas.value_added_module_data
transitclas_custom_fare_name_label = transitclas.custom_fare_name_label
transitclas_custom_transit_terminus_name_label = transitclas.custom_transit_terminus_name_label
transitclas_country_code = transitclas.country_code
transitclas_allow_multiple_users_per_object = transitclas.allow_multiple_users_per_object
transitclas_custom_route_restrictions_details_label = transitclas.custom_route_restrictions_details_label
transitclas_custom_route_restrictions_label = transitclas.custom_route_restrictions_label
transitclas_custom_fare_class_label = transitclas.custom_fare_class_label
transitclas_activation_options = transitclas.activation_options
transitclas_custom_purchase_receipt_number_label = transitclas.custom_purchase_receipt_number_label
transitclas_id = transitclas.id
transitclas_info_module_data = transitclas.info_module_data
transitclas_custom_time_restrictions_label = transitclas.custom_time_restrictions_label
transitclas_hex_background_color = transitclas.hex_background_color
transitclas_hero_image = transitclas.hero_image
transitclas_custom_coach_label = transitclas.custom_coach_label
transitclas_callback_options = transitclas.callback_options
transitclas_image_modules_data = transitclas.image_modules_data
transitclas_review_status = transitclas.review_status
transitclas_watermark = transitclas.watermark
transitclas_wide_logo = transitclas.wide_logo
transitclas_logo = transitclas.logo
transitclas_custom_ticket_number_label = transitclas.custom_ticket_number_label
transitclas_custom_discount_message_label = transitclas.custom_discount_message_label
transitclas_links_module_data = transitclas.links_module_data
transitclas_locations = transitclas.locations
transitclas_multiple_devices_and_holders_allowed_status = transitclas.multiple_devices_and_holders_allowed_status
transitclas_review = transitclas.review
transitclas_transit_operator_name = transitclas.transit_operator_name
transitclas_text_modules_data = transitclas.text_modules_data
transitclas_view_unlock_requirement = transitclas.view_unlock_requirement
transitclas_messages = transitclas.messages
transitclas_word_mark = transitclas.word_mark
transitclas_redemption_issuers = transitclas.redemption_issuers
transitclas_security_animation = transitclas.security_animation
transitclas_transit_type = transitclas.transit_type
transitclas_issuer_name = transitclas.issuer_name
transitclas_custom_carriage_label = transitclas.custom_carriage_label
transitclas_custom_concession_category_label = transitclas.custom_concession_category_label
transitclas_custom_purchase_price_label = transitclas.custom_purchase_price_label
transitclas_custom_seat_label = transitclas.custom_seat_label
transitclas_app_link_data = transitclas.app_link_data
transitclas_class_template_info = transitclas.class_template_info
transitclas_enable_smart_tap = transitclas.enable_smart_tap
transitclas_custom_confirmation_code_label = transitclas.custom_confirmation_code_label
transitclas_custom_other_restrictions_label = transitclas.custom_other_restrictions_label
transitclas_enable_single_leg_itinerary = transitclas.enable_single_leg_itinerary
transitclas_custom_platform_label = transitclas.custom_platform_label
transitclas_custom_purchase_face_value_label = transitclas.custom_purchase_face_value_label
transitclas_homepage_uri = transitclas.homepage_uri
transitclas_language_override = transitclas.language_override
transitclas_custom_zone_label = transitclas.custom_zone_label
transitclas_localized_issuer_name = transitclas.localized_issuer_name
```

---


### Permission

Returns the permissions for the given issuer id.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `permissions` | Vec<String> |  | The complete list of permissions for the issuer account. |
| `issuer_id` | String |  | ID of the issuer the list of permissions refer to. |
| `resource_id` | String | ✅ | The unique identifier for an issuer. This ID must be unique across all issuers. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `permissions` | Vec<String> | The complete list of permissions for the issuer account. |
| `issuer_id` | String | ID of the issuer the list of permissions refer to. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Access permission outputs
permission_id = permission.id
permission_permissions = permission.permissions
permission_issuer_id = permission.issuer_id
```

---


### Giftcardclas

Inserts an gift card class with the given ID and properties.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `review_status` | String |  | Required. The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`. |
| `localized_issuer_name` | String |  | Translated strings for the issuer_name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `pin_label` | String |  | The label to display for the PIN, such as "4-digit PIN". |
| `value_added_module_data` | Vec<String> |  | Optional value added module data. Maximum of ten on the class. For a pass only ten will be displayed, prioritizing those from the object. |
| `version` | String |  | Deprecated |
| `wide_program_logo` | String |  | The wide logo of the gift card program or company. When provided, this will be used in place of the program logo in the top left of the card view. |
| `country_code` | String |  | Country code used to display the card's country (when the user is not in that country), as well as to display localized content when content is not available in the user's locale. |
| `id` | String |  | Required. The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `program_logo` | String |  | The logo of the gift card program or company. This logo is displayed in both the details and list views of the app. |
| `app_link_data` | String |  | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding object that will be used instead. |
| `card_number_label` | String |  | The label to display for the card number, such as "Card Number". |
| `homepage_uri` | String |  | The URI of your application's home page. Populating the URI in this field results in the exact same behavior as populating an URI in linksModuleData (when an object is rendered, a link to the homepage is shown in what would usually be thought of as the linksModuleData section of the object). |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#giftCardClass"`. |
| `merchant_locations` | Vec<String> |  | Merchant locations. There is a maximum of ten on the class. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `event_number_label` | String |  | The label to display for event number, such as "Target Event #". |
| `hero_image` | String |  | Optional banner image displayed on the front of the card. If none is present, nothing will be displayed. The image will display at 100% width. |
| `issuer_name` | String |  | Required. The issuer name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `class_template_info` | String |  | Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display. |
| `allow_barcode_redemption` | bool |  | Determines whether the merchant supports gift card redemption using barcode. If true, app displays a barcode for the gift card on the Gift card details screen. If false, a barcode is not displayed. |
| `callback_options` | String |  | Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback. |
| `localized_card_number_label` | String |  | Translated strings for the card_number_label. |
| `localized_pin_label` | String |  | Translated strings for the pin_label. |
| `enable_smart_tap` | bool |  | Identifies whether this class supports Smart Tap. The `redemptionIssuers` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `links_module_data` | String |  | Links module data. If links module data is also defined on the object, both will be displayed. |
| `merchant_name` | String |  | Merchant name, such as "Adam's Apparel". The app may display an ellipsis after the first 20 characters to ensure full string is displayed on smaller screens. |
| `messages` | Vec<String> |  | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `notify_preference` | String |  | Whether or not field updates to this class should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If not specified, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `redemption_issuers` | Vec<String> |  | Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `multiple_devices_and_holders_allowed_status` | String |  | Identifies whether multiple users and devices will save the same object referencing this class. |
| `localized_merchant_name` | String |  | Translated strings for the merchant_name. The app may display an ellipsis after the first 20 characters to ensure full string is displayed on smaller screens. |
| `review` | String |  | The review comments set by the platform when a class is marked `approved` or `rejected`. |
| `allow_multiple_users_per_object` | bool |  | Deprecated. Use `multipleDevicesAndHoldersAllowedStatus` instead. |
| `hex_background_color` | String |  | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `security_animation` | String |  | Optional information about the security animation. If this is set a security animation will be rendered on pass details. |
| `localized_event_number_label` | String |  | Translated strings for the event_number_label. |
| `text_modules_data` | Vec<String> |  | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `view_unlock_requirement` | String |  | View Unlock Requirement options for the gift card. |
| `image_modules_data` | Vec<String> |  | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `word_mark` | String |  | Deprecated. |
| `info_module_data` | String |  | Deprecated. Use textModulesData instead. |
| `locations` | Vec<String> |  | Note: This field is currently not supported to trigger geo notifications. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `review_status` | String | Required. The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`. |
| `localized_issuer_name` | String | Translated strings for the issuer_name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `pin_label` | String | The label to display for the PIN, such as "4-digit PIN". |
| `value_added_module_data` | Vec<String> | Optional value added module data. Maximum of ten on the class. For a pass only ten will be displayed, prioritizing those from the object. |
| `version` | String | Deprecated |
| `wide_program_logo` | String | The wide logo of the gift card program or company. When provided, this will be used in place of the program logo in the top left of the card view. |
| `country_code` | String | Country code used to display the card's country (when the user is not in that country), as well as to display localized content when content is not available in the user's locale. |
| `id` | String | Required. The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `program_logo` | String | The logo of the gift card program or company. This logo is displayed in both the details and list views of the app. |
| `app_link_data` | String | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding object that will be used instead. |
| `card_number_label` | String | The label to display for the card number, such as "Card Number". |
| `homepage_uri` | String | The URI of your application's home page. Populating the URI in this field results in the exact same behavior as populating an URI in linksModuleData (when an object is rendered, a link to the homepage is shown in what would usually be thought of as the linksModuleData section of the object). |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#giftCardClass"`. |
| `merchant_locations` | Vec<String> | Merchant locations. There is a maximum of ten on the class. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `event_number_label` | String | The label to display for event number, such as "Target Event #". |
| `hero_image` | String | Optional banner image displayed on the front of the card. If none is present, nothing will be displayed. The image will display at 100% width. |
| `issuer_name` | String | Required. The issuer name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `class_template_info` | String | Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display. |
| `allow_barcode_redemption` | bool | Determines whether the merchant supports gift card redemption using barcode. If true, app displays a barcode for the gift card on the Gift card details screen. If false, a barcode is not displayed. |
| `callback_options` | String | Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback. |
| `localized_card_number_label` | String | Translated strings for the card_number_label. |
| `localized_pin_label` | String | Translated strings for the pin_label. |
| `enable_smart_tap` | bool | Identifies whether this class supports Smart Tap. The `redemptionIssuers` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `links_module_data` | String | Links module data. If links module data is also defined on the object, both will be displayed. |
| `merchant_name` | String | Merchant name, such as "Adam's Apparel". The app may display an ellipsis after the first 20 characters to ensure full string is displayed on smaller screens. |
| `messages` | Vec<String> | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `notify_preference` | String | Whether or not field updates to this class should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If not specified, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `redemption_issuers` | Vec<String> | Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `multiple_devices_and_holders_allowed_status` | String | Identifies whether multiple users and devices will save the same object referencing this class. |
| `localized_merchant_name` | String | Translated strings for the merchant_name. The app may display an ellipsis after the first 20 characters to ensure full string is displayed on smaller screens. |
| `review` | String | The review comments set by the platform when a class is marked `approved` or `rejected`. |
| `allow_multiple_users_per_object` | bool | Deprecated. Use `multipleDevicesAndHoldersAllowedStatus` instead. |
| `hex_background_color` | String | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `security_animation` | String | Optional information about the security animation. If this is set a security animation will be rendered on pass details. |
| `localized_event_number_label` | String | Translated strings for the event_number_label. |
| `text_modules_data` | Vec<String> | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `view_unlock_requirement` | String | View Unlock Requirement options for the gift card. |
| `image_modules_data` | Vec<String> | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `word_mark` | String | Deprecated. |
| `info_module_data` | String | Deprecated. Use textModulesData instead. |
| `locations` | Vec<String> | Note: This field is currently not supported to trigger geo notifications. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create giftcardclas
giftcardclas = provider.walletobjects_api.Giftcardclas {
}

# Access giftcardclas outputs
giftcardclas_id = giftcardclas.id
giftcardclas_review_status = giftcardclas.review_status
giftcardclas_localized_issuer_name = giftcardclas.localized_issuer_name
giftcardclas_pin_label = giftcardclas.pin_label
giftcardclas_value_added_module_data = giftcardclas.value_added_module_data
giftcardclas_version = giftcardclas.version
giftcardclas_wide_program_logo = giftcardclas.wide_program_logo
giftcardclas_country_code = giftcardclas.country_code
giftcardclas_id = giftcardclas.id
giftcardclas_program_logo = giftcardclas.program_logo
giftcardclas_app_link_data = giftcardclas.app_link_data
giftcardclas_card_number_label = giftcardclas.card_number_label
giftcardclas_homepage_uri = giftcardclas.homepage_uri
giftcardclas_kind = giftcardclas.kind
giftcardclas_merchant_locations = giftcardclas.merchant_locations
giftcardclas_event_number_label = giftcardclas.event_number_label
giftcardclas_hero_image = giftcardclas.hero_image
giftcardclas_issuer_name = giftcardclas.issuer_name
giftcardclas_class_template_info = giftcardclas.class_template_info
giftcardclas_allow_barcode_redemption = giftcardclas.allow_barcode_redemption
giftcardclas_callback_options = giftcardclas.callback_options
giftcardclas_localized_card_number_label = giftcardclas.localized_card_number_label
giftcardclas_localized_pin_label = giftcardclas.localized_pin_label
giftcardclas_enable_smart_tap = giftcardclas.enable_smart_tap
giftcardclas_links_module_data = giftcardclas.links_module_data
giftcardclas_merchant_name = giftcardclas.merchant_name
giftcardclas_messages = giftcardclas.messages
giftcardclas_notify_preference = giftcardclas.notify_preference
giftcardclas_redemption_issuers = giftcardclas.redemption_issuers
giftcardclas_multiple_devices_and_holders_allowed_status = giftcardclas.multiple_devices_and_holders_allowed_status
giftcardclas_localized_merchant_name = giftcardclas.localized_merchant_name
giftcardclas_review = giftcardclas.review
giftcardclas_allow_multiple_users_per_object = giftcardclas.allow_multiple_users_per_object
giftcardclas_hex_background_color = giftcardclas.hex_background_color
giftcardclas_security_animation = giftcardclas.security_animation
giftcardclas_localized_event_number_label = giftcardclas.localized_event_number_label
giftcardclas_text_modules_data = giftcardclas.text_modules_data
giftcardclas_view_unlock_requirement = giftcardclas.view_unlock_requirement
giftcardclas_image_modules_data = giftcardclas.image_modules_data
giftcardclas_word_mark = giftcardclas.word_mark
giftcardclas_info_module_data = giftcardclas.info_module_data
giftcardclas_locations = giftcardclas.locations
```

---


### Loyaltyclas

Inserts an loyalty class with the given ID and properties.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `multiple_devices_and_holders_allowed_status` | String |  | Identifies whether multiple users and devices will save the same object referencing this class. |
| `account_id_label` | String |  | The account ID label, such as "Member ID." Recommended maximum length is 15 characters to ensure full string is displayed on smaller screens. |
| `allow_multiple_users_per_object` | bool |  | Deprecated. Use `multipleDevicesAndHoldersAllowedStatus` instead. |
| `class_template_info` | String |  | Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display. |
| `localized_issuer_name` | String |  | Translated strings for the issuer_name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `localized_rewards_tier` | String |  | Translated strings for the rewards_tier. Recommended maximum length is 7 characters to ensure full string is displayed on smaller screens. |
| `app_link_data` | String |  | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding object that will be used instead. |
| `hero_image` | String |  | Optional banner image displayed on the front of the card. If none is present, nothing will be displayed. The image will display at 100% width. |
| `localized_secondary_rewards_tier_label` | String |  | Translated strings for the secondary_rewards_tier_label. |
| `secondary_rewards_tier_label` | String |  | The secondary rewards tier label, such as "Rewards Tier." |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#loyaltyClass"`. |
| `secondary_rewards_tier` | String |  | The secondary rewards tier, such as "Gold" or "Platinum." |
| `program_logo` | String |  | Required. The logo of the loyalty program or company. This logo is displayed in both the details and list views of the app. |
| `review` | String |  | The review comments set by the platform when a class is marked `approved` or `rejected`. |
| `text_modules_data` | Vec<String> |  | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `discoverable_program` | String |  | Information about how the class may be discovered and instantiated from within the Google Pay app. |
| `program_name` | String |  | Required. The program name, such as "Adam's Apparel". The app may display an ellipsis after the first 20 characters to ensure full string is displayed on smaller screens. |
| `redemption_issuers` | Vec<String> |  | Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and one of object level `smartTapRedemptionValue`, barcode.value`, or `accountId` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `hex_background_color` | String |  | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `notify_preference` | String |  | Whether or not field updates to this class should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If not specified, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `review_status` | String |  | Required. The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`. |
| `id` | String |  | Required. The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `rewards_tier` | String |  | The rewards tier, such as "Gold" or "Platinum." Recommended maximum length is 7 characters to ensure full string is displayed on smaller screens. |
| `version` | String |  | Deprecated |
| `account_name_label` | String |  | The account name label, such as "Member Name." Recommended maximum length is 15 characters to ensure full string is displayed on smaller screens. |
| `issuer_name` | String |  | Required. The issuer name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `enable_smart_tap` | bool |  | Identifies whether this class supports Smart Tap. The `redemptionIssuers` and one of object level `smartTapRedemptionLevel`, barcode.value`, or `accountId` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `locations` | Vec<String> |  | Note: This field is currently not supported to trigger geo notifications. |
| `image_modules_data` | Vec<String> |  | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `messages` | Vec<String> |  | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `value_added_module_data` | Vec<String> |  | Optional value added module data. Maximum of ten on the class. For a pass only ten will be displayed, prioritizing those from the object. |
| `localized_secondary_rewards_tier` | String |  | Translated strings for the secondary_rewards_tier. |
| `localized_account_id_label` | String |  | Translated strings for the account_id_label. Recommended maximum length is 15 characters to ensure full string is displayed on smaller screens. |
| `localized_program_name` | String |  | Translated strings for the program_name. The app may display an ellipsis after the first 20 characters to ensure full string is displayed on smaller screens. |
| `homepage_uri` | String |  | The URI of your application's home page. Populating the URI in this field results in the exact same behavior as populating an URI in linksModuleData (when an object is rendered, a link to the homepage is shown in what would usually be thought of as the linksModuleData section of the object). |
| `localized_rewards_tier_label` | String |  | Translated strings for the rewards_tier_label. Recommended maximum length is 9 characters to ensure full string is displayed on smaller screens. |
| `word_mark` | String |  | Deprecated. |
| `view_unlock_requirement` | String |  | View Unlock Requirement options for the loyalty card. |
| `wide_program_logo` | String |  | The wide logo of the loyalty program or company. When provided, this will be used in place of the program logo in the top left of the card view. |
| `info_module_data` | String |  | Deprecated. Use textModulesData instead. |
| `merchant_locations` | Vec<String> |  | Merchant locations. There is a maximum of ten on the class. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `security_animation` | String |  | Optional information about the security animation. If this is set a security animation will be rendered on pass details. |
| `links_module_data` | String |  | Links module data. If links module data is also defined on the object, both will be displayed. |
| `rewards_tier_label` | String |  | The rewards tier label, such as "Rewards Tier." Recommended maximum length is 9 characters to ensure full string is displayed on smaller screens. |
| `country_code` | String |  | Country code used to display the card's country (when the user is not in that country), as well as to display localized content when content is not available in the user's locale. |
| `localized_account_name_label` | String |  | Translated strings for the account_name_label. Recommended maximum length is 15 characters to ensure full string is displayed on smaller screens. |
| `callback_options` | String |  | Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `multiple_devices_and_holders_allowed_status` | String | Identifies whether multiple users and devices will save the same object referencing this class. |
| `account_id_label` | String | The account ID label, such as "Member ID." Recommended maximum length is 15 characters to ensure full string is displayed on smaller screens. |
| `allow_multiple_users_per_object` | bool | Deprecated. Use `multipleDevicesAndHoldersAllowedStatus` instead. |
| `class_template_info` | String | Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display. |
| `localized_issuer_name` | String | Translated strings for the issuer_name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `localized_rewards_tier` | String | Translated strings for the rewards_tier. Recommended maximum length is 7 characters to ensure full string is displayed on smaller screens. |
| `app_link_data` | String | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding object that will be used instead. |
| `hero_image` | String | Optional banner image displayed on the front of the card. If none is present, nothing will be displayed. The image will display at 100% width. |
| `localized_secondary_rewards_tier_label` | String | Translated strings for the secondary_rewards_tier_label. |
| `secondary_rewards_tier_label` | String | The secondary rewards tier label, such as "Rewards Tier." |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#loyaltyClass"`. |
| `secondary_rewards_tier` | String | The secondary rewards tier, such as "Gold" or "Platinum." |
| `program_logo` | String | Required. The logo of the loyalty program or company. This logo is displayed in both the details and list views of the app. |
| `review` | String | The review comments set by the platform when a class is marked `approved` or `rejected`. |
| `text_modules_data` | Vec<String> | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `discoverable_program` | String | Information about how the class may be discovered and instantiated from within the Google Pay app. |
| `program_name` | String | Required. The program name, such as "Adam's Apparel". The app may display an ellipsis after the first 20 characters to ensure full string is displayed on smaller screens. |
| `redemption_issuers` | Vec<String> | Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and one of object level `smartTapRedemptionValue`, barcode.value`, or `accountId` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `hex_background_color` | String | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `notify_preference` | String | Whether or not field updates to this class should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If not specified, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `review_status` | String | Required. The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`. |
| `id` | String | Required. The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `rewards_tier` | String | The rewards tier, such as "Gold" or "Platinum." Recommended maximum length is 7 characters to ensure full string is displayed on smaller screens. |
| `version` | String | Deprecated |
| `account_name_label` | String | The account name label, such as "Member Name." Recommended maximum length is 15 characters to ensure full string is displayed on smaller screens. |
| `issuer_name` | String | Required. The issuer name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `enable_smart_tap` | bool | Identifies whether this class supports Smart Tap. The `redemptionIssuers` and one of object level `smartTapRedemptionLevel`, barcode.value`, or `accountId` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `locations` | Vec<String> | Note: This field is currently not supported to trigger geo notifications. |
| `image_modules_data` | Vec<String> | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `messages` | Vec<String> | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `value_added_module_data` | Vec<String> | Optional value added module data. Maximum of ten on the class. For a pass only ten will be displayed, prioritizing those from the object. |
| `localized_secondary_rewards_tier` | String | Translated strings for the secondary_rewards_tier. |
| `localized_account_id_label` | String | Translated strings for the account_id_label. Recommended maximum length is 15 characters to ensure full string is displayed on smaller screens. |
| `localized_program_name` | String | Translated strings for the program_name. The app may display an ellipsis after the first 20 characters to ensure full string is displayed on smaller screens. |
| `homepage_uri` | String | The URI of your application's home page. Populating the URI in this field results in the exact same behavior as populating an URI in linksModuleData (when an object is rendered, a link to the homepage is shown in what would usually be thought of as the linksModuleData section of the object). |
| `localized_rewards_tier_label` | String | Translated strings for the rewards_tier_label. Recommended maximum length is 9 characters to ensure full string is displayed on smaller screens. |
| `word_mark` | String | Deprecated. |
| `view_unlock_requirement` | String | View Unlock Requirement options for the loyalty card. |
| `wide_program_logo` | String | The wide logo of the loyalty program or company. When provided, this will be used in place of the program logo in the top left of the card view. |
| `info_module_data` | String | Deprecated. Use textModulesData instead. |
| `merchant_locations` | Vec<String> | Merchant locations. There is a maximum of ten on the class. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `security_animation` | String | Optional information about the security animation. If this is set a security animation will be rendered on pass details. |
| `links_module_data` | String | Links module data. If links module data is also defined on the object, both will be displayed. |
| `rewards_tier_label` | String | The rewards tier label, such as "Rewards Tier." Recommended maximum length is 9 characters to ensure full string is displayed on smaller screens. |
| `country_code` | String | Country code used to display the card's country (when the user is not in that country), as well as to display localized content when content is not available in the user's locale. |
| `localized_account_name_label` | String | Translated strings for the account_name_label. Recommended maximum length is 15 characters to ensure full string is displayed on smaller screens. |
| `callback_options` | String | Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create loyaltyclas
loyaltyclas = provider.walletobjects_api.Loyaltyclas {
}

# Access loyaltyclas outputs
loyaltyclas_id = loyaltyclas.id
loyaltyclas_multiple_devices_and_holders_allowed_status = loyaltyclas.multiple_devices_and_holders_allowed_status
loyaltyclas_account_id_label = loyaltyclas.account_id_label
loyaltyclas_allow_multiple_users_per_object = loyaltyclas.allow_multiple_users_per_object
loyaltyclas_class_template_info = loyaltyclas.class_template_info
loyaltyclas_localized_issuer_name = loyaltyclas.localized_issuer_name
loyaltyclas_localized_rewards_tier = loyaltyclas.localized_rewards_tier
loyaltyclas_app_link_data = loyaltyclas.app_link_data
loyaltyclas_hero_image = loyaltyclas.hero_image
loyaltyclas_localized_secondary_rewards_tier_label = loyaltyclas.localized_secondary_rewards_tier_label
loyaltyclas_secondary_rewards_tier_label = loyaltyclas.secondary_rewards_tier_label
loyaltyclas_kind = loyaltyclas.kind
loyaltyclas_secondary_rewards_tier = loyaltyclas.secondary_rewards_tier
loyaltyclas_program_logo = loyaltyclas.program_logo
loyaltyclas_review = loyaltyclas.review
loyaltyclas_text_modules_data = loyaltyclas.text_modules_data
loyaltyclas_discoverable_program = loyaltyclas.discoverable_program
loyaltyclas_program_name = loyaltyclas.program_name
loyaltyclas_redemption_issuers = loyaltyclas.redemption_issuers
loyaltyclas_hex_background_color = loyaltyclas.hex_background_color
loyaltyclas_notify_preference = loyaltyclas.notify_preference
loyaltyclas_review_status = loyaltyclas.review_status
loyaltyclas_id = loyaltyclas.id
loyaltyclas_rewards_tier = loyaltyclas.rewards_tier
loyaltyclas_version = loyaltyclas.version
loyaltyclas_account_name_label = loyaltyclas.account_name_label
loyaltyclas_issuer_name = loyaltyclas.issuer_name
loyaltyclas_enable_smart_tap = loyaltyclas.enable_smart_tap
loyaltyclas_locations = loyaltyclas.locations
loyaltyclas_image_modules_data = loyaltyclas.image_modules_data
loyaltyclas_messages = loyaltyclas.messages
loyaltyclas_value_added_module_data = loyaltyclas.value_added_module_data
loyaltyclas_localized_secondary_rewards_tier = loyaltyclas.localized_secondary_rewards_tier
loyaltyclas_localized_account_id_label = loyaltyclas.localized_account_id_label
loyaltyclas_localized_program_name = loyaltyclas.localized_program_name
loyaltyclas_homepage_uri = loyaltyclas.homepage_uri
loyaltyclas_localized_rewards_tier_label = loyaltyclas.localized_rewards_tier_label
loyaltyclas_word_mark = loyaltyclas.word_mark
loyaltyclas_view_unlock_requirement = loyaltyclas.view_unlock_requirement
loyaltyclas_wide_program_logo = loyaltyclas.wide_program_logo
loyaltyclas_info_module_data = loyaltyclas.info_module_data
loyaltyclas_merchant_locations = loyaltyclas.merchant_locations
loyaltyclas_security_animation = loyaltyclas.security_animation
loyaltyclas_links_module_data = loyaltyclas.links_module_data
loyaltyclas_rewards_tier_label = loyaltyclas.rewards_tier_label
loyaltyclas_country_code = loyaltyclas.country_code
loyaltyclas_localized_account_name_label = loyaltyclas.localized_account_name_label
loyaltyclas_callback_options = loyaltyclas.callback_options
```

---


### Issuer

Inserts an issuer with the given ID and properties.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `issuer_id` | String |  | The unique identifier for an issuer account. This is automatically generated when the issuer is inserted. |
| `name` | String |  | The account name of the issuer. |
| `smart_tap_merchant_data` | String |  | Available only to Smart Tap enabled partners. Contact support for additional guidance. |
| `contact_info` | String |  | Issuer contact information. |
| `callback_options` | String |  | Allows the issuer to provide their callback settings. |
| `homepage_url` | String |  | URL for the issuer's home page. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `issuer_id` | String | The unique identifier for an issuer account. This is automatically generated when the issuer is inserted. |
| `name` | String | The account name of the issuer. |
| `smart_tap_merchant_data` | String | Available only to Smart Tap enabled partners. Contact support for additional guidance. |
| `contact_info` | String | Issuer contact information. |
| `callback_options` | String | Allows the issuer to provide their callback settings. |
| `homepage_url` | String | URL for the issuer's home page. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create issuer
issuer = provider.walletobjects_api.Issuer {
}

# Access issuer outputs
issuer_id = issuer.id
issuer_issuer_id = issuer.issuer_id
issuer_name = issuer.name
issuer_smart_tap_merchant_data = issuer.smart_tap_merchant_data
issuer_contact_info = issuer.contact_info
issuer_callback_options = issuer.callback_options
issuer_homepage_url = issuer.homepage_url
```

---


### Eventticketclas

Inserts an event ticket class with the given ID and properties.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `app_link_data` | String |  | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding object that will be used instead. |
| `review` | String |  | The review comments set by the platform when a class is marked `approved` or `rejected`. |
| `allow_multiple_users_per_object` | bool |  | Deprecated. Use `multipleDevicesAndHoldersAllowedStatus` instead. |
| `venue` | String |  | Event venue details. |
| `class_template_info` | String |  | Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display. |
| `hex_background_color` | String |  | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `gate_label` | String |  | The label to use for the gate value (`eventTicketObject.seatInfo.gate`) on the card detail view. Each available option maps to a set of localized strings, so that translations are shown to the user based on their locale. Both `gateLabel` and `customGateLabel` may not be set. If neither is set, the label will default to "Gate", localized. If the gate field is unset, this label will not be used. |
| `date_time` | String |  | The date & time information of the event. |
| `id` | String |  | Required. The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `info_module_data` | String |  | Deprecated. Use textModulesData instead. |
| `custom_seat_label` | String |  | A custom label to use for the seat value (`eventTicketObject.seatInfo.seat`) on the card detail view. This should only be used if the default "Seat" label or one of the `seatLabel` options is not sufficient. Both `seatLabel` and `customSeatLabel` may not be set. If neither is set, the label will default to "Seat", localized. If the seat field is unset, this label will not be used. |
| `event_id` | String |  | The ID of the event. This ID should be unique for every event in an account. It is used to group tickets together if the user has saved multiple tickets for the same event. It can be at most 64 characters. If provided, the grouping will be stable. Be wary of unintentional collision to avoid grouping tickets that should not be grouped. If you use only one class per event, you can simply set this to the `classId` (with or without the issuer ID portion). If not provided, the platform will attempt to use other data to group tickets (potentially unstable). |
| `security_animation` | String |  | Optional information about the security animation. If this is set a security animation will be rendered on pass details. |
| `custom_row_label` | String |  | A custom label to use for the row value (`eventTicketObject.seatInfo.row`) on the card detail view. This should only be used if the default "Row" label or one of the `rowLabel` options is not sufficient. Both `rowLabel` and `customRowLabel` may not be set. If neither is set, the label will default to "Row", localized. If the row field is unset, this label will not be used. |
| `confirmation_code_label` | String |  | The label to use for the confirmation code value (`eventTicketObject.reservationInfo.confirmationCode`) on the card detail view. Each available option maps to a set of localized strings, so that translations are shown to the user based on their locale. Both `confirmationCodeLabel` and `customConfirmationCodeLabel` may not be set. If neither is set, the label will default to "Confirmation Code", localized. If the confirmation code field is unset, this label will not be used. |
| `hero_image` | String |  | Optional banner image displayed on the front of the card. If none is present, nothing will be displayed. The image will display at 100% width. |
| `messages` | Vec<String> |  | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `multiple_devices_and_holders_allowed_status` | String |  | Identifies whether multiple users and devices will save the same object referencing this class. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#eventTicketClass"`. |
| `notify_preference` | String |  | Whether or not field updates to this class should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If not specified, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `value_added_module_data` | Vec<String> |  | Optional value added module data. Maximum of ten on the class. For a pass only ten will be displayed, prioritizing those from the object. |
| `version` | String |  | Deprecated |
| `locations` | Vec<String> |  | Note: This field is currently not supported to trigger geo notifications. |
| `merchant_locations` | Vec<String> |  | Merchant locations. There is a maximum of ten on the class. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `row_label` | String |  | The label to use for the row value (`eventTicketObject.seatInfo.row`) on the card detail view. Each available option maps to a set of localized strings, so that translations are shown to the user based on their locale. Both `rowLabel` and `customRowLabel` may not be set. If neither is set, the label will default to "Row", localized. If the row field is unset, this label will not be used. |
| `enable_smart_tap` | bool |  | Identifies whether this class supports Smart Tap. The `redemptionIssuers` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `issuer_name` | String |  | Required. The issuer name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `review_status` | String |  | Required. The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`. |
| `custom_confirmation_code_label` | String |  | A custom label to use for the confirmation code value (`eventTicketObject.reservationInfo.confirmationCode`) on the card detail view. This should only be used if the default "Confirmation Code" label or one of the `confirmationCodeLabel` options is not sufficient. Both `confirmationCodeLabel` and `customConfirmationCodeLabel` may not be set. If neither is set, the label will default to "Confirmation Code", localized. If the confirmation code field is unset, this label will not be used. |
| `logo` | String |  | The logo image of the ticket. This image is displayed in the card detail view of the app. |
| `section_label` | String |  | The label to use for the section value (`eventTicketObject.seatInfo.section`) on the card detail view. Each available option maps to a set of localized strings, so that translations are shown to the user based on their locale. Both `sectionLabel` and `customSectionLabel` may not be set. If neither is set, the label will default to "Section", localized. If the section field is unset, this label will not be used. |
| `custom_section_label` | String |  | A custom label to use for the section value (`eventTicketObject.seatInfo.section`) on the card detail view. This should only be used if the default "Section" label or one of the `sectionLabel` options is not sufficient. Both `sectionLabel` and `customSectionLabel` may not be set. If neither is set, the label will default to "Section", localized. If the section field is unset, this label will not be used. |
| `word_mark` | String |  | Deprecated. |
| `custom_gate_label` | String |  | A custom label to use for the gate value (`eventTicketObject.seatInfo.gate`) on the card detail view. This should only be used if the default "Gate" label or one of the `gateLabel` options is not sufficient. Both `gateLabel` and `customGateLabel` may not be set. If neither is set, the label will default to "Gate", localized. If the gate field is unset, this label will not be used. |
| `text_modules_data` | Vec<String> |  | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `homepage_uri` | String |  | The URI of your application's home page. Populating the URI in this field results in the exact same behavior as populating an URI in linksModuleData (when an object is rendered, a link to the homepage is shown in what would usually be thought of as the linksModuleData section of the object). |
| `seat_label` | String |  | The label to use for the seat value (`eventTicketObject.seatInfo.seat`) on the card detail view. Each available option maps to a set of localized strings, so that translations are shown to the user based on their locale. Both `seatLabel` and `customSeatLabel` may not be set. If neither is set, the label will default to "Seat", localized. If the seat field is unset, this label will not be used. |
| `view_unlock_requirement` | String |  | View Unlock Requirement options for the event ticket. |
| `localized_issuer_name` | String |  | Translated strings for the issuer_name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `wide_logo` | String |  | The wide logo of the ticket. When provided, this will be used in place of the logo in the top left of the card view. |
| `callback_options` | String |  | Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback. |
| `country_code` | String |  | Country code used to display the card's country (when the user is not in that country), as well as to display localized content when content is not available in the user's locale. |
| `redemption_issuers` | Vec<String> |  | Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `event_name` | String |  | Required. The name of the event, such as "LA Dodgers at SF Giants". |
| `fine_print` | String |  | The fine print, terms, or conditions of the ticket. |
| `links_module_data` | String |  | Links module data. If links module data is also defined on the object, both will be displayed. |
| `image_modules_data` | Vec<String> |  | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `app_link_data` | String | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding object that will be used instead. |
| `review` | String | The review comments set by the platform when a class is marked `approved` or `rejected`. |
| `allow_multiple_users_per_object` | bool | Deprecated. Use `multipleDevicesAndHoldersAllowedStatus` instead. |
| `venue` | String | Event venue details. |
| `class_template_info` | String | Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display. |
| `hex_background_color` | String | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `gate_label` | String | The label to use for the gate value (`eventTicketObject.seatInfo.gate`) on the card detail view. Each available option maps to a set of localized strings, so that translations are shown to the user based on their locale. Both `gateLabel` and `customGateLabel` may not be set. If neither is set, the label will default to "Gate", localized. If the gate field is unset, this label will not be used. |
| `date_time` | String | The date & time information of the event. |
| `id` | String | Required. The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `info_module_data` | String | Deprecated. Use textModulesData instead. |
| `custom_seat_label` | String | A custom label to use for the seat value (`eventTicketObject.seatInfo.seat`) on the card detail view. This should only be used if the default "Seat" label or one of the `seatLabel` options is not sufficient. Both `seatLabel` and `customSeatLabel` may not be set. If neither is set, the label will default to "Seat", localized. If the seat field is unset, this label will not be used. |
| `event_id` | String | The ID of the event. This ID should be unique for every event in an account. It is used to group tickets together if the user has saved multiple tickets for the same event. It can be at most 64 characters. If provided, the grouping will be stable. Be wary of unintentional collision to avoid grouping tickets that should not be grouped. If you use only one class per event, you can simply set this to the `classId` (with or without the issuer ID portion). If not provided, the platform will attempt to use other data to group tickets (potentially unstable). |
| `security_animation` | String | Optional information about the security animation. If this is set a security animation will be rendered on pass details. |
| `custom_row_label` | String | A custom label to use for the row value (`eventTicketObject.seatInfo.row`) on the card detail view. This should only be used if the default "Row" label or one of the `rowLabel` options is not sufficient. Both `rowLabel` and `customRowLabel` may not be set. If neither is set, the label will default to "Row", localized. If the row field is unset, this label will not be used. |
| `confirmation_code_label` | String | The label to use for the confirmation code value (`eventTicketObject.reservationInfo.confirmationCode`) on the card detail view. Each available option maps to a set of localized strings, so that translations are shown to the user based on their locale. Both `confirmationCodeLabel` and `customConfirmationCodeLabel` may not be set. If neither is set, the label will default to "Confirmation Code", localized. If the confirmation code field is unset, this label will not be used. |
| `hero_image` | String | Optional banner image displayed on the front of the card. If none is present, nothing will be displayed. The image will display at 100% width. |
| `messages` | Vec<String> | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `multiple_devices_and_holders_allowed_status` | String | Identifies whether multiple users and devices will save the same object referencing this class. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#eventTicketClass"`. |
| `notify_preference` | String | Whether or not field updates to this class should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If not specified, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `value_added_module_data` | Vec<String> | Optional value added module data. Maximum of ten on the class. For a pass only ten will be displayed, prioritizing those from the object. |
| `version` | String | Deprecated |
| `locations` | Vec<String> | Note: This field is currently not supported to trigger geo notifications. |
| `merchant_locations` | Vec<String> | Merchant locations. There is a maximum of ten on the class. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `row_label` | String | The label to use for the row value (`eventTicketObject.seatInfo.row`) on the card detail view. Each available option maps to a set of localized strings, so that translations are shown to the user based on their locale. Both `rowLabel` and `customRowLabel` may not be set. If neither is set, the label will default to "Row", localized. If the row field is unset, this label will not be used. |
| `enable_smart_tap` | bool | Identifies whether this class supports Smart Tap. The `redemptionIssuers` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `issuer_name` | String | Required. The issuer name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `review_status` | String | Required. The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`. |
| `custom_confirmation_code_label` | String | A custom label to use for the confirmation code value (`eventTicketObject.reservationInfo.confirmationCode`) on the card detail view. This should only be used if the default "Confirmation Code" label or one of the `confirmationCodeLabel` options is not sufficient. Both `confirmationCodeLabel` and `customConfirmationCodeLabel` may not be set. If neither is set, the label will default to "Confirmation Code", localized. If the confirmation code field is unset, this label will not be used. |
| `logo` | String | The logo image of the ticket. This image is displayed in the card detail view of the app. |
| `section_label` | String | The label to use for the section value (`eventTicketObject.seatInfo.section`) on the card detail view. Each available option maps to a set of localized strings, so that translations are shown to the user based on their locale. Both `sectionLabel` and `customSectionLabel` may not be set. If neither is set, the label will default to "Section", localized. If the section field is unset, this label will not be used. |
| `custom_section_label` | String | A custom label to use for the section value (`eventTicketObject.seatInfo.section`) on the card detail view. This should only be used if the default "Section" label or one of the `sectionLabel` options is not sufficient. Both `sectionLabel` and `customSectionLabel` may not be set. If neither is set, the label will default to "Section", localized. If the section field is unset, this label will not be used. |
| `word_mark` | String | Deprecated. |
| `custom_gate_label` | String | A custom label to use for the gate value (`eventTicketObject.seatInfo.gate`) on the card detail view. This should only be used if the default "Gate" label or one of the `gateLabel` options is not sufficient. Both `gateLabel` and `customGateLabel` may not be set. If neither is set, the label will default to "Gate", localized. If the gate field is unset, this label will not be used. |
| `text_modules_data` | Vec<String> | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `homepage_uri` | String | The URI of your application's home page. Populating the URI in this field results in the exact same behavior as populating an URI in linksModuleData (when an object is rendered, a link to the homepage is shown in what would usually be thought of as the linksModuleData section of the object). |
| `seat_label` | String | The label to use for the seat value (`eventTicketObject.seatInfo.seat`) on the card detail view. Each available option maps to a set of localized strings, so that translations are shown to the user based on their locale. Both `seatLabel` and `customSeatLabel` may not be set. If neither is set, the label will default to "Seat", localized. If the seat field is unset, this label will not be used. |
| `view_unlock_requirement` | String | View Unlock Requirement options for the event ticket. |
| `localized_issuer_name` | String | Translated strings for the issuer_name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `wide_logo` | String | The wide logo of the ticket. When provided, this will be used in place of the logo in the top left of the card view. |
| `callback_options` | String | Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback. |
| `country_code` | String | Country code used to display the card's country (when the user is not in that country), as well as to display localized content when content is not available in the user's locale. |
| `redemption_issuers` | Vec<String> | Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `event_name` | String | Required. The name of the event, such as "LA Dodgers at SF Giants". |
| `fine_print` | String | The fine print, terms, or conditions of the ticket. |
| `links_module_data` | String | Links module data. If links module data is also defined on the object, both will be displayed. |
| `image_modules_data` | Vec<String> | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create eventticketclas
eventticketclas = provider.walletobjects_api.Eventticketclas {
}

# Access eventticketclas outputs
eventticketclas_id = eventticketclas.id
eventticketclas_app_link_data = eventticketclas.app_link_data
eventticketclas_review = eventticketclas.review
eventticketclas_allow_multiple_users_per_object = eventticketclas.allow_multiple_users_per_object
eventticketclas_venue = eventticketclas.venue
eventticketclas_class_template_info = eventticketclas.class_template_info
eventticketclas_hex_background_color = eventticketclas.hex_background_color
eventticketclas_gate_label = eventticketclas.gate_label
eventticketclas_date_time = eventticketclas.date_time
eventticketclas_id = eventticketclas.id
eventticketclas_info_module_data = eventticketclas.info_module_data
eventticketclas_custom_seat_label = eventticketclas.custom_seat_label
eventticketclas_event_id = eventticketclas.event_id
eventticketclas_security_animation = eventticketclas.security_animation
eventticketclas_custom_row_label = eventticketclas.custom_row_label
eventticketclas_confirmation_code_label = eventticketclas.confirmation_code_label
eventticketclas_hero_image = eventticketclas.hero_image
eventticketclas_messages = eventticketclas.messages
eventticketclas_multiple_devices_and_holders_allowed_status = eventticketclas.multiple_devices_and_holders_allowed_status
eventticketclas_kind = eventticketclas.kind
eventticketclas_notify_preference = eventticketclas.notify_preference
eventticketclas_value_added_module_data = eventticketclas.value_added_module_data
eventticketclas_version = eventticketclas.version
eventticketclas_locations = eventticketclas.locations
eventticketclas_merchant_locations = eventticketclas.merchant_locations
eventticketclas_row_label = eventticketclas.row_label
eventticketclas_enable_smart_tap = eventticketclas.enable_smart_tap
eventticketclas_issuer_name = eventticketclas.issuer_name
eventticketclas_review_status = eventticketclas.review_status
eventticketclas_custom_confirmation_code_label = eventticketclas.custom_confirmation_code_label
eventticketclas_logo = eventticketclas.logo
eventticketclas_section_label = eventticketclas.section_label
eventticketclas_custom_section_label = eventticketclas.custom_section_label
eventticketclas_word_mark = eventticketclas.word_mark
eventticketclas_custom_gate_label = eventticketclas.custom_gate_label
eventticketclas_text_modules_data = eventticketclas.text_modules_data
eventticketclas_homepage_uri = eventticketclas.homepage_uri
eventticketclas_seat_label = eventticketclas.seat_label
eventticketclas_view_unlock_requirement = eventticketclas.view_unlock_requirement
eventticketclas_localized_issuer_name = eventticketclas.localized_issuer_name
eventticketclas_wide_logo = eventticketclas.wide_logo
eventticketclas_callback_options = eventticketclas.callback_options
eventticketclas_country_code = eventticketclas.country_code
eventticketclas_redemption_issuers = eventticketclas.redemption_issuers
eventticketclas_event_name = eventticketclas.event_name
eventticketclas_fine_print = eventticketclas.fine_print
eventticketclas_links_module_data = eventticketclas.links_module_data
eventticketclas_image_modules_data = eventticketclas.image_modules_data
```

---


### Jwt

Inserts the resources in the JWT.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `jwt` | String |  | A string representing a JWT of the format described at https://developers.google.com/wallet/reference/rest/v1/Jwt |



#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create jwt
jwt = provider.walletobjects_api.Jwt {
}

```

---


### Flightclas

Inserts an flight class with the given ID and properties.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `hero_image` | String |  | Optional banner image displayed on the front of the card. If none is present, nothing will be displayed. The image will display at 100% width. |
| `localized_issuer_name` | String |  | Translated strings for the issuer_name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `text_modules_data` | Vec<String> |  | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `enable_smart_tap` | bool |  | Identifies whether this class supports Smart Tap. The `redemptionIssuers` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `multiple_devices_and_holders_allowed_status` | String |  | Identifies whether multiple users and devices will save the same object referencing this class. |
| `destination` | String |  | Required. Destination airport. |
| `class_template_info` | String |  | Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display. |
| `view_unlock_requirement` | String |  | View Unlock Requirement options for the boarding pass. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#flightClass"`. |
| `merchant_locations` | Vec<String> |  | Merchant locations. There is a maximum of ten on the class. Any additional MerchantLocations added beyond the 10 will be rejected by the validator. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `local_estimated_or_actual_arrival_date_time` | String |  | The estimated time the aircraft plans to reach the destination gate (not the runway) or the actual time it reached the gate. This field should be set if at least one of the below is true: - It differs from the scheduled time. Google will use it to calculate the delay. - The aircraft already arrived at the gate. Google will use it to inform the user that the flight has arrived at the gate. This is an ISO 8601 extended format date/time without an offset. Time may be specified up to millisecond precision. eg: `2027-03-05T06:30:00` This should be the local date/time at the airport (not a UTC time). Google will reject the request if UTC offset is provided. Time zones will be calculated by Google based on arrival airport. |
| `notify_preference` | String |  | Whether or not field updates to this class should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If not specified, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `hex_background_color` | String |  | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `review_status` | String |  | Required. The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`. |
| `flight_status` | String |  | Status of this flight. If unset, Google will compute status based on data from other sources, such as FlightStats, etc. Note: Google-computed status will not be returned in API responses. |
| `flight_header` | String |  | Required. Information about the flight carrier and number. |
| `word_mark` | String |  | Deprecated. |
| `origin` | String |  | Required. Origin airport. |
| `local_scheduled_departure_date_time` | String |  | Required. The scheduled date and time when the aircraft is expected to depart the gate (not the runway) Note: This field should not change too close to the departure time. For updates to departure times (delays, etc), please set `localEstimatedOrActualDepartureDateTime`. This is an ISO 8601 extended format date/time without an offset. Time may be specified up to millisecond precision. eg: `2027-03-05T06:30:00` This should be the local date/time at the airport (not a UTC time). Google will reject the request if UTC offset is provided. Time zones will be calculated by Google based on departure airport. |
| `boarding_and_seating_policy` | String |  | Policies for boarding and seating. These will inform which labels will be shown to users. |
| `security_animation` | String |  | Optional information about the security animation. If this is set a security animation will be rendered on pass details. |
| `locations` | Vec<String> |  | Note: This field is currently not supported to trigger geo notifications. |
| `issuer_name` | String |  | Required. The issuer name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `links_module_data` | String |  | Links module data. If links module data is also defined on the object, both will be displayed. |
| `homepage_uri` | String |  | The URI of your application's home page. Populating the URI in this field results in the exact same behavior as populating an URI in linksModuleData (when an object is rendered, a link to the homepage is shown in what would usually be thought of as the linksModuleData section of the object). |
| `id` | String |  | Required. The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `local_gate_closing_date_time` | String |  | The gate closing time as it would be printed on the boarding pass. Do not set this field if you do not want to print it in the boarding pass. This is an ISO 8601 extended format date/time without an offset. Time may be specified up to millisecond precision. eg: `2027-03-05T06:30:00` This should be the local date/time at the airport (not a UTC time). Google will reject the request if UTC offset is provided. Time zones will be calculated by Google based on departure airport. |
| `local_estimated_or_actual_departure_date_time` | String |  | The estimated time the aircraft plans to pull from the gate or the actual time the aircraft already pulled from the gate. Note: This is not the runway time. This field should be set if at least one of the below is true: - It differs from the scheduled time. Google will use it to calculate the delay. - The aircraft already pulled from the gate. Google will use it to inform the user when the flight actually departed. This is an ISO 8601 extended format date/time without an offset. Time may be specified up to millisecond precision. eg: `2027-03-05T06:30:00` This should be the local date/time at the airport (not a UTC time). Google will reject the request if UTC offset is provided. Time zones will be calculated by Google based on departure airport. |
| `redemption_issuers` | Vec<String> |  | Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `version` | String |  | Deprecated |
| `messages` | Vec<String> |  | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `country_code` | String |  | Country code used to display the card's country (when the user is not in that country), as well as to display localized content when content is not available in the user's locale. |
| `image_modules_data` | Vec<String> |  | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `local_scheduled_arrival_date_time` | String |  | The scheduled time the aircraft plans to reach the destination gate (not the runway). Note: This field should not change too close to the flight time. For updates to departure times (delays, etc), please set `localEstimatedOrActualArrivalDateTime`. This is an ISO 8601 extended format date/time without an offset. Time may be specified up to millisecond precision. eg: `2027-03-05T06:30:00` This should be the local date/time at the airport (not a UTC time). Google will reject the request if UTC offset is provided. Time zones will be calculated by Google based on arrival airport. |
| `allow_multiple_users_per_object` | bool |  | Deprecated. Use `multipleDevicesAndHoldersAllowedStatus` instead. |
| `language_override` | String |  | If this field is present, boarding passes served to a user's device will always be in this language. Represents the BCP 47 language tag. Example values are "en-US", "en-GB", "de", or "de-AT". |
| `info_module_data` | String |  | Deprecated. Use textModulesData instead. |
| `app_link_data` | String |  | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding object that will be used instead. |
| `local_boarding_date_time` | String |  | The boarding time as it would be printed on the boarding pass. This is an ISO 8601 extended format date/time without an offset. Time may be specified up to millisecond precision. eg: `2027-03-05T06:30:00` This should be the local date/time at the airport (not a UTC time). Google will reject the request if UTC offset is provided. Time zones will be calculated by Google based on departure airport. |
| `callback_options` | String |  | Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback. |
| `value_added_module_data` | Vec<String> |  | Optional value added module data. Maximum of ten on the class. For a pass only ten will be displayed, prioritizing those from the object. |
| `review` | String |  | The review comments set by the platform when a class is marked `approved` or `rejected`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `hero_image` | String | Optional banner image displayed on the front of the card. If none is present, nothing will be displayed. The image will display at 100% width. |
| `localized_issuer_name` | String | Translated strings for the issuer_name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `text_modules_data` | Vec<String> | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `enable_smart_tap` | bool | Identifies whether this class supports Smart Tap. The `redemptionIssuers` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `multiple_devices_and_holders_allowed_status` | String | Identifies whether multiple users and devices will save the same object referencing this class. |
| `destination` | String | Required. Destination airport. |
| `class_template_info` | String | Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display. |
| `view_unlock_requirement` | String | View Unlock Requirement options for the boarding pass. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#flightClass"`. |
| `merchant_locations` | Vec<String> | Merchant locations. There is a maximum of ten on the class. Any additional MerchantLocations added beyond the 10 will be rejected by the validator. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `local_estimated_or_actual_arrival_date_time` | String | The estimated time the aircraft plans to reach the destination gate (not the runway) or the actual time it reached the gate. This field should be set if at least one of the below is true: - It differs from the scheduled time. Google will use it to calculate the delay. - The aircraft already arrived at the gate. Google will use it to inform the user that the flight has arrived at the gate. This is an ISO 8601 extended format date/time without an offset. Time may be specified up to millisecond precision. eg: `2027-03-05T06:30:00` This should be the local date/time at the airport (not a UTC time). Google will reject the request if UTC offset is provided. Time zones will be calculated by Google based on arrival airport. |
| `notify_preference` | String | Whether or not field updates to this class should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If not specified, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `hex_background_color` | String | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `review_status` | String | Required. The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`. |
| `flight_status` | String | Status of this flight. If unset, Google will compute status based on data from other sources, such as FlightStats, etc. Note: Google-computed status will not be returned in API responses. |
| `flight_header` | String | Required. Information about the flight carrier and number. |
| `word_mark` | String | Deprecated. |
| `origin` | String | Required. Origin airport. |
| `local_scheduled_departure_date_time` | String | Required. The scheduled date and time when the aircraft is expected to depart the gate (not the runway) Note: This field should not change too close to the departure time. For updates to departure times (delays, etc), please set `localEstimatedOrActualDepartureDateTime`. This is an ISO 8601 extended format date/time without an offset. Time may be specified up to millisecond precision. eg: `2027-03-05T06:30:00` This should be the local date/time at the airport (not a UTC time). Google will reject the request if UTC offset is provided. Time zones will be calculated by Google based on departure airport. |
| `boarding_and_seating_policy` | String | Policies for boarding and seating. These will inform which labels will be shown to users. |
| `security_animation` | String | Optional information about the security animation. If this is set a security animation will be rendered on pass details. |
| `locations` | Vec<String> | Note: This field is currently not supported to trigger geo notifications. |
| `issuer_name` | String | Required. The issuer name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `links_module_data` | String | Links module data. If links module data is also defined on the object, both will be displayed. |
| `homepage_uri` | String | The URI of your application's home page. Populating the URI in this field results in the exact same behavior as populating an URI in linksModuleData (when an object is rendered, a link to the homepage is shown in what would usually be thought of as the linksModuleData section of the object). |
| `id` | String | Required. The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `local_gate_closing_date_time` | String | The gate closing time as it would be printed on the boarding pass. Do not set this field if you do not want to print it in the boarding pass. This is an ISO 8601 extended format date/time without an offset. Time may be specified up to millisecond precision. eg: `2027-03-05T06:30:00` This should be the local date/time at the airport (not a UTC time). Google will reject the request if UTC offset is provided. Time zones will be calculated by Google based on departure airport. |
| `local_estimated_or_actual_departure_date_time` | String | The estimated time the aircraft plans to pull from the gate or the actual time the aircraft already pulled from the gate. Note: This is not the runway time. This field should be set if at least one of the below is true: - It differs from the scheduled time. Google will use it to calculate the delay. - The aircraft already pulled from the gate. Google will use it to inform the user when the flight actually departed. This is an ISO 8601 extended format date/time without an offset. Time may be specified up to millisecond precision. eg: `2027-03-05T06:30:00` This should be the local date/time at the airport (not a UTC time). Google will reject the request if UTC offset is provided. Time zones will be calculated by Google based on departure airport. |
| `redemption_issuers` | Vec<String> | Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `version` | String | Deprecated |
| `messages` | Vec<String> | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `country_code` | String | Country code used to display the card's country (when the user is not in that country), as well as to display localized content when content is not available in the user's locale. |
| `image_modules_data` | Vec<String> | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `local_scheduled_arrival_date_time` | String | The scheduled time the aircraft plans to reach the destination gate (not the runway). Note: This field should not change too close to the flight time. For updates to departure times (delays, etc), please set `localEstimatedOrActualArrivalDateTime`. This is an ISO 8601 extended format date/time without an offset. Time may be specified up to millisecond precision. eg: `2027-03-05T06:30:00` This should be the local date/time at the airport (not a UTC time). Google will reject the request if UTC offset is provided. Time zones will be calculated by Google based on arrival airport. |
| `allow_multiple_users_per_object` | bool | Deprecated. Use `multipleDevicesAndHoldersAllowedStatus` instead. |
| `language_override` | String | If this field is present, boarding passes served to a user's device will always be in this language. Represents the BCP 47 language tag. Example values are "en-US", "en-GB", "de", or "de-AT". |
| `info_module_data` | String | Deprecated. Use textModulesData instead. |
| `app_link_data` | String | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding object that will be used instead. |
| `local_boarding_date_time` | String | The boarding time as it would be printed on the boarding pass. This is an ISO 8601 extended format date/time without an offset. Time may be specified up to millisecond precision. eg: `2027-03-05T06:30:00` This should be the local date/time at the airport (not a UTC time). Google will reject the request if UTC offset is provided. Time zones will be calculated by Google based on departure airport. |
| `callback_options` | String | Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback. |
| `value_added_module_data` | Vec<String> | Optional value added module data. Maximum of ten on the class. For a pass only ten will be displayed, prioritizing those from the object. |
| `review` | String | The review comments set by the platform when a class is marked `approved` or `rejected`. |


#### Usage Example

```kcl
# main.k
import gcp

# Initialize provider
provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create flightclas
flightclas = provider.walletobjects_api.Flightclas {
}

# Access flightclas outputs
flightclas_id = flightclas.id
flightclas_hero_image = flightclas.hero_image
flightclas_localized_issuer_name = flightclas.localized_issuer_name
flightclas_text_modules_data = flightclas.text_modules_data
flightclas_enable_smart_tap = flightclas.enable_smart_tap
flightclas_multiple_devices_and_holders_allowed_status = flightclas.multiple_devices_and_holders_allowed_status
flightclas_destination = flightclas.destination
flightclas_class_template_info = flightclas.class_template_info
flightclas_view_unlock_requirement = flightclas.view_unlock_requirement
flightclas_kind = flightclas.kind
flightclas_merchant_locations = flightclas.merchant_locations
flightclas_local_estimated_or_actual_arrival_date_time = flightclas.local_estimated_or_actual_arrival_date_time
flightclas_notify_preference = flightclas.notify_preference
flightclas_hex_background_color = flightclas.hex_background_color
flightclas_review_status = flightclas.review_status
flightclas_flight_status = flightclas.flight_status
flightclas_flight_header = flightclas.flight_header
flightclas_word_mark = flightclas.word_mark
flightclas_origin = flightclas.origin
flightclas_local_scheduled_departure_date_time = flightclas.local_scheduled_departure_date_time
flightclas_boarding_and_seating_policy = flightclas.boarding_and_seating_policy
flightclas_security_animation = flightclas.security_animation
flightclas_locations = flightclas.locations
flightclas_issuer_name = flightclas.issuer_name
flightclas_links_module_data = flightclas.links_module_data
flightclas_homepage_uri = flightclas.homepage_uri
flightclas_id = flightclas.id
flightclas_local_gate_closing_date_time = flightclas.local_gate_closing_date_time
flightclas_local_estimated_or_actual_departure_date_time = flightclas.local_estimated_or_actual_departure_date_time
flightclas_redemption_issuers = flightclas.redemption_issuers
flightclas_version = flightclas.version
flightclas_messages = flightclas.messages
flightclas_country_code = flightclas.country_code
flightclas_image_modules_data = flightclas.image_modules_data
flightclas_local_scheduled_arrival_date_time = flightclas.local_scheduled_arrival_date_time
flightclas_allow_multiple_users_per_object = flightclas.allow_multiple_users_per_object
flightclas_language_override = flightclas.language_override
flightclas_info_module_data = flightclas.info_module_data
flightclas_app_link_data = flightclas.app_link_data
flightclas_local_boarding_date_time = flightclas.local_boarding_date_time
flightclas_callback_options = flightclas.callback_options
flightclas_value_added_module_data = flightclas.value_added_module_data
flightclas_review = flightclas.review
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple genericclas resources
genericclas_0 = provider.walletobjects_api.Genericclas {
}
genericclas_1 = provider.walletobjects_api.Genericclas {
}
genericclas_2 = provider.walletobjects_api.Genericclas {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    genericclas = provider.walletobjects_api.Genericclas {
    }
```

---

## Related Documentation

- [GCP Walletobjects_api Documentation](https://cloud.google.com/walletobjects_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
