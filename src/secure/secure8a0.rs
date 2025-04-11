#[doc = "Register `SECURE8A0` reader"]
pub type R = crate::R<Secure8a0Spec>;
#[doc = "Register `SECURE8A0` writer"]
pub type W = crate::W<Secure8a0Spec>;
#[doc = "Field `SecBootCryptoKeyBuffer0Reg` reader - Secure Boot Crypto Key Buffer 0 Register"]
pub type SecBootCryptoKeyBuffer0regR = crate::FieldReader<u32>;
#[doc = "Field `SecBootCryptoKeyBuffer0Reg` writer - Secure Boot Crypto Key Buffer 0 Register"]
pub type SecBootCryptoKeyBuffer0regW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot Crypto Key Buffer 0 Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_key_buffer0reg(&self) -> SecBootCryptoKeyBuffer0regR {
        SecBootCryptoKeyBuffer0regR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot Crypto Key Buffer 0 Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_key_buffer0reg(&mut self) -> SecBootCryptoKeyBuffer0regW<Secure8a0Spec> {
        SecBootCryptoKeyBuffer0regW::new(self, 0)
    }
}
#[doc = "Secure Boot Crypto Key Buffer 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure8a0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure8a0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure8a0Spec;
impl crate::RegisterSpec for Secure8a0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure8a0::R`](R) reader structure"]
impl crate::Readable for Secure8a0Spec {}
#[doc = "`write(|w| ..)` method takes [`secure8a0::W`](W) writer structure"]
impl crate::Writable for Secure8a0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE8A0 to value 0"]
impl crate::Resettable for Secure8a0Spec {}
