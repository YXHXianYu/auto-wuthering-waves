use aah_controller::PcControllerTrait;





pub struct PcControllerWrapper {
    pub pc_controller: Box<dyn PcControllerTrait + Sync + Send>,
}

impl PcControllerWrapper {
    pub fn new() -> Self {
        let pc_controller = aah_controller::pc_controller::create_pc_controller().unwrap();
        Self {
            pc_controller,
        }
    }
}