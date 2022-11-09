#[doc = "Register `DCACHE_LOCK_SIZE` reader"]
pub struct R(crate::R<DCACHE_LOCK_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCACHE_LOCK_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCACHE_LOCK_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCACHE_LOCK_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCACHE_LOCK_SIZE` writer"]
pub struct W(crate::W<DCACHE_LOCK_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCACHE_LOCK_SIZE_SPEC>;
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
impl From<crate::W<DCACHE_LOCK_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCACHE_LOCK_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCACHE_LOCK_SIZE` reader - The bits are used to configure the length for lock operations. The bits are the counts of cache block. It should be combined with DCACHE_LOCK_ADDR_REG."]
pub type DCACHE_LOCK_SIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DCACHE_LOCK_SIZE` writer - The bits are used to configure the length for lock operations. The bits are the counts of cache block. It should be combined with DCACHE_LOCK_ADDR_REG."]
pub type DCACHE_LOCK_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCACHE_LOCK_SIZE_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - The bits are used to configure the length for lock operations. The bits are the counts of cache block. It should be combined with DCACHE_LOCK_ADDR_REG."]
    #[inline(always)]
    pub fn dcache_lock_size(&self) -> DCACHE_LOCK_SIZE_R {
        DCACHE_LOCK_SIZE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The bits are used to configure the length for lock operations. The bits are the counts of cache block. It should be combined with DCACHE_LOCK_ADDR_REG."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_lock_size(&mut self) -> DCACHE_LOCK_SIZE_W<0> {
        DCACHE_LOCK_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcache_lock_size](index.html) module"]
pub struct DCACHE_LOCK_SIZE_SPEC;
impl crate::RegisterSpec for DCACHE_LOCK_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcache_lock_size::R](R) reader structure"]
impl crate::Readable for DCACHE_LOCK_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcache_lock_size::W](W) writer structure"]
impl crate::Writable for DCACHE_LOCK_SIZE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCACHE_LOCK_SIZE to value 0"]
impl crate::Resettable for DCACHE_LOCK_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
