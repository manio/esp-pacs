#[doc = "Register `IN_EOF_DES_ADDR` reader"]
pub struct R(crate::R<IN_EOF_DES_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_EOF_DES_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_EOF_DES_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_EOF_DES_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IN_SUC_EOF_DES_ADDR` reader - The address of inlink descriptor that produces EOF."]
pub type IN_SUC_EOF_DES_ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The address of inlink descriptor that produces EOF."]
    #[inline(always)]
    pub fn in_suc_eof_des_addr(&self) -> IN_SUC_EOF_DES_ADDR_R {
        IN_SUC_EOF_DES_ADDR_R::new(self.bits)
    }
}
#[doc = "Address of inlink descriptor that produces EOF\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_eof_des_addr](index.html) module"]
pub struct IN_EOF_DES_ADDR_SPEC;
impl crate::RegisterSpec for IN_EOF_DES_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_eof_des_addr::R](R) reader structure"]
impl crate::Readable for IN_EOF_DES_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IN_EOF_DES_ADDR to value 0"]
impl crate::Resettable for IN_EOF_DES_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
