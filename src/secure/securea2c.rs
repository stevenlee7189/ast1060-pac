#[doc = "Register `SECUREA2C` reader"]
pub type R = crate::R<Securea2cSpec>;
#[doc = "Register `SECUREA2C` writer"]
pub type W = crate::W<Securea2cSpec>;
#[doc = "Field `SecBootECCDefaultPublicKeyReadBack` reader - Secure Boot ECC Default Public Key Read Back"]
pub type SecBootEccdefaultPublicKeyReadBackR = crate::FieldReader<u32>;
#[doc = "Field `SecBootECCDefaultPublicKeyReadBack` writer - Secure Boot ECC Default Public Key Read Back"]
pub type SecBootEccdefaultPublicKeyReadBackW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot ECC Default Public Key Read Back"]
    #[inline(always)]
    pub fn sec_boot_eccdefault_public_key_read_back(&self) -> SecBootEccdefaultPublicKeyReadBackR {
        SecBootEccdefaultPublicKeyReadBackR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot ECC Default Public Key Read Back"]
    #[inline(always)]
    pub fn sec_boot_eccdefault_public_key_read_back(
        &mut self,
    ) -> SecBootEccdefaultPublicKeyReadBackW<Securea2cSpec> {
        SecBootEccdefaultPublicKeyReadBackW::new(self, 0)
    }
}
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea2c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea2c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Securea2cSpec;
impl crate::RegisterSpec for Securea2cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`securea2c::R`](R) reader structure"]
impl crate::Readable for Securea2cSpec {}
#[doc = "`write(|w| ..)` method takes [`securea2c::W`](W) writer structure"]
impl crate::Writable for Securea2cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECUREA2C to value 0"]
impl crate::Resettable for Securea2cSpec {}
