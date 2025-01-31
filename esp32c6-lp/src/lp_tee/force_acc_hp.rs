#[doc = "Register `FORCE_ACC_HP` reader"]
pub type R = crate::R<FORCE_ACC_HP_SPEC>;
#[doc = "Register `FORCE_ACC_HP` writer"]
pub type W = crate::W<FORCE_ACC_HP_SPEC>;
#[doc = "Field `LP_AON_FORCE_ACC_HPMEM_EN` reader - need_des"]
pub type LP_AON_FORCE_ACC_HPMEM_EN_R = crate::BitReader;
#[doc = "Field `LP_AON_FORCE_ACC_HPMEM_EN` writer - need_des"]
pub type LP_AON_FORCE_ACC_HPMEM_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn lp_aon_force_acc_hpmem_en(&self) -> LP_AON_FORCE_ACC_HPMEM_EN_R {
        LP_AON_FORCE_ACC_HPMEM_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FORCE_ACC_HP")
            .field(
                "lp_aon_force_acc_hpmem_en",
                &format_args!("{}", self.lp_aon_force_acc_hpmem_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FORCE_ACC_HP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_aon_force_acc_hpmem_en(
        &mut self,
    ) -> LP_AON_FORCE_ACC_HPMEM_EN_W<FORCE_ACC_HP_SPEC, 0> {
        LP_AON_FORCE_ACC_HPMEM_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`force_acc_hp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`force_acc_hp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FORCE_ACC_HP_SPEC;
impl crate::RegisterSpec for FORCE_ACC_HP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`force_acc_hp::R`](R) reader structure"]
impl crate::Readable for FORCE_ACC_HP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`force_acc_hp::W`](W) writer structure"]
impl crate::Writable for FORCE_ACC_HP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FORCE_ACC_HP to value 0"]
impl crate::Resettable for FORCE_ACC_HP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
