#[doc = "Register `_1_RXLINK_DSCR_BF0` reader"]
pub struct R(crate::R<_1_RXLINK_DSCR_BF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_1_RXLINK_DSCR_BF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_1_RXLINK_DSCR_BF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_1_RXLINK_DSCR_BF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLC1_RXLINK_DSCR_BF0` reader - "]
pub type SLC1_RXLINK_DSCR_BF0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn slc1_rxlink_dscr_bf0(&self) -> SLC1_RXLINK_DSCR_BF0_R {
        SLC1_RXLINK_DSCR_BF0_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_1_rxlink_dscr_bf0](index.html) module"]
pub struct _1_RXLINK_DSCR_BF0_SPEC;
impl crate::RegisterSpec for _1_RXLINK_DSCR_BF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_1_rxlink_dscr_bf0::R](R) reader structure"]
impl crate::Readable for _1_RXLINK_DSCR_BF0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets _1_RXLINK_DSCR_BF0 to value 0"]
impl crate::Resettable for _1_RXLINK_DSCR_BF0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
