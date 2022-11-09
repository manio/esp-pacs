#[doc = "Register `SHROM_MPU_TABLE12` reader"]
pub struct R(crate::R<SHROM_MPU_TABLE12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHROM_MPU_TABLE12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHROM_MPU_TABLE12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHROM_MPU_TABLE12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHROM_MPU_TABLE12` writer"]
pub struct W(crate::W<SHROM_MPU_TABLE12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHROM_MPU_TABLE12_SPEC>;
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
impl From<crate::W<SHROM_MPU_TABLE12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHROM_MPU_TABLE12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHROM_MPU_TABLE12` reader - "]
pub type SHROM_MPU_TABLE12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHROM_MPU_TABLE12` writer - "]
pub type SHROM_MPU_TABLE12_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SHROM_MPU_TABLE12_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn shrom_mpu_table12(&self) -> SHROM_MPU_TABLE12_R {
        SHROM_MPU_TABLE12_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn shrom_mpu_table12(&mut self) -> SHROM_MPU_TABLE12_W<0> {
        SHROM_MPU_TABLE12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shrom_mpu_table12](index.html) module"]
pub struct SHROM_MPU_TABLE12_SPEC;
impl crate::RegisterSpec for SHROM_MPU_TABLE12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shrom_mpu_table12::R](R) reader structure"]
impl crate::Readable for SHROM_MPU_TABLE12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shrom_mpu_table12::W](W) writer structure"]
impl crate::Writable for SHROM_MPU_TABLE12_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHROM_MPU_TABLE12 to value 0x01"]
impl crate::Resettable for SHROM_MPU_TABLE12_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
