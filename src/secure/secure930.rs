#[doc = "Register `SECURE930` reader"]
pub type R = crate::R<Secure930Spec>;
#[doc = "Register `SECURE930` writer"]
pub type W = crate::W<Secure930Spec>;
#[doc = "Field `SecBootSecondVaultKey4Reg` reader - Secure Boot Second Vault Key 4 Register"]
pub type SecBootSecondVaultKey4regR = crate::FieldReader<u32>;
#[doc = "Field `SecBootSecondVaultKey4Reg` writer - Secure Boot Second Vault Key 4 Register"]
pub type SecBootSecondVaultKey4regW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot Second Vault Key 4 Register"]
    #[inline(always)]
    pub fn sec_boot_second_vault_key4reg(&self) -> SecBootSecondVaultKey4regR {
        SecBootSecondVaultKey4regR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot Second Vault Key 4 Register"]
    #[inline(always)]
    pub fn sec_boot_second_vault_key4reg(&mut self) -> SecBootSecondVaultKey4regW<Secure930Spec> {
        SecBootSecondVaultKey4regW::new(self, 0)
    }
}
#[doc = "Secure Boot Second Vault Key 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure930::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure930::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure930Spec;
impl crate::RegisterSpec for Secure930Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure930::R`](R) reader structure"]
impl crate::Readable for Secure930Spec {}
#[doc = "`write(|w| ..)` method takes [`secure930::W`](W) writer structure"]
impl crate::Writable for Secure930Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE930 to value 0"]
impl crate::Resettable for Secure930Spec {}
