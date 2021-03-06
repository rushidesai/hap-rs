// this file is auto-generated by hap-codegen

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
    service::HapService,
    characteristic::{
        HapCharacteristic,
		current_position::CurrentPositionCharacteristic,
		target_position::TargetPositionCharacteristic,
		position_state::PositionStateCharacteristic,
		hold_position::HoldPositionCharacteristic,
		target_horizontal_tilt_angle::TargetHorizontalTiltAngleCharacteristic,
		target_vertical_tilt_angle::TargetVerticalTiltAngleCharacteristic,
		current_horizontal_tilt_angle::CurrentHorizontalTiltAngleCharacteristic,
		current_vertical_tilt_angle::CurrentVerticalTiltAngleCharacteristic,
		obstruction_detected::ObstructionDetectedCharacteristic,
		name::NameCharacteristic,
	},
    HapType,
};

/// Window Covering Service.
#[derive(Debug, Default)]
pub struct WindowCoveringService {
    /// ID of the Window Covering Service.
    id: u64,
    /// `HapType` of the Window Covering Service.
    hap_type: HapType,
    /// Specifies if the Service is hidden.
    hidden: bool,
    /// Specifies if the Service is the primary Service of the Accessory.
    primary: bool,

	/// Current Position Characteristic (required).
	pub current_position: CurrentPositionCharacteristic,
	/// Target Position Characteristic (required).
	pub target_position: TargetPositionCharacteristic,
	/// Position State Characteristic (required).
	pub position_state: PositionStateCharacteristic,

	/// Hold Position Characteristic (optional).
	pub hold_position: Option<HoldPositionCharacteristic>,
	/// Target Horizontal Tilt Angle Characteristic (optional).
	pub target_horizontal_tilt_angle: Option<TargetHorizontalTiltAngleCharacteristic>,
	/// Target Vertical Tilt Angle Characteristic (optional).
	pub target_vertical_tilt_angle: Option<TargetVerticalTiltAngleCharacteristic>,
	/// Current Horizontal Tilt Angle Characteristic (optional).
	pub current_horizontal_tilt_angle: Option<CurrentHorizontalTiltAngleCharacteristic>,
	/// Current Vertical Tilt Angle Characteristic (optional).
	pub current_vertical_tilt_angle: Option<CurrentVerticalTiltAngleCharacteristic>,
	/// Obstruction Detected Characteristic (optional).
	pub obstruction_detected: Option<ObstructionDetectedCharacteristic>,
	/// Name Characteristic (optional).
	pub name: Option<NameCharacteristic>,
}

impl WindowCoveringService {
    /// Creates a new Window Covering Service.
    pub fn new(id: u64, accessory_id: u64) -> Self {
        Self {
            id,
            hap_type: HapType::WindowCovering,
			current_position: CurrentPositionCharacteristic::new(id + 1 + 0, accessory_id),
			target_position: TargetPositionCharacteristic::new(id + 1 + 1, accessory_id),
			position_state: PositionStateCharacteristic::new(id + 1 + 2, accessory_id),
			..Default::default()
        }
    }
}

impl HapService for WindowCoveringService {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_type(&self) -> HapType {
        self.hap_type
    }

    fn get_hidden(&self) -> bool {
        self.hidden
    }

    fn set_hidden(&mut self, hidden: bool) {
        self.hidden = hidden;
    }

    fn get_primary(&self) -> bool {
        self.primary
    }

    fn set_primary(&mut self, primary: bool) {
        self.primary = primary;
    }

    fn get_characteristic(&self, hap_type: HapType) -> Option<&dyn HapCharacteristic> {
        for characteristic in self.get_characteristics() {
            if characteristic.get_type() == hap_type {
                return Some(characteristic);
            }
        }
        None
    }

    fn get_mut_characteristic(&mut self, hap_type: HapType) -> Option<&mut dyn HapCharacteristic> {
        for characteristic in self.get_mut_characteristics() {
            if characteristic.get_type() == hap_type {
                return Some(characteristic);
            }
        }
        None
    }

    fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> {
        let mut characteristics: Vec<&dyn HapCharacteristic> = vec![
			&self.current_position,
			&self.target_position,
			&self.position_state,
		];
		if let Some(c) = &self.hold_position {
		    characteristics.push(c);
		}
		if let Some(c) = &self.target_horizontal_tilt_angle {
		    characteristics.push(c);
		}
		if let Some(c) = &self.target_vertical_tilt_angle {
		    characteristics.push(c);
		}
		if let Some(c) = &self.current_horizontal_tilt_angle {
		    characteristics.push(c);
		}
		if let Some(c) = &self.current_vertical_tilt_angle {
		    characteristics.push(c);
		}
		if let Some(c) = &self.obstruction_detected {
		    characteristics.push(c);
		}
		if let Some(c) = &self.name {
		    characteristics.push(c);
		}
		characteristics
    }

    fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
        let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
			&mut self.current_position,
			&mut self.target_position,
			&mut self.position_state,
		];
		if let Some(c) = &mut self.hold_position {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.target_horizontal_tilt_angle {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.target_vertical_tilt_angle {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.current_horizontal_tilt_angle {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.current_vertical_tilt_angle {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.obstruction_detected {
		    characteristics.push(c);
		}
		if let Some(c) = &mut self.name {
		    characteristics.push(c);
		}
		characteristics
    }
}

impl Serialize for WindowCoveringService {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("HapService", 5)?;
        state.serialize_field("iid", &self.get_id())?;
        state.serialize_field("type", &self.get_type())?;
        state.serialize_field("hidden", &self.get_hidden())?;
        state.serialize_field("primary", &self.get_primary())?;
        state.serialize_field("characteristics", &self.get_characteristics())?;
        // linked services left out for now
        state.end()
    }
}
