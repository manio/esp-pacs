#[doc = "Register `RETENTION_CTRL1` reader"]
pub type R = crate::R<RETENTION_CTRL1_SPEC>;
#[doc = "Register `RETENTION_CTRL1` writer"]
pub type W = crate::W<RETENTION_CTRL1_SPEC>;
#[doc = "Field `RETENTION_TAG_LINK_ADDR` reader - ******* Description ***********"]
pub type RETENTION_TAG_LINK_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `RETENTION_TAG_LINK_ADDR` writer - ******* Description ***********"]
pub type RETENTION_TAG_LINK_ADDR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 27, O, u32>;
impl R {
    #[doc = "Bits 0:26 - ******* Description ***********"]
    #[inline(always)]
    pub fn retention_tag_link_addr(&self) -> RETENTION_TAG_LINK_ADDR_R {
        RETENTION_TAG_LINK_ADDR_R::new(self.bits & 0x07ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RETENTION_CTRL1")
            .field(
                "retention_tag_link_addr",
                &format_args!("{}", self.retention_tag_link_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RETENTION_CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:26 - ******* Description ***********"]
    #[inline(always)]
    #[must_use]
    pub fn retention_tag_link_addr(
        &mut self,
    ) -> RETENTION_TAG_LINK_ADDR_W<RETENTION_CTRL1_SPEC, 0> {
        RETENTION_TAG_LINK_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`retention_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`retention_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RETENTION_CTRL1_SPEC;
impl crate::RegisterSpec for RETENTION_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`retention_ctrl1::R`](R) reader structure"]
impl crate::Readable for RETENTION_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`retention_ctrl1::W`](W) writer structure"]
impl crate::Writable for RETENTION_CTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RETENTION_CTRL1 to value 0"]
impl crate::Resettable for RETENTION_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
