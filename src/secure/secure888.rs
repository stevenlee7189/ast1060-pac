#[doc = "Register `SECURE888` reader"]
pub type R = crate::R<Secure888Spec>;
#[doc = "Register `SECURE888` writer"]
pub type W = crate::W<Secure888Spec>;
#[doc = "Field `SecBootCryptoDataBuffer2Reg` reader - Secure Boot Crypto Data Buffer 2 Register"]
pub type SecBootCryptoDataBuffer2regR = crate::FieldReader<u32>;
#[doc = "Field `SecBootCryptoDataBuffer2Reg` writer - Secure Boot Crypto Data Buffer 2 Register"]
pub type SecBootCryptoDataBuffer2regW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot Crypto Data Buffer 2 Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_data_buffer2reg(&self) -> SecBootCryptoDataBuffer2regR {
        SecBootCryptoDataBuffer2regR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot Crypto Data Buffer 2 Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_data_buffer2reg(
        &mut self,
    ) -> SecBootCryptoDataBuffer2regW<Secure888Spec> {
        SecBootCryptoDataBuffer2regW::new(self, 0)
    }
}
#[doc = "Secure Boot Crypto Data Buffer 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure888::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure888::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure888Spec;
impl crate::RegisterSpec for Secure888Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure888::R`](R) reader structure"]
impl crate::Readable for Secure888Spec {}
#[doc = "`write(|w| ..)` method takes [`secure888::W`](W) writer structure"]
impl crate::Writable for Secure888Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE888 to value 0"]
impl crate::Resettable for Secure888Spec {}
