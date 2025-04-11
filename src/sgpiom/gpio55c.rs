#[doc = "Register `GPIO55C` reader"]
pub type R = crate::R<Gpio55cSpec>;
#[doc = "Register `GPIO55C` writer"]
pub type W = crate::W<Gpio55cSpec>;
#[doc = "Field `PortGPIOE70InputMask` reader - Port GPIOE\\[7:0\\] input mask"]
pub type PortGpioe70inputMaskR = crate::FieldReader;
#[doc = "Field `PortGPIOE70InputMask` writer - Port GPIOE\\[7:0\\] input mask"]
pub type PortGpioe70inputMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOF70InputMask` reader - Port GPIOF\\[7:0\\] input mask"]
pub type PortGpiof70inputMaskR = crate::FieldReader;
#[doc = "Field `PortGPIOF70InputMask` writer - Port GPIOF\\[7:0\\] input mask"]
pub type PortGpiof70inputMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOG70InputMask` reader - Port GPIOG\\[7:0\\] input mask"]
pub type PortGpiog70inputMaskR = crate::FieldReader;
#[doc = "Field `PortGPIOG70InputMask` writer - Port GPIOG\\[7:0\\] input mask"]
pub type PortGpiog70inputMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOH70InputMask` reader - Port GPIOH\\[7:0\\] input mask"]
pub type PortGpioh70inputMaskR = crate::FieldReader;
#[doc = "Field `PortGPIOH70InputMask` writer - Port GPIOH\\[7:0\\] input mask"]
pub type PortGpioh70inputMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOE\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpioe70input_mask(&self) -> PortGpioe70inputMaskR {
        PortGpioe70inputMaskR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOF\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpiof70input_mask(&self) -> PortGpiof70inputMaskR {
        PortGpiof70inputMaskR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOG\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpiog70input_mask(&self) -> PortGpiog70inputMaskR {
        PortGpiog70inputMaskR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOH\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpioh70input_mask(&self) -> PortGpioh70inputMaskR {
        PortGpioh70inputMaskR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOE\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpioe70input_mask(&mut self) -> PortGpioe70inputMaskW<Gpio55cSpec> {
        PortGpioe70inputMaskW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOF\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpiof70input_mask(&mut self) -> PortGpiof70inputMaskW<Gpio55cSpec> {
        PortGpiof70inputMaskW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOG\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpiog70input_mask(&mut self) -> PortGpiog70inputMaskW<Gpio55cSpec> {
        PortGpiog70inputMaskW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOH\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpioh70input_mask(&mut self) -> PortGpioh70inputMaskW<Gpio55cSpec> {
        PortGpioh70inputMaskW::new(self, 24)
    }
}
#[doc = "Serial GPIO\\_E/F/G/H Input Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio55c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio55c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio55cSpec;
impl crate::RegisterSpec for Gpio55cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio55c::R`](R) reader structure"]
impl crate::Readable for Gpio55cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio55c::W`](W) writer structure"]
impl crate::Writable for Gpio55cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO55C to value 0"]
impl crate::Resettable for Gpio55cSpec {}
