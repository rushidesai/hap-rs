use erased_serde::serialize_trait_object;
use futures::executor;

use crate::{
    characteristic::{
        accessory_flags::AccessoryFlagsCharacteristic,
        hardware_revision::HardwareRevisionCharacteristic,
        HapCharacteristic,
    },
    pointer,
    service::{accessory_information::AccessoryInformationService, HapService},
    HapType,
    Result,
};

mod category;
mod defined;
mod generated;

pub use crate::accessory::{category::AccessoryCategory, defined::*, generated::*};

/// `HapAccessory` is implemented by the inner type of every `Accessory`.
pub trait HapAccessory: HapAccessorySetup + erased_serde::Serialize + Send + Sync {
    /// Returns the ID of the Accessory.
    fn get_id(&self) -> u64;
    /// Sets the ID of the Accessory.
    fn set_id(&mut self, id: u64);
    /// Returns a reference to a specific Service of the Accessory if it's present on it.
    fn get_service(&self, hap_type: HapType) -> Option<&dyn HapService>;
    /// Returns a mutable reference to a specific Service of the Accessory if it's present on it.
    fn get_mut_service(&mut self, hap_type: HapType) -> Option<&mut dyn HapService>;
    /// Returns references to all Services of the Accessory.
    fn get_services(&self) -> Vec<&dyn HapService>;
    /// Returns mutable references to the Services of the Accessory.
    fn get_mut_services(&mut self) -> Vec<&mut dyn HapService>;
}

serialize_trait_object!(HapAccessory);

pub trait HapAccessorySetup {
    /// Sets a `hap::event::pointer::EventEmitter` on all Characteristics of the Accessory.
    fn set_event_emitter_on_characteristics(&mut self, event_emitter: Option<pointer::EventEmitter>);
}

impl<H> HapAccessorySetup for H
where
    H: HapAccessory,
{
    fn set_event_emitter_on_characteristics(&mut self, event_emitter: Option<pointer::EventEmitter>) {
        for service in self.get_mut_services() {
            for characteristic in service.get_mut_characteristics() {
                characteristic.set_event_emitter(event_emitter.clone());
            }
        }
    }
}

/// The `AccessoryInformationInformation` struct is used to store metadata about an `Accessory` and is converted to the
/// Accessory Information Service of the `Accessory` it is passed to on its creation.
///
/// # Examples
///
/// ```
/// use hap::accessory::{outlet::OutletAccessory, AccessoryInformation};
///
/// let information = AccessoryInformation {
///     manufacturer: "Acme".into(),
///     model: "A1234".into(),
///     name: "Acme Outlet".into(),
///     serial_number: "1A2B3C4D5E6F".into(),
///     ..Default::default()
/// };
///
/// let outlet = OutletAccessory::new(1, information).unwrap();
/// ```
#[derive(Debug)]
pub struct AccessoryInformation {
    /// Contains the name of the company whose brand will appear on the `Accessory`, e.g., "Acme".
    pub manufacturer: String,
    /// Contains the manufacturer-specific model of the `Accessory`, e.g. "A1234".
    pub model: String,
    /// Describes the name of the `Accessory`.
    pub name: String,
    /// Contains the manufacturer-specific serial number of the `Accessory`, e.g. "1A2B3C4D5E6F".
    /// The length must be greater than 1.
    pub serial_number: String,
    /// Describes a firmware revision string x[.y[.z]] (e.g. "100.1.1"):
    /// - <x> is the major version number, required.
    /// - <y> is the minor version number, required if it is non-zero or if <z> is present.
    /// - <z> is the revision version number, required if non-zero.
    ///
    /// The firmware revision must follow the below rules:
    /// - <x> is incremented when there is significant change. e.g.,1.0.0, 2.0.0, 3.0.0, etc.
    /// - <y> is incremented when minor changes are introduced such as 1.1.0, 2.1.0, 3.1.0 etc.
    /// - <z> is incremented when bug-fixes are introduced such as 1.0.1, 2.0.1, 3.0.1 etc.
    /// - Subsequent firmware updates can have a lower <y> version only if <x> is incremented
    /// - Subsequent firmware updates can have a lower <z> version only if <x> or <y> is incremented
    ///
    /// The value must change after every firmware update.
    pub firmware_revision: String,
    /// Describes a hardware revision string x[.y[.z]] (e.g. "100.1.1") and tracked when the board
    /// or components of the same accessory is changed:
    /// - <x> is the major version number, required.
    /// - <y> is the minor version number, required if it is non-zero or if <z> is present.
    /// - <z> is the revision version number, required if non-zero.
    ///
    /// The hardware revision must follow the below rules:
    /// - <x> is incremented when there is significant change. e.g.,1.0.0, 2.0.0, 3.0.0, etc.
    /// - <y> is incremented when minor changes are introduced such as 1.1.0, 2.1.0, 3.1.0 etc.
    /// - <z> is incremented when bug-fixes are introduced such as 1.0.1, 2.0.1, 3.0.1 etc.
    /// - Subsequent firmware updates can have a lower <y> version only if <x> is incremented
    /// - Subsequent firmware updates can have a lower <z> version only if <x> or <y> is incremented
    ///
    /// The value must change after every hardware update.
    pub hardware_revision: Option<String>,
    /// When set indicates accessory requires additional setup. Use of Accessory Flags requires
    /// written approval by Apple in advance.
    pub accessory_flags: Option<u32>,
}

impl AccessoryInformation {
    /// Converts the `Information` struct to an Accessory Information Service.
    pub(crate) fn to_service(self, id: u64, accessory_id: u64) -> Result<AccessoryInformationService> {
        let mut i = AccessoryInformationService::new(id, accessory_id);
        executor::block_on(i.identify.set_value(serde_json::Value::Bool(false)))?;
        executor::block_on(i.manufacturer.set_value(serde_json::Value::String(self.manufacturer)))?;
        executor::block_on(i.model.set_value(serde_json::Value::String(self.model)))?;
        executor::block_on(i.name.set_value(serde_json::Value::String(self.name)))?;
        executor::block_on(i.serial_number.set_value(serde_json::Value::String(self.serial_number)))?;
        executor::block_on(
            i.firmware_revision
                .set_value(serde_json::Value::String(self.firmware_revision)),
        )?;
        if let Some(v) = self.hardware_revision {
            let mut hr = HardwareRevisionCharacteristic::new(7, accessory_id);
            executor::block_on(hr.set_value(serde_json::Value::String(v)))?;
            i.hardware_revision = Some(hr);
        }
        if let Some(v) = self.accessory_flags {
            let mut af = AccessoryFlagsCharacteristic::new(8, accessory_id);
            executor::block_on(af.set_value(serde_json::Value::Number(v.into())))?;
            i.accessory_flags = Some(af);
        }
        Ok(i)
    }
}

impl Default for AccessoryInformation {
    fn default() -> Self {
        Self {
            manufacturer: "undefined".into(),
            model: "undefined".into(),
            name: "undefined".into(),
            serial_number: "undefined".into(),
            firmware_revision: "undefined".into(),
            hardware_revision: None,
            accessory_flags: None,
        }
    }
}
