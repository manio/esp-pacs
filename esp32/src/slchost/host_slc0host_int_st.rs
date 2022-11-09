#[doc = "Register `HOST_SLC0HOST_INT_ST` reader"]
pub struct R(crate::R<HOST_SLC0HOST_INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLC0HOST_INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLC0HOST_INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLC0HOST_INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HOST_SLC0_TOHOST_BIT0_INT_ST` reader - "]
pub type HOST_SLC0_TOHOST_BIT0_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `HOST_SLC0_TOHOST_BIT1_INT_ST` reader - "]
pub type HOST_SLC0_TOHOST_BIT1_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `HOST_SLC0_TOHOST_BIT2_INT_ST` reader - "]
pub type HOST_SLC0_TOHOST_BIT2_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `HOST_SLC0_TOHOST_BIT3_INT_ST` reader - "]
pub type HOST_SLC0_TOHOST_BIT3_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `HOST_SLC0_TOHOST_BIT4_INT_ST` reader - "]
pub type HOST_SLC0_TOHOST_BIT4_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `HOST_SLC0_TOHOST_BIT5_INT_ST` reader - "]
pub type HOST_SLC0_TOHOST_BIT5_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `HOST_SLC0_TOHOST_BIT6_INT_ST` reader - "]
pub type HOST_SLC0_TOHOST_BIT6_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `HOST_SLC0_TOHOST_BIT7_INT_ST` reader - "]
pub type HOST_SLC0_TOHOST_BIT7_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `HOST_SLC0_TOKEN0_1TO0_INT_ST` reader - "]
pub type HOST_SLC0_TOKEN0_1TO0_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `HOST_SLC0_TOKEN1_1TO0_INT_ST` reader - "]
pub type HOST_SLC0_TOKEN1_1TO0_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `HOST_SLC0_TOKEN0_0TO1_INT_ST` reader - "]
pub type HOST_SLC0_TOKEN0_0TO1_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `HOST_SLC0_TOKEN1_0TO1_INT_ST` reader - "]
pub type HOST_SLC0_TOKEN1_0TO1_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `HOST_SLC0HOST_RX_SOF_INT_ST` reader - "]
pub type HOST_SLC0HOST_RX_SOF_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `HOST_SLC0HOST_RX_EOF_INT_ST` reader - "]
pub type HOST_SLC0HOST_RX_EOF_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `HOST_SLC0HOST_RX_START_INT_ST` reader - "]
pub type HOST_SLC0HOST_RX_START_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `HOST_SLC0HOST_TX_START_INT_ST` reader - "]
pub type HOST_SLC0HOST_TX_START_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `HOST_SLC0_RX_UDF_INT_ST` reader - "]
pub type HOST_SLC0_RX_UDF_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `HOST_SLC0_TX_OVF_INT_ST` reader - "]
pub type HOST_SLC0_TX_OVF_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `HOST_SLC0_RX_PF_VALID_INT_ST` reader - "]
pub type HOST_SLC0_RX_PF_VALID_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `HOST_SLC0_EXT_BIT0_INT_ST` reader - "]
pub type HOST_SLC0_EXT_BIT0_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `HOST_SLC0_EXT_BIT1_INT_ST` reader - "]
pub type HOST_SLC0_EXT_BIT1_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `HOST_SLC0_EXT_BIT2_INT_ST` reader - "]
pub type HOST_SLC0_EXT_BIT2_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `HOST_SLC0_EXT_BIT3_INT_ST` reader - "]
pub type HOST_SLC0_EXT_BIT3_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `HOST_SLC0_RX_NEW_PACKET_INT_ST` reader - "]
pub type HOST_SLC0_RX_NEW_PACKET_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `HOST_SLC0_HOST_RD_RETRY_INT_ST` reader - "]
pub type HOST_SLC0_HOST_RD_RETRY_INT_ST_R = crate::BitReader<bool>;
#[doc = "Field `HOST_GPIO_SDIO_INT_ST` reader - "]
pub type HOST_GPIO_SDIO_INT_ST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit0_int_st(&self) -> HOST_SLC0_TOHOST_BIT0_INT_ST_R {
        HOST_SLC0_TOHOST_BIT0_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit1_int_st(&self) -> HOST_SLC0_TOHOST_BIT1_INT_ST_R {
        HOST_SLC0_TOHOST_BIT1_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit2_int_st(&self) -> HOST_SLC0_TOHOST_BIT2_INT_ST_R {
        HOST_SLC0_TOHOST_BIT2_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit3_int_st(&self) -> HOST_SLC0_TOHOST_BIT3_INT_ST_R {
        HOST_SLC0_TOHOST_BIT3_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit4_int_st(&self) -> HOST_SLC0_TOHOST_BIT4_INT_ST_R {
        HOST_SLC0_TOHOST_BIT4_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit5_int_st(&self) -> HOST_SLC0_TOHOST_BIT5_INT_ST_R {
        HOST_SLC0_TOHOST_BIT5_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit6_int_st(&self) -> HOST_SLC0_TOHOST_BIT6_INT_ST_R {
        HOST_SLC0_TOHOST_BIT6_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn host_slc0_tohost_bit7_int_st(&self) -> HOST_SLC0_TOHOST_BIT7_INT_ST_R {
        HOST_SLC0_TOHOST_BIT7_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn host_slc0_token0_1to0_int_st(&self) -> HOST_SLC0_TOKEN0_1TO0_INT_ST_R {
        HOST_SLC0_TOKEN0_1TO0_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn host_slc0_token1_1to0_int_st(&self) -> HOST_SLC0_TOKEN1_1TO0_INT_ST_R {
        HOST_SLC0_TOKEN1_1TO0_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn host_slc0_token0_0to1_int_st(&self) -> HOST_SLC0_TOKEN0_0TO1_INT_ST_R {
        HOST_SLC0_TOKEN0_0TO1_INT_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn host_slc0_token1_0to1_int_st(&self) -> HOST_SLC0_TOKEN1_0TO1_INT_ST_R {
        HOST_SLC0_TOKEN1_0TO1_INT_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn host_slc0host_rx_sof_int_st(&self) -> HOST_SLC0HOST_RX_SOF_INT_ST_R {
        HOST_SLC0HOST_RX_SOF_INT_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn host_slc0host_rx_eof_int_st(&self) -> HOST_SLC0HOST_RX_EOF_INT_ST_R {
        HOST_SLC0HOST_RX_EOF_INT_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn host_slc0host_rx_start_int_st(&self) -> HOST_SLC0HOST_RX_START_INT_ST_R {
        HOST_SLC0HOST_RX_START_INT_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn host_slc0host_tx_start_int_st(&self) -> HOST_SLC0HOST_TX_START_INT_ST_R {
        HOST_SLC0HOST_TX_START_INT_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn host_slc0_rx_udf_int_st(&self) -> HOST_SLC0_RX_UDF_INT_ST_R {
        HOST_SLC0_RX_UDF_INT_ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn host_slc0_tx_ovf_int_st(&self) -> HOST_SLC0_TX_OVF_INT_ST_R {
        HOST_SLC0_TX_OVF_INT_ST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn host_slc0_rx_pf_valid_int_st(&self) -> HOST_SLC0_RX_PF_VALID_INT_ST_R {
        HOST_SLC0_RX_PF_VALID_INT_ST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn host_slc0_ext_bit0_int_st(&self) -> HOST_SLC0_EXT_BIT0_INT_ST_R {
        HOST_SLC0_EXT_BIT0_INT_ST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn host_slc0_ext_bit1_int_st(&self) -> HOST_SLC0_EXT_BIT1_INT_ST_R {
        HOST_SLC0_EXT_BIT1_INT_ST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn host_slc0_ext_bit2_int_st(&self) -> HOST_SLC0_EXT_BIT2_INT_ST_R {
        HOST_SLC0_EXT_BIT2_INT_ST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn host_slc0_ext_bit3_int_st(&self) -> HOST_SLC0_EXT_BIT3_INT_ST_R {
        HOST_SLC0_EXT_BIT3_INT_ST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn host_slc0_rx_new_packet_int_st(&self) -> HOST_SLC0_RX_NEW_PACKET_INT_ST_R {
        HOST_SLC0_RX_NEW_PACKET_INT_ST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn host_slc0_host_rd_retry_int_st(&self) -> HOST_SLC0_HOST_RD_RETRY_INT_ST_R {
        HOST_SLC0_HOST_RD_RETRY_INT_ST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn host_gpio_sdio_int_st(&self) -> HOST_GPIO_SDIO_INT_ST_R {
        HOST_GPIO_SDIO_INT_ST_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slc0host_int_st](index.html) module"]
pub struct HOST_SLC0HOST_INT_ST_SPEC;
impl crate::RegisterSpec for HOST_SLC0HOST_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slc0host_int_st::R](R) reader structure"]
impl crate::Readable for HOST_SLC0HOST_INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HOST_SLC0HOST_INT_ST to value 0"]
impl crate::Resettable for HOST_SLC0HOST_INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
