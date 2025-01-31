#[doc = "Register `HOST_SLCHOST_CONF_W11` reader"]
pub type R = crate::R<HOST_SLCHOST_CONF_W11_SPEC>;
#[doc = "Register `HOST_SLCHOST_CONF_W11` writer"]
pub type W = crate::W<HOST_SLCHOST_CONF_W11_SPEC>;
#[doc = "Field `HOST_SLCHOST_CONF44` reader - "]
pub type HOST_SLCHOST_CONF44_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF44` writer - "]
pub type HOST_SLCHOST_CONF44_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `HOST_SLCHOST_CONF45` reader - "]
pub type HOST_SLCHOST_CONF45_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF45` writer - "]
pub type HOST_SLCHOST_CONF45_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `HOST_SLCHOST_CONF46` reader - "]
pub type HOST_SLCHOST_CONF46_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF46` writer - "]
pub type HOST_SLCHOST_CONF46_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `HOST_SLCHOST_CONF47` reader - "]
pub type HOST_SLCHOST_CONF47_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF47` writer - "]
pub type HOST_SLCHOST_CONF47_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf44(&self) -> HOST_SLCHOST_CONF44_R {
        HOST_SLCHOST_CONF44_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf45(&self) -> HOST_SLCHOST_CONF45_R {
        HOST_SLCHOST_CONF45_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf46(&self) -> HOST_SLCHOST_CONF46_R {
        HOST_SLCHOST_CONF46_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf47(&self) -> HOST_SLCHOST_CONF47_R {
        HOST_SLCHOST_CONF47_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_CONF_W11")
            .field(
                "host_slchost_conf44",
                &format_args!("{}", self.host_slchost_conf44().bits()),
            )
            .field(
                "host_slchost_conf45",
                &format_args!("{}", self.host_slchost_conf45().bits()),
            )
            .field(
                "host_slchost_conf46",
                &format_args!("{}", self.host_slchost_conf46().bits()),
            )
            .field(
                "host_slchost_conf47",
                &format_args!("{}", self.host_slchost_conf47().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLCHOST_CONF_W11_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf44(&mut self) -> HOST_SLCHOST_CONF44_W<HOST_SLCHOST_CONF_W11_SPEC, 0> {
        HOST_SLCHOST_CONF44_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf45(&mut self) -> HOST_SLCHOST_CONF45_W<HOST_SLCHOST_CONF_W11_SPEC, 8> {
        HOST_SLCHOST_CONF45_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf46(&mut self) -> HOST_SLCHOST_CONF46_W<HOST_SLCHOST_CONF_W11_SPEC, 16> {
        HOST_SLCHOST_CONF46_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf47(&mut self) -> HOST_SLCHOST_CONF47_W<HOST_SLCHOST_CONF_W11_SPEC, 24> {
        HOST_SLCHOST_CONF47_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_conf_w11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_conf_w11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLCHOST_CONF_W11_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_CONF_W11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_slchost_conf_w11::R`](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W11_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_slchost_conf_w11::W`](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W11_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_SLCHOST_CONF_W11 to value 0"]
impl crate::Resettable for HOST_SLCHOST_CONF_W11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
