#[doc = "Register `DEVICE_EN` reader"]
pub type R = crate::R<DEVICE_EN_SPEC>;
#[doc = "Register `DEVICE_EN` writer"]
pub type W = crate::W<DEVICE_EN_SPEC>;
#[doc = "Field `LP_I2C_ANA_MAST_I2C_DEVICE_EN` reader - need_des"]
pub type LP_I2C_ANA_MAST_I2C_DEVICE_EN_R = crate::FieldReader<u16>;
#[doc = "Field `LP_I2C_ANA_MAST_I2C_DEVICE_EN` writer - need_des"]
pub type LP_I2C_ANA_MAST_I2C_DEVICE_EN_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - need_des"]
    #[inline(always)]
    pub fn lp_i2c_ana_mast_i2c_device_en(&self) -> LP_I2C_ANA_MAST_I2C_DEVICE_EN_R {
        LP_I2C_ANA_MAST_I2C_DEVICE_EN_R::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVICE_EN")
            .field(
                "lp_i2c_ana_mast_i2c_device_en",
                &format_args!("{}", self.lp_i2c_ana_mast_i2c_device_en().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DEVICE_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:11 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_i2c_ana_mast_i2c_device_en(
        &mut self,
    ) -> LP_I2C_ANA_MAST_I2C_DEVICE_EN_W<DEVICE_EN_SPEC, 0> {
        LP_I2C_ANA_MAST_I2C_DEVICE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`device_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`device_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEVICE_EN_SPEC;
impl crate::RegisterSpec for DEVICE_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`device_en::R`](R) reader structure"]
impl crate::Readable for DEVICE_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`device_en::W`](W) writer structure"]
impl crate::Writable for DEVICE_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEVICE_EN to value 0"]
impl crate::Resettable for DEVICE_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
