#[doc = "Register `FRC2_INT` reader"]
pub type R = crate::R<FRC2_INT_SPEC>;
#[doc = "Register `FRC2_INT` writer"]
pub type W = crate::W<FRC2_INT_SPEC>;
#[doc = "Field `frc2_int_clr_mask` reader - write to clear the status of the interrupt, if theinterrupt type is \"level\""]
pub type FRC2_INT_CLR_MASK_R = crate::BitReader;
#[doc = "Field `frc2_int_clr_mask` writer - write to clear the status of the interrupt, if theinterrupt type is \"level\""]
pub type FRC2_INT_CLR_MASK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - write to clear the status of the interrupt, if theinterrupt type is \"level\""]
    #[inline(always)]
    pub fn frc2_int_clr_mask(&self) -> FRC2_INT_CLR_MASK_R {
        FRC2_INT_CLR_MASK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRC2_INT")
            .field(
                "frc2_int_clr_mask",
                &format_args!("{}", self.frc2_int_clr_mask().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FRC2_INT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - write to clear the status of the interrupt, if theinterrupt type is \"level\""]
    #[inline(always)]
    #[must_use]
    pub fn frc2_int_clr_mask(&mut self) -> FRC2_INT_CLR_MASK_W<FRC2_INT_SPEC, 0> {
        FRC2_INT_CLR_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FRC2_INT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frc2_int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frc2_int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRC2_INT_SPEC;
impl crate::RegisterSpec for FRC2_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frc2_int::R`](R) reader structure"]
impl crate::Readable for FRC2_INT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`frc2_int::W`](W) writer structure"]
impl crate::Writable for FRC2_INT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRC2_INT to value 0"]
impl crate::Resettable for FRC2_INT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
