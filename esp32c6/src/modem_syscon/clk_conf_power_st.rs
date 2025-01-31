#[doc = "Register `CLK_CONF_POWER_ST` reader"]
pub type R = crate::R<CLK_CONF_POWER_ST_SPEC>;
#[doc = "Register `CLK_CONF_POWER_ST` writer"]
pub type W = crate::W<CLK_CONF_POWER_ST_SPEC>;
#[doc = "Field `CLK_ZB_ST_MAP` reader - "]
pub type CLK_ZB_ST_MAP_R = crate::FieldReader;
#[doc = "Field `CLK_ZB_ST_MAP` writer - "]
pub type CLK_ZB_ST_MAP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CLK_FE_ST_MAP` reader - "]
pub type CLK_FE_ST_MAP_R = crate::FieldReader;
#[doc = "Field `CLK_FE_ST_MAP` writer - "]
pub type CLK_FE_ST_MAP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CLK_BT_ST_MAP` reader - "]
pub type CLK_BT_ST_MAP_R = crate::FieldReader;
#[doc = "Field `CLK_BT_ST_MAP` writer - "]
pub type CLK_BT_ST_MAP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CLK_WIFI_ST_MAP` reader - "]
pub type CLK_WIFI_ST_MAP_R = crate::FieldReader;
#[doc = "Field `CLK_WIFI_ST_MAP` writer - "]
pub type CLK_WIFI_ST_MAP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CLK_MODEM_PERI_ST_MAP` reader - "]
pub type CLK_MODEM_PERI_ST_MAP_R = crate::FieldReader;
#[doc = "Field `CLK_MODEM_PERI_ST_MAP` writer - "]
pub type CLK_MODEM_PERI_ST_MAP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CLK_MODEM_APB_ST_MAP` reader - "]
pub type CLK_MODEM_APB_ST_MAP_R = crate::FieldReader;
#[doc = "Field `CLK_MODEM_APB_ST_MAP` writer - "]
pub type CLK_MODEM_APB_ST_MAP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn clk_zb_st_map(&self) -> CLK_ZB_ST_MAP_R {
        CLK_ZB_ST_MAP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn clk_fe_st_map(&self) -> CLK_FE_ST_MAP_R {
        CLK_FE_ST_MAP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn clk_bt_st_map(&self) -> CLK_BT_ST_MAP_R {
        CLK_BT_ST_MAP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn clk_wifi_st_map(&self) -> CLK_WIFI_ST_MAP_R {
        CLK_WIFI_ST_MAP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn clk_modem_peri_st_map(&self) -> CLK_MODEM_PERI_ST_MAP_R {
        CLK_MODEM_PERI_ST_MAP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn clk_modem_apb_st_map(&self) -> CLK_MODEM_APB_ST_MAP_R {
        CLK_MODEM_APB_ST_MAP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_CONF_POWER_ST")
            .field(
                "clk_zb_st_map",
                &format_args!("{}", self.clk_zb_st_map().bits()),
            )
            .field(
                "clk_fe_st_map",
                &format_args!("{}", self.clk_fe_st_map().bits()),
            )
            .field(
                "clk_bt_st_map",
                &format_args!("{}", self.clk_bt_st_map().bits()),
            )
            .field(
                "clk_wifi_st_map",
                &format_args!("{}", self.clk_wifi_st_map().bits()),
            )
            .field(
                "clk_modem_peri_st_map",
                &format_args!("{}", self.clk_modem_peri_st_map().bits()),
            )
            .field(
                "clk_modem_apb_st_map",
                &format_args!("{}", self.clk_modem_apb_st_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLK_CONF_POWER_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn clk_zb_st_map(&mut self) -> CLK_ZB_ST_MAP_W<CLK_CONF_POWER_ST_SPEC, 8> {
        CLK_ZB_ST_MAP_W::new(self)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn clk_fe_st_map(&mut self) -> CLK_FE_ST_MAP_W<CLK_CONF_POWER_ST_SPEC, 12> {
        CLK_FE_ST_MAP_W::new(self)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn clk_bt_st_map(&mut self) -> CLK_BT_ST_MAP_W<CLK_CONF_POWER_ST_SPEC, 16> {
        CLK_BT_ST_MAP_W::new(self)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn clk_wifi_st_map(&mut self) -> CLK_WIFI_ST_MAP_W<CLK_CONF_POWER_ST_SPEC, 20> {
        CLK_WIFI_ST_MAP_W::new(self)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn clk_modem_peri_st_map(&mut self) -> CLK_MODEM_PERI_ST_MAP_W<CLK_CONF_POWER_ST_SPEC, 24> {
        CLK_MODEM_PERI_ST_MAP_W::new(self)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn clk_modem_apb_st_map(&mut self) -> CLK_MODEM_APB_ST_MAP_W<CLK_CONF_POWER_ST_SPEC, 28> {
        CLK_MODEM_APB_ST_MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_conf_power_st::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_conf_power_st::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_CONF_POWER_ST_SPEC;
impl crate::RegisterSpec for CLK_CONF_POWER_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_conf_power_st::R`](R) reader structure"]
impl crate::Readable for CLK_CONF_POWER_ST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_conf_power_st::W`](W) writer structure"]
impl crate::Writable for CLK_CONF_POWER_ST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_CONF_POWER_ST to value 0"]
impl crate::Resettable for CLK_CONF_POWER_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
