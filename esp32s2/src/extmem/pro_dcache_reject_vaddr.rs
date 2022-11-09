#[doc = "Register `PRO_DCACHE_REJECT_VADDR` reader"]
pub struct R(crate::R<PRO_DCACHE_REJECT_VADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_DCACHE_REJECT_VADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_DCACHE_REJECT_VADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_DCACHE_REJECT_VADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRO_DCACHE_CPU_VADDR` reader - The bits are used to indicate the virtual address of CPU access dcache when authentication fail."]
pub type PRO_DCACHE_CPU_VADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to indicate the virtual address of CPU access dcache when authentication fail."]
    #[inline(always)]
    pub fn pro_dcache_cpu_vaddr(&self) -> PRO_DCACHE_CPU_VADDR_R {
        PRO_DCACHE_CPU_VADDR_R::new(self.bits)
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_dcache_reject_vaddr](index.html) module"]
pub struct PRO_DCACHE_REJECT_VADDR_SPEC;
impl crate::RegisterSpec for PRO_DCACHE_REJECT_VADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_dcache_reject_vaddr::R](R) reader structure"]
impl crate::Readable for PRO_DCACHE_REJECT_VADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRO_DCACHE_REJECT_VADDR to value 0"]
impl crate::Resettable for PRO_DCACHE_REJECT_VADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
