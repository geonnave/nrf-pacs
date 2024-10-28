#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_activate: TasksActivate,
    tasks_readstart: TasksReadstart,
    tasks_writestart: TasksWritestart,
    tasks_erasestart: TasksErasestart,
    tasks_deactivate: TasksDeactivate,
    _reserved5: [u8; 0xec],
    events_ready: EventsReady,
    _reserved6: [u8; 0x01fc],
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved9: [u8; 0x01f4],
    enable: Enable,
    read: Read,
    write: Write,
    erase: Erase,
    psel: Psel,
    xipoffset: Xipoffset,
    ifconfig0: Ifconfig0,
    _reserved16: [u8; 0xb8],
    ifconfig1: Ifconfig1,
    status: Status,
    _reserved18: [u8; 0x0c],
    dpmdur: Dpmdur,
    _reserved19: [u8; 0x0c],
    addrconf: Addrconf,
    _reserved20: [u8; 0x0c],
    cinstrconf: Cinstrconf,
    cinstrdat0: Cinstrdat0,
    cinstrdat1: Cinstrdat1,
    iftiming: Iftiming,
}
impl RegisterBlock {
    #[doc = "0x00 - Activate QSPI interface"]
    #[inline(always)]
    pub const fn tasks_activate(&self) -> &TasksActivate {
        &self.tasks_activate
    }
    #[doc = "0x04 - Start transfer from external flash memory to internal RAM"]
    #[inline(always)]
    pub const fn tasks_readstart(&self) -> &TasksReadstart {
        &self.tasks_readstart
    }
    #[doc = "0x08 - Start transfer from internal RAM to external flash memory"]
    #[inline(always)]
    pub const fn tasks_writestart(&self) -> &TasksWritestart {
        &self.tasks_writestart
    }
    #[doc = "0x0c - Start external flash memory erase operation"]
    #[inline(always)]
    pub const fn tasks_erasestart(&self) -> &TasksErasestart {
        &self.tasks_erasestart
    }
    #[doc = "0x10 - Deactivate QSPI interface"]
    #[inline(always)]
    pub const fn tasks_deactivate(&self) -> &TasksDeactivate {
        &self.tasks_deactivate
    }
    #[doc = "0x100 - QSPI peripheral is ready. This event will be generated as a response to any QSPI task."]
    #[inline(always)]
    pub const fn events_ready(&self) -> &EventsReady {
        &self.events_ready
    }
    #[doc = "0x300 - Enable or disable interrupt"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x304 - Enable interrupt"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x308 - Disable interrupt"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x500 - Enable QSPI peripheral and acquire the pins selected in PSELn registers"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x504..0x510 - Unspecified"]
    #[inline(always)]
    pub const fn read(&self) -> &Read {
        &self.read
    }
    #[doc = "0x510..0x51c - Unspecified"]
    #[inline(always)]
    pub const fn write(&self) -> &Write {
        &self.write
    }
    #[doc = "0x51c..0x524 - Unspecified"]
    #[inline(always)]
    pub const fn erase(&self) -> &Erase {
        &self.erase
    }
    #[doc = "0x524..0x540 - Unspecified"]
    #[inline(always)]
    pub const fn psel(&self) -> &Psel {
        &self.psel
    }
    #[doc = "0x540 - Address offset into the external memory for Execute in Place operation."]
    #[inline(always)]
    pub const fn xipoffset(&self) -> &Xipoffset {
        &self.xipoffset
    }
    #[doc = "0x544 - Interface configuration."]
    #[inline(always)]
    pub const fn ifconfig0(&self) -> &Ifconfig0 {
        &self.ifconfig0
    }
    #[doc = "0x600 - Interface configuration."]
    #[inline(always)]
    pub const fn ifconfig1(&self) -> &Ifconfig1 {
        &self.ifconfig1
    }
    #[doc = "0x604 - Status register."]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x614 - Set the duration required to enter/exit deep power-down mode (DPM)."]
    #[inline(always)]
    pub const fn dpmdur(&self) -> &Dpmdur {
        &self.dpmdur
    }
    #[doc = "0x624 - Extended address configuration."]
    #[inline(always)]
    pub const fn addrconf(&self) -> &Addrconf {
        &self.addrconf
    }
    #[doc = "0x634 - Custom instruction configuration register."]
    #[inline(always)]
    pub const fn cinstrconf(&self) -> &Cinstrconf {
        &self.cinstrconf
    }
    #[doc = "0x638 - Custom instruction data register 0."]
    #[inline(always)]
    pub const fn cinstrdat0(&self) -> &Cinstrdat0 {
        &self.cinstrdat0
    }
    #[doc = "0x63c - Custom instruction data register 1."]
    #[inline(always)]
    pub const fn cinstrdat1(&self) -> &Cinstrdat1 {
        &self.cinstrdat1
    }
    #[doc = "0x640 - SPI interface timing."]
    #[inline(always)]
    pub const fn iftiming(&self) -> &Iftiming {
        &self.iftiming
    }
}
#[doc = "TASKS_ACTIVATE (w) register accessor: Activate QSPI interface\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_activate::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_activate`]
module"]
#[doc(alias = "TASKS_ACTIVATE")]
pub type TasksActivate = crate::Reg<tasks_activate::TasksActivateSpec>;
#[doc = "Activate QSPI interface"]
pub mod tasks_activate;
#[doc = "TASKS_READSTART (w) register accessor: Start transfer from external flash memory to internal RAM\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_readstart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_readstart`]
module"]
#[doc(alias = "TASKS_READSTART")]
pub type TasksReadstart = crate::Reg<tasks_readstart::TasksReadstartSpec>;
#[doc = "Start transfer from external flash memory to internal RAM"]
pub mod tasks_readstart;
#[doc = "TASKS_WRITESTART (w) register accessor: Start transfer from internal RAM to external flash memory\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_writestart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_writestart`]
module"]
#[doc(alias = "TASKS_WRITESTART")]
pub type TasksWritestart = crate::Reg<tasks_writestart::TasksWritestartSpec>;
#[doc = "Start transfer from internal RAM to external flash memory"]
pub mod tasks_writestart;
#[doc = "TASKS_ERASESTART (w) register accessor: Start external flash memory erase operation\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_erasestart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_erasestart`]
module"]
#[doc(alias = "TASKS_ERASESTART")]
pub type TasksErasestart = crate::Reg<tasks_erasestart::TasksErasestartSpec>;
#[doc = "Start external flash memory erase operation"]
pub mod tasks_erasestart;
#[doc = "TASKS_DEACTIVATE (w) register accessor: Deactivate QSPI interface\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_deactivate::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_deactivate`]
module"]
#[doc(alias = "TASKS_DEACTIVATE")]
pub type TasksDeactivate = crate::Reg<tasks_deactivate::TasksDeactivateSpec>;
#[doc = "Deactivate QSPI interface"]
pub mod tasks_deactivate;
#[doc = "EVENTS_READY (rw) register accessor: QSPI peripheral is ready. This event will be generated as a response to any QSPI task.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ready::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ready::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_ready`]
module"]
#[doc(alias = "EVENTS_READY")]
pub type EventsReady = crate::Reg<events_ready::EventsReadySpec>;
#[doc = "QSPI peripheral is ready. This event will be generated as a response to any QSPI task."]
pub mod events_ready;
#[doc = "INTEN (rw) register accessor: Enable or disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "INTENSET (rw) register accessor: Enable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`]
module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: Disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`]
module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "ENABLE (rw) register accessor: Enable QSPI peripheral and acquire the pins selected in PSELn registers\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Enable QSPI peripheral and acquire the pins selected in PSELn registers"]
pub mod enable;
#[doc = "Unspecified"]
pub use self::read::Read;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod read;
#[doc = "Unspecified"]
pub use self::write::Write;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod write;
#[doc = "Unspecified"]
pub use self::erase::Erase;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod erase;
#[doc = "Unspecified"]
pub use self::psel::Psel;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "XIPOFFSET (rw) register accessor: Address offset into the external memory for Execute in Place operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`xipoffset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xipoffset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xipoffset`]
module"]
#[doc(alias = "XIPOFFSET")]
pub type Xipoffset = crate::Reg<xipoffset::XipoffsetSpec>;
#[doc = "Address offset into the external memory for Execute in Place operation."]
pub mod xipoffset;
#[doc = "IFCONFIG0 (rw) register accessor: Interface configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`ifconfig0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifconfig0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifconfig0`]
module"]
#[doc(alias = "IFCONFIG0")]
pub type Ifconfig0 = crate::Reg<ifconfig0::Ifconfig0Spec>;
#[doc = "Interface configuration."]
pub mod ifconfig0;
#[doc = "IFCONFIG1 (rw) register accessor: Interface configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`ifconfig1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifconfig1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifconfig1`]
module"]
#[doc(alias = "IFCONFIG1")]
pub type Ifconfig1 = crate::Reg<ifconfig1::Ifconfig1Spec>;
#[doc = "Interface configuration."]
pub mod ifconfig1;
#[doc = "STATUS (r) register accessor: Status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status register."]
pub mod status;
#[doc = "DPMDUR (rw) register accessor: Set the duration required to enter/exit deep power-down mode (DPM).\n\nYou can [`read`](crate::Reg::read) this register and get [`dpmdur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpmdur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpmdur`]
module"]
#[doc(alias = "DPMDUR")]
pub type Dpmdur = crate::Reg<dpmdur::DpmdurSpec>;
#[doc = "Set the duration required to enter/exit deep power-down mode (DPM)."]
pub mod dpmdur;
#[doc = "ADDRCONF (rw) register accessor: Extended address configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`addrconf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrconf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addrconf`]
module"]
#[doc(alias = "ADDRCONF")]
pub type Addrconf = crate::Reg<addrconf::AddrconfSpec>;
#[doc = "Extended address configuration."]
pub mod addrconf;
#[doc = "CINSTRCONF (rw) register accessor: Custom instruction configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cinstrconf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cinstrconf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cinstrconf`]
module"]
#[doc(alias = "CINSTRCONF")]
pub type Cinstrconf = crate::Reg<cinstrconf::CinstrconfSpec>;
#[doc = "Custom instruction configuration register."]
pub mod cinstrconf;
#[doc = "CINSTRDAT0 (rw) register accessor: Custom instruction data register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`cinstrdat0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cinstrdat0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cinstrdat0`]
module"]
#[doc(alias = "CINSTRDAT0")]
pub type Cinstrdat0 = crate::Reg<cinstrdat0::Cinstrdat0Spec>;
#[doc = "Custom instruction data register 0."]
pub mod cinstrdat0;
#[doc = "CINSTRDAT1 (rw) register accessor: Custom instruction data register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`cinstrdat1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cinstrdat1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cinstrdat1`]
module"]
#[doc(alias = "CINSTRDAT1")]
pub type Cinstrdat1 = crate::Reg<cinstrdat1::Cinstrdat1Spec>;
#[doc = "Custom instruction data register 1."]
pub mod cinstrdat1;
#[doc = "IFTIMING (rw) register accessor: SPI interface timing.\n\nYou can [`read`](crate::Reg::read) this register and get [`iftiming::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iftiming::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iftiming`]
module"]
#[doc(alias = "IFTIMING")]
pub type Iftiming = crate::Reg<iftiming::IftimingSpec>;
#[doc = "SPI interface timing."]
pub mod iftiming;
