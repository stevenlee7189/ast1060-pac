#[doc = "Register `I3CD0EC` reader"]
pub type R = crate::R<I3cd0ecSpec>;
#[doc = "Register `I3CD0EC` writer"]
pub type W = crate::W<I3cd0ecSpec>;
#[doc = "Field `IBIMAXPAYLOADSIZE` reader - IBI_MAX_PAYLOAD_SIZE"]
pub type IbimaxpayloadsizeR = crate::FieldReader;
#[doc = "Field `IBIPAYLOADSIZE` reader - IBI_PAYLOAD_SIZE"]
pub type IbipayloadsizeR = crate::FieldReader;
#[doc = "Field `IBIPAYLOADSIZE` writer - IBI_PAYLOAD_SIZE"]
pub type IbipayloadsizeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - IBI_MAX_PAYLOAD_SIZE"]
    #[inline(always)]
    pub fn ibimaxpayloadsize(&self) -> IbimaxpayloadsizeR {
        IbimaxpayloadsizeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - IBI_PAYLOAD_SIZE"]
    #[inline(always)]
    pub fn ibipayloadsize(&self) -> IbipayloadsizeR {
        IbipayloadsizeR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - IBI_PAYLOAD_SIZE"]
    #[inline(always)]
    pub fn ibipayloadsize(&mut self) -> IbipayloadsizeW<I3cd0ecSpec> {
        IbipayloadsizeW::new(self, 16)
    }
}
#[doc = "I3C IBI Payload Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd0ec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd0ec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd0ecSpec;
impl crate::RegisterSpec for I3cd0ecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd0ec::R`](R) reader structure"]
impl crate::Readable for I3cd0ecSpec {}
#[doc = "`write(|w| ..)` method takes [`i3cd0ec::W`](W) writer structure"]
impl crate::Writable for I3cd0ecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD0EC to value 0"]
impl crate::Resettable for I3cd0ecSpec {}
