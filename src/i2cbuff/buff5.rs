#[doc = "Register `BUFF5` reader"]
pub type R = crate::R<Buff5Spec>;
#[doc = "Register `BUFF5` writer"]
pub type W = crate::W<Buff5Spec>;
#[doc = "Field `Data` reader - data"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `Data` writer - data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<Buff5Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "I2C Buffer Mode Buff5\n\nYou can [`read`](crate::Reg::read) this register and get [`buff5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buff5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Buff5Spec;
impl crate::RegisterSpec for Buff5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buff5::R`](R) reader structure"]
impl crate::Readable for Buff5Spec {}
#[doc = "`write(|w| ..)` method takes [`buff5::W`](W) writer structure"]
impl crate::Writable for Buff5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUFF5 to value 0"]
impl crate::Resettable for Buff5Spec {}
