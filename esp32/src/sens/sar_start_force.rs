#[doc = "Register `SAR_START_FORCE` reader"]
pub type R = crate::R<SAR_START_FORCE_SPEC>;
#[doc = "Register `SAR_START_FORCE` writer"]
pub type W = crate::W<SAR_START_FORCE_SPEC>;
#[doc = "Field `SAR1_BIT_WIDTH` reader - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
pub type SAR1_BIT_WIDTH_R = crate::FieldReader;
#[doc = "Field `SAR1_BIT_WIDTH` writer - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
pub type SAR1_BIT_WIDTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SAR2_BIT_WIDTH` reader - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
pub type SAR2_BIT_WIDTH_R = crate::FieldReader;
#[doc = "Field `SAR2_BIT_WIDTH` writer - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
pub type SAR2_BIT_WIDTH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SAR2_EN_TEST` reader - SAR2_EN_TEST only active when reg_sar2_dig_force = 0"]
pub type SAR2_EN_TEST_R = crate::BitReader;
#[doc = "Field `SAR2_EN_TEST` writer - SAR2_EN_TEST only active when reg_sar2_dig_force = 0"]
pub type SAR2_EN_TEST_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SAR2_PWDET_CCT` reader - SAR2_PWDET_CCT PA power detector capacitance tuning."]
pub type SAR2_PWDET_CCT_R = crate::FieldReader;
#[doc = "Field `SAR2_PWDET_CCT` writer - SAR2_PWDET_CCT PA power detector capacitance tuning."]
pub type SAR2_PWDET_CCT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `ULP_CP_FORCE_START_TOP` reader - 1: ULP-coprocessor is started by SW 0: ULP-coprocessor is started by timer"]
pub type ULP_CP_FORCE_START_TOP_R = crate::BitReader;
#[doc = "Field `ULP_CP_FORCE_START_TOP` writer - 1: ULP-coprocessor is started by SW 0: ULP-coprocessor is started by timer"]
pub type ULP_CP_FORCE_START_TOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ULP_CP_START_TOP` reader - Write 1 to start ULP-coprocessor only active when reg_ulp_cp_force_start_top = 1"]
pub type ULP_CP_START_TOP_R = crate::BitReader;
#[doc = "Field `ULP_CP_START_TOP` writer - Write 1 to start ULP-coprocessor only active when reg_ulp_cp_force_start_top = 1"]
pub type ULP_CP_START_TOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SARCLK_EN` reader - "]
pub type SARCLK_EN_R = crate::BitReader;
#[doc = "Field `SARCLK_EN` writer - "]
pub type SARCLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PC_INIT` reader - initialized PC for ULP-coprocessor"]
pub type PC_INIT_R = crate::FieldReader<u16>;
#[doc = "Field `PC_INIT` writer - initialized PC for ULP-coprocessor"]
pub type PC_INIT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
#[doc = "Field `SAR2_STOP` reader - stop SAR ADC2 conversion"]
pub type SAR2_STOP_R = crate::BitReader;
#[doc = "Field `SAR2_STOP` writer - stop SAR ADC2 conversion"]
pub type SAR2_STOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SAR1_STOP` reader - stop SAR ADC1 conversion"]
pub type SAR1_STOP_R = crate::BitReader;
#[doc = "Field `SAR1_STOP` writer - stop SAR ADC1 conversion"]
pub type SAR1_STOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SAR2_PWDET_EN` reader - N/A"]
pub type SAR2_PWDET_EN_R = crate::BitReader;
#[doc = "Field `SAR2_PWDET_EN` writer - N/A"]
pub type SAR2_PWDET_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
    #[inline(always)]
    pub fn sar1_bit_width(&self) -> SAR1_BIT_WIDTH_R {
        SAR1_BIT_WIDTH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
    #[inline(always)]
    pub fn sar2_bit_width(&self) -> SAR2_BIT_WIDTH_R {
        SAR2_BIT_WIDTH_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - SAR2_EN_TEST only active when reg_sar2_dig_force = 0"]
    #[inline(always)]
    pub fn sar2_en_test(&self) -> SAR2_EN_TEST_R {
        SAR2_EN_TEST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - SAR2_PWDET_CCT PA power detector capacitance tuning."]
    #[inline(always)]
    pub fn sar2_pwdet_cct(&self) -> SAR2_PWDET_CCT_R {
        SAR2_PWDET_CCT_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - 1: ULP-coprocessor is started by SW 0: ULP-coprocessor is started by timer"]
    #[inline(always)]
    pub fn ulp_cp_force_start_top(&self) -> ULP_CP_FORCE_START_TOP_R {
        ULP_CP_FORCE_START_TOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write 1 to start ULP-coprocessor only active when reg_ulp_cp_force_start_top = 1"]
    #[inline(always)]
    pub fn ulp_cp_start_top(&self) -> ULP_CP_START_TOP_R {
        ULP_CP_START_TOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sarclk_en(&self) -> SARCLK_EN_R {
        SARCLK_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:21 - initialized PC for ULP-coprocessor"]
    #[inline(always)]
    pub fn pc_init(&self) -> PC_INIT_R {
        PC_INIT_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
    #[doc = "Bit 22 - stop SAR ADC2 conversion"]
    #[inline(always)]
    pub fn sar2_stop(&self) -> SAR2_STOP_R {
        SAR2_STOP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - stop SAR ADC1 conversion"]
    #[inline(always)]
    pub fn sar1_stop(&self) -> SAR1_STOP_R {
        SAR1_STOP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    pub fn sar2_pwdet_en(&self) -> SAR2_PWDET_EN_R {
        SAR2_PWDET_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_START_FORCE")
            .field(
                "sar1_bit_width",
                &format_args!("{}", self.sar1_bit_width().bits()),
            )
            .field(
                "sar2_bit_width",
                &format_args!("{}", self.sar2_bit_width().bits()),
            )
            .field(
                "sar2_en_test",
                &format_args!("{}", self.sar2_en_test().bit()),
            )
            .field(
                "sar2_pwdet_cct",
                &format_args!("{}", self.sar2_pwdet_cct().bits()),
            )
            .field(
                "ulp_cp_force_start_top",
                &format_args!("{}", self.ulp_cp_force_start_top().bit()),
            )
            .field(
                "ulp_cp_start_top",
                &format_args!("{}", self.ulp_cp_start_top().bit()),
            )
            .field("sarclk_en", &format_args!("{}", self.sarclk_en().bit()))
            .field("pc_init", &format_args!("{}", self.pc_init().bits()))
            .field("sar2_stop", &format_args!("{}", self.sar2_stop().bit()))
            .field("sar1_stop", &format_args!("{}", self.sar1_stop().bit()))
            .field(
                "sar2_pwdet_en",
                &format_args!("{}", self.sar2_pwdet_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_START_FORCE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
    #[inline(always)]
    #[must_use]
    pub fn sar1_bit_width(&mut self) -> SAR1_BIT_WIDTH_W<SAR_START_FORCE_SPEC, 0> {
        SAR1_BIT_WIDTH_W::new(self)
    }
    #[doc = "Bits 2:3 - 00: 9 bit 01: 10 bits 10: 11bits 11: 12bits"]
    #[inline(always)]
    #[must_use]
    pub fn sar2_bit_width(&mut self) -> SAR2_BIT_WIDTH_W<SAR_START_FORCE_SPEC, 2> {
        SAR2_BIT_WIDTH_W::new(self)
    }
    #[doc = "Bit 4 - SAR2_EN_TEST only active when reg_sar2_dig_force = 0"]
    #[inline(always)]
    #[must_use]
    pub fn sar2_en_test(&mut self) -> SAR2_EN_TEST_W<SAR_START_FORCE_SPEC, 4> {
        SAR2_EN_TEST_W::new(self)
    }
    #[doc = "Bits 5:7 - SAR2_PWDET_CCT PA power detector capacitance tuning."]
    #[inline(always)]
    #[must_use]
    pub fn sar2_pwdet_cct(&mut self) -> SAR2_PWDET_CCT_W<SAR_START_FORCE_SPEC, 5> {
        SAR2_PWDET_CCT_W::new(self)
    }
    #[doc = "Bit 8 - 1: ULP-coprocessor is started by SW 0: ULP-coprocessor is started by timer"]
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp_force_start_top(&mut self) -> ULP_CP_FORCE_START_TOP_W<SAR_START_FORCE_SPEC, 8> {
        ULP_CP_FORCE_START_TOP_W::new(self)
    }
    #[doc = "Bit 9 - Write 1 to start ULP-coprocessor only active when reg_ulp_cp_force_start_top = 1"]
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp_start_top(&mut self) -> ULP_CP_START_TOP_W<SAR_START_FORCE_SPEC, 9> {
        ULP_CP_START_TOP_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn sarclk_en(&mut self) -> SARCLK_EN_W<SAR_START_FORCE_SPEC, 10> {
        SARCLK_EN_W::new(self)
    }
    #[doc = "Bits 11:21 - initialized PC for ULP-coprocessor"]
    #[inline(always)]
    #[must_use]
    pub fn pc_init(&mut self) -> PC_INIT_W<SAR_START_FORCE_SPEC, 11> {
        PC_INIT_W::new(self)
    }
    #[doc = "Bit 22 - stop SAR ADC2 conversion"]
    #[inline(always)]
    #[must_use]
    pub fn sar2_stop(&mut self) -> SAR2_STOP_W<SAR_START_FORCE_SPEC, 22> {
        SAR2_STOP_W::new(self)
    }
    #[doc = "Bit 23 - stop SAR ADC1 conversion"]
    #[inline(always)]
    #[must_use]
    pub fn sar1_stop(&mut self) -> SAR1_STOP_W<SAR_START_FORCE_SPEC, 23> {
        SAR1_STOP_W::new(self)
    }
    #[doc = "Bit 24 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn sar2_pwdet_en(&mut self) -> SAR2_PWDET_EN_W<SAR_START_FORCE_SPEC, 24> {
        SAR2_PWDET_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_start_force::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_start_force::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_START_FORCE_SPEC;
impl crate::RegisterSpec for SAR_START_FORCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_start_force::R`](R) reader structure"]
impl crate::Readable for SAR_START_FORCE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_start_force::W`](W) writer structure"]
impl crate::Writable for SAR_START_FORCE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_START_FORCE to value 0x0f"]
impl crate::Resettable for SAR_START_FORCE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
