#[doc = "Register `ETM_TASK_P2_CFG` reader"]
pub type R = crate::R<ETM_TASK_P2_CFG_SPEC>;
#[doc = "Register `ETM_TASK_P2_CFG` writer"]
pub type W = crate::W<ETM_TASK_P2_CFG_SPEC>;
#[doc = "Field `ETM_TASK_GPIO8_EN` reader - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO8_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO8_EN` writer - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO8_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETM_TASK_GPIO8_SEL` reader - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO8_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO8_SEL` writer - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO8_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `ETM_TASK_GPIO9_EN` reader - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO9_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO9_EN` writer - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO9_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETM_TASK_GPIO9_SEL` reader - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO9_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO9_SEL` writer - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO9_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `ETM_TASK_GPIO10_EN` reader - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO10_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO10_EN` writer - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO10_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETM_TASK_GPIO10_SEL` reader - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO10_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO10_SEL` writer - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO10_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `ETM_TASK_GPIO11_EN` reader - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO11_EN_R = crate::BitReader;
#[doc = "Field `ETM_TASK_GPIO11_EN` writer - Enable bit of GPIO response etm task."]
pub type ETM_TASK_GPIO11_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ETM_TASK_GPIO11_SEL` reader - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO11_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_TASK_GPIO11_SEL` writer - GPIO choose a etm task channel."]
pub type ETM_TASK_GPIO11_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bit 0 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio8_en(&self) -> ETM_TASK_GPIO8_EN_R {
        ETM_TASK_GPIO8_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio8_sel(&self) -> ETM_TASK_GPIO8_SEL_R {
        ETM_TASK_GPIO8_SEL_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 8 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio9_en(&self) -> ETM_TASK_GPIO9_EN_R {
        ETM_TASK_GPIO9_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio9_sel(&self) -> ETM_TASK_GPIO9_SEL_R {
        ETM_TASK_GPIO9_SEL_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 16 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio10_en(&self) -> ETM_TASK_GPIO10_EN_R {
        ETM_TASK_GPIO10_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio10_sel(&self) -> ETM_TASK_GPIO10_SEL_R {
        ETM_TASK_GPIO10_SEL_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 24 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn etm_task_gpio11_en(&self) -> ETM_TASK_GPIO11_EN_R {
        ETM_TASK_GPIO11_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn etm_task_gpio11_sel(&self) -> ETM_TASK_GPIO11_SEL_R {
        ETM_TASK_GPIO11_SEL_R::new(((self.bits >> 25) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_TASK_P2_CFG")
            .field(
                "etm_task_gpio8_en",
                &format_args!("{}", self.etm_task_gpio8_en().bit()),
            )
            .field(
                "etm_task_gpio8_sel",
                &format_args!("{}", self.etm_task_gpio8_sel().bits()),
            )
            .field(
                "etm_task_gpio9_en",
                &format_args!("{}", self.etm_task_gpio9_en().bit()),
            )
            .field(
                "etm_task_gpio9_sel",
                &format_args!("{}", self.etm_task_gpio9_sel().bits()),
            )
            .field(
                "etm_task_gpio10_en",
                &format_args!("{}", self.etm_task_gpio10_en().bit()),
            )
            .field(
                "etm_task_gpio10_sel",
                &format_args!("{}", self.etm_task_gpio10_sel().bits()),
            )
            .field(
                "etm_task_gpio11_en",
                &format_args!("{}", self.etm_task_gpio11_en().bit()),
            )
            .field(
                "etm_task_gpio11_sel",
                &format_args!("{}", self.etm_task_gpio11_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ETM_TASK_P2_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    #[must_use]
    pub fn etm_task_gpio8_en(&mut self) -> ETM_TASK_GPIO8_EN_W<ETM_TASK_P2_CFG_SPEC, 0> {
        ETM_TASK_GPIO8_EN_W::new(self)
    }
    #[doc = "Bits 1:3 - GPIO choose a etm task channel."]
    #[inline(always)]
    #[must_use]
    pub fn etm_task_gpio8_sel(&mut self) -> ETM_TASK_GPIO8_SEL_W<ETM_TASK_P2_CFG_SPEC, 1> {
        ETM_TASK_GPIO8_SEL_W::new(self)
    }
    #[doc = "Bit 8 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    #[must_use]
    pub fn etm_task_gpio9_en(&mut self) -> ETM_TASK_GPIO9_EN_W<ETM_TASK_P2_CFG_SPEC, 8> {
        ETM_TASK_GPIO9_EN_W::new(self)
    }
    #[doc = "Bits 9:11 - GPIO choose a etm task channel."]
    #[inline(always)]
    #[must_use]
    pub fn etm_task_gpio9_sel(&mut self) -> ETM_TASK_GPIO9_SEL_W<ETM_TASK_P2_CFG_SPEC, 9> {
        ETM_TASK_GPIO9_SEL_W::new(self)
    }
    #[doc = "Bit 16 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    #[must_use]
    pub fn etm_task_gpio10_en(&mut self) -> ETM_TASK_GPIO10_EN_W<ETM_TASK_P2_CFG_SPEC, 16> {
        ETM_TASK_GPIO10_EN_W::new(self)
    }
    #[doc = "Bits 17:19 - GPIO choose a etm task channel."]
    #[inline(always)]
    #[must_use]
    pub fn etm_task_gpio10_sel(&mut self) -> ETM_TASK_GPIO10_SEL_W<ETM_TASK_P2_CFG_SPEC, 17> {
        ETM_TASK_GPIO10_SEL_W::new(self)
    }
    #[doc = "Bit 24 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    #[must_use]
    pub fn etm_task_gpio11_en(&mut self) -> ETM_TASK_GPIO11_EN_W<ETM_TASK_P2_CFG_SPEC, 24> {
        ETM_TASK_GPIO11_EN_W::new(self)
    }
    #[doc = "Bits 25:27 - GPIO choose a etm task channel."]
    #[inline(always)]
    #[must_use]
    pub fn etm_task_gpio11_sel(&mut self) -> ETM_TASK_GPIO11_SEL_W<ETM_TASK_P2_CFG_SPEC, 25> {
        ETM_TASK_GPIO11_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etm_task_p2_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_task_p2_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETM_TASK_P2_CFG_SPEC;
impl crate::RegisterSpec for ETM_TASK_P2_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etm_task_p2_cfg::R`](R) reader structure"]
impl crate::Readable for ETM_TASK_P2_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etm_task_p2_cfg::W`](W) writer structure"]
impl crate::Writable for ETM_TASK_P2_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETM_TASK_P2_CFG to value 0"]
impl crate::Resettable for ETM_TASK_P2_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
