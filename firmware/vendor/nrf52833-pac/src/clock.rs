#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start HFXO crystal oscillator"]
    pub tasks_hfclkstart: crate::Reg<tasks_hfclkstart::TASKS_HFCLKSTART_SPEC>,
    #[doc = "0x04 - Stop HFXO crystal oscillator"]
    pub tasks_hfclkstop: crate::Reg<tasks_hfclkstop::TASKS_HFCLKSTOP_SPEC>,
    #[doc = "0x08 - Start LFCLK"]
    pub tasks_lfclkstart: crate::Reg<tasks_lfclkstart::TASKS_LFCLKSTART_SPEC>,
    #[doc = "0x0c - Stop LFCLK"]
    pub tasks_lfclkstop: crate::Reg<tasks_lfclkstop::TASKS_LFCLKSTOP_SPEC>,
    #[doc = "0x10 - Start calibration of LFRC"]
    pub tasks_cal: crate::Reg<tasks_cal::TASKS_CAL_SPEC>,
    #[doc = "0x14 - Start calibration timer"]
    pub tasks_ctstart: crate::Reg<tasks_ctstart::TASKS_CTSTART_SPEC>,
    #[doc = "0x18 - Stop calibration timer"]
    pub tasks_ctstop: crate::Reg<tasks_ctstop::TASKS_CTSTOP_SPEC>,
    _reserved7: [u8; 0xe4],
    #[doc = "0x100 - HFXO crystal oscillator started"]
    pub events_hfclkstarted: crate::Reg<events_hfclkstarted::EVENTS_HFCLKSTARTED_SPEC>,
    #[doc = "0x104 - LFCLK started"]
    pub events_lfclkstarted: crate::Reg<events_lfclkstarted::EVENTS_LFCLKSTARTED_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x10c - Calibration of LFRC completed"]
    pub events_done: crate::Reg<events_done::EVENTS_DONE_SPEC>,
    #[doc = "0x110 - Calibration timer timeout"]
    pub events_ctto: crate::Reg<events_ctto::EVENTS_CTTO_SPEC>,
    _reserved11: [u8; 0x14],
    #[doc = "0x128 - Calibration timer has been started and is ready to process new tasks"]
    pub events_ctstarted: crate::Reg<events_ctstarted::EVENTS_CTSTARTED_SPEC>,
    #[doc = "0x12c - Calibration timer has been stopped and is ready to process new tasks"]
    pub events_ctstopped: crate::Reg<events_ctstopped::EVENTS_CTSTOPPED_SPEC>,
    _reserved13: [u8; 0x01d4],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved15: [u8; 0xfc],
    #[doc = "0x408 - Status indicating that HFCLKSTART task has been triggered"]
    pub hfclkrun: crate::Reg<hfclkrun::HFCLKRUN_SPEC>,
    #[doc = "0x40c - HFCLK status"]
    pub hfclkstat: crate::Reg<hfclkstat::HFCLKSTAT_SPEC>,
    _reserved17: [u8; 0x04],
    #[doc = "0x414 - Status indicating that LFCLKSTART task has been triggered"]
    pub lfclkrun: crate::Reg<lfclkrun::LFCLKRUN_SPEC>,
    #[doc = "0x418 - LFCLK status"]
    pub lfclkstat: crate::Reg<lfclkstat::LFCLKSTAT_SPEC>,
    #[doc = "0x41c - Copy of LFCLKSRC register, set when LFCLKSTART task was triggered"]
    pub lfclksrccopy: crate::Reg<lfclksrccopy::LFCLKSRCCOPY_SPEC>,
    _reserved20: [u8; 0xf8],
    #[doc = "0x518 - Clock source for the LFCLK"]
    pub lfclksrc: crate::Reg<lfclksrc::LFCLKSRC_SPEC>,
    _reserved21: [u8; 0x0c],
    #[doc = "0x528 - HFXO debounce time. The HFXO is started by triggering the TASKS_HFCLKSTART task."]
    pub hfxodebounce: crate::Reg<hfxodebounce::HFXODEBOUNCE_SPEC>,
    #[doc = "0x52c - LFXO debounce time. The LFXO is started by triggering the TASKS_LFCLKSTART task when the LFCLKSRC register is configured for Xtal."]
    pub lfxodebounce: crate::Reg<lfxodebounce::LFXODEBOUNCE_SPEC>,
    _reserved23: [u8; 0x08],
    #[doc = "0x538 - Calibration timer interval"]
    pub ctiv: crate::Reg<ctiv::CTIV_SPEC>,
    _reserved24: [u8; 0x20],
    #[doc = "0x55c - Clocking options for the trace port debug interface"]
    pub traceconfig: crate::Reg<traceconfig::TRACECONFIG_SPEC>,
}
#[doc = "TASKS_HFCLKSTART register accessor: an alias for `Reg<TASKS_HFCLKSTART_SPEC>`"]
pub type TASKS_HFCLKSTART = crate::Reg<tasks_hfclkstart::TASKS_HFCLKSTART_SPEC>;
#[doc = "Start HFXO crystal oscillator"]
pub mod tasks_hfclkstart;
#[doc = "TASKS_HFCLKSTOP register accessor: an alias for `Reg<TASKS_HFCLKSTOP_SPEC>`"]
pub type TASKS_HFCLKSTOP = crate::Reg<tasks_hfclkstop::TASKS_HFCLKSTOP_SPEC>;
#[doc = "Stop HFXO crystal oscillator"]
pub mod tasks_hfclkstop;
#[doc = "TASKS_LFCLKSTART register accessor: an alias for `Reg<TASKS_LFCLKSTART_SPEC>`"]
pub type TASKS_LFCLKSTART = crate::Reg<tasks_lfclkstart::TASKS_LFCLKSTART_SPEC>;
#[doc = "Start LFCLK"]
pub mod tasks_lfclkstart;
#[doc = "TASKS_LFCLKSTOP register accessor: an alias for `Reg<TASKS_LFCLKSTOP_SPEC>`"]
pub type TASKS_LFCLKSTOP = crate::Reg<tasks_lfclkstop::TASKS_LFCLKSTOP_SPEC>;
#[doc = "Stop LFCLK"]
pub mod tasks_lfclkstop;
#[doc = "TASKS_CAL register accessor: an alias for `Reg<TASKS_CAL_SPEC>`"]
pub type TASKS_CAL = crate::Reg<tasks_cal::TASKS_CAL_SPEC>;
#[doc = "Start calibration of LFRC"]
pub mod tasks_cal;
#[doc = "TASKS_CTSTART register accessor: an alias for `Reg<TASKS_CTSTART_SPEC>`"]
pub type TASKS_CTSTART = crate::Reg<tasks_ctstart::TASKS_CTSTART_SPEC>;
#[doc = "Start calibration timer"]
pub mod tasks_ctstart;
#[doc = "TASKS_CTSTOP register accessor: an alias for `Reg<TASKS_CTSTOP_SPEC>`"]
pub type TASKS_CTSTOP = crate::Reg<tasks_ctstop::TASKS_CTSTOP_SPEC>;
#[doc = "Stop calibration timer"]
pub mod tasks_ctstop;
#[doc = "EVENTS_HFCLKSTARTED register accessor: an alias for `Reg<EVENTS_HFCLKSTARTED_SPEC>`"]
pub type EVENTS_HFCLKSTARTED = crate::Reg<events_hfclkstarted::EVENTS_HFCLKSTARTED_SPEC>;
#[doc = "HFXO crystal oscillator started"]
pub mod events_hfclkstarted;
#[doc = "EVENTS_LFCLKSTARTED register accessor: an alias for `Reg<EVENTS_LFCLKSTARTED_SPEC>`"]
pub type EVENTS_LFCLKSTARTED = crate::Reg<events_lfclkstarted::EVENTS_LFCLKSTARTED_SPEC>;
#[doc = "LFCLK started"]
pub mod events_lfclkstarted;
#[doc = "EVENTS_DONE register accessor: an alias for `Reg<EVENTS_DONE_SPEC>`"]
pub type EVENTS_DONE = crate::Reg<events_done::EVENTS_DONE_SPEC>;
#[doc = "Calibration of LFRC completed"]
pub mod events_done;
#[doc = "EVENTS_CTTO register accessor: an alias for `Reg<EVENTS_CTTO_SPEC>`"]
pub type EVENTS_CTTO = crate::Reg<events_ctto::EVENTS_CTTO_SPEC>;
#[doc = "Calibration timer timeout"]
pub mod events_ctto;
#[doc = "EVENTS_CTSTARTED register accessor: an alias for `Reg<EVENTS_CTSTARTED_SPEC>`"]
pub type EVENTS_CTSTARTED = crate::Reg<events_ctstarted::EVENTS_CTSTARTED_SPEC>;
#[doc = "Calibration timer has been started and is ready to process new tasks"]
pub mod events_ctstarted;
#[doc = "EVENTS_CTSTOPPED register accessor: an alias for `Reg<EVENTS_CTSTOPPED_SPEC>`"]
pub type EVENTS_CTSTOPPED = crate::Reg<events_ctstopped::EVENTS_CTSTOPPED_SPEC>;
#[doc = "Calibration timer has been stopped and is ready to process new tasks"]
pub mod events_ctstopped;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "HFCLKRUN register accessor: an alias for `Reg<HFCLKRUN_SPEC>`"]
pub type HFCLKRUN = crate::Reg<hfclkrun::HFCLKRUN_SPEC>;
#[doc = "Status indicating that HFCLKSTART task has been triggered"]
pub mod hfclkrun;
#[doc = "HFCLKSTAT register accessor: an alias for `Reg<HFCLKSTAT_SPEC>`"]
pub type HFCLKSTAT = crate::Reg<hfclkstat::HFCLKSTAT_SPEC>;
#[doc = "HFCLK status"]
pub mod hfclkstat;
#[doc = "LFCLKRUN register accessor: an alias for `Reg<LFCLKRUN_SPEC>`"]
pub type LFCLKRUN = crate::Reg<lfclkrun::LFCLKRUN_SPEC>;
#[doc = "Status indicating that LFCLKSTART task has been triggered"]
pub mod lfclkrun;
#[doc = "LFCLKSTAT register accessor: an alias for `Reg<LFCLKSTAT_SPEC>`"]
pub type LFCLKSTAT = crate::Reg<lfclkstat::LFCLKSTAT_SPEC>;
#[doc = "LFCLK status"]
pub mod lfclkstat;
#[doc = "LFCLKSRCCOPY register accessor: an alias for `Reg<LFCLKSRCCOPY_SPEC>`"]
pub type LFCLKSRCCOPY = crate::Reg<lfclksrccopy::LFCLKSRCCOPY_SPEC>;
#[doc = "Copy of LFCLKSRC register, set when LFCLKSTART task was triggered"]
pub mod lfclksrccopy;
#[doc = "LFCLKSRC register accessor: an alias for `Reg<LFCLKSRC_SPEC>`"]
pub type LFCLKSRC = crate::Reg<lfclksrc::LFCLKSRC_SPEC>;
#[doc = "Clock source for the LFCLK"]
pub mod lfclksrc;
#[doc = "HFXODEBOUNCE register accessor: an alias for `Reg<HFXODEBOUNCE_SPEC>`"]
pub type HFXODEBOUNCE = crate::Reg<hfxodebounce::HFXODEBOUNCE_SPEC>;
#[doc = "HFXO debounce time. The HFXO is started by triggering the TASKS_HFCLKSTART task."]
pub mod hfxodebounce;
#[doc = "LFXODEBOUNCE register accessor: an alias for `Reg<LFXODEBOUNCE_SPEC>`"]
pub type LFXODEBOUNCE = crate::Reg<lfxodebounce::LFXODEBOUNCE_SPEC>;
#[doc = "LFXO debounce time. The LFXO is started by triggering the TASKS_LFCLKSTART task when the LFCLKSRC register is configured for Xtal."]
pub mod lfxodebounce;
#[doc = "CTIV register accessor: an alias for `Reg<CTIV_SPEC>`"]
pub type CTIV = crate::Reg<ctiv::CTIV_SPEC>;
#[doc = "Calibration timer interval"]
pub mod ctiv;
#[doc = "TRACECONFIG register accessor: an alias for `Reg<TRACECONFIG_SPEC>`"]
pub type TRACECONFIG = crate::Reg<traceconfig::TRACECONFIG_SPEC>;
#[doc = "Clocking options for the trace port debug interface"]
pub mod traceconfig;
