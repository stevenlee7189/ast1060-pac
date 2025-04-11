#[doc = "Register `I3CD05C` reader"]
pub type R = crate::R<I3cd05cSpec>;
#[doc = "Register `I3CD05C` writer"]
pub type W = crate::W<I3cd05cSpec>;
#[doc = "Field `PDEVADDRTABLESTARTADDR` reader - P_DEV_ADDR_TABLE_START_ADDR"]
pub type PdevaddrtablestartaddrR = crate::FieldReader<u16>;
#[doc = "Field `DEVADDRTABLEDEPTH` reader - DEV_ADDR_TABLE_DEPTH"]
pub type DevaddrtabledepthR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - P_DEV_ADDR_TABLE_START_ADDR"]
    #[inline(always)]
    pub fn pdevaddrtablestartaddr(&self) -> PdevaddrtablestartaddrR {
        PdevaddrtablestartaddrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - DEV_ADDR_TABLE_DEPTH"]
    #[inline(always)]
    pub fn devaddrtabledepth(&self) -> DevaddrtabledepthR {
        DevaddrtabledepthR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Pointer for Device Address Table Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd05c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd05c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd05cSpec;
impl crate::RegisterSpec for I3cd05cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd05c::R`](R) reader structure"]
impl crate::Readable for I3cd05cSpec {}
#[doc = "`write(|w| ..)` method takes [`i3cd05c::W`](W) writer structure"]
impl crate::Writable for I3cd05cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD05C to value 0"]
impl crate::Resettable for I3cd05cSpec {}
