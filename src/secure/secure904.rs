#[doc = "Register `SECURE904` reader"]
pub type R = crate::R<Secure904Spec>;
#[doc = "Register `SECURE904` writer"]
pub type W = crate::W<Secure904Spec>;
#[doc = "Field `SecBootFirstVaultKey1Reg` reader - Secure Boot First Vault Key 1 Register"]
pub type SecBootFirstVaultKey1regR = crate::FieldReader<u32>;
#[doc = "Field `SecBootFirstVaultKey1Reg` writer - Secure Boot First Vault Key 1 Register"]
pub type SecBootFirstVaultKey1regW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot First Vault Key 1 Register"]
    #[inline(always)]
    pub fn sec_boot_first_vault_key1reg(&self) -> SecBootFirstVaultKey1regR {
        SecBootFirstVaultKey1regR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot First Vault Key 1 Register"]
    #[inline(always)]
    pub fn sec_boot_first_vault_key1reg(&mut self) -> SecBootFirstVaultKey1regW<Secure904Spec> {
        SecBootFirstVaultKey1regW::new(self, 0)
    }
}
#[doc = "Secure Boot First Vault Key 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure904::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure904::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure904Spec;
impl crate::RegisterSpec for Secure904Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure904::R`](R) reader structure"]
impl crate::Readable for Secure904Spec {}
#[doc = "`write(|w| ..)` method takes [`secure904::W`](W) writer structure"]
impl crate::Writable for Secure904Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE904 to value 0"]
impl crate::Resettable for Secure904Spec {}
