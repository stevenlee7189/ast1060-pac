#[doc = "Register `SECURE858` reader"]
pub type R = crate::R<Secure858Spec>;
#[doc = "Register `SECURE858` writer"]
pub type W = crate::W<Secure858Spec>;
#[doc = "Field `SecBootHashModeReg` reader - Secure Boot Hash Mode Register"]
pub type SecBootHashModeRegR = crate::FieldReader;
#[doc = "Field `SecBootHashModeReg` writer - Secure Boot Hash Mode Register"]
pub type SecBootHashModeRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Reserved0` reader - Reserved(0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - Secure Boot Hash Mode Register"]
    #[inline(always)]
    pub fn sec_boot_hash_mode_reg(&self) -> SecBootHashModeRegR {
        SecBootHashModeRegR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - Secure Boot Hash Mode Register"]
    #[inline(always)]
    pub fn sec_boot_hash_mode_reg(&mut self) -> SecBootHashModeRegW<Secure858Spec> {
        SecBootHashModeRegW::new(self, 0)
    }
}
#[doc = "Secure Boot Hash Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure858::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure858::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure858Spec;
impl crate::RegisterSpec for Secure858Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure858::R`](R) reader structure"]
impl crate::Readable for Secure858Spec {}
#[doc = "`write(|w| ..)` method takes [`secure858::W`](W) writer structure"]
impl crate::Writable for Secure858Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE858 to value 0"]
impl crate::Resettable for Secure858Spec {}
