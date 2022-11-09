#[doc = "Register `USB_CONF` reader"]
pub struct R(crate::R<USB_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_CONF` writer"]
pub struct W(crate::W<USB_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_CONF_SPEC>;
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
impl From<crate::W<USB_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_VREFH` reader - "]
pub type USB_VREFH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_VREFH` writer - "]
pub type USB_VREFH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USB_CONF_SPEC, u8, u8, 2, O>;
#[doc = "Field `USB_VREFL` reader - "]
pub type USB_VREFL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USB_VREFL` writer - "]
pub type USB_VREFL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USB_CONF_SPEC, u8, u8, 2, O>;
#[doc = "Field `USB_VREF_OVERRIDE` reader - "]
pub type USB_VREF_OVERRIDE_R = crate::BitReader<bool>;
#[doc = "Field `USB_VREF_OVERRIDE` writer - "]
pub type USB_VREF_OVERRIDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CONF_SPEC, bool, O>;
#[doc = "Field `USB_PAD_PULL_OVERRIDE` reader - "]
pub type USB_PAD_PULL_OVERRIDE_R = crate::BitReader<bool>;
#[doc = "Field `USB_PAD_PULL_OVERRIDE` writer - "]
pub type USB_PAD_PULL_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB_CONF_SPEC, bool, O>;
#[doc = "Field `USB_DP_PULLUP` reader - "]
pub type USB_DP_PULLUP_R = crate::BitReader<bool>;
#[doc = "Field `USB_DP_PULLUP` writer - "]
pub type USB_DP_PULLUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CONF_SPEC, bool, O>;
#[doc = "Field `USB_DP_PULLDOWN` reader - "]
pub type USB_DP_PULLDOWN_R = crate::BitReader<bool>;
#[doc = "Field `USB_DP_PULLDOWN` writer - "]
pub type USB_DP_PULLDOWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CONF_SPEC, bool, O>;
#[doc = "Field `USB_DM_PULLUP` reader - "]
pub type USB_DM_PULLUP_R = crate::BitReader<bool>;
#[doc = "Field `USB_DM_PULLUP` writer - "]
pub type USB_DM_PULLUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CONF_SPEC, bool, O>;
#[doc = "Field `USB_DM_PULLDOWN` reader - "]
pub type USB_DM_PULLDOWN_R = crate::BitReader<bool>;
#[doc = "Field `USB_DM_PULLDOWN` writer - "]
pub type USB_DM_PULLDOWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CONF_SPEC, bool, O>;
#[doc = "Field `USB_PULLUP_VALUE` reader - "]
pub type USB_PULLUP_VALUE_R = crate::BitReader<bool>;
#[doc = "Field `USB_PULLUP_VALUE` writer - "]
pub type USB_PULLUP_VALUE_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CONF_SPEC, bool, O>;
#[doc = "Field `USB_PAD_ENABLE_OVERRIDE` reader - "]
pub type USB_PAD_ENABLE_OVERRIDE_R = crate::BitReader<bool>;
#[doc = "Field `USB_PAD_ENABLE_OVERRIDE` writer - "]
pub type USB_PAD_ENABLE_OVERRIDE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB_CONF_SPEC, bool, O>;
#[doc = "Field `USB_PAD_ENABLE` reader - "]
pub type USB_PAD_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `USB_PAD_ENABLE` writer - "]
pub type USB_PAD_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CONF_SPEC, bool, O>;
#[doc = "Field `USB_TXM` reader - "]
pub type USB_TXM_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXM` writer - "]
pub type USB_TXM_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CONF_SPEC, bool, O>;
#[doc = "Field `USB_TXP` reader - "]
pub type USB_TXP_R = crate::BitReader<bool>;
#[doc = "Field `USB_TXP` writer - "]
pub type USB_TXP_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CONF_SPEC, bool, O>;
#[doc = "Field `USB_TX_EN` reader - "]
pub type USB_TX_EN_R = crate::BitReader<bool>;
#[doc = "Field `USB_TX_EN` writer - "]
pub type USB_TX_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CONF_SPEC, bool, O>;
#[doc = "Field `USB_TX_EN_OVERRIDE` reader - "]
pub type USB_TX_EN_OVERRIDE_R = crate::BitReader<bool>;
#[doc = "Field `USB_TX_EN_OVERRIDE` writer - "]
pub type USB_TX_EN_OVERRIDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CONF_SPEC, bool, O>;
#[doc = "Field `USB_RESET_DISABLE` reader - "]
pub type USB_RESET_DISABLE_R = crate::BitReader<bool>;
#[doc = "Field `USB_RESET_DISABLE` writer - "]
pub type USB_RESET_DISABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CONF_SPEC, bool, O>;
#[doc = "Field `IO_MUX_RESET_DISABLE` reader - "]
pub type IO_MUX_RESET_DISABLE_R = crate::BitReader<bool>;
#[doc = "Field `IO_MUX_RESET_DISABLE` writer - "]
pub type IO_MUX_RESET_DISABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB_CONF_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn usb_vrefh(&self) -> USB_VREFH_R {
        USB_VREFH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn usb_vrefl(&self) -> USB_VREFL_R {
        USB_VREFL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn usb_vref_override(&self) -> USB_VREF_OVERRIDE_R {
        USB_VREF_OVERRIDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn usb_pad_pull_override(&self) -> USB_PAD_PULL_OVERRIDE_R {
        USB_PAD_PULL_OVERRIDE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn usb_dp_pullup(&self) -> USB_DP_PULLUP_R {
        USB_DP_PULLUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn usb_dp_pulldown(&self) -> USB_DP_PULLDOWN_R {
        USB_DP_PULLDOWN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn usb_dm_pullup(&self) -> USB_DM_PULLUP_R {
        USB_DM_PULLUP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn usb_dm_pulldown(&self) -> USB_DM_PULLDOWN_R {
        USB_DM_PULLDOWN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn usb_pullup_value(&self) -> USB_PULLUP_VALUE_R {
        USB_PULLUP_VALUE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn usb_pad_enable_override(&self) -> USB_PAD_ENABLE_OVERRIDE_R {
        USB_PAD_ENABLE_OVERRIDE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn usb_pad_enable(&self) -> USB_PAD_ENABLE_R {
        USB_PAD_ENABLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn usb_txm(&self) -> USB_TXM_R {
        USB_TXM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn usb_txp(&self) -> USB_TXP_R {
        USB_TXP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn usb_tx_en(&self) -> USB_TX_EN_R {
        USB_TX_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn usb_tx_en_override(&self) -> USB_TX_EN_OVERRIDE_R {
        USB_TX_EN_OVERRIDE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn usb_reset_disable(&self) -> USB_RESET_DISABLE_R {
        USB_RESET_DISABLE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn io_mux_reset_disable(&self) -> IO_MUX_RESET_DISABLE_R {
        IO_MUX_RESET_DISABLE_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn usb_vrefh(&mut self) -> USB_VREFH_W<0> {
        USB_VREFH_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn usb_vrefl(&mut self) -> USB_VREFL_W<2> {
        USB_VREFL_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn usb_vref_override(&mut self) -> USB_VREF_OVERRIDE_W<4> {
        USB_VREF_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn usb_pad_pull_override(&mut self) -> USB_PAD_PULL_OVERRIDE_W<5> {
        USB_PAD_PULL_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn usb_dp_pullup(&mut self) -> USB_DP_PULLUP_W<6> {
        USB_DP_PULLUP_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn usb_dp_pulldown(&mut self) -> USB_DP_PULLDOWN_W<7> {
        USB_DP_PULLDOWN_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn usb_dm_pullup(&mut self) -> USB_DM_PULLUP_W<8> {
        USB_DM_PULLUP_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn usb_dm_pulldown(&mut self) -> USB_DM_PULLDOWN_W<9> {
        USB_DM_PULLDOWN_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn usb_pullup_value(&mut self) -> USB_PULLUP_VALUE_W<10> {
        USB_PULLUP_VALUE_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn usb_pad_enable_override(&mut self) -> USB_PAD_ENABLE_OVERRIDE_W<11> {
        USB_PAD_ENABLE_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn usb_pad_enable(&mut self) -> USB_PAD_ENABLE_W<12> {
        USB_PAD_ENABLE_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txm(&mut self) -> USB_TXM_W<13> {
        USB_TXM_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn usb_txp(&mut self) -> USB_TXP_W<14> {
        USB_TXP_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn usb_tx_en(&mut self) -> USB_TX_EN_W<15> {
        USB_TX_EN_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn usb_tx_en_override(&mut self) -> USB_TX_EN_OVERRIDE_W<16> {
        USB_TX_EN_OVERRIDE_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn usb_reset_disable(&mut self) -> USB_RESET_DISABLE_W<17> {
        USB_RESET_DISABLE_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn io_mux_reset_disable(&mut self) -> IO_MUX_RESET_DISABLE_W<18> {
        IO_MUX_RESET_DISABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure usb control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_conf](index.html) module"]
pub struct USB_CONF_SPEC;
impl crate::RegisterSpec for USB_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_conf::R](R) reader structure"]
impl crate::Readable for USB_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_conf::W](W) writer structure"]
impl crate::Writable for USB_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB_CONF to value 0"]
impl crate::Resettable for USB_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
