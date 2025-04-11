#[doc = "Register `SCU0D8` reader"]
pub type R = crate::R<Scu0d8Spec>;
#[doc = "Register `SCU0D8` writer"]
pub type W = crate::W<Scu0d8Spec>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ForceDisableUARTDebugPortFn` reader - Force disable UART debug port function"]
pub type ForceDisableUartdebugPortFnR = crate::BitReader;
#[doc = "Field `ForceDisableUARTDebugPortFn` writer - Force disable UART debug port function"]
pub type ForceDisableUartdebugPortFnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EwverAAForceDisableSysRstSigOfWDT` reader - ewverAAForce disable system reset signal of WDT"]
pub type EwverAaforceDisableSysRstSigOfWdtR = crate::BitReader;
#[doc = "Field `EwverAAForceDisableSysRstSigOfWDT` writer - ewverAAForce disable system reset signal of WDT"]
pub type EwverAaforceDisableSysRstSigOfWdtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EwverAADisSBMCUDebugCapability` reader - ewverAADisable SBMCU debug capability"]
pub type EwverAadisSbmcudebugCapabilityR = crate::BitReader;
#[doc = "Field `EwverAADisSBMCUDebugCapability` writer - ewverAADisable SBMCU debug capability"]
pub type EwverAadisSbmcudebugCapabilityW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Force disable UART debug port function"]
    #[inline(always)]
    pub fn force_disable_uartdebug_port_fn(&self) -> ForceDisableUartdebugPortFnR {
        ForceDisableUartdebugPortFnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ewverAAForce disable system reset signal of WDT"]
    #[inline(always)]
    pub fn ewver_aaforce_disable_sys_rst_sig_of_wdt(&self) -> EwverAaforceDisableSysRstSigOfWdtR {
        EwverAaforceDisableSysRstSigOfWdtR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ewverAADisable SBMCU debug capability"]
    #[inline(always)]
    pub fn ewver_aadis_sbmcudebug_capability(&self) -> EwverAadisSbmcudebugCapabilityR {
        EwverAadisSbmcudebugCapabilityR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Scu0d8Spec> {
        Reserved1W::new(self, 0)
    }
    #[doc = "Bit 3 - Force disable UART debug port function"]
    #[inline(always)]
    pub fn force_disable_uartdebug_port_fn(&mut self) -> ForceDisableUartdebugPortFnW<Scu0d8Spec> {
        ForceDisableUartdebugPortFnW::new(self, 3)
    }
    #[doc = "Bit 4 - ewverAAForce disable system reset signal of WDT"]
    #[inline(always)]
    pub fn ewver_aaforce_disable_sys_rst_sig_of_wdt(
        &mut self,
    ) -> EwverAaforceDisableSysRstSigOfWdtW<Scu0d8Spec> {
        EwverAaforceDisableSysRstSigOfWdtW::new(self, 4)
    }
    #[doc = "Bit 5 - ewverAADisable SBMCU debug capability"]
    #[inline(always)]
    pub fn ewver_aadis_sbmcudebug_capability(
        &mut self,
    ) -> EwverAadisSbmcudebugCapabilityW<Scu0d8Spec> {
        EwverAadisSbmcudebugCapabilityW::new(self, 5)
    }
}
#[doc = "Debug Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu0d8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu0d8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu0d8Spec;
impl crate::RegisterSpec for Scu0d8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu0d8::R`](R) reader structure"]
impl crate::Readable for Scu0d8Spec {}
#[doc = "`write(|w| ..)` method takes [`scu0d8::W`](W) writer structure"]
impl crate::Writable for Scu0d8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU0D8 to value 0"]
impl crate::Resettable for Scu0d8Spec {}
