#[doc = "Register `CACHE_FCTRL` reader"]
pub struct R(crate::R<CACHE_FCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_FCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_FCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_FCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_FCTRL` writer"]
pub struct W(crate::W<CACHE_FCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_FCTRL_SPEC>;
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
impl From<crate::W<CACHE_FCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_FCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CACHE_REQ_EN` reader - Set this bit to enable Cache's access and SPI0's transfer."]
pub type CACHE_REQ_EN_R = crate::BitReader<bool>;
#[doc = "Field `CACHE_REQ_EN` writer - Set this bit to enable Cache's access and SPI0's transfer."]
pub type CACHE_REQ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CACHE_FCTRL_SPEC, bool, O>;
#[doc = "Field `CACHE_USR_CMD_4BYTE` reader - Set this bit to enable SPI0 read flash with 32 bits address. The value of SPI_MEM_USR_ADDR_BITLEN should be 31."]
pub type CACHE_USR_CMD_4BYTE_R = crate::BitReader<bool>;
#[doc = "Field `CACHE_USR_CMD_4BYTE` writer - Set this bit to enable SPI0 read flash with 32 bits address. The value of SPI_MEM_USR_ADDR_BITLEN should be 31."]
pub type CACHE_USR_CMD_4BYTE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CACHE_FCTRL_SPEC, bool, O>;
#[doc = "Field `CACHE_FLASH_USR_CMD` reader - 1: The command value of SPI0 reads flash is SPI_MEM_USR_COMMAND_VALUE. 0: Hardware read command value, controlled by SPI_MEM_FREAD_QIO, SPI_MEM_FREAD_DIO, SPI_MEM_FREAD_QUAD, SPI_MEM_FREAD_DUAL and SPI_MEM_FASTRD_MODE bits."]
pub type CACHE_FLASH_USR_CMD_R = crate::BitReader<bool>;
#[doc = "Field `CACHE_FLASH_USR_CMD` writer - 1: The command value of SPI0 reads flash is SPI_MEM_USR_COMMAND_VALUE. 0: Hardware read command value, controlled by SPI_MEM_FREAD_QIO, SPI_MEM_FREAD_DIO, SPI_MEM_FREAD_QUAD, SPI_MEM_FREAD_DUAL and SPI_MEM_FASTRD_MODE bits."]
pub type CACHE_FLASH_USR_CMD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CACHE_FCTRL_SPEC, bool, O>;
#[doc = "Field `FDIN_DUAL` reader - When SPI0 accesses to flash, set this bit to enable 2-bm in DIN phase."]
pub type FDIN_DUAL_R = crate::BitReader<bool>;
#[doc = "Field `FDIN_DUAL` writer - When SPI0 accesses to flash, set this bit to enable 2-bm in DIN phase."]
pub type FDIN_DUAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CACHE_FCTRL_SPEC, bool, O>;
#[doc = "Field `FDOUT_DUAL` reader - When SPI0 accesses to flash, set this bit to enable 2-bm in DOUT phase."]
pub type FDOUT_DUAL_R = crate::BitReader<bool>;
#[doc = "Field `FDOUT_DUAL` writer - When SPI0 accesses to flash, set this bit to enable 2-bm in DOUT phase."]
pub type FDOUT_DUAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CACHE_FCTRL_SPEC, bool, O>;
#[doc = "Field `FADDR_DUAL` reader - When SPI0 accesses to flash, set this bit to enable 2-bm in ADDR phase."]
pub type FADDR_DUAL_R = crate::BitReader<bool>;
#[doc = "Field `FADDR_DUAL` writer - When SPI0 accesses to flash, set this bit to enable 2-bm in ADDR phase."]
pub type FADDR_DUAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CACHE_FCTRL_SPEC, bool, O>;
#[doc = "Field `FDIN_QUAD` reader - When SPI0 accesses to flash, set this bit to enable 4-bm in DIN phase."]
pub type FDIN_QUAD_R = crate::BitReader<bool>;
#[doc = "Field `FDIN_QUAD` writer - When SPI0 accesses to flash, set this bit to enable 4-bm in DIN phase."]
pub type FDIN_QUAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CACHE_FCTRL_SPEC, bool, O>;
#[doc = "Field `FDOUT_QUAD` reader - When SPI0 accesses to flash, set this bit to enable 4-bm in DOUT phase."]
pub type FDOUT_QUAD_R = crate::BitReader<bool>;
#[doc = "Field `FDOUT_QUAD` writer - When SPI0 accesses to flash, set this bit to enable 4-bm in DOUT phase."]
pub type FDOUT_QUAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CACHE_FCTRL_SPEC, bool, O>;
#[doc = "Field `FADDR_QUAD` reader - When SPI0 accesses to flash, set this bit to enable 4-bm in ADDR phase."]
pub type FADDR_QUAD_R = crate::BitReader<bool>;
#[doc = "Field `FADDR_QUAD` writer - When SPI0 accesses to flash, set this bit to enable 4-bm in ADDR phase."]
pub type FADDR_QUAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CACHE_FCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable Cache's access and SPI0's transfer."]
    #[inline(always)]
    pub fn cache_req_en(&self) -> CACHE_REQ_EN_R {
        CACHE_REQ_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to enable SPI0 read flash with 32 bits address. The value of SPI_MEM_USR_ADDR_BITLEN should be 31."]
    #[inline(always)]
    pub fn cache_usr_cmd_4byte(&self) -> CACHE_USR_CMD_4BYTE_R {
        CACHE_USR_CMD_4BYTE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: The command value of SPI0 reads flash is SPI_MEM_USR_COMMAND_VALUE. 0: Hardware read command value, controlled by SPI_MEM_FREAD_QIO, SPI_MEM_FREAD_DIO, SPI_MEM_FREAD_QUAD, SPI_MEM_FREAD_DUAL and SPI_MEM_FASTRD_MODE bits."]
    #[inline(always)]
    pub fn cache_flash_usr_cmd(&self) -> CACHE_FLASH_USR_CMD_R {
        CACHE_FLASH_USR_CMD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When SPI0 accesses to flash, set this bit to enable 2-bm in DIN phase."]
    #[inline(always)]
    pub fn fdin_dual(&self) -> FDIN_DUAL_R {
        FDIN_DUAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When SPI0 accesses to flash, set this bit to enable 2-bm in DOUT phase."]
    #[inline(always)]
    pub fn fdout_dual(&self) -> FDOUT_DUAL_R {
        FDOUT_DUAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When SPI0 accesses to flash, set this bit to enable 2-bm in ADDR phase."]
    #[inline(always)]
    pub fn faddr_dual(&self) -> FADDR_DUAL_R {
        FADDR_DUAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When SPI0 accesses to flash, set this bit to enable 4-bm in DIN phase."]
    #[inline(always)]
    pub fn fdin_quad(&self) -> FDIN_QUAD_R {
        FDIN_QUAD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When SPI0 accesses to flash, set this bit to enable 4-bm in DOUT phase."]
    #[inline(always)]
    pub fn fdout_quad(&self) -> FDOUT_QUAD_R {
        FDOUT_QUAD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - When SPI0 accesses to flash, set this bit to enable 4-bm in ADDR phase."]
    #[inline(always)]
    pub fn faddr_quad(&self) -> FADDR_QUAD_R {
        FADDR_QUAD_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable Cache's access and SPI0's transfer."]
    #[inline(always)]
    #[must_use]
    pub fn cache_req_en(&mut self) -> CACHE_REQ_EN_W<0> {
        CACHE_REQ_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to enable SPI0 read flash with 32 bits address. The value of SPI_MEM_USR_ADDR_BITLEN should be 31."]
    #[inline(always)]
    #[must_use]
    pub fn cache_usr_cmd_4byte(&mut self) -> CACHE_USR_CMD_4BYTE_W<1> {
        CACHE_USR_CMD_4BYTE_W::new(self)
    }
    #[doc = "Bit 2 - 1: The command value of SPI0 reads flash is SPI_MEM_USR_COMMAND_VALUE. 0: Hardware read command value, controlled by SPI_MEM_FREAD_QIO, SPI_MEM_FREAD_DIO, SPI_MEM_FREAD_QUAD, SPI_MEM_FREAD_DUAL and SPI_MEM_FASTRD_MODE bits."]
    #[inline(always)]
    #[must_use]
    pub fn cache_flash_usr_cmd(&mut self) -> CACHE_FLASH_USR_CMD_W<2> {
        CACHE_FLASH_USR_CMD_W::new(self)
    }
    #[doc = "Bit 3 - When SPI0 accesses to flash, set this bit to enable 2-bm in DIN phase."]
    #[inline(always)]
    #[must_use]
    pub fn fdin_dual(&mut self) -> FDIN_DUAL_W<3> {
        FDIN_DUAL_W::new(self)
    }
    #[doc = "Bit 4 - When SPI0 accesses to flash, set this bit to enable 2-bm in DOUT phase."]
    #[inline(always)]
    #[must_use]
    pub fn fdout_dual(&mut self) -> FDOUT_DUAL_W<4> {
        FDOUT_DUAL_W::new(self)
    }
    #[doc = "Bit 5 - When SPI0 accesses to flash, set this bit to enable 2-bm in ADDR phase."]
    #[inline(always)]
    #[must_use]
    pub fn faddr_dual(&mut self) -> FADDR_DUAL_W<5> {
        FADDR_DUAL_W::new(self)
    }
    #[doc = "Bit 6 - When SPI0 accesses to flash, set this bit to enable 4-bm in DIN phase."]
    #[inline(always)]
    #[must_use]
    pub fn fdin_quad(&mut self) -> FDIN_QUAD_W<6> {
        FDIN_QUAD_W::new(self)
    }
    #[doc = "Bit 7 - When SPI0 accesses to flash, set this bit to enable 4-bm in DOUT phase."]
    #[inline(always)]
    #[must_use]
    pub fn fdout_quad(&mut self) -> FDOUT_QUAD_W<7> {
        FDOUT_QUAD_W::new(self)
    }
    #[doc = "Bit 8 - When SPI0 accesses to flash, set this bit to enable 4-bm in ADDR phase."]
    #[inline(always)]
    #[must_use]
    pub fn faddr_quad(&mut self) -> FADDR_QUAD_W<8> {
        FADDR_QUAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 external RAM bit mode control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_fctrl](index.html) module"]
pub struct CACHE_FCTRL_SPEC;
impl crate::RegisterSpec for CACHE_FCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_fctrl::R](R) reader structure"]
impl crate::Readable for CACHE_FCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_fctrl::W](W) writer structure"]
impl crate::Writable for CACHE_FCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHE_FCTRL to value 0"]
impl crate::Resettable for CACHE_FCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
