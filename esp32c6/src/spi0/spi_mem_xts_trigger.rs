#[doc = "Register `SPI_MEM_XTS_TRIGGER` writer"]
pub type W = crate::W<SPI_MEM_XTS_TRIGGER_SPEC>;
#[doc = "Field `SPI_XTS_TRIGGER` writer - Set this bit to trigger the process of manual encryption calculation. This action should only be asserted when manual encryption status is 0. After this action, manual encryption status becomes 1. After calculation is done, manual encryption status becomes 2."]
pub type SPI_XTS_TRIGGER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_XTS_TRIGGER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to trigger the process of manual encryption calculation. This action should only be asserted when manual encryption status is 0. After this action, manual encryption status becomes 1. After calculation is done, manual encryption status becomes 2."]
    #[inline(always)]
    #[must_use]
    pub fn spi_xts_trigger(&mut self) -> SPI_XTS_TRIGGER_W<SPI_MEM_XTS_TRIGGER_SPEC, 0> {
        SPI_XTS_TRIGGER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Manual Encryption physical address register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_xts_trigger::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_XTS_TRIGGER_SPEC;
impl crate::RegisterSpec for SPI_MEM_XTS_TRIGGER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`spi_mem_xts_trigger::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_XTS_TRIGGER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_XTS_TRIGGER to value 0"]
impl crate::Resettable for SPI_MEM_XTS_TRIGGER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
