#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0500],
    enable: Enable,
}
impl RegisterBlock {
    #[doc = "0x500 - Enable CRYPTOCELL subsystem."]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
}
#[doc = "ENABLE (rw) register accessor: Enable CRYPTOCELL subsystem.\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Enable CRYPTOCELL subsystem."]
pub mod enable;
