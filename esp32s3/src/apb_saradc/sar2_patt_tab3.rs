#[doc = "Register `SAR2_PATT_TAB3` reader"]
pub struct R(crate::R<SAR2_PATT_TAB3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR2_PATT_TAB3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR2_PATT_TAB3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR2_PATT_TAB3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR2_PATT_TAB3` writer"]
pub struct W(crate::W<SAR2_PATT_TAB3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR2_PATT_TAB3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SAR2_PATT_TAB3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR2_PATT_TAB3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SARADC_SAR2_PATT_TAB3` reader - Item 8 ~ 11 for pattern table 2 (each item 6bit)"]
pub type SARADC_SAR2_PATT_TAB3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SARADC_SAR2_PATT_TAB3` writer - Item 8 ~ 11 for pattern table 2 (each item 6bit)"]
pub type SARADC_SAR2_PATT_TAB3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR2_PATT_TAB3_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - Item 8 ~ 11 for pattern table 2 (each item 6bit)"]
    #[inline(always)]
    pub fn saradc_sar2_patt_tab3(&self) -> SARADC_SAR2_PATT_TAB3_R {
        SARADC_SAR2_PATT_TAB3_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Item 8 ~ 11 for pattern table 2 (each item 6bit)"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_sar2_patt_tab3(&mut self) -> SARADC_SAR2_PATT_TAB3_W<0> {
        SARADC_SAR2_PATT_TAB3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure apb saradc pattern table\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar2_patt_tab3](index.html) module"]
pub struct SAR2_PATT_TAB3_SPEC;
impl crate::RegisterSpec for SAR2_PATT_TAB3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar2_patt_tab3::R](R) reader structure"]
impl crate::Readable for SAR2_PATT_TAB3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar2_patt_tab3::W](W) writer structure"]
impl crate::Writable for SAR2_PATT_TAB3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR2_PATT_TAB3 to value 0"]
impl crate::Resettable for SAR2_PATT_TAB3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
