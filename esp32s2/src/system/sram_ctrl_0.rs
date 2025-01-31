#[doc = "Register `SRAM_CTRL_0` reader"]
pub type R = crate::R<SRAM_CTRL_0_SPEC>;
#[doc = "Register `SRAM_CTRL_0` writer"]
pub type W = crate::W<SRAM_CTRL_0_SPEC>;
#[doc = "Field `SRAM_FO` reader - This field is used to force on clock gate of internal SRAM."]
pub type SRAM_FO_R = crate::FieldReader<u32>;
#[doc = "Field `SRAM_FO` writer - This field is used to force on clock gate of internal SRAM."]
pub type SRAM_FO_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 22, O, u32>;
impl R {
    #[doc = "Bits 0:21 - This field is used to force on clock gate of internal SRAM."]
    #[inline(always)]
    pub fn sram_fo(&self) -> SRAM_FO_R {
        SRAM_FO_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_CTRL_0")
            .field("sram_fo", &format_args!("{}", self.sram_fo().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SRAM_CTRL_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:21 - This field is used to force on clock gate of internal SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn sram_fo(&mut self) -> SRAM_FO_W<SRAM_CTRL_0_SPEC, 0> {
        SRAM_FO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "System SRAM configuration register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_ctrl_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ctrl_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAM_CTRL_0_SPEC;
impl crate::RegisterSpec for SRAM_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_ctrl_0::R`](R) reader structure"]
impl crate::Readable for SRAM_CTRL_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sram_ctrl_0::W`](W) writer structure"]
impl crate::Writable for SRAM_CTRL_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRAM_CTRL_0 to value 0x003f_ffff"]
impl crate::Resettable for SRAM_CTRL_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x003f_ffff;
}
