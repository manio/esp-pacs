#[doc = "Register `CONFIG_UPDATE` writer"]
pub type W = crate::W<CONFIG_UPDATE_SPEC>;
#[doc = "Field `CONFIG_UPDATE` writer - Write 1 to this register would update the value of configure registers from APB clock domain to 48MHz clock domain."]
pub type CONFIG_UPDATE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONFIG_UPDATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to this register would update the value of configure registers from APB clock domain to 48MHz clock domain."]
    #[inline(always)]
    #[must_use]
    pub fn config_update(&mut self) -> CONFIG_UPDATE_W<CONFIG_UPDATE_SPEC, 0> {
        CONFIG_UPDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Configuration registers' value update\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_update::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFIG_UPDATE_SPEC;
impl crate::RegisterSpec for CONFIG_UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`config_update::W`](W) writer structure"]
impl crate::Writable for CONFIG_UPDATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFIG_UPDATE to value 0"]
impl crate::Resettable for CONFIG_UPDATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
