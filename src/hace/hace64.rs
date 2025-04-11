#[doc = "Register `HACE64` reader"]
pub type R = crate::R<Hace64Spec>;
#[doc = "Register `HACE64` writer"]
pub type W = crate::W<Hace64Spec>;
#[doc = "Field `HACSwTagReg270` reader - HAC software tag register\\[27:0\\]"]
pub type HacswTagReg270R = crate::FieldReader<u32>;
#[doc = "Field `HACSwTagReg270` writer - HAC software tag register\\[27:0\\]"]
pub type HacswTagReg270W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `WaitHashEngIDLECtrl` reader - Wait Hash Engine IDLE Control"]
pub type WaitHashEngIdlectrlR = crate::BitReader;
#[doc = "Field `WaitHashEngIDLECtrl` writer - Wait Hash Engine IDLE Control"]
pub type WaitHashEngIdlectrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WaitCryptoEngIDLECtrl` reader - Wait Crypto Engine IDLE Control"]
pub type WaitCryptoEngIdlectrlR = crate::BitReader;
#[doc = "Field `WaitCryptoEngIDLECtrl` writer - Wait Crypto Engine IDLE Control"]
pub type WaitCryptoEngIdlectrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::BitReader;
#[doc = "Field `EnblWrSwTagINT` reader - Enable write software tag interrupt"]
pub type EnblWrSwTagIntR = crate::BitReader;
#[doc = "Field `EnblWrSwTagINT` writer - Enable write software tag interrupt"]
pub type EnblWrSwTagIntW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:27 - HAC software tag register\\[27:0\\]"]
    #[inline(always)]
    pub fn hacsw_tag_reg270(&self) -> HacswTagReg270R {
        HacswTagReg270R::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bit 28 - Wait Hash Engine IDLE Control"]
    #[inline(always)]
    pub fn wait_hash_eng_idlectrl(&self) -> WaitHashEngIdlectrlR {
        WaitHashEngIdlectrlR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Wait Crypto Engine IDLE Control"]
    #[inline(always)]
    pub fn wait_crypto_eng_idlectrl(&self) -> WaitCryptoEngIdlectrlR {
        WaitCryptoEngIdlectrlR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable write software tag interrupt"]
    #[inline(always)]
    pub fn enbl_wr_sw_tag_int(&self) -> EnblWrSwTagIntR {
        EnblWrSwTagIntR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:27 - HAC software tag register\\[27:0\\]"]
    #[inline(always)]
    pub fn hacsw_tag_reg270(&mut self) -> HacswTagReg270W<Hace64Spec> {
        HacswTagReg270W::new(self, 0)
    }
    #[doc = "Bit 28 - Wait Hash Engine IDLE Control"]
    #[inline(always)]
    pub fn wait_hash_eng_idlectrl(&mut self) -> WaitHashEngIdlectrlW<Hace64Spec> {
        WaitHashEngIdlectrlW::new(self, 28)
    }
    #[doc = "Bit 29 - Wait Crypto Engine IDLE Control"]
    #[inline(always)]
    pub fn wait_crypto_eng_idlectrl(&mut self) -> WaitCryptoEngIdlectrlW<Hace64Spec> {
        WaitCryptoEngIdlectrlW::new(self, 29)
    }
    #[doc = "Bit 31 - Enable write software tag interrupt"]
    #[inline(always)]
    pub fn enbl_wr_sw_tag_int(&mut self) -> EnblWrSwTagIntW<Hace64Spec> {
        EnblWrSwTagIntW::new(self, 31)
    }
}
#[doc = "HAC Software Tag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hace64::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace64::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hace64Spec;
impl crate::RegisterSpec for Hace64Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hace64::R`](R) reader structure"]
impl crate::Readable for Hace64Spec {}
#[doc = "`write(|w| ..)` method takes [`hace64::W`](W) writer structure"]
impl crate::Writable for Hace64Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HACE64 to value 0"]
impl crate::Resettable for Hace64Spec {}
