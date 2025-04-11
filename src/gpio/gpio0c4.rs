#[doc = "Register `GPIO0C4` reader"]
pub type R = crate::R<Gpio0c4Spec>;
#[doc = "Register `GPIO0C4` writer"]
pub type W = crate::W<Gpio0c4Spec>;
#[doc = "Field `DataWrittenToGPIO020GPIOHGPIOGGPIOFGPIOE` reader - Data written to GPIO020.(GPIOH/GPIOG/GPIOF/GPIOE)"]
pub type DataWrittenToGpio020gpiohgpioggpiofgpioeR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data written to GPIO020.(GPIOH/GPIOG/GPIOF/GPIOE)"]
    #[inline(always)]
    pub fn data_written_to_gpio020gpiohgpioggpiofgpioe(
        &self,
    ) -> DataWrittenToGpio020gpiohgpioggpiofgpioeR {
        DataWrittenToGpio020gpiohgpioggpiofgpioeR::new(self.bits)
    }
}
impl W {}
#[doc = "GPIO\\_E/F/G/H Data Read Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0c4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0c4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio0c4Spec;
impl crate::RegisterSpec for Gpio0c4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio0c4::R`](R) reader structure"]
impl crate::Readable for Gpio0c4Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio0c4::W`](W) writer structure"]
impl crate::Writable for Gpio0c4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO0C4 to value 0"]
impl crate::Resettable for Gpio0c4Spec {}
