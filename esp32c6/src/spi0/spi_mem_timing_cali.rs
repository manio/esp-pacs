#[doc = "Register `SPI_MEM_TIMING_CALI` reader"]
pub type R = crate::R<SPI_MEM_TIMING_CALI_SPEC>;
#[doc = "Register `SPI_MEM_TIMING_CALI` writer"]
pub type W = crate::W<SPI_MEM_TIMING_CALI_SPEC>;
#[doc = "Field `SPI_MEM_TIMING_CLK_ENA` reader - The bit is used to enable timing adjust clock for all reading operations."]
pub type SPI_MEM_TIMING_CLK_ENA_R = crate::BitReader;
#[doc = "Field `SPI_MEM_TIMING_CLK_ENA` writer - The bit is used to enable timing adjust clock for all reading operations."]
pub type SPI_MEM_TIMING_CLK_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI_MEM_TIMING_CALI` reader - The bit is used to enable timing auto-calibration for all reading operations."]
pub type SPI_MEM_TIMING_CALI_R = crate::BitReader;
#[doc = "Field `SPI_MEM_TIMING_CALI` writer - The bit is used to enable timing auto-calibration for all reading operations."]
pub type SPI_MEM_TIMING_CALI_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI_MEM_EXTRA_DUMMY_CYCLELEN` reader - add extra dummy spi clock cycle length for spi clock calibration."]
pub type SPI_MEM_EXTRA_DUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_EXTRA_DUMMY_CYCLELEN` writer - add extra dummy spi clock cycle length for spi clock calibration."]
pub type SPI_MEM_EXTRA_DUMMY_CYCLELEN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `SPI_MEM_DLL_TIMING_CALI` reader - Set this bit to enable DLL for timing calibration in DDR mode when accessed to flash."]
pub type SPI_MEM_DLL_TIMING_CALI_R = crate::BitReader;
#[doc = "Field `UPDATE` writer - Set this bit to update delay mode, delay num and extra dummy in MSPI."]
pub type UPDATE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable timing adjust clock for all reading operations."]
    #[inline(always)]
    pub fn spi_mem_timing_clk_ena(&self) -> SPI_MEM_TIMING_CLK_ENA_R {
        SPI_MEM_TIMING_CLK_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable timing auto-calibration for all reading operations."]
    #[inline(always)]
    pub fn spi_mem_timing_cali(&self) -> SPI_MEM_TIMING_CALI_R {
        SPI_MEM_TIMING_CALI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - add extra dummy spi clock cycle length for spi clock calibration."]
    #[inline(always)]
    pub fn spi_mem_extra_dummy_cyclelen(&self) -> SPI_MEM_EXTRA_DUMMY_CYCLELEN_R {
        SPI_MEM_EXTRA_DUMMY_CYCLELEN_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - Set this bit to enable DLL for timing calibration in DDR mode when accessed to flash."]
    #[inline(always)]
    pub fn spi_mem_dll_timing_cali(&self) -> SPI_MEM_DLL_TIMING_CALI_R {
        SPI_MEM_DLL_TIMING_CALI_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_TIMING_CALI")
            .field(
                "spi_mem_timing_clk_ena",
                &format_args!("{}", self.spi_mem_timing_clk_ena().bit()),
            )
            .field(
                "spi_mem_timing_cali",
                &format_args!("{}", self.spi_mem_timing_cali().bit()),
            )
            .field(
                "spi_mem_extra_dummy_cyclelen",
                &format_args!("{}", self.spi_mem_extra_dummy_cyclelen().bits()),
            )
            .field(
                "spi_mem_dll_timing_cali",
                &format_args!("{}", self.spi_mem_dll_timing_cali().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_TIMING_CALI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable timing adjust clock for all reading operations."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_timing_clk_ena(
        &mut self,
    ) -> SPI_MEM_TIMING_CLK_ENA_W<SPI_MEM_TIMING_CALI_SPEC, 0> {
        SPI_MEM_TIMING_CLK_ENA_W::new(self)
    }
    #[doc = "Bit 1 - The bit is used to enable timing auto-calibration for all reading operations."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_timing_cali(&mut self) -> SPI_MEM_TIMING_CALI_W<SPI_MEM_TIMING_CALI_SPEC, 1> {
        SPI_MEM_TIMING_CALI_W::new(self)
    }
    #[doc = "Bits 2:4 - add extra dummy spi clock cycle length for spi clock calibration."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_extra_dummy_cyclelen(
        &mut self,
    ) -> SPI_MEM_EXTRA_DUMMY_CYCLELEN_W<SPI_MEM_TIMING_CALI_SPEC, 2> {
        SPI_MEM_EXTRA_DUMMY_CYCLELEN_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to update delay mode, delay num and extra dummy in MSPI."]
    #[inline(always)]
    #[must_use]
    pub fn update(&mut self) -> UPDATE_W<SPI_MEM_TIMING_CALI_SPEC, 6> {
        UPDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SPI0 flash timing calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_timing_cali::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_timing_cali::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_TIMING_CALI_SPEC;
impl crate::RegisterSpec for SPI_MEM_TIMING_CALI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_timing_cali::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_TIMING_CALI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_timing_cali::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_TIMING_CALI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_TIMING_CALI to value 0x01"]
impl crate::Resettable for SPI_MEM_TIMING_CALI_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
