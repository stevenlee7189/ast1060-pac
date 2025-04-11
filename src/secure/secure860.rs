#[doc = "Register `SECURE860` reader"]
pub type R = crate::R<Secure860Spec>;
#[doc = "Register `SECURE860` writer"]
pub type W = crate::W<Secure860Spec>;
#[doc = "Field `SecBootCryptoModeReg` reader - Secure Boot Crypto Mode Register"]
pub type SecBootCryptoModeRegR = crate::FieldReader;
#[doc = "Field `SecBootCryptoModeReg` writer - Secure Boot Crypto Mode Register"]
pub type SecBootCryptoModeRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Reserved0` reader - Reserved(0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - Secure Boot Crypto Mode Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_mode_reg(&self) -> SecBootCryptoModeRegR {
        SecBootCryptoModeRegR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - Secure Boot Crypto Mode Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_mode_reg(&mut self) -> SecBootCryptoModeRegW<Secure860Spec> {
        SecBootCryptoModeRegW::new(self, 0)
    }
}
#[doc = "Secure Boot Crypto Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure860::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure860::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure860Spec;
impl crate::RegisterSpec for Secure860Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure860::R`](R) reader structure"]
impl crate::Readable for Secure860Spec {}
#[doc = "`write(|w| ..)` method takes [`secure860::W`](W) writer structure"]
impl crate::Writable for Secure860Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE860 to value 0"]
impl crate::Resettable for Secure860Spec {}
