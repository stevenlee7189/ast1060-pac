#[doc = "Register `SECUREAB4` reader"]
pub type R = crate::R<Secureab4Spec>;
#[doc = "Register `SECUREAB4` writer"]
pub type W = crate::W<Secureab4Spec>;
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
    ) -> SecBootEccdefaultPublicKeyReadBackW<Secureab4Spec> {
        SecBootEccdefaultPublicKeyReadBackW::new(self, 0)
    }
}
#[doc = "Secure Boot ECC Default Public Key Read Back \\#0 - \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`secureab4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secureab4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secureab4Spec;
impl crate::RegisterSpec for Secureab4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secureab4::R`](R) reader structure"]
impl crate::Readable for Secureab4Spec {}
#[doc = "`write(|w| ..)` method takes [`secureab4::W`](W) writer structure"]
impl crate::Writable for Secureab4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECUREAB4 to value 0"]
impl crate::Resettable for Secureab4Spec {}
