#[doc = "Register `LACTUPDATE` writer"]
pub type W = crate::W<LACTUPDATE_SPEC>;
#[doc = "Field `LACT_UPDATE` writer - Reserved."]
pub type LACT_UPDATE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LACTUPDATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn lact_update(&mut self) -> LACT_UPDATE_W<LACTUPDATE_SPEC, 0> {
        LACT_UPDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "LACT update register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactupdate::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LACTUPDATE_SPEC;
impl crate::RegisterSpec for LACTUPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lactupdate::W`](W) writer structure"]
impl crate::Writable for LACTUPDATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LACTUPDATE to value 0"]
impl crate::Resettable for LACTUPDATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
