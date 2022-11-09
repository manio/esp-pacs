#[doc = "Register `TIMING_CALI` reader"]
pub struct R(crate::R<TIMING_CALI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMING_CALI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMING_CALI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMING_CALI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMING_CALI` writer"]
pub struct W(crate::W<TIMING_CALI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMING_CALI_SPEC>;
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
impl From<crate::W<TIMING_CALI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMING_CALI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMING_CLK_ENA` reader - Set this bit to power on HCLK. When PLL is powered on, the frequency of HCLK equals to that of PLL. Otherwise, the frequency equals to that of XTAL."]
pub type TIMING_CLK_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TIMING_CLK_ENA` writer - Set this bit to power on HCLK. When PLL is powered on, the frequency of HCLK equals to that of PLL. Otherwise, the frequency equals to that of XTAL."]
pub type TIMING_CLK_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMING_CALI_SPEC, bool, O>;
#[doc = "Field `TIMING_CALI` reader - Set this bit to add extra SPI_CLK cycles in DUMMY phase for all reading operations."]
pub type TIMING_CALI_R = crate::BitReader<bool>;
#[doc = "Field `TIMING_CALI` writer - Set this bit to add extra SPI_CLK cycles in DUMMY phase for all reading operations."]
pub type TIMING_CALI_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMING_CALI_SPEC, bool, O>;
#[doc = "Field `EXTRA_DUMMY_CYCLELEN` reader - Extra SPI_CLK cycles added in DUMMY phase for timing compensation, when SPI0 accesses to flash. Active when SPI_MEM_TIMING_CALI bit is set."]
pub type EXTRA_DUMMY_CYCLELEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTRA_DUMMY_CYCLELEN` writer - Extra SPI_CLK cycles added in DUMMY phase for timing compensation, when SPI0 accesses to flash. Active when SPI_MEM_TIMING_CALI bit is set."]
pub type EXTRA_DUMMY_CYCLELEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMING_CALI_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - Set this bit to power on HCLK. When PLL is powered on, the frequency of HCLK equals to that of PLL. Otherwise, the frequency equals to that of XTAL."]
    #[inline(always)]
    pub fn timing_clk_ena(&self) -> TIMING_CLK_ENA_R {
        TIMING_CLK_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to add extra SPI_CLK cycles in DUMMY phase for all reading operations."]
    #[inline(always)]
    pub fn timing_cali(&self) -> TIMING_CALI_R {
        TIMING_CALI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Extra SPI_CLK cycles added in DUMMY phase for timing compensation, when SPI0 accesses to flash. Active when SPI_MEM_TIMING_CALI bit is set."]
    #[inline(always)]
    pub fn extra_dummy_cyclelen(&self) -> EXTRA_DUMMY_CYCLELEN_R {
        EXTRA_DUMMY_CYCLELEN_R::new(((self.bits >> 2) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to power on HCLK. When PLL is powered on, the frequency of HCLK equals to that of PLL. Otherwise, the frequency equals to that of XTAL."]
    #[inline(always)]
    #[must_use]
    pub fn timing_clk_ena(&mut self) -> TIMING_CLK_ENA_W<0> {
        TIMING_CLK_ENA_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to add extra SPI_CLK cycles in DUMMY phase for all reading operations."]
    #[inline(always)]
    #[must_use]
    pub fn timing_cali(&mut self) -> TIMING_CALI_W<1> {
        TIMING_CALI_W::new(self)
    }
    #[doc = "Bits 2:4 - Extra SPI_CLK cycles added in DUMMY phase for timing compensation, when SPI0 accesses to flash. Active when SPI_MEM_TIMING_CALI bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn extra_dummy_cyclelen(&mut self) -> EXTRA_DUMMY_CYCLELEN_W<2> {
        EXTRA_DUMMY_CYCLELEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 timing compensation register when accesses to flash.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timing_cali](index.html) module"]
pub struct TIMING_CALI_SPEC;
impl crate::RegisterSpec for TIMING_CALI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timing_cali::R](R) reader structure"]
impl crate::Readable for TIMING_CALI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timing_cali::W](W) writer structure"]
impl crate::Writable for TIMING_CALI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMING_CALI to value 0"]
impl crate::Resettable for TIMING_CALI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
