#[doc = "Register `GPIO0CC` reader"]
pub type R = crate::R<Gpio0ccSpec>;
#[doc = "Register `GPIO0CC` writer"]
pub type W = crate::W<Gpio0ccSpec>;
#[doc = "Field `DataWrittenToGPIO078GPIOPGPIOOGPIONGPIOM` reader - Data written to GPIO078.(GPIOP/GPIOO/GPION/GPIOM)"]
pub type DataWrittenToGpio078gpiopgpioogpiongpiomR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data written to GPIO078.(GPIOP/GPIOO/GPION/GPIOM)"]
    #[inline(always)]
    pub fn data_written_to_gpio078gpiopgpioogpiongpiom(
        &self,
    ) -> DataWrittenToGpio078gpiopgpioogpiongpiomR {
        DataWrittenToGpio078gpiopgpioogpiongpiomR::new(self.bits)
    }
}
impl W {}
#[doc = "GPIO\\_M/N/O/P Data Read Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio0ccSpec;
impl crate::RegisterSpec for Gpio0ccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio0cc::R`](R) reader structure"]
impl crate::Readable for Gpio0ccSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio0cc::W`](W) writer structure"]
impl crate::Writable for Gpio0ccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO0CC to value 0"]
impl crate::Resettable for Gpio0ccSpec {}
