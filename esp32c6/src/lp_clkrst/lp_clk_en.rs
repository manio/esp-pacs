#[doc = "Register `LP_CLK_EN` reader"]
pub type R = crate::R<LP_CLK_EN_SPEC>;
#[doc = "Register `LP_CLK_EN` writer"]
pub type W = crate::W<LP_CLK_EN_SPEC>;
#[doc = "Field `FAST_ORI_GATE` reader - need_des"]
pub type FAST_ORI_GATE_R = crate::BitReader;
#[doc = "Field `FAST_ORI_GATE` writer - need_des"]
pub type FAST_ORI_GATE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn fast_ori_gate(&self) -> FAST_ORI_GATE_R {
        FAST_ORI_GATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_CLK_EN")
            .field(
                "fast_ori_gate",
                &format_args!("{}", self.fast_ori_gate().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_CLK_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn fast_ori_gate(&mut self) -> FAST_ORI_GATE_W<LP_CLK_EN_SPEC, 31> {
        FAST_ORI_GATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_clk_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_clk_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_CLK_EN_SPEC;
impl crate::RegisterSpec for LP_CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_clk_en::R`](R) reader structure"]
impl crate::Readable for LP_CLK_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_clk_en::W`](W) writer structure"]
impl crate::Writable for LP_CLK_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LP_CLK_EN to value 0"]
impl crate::Resettable for LP_CLK_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
