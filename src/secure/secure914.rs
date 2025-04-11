#[doc = "Register `SECURE914` reader"]
pub type R = crate::R<Secure914Spec>;
#[doc = "Register `SECURE914` writer"]
pub type W = crate::W<Secure914Spec>;
#[doc = "Field `SecBootFirstVaultKey5Reg` reader - Secure Boot First Vault Key 5 Register"]
pub type SecBootFirstVaultKey5regR = crate::FieldReader<u32>;
#[doc = "Field `SecBootFirstVaultKey5Reg` writer - Secure Boot First Vault Key 5 Register"]
pub type SecBootFirstVaultKey5regW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot First Vault Key 5 Register"]
    #[inline(always)]
    pub fn sec_boot_first_vault_key5reg(&self) -> SecBootFirstVaultKey5regR {
        SecBootFirstVaultKey5regR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot First Vault Key 5 Register"]
    #[inline(always)]
    pub fn sec_boot_first_vault_key5reg(&mut self) -> SecBootFirstVaultKey5regW<Secure914Spec> {
        SecBootFirstVaultKey5regW::new(self, 0)
    }
}
#[doc = "Secure Boot First Vault Key 5 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure914::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure914::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure914Spec;
impl crate::RegisterSpec for Secure914Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure914::R`](R) reader structure"]
impl crate::Readable for Secure914Spec {}
#[doc = "`write(|w| ..)` method takes [`secure914::W`](W) writer structure"]
impl crate::Writable for Secure914Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE914 to value 0"]
impl crate::Resettable for Secure914Spec {}
