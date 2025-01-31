#[doc = "Register `WPROTECT` reader"]
pub type R = crate::R<WPROTECT_SPEC>;
#[doc = "Register `WPROTECT` writer"]
pub type W = crate::W<WPROTECT_SPEC>;
#[doc = "Field `WDT_WKEY` reader - need_des"]
pub type WDT_WKEY_R = crate::FieldReader<u32>;
#[doc = "Field `WDT_WKEY` writer - need_des"]
pub type WDT_WKEY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn wdt_wkey(&self) -> WDT_WKEY_R {
        WDT_WKEY_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WPROTECT")
            .field("wdt_wkey", &format_args!("{}", self.wdt_wkey().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WPROTECT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_wkey(&mut self) -> WDT_WKEY_W<WPROTECT_SPEC, 0> {
        WDT_WKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wprotect::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wprotect::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WPROTECT_SPEC;
impl crate::RegisterSpec for WPROTECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wprotect::R`](R) reader structure"]
impl crate::Readable for WPROTECT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wprotect::W`](W) writer structure"]
impl crate::Writable for WPROTECT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WPROTECT to value 0"]
impl crate::Resettable for WPROTECT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
