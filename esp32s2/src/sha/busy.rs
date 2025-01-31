#[doc = "Register `BUSY` reader"]
pub type R = crate::R<BUSY_SPEC>;
#[doc = "Field `STATE` reader - Indicates the states of SHA accelerator. 1'h0: idle 1'h1: busy"]
pub type STATE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicates the states of SHA accelerator. 1'h0: idle 1'h1: busy"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUSY")
            .field("state", &format_args!("{}", self.state().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BUSY_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Indicates if SHA Accelerator is busy or not\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`busy::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUSY_SPEC;
impl crate::RegisterSpec for BUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`busy::R`](R) reader structure"]
impl crate::Readable for BUSY_SPEC {}
#[doc = "`reset()` method sets BUSY to value 0"]
impl crate::Resettable for BUSY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
