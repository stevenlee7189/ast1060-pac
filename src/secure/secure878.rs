#[doc = "Register `SECURE878` reader"]
pub type R = crate::R<Secure878Spec>;
#[doc = "Register `SECURE878` writer"]
pub type W = crate::W<Secure878Spec>;
#[doc = "Field `SecBootCryptoVectorWrTriggerReg` reader - Secure Boot Crypto Vector Write Trigger Register"]
pub type SecBootCryptoVectorWrTriggerRegR = crate::FieldReader<u32>;
#[doc = "Field `SecBootCryptoVectorWrTriggerReg` writer - Secure Boot Crypto Vector Write Trigger Register"]
pub type SecBootCryptoVectorWrTriggerRegW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot Crypto Vector Write Trigger Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_vector_wr_trigger_reg(&self) -> SecBootCryptoVectorWrTriggerRegR {
        SecBootCryptoVectorWrTriggerRegR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot Crypto Vector Write Trigger Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_vector_wr_trigger_reg(
        &mut self,
    ) -> SecBootCryptoVectorWrTriggerRegW<Secure878Spec> {
        SecBootCryptoVectorWrTriggerRegW::new(self, 0)
    }
}
#[doc = "Secure Boot Crypto Vector Write Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure878::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure878::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure878Spec;
impl crate::RegisterSpec for Secure878Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure878::R`](R) reader structure"]
impl crate::Readable for Secure878Spec {}
#[doc = "`write(|w| ..)` method takes [`secure878::W`](W) writer structure"]
impl crate::Writable for Secure878Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE878 to value 0"]
impl crate::Resettable for Secure878Spec {}
