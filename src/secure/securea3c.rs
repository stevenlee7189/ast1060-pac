#[doc = "Register `SECUREA3C` reader"]
pub type R = crate::R<Securea3cSpec>;
#[doc = "Register `SECUREA3C` writer"]
pub type W = crate::W<Securea3cSpec>;
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
    ) -> SecBootEccdefaultPublicKeyReadBackW<Securea3cSpec> {
        SecBootEccdefaultPublicKeyReadBackW::new(self, 0)
    }
}
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`securea3c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`securea3c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Securea3cSpec;
impl crate::RegisterSpec for Securea3cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`securea3c::R`](R) reader structure"]
impl crate::Readable for Securea3cSpec {}
#[doc = "`write(|w| ..)` method takes [`securea3c::W`](W) writer structure"]
impl crate::Writable for Securea3cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECUREA3C to value 0"]
impl crate::Resettable for Securea3cSpec {}
