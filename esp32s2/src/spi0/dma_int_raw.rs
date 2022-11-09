#[doc = "Register `DMA_INT_RAW` reader"]
pub struct R(crate::R<DMA_INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_INT_RAW` writer"]
pub struct W(crate::W<DMA_INT_RAW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_INT_RAW_SPEC>;
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
impl From<crate::W<DMA_INT_RAW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_INT_RAW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INLINK_DSCR_EMPTY_INT_RAW` reader - The raw bit for lack of enough inlink descriptors. Can be configured in CONF state."]
pub type INLINK_DSCR_EMPTY_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `OUTLINK_DSCR_ERROR_INT_RAW` reader - The raw bit for outlink descriptor error. Can be configured in CONF state."]
pub type OUTLINK_DSCR_ERROR_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `INLINK_DSCR_ERROR_INT_RAW` reader - The raw bit for inlink descriptor error. Can be configured in CONF state."]
pub type INLINK_DSCR_ERROR_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `IN_DONE_INT_RAW` reader - The raw bit for completing usage of a inlink descriptor. Can be configured in CONF state."]
pub type IN_DONE_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `IN_ERR_EOF_INT_RAW` reader - The raw bit for receiving error. Can be configured in CONF state."]
pub type IN_ERR_EOF_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `IN_SUC_EOF_INT_RAW` reader - The raw bit for completing receiving all the packets from host. Can be configured in CONF state."]
pub type IN_SUC_EOF_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `OUT_DONE_INT_RAW` reader - The raw bit for completing usage of a outlink descriptor. Can be configured in CONF state."]
pub type OUT_DONE_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `OUT_EOF_INT_RAW` reader - The raw bit for sending a packet to host done. Can be configured in CONF state."]
pub type OUT_EOF_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `OUT_TOTAL_EOF_INT_RAW` reader - The raw bit for sending all the packets to host done. Can be configured in CONF state."]
pub type OUT_TOTAL_EOF_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `INFIFO_FULL_ERR_INT_RAW` reader - 1:SPI_DMA_INFIFO_FULL and spi_push_data_prep are valid, which means that DMA Rx buffer is full but push is valid. 0: Others. Can not be changed by CONF_buf."]
pub type INFIFO_FULL_ERR_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `OUTFIFO_EMPTY_ERR_INT_RAW` reader - 1:SPI_DMA_OUTFIFO_EMPTY and spi_pop_data_prep are valid, which means that there is no data to pop but pop is valid. 0: Others. Can not be changed by CONF_buf."]
pub type OUTFIFO_EMPTY_ERR_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLV_CMD6_INT_RAW` reader - The raw bit for SPI slave CMD6 interrupt."]
pub type SLV_CMD6_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLV_CMD6_INT_RAW` writer - The raw bit for SPI slave CMD6 interrupt."]
pub type SLV_CMD6_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT_RAW_SPEC, bool, O>;
#[doc = "Field `SLV_CMD7_INT_RAW` reader - The raw bit for SPI slave CMD7 interrupt."]
pub type SLV_CMD7_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLV_CMD7_INT_RAW` writer - The raw bit for SPI slave CMD7 interrupt."]
pub type SLV_CMD7_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT_RAW_SPEC, bool, O>;
#[doc = "Field `SLV_CMD8_INT_RAW` reader - The raw bit for SPI slave CMD8 interrupt."]
pub type SLV_CMD8_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLV_CMD8_INT_RAW` writer - The raw bit for SPI slave CMD8 interrupt."]
pub type SLV_CMD8_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT_RAW_SPEC, bool, O>;
#[doc = "Field `SLV_CMD9_INT_RAW` reader - The raw bit for SPI slave CMD9 interrupt."]
pub type SLV_CMD9_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLV_CMD9_INT_RAW` writer - The raw bit for SPI slave CMD9 interrupt."]
pub type SLV_CMD9_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT_RAW_SPEC, bool, O>;
#[doc = "Field `SLV_CMDA_INT_RAW` reader - The raw bit for SPI slave CMDA interrupt."]
pub type SLV_CMDA_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `SLV_CMDA_INT_RAW` writer - The raw bit for SPI slave CMDA interrupt."]
pub type SLV_CMDA_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_INT_RAW_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - The raw bit for lack of enough inlink descriptors. Can be configured in CONF state."]
    #[inline(always)]
    pub fn inlink_dscr_empty_int_raw(&self) -> INLINK_DSCR_EMPTY_INT_RAW_R {
        INLINK_DSCR_EMPTY_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw bit for outlink descriptor error. Can be configured in CONF state."]
    #[inline(always)]
    pub fn outlink_dscr_error_int_raw(&self) -> OUTLINK_DSCR_ERROR_INT_RAW_R {
        OUTLINK_DSCR_ERROR_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw bit for inlink descriptor error. Can be configured in CONF state."]
    #[inline(always)]
    pub fn inlink_dscr_error_int_raw(&self) -> INLINK_DSCR_ERROR_INT_RAW_R {
        INLINK_DSCR_ERROR_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw bit for completing usage of a inlink descriptor. Can be configured in CONF state."]
    #[inline(always)]
    pub fn in_done_int_raw(&self) -> IN_DONE_INT_RAW_R {
        IN_DONE_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw bit for receiving error. Can be configured in CONF state."]
    #[inline(always)]
    pub fn in_err_eof_int_raw(&self) -> IN_ERR_EOF_INT_RAW_R {
        IN_ERR_EOF_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw bit for completing receiving all the packets from host. Can be configured in CONF state."]
    #[inline(always)]
    pub fn in_suc_eof_int_raw(&self) -> IN_SUC_EOF_INT_RAW_R {
        IN_SUC_EOF_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw bit for completing usage of a outlink descriptor. Can be configured in CONF state."]
    #[inline(always)]
    pub fn out_done_int_raw(&self) -> OUT_DONE_INT_RAW_R {
        OUT_DONE_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw bit for sending a packet to host done. Can be configured in CONF state."]
    #[inline(always)]
    pub fn out_eof_int_raw(&self) -> OUT_EOF_INT_RAW_R {
        OUT_EOF_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw bit for sending all the packets to host done. Can be configured in CONF state."]
    #[inline(always)]
    pub fn out_total_eof_int_raw(&self) -> OUT_TOTAL_EOF_INT_RAW_R {
        OUT_TOTAL_EOF_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1:SPI_DMA_INFIFO_FULL and spi_push_data_prep are valid, which means that DMA Rx buffer is full but push is valid. 0: Others. Can not be changed by CONF_buf."]
    #[inline(always)]
    pub fn infifo_full_err_int_raw(&self) -> INFIFO_FULL_ERR_INT_RAW_R {
        INFIFO_FULL_ERR_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1:SPI_DMA_OUTFIFO_EMPTY and spi_pop_data_prep are valid, which means that there is no data to pop but pop is valid. 0: Others. Can not be changed by CONF_buf."]
    #[inline(always)]
    pub fn outfifo_empty_err_int_raw(&self) -> OUTFIFO_EMPTY_ERR_INT_RAW_R {
        OUTFIFO_EMPTY_ERR_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The raw bit for SPI slave CMD6 interrupt."]
    #[inline(always)]
    pub fn slv_cmd6_int_raw(&self) -> SLV_CMD6_INT_RAW_R {
        SLV_CMD6_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The raw bit for SPI slave CMD7 interrupt."]
    #[inline(always)]
    pub fn slv_cmd7_int_raw(&self) -> SLV_CMD7_INT_RAW_R {
        SLV_CMD7_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The raw bit for SPI slave CMD8 interrupt."]
    #[inline(always)]
    pub fn slv_cmd8_int_raw(&self) -> SLV_CMD8_INT_RAW_R {
        SLV_CMD8_INT_RAW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The raw bit for SPI slave CMD9 interrupt."]
    #[inline(always)]
    pub fn slv_cmd9_int_raw(&self) -> SLV_CMD9_INT_RAW_R {
        SLV_CMD9_INT_RAW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The raw bit for SPI slave CMDA interrupt."]
    #[inline(always)]
    pub fn slv_cmda_int_raw(&self) -> SLV_CMDA_INT_RAW_R {
        SLV_CMDA_INT_RAW_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - The raw bit for SPI slave CMD6 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_cmd6_int_raw(&mut self) -> SLV_CMD6_INT_RAW_W<11> {
        SLV_CMD6_INT_RAW_W::new(self)
    }
    #[doc = "Bit 12 - The raw bit for SPI slave CMD7 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_cmd7_int_raw(&mut self) -> SLV_CMD7_INT_RAW_W<12> {
        SLV_CMD7_INT_RAW_W::new(self)
    }
    #[doc = "Bit 13 - The raw bit for SPI slave CMD8 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_cmd8_int_raw(&mut self) -> SLV_CMD8_INT_RAW_W<13> {
        SLV_CMD8_INT_RAW_W::new(self)
    }
    #[doc = "Bit 14 - The raw bit for SPI slave CMD9 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_cmd9_int_raw(&mut self) -> SLV_CMD9_INT_RAW_W<14> {
        SLV_CMD9_INT_RAW_W::new(self)
    }
    #[doc = "Bit 15 - The raw bit for SPI slave CMDA interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_cmda_int_raw(&mut self) -> SLV_CMDA_INT_RAW_W<15> {
        SLV_CMDA_INT_RAW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI DMA interrupt raw register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_raw](index.html) module"]
pub struct DMA_INT_RAW_SPEC;
impl crate::RegisterSpec for DMA_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_int_raw::R](R) reader structure"]
impl crate::Readable for DMA_INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_int_raw::W](W) writer structure"]
impl crate::Writable for DMA_INT_RAW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_INT_RAW to value 0"]
impl crate::Resettable for DMA_INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
