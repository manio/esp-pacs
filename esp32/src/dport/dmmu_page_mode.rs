#[doc = "Register `DMMU_PAGE_MODE` reader"]
pub type R = crate::R<DMMU_PAGE_MODE_SPEC>;
#[doc = "Register `DMMU_PAGE_MODE` writer"]
pub type W = crate::W<DMMU_PAGE_MODE_SPEC>;
#[doc = "Field `INTERNAL_SRAM_DMMU_ENA` reader - "]
pub type INTERNAL_SRAM_DMMU_ENA_R = crate::BitReader;
#[doc = "Field `INTERNAL_SRAM_DMMU_ENA` writer - "]
pub type INTERNAL_SRAM_DMMU_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMMU_PAGE_MODE` reader - "]
pub type DMMU_PAGE_MODE_R = crate::FieldReader;
#[doc = "Field `DMMU_PAGE_MODE` writer - "]
pub type DMMU_PAGE_MODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn internal_sram_dmmu_ena(&self) -> INTERNAL_SRAM_DMMU_ENA_R {
        INTERNAL_SRAM_DMMU_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn dmmu_page_mode(&self) -> DMMU_PAGE_MODE_R {
        DMMU_PAGE_MODE_R::new(((self.bits >> 1) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMMU_PAGE_MODE")
            .field(
                "internal_sram_dmmu_ena",
                &format_args!("{}", self.internal_sram_dmmu_ena().bit()),
            )
            .field(
                "dmmu_page_mode",
                &format_args!("{}", self.dmmu_page_mode().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMMU_PAGE_MODE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn internal_sram_dmmu_ena(&mut self) -> INTERNAL_SRAM_DMMU_ENA_W<DMMU_PAGE_MODE_SPEC, 0> {
        INTERNAL_SRAM_DMMU_ENA_W::new(self)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    #[must_use]
    pub fn dmmu_page_mode(&mut self) -> DMMU_PAGE_MODE_W<DMMU_PAGE_MODE_SPEC, 1> {
        DMMU_PAGE_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmmu_page_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmmu_page_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMMU_PAGE_MODE_SPEC;
impl crate::RegisterSpec for DMMU_PAGE_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmmu_page_mode::R`](R) reader structure"]
impl crate::Readable for DMMU_PAGE_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmmu_page_mode::W`](W) writer structure"]
impl crate::Writable for DMMU_PAGE_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMMU_PAGE_MODE to value 0"]
impl crate::Resettable for DMMU_PAGE_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
