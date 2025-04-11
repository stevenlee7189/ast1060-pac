#[doc = "Register `SECURE928` reader"]
pub type R = crate::R<Secure928Spec>;
#[doc = "Register `SECURE928` writer"]
pub type W = crate::W<Secure928Spec>;
#[doc = "Field `SecBootSecondVaultKey2Reg` reader - Secure Boot Second Vault Key 2 Register"]
pub type SecBootSecondVaultKey2regR = crate::FieldReader<u32>;
#[doc = "Field `SecBootSecondVaultKey2Reg` writer - Secure Boot Second Vault Key 2 Register"]
pub type SecBootSecondVaultKey2regW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot Second Vault Key 2 Register"]
    #[inline(always)]
    pub fn sec_boot_second_vault_key2reg(&self) -> SecBootSecondVaultKey2regR {
        SecBootSecondVaultKey2regR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot Second Vault Key 2 Register"]
    #[inline(always)]
    pub fn sec_boot_second_vault_key2reg(&mut self) -> SecBootSecondVaultKey2regW<Secure928Spec> {
        SecBootSecondVaultKey2regW::new(self, 0)
    }
}
#[doc = "Secure Boot Second Vault Key 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure928::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure928::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure928Spec;
impl crate::RegisterSpec for Secure928Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure928::R`](R) reader structure"]
impl crate::Readable for Secure928Spec {}
#[doc = "`write(|w| ..)` method takes [`secure928::W`](W) writer structure"]
impl crate::Writable for Secure928Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE928 to value 0"]
impl crate::Resettable for Secure928Spec {}
