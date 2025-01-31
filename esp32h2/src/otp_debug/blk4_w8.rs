#[doc = "Register `BLK4_W8` reader"]
pub type R = crate::R<BLK4_W8_SPEC>;
#[doc = "Field `BLOCK4_W8` reader - Otp block4 word8 data."]
pub type BLOCK4_W8_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block4 word8 data."]
    #[inline(always)]
    pub fn block4_w8(&self) -> BLOCK4_W8_R {
        BLOCK4_W8_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK4_W8")
            .field("block4_w8", &format_args!("{}", self.block4_w8().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK4_W8_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Otp debuger block4 data register8.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk4_w8::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK4_W8_SPEC;
impl crate::RegisterSpec for BLK4_W8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk4_w8::R`](R) reader structure"]
impl crate::Readable for BLK4_W8_SPEC {}
#[doc = "`reset()` method sets BLK4_W8 to value 0"]
impl crate::Resettable for BLK4_W8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
