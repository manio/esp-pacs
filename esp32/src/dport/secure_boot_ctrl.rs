#[doc = "Register `SECURE_BOOT_CTRL` reader"]
pub type R = crate::R<SECURE_BOOT_CTRL_SPEC>;
#[doc = "Register `SECURE_BOOT_CTRL` writer"]
pub type W = crate::W<SECURE_BOOT_CTRL_SPEC>;
#[doc = "Field `SW_BOOTLOADER_SEL` reader - "]
pub type SW_BOOTLOADER_SEL_R = crate::BitReader;
#[doc = "Field `SW_BOOTLOADER_SEL` writer - "]
pub type SW_BOOTLOADER_SEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sw_bootloader_sel(&self) -> SW_BOOTLOADER_SEL_R {
        SW_BOOTLOADER_SEL_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECURE_BOOT_CTRL")
            .field(
                "sw_bootloader_sel",
                &format_args!("{}", self.sw_bootloader_sel().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SECURE_BOOT_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sw_bootloader_sel(&mut self) -> SW_BOOTLOADER_SEL_W<SECURE_BOOT_CTRL_SPEC, 0> {
        SW_BOOTLOADER_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secure_boot_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secure_boot_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECURE_BOOT_CTRL_SPEC;
impl crate::RegisterSpec for SECURE_BOOT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure_boot_ctrl::R`](R) reader structure"]
impl crate::Readable for SECURE_BOOT_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`secure_boot_ctrl::W`](W) writer structure"]
impl crate::Writable for SECURE_BOOT_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SECURE_BOOT_CTRL to value 0"]
impl crate::Resettable for SECURE_BOOT_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
