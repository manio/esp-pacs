#[doc = "Register `TAR0_LOW` reader"]
pub type R = crate::R<TAR0_LOW_SPEC>;
#[doc = "Register `TAR0_LOW` writer"]
pub type W = crate::W<TAR0_LOW_SPEC>;
#[doc = "Field `MAIN_TIMER_TAR_LOW0` reader - need_des"]
pub type MAIN_TIMER_TAR_LOW0_R = crate::FieldReader<u32>;
#[doc = "Field `MAIN_TIMER_TAR_LOW0` writer - need_des"]
pub type MAIN_TIMER_TAR_LOW0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn main_timer_tar_low0(&self) -> MAIN_TIMER_TAR_LOW0_R {
        MAIN_TIMER_TAR_LOW0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TAR0_LOW")
            .field(
                "main_timer_tar_low0",
                &format_args!("{}", self.main_timer_tar_low0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TAR0_LOW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn main_timer_tar_low0(&mut self) -> MAIN_TIMER_TAR_LOW0_W<TAR0_LOW_SPEC, 0> {
        MAIN_TIMER_TAR_LOW0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tar0_low::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tar0_low::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAR0_LOW_SPEC;
impl crate::RegisterSpec for TAR0_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tar0_low::R`](R) reader structure"]
impl crate::Readable for TAR0_LOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tar0_low::W`](W) writer structure"]
impl crate::Writable for TAR0_LOW_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAR0_LOW to value 0"]
impl crate::Resettable for TAR0_LOW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
