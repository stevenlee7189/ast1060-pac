#[doc = "Register `SECURE93C` reader"]
pub type R = crate::R<Secure93cSpec>;
#[doc = "Register `SECURE93C` writer"]
pub type W = crate::W<Secure93cSpec>;
#[doc = "Field `SecBootSecondVaultKey7Reg` reader - Secure Boot Second Vault Key 7 Register"]
pub type SecBootSecondVaultKey7regR = crate::FieldReader<u32>;
#[doc = "Field `SecBootSecondVaultKey7Reg` writer - Secure Boot Second Vault Key 7 Register"]
pub type SecBootSecondVaultKey7regW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot Second Vault Key 7 Register"]
    #[inline(always)]
    pub fn sec_boot_second_vault_key7reg(&self) -> SecBootSecondVaultKey7regR {
        SecBootSecondVaultKey7regR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot Second Vault Key 7 Register"]
    #[inline(always)]
    pub fn sec_boot_second_vault_key7reg(&mut self) -> SecBootSecondVaultKey7regW<Secure93cSpec> {
        SecBootSecondVaultKey7regW::new(self, 0)
    }
}
#[doc = "Secure Boot Second Vault Key 7 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure93c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure93c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure93cSpec;
impl crate::RegisterSpec for Secure93cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure93c::R`](R) reader structure"]
impl crate::Readable for Secure93cSpec {}
#[doc = "`write(|w| ..)` method takes [`secure93c::W`](W) writer structure"]
impl crate::Writable for Secure93cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE93C to value 0"]
impl crate::Resettable for Secure93cSpec {}
