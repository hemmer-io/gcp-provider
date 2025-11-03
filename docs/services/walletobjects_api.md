# Walletobjects_api Service



**Resources**: 20

---

## Overview

The walletobjects_api service provides access to 20 resource types:

- [Loyaltyobject](#loyaltyobject) [CRU]
- [Genericobject](#genericobject) [CRU]
- [Media](#media) [CR]
- [Permission](#permission) [RU]
- [Flightobject](#flightobject) [CRU]
- [Offerclas](#offerclas) [CRU]
- [Eventticketobject](#eventticketobject) [CRU]
- [Issuer](#issuer) [CRU]
- [Smarttap](#smarttap) [C]
- [Eventticketclas](#eventticketclas) [CRU]
- [Transitclas](#transitclas) [CRU]
- [Jwt](#jwt) [C]
- [Giftcardobject](#giftcardobject) [CRU]
- [Transitobject](#transitobject) [CRU]
- [Flightclas](#flightclas) [CRU]
- [Genericclas](#genericclas) [CRU]
- [Offerobject](#offerobject) [CRU]
- [Giftcardclas](#giftcardclas) [CRU]
- [Private_content](#private_content) [C]
- [Loyaltyclas](#loyaltyclas) [CRU]

---

## Resources


### Loyaltyobject

Inserts an loyalty object with the given ID and properties.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `class_reference` | String |  | A copy of the inherited fields of the parent class. These fields are retrieved during a GET. |
| `hero_image` | String |  | Optional banner image displayed on the front of the card. If none is present, hero image of the class, if present, will be displayed. If hero image of the class is also not present, nothing will be displayed. |
| `disable_expiration_notification` | bool |  | Indicates if notifications should explicitly be suppressed. If this field is set to true, regardless of the `messages` field, expiration notifications to the user will be suppressed. By default, this field is set to false. Currently, this can only be set for offers. |
| `info_module_data` | String |  | Deprecated. Use textModulesData instead. |
| `linked_offer_ids` | Vec<String> |  | A list of offer objects linked to this loyalty card. The offer objects must already exist. Offer object IDs should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. |
| `notify_preference` | String |  | Whether or not field updates to this object should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If set to DO_NOT_NOTIFY or NOTIFICATION_SETTINGS_UNSPECIFIED, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `smart_tap_redemption_value` | String |  | The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported. If this value is not set but the class level fields `enableSmartTap` and `redemptionIssuers` are set up correctly, the `barcode.value` or the `accountId` fields are used as fallback if present. |
| `grouping_info` | String |  | Information that controls how passes are grouped together. |
| `links_module_data` | String |  | Links module data. If links module data is also defined on the class, both will be displayed. |
| `locations` | Vec<String> |  | Note: This field is currently not supported to trigger geo notifications. |
| `valid_time_interval` | String |  | The time period this object will be `active` and object can be used. An object's state will be changed to `expired` when this time period has passed. |
| `class_id` | String |  | Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. |
| `image_modules_data` | Vec<String> |  | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `pass_constraints` | String |  | Pass constraints for the object. Includes limiting NFC and screenshot behaviors. |
| `state` | String |  | Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section. |
| `barcode` | String |  | The barcode type and value. |
| `value_added_module_data` | Vec<String> |  | Optional value added module data. Maximum of ten on the object. |
| `account_id` | String |  | The loyalty account identifier. Recommended maximum length is 20 characters. |
| `has_users` | bool |  | Indicates if the object has users. This field is set by the platform. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#loyaltyObject"`. |
| `messages` | Vec<String> |  | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `save_restrictions` | String |  | Restrictions on the object that needs to be verified before the user tries to save the pass. Note that this restrictions will only be applied during save time. If the restrictions changed after a user saves the pass, the new restrictions will not be applied to an already saved pass. |
| `has_linked_device` | bool |  | Whether this object is currently linked to a single device. This field is set by the platform when a user saves the object, linking it to their device. Intended for use by select partners. Contact support for additional information. |
| `rotating_barcode` | String |  | The rotating barcode type and value. |
| `app_link_data` | String |  | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding class only object AppLinkData will be displayed. |
| `secondary_loyalty_points` | String |  | The secondary loyalty reward points label, balance, and type. Shown in addition to the primary loyalty points. |
| `id` | String |  | Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. The unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `loyalty_points` | String |  | The loyalty reward points label, balance, and type. |
| `merchant_locations` | Vec<String> |  | Merchant locations. There is a maximum of ten on the object. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `text_modules_data` | Vec<String> |  | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `linked_object_ids` | Vec<String> |  | linked_object_ids are a list of other objects such as event ticket, loyalty, offer, generic, giftcard, transit and boarding pass that should be automatically attached to this loyalty object. If a user had saved this loyalty card, then these linked_object_ids would be automatically pushed to the user's wallet (unless they turned off the setting to receive such linked passes). Make sure that objects present in linked_object_ids are already inserted - if not, calls would fail. Once linked, the linked objects cannot be unlinked. You cannot link objects belonging to another issuer. There is a limit to the number of objects that can be linked to a single object. After the limit is reached, new linked objects in the call will be ignored silently. Object IDs should follow the format issuer ID. identifier where the former is issued by Google and the latter is chosen by you. |
| `account_name` | String |  | The loyalty account holder name, such as "John Smith." Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `version` | String |  | Deprecated |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `class_reference` | String | A copy of the inherited fields of the parent class. These fields are retrieved during a GET. |
| `hero_image` | String | Optional banner image displayed on the front of the card. If none is present, hero image of the class, if present, will be displayed. If hero image of the class is also not present, nothing will be displayed. |
| `disable_expiration_notification` | bool | Indicates if notifications should explicitly be suppressed. If this field is set to true, regardless of the `messages` field, expiration notifications to the user will be suppressed. By default, this field is set to false. Currently, this can only be set for offers. |
| `info_module_data` | String | Deprecated. Use textModulesData instead. |
| `linked_offer_ids` | Vec<String> | A list of offer objects linked to this loyalty card. The offer objects must already exist. Offer object IDs should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. |
| `notify_preference` | String | Whether or not field updates to this object should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If set to DO_NOT_NOTIFY or NOTIFICATION_SETTINGS_UNSPECIFIED, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `smart_tap_redemption_value` | String | The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported. If this value is not set but the class level fields `enableSmartTap` and `redemptionIssuers` are set up correctly, the `barcode.value` or the `accountId` fields are used as fallback if present. |
| `grouping_info` | String | Information that controls how passes are grouped together. |
| `links_module_data` | String | Links module data. If links module data is also defined on the class, both will be displayed. |
| `locations` | Vec<String> | Note: This field is currently not supported to trigger geo notifications. |
| `valid_time_interval` | String | The time period this object will be `active` and object can be used. An object's state will be changed to `expired` when this time period has passed. |
| `class_id` | String | Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. |
| `image_modules_data` | Vec<String> | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `pass_constraints` | String | Pass constraints for the object. Includes limiting NFC and screenshot behaviors. |
| `state` | String | Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section. |
| `barcode` | String | The barcode type and value. |
| `value_added_module_data` | Vec<String> | Optional value added module data. Maximum of ten on the object. |
| `account_id` | String | The loyalty account identifier. Recommended maximum length is 20 characters. |
| `has_users` | bool | Indicates if the object has users. This field is set by the platform. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#loyaltyObject"`. |
| `messages` | Vec<String> | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `save_restrictions` | String | Restrictions on the object that needs to be verified before the user tries to save the pass. Note that this restrictions will only be applied during save time. If the restrictions changed after a user saves the pass, the new restrictions will not be applied to an already saved pass. |
| `has_linked_device` | bool | Whether this object is currently linked to a single device. This field is set by the platform when a user saves the object, linking it to their device. Intended for use by select partners. Contact support for additional information. |
| `rotating_barcode` | String | The rotating barcode type and value. |
| `app_link_data` | String | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding class only object AppLinkData will be displayed. |
| `secondary_loyalty_points` | String | The secondary loyalty reward points label, balance, and type. Shown in addition to the primary loyalty points. |
| `id` | String | Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. The unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `loyalty_points` | String | The loyalty reward points label, balance, and type. |
| `merchant_locations` | Vec<String> | Merchant locations. There is a maximum of ten on the object. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `text_modules_data` | Vec<String> | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `linked_object_ids` | Vec<String> | linked_object_ids are a list of other objects such as event ticket, loyalty, offer, generic, giftcard, transit and boarding pass that should be automatically attached to this loyalty object. If a user had saved this loyalty card, then these linked_object_ids would be automatically pushed to the user's wallet (unless they turned off the setting to receive such linked passes). Make sure that objects present in linked_object_ids are already inserted - if not, calls would fail. Once linked, the linked objects cannot be unlinked. You cannot link objects belonging to another issuer. There is a limit to the number of objects that can be linked to a single object. After the limit is reached, new linked objects in the call will be ignored silently. Object IDs should follow the format issuer ID. identifier where the former is issued by Google and the latter is chosen by you. |
| `account_name` | String | The loyalty account holder name, such as "John Smith." Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `version` | String | Deprecated |


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
loyaltyobject_class_reference = loyaltyobject.class_reference
loyaltyobject_hero_image = loyaltyobject.hero_image
loyaltyobject_disable_expiration_notification = loyaltyobject.disable_expiration_notification
loyaltyobject_info_module_data = loyaltyobject.info_module_data
loyaltyobject_linked_offer_ids = loyaltyobject.linked_offer_ids
loyaltyobject_notify_preference = loyaltyobject.notify_preference
loyaltyobject_smart_tap_redemption_value = loyaltyobject.smart_tap_redemption_value
loyaltyobject_grouping_info = loyaltyobject.grouping_info
loyaltyobject_links_module_data = loyaltyobject.links_module_data
loyaltyobject_locations = loyaltyobject.locations
loyaltyobject_valid_time_interval = loyaltyobject.valid_time_interval
loyaltyobject_class_id = loyaltyobject.class_id
loyaltyobject_image_modules_data = loyaltyobject.image_modules_data
loyaltyobject_pass_constraints = loyaltyobject.pass_constraints
loyaltyobject_state = loyaltyobject.state
loyaltyobject_barcode = loyaltyobject.barcode
loyaltyobject_value_added_module_data = loyaltyobject.value_added_module_data
loyaltyobject_account_id = loyaltyobject.account_id
loyaltyobject_has_users = loyaltyobject.has_users
loyaltyobject_kind = loyaltyobject.kind
loyaltyobject_messages = loyaltyobject.messages
loyaltyobject_save_restrictions = loyaltyobject.save_restrictions
loyaltyobject_has_linked_device = loyaltyobject.has_linked_device
loyaltyobject_rotating_barcode = loyaltyobject.rotating_barcode
loyaltyobject_app_link_data = loyaltyobject.app_link_data
loyaltyobject_secondary_loyalty_points = loyaltyobject.secondary_loyalty_points
loyaltyobject_id = loyaltyobject.id
loyaltyobject_loyalty_points = loyaltyobject.loyalty_points
loyaltyobject_merchant_locations = loyaltyobject.merchant_locations
loyaltyobject_text_modules_data = loyaltyobject.text_modules_data
loyaltyobject_linked_object_ids = loyaltyobject.linked_object_ids
loyaltyobject_account_name = loyaltyobject.account_name
loyaltyobject_version = loyaltyobject.version
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
| `subheader` | String |  | The title label of the pass, such as location where this pass can be used. Appears right above the title in the title row in the pass detail view. |
| `text_modules_data` | Vec<String> |  | Text module data. If `textModulesData` is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from class and 10 from object. |
| `id` | String |  | Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value needs to follow the format `issuerID.identifier` where `issuerID` is issued by Google and `identifier` is chosen by you. The unique identifier can only include alphanumeric characters, `.`, `_`, or `-`. |
| `value_added_module_data` | Vec<String> |  | Optional value added module data. Maximum of ten on the object. |
| `linked_object_ids` | Vec<String> |  | linked_object_ids are a list of other objects such as event ticket, loyalty, offer, generic, giftcard, transit and boarding pass that should be automatically attached to this generic object. If a user had saved this generic card, then these linked_object_ids would be automatically pushed to the user's wallet (unless they turned off the setting to receive such linked passes). Make sure that objects present in linked_object_ids are already inserted - if not, calls would fail. Once linked, the linked objects cannot be unlinked. You cannot link objects belonging to another issuer. There is a limit to the number of objects that can be linked to a single object. After the limit is reached, new linked objects in the call will be ignored silently. Object IDs should follow the format issuer ID. identifier where the former is issued by Google and the latter is chosen by you. |
| `generic_type` | String |  | Specify which `GenericType` the card belongs to. |
| `header` | String |  | Required. The title of the pass, such as "50% off coupon" or "Library card" or "Voucher". This field is required and appears in the title row of the pass detail view. |
| `merchant_locations` | Vec<String> |  | Merchant locations. There is a maximum of ten on the object. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `hex_background_color` | String |  | The background color for the card. If not set, the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used and if logo is not set, a color would be chosen by Google. |
| `links_module_data` | String |  | Links module data. If `linksModuleData` is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from class and 10 from object. |
| `card_title` | String |  | Required. The header of the pass. This is usually the Business name such as "XXX Gym", "AAA Insurance". This field is required and appears in the header row at the very top of the pass. |
| `grouping_info` | String |  | Information that controls how passes are grouped together. |
| `app_link_data` | String |  | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding class only object AppLinkData will be displayed. |
| `class_id` | String |  | Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format `issuerID.identifier` where `issuerID` is issued by Google and `identifier` is chosen by you. |
| `logo` | String |  | The logo image of the pass. This image is displayed in the card detail view in upper left, and also on the list/thumbnail view. If the logo is not present, the first letter of `cardTitle` would be shown as logo. |
| `smart_tap_redemption_value` | String |  | The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported. |
| `pass_constraints` | String |  | Pass constraints for the object. Includes limiting NFC and screenshot behaviors. |
| `valid_time_interval` | String |  | The time period this object will be considered valid or usable. When the time period is passed, the object will be considered expired, which will affect the rendering on user's devices. |
| `state` | String |  | The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section. If this is not provided, the object would be considered `ACTIVE`. |
| `save_restrictions` | String |  | Restrictions on the object that needs to be verified before the user tries to save the pass. Note that this restrictions will only be applied during save time. If the restrictions changed after a user saves the pass, the new restrictions will not be applied to an already saved pass. |
| `barcode` | String |  | The barcode type and value. If pass does not have a barcode, we can allow the issuer to set Barcode.alternate_text and display just that. |
| `has_users` | bool |  | Indicates if the object has users. This field is set by the platform. |
| `wide_logo` | String |  | The wide logo of the pass. When provided, this will be used in place of the logo in the top left of the card view. |
| `notifications` | String |  | The notification settings that are enabled for this object. |
| `image_modules_data` | Vec<String> |  | Image module data. Only one of the image from class and one from object level will be rendered when both set. |
| `hero_image` | String |  | Banner image displayed on the front of the card if present. The image will be displayed at 100% width. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `messages` | Vec<String> | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `rotating_barcode` | String | The rotating barcode settings/details. |
| `subheader` | String | The title label of the pass, such as location where this pass can be used. Appears right above the title in the title row in the pass detail view. |
| `text_modules_data` | Vec<String> | Text module data. If `textModulesData` is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from class and 10 from object. |
| `id` | String | Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value needs to follow the format `issuerID.identifier` where `issuerID` is issued by Google and `identifier` is chosen by you. The unique identifier can only include alphanumeric characters, `.`, `_`, or `-`. |
| `value_added_module_data` | Vec<String> | Optional value added module data. Maximum of ten on the object. |
| `linked_object_ids` | Vec<String> | linked_object_ids are a list of other objects such as event ticket, loyalty, offer, generic, giftcard, transit and boarding pass that should be automatically attached to this generic object. If a user had saved this generic card, then these linked_object_ids would be automatically pushed to the user's wallet (unless they turned off the setting to receive such linked passes). Make sure that objects present in linked_object_ids are already inserted - if not, calls would fail. Once linked, the linked objects cannot be unlinked. You cannot link objects belonging to another issuer. There is a limit to the number of objects that can be linked to a single object. After the limit is reached, new linked objects in the call will be ignored silently. Object IDs should follow the format issuer ID. identifier where the former is issued by Google and the latter is chosen by you. |
| `generic_type` | String | Specify which `GenericType` the card belongs to. |
| `header` | String | Required. The title of the pass, such as "50% off coupon" or "Library card" or "Voucher". This field is required and appears in the title row of the pass detail view. |
| `merchant_locations` | Vec<String> | Merchant locations. There is a maximum of ten on the object. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `hex_background_color` | String | The background color for the card. If not set, the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used and if logo is not set, a color would be chosen by Google. |
| `links_module_data` | String | Links module data. If `linksModuleData` is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from class and 10 from object. |
| `card_title` | String | Required. The header of the pass. This is usually the Business name such as "XXX Gym", "AAA Insurance". This field is required and appears in the header row at the very top of the pass. |
| `grouping_info` | String | Information that controls how passes are grouped together. |
| `app_link_data` | String | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding class only object AppLinkData will be displayed. |
| `class_id` | String | Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format `issuerID.identifier` where `issuerID` is issued by Google and `identifier` is chosen by you. |
| `logo` | String | The logo image of the pass. This image is displayed in the card detail view in upper left, and also on the list/thumbnail view. If the logo is not present, the first letter of `cardTitle` would be shown as logo. |
| `smart_tap_redemption_value` | String | The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported. |
| `pass_constraints` | String | Pass constraints for the object. Includes limiting NFC and screenshot behaviors. |
| `valid_time_interval` | String | The time period this object will be considered valid or usable. When the time period is passed, the object will be considered expired, which will affect the rendering on user's devices. |
| `state` | String | The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section. If this is not provided, the object would be considered `ACTIVE`. |
| `save_restrictions` | String | Restrictions on the object that needs to be verified before the user tries to save the pass. Note that this restrictions will only be applied during save time. If the restrictions changed after a user saves the pass, the new restrictions will not be applied to an already saved pass. |
| `barcode` | String | The barcode type and value. If pass does not have a barcode, we can allow the issuer to set Barcode.alternate_text and display just that. |
| `has_users` | bool | Indicates if the object has users. This field is set by the platform. |
| `wide_logo` | String | The wide logo of the pass. When provided, this will be used in place of the logo in the top left of the card view. |
| `notifications` | String | The notification settings that are enabled for this object. |
| `image_modules_data` | Vec<String> | Image module data. Only one of the image from class and one from object level will be rendered when both set. |
| `hero_image` | String | Banner image displayed on the front of the card if present. The image will be displayed at 100% width. |


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
genericobject_subheader = genericobject.subheader
genericobject_text_modules_data = genericobject.text_modules_data
genericobject_id = genericobject.id
genericobject_value_added_module_data = genericobject.value_added_module_data
genericobject_linked_object_ids = genericobject.linked_object_ids
genericobject_generic_type = genericobject.generic_type
genericobject_header = genericobject.header
genericobject_merchant_locations = genericobject.merchant_locations
genericobject_hex_background_color = genericobject.hex_background_color
genericobject_links_module_data = genericobject.links_module_data
genericobject_card_title = genericobject.card_title
genericobject_grouping_info = genericobject.grouping_info
genericobject_app_link_data = genericobject.app_link_data
genericobject_class_id = genericobject.class_id
genericobject_logo = genericobject.logo
genericobject_smart_tap_redemption_value = genericobject.smart_tap_redemption_value
genericobject_pass_constraints = genericobject.pass_constraints
genericobject_valid_time_interval = genericobject.valid_time_interval
genericobject_state = genericobject.state
genericobject_save_restrictions = genericobject.save_restrictions
genericobject_barcode = genericobject.barcode
genericobject_has_users = genericobject.has_users
genericobject_wide_logo = genericobject.wide_logo
genericobject_notifications = genericobject.notifications
genericobject_image_modules_data = genericobject.image_modules_data
genericobject_hero_image = genericobject.hero_image
```

---


### Media

Uploads rotating barcode values for the transit object referenced by the given object ID. Note the max upload size is specified in google3/production/config/cdd/apps-upload/customers/payments-consumer-passes/config.gcl and enforced by Scotty.

**Operations**: ✅ Create ✅ Read

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `media_request_info` | String |  | Extra information about the uploaded media. |
| `blob` | String |  | A reference to the rotating barcode values payload that was uploaded. |
| `resource_id` | String | ✅ | The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `is_potential_retry` | bool | |is_potential_retry| is set false only when Scotty is certain that it has not sent the request before. When a client resumes an upload, this field must be set true in agent calls, because Scotty cannot be certain that it has never sent the request before due to potential failure in the session state persistence. |
| `object_id` | String | Reference to a TI Blob, set if reference_type is BIGSTORE_REF. |
| `hash` | String | Deprecated, use one of explicit hash type fields instead. These two hash related fields will only be populated on Scotty based media uploads and will contain the content of the hash group in the NotificationRequest: http://cs/#google3/blobstore2/api/scotty/service/proto/upload_listener.proto&q=class:Hash Hex encoded hash value of the uploaded media. |
| `timestamp` | String | Time at which the media data was last updated, in milliseconds since UNIX epoch |
| `content_type` | String | MIME type of the data |
| `reference_type` | String | Describes what the field reference contains. |
| `filename` | String | Original file name |
| `sha256_hash` | String | Scotty-provided SHA256 hash for an upload. |
| `download_parameters` | String | Parameters for a media download. |
| `path` | String | Path to the data, set if reference_type is PATH |
| `diff_checksums_response` | String | Set if reference_type is DIFF_CHECKSUMS_RESPONSE. |
| `media_id` | String | Media id to forward to the operation GetMedia. Can be set if reference_type is GET_MEDIA. |
| `sha1_hash` | String | Scotty-provided SHA1 hash for an upload. |
| `bigstore_object_ref` | String | Use object_id instead. |
| `diff_upload_request` | String | Set if reference_type is DIFF_UPLOAD_REQUEST. |
| `hash_verified` | bool | For Scotty uploads only. If a user sends a hash code and the backend has requested that Scotty verify the upload against the client hash, Scotty will perform the check on behalf of the backend and will reject it if the hashes don't match. This is set to true if Scotty performed this verification. |
| `md5_hash` | String | Scotty-provided MD5 hash for an upload. |
| `content_type_info` | String | Extended content type information provided for Scotty uploads. |
| `blob_ref` | String | Blobstore v1 reference, set if reference_type is BLOBSTORE_REF This should be the byte representation of a blobstore.BlobRef. Since Blobstore is deprecating v1, use blobstore2_info instead. For now, any v2 blob will also be represented in this field as v1 BlobRef. |
| `crc32c_hash` | i64 | For Scotty Uploads: Scotty-provided hashes for uploads For Scotty Downloads: (WARNING: DO NOT USE WITHOUT PERMISSION FROM THE SCOTTY TEAM.) A Hash provided by the agent to be used to verify the data being downloaded. Currently only supported for inline payloads. Further, only crc32c_hash is currently supported. |
| `token` | String | A unique fingerprint/version id for the media data |
| `diff_download_response` | String | Set if reference_type is DIFF_DOWNLOAD_RESPONSE. |
| `algorithm` | String | Deprecated, use one of explicit hash type fields instead. Algorithm used for calculating the hash. As of 2011/01/21, "MD5" is the only possible value for this field. New values may be added at any time. |
| `composite_media` | Vec<String> | A composite media composed of one or more media objects, set if reference_type is COMPOSITE_MEDIA. The media length field must be set to the sum of the lengths of all composite media objects. Note: All composite media must have length specified. |
| `blobstore2_info` | String | Blobstore v2 info, set if reference_type is BLOBSTORE_REF and it refers to a v2 blob. |
| `diff_upload_response` | String | Set if reference_type is DIFF_UPLOAD_RESPONSE. |
| `diff_version_response` | String | Set if reference_type is DIFF_VERSION_RESPONSE. |
| `cosmo_binary_reference` | String | A binary data reference for a media download. Serves as a technology-agnostic binary reference in some Google infrastructure. This value is a serialized storage_cosmo.BinaryReference proto. Storing it as bytes is a hack to get around the fact that the cosmo proto (as well as others it includes) doesn't support JavaScript. This prevents us from including the actual type of this field. |
| `length` | String | Size of the data, in bytes |
| `inline` | String | Media data, set if reference_type is INLINE |


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
media_is_potential_retry = media.is_potential_retry
media_object_id = media.object_id
media_hash = media.hash
media_timestamp = media.timestamp
media_content_type = media.content_type
media_reference_type = media.reference_type
media_filename = media.filename
media_sha256_hash = media.sha256_hash
media_download_parameters = media.download_parameters
media_path = media.path
media_diff_checksums_response = media.diff_checksums_response
media_media_id = media.media_id
media_sha1_hash = media.sha1_hash
media_bigstore_object_ref = media.bigstore_object_ref
media_diff_upload_request = media.diff_upload_request
media_hash_verified = media.hash_verified
media_md5_hash = media.md5_hash
media_content_type_info = media.content_type_info
media_blob_ref = media.blob_ref
media_crc32c_hash = media.crc32c_hash
media_token = media.token
media_diff_download_response = media.diff_download_response
media_algorithm = media.algorithm
media_composite_media = media.composite_media
media_blobstore2_info = media.blobstore2_info
media_diff_upload_response = media.diff_upload_response
media_diff_version_response = media.diff_version_response
media_cosmo_binary_reference = media.cosmo_binary_reference
media_length = media.length
media_inline = media.inline
```

---


### Permission

Returns the permissions for the given issuer id.

**Operations**: ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `issuer_id` | String |  | ID of the issuer the list of permissions refer to. |
| `permissions` | Vec<String> |  | The complete list of permissions for the issuer account. |
| `resource_id` | String | ✅ | The unique identifier for an issuer. This ID must be unique across all issuers. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `issuer_id` | String | ID of the issuer the list of permissions refer to. |
| `permissions` | Vec<String> | The complete list of permissions for the issuer account. |


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
permission_issuer_id = permission.issuer_id
permission_permissions = permission.permissions
```

---


### Flightobject

Inserts an flight object with the given ID and properties.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `class_id` | String |  | Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. |
| `app_link_data` | String |  | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding class only object AppLinkData will be displayed. |
| `class_reference` | String |  | A copy of the inherited fields of the parent class. These fields are retrieved during a GET. |
| `rotating_barcode` | String |  | The rotating barcode type and value. |
| `save_restrictions` | String |  | Restrictions on the object that needs to be verified before the user tries to save the pass. Note that this restrictions will only be applied during save time. If the restrictions changed after a user saves the pass, the new restrictions will not be applied to an already saved pass. |
| `merchant_locations` | Vec<String> |  | Merchant locations. There is a maximum of ten on the object. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `smart_tap_redemption_value` | String |  | The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported. |
| `has_users` | bool |  | Indicates if the object has users. This field is set by the platform. |
| `version` | String |  | Deprecated |
| `linked_object_ids` | Vec<String> |  | linked_object_ids are a list of other objects such as event ticket, loyalty, offer, generic, giftcard, transit and boarding pass that should be automatically attached to this flight object. If a user had saved this boarding pass, then these linked_object_ids would be automatically pushed to the user's wallet (unless they turned off the setting to receive such linked passes). Make sure that objects present in linked_object_ids are already inserted - if not, calls would fail. Once linked, the linked objects cannot be unlinked. You cannot link objects belonging to another issuer. There is a limit to the number of objects that can be linked to a single object. After the limit is reached, new linked objects in the call will be ignored silently. Object IDs should follow the format issuer ID. identifier where the former is issued by Google and the latter is chosen by you. |
| `valid_time_interval` | String |  | The time period this object will be `active` and object can be used. An object's state will be changed to `expired` when this time period has passed. |
| `text_modules_data` | Vec<String> |  | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `pass_constraints` | String |  | Pass constraints for the object. Includes limiting NFC and screenshot behaviors. |
| `image_modules_data` | Vec<String> |  | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `messages` | Vec<String> |  | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `state` | String |  | Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section. |
| `info_module_data` | String |  | Deprecated. Use textModulesData instead. |
| `barcode` | String |  | The barcode type and value. |
| `hero_image` | String |  | Optional banner image displayed on the front of the card. If none is present, hero image of the class, if present, will be displayed. If hero image of the class is also not present, nothing will be displayed. |
| `boarding_and_seating_info` | String |  | Passenger specific information about boarding and seating. |
| `locations` | Vec<String> |  | Note: This field is currently not supported to trigger geo notifications. |
| `grouping_info` | String |  | Information that controls how passes are grouped together. |
| `disable_expiration_notification` | bool |  | Indicates if notifications should explicitly be suppressed. If this field is set to true, regardless of the `messages` field, expiration notifications to the user will be suppressed. By default, this field is set to false. Currently, this can only be set for Flights. |
| `has_linked_device` | bool |  | Whether this object is currently linked to a single device. This field is set by the platform when a user saves the object, linking it to their device. Intended for use by select partners. Contact support for additional information. |
| `value_added_module_data` | Vec<String> |  | Optional value added module data. Maximum of ten on the object. |
| `reservation_info` | String |  | Required. Information about flight reservation. |
| `links_module_data` | String |  | Links module data. If links module data is also defined on the class, both will be displayed. |
| `security_program_logo` | String |  | An image for the security program that applies to the passenger. |
| `hex_background_color` | String |  | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `notify_preference` | String |  | Whether or not field updates to this object should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If set to DO_NOT_NOTIFY or NOTIFICATION_SETTINGS_UNSPECIFIED, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `id` | String |  | Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. The unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `passenger_name` | String |  | Required. Passenger name as it would appear on the boarding pass. eg: "Dave M Gahan" or "Gahan/Dave" or "GAHAN/DAVEM" |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#flightObject"`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `class_id` | String | Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. |
| `app_link_data` | String | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding class only object AppLinkData will be displayed. |
| `class_reference` | String | A copy of the inherited fields of the parent class. These fields are retrieved during a GET. |
| `rotating_barcode` | String | The rotating barcode type and value. |
| `save_restrictions` | String | Restrictions on the object that needs to be verified before the user tries to save the pass. Note that this restrictions will only be applied during save time. If the restrictions changed after a user saves the pass, the new restrictions will not be applied to an already saved pass. |
| `merchant_locations` | Vec<String> | Merchant locations. There is a maximum of ten on the object. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `smart_tap_redemption_value` | String | The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported. |
| `has_users` | bool | Indicates if the object has users. This field is set by the platform. |
| `version` | String | Deprecated |
| `linked_object_ids` | Vec<String> | linked_object_ids are a list of other objects such as event ticket, loyalty, offer, generic, giftcard, transit and boarding pass that should be automatically attached to this flight object. If a user had saved this boarding pass, then these linked_object_ids would be automatically pushed to the user's wallet (unless they turned off the setting to receive such linked passes). Make sure that objects present in linked_object_ids are already inserted - if not, calls would fail. Once linked, the linked objects cannot be unlinked. You cannot link objects belonging to another issuer. There is a limit to the number of objects that can be linked to a single object. After the limit is reached, new linked objects in the call will be ignored silently. Object IDs should follow the format issuer ID. identifier where the former is issued by Google and the latter is chosen by you. |
| `valid_time_interval` | String | The time period this object will be `active` and object can be used. An object's state will be changed to `expired` when this time period has passed. |
| `text_modules_data` | Vec<String> | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `pass_constraints` | String | Pass constraints for the object. Includes limiting NFC and screenshot behaviors. |
| `image_modules_data` | Vec<String> | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `messages` | Vec<String> | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `state` | String | Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section. |
| `info_module_data` | String | Deprecated. Use textModulesData instead. |
| `barcode` | String | The barcode type and value. |
| `hero_image` | String | Optional banner image displayed on the front of the card. If none is present, hero image of the class, if present, will be displayed. If hero image of the class is also not present, nothing will be displayed. |
| `boarding_and_seating_info` | String | Passenger specific information about boarding and seating. |
| `locations` | Vec<String> | Note: This field is currently not supported to trigger geo notifications. |
| `grouping_info` | String | Information that controls how passes are grouped together. |
| `disable_expiration_notification` | bool | Indicates if notifications should explicitly be suppressed. If this field is set to true, regardless of the `messages` field, expiration notifications to the user will be suppressed. By default, this field is set to false. Currently, this can only be set for Flights. |
| `has_linked_device` | bool | Whether this object is currently linked to a single device. This field is set by the platform when a user saves the object, linking it to their device. Intended for use by select partners. Contact support for additional information. |
| `value_added_module_data` | Vec<String> | Optional value added module data. Maximum of ten on the object. |
| `reservation_info` | String | Required. Information about flight reservation. |
| `links_module_data` | String | Links module data. If links module data is also defined on the class, both will be displayed. |
| `security_program_logo` | String | An image for the security program that applies to the passenger. |
| `hex_background_color` | String | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `notify_preference` | String | Whether or not field updates to this object should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If set to DO_NOT_NOTIFY or NOTIFICATION_SETTINGS_UNSPECIFIED, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `id` | String | Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. The unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `passenger_name` | String | Required. Passenger name as it would appear on the boarding pass. eg: "Dave M Gahan" or "Gahan/Dave" or "GAHAN/DAVEM" |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#flightObject"`. |


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
flightobject_app_link_data = flightobject.app_link_data
flightobject_class_reference = flightobject.class_reference
flightobject_rotating_barcode = flightobject.rotating_barcode
flightobject_save_restrictions = flightobject.save_restrictions
flightobject_merchant_locations = flightobject.merchant_locations
flightobject_smart_tap_redemption_value = flightobject.smart_tap_redemption_value
flightobject_has_users = flightobject.has_users
flightobject_version = flightobject.version
flightobject_linked_object_ids = flightobject.linked_object_ids
flightobject_valid_time_interval = flightobject.valid_time_interval
flightobject_text_modules_data = flightobject.text_modules_data
flightobject_pass_constraints = flightobject.pass_constraints
flightobject_image_modules_data = flightobject.image_modules_data
flightobject_messages = flightobject.messages
flightobject_state = flightobject.state
flightobject_info_module_data = flightobject.info_module_data
flightobject_barcode = flightobject.barcode
flightobject_hero_image = flightobject.hero_image
flightobject_boarding_and_seating_info = flightobject.boarding_and_seating_info
flightobject_locations = flightobject.locations
flightobject_grouping_info = flightobject.grouping_info
flightobject_disable_expiration_notification = flightobject.disable_expiration_notification
flightobject_has_linked_device = flightobject.has_linked_device
flightobject_value_added_module_data = flightobject.value_added_module_data
flightobject_reservation_info = flightobject.reservation_info
flightobject_links_module_data = flightobject.links_module_data
flightobject_security_program_logo = flightobject.security_program_logo
flightobject_hex_background_color = flightobject.hex_background_color
flightobject_notify_preference = flightobject.notify_preference
flightobject_id = flightobject.id
flightobject_passenger_name = flightobject.passenger_name
flightobject_kind = flightobject.kind
```

---


### Offerclas

Inserts an offer class with the given ID and properties.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `class_template_info` | String |  | Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display. |
| `notify_preference` | String |  | Whether or not field updates to this class should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If not specified, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `redemption_channel` | String |  | Required. The redemption channels applicable to this offer. |
| `redemption_issuers` | Vec<String> |  | Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `short_title` | String |  | A shortened version of the title of the offer, such as "20% off," shown to users as a quick reference to the offer contents. Recommended maximum length is 20 characters. |
| `title_image` | String |  | The title image of the offer. This image is displayed in both the details and list views of the app. |
| `help_uri` | String |  | The help link for the offer, such as `http://myownpersonaldomain.com/help` |
| `info_module_data` | String |  | Deprecated. Use textModulesData instead. |
| `issuer_name` | String |  | Required. The issuer name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `allow_multiple_users_per_object` | bool |  | Deprecated. Use `multipleDevicesAndHoldersAllowedStatus` instead. |
| `app_link_data` | String |  | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding object that will be used instead. |
| `hero_image` | String |  | Optional banner image displayed on the front of the card. If none is present, nothing will be displayed. The image will display at 100% width. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#offerClass"`. |
| `links_module_data` | String |  | Links module data. If links module data is also defined on the object, both will be displayed. |
| `localized_details` | String |  | Translated strings for the details. |
| `localized_short_title` | String |  | Translated strings for the short title. Recommended maximum length is 20 characters. |
| `provider` | String |  | Required. The offer provider (either the aggregator name or merchant name). Recommended maximum length is 12 characters to ensure full string is displayed on smaller screens. |
| `review` | String |  | The review comments set by the platform when a class is marked `approved` or `rejected`. |
| `text_modules_data` | Vec<String> |  | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `title` | String |  | Required. The title of the offer, such as "20% off any t-shirt." Recommended maximum length is 60 characters to ensure full string is displayed on smaller screens. |
| `version` | String |  | Deprecated |
| `localized_fine_print` | String |  | Translated strings for the fine_print. |
| `wide_title_image` | String |  | The wide title image of the offer. When provided, this will be used in place of the title image in the top left of the card view. |
| `localized_issuer_name` | String |  | Translated strings for the issuer_name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `image_modules_data` | Vec<String> |  | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `enable_smart_tap` | bool |  | Identifies whether this class supports Smart Tap. The `redemptionIssuers` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `view_unlock_requirement` | String |  | View Unlock Requirement options for the offer. |
| `fine_print` | String |  | The fine print or terms of the offer, such as "20% off any t-shirt at Adam's Apparel." |
| `hex_background_color` | String |  | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `localized_title` | String |  | Translated strings for the title. Recommended maximum length is 60 characters to ensure full string is displayed on smaller screens. |
| `id` | String |  | Required. The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `merchant_locations` | Vec<String> |  | Merchant locations. There is a maximum of ten on the class. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `value_added_module_data` | Vec<String> |  | Optional value added module data. Maximum of ten on the class. For a pass only ten will be displayed, prioritizing those from the object. |
| `word_mark` | String |  | Deprecated. |
| `callback_options` | String |  | Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback. |
| `country_code` | String |  | Country code used to display the card's country (when the user is not in that country), as well as to display localized content when content is not available in the user's locale. |
| `homepage_uri` | String |  | The URI of your application's home page. Populating the URI in this field results in the exact same behavior as populating an URI in linksModuleData (when an object is rendered, a link to the homepage is shown in what would usually be thought of as the linksModuleData section of the object). |
| `security_animation` | String |  | Optional information about the security animation. If this is set a security animation will be rendered on pass details. |
| `review_status` | String |  | Required. The status of the class. This field can be set to `draft` or The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`. |
| `multiple_devices_and_holders_allowed_status` | String |  | Identifies whether multiple users and devices will save the same object referencing this class. |
| `locations` | Vec<String> |  | Note: This field is currently not supported to trigger geo notifications. |
| `messages` | Vec<String> |  | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `localized_provider` | String |  | Translated strings for the provider. Recommended maximum length is 12 characters to ensure full string is displayed on smaller screens. |
| `details` | String |  | The details of the offer. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `class_template_info` | String | Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display. |
| `notify_preference` | String | Whether or not field updates to this class should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If not specified, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `redemption_channel` | String | Required. The redemption channels applicable to this offer. |
| `redemption_issuers` | Vec<String> | Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `short_title` | String | A shortened version of the title of the offer, such as "20% off," shown to users as a quick reference to the offer contents. Recommended maximum length is 20 characters. |
| `title_image` | String | The title image of the offer. This image is displayed in both the details and list views of the app. |
| `help_uri` | String | The help link for the offer, such as `http://myownpersonaldomain.com/help` |
| `info_module_data` | String | Deprecated. Use textModulesData instead. |
| `issuer_name` | String | Required. The issuer name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `allow_multiple_users_per_object` | bool | Deprecated. Use `multipleDevicesAndHoldersAllowedStatus` instead. |
| `app_link_data` | String | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding object that will be used instead. |
| `hero_image` | String | Optional banner image displayed on the front of the card. If none is present, nothing will be displayed. The image will display at 100% width. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#offerClass"`. |
| `links_module_data` | String | Links module data. If links module data is also defined on the object, both will be displayed. |
| `localized_details` | String | Translated strings for the details. |
| `localized_short_title` | String | Translated strings for the short title. Recommended maximum length is 20 characters. |
| `provider` | String | Required. The offer provider (either the aggregator name or merchant name). Recommended maximum length is 12 characters to ensure full string is displayed on smaller screens. |
| `review` | String | The review comments set by the platform when a class is marked `approved` or `rejected`. |
| `text_modules_data` | Vec<String> | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `title` | String | Required. The title of the offer, such as "20% off any t-shirt." Recommended maximum length is 60 characters to ensure full string is displayed on smaller screens. |
| `version` | String | Deprecated |
| `localized_fine_print` | String | Translated strings for the fine_print. |
| `wide_title_image` | String | The wide title image of the offer. When provided, this will be used in place of the title image in the top left of the card view. |
| `localized_issuer_name` | String | Translated strings for the issuer_name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `image_modules_data` | Vec<String> | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `enable_smart_tap` | bool | Identifies whether this class supports Smart Tap. The `redemptionIssuers` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `view_unlock_requirement` | String | View Unlock Requirement options for the offer. |
| `fine_print` | String | The fine print or terms of the offer, such as "20% off any t-shirt at Adam's Apparel." |
| `hex_background_color` | String | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `localized_title` | String | Translated strings for the title. Recommended maximum length is 60 characters to ensure full string is displayed on smaller screens. |
| `id` | String | Required. The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `merchant_locations` | Vec<String> | Merchant locations. There is a maximum of ten on the class. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `value_added_module_data` | Vec<String> | Optional value added module data. Maximum of ten on the class. For a pass only ten will be displayed, prioritizing those from the object. |
| `word_mark` | String | Deprecated. |
| `callback_options` | String | Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback. |
| `country_code` | String | Country code used to display the card's country (when the user is not in that country), as well as to display localized content when content is not available in the user's locale. |
| `homepage_uri` | String | The URI of your application's home page. Populating the URI in this field results in the exact same behavior as populating an URI in linksModuleData (when an object is rendered, a link to the homepage is shown in what would usually be thought of as the linksModuleData section of the object). |
| `security_animation` | String | Optional information about the security animation. If this is set a security animation will be rendered on pass details. |
| `review_status` | String | Required. The status of the class. This field can be set to `draft` or The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`. |
| `multiple_devices_and_holders_allowed_status` | String | Identifies whether multiple users and devices will save the same object referencing this class. |
| `locations` | Vec<String> | Note: This field is currently not supported to trigger geo notifications. |
| `messages` | Vec<String> | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `localized_provider` | String | Translated strings for the provider. Recommended maximum length is 12 characters to ensure full string is displayed on smaller screens. |
| `details` | String | The details of the offer. |


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
offerclas_class_template_info = offerclas.class_template_info
offerclas_notify_preference = offerclas.notify_preference
offerclas_redemption_channel = offerclas.redemption_channel
offerclas_redemption_issuers = offerclas.redemption_issuers
offerclas_short_title = offerclas.short_title
offerclas_title_image = offerclas.title_image
offerclas_help_uri = offerclas.help_uri
offerclas_info_module_data = offerclas.info_module_data
offerclas_issuer_name = offerclas.issuer_name
offerclas_allow_multiple_users_per_object = offerclas.allow_multiple_users_per_object
offerclas_app_link_data = offerclas.app_link_data
offerclas_hero_image = offerclas.hero_image
offerclas_kind = offerclas.kind
offerclas_links_module_data = offerclas.links_module_data
offerclas_localized_details = offerclas.localized_details
offerclas_localized_short_title = offerclas.localized_short_title
offerclas_provider = offerclas.provider
offerclas_review = offerclas.review
offerclas_text_modules_data = offerclas.text_modules_data
offerclas_title = offerclas.title
offerclas_version = offerclas.version
offerclas_localized_fine_print = offerclas.localized_fine_print
offerclas_wide_title_image = offerclas.wide_title_image
offerclas_localized_issuer_name = offerclas.localized_issuer_name
offerclas_image_modules_data = offerclas.image_modules_data
offerclas_enable_smart_tap = offerclas.enable_smart_tap
offerclas_view_unlock_requirement = offerclas.view_unlock_requirement
offerclas_fine_print = offerclas.fine_print
offerclas_hex_background_color = offerclas.hex_background_color
offerclas_localized_title = offerclas.localized_title
offerclas_id = offerclas.id
offerclas_merchant_locations = offerclas.merchant_locations
offerclas_value_added_module_data = offerclas.value_added_module_data
offerclas_word_mark = offerclas.word_mark
offerclas_callback_options = offerclas.callback_options
offerclas_country_code = offerclas.country_code
offerclas_homepage_uri = offerclas.homepage_uri
offerclas_security_animation = offerclas.security_animation
offerclas_review_status = offerclas.review_status
offerclas_multiple_devices_and_holders_allowed_status = offerclas.multiple_devices_and_holders_allowed_status
offerclas_locations = offerclas.locations
offerclas_messages = offerclas.messages
offerclas_localized_provider = offerclas.localized_provider
offerclas_details = offerclas.details
```

---


### Eventticketobject

Inserts an event ticket object with the given ID and properties.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `class_reference` | String |  | A copy of the inherited fields of the parent class. These fields are retrieved during a GET. |
| `reservation_info` | String |  | Reservation details for this ticket. This is expected to be shared amongst all tickets that were purchased in the same order. |
| `face_value` | String |  | The face value of the ticket, matching what would be printed on a physical version of the ticket. |
| `value_added_module_data` | Vec<String> |  | Optional value added module data. Maximum of ten on the object. |
| `ticket_holder_name` | String |  | Name of the ticket holder, if the ticket is assigned to a person. E.g. "John Doe" or "Jane Doe". |
| `messages` | Vec<String> |  | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `info_module_data` | String |  | Deprecated. Use textModulesData instead. |
| `class_id` | String |  | Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. |
| `app_link_data` | String |  | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding class only object AppLinkData will be displayed. |
| `ticket_number` | String |  | The number of the ticket. This can be a unique identifier across all tickets in an issuer's system, all tickets for the event (e.g. XYZ1234512345), or all tickets in the order (1, 2, 3, etc.). |
| `has_users` | bool |  | Indicates if the object has users. This field is set by the platform. |
| `valid_time_interval` | String |  | The time period this object will be `active` and object can be used. An object's state will be changed to `expired` when this time period has passed. |
| `linked_object_ids` | Vec<String> |  | linked_object_ids are a list of other objects such as event ticket, loyalty, offer, generic, giftcard, transit and boarding pass that should be automatically attached to this event ticket object. If a user had saved this event ticket, then these linked_object_ids would be automatically pushed to the user's wallet (unless they turned off the setting to receive such linked passes). Make sure that objects present in linked_object_ids are already inserted - if not, calls would fail. Once linked, the linked objects cannot be unlinked. You cannot link objects belonging to another issuer. There is a limit to the number of objects that can be linked to a single object. After the limit is reached, new linked objects in the call will be ignored silently. Object IDs should follow the format issuer ID. identifier where the former is issued by Google and the latter is chosen by you. |
| `state` | String |  | Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section. |
| `id` | String |  | Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. The unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `version` | String |  | Deprecated |
| `disable_expiration_notification` | bool |  | Indicates if notifications should explicitly be suppressed. If this field is set to true, regardless of the `messages` field, expiration notifications to the user will be suppressed. By default, this field is set to false. Currently, this can only be set for offers. |
| `merchant_locations` | Vec<String> |  | Merchant locations. There is a maximum of ten on the object. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `save_restrictions` | String |  | Restrictions on the object that needs to be verified before the user tries to save the pass. Note that this restrictions will only be applied during save time. If the restrictions changed after a user saves the pass, the new restrictions will not be applied to an already saved pass. |
| `image_modules_data` | Vec<String> |  | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `hex_background_color` | String |  | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `pass_constraints` | String |  | Pass constraints for the object. Includes limiting NFC and screenshot behaviors. |
| `seat_info` | String |  | Seating details for this ticket. |
| `notify_preference` | String |  | Whether or not field updates to this object should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If set to DO_NOT_NOTIFY or NOTIFICATION_SETTINGS_UNSPECIFIED, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `hero_image` | String |  | Optional banner image displayed on the front of the card. If none is present, hero image of the class, if present, will be displayed. If hero image of the class is also not present, nothing will be displayed. |
| `has_linked_device` | bool |  | Whether this object is currently linked to a single device. This field is set by the platform when a user saves the object, linking it to their device. Intended for use by select partners. Contact support for additional information. |
| `locations` | Vec<String> |  | Note: This field is currently not supported to trigger geo notifications. |
| `text_modules_data` | Vec<String> |  | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `ticket_type` | String |  | The type of the ticket, such as "Adult" or "Child", or "VIP" or "Standard". |
| `barcode` | String |  | The barcode type and value. |
| `links_module_data` | String |  | Links module data. If links module data is also defined on the class, both will be displayed. |
| `smart_tap_redemption_value` | String |  | The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported. |
| `linked_offer_ids` | Vec<String> |  | A list of offer objects linked to this event ticket. The offer objects must already exist. Offer object IDs should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. |
| `rotating_barcode` | String |  | The rotating barcode type and value. |
| `grouping_info` | String |  | Information that controls how passes are grouped together. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#eventTicketObject"`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `class_reference` | String | A copy of the inherited fields of the parent class. These fields are retrieved during a GET. |
| `reservation_info` | String | Reservation details for this ticket. This is expected to be shared amongst all tickets that were purchased in the same order. |
| `face_value` | String | The face value of the ticket, matching what would be printed on a physical version of the ticket. |
| `value_added_module_data` | Vec<String> | Optional value added module data. Maximum of ten on the object. |
| `ticket_holder_name` | String | Name of the ticket holder, if the ticket is assigned to a person. E.g. "John Doe" or "Jane Doe". |
| `messages` | Vec<String> | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `info_module_data` | String | Deprecated. Use textModulesData instead. |
| `class_id` | String | Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. |
| `app_link_data` | String | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding class only object AppLinkData will be displayed. |
| `ticket_number` | String | The number of the ticket. This can be a unique identifier across all tickets in an issuer's system, all tickets for the event (e.g. XYZ1234512345), or all tickets in the order (1, 2, 3, etc.). |
| `has_users` | bool | Indicates if the object has users. This field is set by the platform. |
| `valid_time_interval` | String | The time period this object will be `active` and object can be used. An object's state will be changed to `expired` when this time period has passed. |
| `linked_object_ids` | Vec<String> | linked_object_ids are a list of other objects such as event ticket, loyalty, offer, generic, giftcard, transit and boarding pass that should be automatically attached to this event ticket object. If a user had saved this event ticket, then these linked_object_ids would be automatically pushed to the user's wallet (unless they turned off the setting to receive such linked passes). Make sure that objects present in linked_object_ids are already inserted - if not, calls would fail. Once linked, the linked objects cannot be unlinked. You cannot link objects belonging to another issuer. There is a limit to the number of objects that can be linked to a single object. After the limit is reached, new linked objects in the call will be ignored silently. Object IDs should follow the format issuer ID. identifier where the former is issued by Google and the latter is chosen by you. |
| `state` | String | Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section. |
| `id` | String | Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. The unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `version` | String | Deprecated |
| `disable_expiration_notification` | bool | Indicates if notifications should explicitly be suppressed. If this field is set to true, regardless of the `messages` field, expiration notifications to the user will be suppressed. By default, this field is set to false. Currently, this can only be set for offers. |
| `merchant_locations` | Vec<String> | Merchant locations. There is a maximum of ten on the object. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `save_restrictions` | String | Restrictions on the object that needs to be verified before the user tries to save the pass. Note that this restrictions will only be applied during save time. If the restrictions changed after a user saves the pass, the new restrictions will not be applied to an already saved pass. |
| `image_modules_data` | Vec<String> | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `hex_background_color` | String | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `pass_constraints` | String | Pass constraints for the object. Includes limiting NFC and screenshot behaviors. |
| `seat_info` | String | Seating details for this ticket. |
| `notify_preference` | String | Whether or not field updates to this object should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If set to DO_NOT_NOTIFY or NOTIFICATION_SETTINGS_UNSPECIFIED, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `hero_image` | String | Optional banner image displayed on the front of the card. If none is present, hero image of the class, if present, will be displayed. If hero image of the class is also not present, nothing will be displayed. |
| `has_linked_device` | bool | Whether this object is currently linked to a single device. This field is set by the platform when a user saves the object, linking it to their device. Intended for use by select partners. Contact support for additional information. |
| `locations` | Vec<String> | Note: This field is currently not supported to trigger geo notifications. |
| `text_modules_data` | Vec<String> | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `ticket_type` | String | The type of the ticket, such as "Adult" or "Child", or "VIP" or "Standard". |
| `barcode` | String | The barcode type and value. |
| `links_module_data` | String | Links module data. If links module data is also defined on the class, both will be displayed. |
| `smart_tap_redemption_value` | String | The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported. |
| `linked_offer_ids` | Vec<String> | A list of offer objects linked to this event ticket. The offer objects must already exist. Offer object IDs should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. |
| `rotating_barcode` | String | The rotating barcode type and value. |
| `grouping_info` | String | Information that controls how passes are grouped together. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#eventTicketObject"`. |


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
eventticketobject_class_reference = eventticketobject.class_reference
eventticketobject_reservation_info = eventticketobject.reservation_info
eventticketobject_face_value = eventticketobject.face_value
eventticketobject_value_added_module_data = eventticketobject.value_added_module_data
eventticketobject_ticket_holder_name = eventticketobject.ticket_holder_name
eventticketobject_messages = eventticketobject.messages
eventticketobject_info_module_data = eventticketobject.info_module_data
eventticketobject_class_id = eventticketobject.class_id
eventticketobject_app_link_data = eventticketobject.app_link_data
eventticketobject_ticket_number = eventticketobject.ticket_number
eventticketobject_has_users = eventticketobject.has_users
eventticketobject_valid_time_interval = eventticketobject.valid_time_interval
eventticketobject_linked_object_ids = eventticketobject.linked_object_ids
eventticketobject_state = eventticketobject.state
eventticketobject_id = eventticketobject.id
eventticketobject_version = eventticketobject.version
eventticketobject_disable_expiration_notification = eventticketobject.disable_expiration_notification
eventticketobject_merchant_locations = eventticketobject.merchant_locations
eventticketobject_save_restrictions = eventticketobject.save_restrictions
eventticketobject_image_modules_data = eventticketobject.image_modules_data
eventticketobject_hex_background_color = eventticketobject.hex_background_color
eventticketobject_pass_constraints = eventticketobject.pass_constraints
eventticketobject_seat_info = eventticketobject.seat_info
eventticketobject_notify_preference = eventticketobject.notify_preference
eventticketobject_hero_image = eventticketobject.hero_image
eventticketobject_has_linked_device = eventticketobject.has_linked_device
eventticketobject_locations = eventticketobject.locations
eventticketobject_text_modules_data = eventticketobject.text_modules_data
eventticketobject_ticket_type = eventticketobject.ticket_type
eventticketobject_barcode = eventticketobject.barcode
eventticketobject_links_module_data = eventticketobject.links_module_data
eventticketobject_smart_tap_redemption_value = eventticketobject.smart_tap_redemption_value
eventticketobject_linked_offer_ids = eventticketobject.linked_offer_ids
eventticketobject_rotating_barcode = eventticketobject.rotating_barcode
eventticketobject_grouping_info = eventticketobject.grouping_info
eventticketobject_kind = eventticketobject.kind
```

---


### Issuer

Inserts an issuer with the given ID and properties.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `contact_info` | String |  | Issuer contact information. |
| `issuer_id` | String |  | The unique identifier for an issuer account. This is automatically generated when the issuer is inserted. |
| `homepage_url` | String |  | URL for the issuer's home page. |
| `smart_tap_merchant_data` | String |  | Available only to Smart Tap enabled partners. Contact support for additional guidance. |
| `callback_options` | String |  | Allows the issuer to provide their callback settings. |
| `name` | String |  | The account name of the issuer. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `contact_info` | String | Issuer contact information. |
| `issuer_id` | String | The unique identifier for an issuer account. This is automatically generated when the issuer is inserted. |
| `homepage_url` | String | URL for the issuer's home page. |
| `smart_tap_merchant_data` | String | Available only to Smart Tap enabled partners. Contact support for additional guidance. |
| `callback_options` | String | Allows the issuer to provide their callback settings. |
| `name` | String | The account name of the issuer. |


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
issuer_contact_info = issuer.contact_info
issuer_issuer_id = issuer.issuer_id
issuer_homepage_url = issuer.homepage_url
issuer_smart_tap_merchant_data = issuer.smart_tap_merchant_data
issuer_callback_options = issuer.callback_options
issuer_name = issuer.name
```

---


### Smarttap

Inserts the smart tap.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#smartTap"`. |
| `infos` | Vec<String> |  | Communication from merchant to user. |
| `id` | String |  | The unique identifier for a smart tap. This value should follow the format issuer ID.identifier where the former is issued by Google and latter is the Smart Tap id. The Smart Tap id is a Base64 encoded string which represents the id which was generated by the Google Pay app. |
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


### Eventticketclas

Inserts an event ticket class with the given ID and properties.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `section_label` | String |  | The label to use for the section value (`eventTicketObject.seatInfo.section`) on the card detail view. Each available option maps to a set of localized strings, so that translations are shown to the user based on their locale. Both `sectionLabel` and `customSectionLabel` may not be set. If neither is set, the label will default to "Section", localized. If the section field is unset, this label will not be used. |
| `value_added_module_data` | Vec<String> |  | Optional value added module data. Maximum of ten on the class. For a pass only ten will be displayed, prioritizing those from the object. |
| `issuer_name` | String |  | Required. The issuer name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `view_unlock_requirement` | String |  | View Unlock Requirement options for the event ticket. |
| `callback_options` | String |  | Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback. |
| `wide_logo` | String |  | The wide logo of the ticket. When provided, this will be used in place of the logo in the top left of the card view. |
| `event_name` | String |  | Required. The name of the event, such as "LA Dodgers at SF Giants". |
| `redemption_issuers` | Vec<String> |  | Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `review` | String |  | The review comments set by the platform when a class is marked `approved` or `rejected`. |
| `class_template_info` | String |  | Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display. |
| `image_modules_data` | Vec<String> |  | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `info_module_data` | String |  | Deprecated. Use textModulesData instead. |
| `version` | String |  | Deprecated |
| `hero_image` | String |  | Optional banner image displayed on the front of the card. If none is present, nothing will be displayed. The image will display at 100% width. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#eventTicketClass"`. |
| `gate_label` | String |  | The label to use for the gate value (`eventTicketObject.seatInfo.gate`) on the card detail view. Each available option maps to a set of localized strings, so that translations are shown to the user based on their locale. Both `gateLabel` and `customGateLabel` may not be set. If neither is set, the label will default to "Gate", localized. If the gate field is unset, this label will not be used. |
| `id` | String |  | Required. The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `homepage_uri` | String |  | The URI of your application's home page. Populating the URI in this field results in the exact same behavior as populating an URI in linksModuleData (when an object is rendered, a link to the homepage is shown in what would usually be thought of as the linksModuleData section of the object). |
| `security_animation` | String |  | Optional information about the security animation. If this is set a security animation will be rendered on pass details. |
| `custom_confirmation_code_label` | String |  | A custom label to use for the confirmation code value (`eventTicketObject.reservationInfo.confirmationCode`) on the card detail view. This should only be used if the default "Confirmation Code" label or one of the `confirmationCodeLabel` options is not sufficient. Both `confirmationCodeLabel` and `customConfirmationCodeLabel` may not be set. If neither is set, the label will default to "Confirmation Code", localized. If the confirmation code field is unset, this label will not be used. |
| `text_modules_data` | Vec<String> |  | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `row_label` | String |  | The label to use for the row value (`eventTicketObject.seatInfo.row`) on the card detail view. Each available option maps to a set of localized strings, so that translations are shown to the user based on their locale. Both `rowLabel` and `customRowLabel` may not be set. If neither is set, the label will default to "Row", localized. If the row field is unset, this label will not be used. |
| `venue` | String |  | Event venue details. |
| `word_mark` | String |  | Deprecated. |
| `country_code` | String |  | Country code used to display the card's country (when the user is not in that country), as well as to display localized content when content is not available in the user's locale. |
| `enable_smart_tap` | bool |  | Identifies whether this class supports Smart Tap. The `redemptionIssuers` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `custom_gate_label` | String |  | A custom label to use for the gate value (`eventTicketObject.seatInfo.gate`) on the card detail view. This should only be used if the default "Gate" label or one of the `gateLabel` options is not sufficient. Both `gateLabel` and `customGateLabel` may not be set. If neither is set, the label will default to "Gate", localized. If the gate field is unset, this label will not be used. |
| `event_id` | String |  | The ID of the event. This ID should be unique for every event in an account. It is used to group tickets together if the user has saved multiple tickets for the same event. It can be at most 64 characters. If provided, the grouping will be stable. Be wary of unintentional collision to avoid grouping tickets that should not be grouped. If you use only one class per event, you can simply set this to the `classId` (with or without the issuer ID portion). If not provided, the platform will attempt to use other data to group tickets (potentially unstable). |
| `fine_print` | String |  | The fine print, terms, or conditions of the ticket. |
| `app_link_data` | String |  | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding object that will be used instead. |
| `hex_background_color` | String |  | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `links_module_data` | String |  | Links module data. If links module data is also defined on the object, both will be displayed. |
| `locations` | Vec<String> |  | Note: This field is currently not supported to trigger geo notifications. |
| `custom_seat_label` | String |  | A custom label to use for the seat value (`eventTicketObject.seatInfo.seat`) on the card detail view. This should only be used if the default "Seat" label or one of the `seatLabel` options is not sufficient. Both `seatLabel` and `customSeatLabel` may not be set. If neither is set, the label will default to "Seat", localized. If the seat field is unset, this label will not be used. |
| `logo` | String |  | The logo image of the ticket. This image is displayed in the card detail view of the app. |
| `date_time` | String |  | The date & time information of the event. |
| `confirmation_code_label` | String |  | The label to use for the confirmation code value (`eventTicketObject.reservationInfo.confirmationCode`) on the card detail view. Each available option maps to a set of localized strings, so that translations are shown to the user based on their locale. Both `confirmationCodeLabel` and `customConfirmationCodeLabel` may not be set. If neither is set, the label will default to "Confirmation Code", localized. If the confirmation code field is unset, this label will not be used. |
| `custom_row_label` | String |  | A custom label to use for the row value (`eventTicketObject.seatInfo.row`) on the card detail view. This should only be used if the default "Row" label or one of the `rowLabel` options is not sufficient. Both `rowLabel` and `customRowLabel` may not be set. If neither is set, the label will default to "Row", localized. If the row field is unset, this label will not be used. |
| `merchant_locations` | Vec<String> |  | Merchant locations. There is a maximum of ten on the class. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `messages` | Vec<String> |  | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `custom_section_label` | String |  | A custom label to use for the section value (`eventTicketObject.seatInfo.section`) on the card detail view. This should only be used if the default "Section" label or one of the `sectionLabel` options is not sufficient. Both `sectionLabel` and `customSectionLabel` may not be set. If neither is set, the label will default to "Section", localized. If the section field is unset, this label will not be used. |
| `notify_preference` | String |  | Whether or not field updates to this class should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If not specified, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `review_status` | String |  | Required. The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`. |
| `allow_multiple_users_per_object` | bool |  | Deprecated. Use `multipleDevicesAndHoldersAllowedStatus` instead. |
| `multiple_devices_and_holders_allowed_status` | String |  | Identifies whether multiple users and devices will save the same object referencing this class. |
| `localized_issuer_name` | String |  | Translated strings for the issuer_name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `seat_label` | String |  | The label to use for the seat value (`eventTicketObject.seatInfo.seat`) on the card detail view. Each available option maps to a set of localized strings, so that translations are shown to the user based on their locale. Both `seatLabel` and `customSeatLabel` may not be set. If neither is set, the label will default to "Seat", localized. If the seat field is unset, this label will not be used. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `section_label` | String | The label to use for the section value (`eventTicketObject.seatInfo.section`) on the card detail view. Each available option maps to a set of localized strings, so that translations are shown to the user based on their locale. Both `sectionLabel` and `customSectionLabel` may not be set. If neither is set, the label will default to "Section", localized. If the section field is unset, this label will not be used. |
| `value_added_module_data` | Vec<String> | Optional value added module data. Maximum of ten on the class. For a pass only ten will be displayed, prioritizing those from the object. |
| `issuer_name` | String | Required. The issuer name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `view_unlock_requirement` | String | View Unlock Requirement options for the event ticket. |
| `callback_options` | String | Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback. |
| `wide_logo` | String | The wide logo of the ticket. When provided, this will be used in place of the logo in the top left of the card view. |
| `event_name` | String | Required. The name of the event, such as "LA Dodgers at SF Giants". |
| `redemption_issuers` | Vec<String> | Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `review` | String | The review comments set by the platform when a class is marked `approved` or `rejected`. |
| `class_template_info` | String | Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display. |
| `image_modules_data` | Vec<String> | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `info_module_data` | String | Deprecated. Use textModulesData instead. |
| `version` | String | Deprecated |
| `hero_image` | String | Optional banner image displayed on the front of the card. If none is present, nothing will be displayed. The image will display at 100% width. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#eventTicketClass"`. |
| `gate_label` | String | The label to use for the gate value (`eventTicketObject.seatInfo.gate`) on the card detail view. Each available option maps to a set of localized strings, so that translations are shown to the user based on their locale. Both `gateLabel` and `customGateLabel` may not be set. If neither is set, the label will default to "Gate", localized. If the gate field is unset, this label will not be used. |
| `id` | String | Required. The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `homepage_uri` | String | The URI of your application's home page. Populating the URI in this field results in the exact same behavior as populating an URI in linksModuleData (when an object is rendered, a link to the homepage is shown in what would usually be thought of as the linksModuleData section of the object). |
| `security_animation` | String | Optional information about the security animation. If this is set a security animation will be rendered on pass details. |
| `custom_confirmation_code_label` | String | A custom label to use for the confirmation code value (`eventTicketObject.reservationInfo.confirmationCode`) on the card detail view. This should only be used if the default "Confirmation Code" label or one of the `confirmationCodeLabel` options is not sufficient. Both `confirmationCodeLabel` and `customConfirmationCodeLabel` may not be set. If neither is set, the label will default to "Confirmation Code", localized. If the confirmation code field is unset, this label will not be used. |
| `text_modules_data` | Vec<String> | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `row_label` | String | The label to use for the row value (`eventTicketObject.seatInfo.row`) on the card detail view. Each available option maps to a set of localized strings, so that translations are shown to the user based on their locale. Both `rowLabel` and `customRowLabel` may not be set. If neither is set, the label will default to "Row", localized. If the row field is unset, this label will not be used. |
| `venue` | String | Event venue details. |
| `word_mark` | String | Deprecated. |
| `country_code` | String | Country code used to display the card's country (when the user is not in that country), as well as to display localized content when content is not available in the user's locale. |
| `enable_smart_tap` | bool | Identifies whether this class supports Smart Tap. The `redemptionIssuers` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `custom_gate_label` | String | A custom label to use for the gate value (`eventTicketObject.seatInfo.gate`) on the card detail view. This should only be used if the default "Gate" label or one of the `gateLabel` options is not sufficient. Both `gateLabel` and `customGateLabel` may not be set. If neither is set, the label will default to "Gate", localized. If the gate field is unset, this label will not be used. |
| `event_id` | String | The ID of the event. This ID should be unique for every event in an account. It is used to group tickets together if the user has saved multiple tickets for the same event. It can be at most 64 characters. If provided, the grouping will be stable. Be wary of unintentional collision to avoid grouping tickets that should not be grouped. If you use only one class per event, you can simply set this to the `classId` (with or without the issuer ID portion). If not provided, the platform will attempt to use other data to group tickets (potentially unstable). |
| `fine_print` | String | The fine print, terms, or conditions of the ticket. |
| `app_link_data` | String | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding object that will be used instead. |
| `hex_background_color` | String | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `links_module_data` | String | Links module data. If links module data is also defined on the object, both will be displayed. |
| `locations` | Vec<String> | Note: This field is currently not supported to trigger geo notifications. |
| `custom_seat_label` | String | A custom label to use for the seat value (`eventTicketObject.seatInfo.seat`) on the card detail view. This should only be used if the default "Seat" label or one of the `seatLabel` options is not sufficient. Both `seatLabel` and `customSeatLabel` may not be set. If neither is set, the label will default to "Seat", localized. If the seat field is unset, this label will not be used. |
| `logo` | String | The logo image of the ticket. This image is displayed in the card detail view of the app. |
| `date_time` | String | The date & time information of the event. |
| `confirmation_code_label` | String | The label to use for the confirmation code value (`eventTicketObject.reservationInfo.confirmationCode`) on the card detail view. Each available option maps to a set of localized strings, so that translations are shown to the user based on their locale. Both `confirmationCodeLabel` and `customConfirmationCodeLabel` may not be set. If neither is set, the label will default to "Confirmation Code", localized. If the confirmation code field is unset, this label will not be used. |
| `custom_row_label` | String | A custom label to use for the row value (`eventTicketObject.seatInfo.row`) on the card detail view. This should only be used if the default "Row" label or one of the `rowLabel` options is not sufficient. Both `rowLabel` and `customRowLabel` may not be set. If neither is set, the label will default to "Row", localized. If the row field is unset, this label will not be used. |
| `merchant_locations` | Vec<String> | Merchant locations. There is a maximum of ten on the class. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `messages` | Vec<String> | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `custom_section_label` | String | A custom label to use for the section value (`eventTicketObject.seatInfo.section`) on the card detail view. This should only be used if the default "Section" label or one of the `sectionLabel` options is not sufficient. Both `sectionLabel` and `customSectionLabel` may not be set. If neither is set, the label will default to "Section", localized. If the section field is unset, this label will not be used. |
| `notify_preference` | String | Whether or not field updates to this class should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If not specified, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `review_status` | String | Required. The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`. |
| `allow_multiple_users_per_object` | bool | Deprecated. Use `multipleDevicesAndHoldersAllowedStatus` instead. |
| `multiple_devices_and_holders_allowed_status` | String | Identifies whether multiple users and devices will save the same object referencing this class. |
| `localized_issuer_name` | String | Translated strings for the issuer_name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `seat_label` | String | The label to use for the seat value (`eventTicketObject.seatInfo.seat`) on the card detail view. Each available option maps to a set of localized strings, so that translations are shown to the user based on their locale. Both `seatLabel` and `customSeatLabel` may not be set. If neither is set, the label will default to "Seat", localized. If the seat field is unset, this label will not be used. |


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
eventticketclas_section_label = eventticketclas.section_label
eventticketclas_value_added_module_data = eventticketclas.value_added_module_data
eventticketclas_issuer_name = eventticketclas.issuer_name
eventticketclas_view_unlock_requirement = eventticketclas.view_unlock_requirement
eventticketclas_callback_options = eventticketclas.callback_options
eventticketclas_wide_logo = eventticketclas.wide_logo
eventticketclas_event_name = eventticketclas.event_name
eventticketclas_redemption_issuers = eventticketclas.redemption_issuers
eventticketclas_review = eventticketclas.review
eventticketclas_class_template_info = eventticketclas.class_template_info
eventticketclas_image_modules_data = eventticketclas.image_modules_data
eventticketclas_info_module_data = eventticketclas.info_module_data
eventticketclas_version = eventticketclas.version
eventticketclas_hero_image = eventticketclas.hero_image
eventticketclas_kind = eventticketclas.kind
eventticketclas_gate_label = eventticketclas.gate_label
eventticketclas_id = eventticketclas.id
eventticketclas_homepage_uri = eventticketclas.homepage_uri
eventticketclas_security_animation = eventticketclas.security_animation
eventticketclas_custom_confirmation_code_label = eventticketclas.custom_confirmation_code_label
eventticketclas_text_modules_data = eventticketclas.text_modules_data
eventticketclas_row_label = eventticketclas.row_label
eventticketclas_venue = eventticketclas.venue
eventticketclas_word_mark = eventticketclas.word_mark
eventticketclas_country_code = eventticketclas.country_code
eventticketclas_enable_smart_tap = eventticketclas.enable_smart_tap
eventticketclas_custom_gate_label = eventticketclas.custom_gate_label
eventticketclas_event_id = eventticketclas.event_id
eventticketclas_fine_print = eventticketclas.fine_print
eventticketclas_app_link_data = eventticketclas.app_link_data
eventticketclas_hex_background_color = eventticketclas.hex_background_color
eventticketclas_links_module_data = eventticketclas.links_module_data
eventticketclas_locations = eventticketclas.locations
eventticketclas_custom_seat_label = eventticketclas.custom_seat_label
eventticketclas_logo = eventticketclas.logo
eventticketclas_date_time = eventticketclas.date_time
eventticketclas_confirmation_code_label = eventticketclas.confirmation_code_label
eventticketclas_custom_row_label = eventticketclas.custom_row_label
eventticketclas_merchant_locations = eventticketclas.merchant_locations
eventticketclas_messages = eventticketclas.messages
eventticketclas_custom_section_label = eventticketclas.custom_section_label
eventticketclas_notify_preference = eventticketclas.notify_preference
eventticketclas_review_status = eventticketclas.review_status
eventticketclas_allow_multiple_users_per_object = eventticketclas.allow_multiple_users_per_object
eventticketclas_multiple_devices_and_holders_allowed_status = eventticketclas.multiple_devices_and_holders_allowed_status
eventticketclas_localized_issuer_name = eventticketclas.localized_issuer_name
eventticketclas_seat_label = eventticketclas.seat_label
```

---


### Transitclas

Inserts a transit class with the given ID and properties.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `custom_carriage_label` | String |  | A custom label to use for the carriage value (`transitObject.ticketLeg.carriage`). |
| `custom_purchase_price_label` | String |  | A custom label to use for the purchase price value (`transitObject.purchaseDetails.ticketCost.purchasePrice`). |
| `messages` | Vec<String> |  | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `value_added_module_data` | Vec<String> |  | Optional value added module data. Maximum of ten on the class. For a pass only ten will be displayed, prioritizing those from the object. |
| `wide_logo` | String |  | The wide logo of the ticket. When provided, this will be used in place of the logo in the top left of the card view. |
| `custom_coach_label` | String |  | A custom label to use for the coach value (`transitObject.ticketLeg.ticketSeat.coach`). |
| `custom_confirmation_code_label` | String |  | A custom label to use for the confirmation code value (`transitObject.purchaseDetails.confirmationCode`). |
| `activation_options` | String |  | Activation options for an activatable ticket. |
| `localized_issuer_name` | String |  | Translated strings for the issuer_name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `hex_background_color` | String |  | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `id` | String |  | Required. The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `merchant_locations` | Vec<String> |  | Merchant locations. There is a maximum of ten on the class. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `view_unlock_requirement` | String |  | View Unlock Requirement options for the transit ticket. |
| `word_mark` | String |  | Deprecated. |
| `custom_purchase_receipt_number_label` | String |  | A custom label to use for the purchase receipt number value (`transitObject.purchaseDetails.purchaseReceiptNumber`). |
| `locations` | Vec<String> |  | Note: This field is currently not supported to trigger geo notifications. |
| `country_code` | String |  | Country code used to display the card's country (when the user is not in that country), as well as to display localized content when content is not available in the user's locale. |
| `logo` | String |  | Required. The logo image of the ticket. This image is displayed in the card detail view of the app. |
| `hero_image` | String |  | Optional banner image displayed on the front of the card. If none is present, nothing will be displayed. The image will display at 100% width. |
| `callback_options` | String |  | Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback. |
| `custom_discount_message_label` | String |  | A custom label to use for the transit discount message value (`transitObject.purchaseDetails.ticketCost.discountMessage`). |
| `app_link_data` | String |  | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding object that will be used instead. |
| `custom_concession_category_label` | String |  | A custom label to use for the transit concession category value (`transitObject.concessionCategory`). |
| `allow_multiple_users_per_object` | bool |  | Deprecated. Use `multipleDevicesAndHoldersAllowedStatus` instead. |
| `custom_time_restrictions_label` | String |  | A custom label to use for the time restrictions details value (`transitObject.ticketRestrictions.timeRestrictions`). |
| `version` | String |  | Deprecated |
| `links_module_data` | String |  | Links module data. If links module data is also defined on the object, both will be displayed. |
| `custom_transit_terminus_name_label` | String |  | A custom label to use for the transit terminus name value (`transitObject.ticketLeg.transitTerminusName`). |
| `class_template_info` | String |  | Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display. |
| `custom_route_restrictions_details_label` | String |  | A custom label to use for the route restrictions details value (`transitObject.ticketRestrictions.routeRestrictionsDetails`). |
| `custom_seat_label` | String |  | A custom label to use for the seat location value (`transitObject.ticketLeg.ticketSeat.seat`). |
| `enable_single_leg_itinerary` | bool |  | Controls the display of the single-leg itinerary for this class. By default, an itinerary will only display for multi-leg trips. |
| `language_override` | String |  | If this field is present, transit tickets served to a user's device will always be in this language. Represents the BCP 47 language tag. Example values are "en-US", "en-GB", "de", or "de-AT". |
| `homepage_uri` | String |  | The URI of your application's home page. Populating the URI in this field results in the exact same behavior as populating an URI in linksModuleData (when an object is rendered, a link to the homepage is shown in what would usually be thought of as the linksModuleData section of the object). |
| `redemption_issuers` | Vec<String> |  | Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `multiple_devices_and_holders_allowed_status` | String |  | Identifies whether multiple users and devices will save the same object referencing this class. |
| `image_modules_data` | Vec<String> |  | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `enable_smart_tap` | bool |  | Identifies whether this class supports Smart Tap. The `redemptionIssuers` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `review_status` | String |  | Required. The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`. |
| `info_module_data` | String |  | Deprecated. Use textModulesData instead. |
| `security_animation` | String |  | Optional information about the security animation. If this is set a security animation will be rendered on pass details. |
| `text_modules_data` | Vec<String> |  | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `review` | String |  | The review comments set by the platform when a class is marked `approved` or `rejected`. |
| `custom_platform_label` | String |  | A custom label to use for the boarding platform value (`transitObject.ticketLeg.platform`). |
| `transit_type` | String |  | Required. The type of transit this class represents, such as "bus". |
| `watermark` | String |  | Watermark image to display on the user's device. |
| `custom_fare_class_label` | String |  | A custom label to use for the fare class value (`transitObject.ticketLeg.ticketSeat.fareClass`). |
| `notify_preference` | String |  | Whether or not field updates to this class should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If set to DO_NOT_NOTIFY or NOTIFICATION_SETTINGS_UNSPECIFIED, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `issuer_name` | String |  | Required. The issuer name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `custom_zone_label` | String |  | A custom label to use for the boarding zone value (`transitObject.ticketLeg.zone`). |
| `custom_ticket_number_label` | String |  | A custom label to use for the ticket number value (`transitObject.ticketNumber`). |
| `custom_fare_name_label` | String |  | A custom label to use for the transit fare name value (`transitObject.ticketLeg.fareName`). |
| `custom_route_restrictions_label` | String |  | A custom label to use for the route restrictions value (`transitObject.ticketRestrictions.routeRestrictions`). |
| `custom_other_restrictions_label` | String |  | A custom label to use for the other restrictions value (`transitObject.ticketRestrictions.otherRestrictions`). |
| `custom_purchase_face_value_label` | String |  | A custom label to use for the purchase face value (`transitObject.purchaseDetails.ticketCost.faceValue`). |
| `transit_operator_name` | String |  | The name of the transit operator. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `custom_carriage_label` | String | A custom label to use for the carriage value (`transitObject.ticketLeg.carriage`). |
| `custom_purchase_price_label` | String | A custom label to use for the purchase price value (`transitObject.purchaseDetails.ticketCost.purchasePrice`). |
| `messages` | Vec<String> | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `value_added_module_data` | Vec<String> | Optional value added module data. Maximum of ten on the class. For a pass only ten will be displayed, prioritizing those from the object. |
| `wide_logo` | String | The wide logo of the ticket. When provided, this will be used in place of the logo in the top left of the card view. |
| `custom_coach_label` | String | A custom label to use for the coach value (`transitObject.ticketLeg.ticketSeat.coach`). |
| `custom_confirmation_code_label` | String | A custom label to use for the confirmation code value (`transitObject.purchaseDetails.confirmationCode`). |
| `activation_options` | String | Activation options for an activatable ticket. |
| `localized_issuer_name` | String | Translated strings for the issuer_name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `hex_background_color` | String | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `id` | String | Required. The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `merchant_locations` | Vec<String> | Merchant locations. There is a maximum of ten on the class. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `view_unlock_requirement` | String | View Unlock Requirement options for the transit ticket. |
| `word_mark` | String | Deprecated. |
| `custom_purchase_receipt_number_label` | String | A custom label to use for the purchase receipt number value (`transitObject.purchaseDetails.purchaseReceiptNumber`). |
| `locations` | Vec<String> | Note: This field is currently not supported to trigger geo notifications. |
| `country_code` | String | Country code used to display the card's country (when the user is not in that country), as well as to display localized content when content is not available in the user's locale. |
| `logo` | String | Required. The logo image of the ticket. This image is displayed in the card detail view of the app. |
| `hero_image` | String | Optional banner image displayed on the front of the card. If none is present, nothing will be displayed. The image will display at 100% width. |
| `callback_options` | String | Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback. |
| `custom_discount_message_label` | String | A custom label to use for the transit discount message value (`transitObject.purchaseDetails.ticketCost.discountMessage`). |
| `app_link_data` | String | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding object that will be used instead. |
| `custom_concession_category_label` | String | A custom label to use for the transit concession category value (`transitObject.concessionCategory`). |
| `allow_multiple_users_per_object` | bool | Deprecated. Use `multipleDevicesAndHoldersAllowedStatus` instead. |
| `custom_time_restrictions_label` | String | A custom label to use for the time restrictions details value (`transitObject.ticketRestrictions.timeRestrictions`). |
| `version` | String | Deprecated |
| `links_module_data` | String | Links module data. If links module data is also defined on the object, both will be displayed. |
| `custom_transit_terminus_name_label` | String | A custom label to use for the transit terminus name value (`transitObject.ticketLeg.transitTerminusName`). |
| `class_template_info` | String | Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display. |
| `custom_route_restrictions_details_label` | String | A custom label to use for the route restrictions details value (`transitObject.ticketRestrictions.routeRestrictionsDetails`). |
| `custom_seat_label` | String | A custom label to use for the seat location value (`transitObject.ticketLeg.ticketSeat.seat`). |
| `enable_single_leg_itinerary` | bool | Controls the display of the single-leg itinerary for this class. By default, an itinerary will only display for multi-leg trips. |
| `language_override` | String | If this field is present, transit tickets served to a user's device will always be in this language. Represents the BCP 47 language tag. Example values are "en-US", "en-GB", "de", or "de-AT". |
| `homepage_uri` | String | The URI of your application's home page. Populating the URI in this field results in the exact same behavior as populating an URI in linksModuleData (when an object is rendered, a link to the homepage is shown in what would usually be thought of as the linksModuleData section of the object). |
| `redemption_issuers` | Vec<String> | Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `multiple_devices_and_holders_allowed_status` | String | Identifies whether multiple users and devices will save the same object referencing this class. |
| `image_modules_data` | Vec<String> | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `enable_smart_tap` | bool | Identifies whether this class supports Smart Tap. The `redemptionIssuers` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `review_status` | String | Required. The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`. |
| `info_module_data` | String | Deprecated. Use textModulesData instead. |
| `security_animation` | String | Optional information about the security animation. If this is set a security animation will be rendered on pass details. |
| `text_modules_data` | Vec<String> | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `review` | String | The review comments set by the platform when a class is marked `approved` or `rejected`. |
| `custom_platform_label` | String | A custom label to use for the boarding platform value (`transitObject.ticketLeg.platform`). |
| `transit_type` | String | Required. The type of transit this class represents, such as "bus". |
| `watermark` | String | Watermark image to display on the user's device. |
| `custom_fare_class_label` | String | A custom label to use for the fare class value (`transitObject.ticketLeg.ticketSeat.fareClass`). |
| `notify_preference` | String | Whether or not field updates to this class should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If set to DO_NOT_NOTIFY or NOTIFICATION_SETTINGS_UNSPECIFIED, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `issuer_name` | String | Required. The issuer name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `custom_zone_label` | String | A custom label to use for the boarding zone value (`transitObject.ticketLeg.zone`). |
| `custom_ticket_number_label` | String | A custom label to use for the ticket number value (`transitObject.ticketNumber`). |
| `custom_fare_name_label` | String | A custom label to use for the transit fare name value (`transitObject.ticketLeg.fareName`). |
| `custom_route_restrictions_label` | String | A custom label to use for the route restrictions value (`transitObject.ticketRestrictions.routeRestrictions`). |
| `custom_other_restrictions_label` | String | A custom label to use for the other restrictions value (`transitObject.ticketRestrictions.otherRestrictions`). |
| `custom_purchase_face_value_label` | String | A custom label to use for the purchase face value (`transitObject.purchaseDetails.ticketCost.faceValue`). |
| `transit_operator_name` | String | The name of the transit operator. |


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
transitclas_custom_carriage_label = transitclas.custom_carriage_label
transitclas_custom_purchase_price_label = transitclas.custom_purchase_price_label
transitclas_messages = transitclas.messages
transitclas_value_added_module_data = transitclas.value_added_module_data
transitclas_wide_logo = transitclas.wide_logo
transitclas_custom_coach_label = transitclas.custom_coach_label
transitclas_custom_confirmation_code_label = transitclas.custom_confirmation_code_label
transitclas_activation_options = transitclas.activation_options
transitclas_localized_issuer_name = transitclas.localized_issuer_name
transitclas_hex_background_color = transitclas.hex_background_color
transitclas_id = transitclas.id
transitclas_merchant_locations = transitclas.merchant_locations
transitclas_view_unlock_requirement = transitclas.view_unlock_requirement
transitclas_word_mark = transitclas.word_mark
transitclas_custom_purchase_receipt_number_label = transitclas.custom_purchase_receipt_number_label
transitclas_locations = transitclas.locations
transitclas_country_code = transitclas.country_code
transitclas_logo = transitclas.logo
transitclas_hero_image = transitclas.hero_image
transitclas_callback_options = transitclas.callback_options
transitclas_custom_discount_message_label = transitclas.custom_discount_message_label
transitclas_app_link_data = transitclas.app_link_data
transitclas_custom_concession_category_label = transitclas.custom_concession_category_label
transitclas_allow_multiple_users_per_object = transitclas.allow_multiple_users_per_object
transitclas_custom_time_restrictions_label = transitclas.custom_time_restrictions_label
transitclas_version = transitclas.version
transitclas_links_module_data = transitclas.links_module_data
transitclas_custom_transit_terminus_name_label = transitclas.custom_transit_terminus_name_label
transitclas_class_template_info = transitclas.class_template_info
transitclas_custom_route_restrictions_details_label = transitclas.custom_route_restrictions_details_label
transitclas_custom_seat_label = transitclas.custom_seat_label
transitclas_enable_single_leg_itinerary = transitclas.enable_single_leg_itinerary
transitclas_language_override = transitclas.language_override
transitclas_homepage_uri = transitclas.homepage_uri
transitclas_redemption_issuers = transitclas.redemption_issuers
transitclas_multiple_devices_and_holders_allowed_status = transitclas.multiple_devices_and_holders_allowed_status
transitclas_image_modules_data = transitclas.image_modules_data
transitclas_enable_smart_tap = transitclas.enable_smart_tap
transitclas_review_status = transitclas.review_status
transitclas_info_module_data = transitclas.info_module_data
transitclas_security_animation = transitclas.security_animation
transitclas_text_modules_data = transitclas.text_modules_data
transitclas_review = transitclas.review
transitclas_custom_platform_label = transitclas.custom_platform_label
transitclas_transit_type = transitclas.transit_type
transitclas_watermark = transitclas.watermark
transitclas_custom_fare_class_label = transitclas.custom_fare_class_label
transitclas_notify_preference = transitclas.notify_preference
transitclas_issuer_name = transitclas.issuer_name
transitclas_custom_zone_label = transitclas.custom_zone_label
transitclas_custom_ticket_number_label = transitclas.custom_ticket_number_label
transitclas_custom_fare_name_label = transitclas.custom_fare_name_label
transitclas_custom_route_restrictions_label = transitclas.custom_route_restrictions_label
transitclas_custom_other_restrictions_label = transitclas.custom_other_restrictions_label
transitclas_custom_purchase_face_value_label = transitclas.custom_purchase_face_value_label
transitclas_transit_operator_name = transitclas.transit_operator_name
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


### Giftcardobject

Inserts an gift card object with the given ID and properties.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `valid_time_interval` | String |  | The time period this object will be `active` and object can be used. An object's state will be changed to `expired` when this time period has passed. |
| `balance_update_time` | String |  | The date and time when the balance was last updated. Offset is required. If balance is updated and this property is not provided, system will default to the current time. |
| `notify_preference` | String |  | Whether or not field updates to this object should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If set to DO_NOT_NOTIFY or NOTIFICATION_SETTINGS_UNSPECIFIED, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `image_modules_data` | Vec<String> |  | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `rotating_barcode` | String |  | The rotating barcode type and value. |
| `card_number` | String |  | Required. The card's number. |
| `state` | String |  | Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section. |
| `barcode` | String |  | The barcode type and value. |
| `pin` | String |  | The card's PIN. |
| `hero_image` | String |  | Optional banner image displayed on the front of the card. If none is present, hero image of the class, if present, will be displayed. If hero image of the class is also not present, nothing will be displayed. |
| `smart_tap_redemption_value` | String |  | The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported. |
| `version` | String |  | Deprecated |
| `has_linked_device` | bool |  | Whether this object is currently linked to a single device. This field is set by the platform when a user saves the object, linking it to their device. Intended for use by select partners. Contact support for additional information. |
| `app_link_data` | String |  | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding class only object AppLinkData will be displayed. |
| `pass_constraints` | String |  | Pass constraints for the object. Includes limiting NFC and screenshot behaviors. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#giftCardObject"`. |
| `text_modules_data` | Vec<String> |  | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `value_added_module_data` | Vec<String> |  | Optional value added module data. Maximum of ten on the object. |
| `id` | String |  | Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. The unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `balance` | String |  | The card's monetary balance. |
| `disable_expiration_notification` | bool |  | Indicates if notifications should explicitly be suppressed. If this field is set to true, regardless of the `messages` field, expiration notifications to the user will be suppressed. By default, this field is set to false. Currently, this can only be set for offers. |
| `event_number` | String |  | The card's event number, an optional field used by some gift cards. |
| `merchant_locations` | Vec<String> |  | Merchant locations. There is a maximum of ten on the object. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `class_id` | String |  | Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. |
| `links_module_data` | String |  | Links module data. If links module data is also defined on the class, both will be displayed. |
| `has_users` | bool |  | Indicates if the object has users. This field is set by the platform. |
| `locations` | Vec<String> |  | Note: This field is currently not supported to trigger geo notifications. |
| `info_module_data` | String |  | Deprecated. Use textModulesData instead. |
| `messages` | Vec<String> |  | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `class_reference` | String |  | A copy of the inherited fields of the parent class. These fields are retrieved during a GET. |
| `linked_object_ids` | Vec<String> |  | linked_object_ids are a list of other objects such as event ticket, loyalty, offer, generic, giftcard, transit and boarding pass that should be automatically attached to this giftcard object. If a user had saved this gift card, then these linked_object_ids would be automatically pushed to the user's wallet (unless they turned off the setting to receive such linked passes). Make sure that objects present in linked_object_ids are already inserted - if not, calls would fail. Once linked, the linked objects cannot be unlinked. You cannot link objects belonging to another issuer. There is a limit to the number of objects that can be linked to a single object. After the limit is reached, new linked objects in the call will be ignored silently. Object IDs should follow the format issuer ID. identifier where the former is issued by Google and the latter is chosen by you. |
| `grouping_info` | String |  | Information that controls how passes are grouped together. |
| `save_restrictions` | String |  | Restrictions on the object that needs to be verified before the user tries to save the pass. Note that this restrictions will only be applied during save time. If the restrictions changed after a user saves the pass, the new restrictions will not be applied to an already saved pass. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `valid_time_interval` | String | The time period this object will be `active` and object can be used. An object's state will be changed to `expired` when this time period has passed. |
| `balance_update_time` | String | The date and time when the balance was last updated. Offset is required. If balance is updated and this property is not provided, system will default to the current time. |
| `notify_preference` | String | Whether or not field updates to this object should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If set to DO_NOT_NOTIFY or NOTIFICATION_SETTINGS_UNSPECIFIED, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `image_modules_data` | Vec<String> | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `rotating_barcode` | String | The rotating barcode type and value. |
| `card_number` | String | Required. The card's number. |
| `state` | String | Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section. |
| `barcode` | String | The barcode type and value. |
| `pin` | String | The card's PIN. |
| `hero_image` | String | Optional banner image displayed on the front of the card. If none is present, hero image of the class, if present, will be displayed. If hero image of the class is also not present, nothing will be displayed. |
| `smart_tap_redemption_value` | String | The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported. |
| `version` | String | Deprecated |
| `has_linked_device` | bool | Whether this object is currently linked to a single device. This field is set by the platform when a user saves the object, linking it to their device. Intended for use by select partners. Contact support for additional information. |
| `app_link_data` | String | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding class only object AppLinkData will be displayed. |
| `pass_constraints` | String | Pass constraints for the object. Includes limiting NFC and screenshot behaviors. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#giftCardObject"`. |
| `text_modules_data` | Vec<String> | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `value_added_module_data` | Vec<String> | Optional value added module data. Maximum of ten on the object. |
| `id` | String | Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. The unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `balance` | String | The card's monetary balance. |
| `disable_expiration_notification` | bool | Indicates if notifications should explicitly be suppressed. If this field is set to true, regardless of the `messages` field, expiration notifications to the user will be suppressed. By default, this field is set to false. Currently, this can only be set for offers. |
| `event_number` | String | The card's event number, an optional field used by some gift cards. |
| `merchant_locations` | Vec<String> | Merchant locations. There is a maximum of ten on the object. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `class_id` | String | Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. |
| `links_module_data` | String | Links module data. If links module data is also defined on the class, both will be displayed. |
| `has_users` | bool | Indicates if the object has users. This field is set by the platform. |
| `locations` | Vec<String> | Note: This field is currently not supported to trigger geo notifications. |
| `info_module_data` | String | Deprecated. Use textModulesData instead. |
| `messages` | Vec<String> | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `class_reference` | String | A copy of the inherited fields of the parent class. These fields are retrieved during a GET. |
| `linked_object_ids` | Vec<String> | linked_object_ids are a list of other objects such as event ticket, loyalty, offer, generic, giftcard, transit and boarding pass that should be automatically attached to this giftcard object. If a user had saved this gift card, then these linked_object_ids would be automatically pushed to the user's wallet (unless they turned off the setting to receive such linked passes). Make sure that objects present in linked_object_ids are already inserted - if not, calls would fail. Once linked, the linked objects cannot be unlinked. You cannot link objects belonging to another issuer. There is a limit to the number of objects that can be linked to a single object. After the limit is reached, new linked objects in the call will be ignored silently. Object IDs should follow the format issuer ID. identifier where the former is issued by Google and the latter is chosen by you. |
| `grouping_info` | String | Information that controls how passes are grouped together. |
| `save_restrictions` | String | Restrictions on the object that needs to be verified before the user tries to save the pass. Note that this restrictions will only be applied during save time. If the restrictions changed after a user saves the pass, the new restrictions will not be applied to an already saved pass. |


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
giftcardobject_valid_time_interval = giftcardobject.valid_time_interval
giftcardobject_balance_update_time = giftcardobject.balance_update_time
giftcardobject_notify_preference = giftcardobject.notify_preference
giftcardobject_image_modules_data = giftcardobject.image_modules_data
giftcardobject_rotating_barcode = giftcardobject.rotating_barcode
giftcardobject_card_number = giftcardobject.card_number
giftcardobject_state = giftcardobject.state
giftcardobject_barcode = giftcardobject.barcode
giftcardobject_pin = giftcardobject.pin
giftcardobject_hero_image = giftcardobject.hero_image
giftcardobject_smart_tap_redemption_value = giftcardobject.smart_tap_redemption_value
giftcardobject_version = giftcardobject.version
giftcardobject_has_linked_device = giftcardobject.has_linked_device
giftcardobject_app_link_data = giftcardobject.app_link_data
giftcardobject_pass_constraints = giftcardobject.pass_constraints
giftcardobject_kind = giftcardobject.kind
giftcardobject_text_modules_data = giftcardobject.text_modules_data
giftcardobject_value_added_module_data = giftcardobject.value_added_module_data
giftcardobject_id = giftcardobject.id
giftcardobject_balance = giftcardobject.balance
giftcardobject_disable_expiration_notification = giftcardobject.disable_expiration_notification
giftcardobject_event_number = giftcardobject.event_number
giftcardobject_merchant_locations = giftcardobject.merchant_locations
giftcardobject_class_id = giftcardobject.class_id
giftcardobject_links_module_data = giftcardobject.links_module_data
giftcardobject_has_users = giftcardobject.has_users
giftcardobject_locations = giftcardobject.locations
giftcardobject_info_module_data = giftcardobject.info_module_data
giftcardobject_messages = giftcardobject.messages
giftcardobject_class_reference = giftcardobject.class_reference
giftcardobject_linked_object_ids = giftcardobject.linked_object_ids
giftcardobject_grouping_info = giftcardobject.grouping_info
giftcardobject_save_restrictions = giftcardobject.save_restrictions
```

---


### Transitobject

Inserts an transit object with the given ID and properties.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `locations` | Vec<String> |  | Note: This field is currently not supported to trigger geo notifications. |
| `notify_preference` | String |  | Whether or not field updates to this object should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If set to DO_NOT_NOTIFY or NOTIFICATION_SETTINGS_UNSPECIFIED, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `ticket_number` | String |  | The number of the ticket. This is a unique identifier for the ticket in the transit operator's system. |
| `state` | String |  | Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section. |
| `ticket_legs` | Vec<String> |  | Each ticket may contain one or more legs. Each leg contains departure and arrival information along with boarding and seating information. If only one leg is to be specified then use the `ticketLeg` field instead. Both `ticketLeg` and `ticketLegs` may not be set. |
| `grouping_info` | String |  | Information that controls how passes are grouped together. |
| `has_linked_device` | bool |  | Whether this object is currently linked to a single device. This field is set by the platform when a user saves the object, linking it to their device. Intended for use by select partners. Contact support for additional information. |
| `value_added_module_data` | Vec<String> |  | Optional value added module data. Maximum of ten on the object. |
| `version` | String |  | Deprecated |
| `links_module_data` | String |  | Links module data. If links module data is also defined on the class, both will be displayed. |
| `passenger_type` | String |  | The number of passengers. |
| `save_restrictions` | String |  | Restrictions on the object that needs to be verified before the user tries to save the pass. Note that this restrictions will only be applied during save time. If the restrictions changed after a user saves the pass, the new restrictions will not be applied to an already saved pass. |
| `app_link_data` | String |  | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding class only object AppLinkData will be displayed. |
| `activation_status` | String |  | The activation status for the object. Required if the class has `activationOptions` set. |
| `text_modules_data` | Vec<String> |  | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `passenger_names` | String |  | The name(s) of the passengers the ticket is assigned to. The above `passengerType` field is meant to give Google context on this field. |
| `id` | String |  | Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. The unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `concession_category` | String |  | The concession category for the ticket. |
| `purchase_details` | String |  | Purchase details for this ticket. |
| `hero_image` | String |  | Optional banner image displayed on the front of the card. If none is present, hero image of the class, if present, will be displayed. If hero image of the class is also not present, nothing will be displayed. |
| `ticket_leg` | String |  | A single ticket leg contains departure and arrival information along with boarding and seating information. If more than one leg is to be specified then use the `ticketLegs` field instead. Both `ticketLeg` and `ticketLegs` may not be set. |
| `disable_expiration_notification` | bool |  | Indicates if notifications should explicitly be suppressed. If this field is set to true, regardless of the `messages` field, expiration notifications to the user will be suppressed. By default, this field is set to false. Currently, this can only be set for offers. |
| `ticket_status` | String |  | The status of the ticket. For states which affect display, use the `state` field instead. |
| `trip_id` | String |  | This id is used to group tickets together if the user has saved multiple tickets for the same trip. |
| `linked_object_ids` | Vec<String> |  | linked_object_ids are a list of other objects such as event ticket, loyalty, offer, generic, giftcard, transit and boarding pass that should be automatically attached to this transit object. If a user had saved this transit card, then these linked_object_ids would be automatically pushed to the user's wallet (unless they turned off the setting to receive such linked passes). Make sure that objects present in linked_object_ids are already inserted - if not, calls would fail. Once linked, the linked objects cannot be unlinked. You cannot link objects belonging to another issuer. There is a limit to the number of objects that can be linked to a single object. After the limit is reached, new linked objects in the call will be ignored silently. Object IDs should follow the format issuer ID. identifier where the former is issued by Google and the latter is chosen by you. |
| `custom_ticket_status` | String |  | A custom status to use for the ticket status value when `ticketStatus` does not provide the right option. Both `ticketStatus` and `customTicketStatus` may not be set. |
| `rotating_barcode` | String |  | The rotating barcode type and value. |
| `custom_concession_category` | String |  | A custom concession category to use when `concessionCategory` does not provide the right option. Both `concessionCategory` and `customConcessionCategory` may not be set. |
| `hex_background_color` | String |  | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `smart_tap_redemption_value` | String |  | The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported. |
| `barcode` | String |  | The barcode type and value. |
| `ticket_restrictions` | String |  | Information about what kind of restrictions there are on using this ticket. For example, which days of the week it must be used, or which routes are allowed to be taken. |
| `pass_constraints` | String |  | Pass constraints for the object. Includes limiting NFC and screenshot behaviors. |
| `merchant_locations` | Vec<String> |  | Merchant locations. There is a maximum of ten on the object. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `trip_type` | String |  | Required. The type of trip this transit object represents. Used to determine the pass title and/or which symbol to use between the origin and destination. |
| `messages` | Vec<String> |  | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `device_context` | String |  | Device context associated with the object. |
| `image_modules_data` | Vec<String> |  | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `valid_time_interval` | String |  | The time period this object will be `active` and object can be used. An object's state will be changed to `expired` when this time period has passed. |
| `class_id` | String |  | Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. |
| `info_module_data` | String |  | Deprecated. Use textModulesData instead. |
| `has_users` | bool |  | Indicates if the object has users. This field is set by the platform. |
| `class_reference` | String |  | A copy of the inherited fields of the parent class. These fields are retrieved during a GET. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `locations` | Vec<String> | Note: This field is currently not supported to trigger geo notifications. |
| `notify_preference` | String | Whether or not field updates to this object should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If set to DO_NOT_NOTIFY or NOTIFICATION_SETTINGS_UNSPECIFIED, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `ticket_number` | String | The number of the ticket. This is a unique identifier for the ticket in the transit operator's system. |
| `state` | String | Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section. |
| `ticket_legs` | Vec<String> | Each ticket may contain one or more legs. Each leg contains departure and arrival information along with boarding and seating information. If only one leg is to be specified then use the `ticketLeg` field instead. Both `ticketLeg` and `ticketLegs` may not be set. |
| `grouping_info` | String | Information that controls how passes are grouped together. |
| `has_linked_device` | bool | Whether this object is currently linked to a single device. This field is set by the platform when a user saves the object, linking it to their device. Intended for use by select partners. Contact support for additional information. |
| `value_added_module_data` | Vec<String> | Optional value added module data. Maximum of ten on the object. |
| `version` | String | Deprecated |
| `links_module_data` | String | Links module data. If links module data is also defined on the class, both will be displayed. |
| `passenger_type` | String | The number of passengers. |
| `save_restrictions` | String | Restrictions on the object that needs to be verified before the user tries to save the pass. Note that this restrictions will only be applied during save time. If the restrictions changed after a user saves the pass, the new restrictions will not be applied to an already saved pass. |
| `app_link_data` | String | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding class only object AppLinkData will be displayed. |
| `activation_status` | String | The activation status for the object. Required if the class has `activationOptions` set. |
| `text_modules_data` | Vec<String> | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `passenger_names` | String | The name(s) of the passengers the ticket is assigned to. The above `passengerType` field is meant to give Google context on this field. |
| `id` | String | Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. The unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `concession_category` | String | The concession category for the ticket. |
| `purchase_details` | String | Purchase details for this ticket. |
| `hero_image` | String | Optional banner image displayed on the front of the card. If none is present, hero image of the class, if present, will be displayed. If hero image of the class is also not present, nothing will be displayed. |
| `ticket_leg` | String | A single ticket leg contains departure and arrival information along with boarding and seating information. If more than one leg is to be specified then use the `ticketLegs` field instead. Both `ticketLeg` and `ticketLegs` may not be set. |
| `disable_expiration_notification` | bool | Indicates if notifications should explicitly be suppressed. If this field is set to true, regardless of the `messages` field, expiration notifications to the user will be suppressed. By default, this field is set to false. Currently, this can only be set for offers. |
| `ticket_status` | String | The status of the ticket. For states which affect display, use the `state` field instead. |
| `trip_id` | String | This id is used to group tickets together if the user has saved multiple tickets for the same trip. |
| `linked_object_ids` | Vec<String> | linked_object_ids are a list of other objects such as event ticket, loyalty, offer, generic, giftcard, transit and boarding pass that should be automatically attached to this transit object. If a user had saved this transit card, then these linked_object_ids would be automatically pushed to the user's wallet (unless they turned off the setting to receive such linked passes). Make sure that objects present in linked_object_ids are already inserted - if not, calls would fail. Once linked, the linked objects cannot be unlinked. You cannot link objects belonging to another issuer. There is a limit to the number of objects that can be linked to a single object. After the limit is reached, new linked objects in the call will be ignored silently. Object IDs should follow the format issuer ID. identifier where the former is issued by Google and the latter is chosen by you. |
| `custom_ticket_status` | String | A custom status to use for the ticket status value when `ticketStatus` does not provide the right option. Both `ticketStatus` and `customTicketStatus` may not be set. |
| `rotating_barcode` | String | The rotating barcode type and value. |
| `custom_concession_category` | String | A custom concession category to use when `concessionCategory` does not provide the right option. Both `concessionCategory` and `customConcessionCategory` may not be set. |
| `hex_background_color` | String | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `smart_tap_redemption_value` | String | The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported. |
| `barcode` | String | The barcode type and value. |
| `ticket_restrictions` | String | Information about what kind of restrictions there are on using this ticket. For example, which days of the week it must be used, or which routes are allowed to be taken. |
| `pass_constraints` | String | Pass constraints for the object. Includes limiting NFC and screenshot behaviors. |
| `merchant_locations` | Vec<String> | Merchant locations. There is a maximum of ten on the object. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `trip_type` | String | Required. The type of trip this transit object represents. Used to determine the pass title and/or which symbol to use between the origin and destination. |
| `messages` | Vec<String> | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `device_context` | String | Device context associated with the object. |
| `image_modules_data` | Vec<String> | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `valid_time_interval` | String | The time period this object will be `active` and object can be used. An object's state will be changed to `expired` when this time period has passed. |
| `class_id` | String | Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. |
| `info_module_data` | String | Deprecated. Use textModulesData instead. |
| `has_users` | bool | Indicates if the object has users. This field is set by the platform. |
| `class_reference` | String | A copy of the inherited fields of the parent class. These fields are retrieved during a GET. |


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
transitobject_locations = transitobject.locations
transitobject_notify_preference = transitobject.notify_preference
transitobject_ticket_number = transitobject.ticket_number
transitobject_state = transitobject.state
transitobject_ticket_legs = transitobject.ticket_legs
transitobject_grouping_info = transitobject.grouping_info
transitobject_has_linked_device = transitobject.has_linked_device
transitobject_value_added_module_data = transitobject.value_added_module_data
transitobject_version = transitobject.version
transitobject_links_module_data = transitobject.links_module_data
transitobject_passenger_type = transitobject.passenger_type
transitobject_save_restrictions = transitobject.save_restrictions
transitobject_app_link_data = transitobject.app_link_data
transitobject_activation_status = transitobject.activation_status
transitobject_text_modules_data = transitobject.text_modules_data
transitobject_passenger_names = transitobject.passenger_names
transitobject_id = transitobject.id
transitobject_concession_category = transitobject.concession_category
transitobject_purchase_details = transitobject.purchase_details
transitobject_hero_image = transitobject.hero_image
transitobject_ticket_leg = transitobject.ticket_leg
transitobject_disable_expiration_notification = transitobject.disable_expiration_notification
transitobject_ticket_status = transitobject.ticket_status
transitobject_trip_id = transitobject.trip_id
transitobject_linked_object_ids = transitobject.linked_object_ids
transitobject_custom_ticket_status = transitobject.custom_ticket_status
transitobject_rotating_barcode = transitobject.rotating_barcode
transitobject_custom_concession_category = transitobject.custom_concession_category
transitobject_hex_background_color = transitobject.hex_background_color
transitobject_smart_tap_redemption_value = transitobject.smart_tap_redemption_value
transitobject_barcode = transitobject.barcode
transitobject_ticket_restrictions = transitobject.ticket_restrictions
transitobject_pass_constraints = transitobject.pass_constraints
transitobject_merchant_locations = transitobject.merchant_locations
transitobject_trip_type = transitobject.trip_type
transitobject_messages = transitobject.messages
transitobject_device_context = transitobject.device_context
transitobject_image_modules_data = transitobject.image_modules_data
transitobject_valid_time_interval = transitobject.valid_time_interval
transitobject_class_id = transitobject.class_id
transitobject_info_module_data = transitobject.info_module_data
transitobject_has_users = transitobject.has_users
transitobject_class_reference = transitobject.class_reference
```

---


### Flightclas

Inserts an flight class with the given ID and properties.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `info_module_data` | String |  | Deprecated. Use textModulesData instead. |
| `localized_issuer_name` | String |  | Translated strings for the issuer_name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `hero_image` | String |  | Optional banner image displayed on the front of the card. If none is present, nothing will be displayed. The image will display at 100% width. |
| `flight_status` | String |  | Status of this flight. If unset, Google will compute status based on data from other sources, such as FlightStats, etc. Note: Google-computed status will not be returned in API responses. |
| `issuer_name` | String |  | Required. The issuer name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `allow_multiple_users_per_object` | bool |  | Deprecated. Use `multipleDevicesAndHoldersAllowedStatus` instead. |
| `review_status` | String |  | Required. The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`. |
| `word_mark` | String |  | Deprecated. |
| `class_template_info` | String |  | Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display. |
| `destination` | String |  | Required. Destination airport. |
| `review` | String |  | The review comments set by the platform when a class is marked `approved` or `rejected`. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#flightClass"`. |
| `view_unlock_requirement` | String |  | View Unlock Requirement options for the boarding pass. |
| `boarding_and_seating_policy` | String |  | Policies for boarding and seating. These will inform which labels will be shown to users. |
| `language_override` | String |  | If this field is present, boarding passes served to a user's device will always be in this language. Represents the BCP 47 language tag. Example values are "en-US", "en-GB", "de", or "de-AT". |
| `links_module_data` | String |  | Links module data. If links module data is also defined on the object, both will be displayed. |
| `multiple_devices_and_holders_allowed_status` | String |  | Identifies whether multiple users and devices will save the same object referencing this class. |
| `locations` | Vec<String> |  | Note: This field is currently not supported to trigger geo notifications. |
| `app_link_data` | String |  | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding object that will be used instead. |
| `messages` | Vec<String> |  | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `id` | String |  | Required. The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `image_modules_data` | Vec<String> |  | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `local_estimated_or_actual_arrival_date_time` | String |  | The estimated time the aircraft plans to reach the destination gate (not the runway) or the actual time it reached the gate. This field should be set if at least one of the below is true: - It differs from the scheduled time. Google will use it to calculate the delay. - The aircraft already arrived at the gate. Google will use it to inform the user that the flight has arrived at the gate. This is an ISO 8601 extended format date/time without an offset. Time may be specified up to millisecond precision. eg: `2027-03-05T06:30:00` This should be the local date/time at the airport (not a UTC time). Google will reject the request if UTC offset is provided. Time zones will be calculated by Google based on arrival airport. |
| `redemption_issuers` | Vec<String> |  | Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `text_modules_data` | Vec<String> |  | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `value_added_module_data` | Vec<String> |  | Optional value added module data. Maximum of ten on the class. For a pass only ten will be displayed, prioritizing those from the object. |
| `local_scheduled_departure_date_time` | String |  | Required. The scheduled date and time when the aircraft is expected to depart the gate (not the runway) Note: This field should not change too close to the departure time. For updates to departure times (delays, etc), please set `localEstimatedOrActualDepartureDateTime`. This is an ISO 8601 extended format date/time without an offset. Time may be specified up to millisecond precision. eg: `2027-03-05T06:30:00` This should be the local date/time at the airport (not a UTC time). Google will reject the request if UTC offset is provided. Time zones will be calculated by Google based on departure airport. |
| `hex_background_color` | String |  | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `origin` | String |  | Required. Origin airport. |
| `local_boarding_date_time` | String |  | The boarding time as it would be printed on the boarding pass. This is an ISO 8601 extended format date/time without an offset. Time may be specified up to millisecond precision. eg: `2027-03-05T06:30:00` This should be the local date/time at the airport (not a UTC time). Google will reject the request if UTC offset is provided. Time zones will be calculated by Google based on departure airport. |
| `local_gate_closing_date_time` | String |  | The gate closing time as it would be printed on the boarding pass. Do not set this field if you do not want to print it in the boarding pass. This is an ISO 8601 extended format date/time without an offset. Time may be specified up to millisecond precision. eg: `2027-03-05T06:30:00` This should be the local date/time at the airport (not a UTC time). Google will reject the request if UTC offset is provided. Time zones will be calculated by Google based on departure airport. |
| `callback_options` | String |  | Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback. |
| `homepage_uri` | String |  | The URI of your application's home page. Populating the URI in this field results in the exact same behavior as populating an URI in linksModuleData (when an object is rendered, a link to the homepage is shown in what would usually be thought of as the linksModuleData section of the object). |
| `flight_header` | String |  | Required. Information about the flight carrier and number. |
| `local_scheduled_arrival_date_time` | String |  | The scheduled time the aircraft plans to reach the destination gate (not the runway). Note: This field should not change too close to the flight time. For updates to departure times (delays, etc), please set `localEstimatedOrActualArrivalDateTime`. This is an ISO 8601 extended format date/time without an offset. Time may be specified up to millisecond precision. eg: `2027-03-05T06:30:00` This should be the local date/time at the airport (not a UTC time). Google will reject the request if UTC offset is provided. Time zones will be calculated by Google based on arrival airport. |
| `local_estimated_or_actual_departure_date_time` | String |  | The estimated time the aircraft plans to pull from the gate or the actual time the aircraft already pulled from the gate. Note: This is not the runway time. This field should be set if at least one of the below is true: - It differs from the scheduled time. Google will use it to calculate the delay. - The aircraft already pulled from the gate. Google will use it to inform the user when the flight actually departed. This is an ISO 8601 extended format date/time without an offset. Time may be specified up to millisecond precision. eg: `2027-03-05T06:30:00` This should be the local date/time at the airport (not a UTC time). Google will reject the request if UTC offset is provided. Time zones will be calculated by Google based on departure airport. |
| `merchant_locations` | Vec<String> |  | Merchant locations. There is a maximum of ten on the class. Any additional MerchantLocations added beyond the 10 will be rejected by the validator. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `country_code` | String |  | Country code used to display the card's country (when the user is not in that country), as well as to display localized content when content is not available in the user's locale. |
| `notify_preference` | String |  | Whether or not field updates to this class should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If not specified, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `security_animation` | String |  | Optional information about the security animation. If this is set a security animation will be rendered on pass details. |
| `enable_smart_tap` | bool |  | Identifies whether this class supports Smart Tap. The `redemptionIssuers` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `version` | String |  | Deprecated |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `info_module_data` | String | Deprecated. Use textModulesData instead. |
| `localized_issuer_name` | String | Translated strings for the issuer_name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `hero_image` | String | Optional banner image displayed on the front of the card. If none is present, nothing will be displayed. The image will display at 100% width. |
| `flight_status` | String | Status of this flight. If unset, Google will compute status based on data from other sources, such as FlightStats, etc. Note: Google-computed status will not be returned in API responses. |
| `issuer_name` | String | Required. The issuer name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `allow_multiple_users_per_object` | bool | Deprecated. Use `multipleDevicesAndHoldersAllowedStatus` instead. |
| `review_status` | String | Required. The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`. |
| `word_mark` | String | Deprecated. |
| `class_template_info` | String | Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display. |
| `destination` | String | Required. Destination airport. |
| `review` | String | The review comments set by the platform when a class is marked `approved` or `rejected`. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#flightClass"`. |
| `view_unlock_requirement` | String | View Unlock Requirement options for the boarding pass. |
| `boarding_and_seating_policy` | String | Policies for boarding and seating. These will inform which labels will be shown to users. |
| `language_override` | String | If this field is present, boarding passes served to a user's device will always be in this language. Represents the BCP 47 language tag. Example values are "en-US", "en-GB", "de", or "de-AT". |
| `links_module_data` | String | Links module data. If links module data is also defined on the object, both will be displayed. |
| `multiple_devices_and_holders_allowed_status` | String | Identifies whether multiple users and devices will save the same object referencing this class. |
| `locations` | Vec<String> | Note: This field is currently not supported to trigger geo notifications. |
| `app_link_data` | String | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding object that will be used instead. |
| `messages` | Vec<String> | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `id` | String | Required. The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `image_modules_data` | Vec<String> | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `local_estimated_or_actual_arrival_date_time` | String | The estimated time the aircraft plans to reach the destination gate (not the runway) or the actual time it reached the gate. This field should be set if at least one of the below is true: - It differs from the scheduled time. Google will use it to calculate the delay. - The aircraft already arrived at the gate. Google will use it to inform the user that the flight has arrived at the gate. This is an ISO 8601 extended format date/time without an offset. Time may be specified up to millisecond precision. eg: `2027-03-05T06:30:00` This should be the local date/time at the airport (not a UTC time). Google will reject the request if UTC offset is provided. Time zones will be calculated by Google based on arrival airport. |
| `redemption_issuers` | Vec<String> | Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `text_modules_data` | Vec<String> | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `value_added_module_data` | Vec<String> | Optional value added module data. Maximum of ten on the class. For a pass only ten will be displayed, prioritizing those from the object. |
| `local_scheduled_departure_date_time` | String | Required. The scheduled date and time when the aircraft is expected to depart the gate (not the runway) Note: This field should not change too close to the departure time. For updates to departure times (delays, etc), please set `localEstimatedOrActualDepartureDateTime`. This is an ISO 8601 extended format date/time without an offset. Time may be specified up to millisecond precision. eg: `2027-03-05T06:30:00` This should be the local date/time at the airport (not a UTC time). Google will reject the request if UTC offset is provided. Time zones will be calculated by Google based on departure airport. |
| `hex_background_color` | String | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `origin` | String | Required. Origin airport. |
| `local_boarding_date_time` | String | The boarding time as it would be printed on the boarding pass. This is an ISO 8601 extended format date/time without an offset. Time may be specified up to millisecond precision. eg: `2027-03-05T06:30:00` This should be the local date/time at the airport (not a UTC time). Google will reject the request if UTC offset is provided. Time zones will be calculated by Google based on departure airport. |
| `local_gate_closing_date_time` | String | The gate closing time as it would be printed on the boarding pass. Do not set this field if you do not want to print it in the boarding pass. This is an ISO 8601 extended format date/time without an offset. Time may be specified up to millisecond precision. eg: `2027-03-05T06:30:00` This should be the local date/time at the airport (not a UTC time). Google will reject the request if UTC offset is provided. Time zones will be calculated by Google based on departure airport. |
| `callback_options` | String | Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback. |
| `homepage_uri` | String | The URI of your application's home page. Populating the URI in this field results in the exact same behavior as populating an URI in linksModuleData (when an object is rendered, a link to the homepage is shown in what would usually be thought of as the linksModuleData section of the object). |
| `flight_header` | String | Required. Information about the flight carrier and number. |
| `local_scheduled_arrival_date_time` | String | The scheduled time the aircraft plans to reach the destination gate (not the runway). Note: This field should not change too close to the flight time. For updates to departure times (delays, etc), please set `localEstimatedOrActualArrivalDateTime`. This is an ISO 8601 extended format date/time without an offset. Time may be specified up to millisecond precision. eg: `2027-03-05T06:30:00` This should be the local date/time at the airport (not a UTC time). Google will reject the request if UTC offset is provided. Time zones will be calculated by Google based on arrival airport. |
| `local_estimated_or_actual_departure_date_time` | String | The estimated time the aircraft plans to pull from the gate or the actual time the aircraft already pulled from the gate. Note: This is not the runway time. This field should be set if at least one of the below is true: - It differs from the scheduled time. Google will use it to calculate the delay. - The aircraft already pulled from the gate. Google will use it to inform the user when the flight actually departed. This is an ISO 8601 extended format date/time without an offset. Time may be specified up to millisecond precision. eg: `2027-03-05T06:30:00` This should be the local date/time at the airport (not a UTC time). Google will reject the request if UTC offset is provided. Time zones will be calculated by Google based on departure airport. |
| `merchant_locations` | Vec<String> | Merchant locations. There is a maximum of ten on the class. Any additional MerchantLocations added beyond the 10 will be rejected by the validator. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `country_code` | String | Country code used to display the card's country (when the user is not in that country), as well as to display localized content when content is not available in the user's locale. |
| `notify_preference` | String | Whether or not field updates to this class should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If not specified, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `security_animation` | String | Optional information about the security animation. If this is set a security animation will be rendered on pass details. |
| `enable_smart_tap` | bool | Identifies whether this class supports Smart Tap. The `redemptionIssuers` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `version` | String | Deprecated |


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
flightclas_info_module_data = flightclas.info_module_data
flightclas_localized_issuer_name = flightclas.localized_issuer_name
flightclas_hero_image = flightclas.hero_image
flightclas_flight_status = flightclas.flight_status
flightclas_issuer_name = flightclas.issuer_name
flightclas_allow_multiple_users_per_object = flightclas.allow_multiple_users_per_object
flightclas_review_status = flightclas.review_status
flightclas_word_mark = flightclas.word_mark
flightclas_class_template_info = flightclas.class_template_info
flightclas_destination = flightclas.destination
flightclas_review = flightclas.review
flightclas_kind = flightclas.kind
flightclas_view_unlock_requirement = flightclas.view_unlock_requirement
flightclas_boarding_and_seating_policy = flightclas.boarding_and_seating_policy
flightclas_language_override = flightclas.language_override
flightclas_links_module_data = flightclas.links_module_data
flightclas_multiple_devices_and_holders_allowed_status = flightclas.multiple_devices_and_holders_allowed_status
flightclas_locations = flightclas.locations
flightclas_app_link_data = flightclas.app_link_data
flightclas_messages = flightclas.messages
flightclas_id = flightclas.id
flightclas_image_modules_data = flightclas.image_modules_data
flightclas_local_estimated_or_actual_arrival_date_time = flightclas.local_estimated_or_actual_arrival_date_time
flightclas_redemption_issuers = flightclas.redemption_issuers
flightclas_text_modules_data = flightclas.text_modules_data
flightclas_value_added_module_data = flightclas.value_added_module_data
flightclas_local_scheduled_departure_date_time = flightclas.local_scheduled_departure_date_time
flightclas_hex_background_color = flightclas.hex_background_color
flightclas_origin = flightclas.origin
flightclas_local_boarding_date_time = flightclas.local_boarding_date_time
flightclas_local_gate_closing_date_time = flightclas.local_gate_closing_date_time
flightclas_callback_options = flightclas.callback_options
flightclas_homepage_uri = flightclas.homepage_uri
flightclas_flight_header = flightclas.flight_header
flightclas_local_scheduled_arrival_date_time = flightclas.local_scheduled_arrival_date_time
flightclas_local_estimated_or_actual_departure_date_time = flightclas.local_estimated_or_actual_departure_date_time
flightclas_merchant_locations = flightclas.merchant_locations
flightclas_country_code = flightclas.country_code
flightclas_notify_preference = flightclas.notify_preference
flightclas_security_animation = flightclas.security_animation
flightclas_enable_smart_tap = flightclas.enable_smart_tap
flightclas_version = flightclas.version
```

---


### Genericclas

Inserts a generic class with the given ID and properties.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | String |  | Required. The unique identifier for the class. This ID must be unique across all from an issuer. This value needs to follow the format `issuerID.identifier` where `issuerID` is issued by Google and `identifier` is chosen by you. The unique identifier can only include alphanumeric characters, `.`, `_`, or `-`. |
| `redemption_issuers` | Vec<String> |  | Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `enable_smart_tap` | bool |  | Available only to Smart Tap enabled partners. Contact support for additional guidance. |
| `app_link_data` | String |  | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding object that will be used instead. |
| `callback_options` | String |  | Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback. |
| `multiple_devices_and_holders_allowed_status` | String |  | Identifies whether multiple users and devices will save the same object referencing this class. |
| `links_module_data` | String |  | Links module data. If `linksModuleData` is also defined on the object, both will be displayed. The maximum number of these fields displayed is 10 from class and 10 from object. |
| `merchant_locations` | Vec<String> |  | Merchant locations. There is a maximum of ten on the class. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `value_added_module_data` | Vec<String> |  | Optional value added module data. Maximum of ten on the class. For a pass only ten will be displayed, prioritizing those from the object. |
| `class_template_info` | String |  | Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display. |
| `messages` | Vec<String> |  | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `image_modules_data` | Vec<String> |  | Image module data. If `imageModulesData` is also defined on the object, both will be displayed. Only one of the image from class and one from object level will be rendered when both set. |
| `security_animation` | String |  | Optional information about the security animation. If this is set a security animation will be rendered on pass details. |
| `text_modules_data` | Vec<String> |  | Text module data. If `textModulesData` is also defined on the object, both will be displayed. The maximum number of these fields displayed is 10 from class and 10 from object. |
| `view_unlock_requirement` | String |  | View Unlock Requirement options for the generic pass. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `id` | String | Required. The unique identifier for the class. This ID must be unique across all from an issuer. This value needs to follow the format `issuerID.identifier` where `issuerID` is issued by Google and `identifier` is chosen by you. The unique identifier can only include alphanumeric characters, `.`, `_`, or `-`. |
| `redemption_issuers` | Vec<String> | Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `enable_smart_tap` | bool | Available only to Smart Tap enabled partners. Contact support for additional guidance. |
| `app_link_data` | String | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding object that will be used instead. |
| `callback_options` | String | Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback. |
| `multiple_devices_and_holders_allowed_status` | String | Identifies whether multiple users and devices will save the same object referencing this class. |
| `links_module_data` | String | Links module data. If `linksModuleData` is also defined on the object, both will be displayed. The maximum number of these fields displayed is 10 from class and 10 from object. |
| `merchant_locations` | Vec<String> | Merchant locations. There is a maximum of ten on the class. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `value_added_module_data` | Vec<String> | Optional value added module data. Maximum of ten on the class. For a pass only ten will be displayed, prioritizing those from the object. |
| `class_template_info` | String | Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display. |
| `messages` | Vec<String> | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `image_modules_data` | Vec<String> | Image module data. If `imageModulesData` is also defined on the object, both will be displayed. Only one of the image from class and one from object level will be rendered when both set. |
| `security_animation` | String | Optional information about the security animation. If this is set a security animation will be rendered on pass details. |
| `text_modules_data` | Vec<String> | Text module data. If `textModulesData` is also defined on the object, both will be displayed. The maximum number of these fields displayed is 10 from class and 10 from object. |
| `view_unlock_requirement` | String | View Unlock Requirement options for the generic pass. |


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
genericclas_id = genericclas.id
genericclas_redemption_issuers = genericclas.redemption_issuers
genericclas_enable_smart_tap = genericclas.enable_smart_tap
genericclas_app_link_data = genericclas.app_link_data
genericclas_callback_options = genericclas.callback_options
genericclas_multiple_devices_and_holders_allowed_status = genericclas.multiple_devices_and_holders_allowed_status
genericclas_links_module_data = genericclas.links_module_data
genericclas_merchant_locations = genericclas.merchant_locations
genericclas_value_added_module_data = genericclas.value_added_module_data
genericclas_class_template_info = genericclas.class_template_info
genericclas_messages = genericclas.messages
genericclas_image_modules_data = genericclas.image_modules_data
genericclas_security_animation = genericclas.security_animation
genericclas_text_modules_data = genericclas.text_modules_data
genericclas_view_unlock_requirement = genericclas.view_unlock_requirement
```

---


### Offerobject

Inserts an offer object with the given ID and properties.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `info_module_data` | String |  | Deprecated. Use textModulesData instead. |
| `image_modules_data` | Vec<String> |  | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `linked_object_ids` | Vec<String> |  | linked_object_ids are a list of other objects such as event ticket, loyalty, offer, generic, giftcard, transit and boarding pass that should be automatically attached to this offer object. If a user had saved this offer, then these linked_object_ids would be automatically pushed to the user's wallet (unless they turned off the setting to receive such linked passes). Make sure that objects present in linked_object_ids are already inserted - if not, calls would fail. Once linked, the linked objects cannot be unlinked. You cannot link objects belonging to another issuer. There is a limit to the number of objects that can be linked to a single object. After the limit is reached, new linked objects in the call will be ignored silently. Object IDs should follow the format issuer ID.identifier where the former is issued by Google and the latter is chosen by you. |
| `barcode` | String |  | The barcode type and value. |
| `pass_constraints` | String |  | Pass constraints for the object. Includes limiting NFC and screenshot behaviors. |
| `rotating_barcode` | String |  | The rotating barcode type and value. |
| `grouping_info` | String |  | Information that controls how passes are grouped together. |
| `value_added_module_data` | Vec<String> |  | Optional value added module data. Maximum of ten on the object. |
| `version` | String |  | Deprecated |
| `class_id` | String |  | Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#offerObject"`. |
| `disable_expiration_notification` | bool |  | Indicates if notifications should explicitly be suppressed. If this field is set to true, regardless of the `messages` field, expiration notifications to the user will be suppressed. By default, this field is set to false. Currently, this can only be set for offers. |
| `id` | String |  | Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. The unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `app_link_data` | String |  | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding class only object AppLinkData will be displayed. |
| `has_users` | bool |  | Indicates if the object has users. This field is set by the platform. |
| `links_module_data` | String |  | Links module data. If links module data is also defined on the class, both will be displayed. |
| `smart_tap_redemption_value` | String |  | The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported. |
| `state` | String |  | Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section. |
| `text_modules_data` | Vec<String> |  | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `merchant_locations` | Vec<String> |  | Merchant locations. There is a maximum of ten on the object. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `notify_preference` | String |  | Whether or not field updates to this object should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If set to DO_NOT_NOTIFY or NOTIFICATION_SETTINGS_UNSPECIFIED, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `class_reference` | String |  | A copy of the inherited fields of the parent class. These fields are retrieved during a GET. |
| `locations` | Vec<String> |  | Note: This field is currently not supported to trigger geo notifications. |
| `save_restrictions` | String |  | Restrictions on the object that needs to be verified before the user tries to save the pass. Note that this restrictions will only be applied during save time. If the restrictions changed after a user saves the pass, the new restrictions will not be applied to an already saved pass. |
| `has_linked_device` | bool |  | Whether this object is currently linked to a single device. This field is set by the platform when a user saves the object, linking it to their device. Intended for use by select partners. Contact support for additional information. |
| `messages` | Vec<String> |  | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `valid_time_interval` | String |  | The time period this object will be `active` and object can be used. An object's state will be changed to `expired` when this time period has passed. |
| `hero_image` | String |  | Optional banner image displayed on the front of the card. If none is present, hero image of the class, if present, will be displayed. If hero image of the class is also not present, nothing will be displayed. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `info_module_data` | String | Deprecated. Use textModulesData instead. |
| `image_modules_data` | Vec<String> | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `linked_object_ids` | Vec<String> | linked_object_ids are a list of other objects such as event ticket, loyalty, offer, generic, giftcard, transit and boarding pass that should be automatically attached to this offer object. If a user had saved this offer, then these linked_object_ids would be automatically pushed to the user's wallet (unless they turned off the setting to receive such linked passes). Make sure that objects present in linked_object_ids are already inserted - if not, calls would fail. Once linked, the linked objects cannot be unlinked. You cannot link objects belonging to another issuer. There is a limit to the number of objects that can be linked to a single object. After the limit is reached, new linked objects in the call will be ignored silently. Object IDs should follow the format issuer ID.identifier where the former is issued by Google and the latter is chosen by you. |
| `barcode` | String | The barcode type and value. |
| `pass_constraints` | String | Pass constraints for the object. Includes limiting NFC and screenshot behaviors. |
| `rotating_barcode` | String | The rotating barcode type and value. |
| `grouping_info` | String | Information that controls how passes are grouped together. |
| `value_added_module_data` | Vec<String> | Optional value added module data. Maximum of ten on the object. |
| `version` | String | Deprecated |
| `class_id` | String | Required. The class associated with this object. The class must be of the same type as this object, must already exist, and must be approved. Class IDs should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#offerObject"`. |
| `disable_expiration_notification` | bool | Indicates if notifications should explicitly be suppressed. If this field is set to true, regardless of the `messages` field, expiration notifications to the user will be suppressed. By default, this field is set to false. Currently, this can only be set for offers. |
| `id` | String | Required. The unique identifier for an object. This ID must be unique across all objects from an issuer. This value should follow the format issuer ID.identifier where the former is issued by Google and latter is chosen by you. The unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `app_link_data` | String | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding class only object AppLinkData will be displayed. |
| `has_users` | bool | Indicates if the object has users. This field is set by the platform. |
| `links_module_data` | String | Links module data. If links module data is also defined on the class, both will be displayed. |
| `smart_tap_redemption_value` | String | The value that will be transmitted to a Smart Tap certified terminal over NFC for this object. The class level fields `enableSmartTap` and `redemptionIssuers` must also be set up correctly in order for the pass to support Smart Tap. Only ASCII characters are supported. |
| `state` | String | Required. The state of the object. This field is used to determine how an object is displayed in the app. For example, an `inactive` object is moved to the "Expired passes" section. |
| `text_modules_data` | Vec<String> | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `merchant_locations` | Vec<String> | Merchant locations. There is a maximum of ten on the object. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `notify_preference` | String | Whether or not field updates to this object should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If set to DO_NOT_NOTIFY or NOTIFICATION_SETTINGS_UNSPECIFIED, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `class_reference` | String | A copy of the inherited fields of the parent class. These fields are retrieved during a GET. |
| `locations` | Vec<String> | Note: This field is currently not supported to trigger geo notifications. |
| `save_restrictions` | String | Restrictions on the object that needs to be verified before the user tries to save the pass. Note that this restrictions will only be applied during save time. If the restrictions changed after a user saves the pass, the new restrictions will not be applied to an already saved pass. |
| `has_linked_device` | bool | Whether this object is currently linked to a single device. This field is set by the platform when a user saves the object, linking it to their device. Intended for use by select partners. Contact support for additional information. |
| `messages` | Vec<String> | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `valid_time_interval` | String | The time period this object will be `active` and object can be used. An object's state will be changed to `expired` when this time period has passed. |
| `hero_image` | String | Optional banner image displayed on the front of the card. If none is present, hero image of the class, if present, will be displayed. If hero image of the class is also not present, nothing will be displayed. |


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
offerobject_info_module_data = offerobject.info_module_data
offerobject_image_modules_data = offerobject.image_modules_data
offerobject_linked_object_ids = offerobject.linked_object_ids
offerobject_barcode = offerobject.barcode
offerobject_pass_constraints = offerobject.pass_constraints
offerobject_rotating_barcode = offerobject.rotating_barcode
offerobject_grouping_info = offerobject.grouping_info
offerobject_value_added_module_data = offerobject.value_added_module_data
offerobject_version = offerobject.version
offerobject_class_id = offerobject.class_id
offerobject_kind = offerobject.kind
offerobject_disable_expiration_notification = offerobject.disable_expiration_notification
offerobject_id = offerobject.id
offerobject_app_link_data = offerobject.app_link_data
offerobject_has_users = offerobject.has_users
offerobject_links_module_data = offerobject.links_module_data
offerobject_smart_tap_redemption_value = offerobject.smart_tap_redemption_value
offerobject_state = offerobject.state
offerobject_text_modules_data = offerobject.text_modules_data
offerobject_merchant_locations = offerobject.merchant_locations
offerobject_notify_preference = offerobject.notify_preference
offerobject_class_reference = offerobject.class_reference
offerobject_locations = offerobject.locations
offerobject_save_restrictions = offerobject.save_restrictions
offerobject_has_linked_device = offerobject.has_linked_device
offerobject_messages = offerobject.messages
offerobject_valid_time_interval = offerobject.valid_time_interval
offerobject_hero_image = offerobject.hero_image
```

---


### Giftcardclas

Inserts an gift card class with the given ID and properties.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#giftCardClass"`. |
| `localized_event_number_label` | String |  | Translated strings for the event_number_label. |
| `merchant_locations` | Vec<String> |  | Merchant locations. There is a maximum of ten on the class. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `hero_image` | String |  | Optional banner image displayed on the front of the card. If none is present, nothing will be displayed. The image will display at 100% width. |
| `allow_barcode_redemption` | bool |  | Determines whether the merchant supports gift card redemption using barcode. If true, app displays a barcode for the gift card on the Gift card details screen. If false, a barcode is not displayed. |
| `locations` | Vec<String> |  | Note: This field is currently not supported to trigger geo notifications. |
| `country_code` | String |  | Country code used to display the card's country (when the user is not in that country), as well as to display localized content when content is not available in the user's locale. |
| `multiple_devices_and_holders_allowed_status` | String |  | Identifies whether multiple users and devices will save the same object referencing this class. |
| `review_status` | String |  | Required. The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`. |
| `links_module_data` | String |  | Links module data. If links module data is also defined on the object, both will be displayed. |
| `view_unlock_requirement` | String |  | View Unlock Requirement options for the gift card. |
| `merchant_name` | String |  | Merchant name, such as "Adam's Apparel". The app may display an ellipsis after the first 20 characters to ensure full string is displayed on smaller screens. |
| `redemption_issuers` | Vec<String> |  | Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `class_template_info` | String |  | Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display. |
| `hex_background_color` | String |  | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `value_added_module_data` | Vec<String> |  | Optional value added module data. Maximum of ten on the class. For a pass only ten will be displayed, prioritizing those from the object. |
| `word_mark` | String |  | Deprecated. |
| `wide_program_logo` | String |  | The wide logo of the gift card program or company. When provided, this will be used in place of the program logo in the top left of the card view. |
| `pin_label` | String |  | The label to display for the PIN, such as "4-digit PIN". |
| `localized_card_number_label` | String |  | Translated strings for the card_number_label. |
| `review` | String |  | The review comments set by the platform when a class is marked `approved` or `rejected`. |
| `text_modules_data` | Vec<String> |  | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `version` | String |  | Deprecated |
| `allow_multiple_users_per_object` | bool |  | Deprecated. Use `multipleDevicesAndHoldersAllowedStatus` instead. |
| `localized_merchant_name` | String |  | Translated strings for the merchant_name. The app may display an ellipsis after the first 20 characters to ensure full string is displayed on smaller screens. |
| `id` | String |  | Required. The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `app_link_data` | String |  | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding object that will be used instead. |
| `security_animation` | String |  | Optional information about the security animation. If this is set a security animation will be rendered on pass details. |
| `event_number_label` | String |  | The label to display for event number, such as "Target Event #". |
| `program_logo` | String |  | The logo of the gift card program or company. This logo is displayed in both the details and list views of the app. |
| `issuer_name` | String |  | Required. The issuer name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `messages` | Vec<String> |  | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `info_module_data` | String |  | Deprecated. Use textModulesData instead. |
| `homepage_uri` | String |  | The URI of your application's home page. Populating the URI in this field results in the exact same behavior as populating an URI in linksModuleData (when an object is rendered, a link to the homepage is shown in what would usually be thought of as the linksModuleData section of the object). |
| `image_modules_data` | Vec<String> |  | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `callback_options` | String |  | Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback. |
| `localized_issuer_name` | String |  | Translated strings for the issuer_name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `enable_smart_tap` | bool |  | Identifies whether this class supports Smart Tap. The `redemptionIssuers` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `card_number_label` | String |  | The label to display for the card number, such as "Card Number". |
| `localized_pin_label` | String |  | Translated strings for the pin_label. |
| `notify_preference` | String |  | Whether or not field updates to this class should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If not specified, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#giftCardClass"`. |
| `localized_event_number_label` | String | Translated strings for the event_number_label. |
| `merchant_locations` | Vec<String> | Merchant locations. There is a maximum of ten on the class. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `hero_image` | String | Optional banner image displayed on the front of the card. If none is present, nothing will be displayed. The image will display at 100% width. |
| `allow_barcode_redemption` | bool | Determines whether the merchant supports gift card redemption using barcode. If true, app displays a barcode for the gift card on the Gift card details screen. If false, a barcode is not displayed. |
| `locations` | Vec<String> | Note: This field is currently not supported to trigger geo notifications. |
| `country_code` | String | Country code used to display the card's country (when the user is not in that country), as well as to display localized content when content is not available in the user's locale. |
| `multiple_devices_and_holders_allowed_status` | String | Identifies whether multiple users and devices will save the same object referencing this class. |
| `review_status` | String | Required. The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`. |
| `links_module_data` | String | Links module data. If links module data is also defined on the object, both will be displayed. |
| `view_unlock_requirement` | String | View Unlock Requirement options for the gift card. |
| `merchant_name` | String | Merchant name, such as "Adam's Apparel". The app may display an ellipsis after the first 20 characters to ensure full string is displayed on smaller screens. |
| `redemption_issuers` | Vec<String> | Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `class_template_info` | String | Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display. |
| `hex_background_color` | String | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `value_added_module_data` | Vec<String> | Optional value added module data. Maximum of ten on the class. For a pass only ten will be displayed, prioritizing those from the object. |
| `word_mark` | String | Deprecated. |
| `wide_program_logo` | String | The wide logo of the gift card program or company. When provided, this will be used in place of the program logo in the top left of the card view. |
| `pin_label` | String | The label to display for the PIN, such as "4-digit PIN". |
| `localized_card_number_label` | String | Translated strings for the card_number_label. |
| `review` | String | The review comments set by the platform when a class is marked `approved` or `rejected`. |
| `text_modules_data` | Vec<String> | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `version` | String | Deprecated |
| `allow_multiple_users_per_object` | bool | Deprecated. Use `multipleDevicesAndHoldersAllowedStatus` instead. |
| `localized_merchant_name` | String | Translated strings for the merchant_name. The app may display an ellipsis after the first 20 characters to ensure full string is displayed on smaller screens. |
| `id` | String | Required. The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `app_link_data` | String | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding object that will be used instead. |
| `security_animation` | String | Optional information about the security animation. If this is set a security animation will be rendered on pass details. |
| `event_number_label` | String | The label to display for event number, such as "Target Event #". |
| `program_logo` | String | The logo of the gift card program or company. This logo is displayed in both the details and list views of the app. |
| `issuer_name` | String | Required. The issuer name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `messages` | Vec<String> | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `info_module_data` | String | Deprecated. Use textModulesData instead. |
| `homepage_uri` | String | The URI of your application's home page. Populating the URI in this field results in the exact same behavior as populating an URI in linksModuleData (when an object is rendered, a link to the homepage is shown in what would usually be thought of as the linksModuleData section of the object). |
| `image_modules_data` | Vec<String> | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `callback_options` | String | Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback. |
| `localized_issuer_name` | String | Translated strings for the issuer_name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `enable_smart_tap` | bool | Identifies whether this class supports Smart Tap. The `redemptionIssuers` and object level `smartTapRedemptionLevel` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `card_number_label` | String | The label to display for the card number, such as "Card Number". |
| `localized_pin_label` | String | Translated strings for the pin_label. |
| `notify_preference` | String | Whether or not field updates to this class should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If not specified, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |


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
giftcardclas_kind = giftcardclas.kind
giftcardclas_localized_event_number_label = giftcardclas.localized_event_number_label
giftcardclas_merchant_locations = giftcardclas.merchant_locations
giftcardclas_hero_image = giftcardclas.hero_image
giftcardclas_allow_barcode_redemption = giftcardclas.allow_barcode_redemption
giftcardclas_locations = giftcardclas.locations
giftcardclas_country_code = giftcardclas.country_code
giftcardclas_multiple_devices_and_holders_allowed_status = giftcardclas.multiple_devices_and_holders_allowed_status
giftcardclas_review_status = giftcardclas.review_status
giftcardclas_links_module_data = giftcardclas.links_module_data
giftcardclas_view_unlock_requirement = giftcardclas.view_unlock_requirement
giftcardclas_merchant_name = giftcardclas.merchant_name
giftcardclas_redemption_issuers = giftcardclas.redemption_issuers
giftcardclas_class_template_info = giftcardclas.class_template_info
giftcardclas_hex_background_color = giftcardclas.hex_background_color
giftcardclas_value_added_module_data = giftcardclas.value_added_module_data
giftcardclas_word_mark = giftcardclas.word_mark
giftcardclas_wide_program_logo = giftcardclas.wide_program_logo
giftcardclas_pin_label = giftcardclas.pin_label
giftcardclas_localized_card_number_label = giftcardclas.localized_card_number_label
giftcardclas_review = giftcardclas.review
giftcardclas_text_modules_data = giftcardclas.text_modules_data
giftcardclas_version = giftcardclas.version
giftcardclas_allow_multiple_users_per_object = giftcardclas.allow_multiple_users_per_object
giftcardclas_localized_merchant_name = giftcardclas.localized_merchant_name
giftcardclas_id = giftcardclas.id
giftcardclas_app_link_data = giftcardclas.app_link_data
giftcardclas_security_animation = giftcardclas.security_animation
giftcardclas_event_number_label = giftcardclas.event_number_label
giftcardclas_program_logo = giftcardclas.program_logo
giftcardclas_issuer_name = giftcardclas.issuer_name
giftcardclas_messages = giftcardclas.messages
giftcardclas_info_module_data = giftcardclas.info_module_data
giftcardclas_homepage_uri = giftcardclas.homepage_uri
giftcardclas_image_modules_data = giftcardclas.image_modules_data
giftcardclas_callback_options = giftcardclas.callback_options
giftcardclas_localized_issuer_name = giftcardclas.localized_issuer_name
giftcardclas_enable_smart_tap = giftcardclas.enable_smart_tap
giftcardclas_card_number_label = giftcardclas.card_number_label
giftcardclas_localized_pin_label = giftcardclas.localized_pin_label
giftcardclas_notify_preference = giftcardclas.notify_preference
```

---


### Private_content

Provide Google with information about awaiting private pass update. This will allow Google to provide the update notification to the device that currently holds this pass.

**Operations**: ✅ Create

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `update_uri` | String |  | Required. The issuer endpoint URI the pass holder needs to follow in order to receive an updated pass JWT. It can not contain any sensitive information. The endpoint needs to authenticate the user before giving the user the updated JWT. Example update URI https://someissuer.com/update/passId=someExternalPassId |
| `updated_pass_jwt_signature` | String |  | Required. The JWT signature of the updated pass that the issuer wants to notify Google about. Only devices that report a different JWT signature than this JWT signature will receive the update notification. |
| `external_pass_id` | String |  | Required. A fully qualified identifier of the pass that the issuer wants to notify the pass holder(s) about. Formatted as . |



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


### Loyaltyclas

Inserts an loyalty class with the given ID and properties.

**Operations**: ✅ Create ✅ Read ✅ Update

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `rewards_tier` | String |  | The rewards tier, such as "Gold" or "Platinum." Recommended maximum length is 7 characters to ensure full string is displayed on smaller screens. |
| `allow_multiple_users_per_object` | bool |  | Deprecated. Use `multipleDevicesAndHoldersAllowedStatus` instead. |
| `id` | String |  | Required. The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `version` | String |  | Deprecated |
| `review_status` | String |  | Required. The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`. |
| `wide_program_logo` | String |  | The wide logo of the loyalty program or company. When provided, this will be used in place of the program logo in the top left of the card view. |
| `class_template_info` | String |  | Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display. |
| `localized_secondary_rewards_tier_label` | String |  | Translated strings for the secondary_rewards_tier_label. |
| `redemption_issuers` | Vec<String> |  | Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and one of object level `smartTapRedemptionValue`, barcode.value`, or `accountId` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `multiple_devices_and_holders_allowed_status` | String |  | Identifies whether multiple users and devices will save the same object referencing this class. |
| `discoverable_program` | String |  | Information about how the class may be discovered and instantiated from within the Google Pay app. |
| `security_animation` | String |  | Optional information about the security animation. If this is set a security animation will be rendered on pass details. |
| `value_added_module_data` | Vec<String> |  | Optional value added module data. Maximum of ten on the class. For a pass only ten will be displayed, prioritizing those from the object. |
| `localized_rewards_tier` | String |  | Translated strings for the rewards_tier. Recommended maximum length is 7 characters to ensure full string is displayed on smaller screens. |
| `locations` | Vec<String> |  | Note: This field is currently not supported to trigger geo notifications. |
| `localized_program_name` | String |  | Translated strings for the program_name. The app may display an ellipsis after the first 20 characters to ensure full string is displayed on smaller screens. |
| `homepage_uri` | String |  | The URI of your application's home page. Populating the URI in this field results in the exact same behavior as populating an URI in linksModuleData (when an object is rendered, a link to the homepage is shown in what would usually be thought of as the linksModuleData section of the object). |
| `account_id_label` | String |  | The account ID label, such as "Member ID." Recommended maximum length is 15 characters to ensure full string is displayed on smaller screens. |
| `rewards_tier_label` | String |  | The rewards tier label, such as "Rewards Tier." Recommended maximum length is 9 characters to ensure full string is displayed on smaller screens. |
| `links_module_data` | String |  | Links module data. If links module data is also defined on the object, both will be displayed. |
| `hex_background_color` | String |  | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `localized_secondary_rewards_tier` | String |  | Translated strings for the secondary_rewards_tier. |
| `program_name` | String |  | Required. The program name, such as "Adam's Apparel". The app may display an ellipsis after the first 20 characters to ensure full string is displayed on smaller screens. |
| `secondary_rewards_tier` | String |  | The secondary rewards tier, such as "Gold" or "Platinum." |
| `view_unlock_requirement` | String |  | View Unlock Requirement options for the loyalty card. |
| `enable_smart_tap` | bool |  | Identifies whether this class supports Smart Tap. The `redemptionIssuers` and one of object level `smartTapRedemptionLevel`, barcode.value`, or `accountId` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `issuer_name` | String |  | Required. The issuer name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `word_mark` | String |  | Deprecated. |
| `localized_account_id_label` | String |  | Translated strings for the account_id_label. Recommended maximum length is 15 characters to ensure full string is displayed on smaller screens. |
| `notify_preference` | String |  | Whether or not field updates to this class should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If not specified, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `image_modules_data` | Vec<String> |  | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `localized_rewards_tier_label` | String |  | Translated strings for the rewards_tier_label. Recommended maximum length is 9 characters to ensure full string is displayed on smaller screens. |
| `app_link_data` | String |  | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding object that will be used instead. |
| `info_module_data` | String |  | Deprecated. Use textModulesData instead. |
| `localized_account_name_label` | String |  | Translated strings for the account_name_label. Recommended maximum length is 15 characters to ensure full string is displayed on smaller screens. |
| `program_logo` | String |  | Required. The logo of the loyalty program or company. This logo is displayed in both the details and list views of the app. |
| `text_modules_data` | Vec<String> |  | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `account_name_label` | String |  | The account name label, such as "Member Name." Recommended maximum length is 15 characters to ensure full string is displayed on smaller screens. |
| `secondary_rewards_tier_label` | String |  | The secondary rewards tier label, such as "Rewards Tier." |
| `country_code` | String |  | Country code used to display the card's country (when the user is not in that country), as well as to display localized content when content is not available in the user's locale. |
| `callback_options` | String |  | Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback. |
| `hero_image` | String |  | Optional banner image displayed on the front of the card. If none is present, nothing will be displayed. The image will display at 100% width. |
| `merchant_locations` | Vec<String> |  | Merchant locations. There is a maximum of ten on the class. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `messages` | Vec<String> |  | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `kind` | String |  | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#loyaltyClass"`. |
| `localized_issuer_name` | String |  | Translated strings for the issuer_name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `review` | String |  | The review comments set by the platform when a class is marked `approved` or `rejected`. |


#### Outputs

| Output | Type | Description |
|--------|------|-------------|
| `rewards_tier` | String | The rewards tier, such as "Gold" or "Platinum." Recommended maximum length is 7 characters to ensure full string is displayed on smaller screens. |
| `allow_multiple_users_per_object` | bool | Deprecated. Use `multipleDevicesAndHoldersAllowedStatus` instead. |
| `id` | String | Required. The unique identifier for a class. This ID must be unique across all classes from an issuer. This value should follow the format issuer ID. identifier where the former is issued by Google and latter is chosen by you. Your unique identifier should only include alphanumeric characters, '.', '_', or '-'. |
| `version` | String | Deprecated |
| `review_status` | String | Required. The status of the class. This field can be set to `draft` or `underReview` using the insert, patch, or update API calls. Once the review state is changed from `draft` it may not be changed back to `draft`. You should keep this field to `draft` when the class is under development. A `draft` class cannot be used to create any object. You should set this field to `underReview` when you believe the class is ready for use. The platform will automatically set this field to `approved` and it can be immediately used to create or migrate objects. When updating an already `approved` class you should keep setting this field to `underReview`. |
| `wide_program_logo` | String | The wide logo of the loyalty program or company. When provided, this will be used in place of the program logo in the top left of the card view. |
| `class_template_info` | String | Template information about how the class should be displayed. If unset, Google will fallback to a default set of fields to display. |
| `localized_secondary_rewards_tier_label` | String | Translated strings for the secondary_rewards_tier_label. |
| `redemption_issuers` | Vec<String> | Identifies which redemption issuers can redeem the pass over Smart Tap. Redemption issuers are identified by their issuer ID. Redemption issuers must have at least one Smart Tap key configured. The `enableSmartTap` and one of object level `smartTapRedemptionValue`, barcode.value`, or `accountId` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `multiple_devices_and_holders_allowed_status` | String | Identifies whether multiple users and devices will save the same object referencing this class. |
| `discoverable_program` | String | Information about how the class may be discovered and instantiated from within the Google Pay app. |
| `security_animation` | String | Optional information about the security animation. If this is set a security animation will be rendered on pass details. |
| `value_added_module_data` | Vec<String> | Optional value added module data. Maximum of ten on the class. For a pass only ten will be displayed, prioritizing those from the object. |
| `localized_rewards_tier` | String | Translated strings for the rewards_tier. Recommended maximum length is 7 characters to ensure full string is displayed on smaller screens. |
| `locations` | Vec<String> | Note: This field is currently not supported to trigger geo notifications. |
| `localized_program_name` | String | Translated strings for the program_name. The app may display an ellipsis after the first 20 characters to ensure full string is displayed on smaller screens. |
| `homepage_uri` | String | The URI of your application's home page. Populating the URI in this field results in the exact same behavior as populating an URI in linksModuleData (when an object is rendered, a link to the homepage is shown in what would usually be thought of as the linksModuleData section of the object). |
| `account_id_label` | String | The account ID label, such as "Member ID." Recommended maximum length is 15 characters to ensure full string is displayed on smaller screens. |
| `rewards_tier_label` | String | The rewards tier label, such as "Rewards Tier." Recommended maximum length is 9 characters to ensure full string is displayed on smaller screens. |
| `links_module_data` | String | Links module data. If links module data is also defined on the object, both will be displayed. |
| `hex_background_color` | String | The background color for the card. If not set the dominant color of the hero image is used, and if no hero image is set, the dominant color of the logo is used. The format is #rrggbb where rrggbb is a hex RGB triplet, such as `#ffcc00`. You can also use the shorthand version of the RGB triplet which is #rgb, such as `#fc0`. |
| `localized_secondary_rewards_tier` | String | Translated strings for the secondary_rewards_tier. |
| `program_name` | String | Required. The program name, such as "Adam's Apparel". The app may display an ellipsis after the first 20 characters to ensure full string is displayed on smaller screens. |
| `secondary_rewards_tier` | String | The secondary rewards tier, such as "Gold" or "Platinum." |
| `view_unlock_requirement` | String | View Unlock Requirement options for the loyalty card. |
| `enable_smart_tap` | bool | Identifies whether this class supports Smart Tap. The `redemptionIssuers` and one of object level `smartTapRedemptionLevel`, barcode.value`, or `accountId` fields must also be set up correctly in order for a pass to support Smart Tap. |
| `issuer_name` | String | Required. The issuer name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `word_mark` | String | Deprecated. |
| `localized_account_id_label` | String | Translated strings for the account_id_label. Recommended maximum length is 15 characters to ensure full string is displayed on smaller screens. |
| `notify_preference` | String | Whether or not field updates to this class should trigger notifications. When set to NOTIFY, we will attempt to trigger a field update notification to users. These notifications will only be sent to users if the field is part of an allowlist. If not specified, no notification will be triggered. This setting is ephemeral and needs to be set with each PATCH or UPDATE request, otherwise a notification will not be triggered. |
| `image_modules_data` | Vec<String> | Image module data. The maximum number of these fields displayed is 1 from object level and 1 for class object level. |
| `localized_rewards_tier_label` | String | Translated strings for the rewards_tier_label. Recommended maximum length is 9 characters to ensure full string is displayed on smaller screens. |
| `app_link_data` | String | Optional app or website link that will be displayed as a button on the front of the pass. If AppLinkData is provided for the corresponding object that will be used instead. |
| `info_module_data` | String | Deprecated. Use textModulesData instead. |
| `localized_account_name_label` | String | Translated strings for the account_name_label. Recommended maximum length is 15 characters to ensure full string is displayed on smaller screens. |
| `program_logo` | String | Required. The logo of the loyalty program or company. This logo is displayed in both the details and list views of the app. |
| `text_modules_data` | Vec<String> | Text module data. If text module data is also defined on the class, both will be displayed. The maximum number of these fields displayed is 10 from the object and 10 from the class. |
| `account_name_label` | String | The account name label, such as "Member Name." Recommended maximum length is 15 characters to ensure full string is displayed on smaller screens. |
| `secondary_rewards_tier_label` | String | The secondary rewards tier label, such as "Rewards Tier." |
| `country_code` | String | Country code used to display the card's country (when the user is not in that country), as well as to display localized content when content is not available in the user's locale. |
| `callback_options` | String | Callback options to be used to call the issuer back for every save/delete of an object for this class by the end-user. All objects of this class are eligible for the callback. |
| `hero_image` | String | Optional banner image displayed on the front of the card. If none is present, nothing will be displayed. The image will display at 100% width. |
| `merchant_locations` | Vec<String> | Merchant locations. There is a maximum of ten on the class. Any additional MerchantLocations added beyond the 10 will be rejected. These locations will trigger a notification when a user enters within a Google-set radius of the point. This field replaces the deprecated LatLongPoints. |
| `messages` | Vec<String> | An array of messages displayed in the app. All users of this object will receive its associated messages. The maximum number of these fields is 10. |
| `kind` | String | Identifies what kind of resource this is. Value: the fixed string `"walletobjects#loyaltyClass"`. |
| `localized_issuer_name` | String | Translated strings for the issuer_name. Recommended maximum length is 20 characters to ensure full string is displayed on smaller screens. |
| `review` | String | The review comments set by the platform when a class is marked `approved` or `rejected`. |


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
loyaltyclas_rewards_tier = loyaltyclas.rewards_tier
loyaltyclas_allow_multiple_users_per_object = loyaltyclas.allow_multiple_users_per_object
loyaltyclas_id = loyaltyclas.id
loyaltyclas_version = loyaltyclas.version
loyaltyclas_review_status = loyaltyclas.review_status
loyaltyclas_wide_program_logo = loyaltyclas.wide_program_logo
loyaltyclas_class_template_info = loyaltyclas.class_template_info
loyaltyclas_localized_secondary_rewards_tier_label = loyaltyclas.localized_secondary_rewards_tier_label
loyaltyclas_redemption_issuers = loyaltyclas.redemption_issuers
loyaltyclas_multiple_devices_and_holders_allowed_status = loyaltyclas.multiple_devices_and_holders_allowed_status
loyaltyclas_discoverable_program = loyaltyclas.discoverable_program
loyaltyclas_security_animation = loyaltyclas.security_animation
loyaltyclas_value_added_module_data = loyaltyclas.value_added_module_data
loyaltyclas_localized_rewards_tier = loyaltyclas.localized_rewards_tier
loyaltyclas_locations = loyaltyclas.locations
loyaltyclas_localized_program_name = loyaltyclas.localized_program_name
loyaltyclas_homepage_uri = loyaltyclas.homepage_uri
loyaltyclas_account_id_label = loyaltyclas.account_id_label
loyaltyclas_rewards_tier_label = loyaltyclas.rewards_tier_label
loyaltyclas_links_module_data = loyaltyclas.links_module_data
loyaltyclas_hex_background_color = loyaltyclas.hex_background_color
loyaltyclas_localized_secondary_rewards_tier = loyaltyclas.localized_secondary_rewards_tier
loyaltyclas_program_name = loyaltyclas.program_name
loyaltyclas_secondary_rewards_tier = loyaltyclas.secondary_rewards_tier
loyaltyclas_view_unlock_requirement = loyaltyclas.view_unlock_requirement
loyaltyclas_enable_smart_tap = loyaltyclas.enable_smart_tap
loyaltyclas_issuer_name = loyaltyclas.issuer_name
loyaltyclas_word_mark = loyaltyclas.word_mark
loyaltyclas_localized_account_id_label = loyaltyclas.localized_account_id_label
loyaltyclas_notify_preference = loyaltyclas.notify_preference
loyaltyclas_image_modules_data = loyaltyclas.image_modules_data
loyaltyclas_localized_rewards_tier_label = loyaltyclas.localized_rewards_tier_label
loyaltyclas_app_link_data = loyaltyclas.app_link_data
loyaltyclas_info_module_data = loyaltyclas.info_module_data
loyaltyclas_localized_account_name_label = loyaltyclas.localized_account_name_label
loyaltyclas_program_logo = loyaltyclas.program_logo
loyaltyclas_text_modules_data = loyaltyclas.text_modules_data
loyaltyclas_account_name_label = loyaltyclas.account_name_label
loyaltyclas_secondary_rewards_tier_label = loyaltyclas.secondary_rewards_tier_label
loyaltyclas_country_code = loyaltyclas.country_code
loyaltyclas_callback_options = loyaltyclas.callback_options
loyaltyclas_hero_image = loyaltyclas.hero_image
loyaltyclas_merchant_locations = loyaltyclas.merchant_locations
loyaltyclas_messages = loyaltyclas.messages
loyaltyclas_kind = loyaltyclas.kind
loyaltyclas_localized_issuer_name = loyaltyclas.localized_issuer_name
loyaltyclas_review = loyaltyclas.review
```

---



## Common Operations

### Creating Multiple Resources

```kcl
import gcp

provider = gcp.GcpProvider {
    project = "my-project-id"
}

# Create multiple loyaltyobject resources
loyaltyobject_0 = provider.walletobjects_api.Loyaltyobject {
}
loyaltyobject_1 = provider.walletobjects_api.Loyaltyobject {
}
loyaltyobject_2 = provider.walletobjects_api.Loyaltyobject {
}
```

### Conditional Creation

```kcl
# Only create in production
if environment == "production":
    loyaltyobject = provider.walletobjects_api.Loyaltyobject {
    }
```

---

## Related Documentation

- [GCP Walletobjects_api Documentation](https://cloud.google.com/walletobjects_api/docs)
- [Getting Started Guide](../getting-started.md)
- [Installation Guide](../installation.md)
- ⬅️ [Back to README](../../README.md)
