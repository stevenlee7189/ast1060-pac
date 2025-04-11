#[doc = "Register `SECURE854` reader"]
pub type R = crate::R<Secure854Spec>;
#[doc = "Register `SECURE854` writer"]
pub type W = crate::W<Secure854Spec>;
#[doc = "Field `Reserved01` reader - Reserved(0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `SecBootHashSizeReg` reader - Secure Boot Hash Size Register"]
pub type SecBootHashSizeRegR = crate::FieldReader<u16>;
#[doc = "Field `SecBootHashSizeReg` writer - Secure Boot Hash Size Register"]
pub type SecBootHashSizeRegW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `Reserved0` reader - Reserved(0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:19 - Secure Boot Hash Size Register"]
    #[inline(always)]
    pub fn sec_boot_hash_size_reg(&self) -> SecBootHashSizeRegR {
        SecBootHashSizeRegR::new(((self.bits >> 4) & 0xffff) as u16)
    }
    #[doc = "Bits 20:31 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 4:19 - Secure Boot Hash Size Register"]
    #[inline(always)]
    pub fn sec_boot_hash_size_reg(&mut self) -> SecBootHashSizeRegW<Secure854Spec> {
        SecBootHashSizeRegW::new(self, 4)
    }
}
#[doc = "Secure Boot Hash Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure854::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure854::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure854Spec;
impl crate::RegisterSpec for Secure854Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure854::R`](R) reader structure"]
impl crate::Readable for Secure854Spec {}
#[doc = "`write(|w| ..)` method takes [`secure854::W`](W) writer structure"]
impl crate::Writable for Secure854Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE854 to value 0"]
impl crate::Resettable for Secure854Spec {}
