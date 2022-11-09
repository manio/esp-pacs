#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_1` reader"]
pub struct R(crate::R<CORE_0_PIF_PMS_CONSTRAIN_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_PIF_PMS_CONSTRAIN_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_PIF_PMS_CONSTRAIN_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_PIF_PMS_CONSTRAIN_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_PIF_PMS_CONSTRAIN_1` writer"]
pub struct W(crate::W<CORE_0_PIF_PMS_CONSTRAIN_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_PIF_PMS_CONSTRAIN_1_SPEC>;
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
impl From<crate::W<CORE_0_PIF_PMS_CONSTRAIN_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_PIF_PMS_CONSTRAIN_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART` reader - Core0 access uart permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART` writer - Core0 access uart permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_1_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_1` reader - Core0 access g0spi_1 permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_1` writer - Core0 access g0spi_1 permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_1_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_0` reader - Core0 access g0spi_0 permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_0` writer - Core0 access g0spi_0 permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_1_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_GPIO` reader - Core0 access gpio permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_GPIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_GPIO` writer - Core0 access gpio permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_GPIO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_1_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE2` reader - Core0 access fe2 permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE2` writer - Core0 access fe2 permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_1_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE` reader - Core0 access fe permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE` writer - Core0 access fe permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_1_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RTC` reader - Core0 access rtc permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RTC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RTC` writer - Core0 access rtc permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RTC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_1_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_IO_MUX` reader - Core0 access io_mux permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_IO_MUX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_IO_MUX` writer - Core0 access io_mux permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_IO_MUX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_1_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_HINF` reader - Core0 access hinf permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_HINF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_HINF` writer - Core0 access hinf permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_HINF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_1_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_MISC` reader - Core0 access misc permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_MISC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_MISC` writer - Core0 access misc permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_MISC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_1_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C` reader - Core0 access i2c permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C` writer - Core0 access i2c permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_1_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2S0` reader - Core0 access i2s0 permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2S0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2S0` writer - Core0 access i2s0 permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2S0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_1_SPEC, u8, u8, 2, O>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART1` reader - Core0 access uart1 permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART1` writer - Core0 access uart1 permission in world0."]
pub type CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CORE_0_PIF_PMS_CONSTRAIN_1_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Core0 access uart permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_uart(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Core0 access g0spi_1 permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_g0spi_1(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_1_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Core0 access g0spi_0 permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_g0spi_0(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_0_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_0_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Core0 access gpio permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_gpio(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_GPIO_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_GPIO_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Core0 access fe2 permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_fe2(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE2_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE2_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Core0 access fe permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_fe(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Core0 access rtc permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_rtc(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RTC_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RTC_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Core0 access io_mux permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_io_mux(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_IO_MUX_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_IO_MUX_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Core0 access hinf permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_hinf(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_HINF_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_HINF_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Core0 access misc permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_misc(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_MISC_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_MISC_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Core0 access i2c permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_i2c(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Core0 access i2s0 permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_i2s0(&self) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2S0_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2S0_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Core0 access uart1 permission in world0."]
    #[inline(always)]
    pub fn core_0_pif_pms_constrain_world_0_uart1(
        &self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART1_R {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART1_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Core0 access uart permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_uart(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART_W<0> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART_W::new(self)
    }
    #[doc = "Bits 2:3 - Core0 access g0spi_1 permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_g0spi_1(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_1_W<2> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_1_W::new(self)
    }
    #[doc = "Bits 4:5 - Core0 access g0spi_0 permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_g0spi_0(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_0_W<4> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_G0SPI_0_W::new(self)
    }
    #[doc = "Bits 6:7 - Core0 access gpio permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_gpio(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_GPIO_W<6> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_GPIO_W::new(self)
    }
    #[doc = "Bits 8:9 - Core0 access fe2 permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_fe2(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE2_W<8> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE2_W::new(self)
    }
    #[doc = "Bits 10:11 - Core0 access fe permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_fe(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE_W<10> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_FE_W::new(self)
    }
    #[doc = "Bits 14:15 - Core0 access rtc permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_rtc(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RTC_W<14> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_RTC_W::new(self)
    }
    #[doc = "Bits 16:17 - Core0 access io_mux permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_io_mux(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_IO_MUX_W<16> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_IO_MUX_W::new(self)
    }
    #[doc = "Bits 20:21 - Core0 access hinf permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_hinf(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_HINF_W<20> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_HINF_W::new(self)
    }
    #[doc = "Bits 24:25 - Core0 access misc permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_misc(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_MISC_W<24> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_MISC_W::new(self)
    }
    #[doc = "Bits 26:27 - Core0 access i2c permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_i2c(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_W<26> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2C_W::new(self)
    }
    #[doc = "Bits 28:29 - Core0 access i2s0 permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_i2s0(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2S0_W<28> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_I2S0_W::new(self)
    }
    #[doc = "Bits 30:31 - Core0 access uart1 permission in world0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_pif_pms_constrain_world_0_uart1(
        &mut self,
    ) -> CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART1_W<30> {
        CORE_0_PIF_PMS_CONSTRAIN_WORLD_0_UART1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core0 access peripherals permission configuration register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_pif_pms_constrain_1](index.html) module"]
pub struct CORE_0_PIF_PMS_CONSTRAIN_1_SPEC;
impl crate::RegisterSpec for CORE_0_PIF_PMS_CONSTRAIN_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_pif_pms_constrain_1::R](R) reader structure"]
impl crate::Readable for CORE_0_PIF_PMS_CONSTRAIN_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_pif_pms_constrain_1::W](W) writer structure"]
impl crate::Writable for CORE_0_PIF_PMS_CONSTRAIN_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_0_PIF_PMS_CONSTRAIN_1 to value 0xff33_cfff"]
impl crate::Resettable for CORE_0_PIF_PMS_CONSTRAIN_1_SPEC {
    const RESET_VALUE: Self::Ux = 0xff33_cfff;
}
