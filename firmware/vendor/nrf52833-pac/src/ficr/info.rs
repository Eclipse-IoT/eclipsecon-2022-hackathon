#[doc = "PART register accessor: an alias for `Reg<PART_SPEC>`"]
pub type PART = crate::Reg<part::PART_SPEC>;
#[doc = "Part code"]
pub mod part;
#[doc = "VARIANT register accessor: an alias for `Reg<VARIANT_SPEC>`"]
pub type VARIANT = crate::Reg<variant::VARIANT_SPEC>;
#[doc = "Build code (hardware version and production configuration)"]
pub mod variant;
#[doc = "PACKAGE register accessor: an alias for `Reg<PACKAGE_SPEC>`"]
pub type PACKAGE = crate::Reg<package::PACKAGE_SPEC>;
#[doc = "Package option"]
pub mod package;
#[doc = "RAM register accessor: an alias for `Reg<RAM_SPEC>`"]
pub type RAM = crate::Reg<ram::RAM_SPEC>;
#[doc = "RAM variant"]
pub mod ram;
#[doc = "FLASH register accessor: an alias for `Reg<FLASH_SPEC>`"]
pub type FLASH = crate::Reg<flash::FLASH_SPEC>;
#[doc = "Flash variant"]
pub mod flash;
