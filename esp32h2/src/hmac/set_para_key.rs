#[doc = "Register `SET_PARA_KEY` writer"]
pub type W = crate::W<SET_PARA_KEY_SPEC>;
#[doc = "Field `KEY_SET` writer - Set hmac parameter key."]
pub type KEY_SET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SET_PARA_KEY_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:2 - Set hmac parameter key."]
    #[inline(always)]
    #[must_use]
    pub fn key_set(&mut self) -> KEY_SET_W<SET_PARA_KEY_SPEC, 0> {
        KEY_SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Configure key.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_para_key::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SET_PARA_KEY_SPEC;
impl crate::RegisterSpec for SET_PARA_KEY_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`set_para_key::W`](W) writer structure"]
impl crate::Writable for SET_PARA_KEY_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SET_PARA_KEY to value 0"]
impl crate::Resettable for SET_PARA_KEY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
