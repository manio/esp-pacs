#[doc = "Register `AHB_FREQ_CONF` reader"]
pub type R = crate::R<AHB_FREQ_CONF_SPEC>;
#[doc = "Register `AHB_FREQ_CONF` writer"]
pub type W = crate::W<AHB_FREQ_CONF_SPEC>;
#[doc = "Field `AHB_DIV_NUM` reader - Set this field to generate clk_ahb drived by clk_hproot. The clk_ahb is div1(default)/div2/div4/div8 of clk_hproot. This field is only avaliable for low-speed clock-source such as XTAL/FOSC, and should be used together with PCR_CPU_DIV_NUM."]
pub type AHB_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `AHB_DIV_NUM` writer - Set this field to generate clk_ahb drived by clk_hproot. The clk_ahb is div1(default)/div2/div4/div8 of clk_hproot. This field is only avaliable for low-speed clock-source such as XTAL/FOSC, and should be used together with PCR_CPU_DIV_NUM."]
pub type AHB_DIV_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Set this field to generate clk_ahb drived by clk_hproot. The clk_ahb is div1(default)/div2/div4/div8 of clk_hproot. This field is only avaliable for low-speed clock-source such as XTAL/FOSC, and should be used together with PCR_CPU_DIV_NUM."]
    #[inline(always)]
    pub fn ahb_div_num(&self) -> AHB_DIV_NUM_R {
        AHB_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_FREQ_CONF")
            .field(
                "ahb_div_num",
                &format_args!("{}", self.ahb_div_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AHB_FREQ_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Set this field to generate clk_ahb drived by clk_hproot. The clk_ahb is div1(default)/div2/div4/div8 of clk_hproot. This field is only avaliable for low-speed clock-source such as XTAL/FOSC, and should be used together with PCR_CPU_DIV_NUM."]
    #[inline(always)]
    #[must_use]
    pub fn ahb_div_num(&mut self) -> AHB_DIV_NUM_W<AHB_FREQ_CONF_SPEC, 0> {
        AHB_DIV_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AHB_FREQ configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_freq_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_freq_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB_FREQ_CONF_SPEC;
impl crate::RegisterSpec for AHB_FREQ_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_freq_conf::R`](R) reader structure"]
impl crate::Readable for AHB_FREQ_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb_freq_conf::W`](W) writer structure"]
impl crate::Writable for AHB_FREQ_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB_FREQ_CONF to value 0"]
impl crate::Resettable for AHB_FREQ_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
