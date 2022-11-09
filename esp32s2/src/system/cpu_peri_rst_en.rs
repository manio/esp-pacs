#[doc = "Register `CPU_PERI_RST_EN` reader"]
pub struct R(crate::R<CPU_PERI_RST_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_PERI_RST_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_PERI_RST_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_PERI_RST_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_PERI_RST_EN` writer"]
pub struct W(crate::W<CPU_PERI_RST_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_PERI_RST_EN_SPEC>;
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
impl From<crate::W<CPU_PERI_RST_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_PERI_RST_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RST_EN_DEDICATED_GPIO` reader - Set this bit to reset DEDICATED GPIO module."]
pub type RST_EN_DEDICATED_GPIO_R = crate::BitReader<bool>;
#[doc = "Field `RST_EN_DEDICATED_GPIO` writer - Set this bit to reset DEDICATED GPIO module."]
pub type RST_EN_DEDICATED_GPIO_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_PERI_RST_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 7 - Set this bit to reset DEDICATED GPIO module."]
    #[inline(always)]
    pub fn rst_en_dedicated_gpio(&self) -> RST_EN_DEDICATED_GPIO_R {
        RST_EN_DEDICATED_GPIO_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Set this bit to reset DEDICATED GPIO module."]
    #[inline(always)]
    #[must_use]
    pub fn rst_en_dedicated_gpio(&mut self) -> RST_EN_DEDICATED_GPIO_W<7> {
        RST_EN_DEDICATED_GPIO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_peri_rst_en](index.html) module"]
pub struct CPU_PERI_RST_EN_SPEC;
impl crate::RegisterSpec for CPU_PERI_RST_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_peri_rst_en::R](R) reader structure"]
impl crate::Readable for CPU_PERI_RST_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_peri_rst_en::W](W) writer structure"]
impl crate::Writable for CPU_PERI_RST_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPU_PERI_RST_EN to value 0x80"]
impl crate::Resettable for CPU_PERI_RST_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
