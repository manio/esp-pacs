#[doc = "Register `OUT_ENABLE` reader"]
pub type R = crate::R<OUT_ENABLE_SPEC>;
#[doc = "Register `OUT_ENABLE` writer"]
pub type W = crate::W<OUT_ENABLE_SPEC>;
#[doc = "Field `LP_GPIO_ENABLE` reader - set lp gpio output data"]
pub type LP_GPIO_ENABLE_R = crate::FieldReader;
#[doc = "Field `LP_GPIO_ENABLE` writer - set lp gpio output data"]
pub type LP_GPIO_ENABLE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - set lp gpio output data"]
    #[inline(always)]
    pub fn lp_gpio_enable(&self) -> LP_GPIO_ENABLE_R {
        LP_GPIO_ENABLE_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_ENABLE")
            .field(
                "lp_gpio_enable",
                &format_args!("{}", self.lp_gpio_enable().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_ENABLE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - set lp gpio output data"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio_enable(&mut self) -> LP_GPIO_ENABLE_W<OUT_ENABLE_SPEC, 0> {
        LP_GPIO_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_ENABLE_SPEC;
impl crate::RegisterSpec for OUT_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_enable::R`](R) reader structure"]
impl crate::Readable for OUT_ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_enable::W`](W) writer structure"]
impl crate::Writable for OUT_ENABLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_ENABLE to value 0"]
impl crate::Resettable for OUT_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
