use elemental_engine::material::{Material, Status};

pub struct Wood {
    status: Status,
}

impl Default for Wood {
    fn default() -> Self {
        Wood {
            status: Status::None,
        }
    }
}

impl Material for Wood {
    fn status(&self) -> Status {
        self.status
    }

    fn apply_material(&mut self, material: impl Material) {
        let applied_status = material.status();

        let new_state = match (self.status, applied_status) {
            (Status::Burning, Status::Burning) => Status::Burning,
            (Status::Burning, Status::Wet) => Status::None,
            (Status::Burning, Status::None) => Status::Burning,
            (Status::Wet, Status::Burning) => Status::None,
            (Status::Wet, Status::Wet) => Status::Wet,
            (Status::Wet, Status::None) => Status::Wet,
            (Status::None, Status::Burning) => Status::Burning,
            (Status::None, Status::Wet) => Status::Wet,
            (Status::None, Status::None) => Status::None,
        };

        self.status = new_state;
    }
}

#[test]
fn wood_burns() {
    let mut wood = Wood::default();
    let burning_wood = Wood {
        status: Status::Burning,
    };

    wood.apply_material(burning_wood);

    assert_eq!(wood.status, Status::Burning);
}

#[test]
fn wet_douse_burning_wood() {
    let mut burning_wood = Wood {
        status: Status::Burning,
    };
    let wet_wood = Wood {
        status: Status::Wet,
    };

    burning_wood.apply_material(wet_wood);

    assert_eq!(burning_wood.status, Status::None);
}
