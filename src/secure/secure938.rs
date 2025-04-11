#[doc = "Register `SECURE938` reader"]
pub type R = crate::R<Secure938Spec>;
#[doc = "Register `SECURE938` writer"]
pub type W = crate::W<Secure938Spec>;
#[doc = "Field `SecBootSecondVaultKey6Reg` reader - Secure Boot Second Vault Key 6 Register"]
pub type SecBootSecondVaultKey6regR = crate::FieldReader<u32>;
#[doc = "Field `SecBootSecondVaultKey6Reg` writer - Secure Boot Second Vault Key 6 Register"]
pub type SecBootSecondVaultKey6regW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot Second Vault Key 6 Register"]
    #[inline(always)]
    pub fn sec_boot_second_vault_key6reg(&self) -> SecBootSecondVaultKey6regR {
        SecBootSecondVaultKey6regR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot Second Vault Key 6 Register"]
    #[inline(always)]
    pub fn sec_boot_second_vault_key6reg(&mut self) -> SecBootSecondVaultKey6regW<Secure938Spec> {
        SecBootSecondVaultKey6regW::new(self, 0)
    }
}
#[doc = "Secure Boot Second Vault Key 6 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure938::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure938::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure938Spec;
impl crate::RegisterSpec for Secure938Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure938::R`](R) reader structure"]
impl crate::Readable for Secure938Spec {}
#[doc = "`write(|w| ..)` method takes [`secure938::W`](W) writer structure"]
impl crate::Writable for Secure938Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE938 to value 0"]
impl crate::Resettable for Secure938Spec {}
