#[doc = "Register `SECURE908` reader"]
pub type R = crate::R<Secure908Spec>;
#[doc = "Register `SECURE908` writer"]
pub type W = crate::W<Secure908Spec>;
#[doc = "Field `SecBootFirstVaultKey2Reg` reader - Secure Boot First Vault Key 2 Register"]
pub type SecBootFirstVaultKey2regR = crate::FieldReader<u32>;
#[doc = "Field `SecBootFirstVaultKey2Reg` writer - Secure Boot First Vault Key 2 Register"]
pub type SecBootFirstVaultKey2regW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot First Vault Key 2 Register"]
    #[inline(always)]
    pub fn sec_boot_first_vault_key2reg(&self) -> SecBootFirstVaultKey2regR {
        SecBootFirstVaultKey2regR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot First Vault Key 2 Register"]
    #[inline(always)]
    pub fn sec_boot_first_vault_key2reg(&mut self) -> SecBootFirstVaultKey2regW<Secure908Spec> {
        SecBootFirstVaultKey2regW::new(self, 0)
    }
}
#[doc = "Secure Boot First Vault Key 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure908::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure908::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure908Spec;
impl crate::RegisterSpec for Secure908Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure908::R`](R) reader structure"]
impl crate::Readable for Secure908Spec {}
#[doc = "`write(|w| ..)` method takes [`secure908::W`](W) writer structure"]
impl crate::Writable for Secure908Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE908 to value 0"]
impl crate::Resettable for Secure908Spec {}
