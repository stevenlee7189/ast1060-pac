#[doc = "Register `FMC08C` reader"]
pub type R = crate::R<Fmc08cSpec>;
#[doc = "Register `FMC08C` writer"]
pub type W = crate::W<Fmc08cSpec>;
#[doc = "Field `DMALength` reader - DMA Length"]
pub type DmalengthR = crate::FieldReader<u32>;
#[doc = "Field `DMALength` writer - DMA Length"]
pub type DmalengthW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:24 - DMA Length"]
    #[inline(always)]
    pub fn dmalength(&self) -> DmalengthR {
        DmalengthR::new(self.bits & 0x01ff_ffff)
    }
    #[doc = "Bits 25:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:24 - DMA Length"]
    #[inline(always)]
    pub fn dmalength(&mut self) -> DmalengthW<Fmc08cSpec> {
        DmalengthW::new(self, 0)
    }
}
#[doc = "DMA Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc08c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc08c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc08cSpec;
impl crate::RegisterSpec for Fmc08cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc08c::R`](R) reader structure"]
impl crate::Readable for Fmc08cSpec {}
#[doc = "`write(|w| ..)` method takes [`fmc08c::W`](W) writer structure"]
impl crate::Writable for Fmc08cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC08C to value 0"]
impl crate::Resettable for Fmc08cSpec {}
