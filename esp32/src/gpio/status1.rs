#[doc = "Register `STATUS1` reader"]
pub struct R(crate::R<STATUS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS1` writer"]
pub struct W(crate::W<STATUS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS1_SPEC>;
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
impl From<crate::W<STATUS1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT` reader - GPIO32~39 interrupt status"]
pub type INT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT` writer - GPIO32~39 interrupt status"]
pub type INT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STATUS1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - GPIO32~39 interrupt status"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO32~39 interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> INT_W<0> {
        INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status1](index.html) module"]
pub struct STATUS1_SPEC;
impl crate::RegisterSpec for STATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status1::R](R) reader structure"]
impl crate::Readable for STATUS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status1::W](W) writer structure"]
impl crate::Writable for STATUS1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS1 to value 0"]
impl crate::Resettable for STATUS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
