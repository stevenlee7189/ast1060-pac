#[doc = "Register `SECURE87C` reader"]
pub type R = crate::R<Secure87cSpec>;
#[doc = "Register `SECURE87C` writer"]
pub type W = crate::W<Secure87cSpec>;
#[doc = "Field `SecBootCryptoEngFireReg` reader - Secure Boot Crypto Engine Fire Register"]
pub type SecBootCryptoEngFireRegR = crate::FieldReader<u32>;
#[doc = "Field `SecBootCryptoEngFireReg` writer - Secure Boot Crypto Engine Fire Register"]
pub type SecBootCryptoEngFireRegW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot Crypto Engine Fire Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_eng_fire_reg(&self) -> SecBootCryptoEngFireRegR {
        SecBootCryptoEngFireRegR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot Crypto Engine Fire Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_eng_fire_reg(&mut self) -> SecBootCryptoEngFireRegW<Secure87cSpec> {
        SecBootCryptoEngFireRegW::new(self, 0)
    }
}
#[doc = "Secure Boot Crypto Engine Fire Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure87c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure87c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure87cSpec;
impl crate::RegisterSpec for Secure87cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure87c::R`](R) reader structure"]
impl crate::Readable for Secure87cSpec {}
#[doc = "`write(|w| ..)` method takes [`secure87c::W`](W) writer structure"]
impl crate::Writable for Secure87cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE87C to value 0"]
impl crate::Resettable for Secure87cSpec {}
