#[doc = "Register `SECURE8AC` reader"]
pub type R = crate::R<Secure8acSpec>;
#[doc = "Register `SECURE8AC` writer"]
pub type W = crate::W<Secure8acSpec>;
#[doc = "Field `SecBootCryptoKeyBuffer3Reg` reader - Secure Boot Crypto Key Buffer 3 Register"]
pub type SecBootCryptoKeyBuffer3regR = crate::FieldReader<u32>;
#[doc = "Field `SecBootCryptoKeyBuffer3Reg` writer - Secure Boot Crypto Key Buffer 3 Register"]
pub type SecBootCryptoKeyBuffer3regW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot Crypto Key Buffer 3 Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_key_buffer3reg(&self) -> SecBootCryptoKeyBuffer3regR {
        SecBootCryptoKeyBuffer3regR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot Crypto Key Buffer 3 Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_key_buffer3reg(&mut self) -> SecBootCryptoKeyBuffer3regW<Secure8acSpec> {
        SecBootCryptoKeyBuffer3regW::new(self, 0)
    }
}
#[doc = "Secure Boot Crypto Key Buffer 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure8ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure8ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure8acSpec;
impl crate::RegisterSpec for Secure8acSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure8ac::R`](R) reader structure"]
impl crate::Readable for Secure8acSpec {}
#[doc = "`write(|w| ..)` method takes [`secure8ac::W`](W) writer structure"]
impl crate::Writable for Secure8acSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE8AC to value 0"]
impl crate::Resettable for Secure8acSpec {}
