#[doc = "Register `SECURE85C` reader"]
pub type R = crate::R<Secure85cSpec>;
#[doc = "Register `SECURE85C` writer"]
pub type W = crate::W<Secure85cSpec>;
#[doc = "Field `SecBootHashEngFireReg` reader - Secure Boot Hash Engine Fire Register"]
pub type SecBootHashEngFireRegR = crate::FieldReader<u32>;
#[doc = "Field `SecBootHashEngFireReg` writer - Secure Boot Hash Engine Fire Register"]
pub type SecBootHashEngFireRegW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot Hash Engine Fire Register"]
    #[inline(always)]
    pub fn sec_boot_hash_eng_fire_reg(&self) -> SecBootHashEngFireRegR {
        SecBootHashEngFireRegR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot Hash Engine Fire Register"]
    #[inline(always)]
    pub fn sec_boot_hash_eng_fire_reg(&mut self) -> SecBootHashEngFireRegW<Secure85cSpec> {
        SecBootHashEngFireRegW::new(self, 0)
    }
}
#[doc = "Secure Boot Hash Engine Fire Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure85c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure85c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure85cSpec;
impl crate::RegisterSpec for Secure85cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure85c::R`](R) reader structure"]
impl crate::Readable for Secure85cSpec {}
#[doc = "`write(|w| ..)` method takes [`secure85c::W`](W) writer structure"]
impl crate::Writable for Secure85cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE85C to value 0"]
impl crate::Resettable for Secure85cSpec {}
