#[doc = "Register `GPIO144` reader"]
pub type R = crate::R<Gpio144Spec>;
#[doc = "Register `GPIO144` writer"]
pub type W = crate::W<Gpio144Spec>;
#[doc = "Field `PortGPIU70CmdSource1` reader - Port GPIU\\[7:0\\] Command Source 1"]
pub type PortGpiu70cmdSource1R = crate::BitReader;
#[doc = "Field `PortGPIU70CmdSource1` writer - Port GPIU\\[7:0\\] Command Source 1"]
pub type PortGpiu70cmdSource1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port GPIU\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpiu70cmd_source1(&self) -> PortGpiu70cmdSource1R {
        PortGpiu70cmdSource1R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port GPIU\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpiu70cmd_source1(&mut self) -> PortGpiu70cmdSource1W<Gpio144Spec> {
        PortGpiu70cmdSource1W::new(self, 0)
    }
}
#[doc = "GPIO\\_U Command Source 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio144::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio144::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio144Spec;
impl crate::RegisterSpec for Gpio144Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio144::R`](R) reader structure"]
impl crate::Readable for Gpio144Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio144::W`](W) writer structure"]
impl crate::Writable for Gpio144Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO144 to value 0"]
impl crate::Resettable for Gpio144Spec {}
