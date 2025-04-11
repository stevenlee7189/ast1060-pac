#[doc = "Register `I3CD030` reader"]
pub type R = crate::R<I3cd030Spec>;
#[doc = "Register `I3CD030` writer"]
pub type W = crate::W<I3cd030Spec>;
#[doc = "Field `InbandSlaveINTReqReject` reader - In-band Slave Interrupt Request Reject."]
pub type InbandSlaveIntreqRejectR = crate::FieldReader<u32>;
#[doc = "Field `InbandSlaveINTReqReject` writer - In-band Slave Interrupt Request Reject."]
pub type InbandSlaveIntreqRejectW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - In-band Slave Interrupt Request Reject."]
    #[inline(always)]
    pub fn inband_slave_intreq_reject(&self) -> InbandSlaveIntreqRejectR {
        InbandSlaveIntreqRejectR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - In-band Slave Interrupt Request Reject."]
    #[inline(always)]
    pub fn inband_slave_intreq_reject(&mut self) -> InbandSlaveIntreqRejectW<I3cd030Spec> {
        InbandSlaveIntreqRejectW::new(self, 0)
    }
}
#[doc = "IBI SIR Request Rejection Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd030::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd030::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd030Spec;
impl crate::RegisterSpec for I3cd030Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd030::R`](R) reader structure"]
impl crate::Readable for I3cd030Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd030::W`](W) writer structure"]
impl crate::Writable for I3cd030Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD030 to value 0"]
impl crate::Resettable for I3cd030Spec {}
