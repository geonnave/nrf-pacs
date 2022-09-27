#[doc = r"Register block"]
#[repr(C)]
pub struct GPIOPORT {
    #[doc = "0x00 - Description cluster: Select between secure and non-secure attribute for pins 0 to 31 of port n"]
    pub perm: PERM,
    #[doc = "0x04 - Description cluster: Prevent further modification of the corresponding PERM register"]
    pub lock: LOCK,
}
#[doc = "PERM (rw) register accessor: an alias for `Reg<PERM_SPEC>`"]
pub type PERM = crate::Reg<perm::PERM_SPEC>;
#[doc = "Description cluster: Select between secure and non-secure attribute for pins 0 to 31 of port n"]
pub mod perm;
#[doc = "LOCK (rw) register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Description cluster: Prevent further modification of the corresponding PERM register"]
pub mod lock;
