#[doc = "Register `POWER_WAIT_TIMER1` reader"]
pub type R = crate::R<POWER_WAIT_TIMER1_SPEC>;
#[doc = "Register `POWER_WAIT_TIMER1` writer"]
pub type W = crate::W<POWER_WAIT_TIMER1_SPEC>;
#[doc = "Field `DG_LP_POWERDOWN_TIMER` reader - need_des"]
pub type DG_LP_POWERDOWN_TIMER_R = crate::FieldReader;
#[doc = "Field `DG_LP_POWERDOWN_TIMER` writer - need_des"]
pub type DG_LP_POWERDOWN_TIMER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `DG_LP_POWERUP_TIMER` reader - need_des"]
pub type DG_LP_POWERUP_TIMER_R = crate::FieldReader;
#[doc = "Field `DG_LP_POWERUP_TIMER` writer - need_des"]
pub type DG_LP_POWERUP_TIMER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `DG_LP_WAIT_TIMER` reader - need_des"]
pub type DG_LP_WAIT_TIMER_R = crate::FieldReader<u16>;
#[doc = "Field `DG_LP_WAIT_TIMER` writer - need_des"]
pub type DG_LP_WAIT_TIMER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
impl R {
    #[doc = "Bits 9:15 - need_des"]
    #[inline(always)]
    pub fn dg_lp_powerdown_timer(&self) -> DG_LP_POWERDOWN_TIMER_R {
        DG_LP_POWERDOWN_TIMER_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - need_des"]
    #[inline(always)]
    pub fn dg_lp_powerup_timer(&self) -> DG_LP_POWERUP_TIMER_R {
        DG_LP_POWERUP_TIMER_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 23:31 - need_des"]
    #[inline(always)]
    pub fn dg_lp_wait_timer(&self) -> DG_LP_WAIT_TIMER_R {
        DG_LP_WAIT_TIMER_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWER_WAIT_TIMER1")
            .field(
                "dg_lp_powerdown_timer",
                &format_args!("{}", self.dg_lp_powerdown_timer().bits()),
            )
            .field(
                "dg_lp_powerup_timer",
                &format_args!("{}", self.dg_lp_powerup_timer().bits()),
            )
            .field(
                "dg_lp_wait_timer",
                &format_args!("{}", self.dg_lp_wait_timer().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<POWER_WAIT_TIMER1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 9:15 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn dg_lp_powerdown_timer(&mut self) -> DG_LP_POWERDOWN_TIMER_W<POWER_WAIT_TIMER1_SPEC, 9> {
        DG_LP_POWERDOWN_TIMER_W::new(self)
    }
    #[doc = "Bits 16:22 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn dg_lp_powerup_timer(&mut self) -> DG_LP_POWERUP_TIMER_W<POWER_WAIT_TIMER1_SPEC, 16> {
        DG_LP_POWERUP_TIMER_W::new(self)
    }
    #[doc = "Bits 23:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn dg_lp_wait_timer(&mut self) -> DG_LP_WAIT_TIMER_W<POWER_WAIT_TIMER1_SPEC, 23> {
        DG_LP_WAIT_TIMER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`power_wait_timer1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_wait_timer1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POWER_WAIT_TIMER1_SPEC;
impl crate::RegisterSpec for POWER_WAIT_TIMER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_wait_timer1::R`](R) reader structure"]
impl crate::Readable for POWER_WAIT_TIMER1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`power_wait_timer1::W`](W) writer structure"]
impl crate::Writable for POWER_WAIT_TIMER1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POWER_WAIT_TIMER1 to value 0x7fff_fe00"]
impl crate::Resettable for POWER_WAIT_TIMER1_SPEC {
    const RESET_VALUE: Self::Ux = 0x7fff_fe00;
}
