#[doc = "Register `PRO_DCACHE_DBUG7` reader"]
pub struct R(crate::R<PRO_DCACHE_DBUG7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_DCACHE_DBUG7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_DCACHE_DBUG7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_DCACHE_DBUG7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRO_IRAM1ADDR_IA` reader - "]
pub type PRO_IRAM1ADDR_IA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn pro_iram1addr_ia(&self) -> PRO_IRAM1ADDR_IA_R {
        PRO_IRAM1ADDR_IA_R::new(self.bits & 0x000f_ffff)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_dcache_dbug7](index.html) module"]
pub struct PRO_DCACHE_DBUG7_SPEC;
impl crate::RegisterSpec for PRO_DCACHE_DBUG7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_dcache_dbug7::R](R) reader structure"]
impl crate::Readable for PRO_DCACHE_DBUG7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRO_DCACHE_DBUG7 to value 0"]
impl crate::Resettable for PRO_DCACHE_DBUG7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
