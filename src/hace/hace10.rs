#[doc = "Register `HACE10` reader"]
pub type R = crate::R<Hace10Spec>;
#[doc = "Register `HACE10` writer"]
pub type W = crate::W<Hace10Spec>;
#[doc = "Field `CryptoEngOpModeCtrl` reader - Crypto engine operation mode control"]
pub type CryptoEngOpModeCtrlR = crate::FieldReader;
#[doc = "Field `CryptoEngOpModeCtrl` writer - Crypto engine operation mode control"]
pub type CryptoEngOpModeCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `KeyLenOfAESCryptoAlg` reader - Key length of AES crypto algorithm"]
pub type KeyLenOfAescryptoAlgR = crate::FieldReader;
#[doc = "Field `KeyLenOfAESCryptoAlg` writer - Key length of AES crypto algorithm"]
pub type KeyLenOfAescryptoAlgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AESDESOpModeSel` reader - AES/DES operation mode selection"]
pub type AesdesopModeSelR = crate::FieldReader;
#[doc = "Field `AESDESOpModeSel` writer - AES/DES operation mode selection"]
pub type AesdesopModeSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CryptoModeSel` reader - Crypto mode selection"]
pub type CryptoModeSelR = crate::BitReader;
#[doc = "Field `CryptoModeSel` writer - Crypto mode selection"]
pub type CryptoModeSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CryptoAlgSel` reader - Crypto algorithm selection"]
pub type CryptoAlgSelR = crate::BitReader;
#[doc = "Field `CryptoAlgSel` writer - Crypto algorithm selection"]
pub type CryptoAlgSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisSavingCtxDataIntoCtxBuf` reader - Disable saving context data into context buffer"]
pub type DisSavingCtxDataIntoCtxBufR = crate::BitReader;
#[doc = "Field `DisSavingCtxDataIntoCtxBuf` writer - Disable saving context data into context buffer"]
pub type DisSavingCtxDataIntoCtxBufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblCryptoINT` reader - Enable crypto interrupt"]
pub type EnblCryptoIntR = crate::BitReader;
#[doc = "Field `EnblCryptoINT` writer - Enable crypto interrupt"]
pub type EnblCryptoIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESKeyExpansionSel` reader - AES key expansion selection"]
pub type AeskeyExpansionSelR = crate::BitReader;
#[doc = "Field `AESKeyExpansionSel` writer - AES key expansion selection"]
pub type AeskeyExpansionSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved03` reader - Reserved (0)"]
pub type Reserved03R = crate::BitReader;
#[doc = "Field `AESDESCTRModeCtrl` reader - AES/DES CTR mode control"]
pub type AesdesctrmodeCtrlR = crate::FieldReader;
#[doc = "Field `AESDESCTRModeCtrl` writer - AES/DES CTR mode control"]
pub type AesdesctrmodeCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AESDESEngSel` reader - AES/DES engine selection"]
pub type AesdesengSelR = crate::BitReader;
#[doc = "Field `AESDESEngSel` writer - AES/DES engine selection"]
pub type AesdesengSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblTripleDES` reader - Enable triple DES"]
pub type EnblTripleDesR = crate::BitReader;
#[doc = "Field `EnblTripleDES` writer - Enable triple DES"]
pub type EnblTripleDesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CryptoSrcDataSgCtrl` reader - Crypto source data scatter-gather control"]
pub type CryptoSrcDataSgCtrlR = crate::BitReader;
#[doc = "Field `CryptoSrcDataSgCtrl` writer - Crypto source data scatter-gather control"]
pub type CryptoSrcDataSgCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved02` reader - Reserved (0)"]
pub type Reserved02R = crate::FieldReader;
#[doc = "Field `CryptoDestDataSgCtrl` reader - Crypto destination data scatter-gather control"]
pub type CryptoDestDataSgCtrlR = crate::BitReader;
#[doc = "Field `CryptoDestDataSgCtrl` writer - Crypto destination data scatter-gather control"]
pub type CryptoDestDataSgCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBusReqSyncForCryptoEngIdle` reader - M-Bus request synchronization for Crypto Engine Idle"]
pub type MbusReqSyncForCryptoEngIdleR = crate::BitReader;
#[doc = "Field `MBusReqSyncForCryptoEngIdle` writer - M-Bus request synchronization for Crypto Engine Idle"]
pub type MbusReqSyncForCryptoEngIdleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESGCMTagWrAddrSel` reader - AES-GCM Tag write address selection"]
pub type AesgcmtagWrAddrSelR = crate::BitReader;
#[doc = "Field `AESGCMTagWrAddrSel` writer - AES-GCM Tag write address selection"]
pub type AesgcmtagWrAddrSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `GHASHPaddingBlockLenFieldSel` reader - GHASH padding block length field selection"]
pub type GhashpaddingBlockLenFieldSelR = crate::BitReader;
#[doc = "Field `GHASHPaddingBlockLenFieldSel` writer - GHASH padding block length field selection"]
pub type GhashpaddingBlockLenFieldSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GHASHTagOutputXORCtrlBit` reader - GHASH Tag output XOR control bit"]
pub type GhashtagOutputXorctrlBitR = crate::BitReader;
#[doc = "Field `GHASHTagOutputXORCtrlBit` writer - GHASH Tag output XOR control bit"]
pub type GhashtagOutputXorctrlBitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AESKeySrcSel` reader - AES Key source selection"]
pub type AeskeySrcSelR = crate::BitReader;
#[doc = "Field `AESKeySrcSel` writer - AES Key source selection"]
pub type AeskeySrcSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Crypto engine operation mode control"]
    #[inline(always)]
    pub fn crypto_eng_op_mode_ctrl(&self) -> CryptoEngOpModeCtrlR {
        CryptoEngOpModeCtrlR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Key length of AES crypto algorithm"]
    #[inline(always)]
    pub fn key_len_of_aescrypto_alg(&self) -> KeyLenOfAescryptoAlgR {
        KeyLenOfAescryptoAlgR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - AES/DES operation mode selection"]
    #[inline(always)]
    pub fn aesdesop_mode_sel(&self) -> AesdesopModeSelR {
        AesdesopModeSelR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Crypto mode selection"]
    #[inline(always)]
    pub fn crypto_mode_sel(&self) -> CryptoModeSelR {
        CryptoModeSelR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Crypto algorithm selection"]
    #[inline(always)]
    pub fn crypto_alg_sel(&self) -> CryptoAlgSelR {
        CryptoAlgSelR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Disable saving context data into context buffer"]
    #[inline(always)]
    pub fn dis_saving_ctx_data_into_ctx_buf(&self) -> DisSavingCtxDataIntoCtxBufR {
        DisSavingCtxDataIntoCtxBufR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable crypto interrupt"]
    #[inline(always)]
    pub fn enbl_crypto_int(&self) -> EnblCryptoIntR {
        EnblCryptoIntR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AES key expansion selection"]
    #[inline(always)]
    pub fn aeskey_expansion_sel(&self) -> AeskeyExpansionSelR {
        AeskeyExpansionSelR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 13 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved03(&self) -> Reserved03R {
        Reserved03R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - AES/DES CTR mode control"]
    #[inline(always)]
    pub fn aesdesctrmode_ctrl(&self) -> AesdesctrmodeCtrlR {
        AesdesctrmodeCtrlR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - AES/DES engine selection"]
    #[inline(always)]
    pub fn aesdeseng_sel(&self) -> AesdesengSelR {
        AesdesengSelR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable triple DES"]
    #[inline(always)]
    pub fn enbl_triple_des(&self) -> EnblTripleDesR {
        EnblTripleDesR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Crypto source data scatter-gather control"]
    #[inline(always)]
    pub fn crypto_src_data_sg_ctrl(&self) -> CryptoSrcDataSgCtrlR {
        CryptoSrcDataSgCtrlR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved02(&self) -> Reserved02R {
        Reserved02R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 19 - Crypto destination data scatter-gather control"]
    #[inline(always)]
    pub fn crypto_dest_data_sg_ctrl(&self) -> CryptoDestDataSgCtrlR {
        CryptoDestDataSgCtrlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - M-Bus request synchronization for Crypto Engine Idle"]
    #[inline(always)]
    pub fn mbus_req_sync_for_crypto_eng_idle(&self) -> MbusReqSyncForCryptoEngIdleR {
        MbusReqSyncForCryptoEngIdleR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - AES-GCM Tag write address selection"]
    #[inline(always)]
    pub fn aesgcmtag_wr_addr_sel(&self) -> AesgcmtagWrAddrSelR {
        AesgcmtagWrAddrSelR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 21:23 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 22 - GHASH padding block length field selection"]
    #[inline(always)]
    pub fn ghashpadding_block_len_field_sel(&self) -> GhashpaddingBlockLenFieldSelR {
        GhashpaddingBlockLenFieldSelR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GHASH Tag output XOR control bit"]
    #[inline(always)]
    pub fn ghashtag_output_xorctrl_bit(&self) -> GhashtagOutputXorctrlBitR {
        GhashtagOutputXorctrlBitR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - AES Key source selection"]
    #[inline(always)]
    pub fn aeskey_src_sel(&self) -> AeskeySrcSelR {
        AeskeySrcSelR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Crypto engine operation mode control"]
    #[inline(always)]
    pub fn crypto_eng_op_mode_ctrl(&mut self) -> CryptoEngOpModeCtrlW<Hace10Spec> {
        CryptoEngOpModeCtrlW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Key length of AES crypto algorithm"]
    #[inline(always)]
    pub fn key_len_of_aescrypto_alg(&mut self) -> KeyLenOfAescryptoAlgW<Hace10Spec> {
        KeyLenOfAescryptoAlgW::new(self, 2)
    }
    #[doc = "Bits 4:6 - AES/DES operation mode selection"]
    #[inline(always)]
    pub fn aesdesop_mode_sel(&mut self) -> AesdesopModeSelW<Hace10Spec> {
        AesdesopModeSelW::new(self, 4)
    }
    #[doc = "Bit 7 - Crypto mode selection"]
    #[inline(always)]
    pub fn crypto_mode_sel(&mut self) -> CryptoModeSelW<Hace10Spec> {
        CryptoModeSelW::new(self, 7)
    }
    #[doc = "Bit 8 - Crypto algorithm selection"]
    #[inline(always)]
    pub fn crypto_alg_sel(&mut self) -> CryptoAlgSelW<Hace10Spec> {
        CryptoAlgSelW::new(self, 8)
    }
    #[doc = "Bit 9 - Disable saving context data into context buffer"]
    #[inline(always)]
    pub fn dis_saving_ctx_data_into_ctx_buf(&mut self) -> DisSavingCtxDataIntoCtxBufW<Hace10Spec> {
        DisSavingCtxDataIntoCtxBufW::new(self, 9)
    }
    #[doc = "Bit 10 - reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Hace10Spec> {
        Reserved1W::new(self, 10)
    }
    #[doc = "Bit 12 - Enable crypto interrupt"]
    #[inline(always)]
    pub fn enbl_crypto_int(&mut self) -> EnblCryptoIntW<Hace10Spec> {
        EnblCryptoIntW::new(self, 12)
    }
    #[doc = "Bit 13 - AES key expansion selection"]
    #[inline(always)]
    pub fn aeskey_expansion_sel(&mut self) -> AeskeyExpansionSelW<Hace10Spec> {
        AeskeyExpansionSelW::new(self, 13)
    }
    #[doc = "Bits 14:15 - AES/DES CTR mode control"]
    #[inline(always)]
    pub fn aesdesctrmode_ctrl(&mut self) -> AesdesctrmodeCtrlW<Hace10Spec> {
        AesdesctrmodeCtrlW::new(self, 14)
    }
    #[doc = "Bit 16 - AES/DES engine selection"]
    #[inline(always)]
    pub fn aesdeseng_sel(&mut self) -> AesdesengSelW<Hace10Spec> {
        AesdesengSelW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable triple DES"]
    #[inline(always)]
    pub fn enbl_triple_des(&mut self) -> EnblTripleDesW<Hace10Spec> {
        EnblTripleDesW::new(self, 17)
    }
    #[doc = "Bit 18 - Crypto source data scatter-gather control"]
    #[inline(always)]
    pub fn crypto_src_data_sg_ctrl(&mut self) -> CryptoSrcDataSgCtrlW<Hace10Spec> {
        CryptoSrcDataSgCtrlW::new(self, 18)
    }
    #[doc = "Bit 19 - Crypto destination data scatter-gather control"]
    #[inline(always)]
    pub fn crypto_dest_data_sg_ctrl(&mut self) -> CryptoDestDataSgCtrlW<Hace10Spec> {
        CryptoDestDataSgCtrlW::new(self, 19)
    }
    #[doc = "Bit 20 - M-Bus request synchronization for Crypto Engine Idle"]
    #[inline(always)]
    pub fn mbus_req_sync_for_crypto_eng_idle(
        &mut self,
    ) -> MbusReqSyncForCryptoEngIdleW<Hace10Spec> {
        MbusReqSyncForCryptoEngIdleW::new(self, 20)
    }
    #[doc = "Bit 21 - AES-GCM Tag write address selection"]
    #[inline(always)]
    pub fn aesgcmtag_wr_addr_sel(&mut self) -> AesgcmtagWrAddrSelW<Hace10Spec> {
        AesgcmtagWrAddrSelW::new(self, 21)
    }
    #[doc = "Bit 22 - GHASH padding block length field selection"]
    #[inline(always)]
    pub fn ghashpadding_block_len_field_sel(
        &mut self,
    ) -> GhashpaddingBlockLenFieldSelW<Hace10Spec> {
        GhashpaddingBlockLenFieldSelW::new(self, 22)
    }
    #[doc = "Bit 23 - GHASH Tag output XOR control bit"]
    #[inline(always)]
    pub fn ghashtag_output_xorctrl_bit(&mut self) -> GhashtagOutputXorctrlBitW<Hace10Spec> {
        GhashtagOutputXorctrlBitW::new(self, 23)
    }
    #[doc = "Bit 24 - AES Key source selection"]
    #[inline(always)]
    pub fn aeskey_src_sel(&mut self) -> AeskeySrcSelW<Hace10Spec> {
        AeskeySrcSelW::new(self, 24)
    }
}
#[doc = "Crypto Engine Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hace10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hace10Spec;
impl crate::RegisterSpec for Hace10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hace10::R`](R) reader structure"]
impl crate::Readable for Hace10Spec {}
#[doc = "`write(|w| ..)` method takes [`hace10::W`](W) writer structure"]
impl crate::Writable for Hace10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HACE10 to value 0"]
impl crate::Resettable for Hace10Spec {}
