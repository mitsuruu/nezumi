use super::bind;
use crate::args::{Binding, Button, MouseFn, ScrollDirection};
use hidapi::HidDevice;

pub fn set(device: &HidDevice, direction: ScrollDirection) {
	for i in 1..=3 {
		match direction {
			ScrollDirection::Default => {
				// Up => Up
				bind::set(
					device,
					Some(i),
					Button::ScrollUp,
					Binding::Mouse(MouseFn::ScrollUp),
				);

				// Down => Down
				bind::set(
					device,
					Some(i),
					Button::ScrollDown,
					Binding::Mouse(MouseFn::ScrollDown),
				);
			}

			ScrollDirection::Invert => {
				// Up => Down
				bind::set(
					device,
					Some(i),
					Button::ScrollUp,
					Binding::Mouse(MouseFn::ScrollDown),
				);

				// Down => Up
				bind::set(
					device,
					Some(i),
					Button::ScrollDown,
					Binding::Mouse(MouseFn::ScrollUp),
				);
			}
		}
	}
}
