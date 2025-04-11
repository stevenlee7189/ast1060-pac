#[doc = "Register `HACE30` reader"]
pub type R = crate::R<Hace30Spec>;
#[doc = "Register `HACE30` writer"]
pub type W = crate::W<Hace30Spec>;
#[doc = "Field `HashEngOpModeCtrl` reader - Hash engine operation mode control"]
pub type HashEngOpModeCtrlR = crate::FieldReader;
#[doc = "Field `HashEngOpModeCtrl` writer - Hash engine operation mode control"]
pub type HashEngOpModeCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ByteSwappingCtrl` reader - Byte swapping control"]
pub type ByteSwappingCtrlR = crate::FieldReader;
#[doc = "Field `ByteSwappingCtrl` writer - Byte swapping control"]
pub type ByteSwappingCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HashAlgSel` reader - Hash algorithm selection"]
pub type HashAlgSelR = crate::FieldReader;
#[doc = "Field `HashAlgSel` writer - Hash algorithm selection"]
pub type HashAlgSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HMACEngCmdMode` reader - HMAC engine command mode"]
pub type HmacengCmdModeR = crate::FieldReader;
#[doc = "Field `HMACEngCmdMode` writer - HMAC engine command mode"]
pub type HmacengCmdModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EnblHashINT` reader - Enable hash interrupt"]
pub type EnblHashIntR = crate::BitReader;
#[doc = "Field `EnblHashINT` writer - Enable hash interrupt"]
pub type EnblHashIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHA512SeriesAlgSel` reader - SHA-512 series algorithm selection"]
pub type Sha512seriesAlgSelR = crate::FieldReader;
#[doc = "Field `SHA512SeriesAlgSel` writer - SHA-512 series algorithm selection"]
pub type Sha512seriesAlgSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FirstBlockForTheAccumMode` reader - First block for the accumulative mode"]
pub type FirstBlockForTheAccumModeR = crate::BitReader;
#[doc = "Field `FirstBlockForTheAccumMode` writer - First block for the accumulative mode"]
pub type FirstBlockForTheAccumModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved04` reader - Reserved (0)"]
pub type Reserved04R = crate::FieldReader;
#[doc = "Field `LastBlockForAccumMode` reader - Last block for accumulative mode"]
pub type LastBlockForAccumModeR = crate::BitReader;
#[doc = "Field `LastBlockForAccumMode` writer - Last block for accumulative mode"]
pub type LastBlockForAccumModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved02` reader - Reserved (0)"]
pub type Reserved02R = crate::FieldReader;
#[doc = "Field `Reserved03` reader - Reserved (0)"]
pub type Reserved03R = crate::FieldReader;
#[doc = "Field `HashSrcDataSgCtrl` reader - Hash source data scatter-gather control"]
pub type HashSrcDataSgCtrlR = crate::BitReader;
#[doc = "Field `HashSrcDataSgCtrl` writer - Hash source data scatter-gather control"]
pub type HashSrcDataSgCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::BitReader;
#[doc = "Field `MBusReqSyncForHashEngIdle` reader - M-Bus request synchronization for Hash Engine Idle"]
pub type MbusReqSyncForHashEngIdleR = crate::BitReader;
#[doc = "Field `MBusReqSyncForHashEngIdle` writer - M-Bus request synchronization for Hash Engine Idle"]
pub type MbusReqSyncForHashEngIdleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:1 - Hash engine operation mode control"]
    #[inline(always)]
    pub fn hash_eng_op_mode_ctrl(&self) -> HashEngOpModeCtrlR {
        HashEngOpModeCtrlR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Byte swapping control"]
    #[inline(always)]
    pub fn byte_swapping_ctrl(&self) -> ByteSwappingCtrlR {
        ByteSwappingCtrlR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Hash algorithm selection"]
    #[inline(always)]
    pub fn hash_alg_sel(&self) -> HashAlgSelR {
        HashAlgSelR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:8 - HMAC engine command mode"]
    #[inline(always)]
    pub fn hmaceng_cmd_mode(&self) -> HmacengCmdModeR {
        HmacengCmdModeR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - Enable hash interrupt"]
    #[inline(always)]
    pub fn enbl_hash_int(&self) -> EnblHashIntR {
        EnblHashIntR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - SHA-512 series algorithm selection"]
    #[inline(always)]
    pub fn sha512series_alg_sel(&self) -> Sha512seriesAlgSelR {
        Sha512seriesAlgSelR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 13 - First block for the accumulative mode"]
    #[inline(always)]
    pub fn first_block_for_the_accum_mode(&self) -> FirstBlockForTheAccumModeR {
        FirstBlockForTheAccumModeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved04(&self) -> Reserved04R {
        Reserved04R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 14 - Last block for accumulative mode"]
    #[inline(always)]
    pub fn last_block_for_accum_mode(&self) -> LastBlockForAccumModeR {
        LastBlockForAccumModeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:17 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved02(&self) -> Reserved02R {
        Reserved02R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 15:19 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved03(&self) -> Reserved03R {
        Reserved03R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bit 18 - Hash source data scatter-gather control"]
    #[inline(always)]
    pub fn hash_src_data_sg_ctrl(&self) -> HashSrcDataSgCtrlR {
        HashSrcDataSgCtrlR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - M-Bus request synchronization for Hash Engine Idle"]
    #[inline(always)]
    pub fn mbus_req_sync_for_hash_eng_idle(&self) -> MbusReqSyncForHashEngIdleR {
        MbusReqSyncForHashEngIdleR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Hash engine operation mode control"]
    #[inline(always)]
    pub fn hash_eng_op_mode_ctrl(&mut self) -> HashEngOpModeCtrlW<Hace30Spec> {
        HashEngOpModeCtrlW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Byte swapping control"]
    #[inline(always)]
    pub fn byte_swapping_ctrl(&mut self) -> ByteSwappingCtrlW<Hace30Spec> {
        ByteSwappingCtrlW::new(self, 2)
    }
    #[doc = "Bits 4:6 - Hash algorithm selection"]
    #[inline(always)]
    pub fn hash_alg_sel(&mut self) -> HashAlgSelW<Hace30Spec> {
        HashAlgSelW::new(self, 4)
    }
    #[doc = "Bits 7:8 - HMAC engine command mode"]
    #[inline(always)]
    pub fn hmaceng_cmd_mode(&mut self) -> HmacengCmdModeW<Hace30Spec> {
        HmacengCmdModeW::new(self, 7)
    }
    #[doc = "Bit 9 - Enable hash interrupt"]
    #[inline(always)]
    pub fn enbl_hash_int(&mut self) -> EnblHashIntW<Hace30Spec> {
        EnblHashIntW::new(self, 9)
    }
    #[doc = "Bits 10:12 - SHA-512 series algorithm selection"]
    #[inline(always)]
    pub fn sha512series_alg_sel(&mut self) -> Sha512seriesAlgSelW<Hace30Spec> {
        Sha512seriesAlgSelW::new(self, 10)
    }
    #[doc = "Bit 13 - First block for the accumulative mode"]
    #[inline(always)]
    pub fn first_block_for_the_accum_mode(&mut self) -> FirstBlockForTheAccumModeW<Hace30Spec> {
        FirstBlockForTheAccumModeW::new(self, 13)
    }
    #[doc = "Bit 14 - Last block for accumulative mode"]
    #[inline(always)]
    pub fn last_block_for_accum_mode(&mut self) -> LastBlockForAccumModeW<Hace30Spec> {
        LastBlockForAccumModeW::new(self, 14)
    }
    #[doc = "Bit 18 - Hash source data scatter-gather control"]
    #[inline(always)]
    pub fn hash_src_data_sg_ctrl(&mut self) -> HashSrcDataSgCtrlW<Hace30Spec> {
        HashSrcDataSgCtrlW::new(self, 18)
    }
    #[doc = "Bit 20 - M-Bus request synchronization for Hash Engine Idle"]
    #[inline(always)]
    pub fn mbus_req_sync_for_hash_eng_idle(&mut self) -> MbusReqSyncForHashEngIdleW<Hace30Spec> {
        MbusReqSyncForHashEngIdleW::new(self, 20)
    }
}
#[doc = "Hash Engine Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hace30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hace30Spec;
impl crate::RegisterSpec for Hace30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hace30::R`](R) reader structure"]
impl crate::Readable for Hace30Spec {}
#[doc = "`write(|w| ..)` method takes [`hace30::W`](W) writer structure"]
impl crate::Writable for Hace30Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HACE30 to value 0"]
impl crate::Resettable for Hace30Spec {}
