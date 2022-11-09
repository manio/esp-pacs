#[doc = "Register `DBUS0_ACS_WB_CNT` reader"]
pub struct R(crate::R<DBUS0_ACS_WB_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBUS0_ACS_WB_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBUS0_ACS_WB_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBUS0_ACS_WB_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DBUS0_ACS_WB_CNT` reader - The bits are used to count the number of cache evictions by dbus0 access cache."]
pub type DBUS0_ACS_WB_CNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:19 - The bits are used to count the number of cache evictions by dbus0 access cache."]
    #[inline(always)]
    pub fn dbus0_acs_wb_cnt(&self) -> DBUS0_ACS_WB_CNT_R {
        DBUS0_ACS_WB_CNT_R::new(self.bits & 0x000f_ffff)
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbus0_acs_wb_cnt](index.html) module"]
pub struct DBUS0_ACS_WB_CNT_SPEC;
impl crate::RegisterSpec for DBUS0_ACS_WB_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbus0_acs_wb_cnt::R](R) reader structure"]
impl crate::Readable for DBUS0_ACS_WB_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DBUS0_ACS_WB_CNT to value 0"]
impl crate::Resettable for DBUS0_ACS_WB_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
