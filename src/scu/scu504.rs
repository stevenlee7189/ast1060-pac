#[doc = "Register `SCU504` reader"]
pub type R = crate::R<Scu504Spec>;
#[doc = "Register `SCU504` writer"]
pub type W = crate::W<Scu504Spec>;
#[doc = "Field `SCU500HwStrap1ClearReg` reader - SCU500 Hardware Strap1 Clear Register"]
pub type Scu500hwStrap1clearRegR = crate::FieldReader<u32>;
#[doc = "Field `SCU500HwStrap1ClearReg` writer - SCU500 Hardware Strap1 Clear Register"]
pub type Scu500hwStrap1clearRegW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU500 Hardware Strap1 Clear Register"]
    #[inline(always)]
    pub fn scu500hw_strap1clear_reg(&self) -> Scu500hwStrap1clearRegR {
        Scu500hwStrap1clearRegR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU500 Hardware Strap1 Clear Register"]
    #[inline(always)]
    pub fn scu500hw_strap1clear_reg(&mut self) -> Scu500hwStrap1clearRegW<Scu504Spec> {
        Scu500hwStrap1clearRegW::new(self, 0)
    }
}
#[doc = "Hardware Strap1 Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu504::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu504::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu504Spec;
impl crate::RegisterSpec for Scu504Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu504::R`](R) reader structure"]
impl crate::Readable for Scu504Spec {}
#[doc = "`write(|w| ..)` method takes [`scu504::W`](W) writer structure"]
impl crate::Writable for Scu504Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU504 to value 0"]
impl crate::Resettable for Scu504Spec {}
