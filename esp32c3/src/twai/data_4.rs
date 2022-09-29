#[doc = "Register `DATA_4` reader"]
pub struct R(crate::R<DATA_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA_4` writer"]
pub struct W(crate::W<DATA_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_4_SPEC>;
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
impl From<crate::W<DATA_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_BYTE_4` reader - In reset mode, it is acceptance code register 4 with R/W Permission. In operation mode, it stores the 4th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
pub type TX_BYTE_4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_BYTE_4` writer - In reset mode, it is acceptance code register 4 with R/W Permission. In operation mode, it stores the 4th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
pub type TX_BYTE_4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DATA_4_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance code register 4 with R/W Permission. In operation mode, it stores the 4th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
    #[inline(always)]
    pub fn tx_byte_4(&self) -> TX_BYTE_4_R {
        TX_BYTE_4_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance code register 4 with R/W Permission. In operation mode, it stores the 4th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
    #[inline(always)]
    pub fn tx_byte_4(&mut self) -> TX_BYTE_4_W<0> {
        TX_BYTE_4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_4](index.html) module"]
pub struct DATA_4_SPEC;
impl crate::RegisterSpec for DATA_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_4::R](R) reader structure"]
impl crate::Readable for DATA_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data_4::W](W) writer structure"]
impl crate::Writable for DATA_4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA_4 to value 0"]
impl crate::Resettable for DATA_4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
