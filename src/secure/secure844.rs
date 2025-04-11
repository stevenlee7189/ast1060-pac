#[doc = "Register `SECURE844` reader"]
pub type R = crate::R<Secure844Spec>;
#[doc = "Register `SECURE844` writer"]
pub type W = crate::W<Secure844Spec>;
#[doc = "Field `Reserved0` reader - Reserved(0)"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `SecBootDMADestinationAddrReg` reader - Secure Boot DMA Destination Address Register"]
pub type SecBootDmadestinationAddrRegR = crate::FieldReader<u32>;
#[doc = "Field `SecBootDMADestinationAddrReg` writer - Secure Boot DMA Destination Address Register"]
pub type SecBootDmadestinationAddrRegW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - Secure Boot DMA Destination Address Register"]
    #[inline(always)]
    pub fn sec_boot_dmadestination_addr_reg(&self) -> SecBootDmadestinationAddrRegR {
        SecBootDmadestinationAddrRegR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Secure Boot DMA Destination Address Register"]
    #[inline(always)]
    pub fn sec_boot_dmadestination_addr_reg(
        &mut self,
    ) -> SecBootDmadestinationAddrRegW<Secure844Spec> {
        SecBootDmadestinationAddrRegW::new(self, 2)
    }
}
#[doc = "Secure Boot DMA Destination Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure844::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure844::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure844Spec;
impl crate::RegisterSpec for Secure844Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure844::R`](R) reader structure"]
impl crate::Readable for Secure844Spec {}
#[doc = "`write(|w| ..)` method takes [`secure844::W`](W) writer structure"]
impl crate::Writable for Secure844Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE844 to value 0"]
impl crate::Resettable for Secure844Spec {}
