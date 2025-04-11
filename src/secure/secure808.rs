#[doc = "Register `SECURE808` reader"]
pub type R = crate::R<Secure808Spec>;
#[doc = "Register `SECURE808` writer"]
pub type W = crate::W<Secure808Spec>;
#[doc = "Field `SecBootDMAModeReg` reader - Secure Boot DMA Mode Register"]
pub type SecBootDmamodeRegR = crate::FieldReader;
#[doc = "Field `SecBootDMAModeReg` writer - Secure Boot DMA Mode Register"]
pub type SecBootDmamodeRegW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Reserved0` reader - Reserved(0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:5 - Secure Boot DMA Mode Register"]
    #[inline(always)]
    pub fn sec_boot_dmamode_reg(&self) -> SecBootDmamodeRegR {
        SecBootDmamodeRegR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:31 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - Secure Boot DMA Mode Register"]
    #[inline(always)]
    pub fn sec_boot_dmamode_reg(&mut self) -> SecBootDmamodeRegW<Secure808Spec> {
        SecBootDmamodeRegW::new(self, 0)
    }
}
#[doc = "Secure Boot DMA Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure808::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure808::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure808Spec;
impl crate::RegisterSpec for Secure808Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure808::R`](R) reader structure"]
impl crate::Readable for Secure808Spec {}
#[doc = "`write(|w| ..)` method takes [`secure808::W`](W) writer structure"]
impl crate::Writable for Secure808Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE808 to value 0"]
impl crate::Resettable for Secure808Spec {}
