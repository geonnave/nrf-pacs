#[doc = r"Register block"]
#[repr(C)]
pub struct VREGH {
    #[doc = "0x00 - DC/DC enable register for VREGH"]
    pub dcdcen: DCDCEN,
}
#[doc = "DCDCEN (rw) register accessor: an alias for `Reg<DCDCEN_SPEC>`"]
pub type DCDCEN = crate::Reg<dcdcen::DCDCEN_SPEC>;
#[doc = "DC/DC enable register for VREGH"]
pub mod dcdcen;
