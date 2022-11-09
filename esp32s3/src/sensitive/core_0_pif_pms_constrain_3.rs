#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_3` reader"]
pub struct R(crate::R<CORE_0_PIF_PMS_CONSTRAIN_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_PIF_PMS_CONSTRAIN_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_PIF_PMS_CONSTRAIN_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_PIF_PMS_CONSTRAIN_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_3` writer"]
pub struct W(crate::W<CORE_0_PIF_PMS_CONSTRAIN_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_PIF_PMS_CONSTRAIN_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CORE_0_PIF_PMS_CONSTRAIN_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_PIF_PMS_CONSTRAIN_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2` reader - Core0 access spi_2 permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2` writer - Core0 access spi_2 permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_3_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SPI_3` reader - Core0 access spi_3 permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SPI_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SPI_3` writer - Core0 access spi_3 permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SPI_3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_3_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL` reader - Core0 access apb_ctrl permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL` writer - Core0 access apb_ctrl permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_3_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT1` reader - Core0 access i2c_ext1 permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT1` writer - Core0 access i2c_ext1 permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_3_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SDIO_HOST` reader - Core0 access sdio_host permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SDIO_HOST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SDIO_HOST` writer - Core0 access sdio_host permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SDIO_HOST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_3_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CAN` reader - Core0 access can permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CAN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CAN` writer - Core0 access can permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CAN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_3_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWM1` reader - Core0 access pwm1 permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWM1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWM1` writer - Core0 access pwm1 permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWM1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_3_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2S1` reader - Core0 access i2s1 permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2S1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2S1` writer - Core0 access i2s1 permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2S1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_3_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART2` reader - Core0 access uart2 permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART2` writer - Core0 access uart2 permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_3_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RWBT` reader - Core0 access rwbt permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RWBT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RWBT` writer - Core0 access rwbt permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RWBT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_3_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC` reader - Core0 access wifimac permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC` writer - Core0 access wifimac permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_3_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWR` reader - Core0 access pwr permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWR` writer - Core0 access pwr permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_3_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Core0 access spi_2 permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_spi_2(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Core0 access spi_3 permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_spi_3(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SPI_3_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SPI_3_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Core0 access apb_ctrl permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_apb_ctrl(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Core0 access i2c_ext1 permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_i2c_ext1(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT1_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT1_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Core0 access sdio_host permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_sdio_host(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SDIO_HOST_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SDIO_HOST_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Core0 access can permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_can(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CAN_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CAN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Core0 access pwm1 permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_pwm1(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWM1_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWM1_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Core0 access i2s1 permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_i2s1(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2S1_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2S1_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Core0 access uart2 permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_uart2(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART2_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART2_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Core0 access rwbt permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_rwbt(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RWBT_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RWBT_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Core0 access wifimac permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_wifimac(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Core0 access pwr permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_pwr(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWR_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWR_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Core0 access spi_2 permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_spi_2(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2_W<0> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SPI_2_W::new(self)
    }
    #[doc = "Bits 2:3 - Core0 access spi_3 permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_spi_3(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SPI_3_W<2> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SPI_3_W::new(self)
    }
    #[doc = "Bits 4:5 - Core0 access apb_ctrl permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_apb_ctrl(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL_W<4> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_APB_CTRL_W::new(self)
    }
    #[doc = "Bits 6:7 - Core0 access i2c_ext1 permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_i2c_ext1(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT1_W<6> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_EXT1_W::new(self)
    }
    #[doc = "Bits 8:9 - Core0 access sdio_host permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_sdio_host(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SDIO_HOST_W<8> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_SDIO_HOST_W::new(self)
    }
    #[doc = "Bits 10:11 - Core0 access can permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_can(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CAN_W<10> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_CAN_W::new(self)
    }
    #[doc = "Bits 12:13 - Core0 access pwm1 permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_pwm1(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWM1_W<12> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWM1_W::new(self)
    }
    #[doc = "Bits 14:15 - Core0 access i2s1 permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_i2s1(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2S1_W<14> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2S1_W::new(self)
    }
    #[doc = "Bits 16:17 - Core0 access uart2 permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_uart2(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART2_W<16> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART2_W::new(self)
    }
    #[doc = "Bits 22:23 - Core0 access rwbt permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_rwbt(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RWBT_W<22> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RWBT_W::new(self)
    }
    #[doc = "Bits 26:27 - Core0 access wifimac permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_wifimac(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC_W<26> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_WIFIMAC_W::new(self)
    }
    #[doc = "Bits 28:29 - Core0 access pwr permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_pwr(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWR_W<28> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_PWR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core0 access peripherals permission configuration register 3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_pif_pms_constrain_3](index.html) module"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_3_SPEC;
impl crate::RegisterSpec for CORE_0_PIF_PMS_CONSTRAIN_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_pif_pms_constrain_3::R](R) reader structure"]
impl crate::Readable for CORE_0_PIF_PMS_CONSTRAIN_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_pif_pms_constrain_3::W](W) writer structure"]
impl crate::Writable for CORE_0_PIF_PMS_CONSTRAIN_3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_0_PIF_PMS_CONSTRAIN_3 to value 0x3cc3_ffff"]
impl crate::Resettable for CORE_0_PIF_PMS_CONSTRAIN_3_SPEC {
    const RESET_VALUE: Self::Ux = 0x3cc3_ffff;
}
