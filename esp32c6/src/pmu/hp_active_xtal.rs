#[doc = "Register `HP_ACTIVE_XTAL` reader"]
pub type R = crate::R<HP_ACTIVE_XTAL_SPEC>;
#[doc = "Register `HP_ACTIVE_XTAL` writer"]
pub type W = crate::W<HP_ACTIVE_XTAL_SPEC>;
#[doc = "Field `HP_ACTIVE_XPD_XTAL` reader - need_des"]
pub type HP_ACTIVE_XPD_XTAL_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_XPD_XTAL` writer - need_des"]
pub type HP_ACTIVE_XPD_XTAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn hp_active_xpd_xtal(&self) -> HP_ACTIVE_XPD_XTAL_R {
        HP_ACTIVE_XPD_XTAL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_ACTIVE_XTAL")
            .field(
                "hp_active_xpd_xtal",
                &format_args!("{}", self.hp_active_xpd_xtal().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_ACTIVE_XTAL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_active_xpd_xtal(&mut self) -> HP_ACTIVE_XPD_XTAL_W<HP_ACTIVE_XTAL_SPEC, 31> {
        HP_ACTIVE_XPD_XTAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_active_xtal::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_active_xtal::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_ACTIVE_XTAL_SPEC;
impl crate::RegisterSpec for HP_ACTIVE_XTAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_active_xtal::R`](R) reader structure"]
impl crate::Readable for HP_ACTIVE_XTAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_active_xtal::W`](W) writer structure"]
impl crate::Writable for HP_ACTIVE_XTAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_ACTIVE_XTAL to value 0x8000_0000"]
impl crate::Resettable for HP_ACTIVE_XTAL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
