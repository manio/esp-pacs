#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - EDGE_INT_ENABLE"]
    pub edge_int_enable: EDGE_INT_ENABLE,
    _reserved1: [u8; 0x04],
    #[doc = "0x0c - Controls SPI memory-mapped caching"]
    pub spi_cache: SPI_CACHE,
    _reserved2: [u8; 0x04],
    #[doc = "0x14 - DPORT_CTL"]
    pub dport_ctl: DPORT_CTL,
    _reserved3: [u8; 0x08],
    #[doc = "0x20 - SPI interrupt type register"]
    pub spi_interrupt_type: SPI_INTERRUPT_TYPE,
    #[doc = "0x24 - Control where the cache is mapped (unconfirmed)"]
    pub spi_cache_target: SPI_CACHE_TARGET,
    #[doc = "0x28 - IO Swap register"]
    pub ioswap: IOSWAP,
}
#[doc = "EDGE_INT_ENABLE (rw) register accessor: EDGE_INT_ENABLE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edge_int_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edge_int_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`edge_int_enable`] module"]
pub type EDGE_INT_ENABLE = crate::Reg<edge_int_enable::EDGE_INT_ENABLE_SPEC>;
#[doc = "EDGE_INT_ENABLE"]
pub mod edge_int_enable;
#[doc = "DPORT_CTL (rw) register accessor: DPORT_CTL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dport_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dport_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dport_ctl`] module"]
pub type DPORT_CTL = crate::Reg<dport_ctl::DPORT_CTL_SPEC>;
#[doc = "DPORT_CTL"]
pub mod dport_ctl;
#[doc = "IOSWAP (rw) register accessor: IO Swap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioswap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioswap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ioswap`] module"]
pub type IOSWAP = crate::Reg<ioswap::IOSWAP_SPEC>;
#[doc = "IO Swap register"]
pub mod ioswap;
#[doc = "SPI_CACHE (rw) register accessor: Controls SPI memory-mapped caching\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_cache::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_cache::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`spi_cache`] module"]
pub type SPI_CACHE = crate::Reg<spi_cache::SPI_CACHE_SPEC>;
#[doc = "Controls SPI memory-mapped caching"]
pub mod spi_cache;
#[doc = "SPI_INTERRUPT_TYPE (r) register accessor: SPI interrupt type register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_interrupt_type::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`spi_interrupt_type`] module"]
pub type SPI_INTERRUPT_TYPE = crate::Reg<spi_interrupt_type::SPI_INTERRUPT_TYPE_SPEC>;
#[doc = "SPI interrupt type register"]
pub mod spi_interrupt_type;
#[doc = "SPI_CACHE_TARGET (rw) register accessor: Control where the cache is mapped (unconfirmed)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_cache_target::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_cache_target::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`spi_cache_target`] module"]
pub type SPI_CACHE_TARGET = crate::Reg<spi_cache_target::SPI_CACHE_TARGET_SPEC>;
#[doc = "Control where the cache is mapped (unconfirmed)"]
pub mod spi_cache_target;
