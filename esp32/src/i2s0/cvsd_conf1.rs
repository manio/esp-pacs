#[doc = "Register `CVSD_CONF1` reader"]
pub type R = crate::R<CVSD_CONF1_SPEC>;
#[doc = "Register `CVSD_CONF1` writer"]
pub type W = crate::W<CVSD_CONF1_SPEC>;
#[doc = "Field `CVSD_SIGMA_MAX` reader - "]
pub type CVSD_SIGMA_MAX_R = crate::FieldReader<u16>;
#[doc = "Field `CVSD_SIGMA_MAX` writer - "]
pub type CVSD_SIGMA_MAX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `CVSD_SIGMA_MIN` reader - "]
pub type CVSD_SIGMA_MIN_R = crate::FieldReader<u16>;
#[doc = "Field `CVSD_SIGMA_MIN` writer - "]
pub type CVSD_SIGMA_MIN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cvsd_sigma_max(&self) -> CVSD_SIGMA_MAX_R {
        CVSD_SIGMA_MAX_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cvsd_sigma_min(&self) -> CVSD_SIGMA_MIN_R {
        CVSD_SIGMA_MIN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CVSD_CONF1")
            .field(
                "cvsd_sigma_max",
                &format_args!("{}", self.cvsd_sigma_max().bits()),
            )
            .field(
                "cvsd_sigma_min",
                &format_args!("{}", self.cvsd_sigma_min().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CVSD_CONF1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn cvsd_sigma_max(&mut self) -> CVSD_SIGMA_MAX_W<CVSD_CONF1_SPEC, 0> {
        CVSD_SIGMA_MAX_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn cvsd_sigma_min(&mut self) -> CVSD_SIGMA_MIN_W<CVSD_CONF1_SPEC, 16> {
        CVSD_SIGMA_MIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cvsd_conf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cvsd_conf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CVSD_CONF1_SPEC;
impl crate::RegisterSpec for CVSD_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cvsd_conf1::R`](R) reader structure"]
impl crate::Readable for CVSD_CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cvsd_conf1::W`](W) writer structure"]
impl crate::Writable for CVSD_CONF1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CVSD_CONF1 to value 0x000a_0500"]
impl crate::Resettable for CVSD_CONF1_SPEC {
    const RESET_VALUE: Self::Ux = 0x000a_0500;
}
