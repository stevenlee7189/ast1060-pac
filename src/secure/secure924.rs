#[doc = "Register `SECURE924` reader"]
pub type R = crate::R<Secure924Spec>;
#[doc = "Register `SECURE924` writer"]
pub type W = crate::W<Secure924Spec>;
#[doc = "Field `SecBootSecondVaultKey1Reg` reader - Secure Boot Second Vault Key 1 Register"]
pub type SecBootSecondVaultKey1regR = crate::FieldReader<u32>;
#[doc = "Field `SecBootSecondVaultKey1Reg` writer - Secure Boot Second Vault Key 1 Register"]
pub type SecBootSecondVaultKey1regW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot Second Vault Key 1 Register"]
    #[inline(always)]
    pub fn sec_boot_second_vault_key1reg(&self) -> SecBootSecondVaultKey1regR {
        SecBootSecondVaultKey1regR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot Second Vault Key 1 Register"]
    #[inline(always)]
    pub fn sec_boot_second_vault_key1reg(&mut self) -> SecBootSecondVaultKey1regW<Secure924Spec> {
        SecBootSecondVaultKey1regW::new(self, 0)
    }
}
#[doc = "Secure Boot Second Vault Key 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure924::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure924::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure924Spec;
impl crate::RegisterSpec for Secure924Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure924::R`](R) reader structure"]
impl crate::Readable for Secure924Spec {}
#[doc = "`write(|w| ..)` method takes [`secure924::W`](W) writer structure"]
impl crate::Writable for Secure924Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE924 to value 0"]
impl crate::Resettable for Secure924Spec {}
