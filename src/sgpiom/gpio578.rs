#[doc = "Register `GPIO578` reader"]
pub type R = crate::R<Gpio578Spec>;
#[doc = "Register `GPIO578` writer"]
pub type W = crate::W<Gpio578Spec>;
#[doc = "Field `DataWrittenToGPIO538` reader - Data written to GPIO538"]
pub type DataWrittenToGpio538R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data written to GPIO538"]
    #[inline(always)]
    pub fn data_written_to_gpio538(&self) -> DataWrittenToGpio538R {
        DataWrittenToGpio538R::new(self.bits)
    }
}
impl W {}
#[doc = "Serial GPIO\\_I/J/K/L 1 Data Read Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio578::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio578::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio578Spec;
impl crate::RegisterSpec for Gpio578Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio578::R`](R) reader structure"]
impl crate::Readable for Gpio578Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio578::W`](W) writer structure"]
impl crate::Writable for Gpio578Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO578 to value 0"]
impl crate::Resettable for Gpio578Spec {}
