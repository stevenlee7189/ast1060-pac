#[doc = "Register `GPIO570` reader"]
pub type R = crate::R<Gpio570Spec>;
#[doc = "Register `GPIO570` writer"]
pub type W = crate::W<Gpio570Spec>;
#[doc = "Field `DataWrittenToGPIO500` reader - Data written to GPIO500"]
pub type DataWrittenToGpio500R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data written to GPIO500"]
    #[inline(always)]
    pub fn data_written_to_gpio500(&self) -> DataWrittenToGpio500R {
        DataWrittenToGpio500R::new(self.bits)
    }
}
impl W {}
#[doc = "Serial GPIO\\_A/B/C/D 1 Data Read Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio570::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio570::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio570Spec;
impl crate::RegisterSpec for Gpio570Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio570::R`](R) reader structure"]
impl crate::Readable for Gpio570Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio570::W`](W) writer structure"]
impl crate::Writable for Gpio570Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO570 to value 0"]
impl crate::Resettable for Gpio570Spec {}
