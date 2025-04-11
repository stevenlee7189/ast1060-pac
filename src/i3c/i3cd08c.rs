#[doc = "Register `I3CD08C` reader"]
pub type R = crate::R<I3cd08cSpec>;
#[doc = "Register `I3CD08C` writer"]
pub type W = crate::W<I3cd08cSpec>;
#[doc = "Field `SIR` reader - SIR"]
pub type SirR = crate::BitReader;
#[doc = "Field `SIR` writer - SIR"]
pub type SirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIRCTRL` reader - SIR_CTRL"]
pub type SirctrlR = crate::FieldReader;
#[doc = "Field `SIRCTRL` writer - SIR_CTRL"]
pub type SirctrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MIR` reader - MIR"]
pub type MirR = crate::BitReader;
#[doc = "Field `MIR` writer - MIR"]
pub type MirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD74` reader - These bits in Slave Interrupt Request Register are reserved."]
pub type Rsvd74R = crate::FieldReader;
#[doc = "Field `IBISTS` reader - IBI_STS"]
pub type IbistsR = crate::FieldReader;
#[doc = "Field `RSVD2310` reader - These bits in Slave Interrupt Request Register are reserved."]
pub type Rsvd2310R = crate::FieldReader<u16>;
#[doc = "Field `RSVD3124` reader - These bits in Slave Interrupt Request Register are reserved."]
pub type Rsvd3124R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - SIR"]
    #[inline(always)]
    pub fn sir(&self) -> SirR {
        SirR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - SIR_CTRL"]
    #[inline(always)]
    pub fn sirctrl(&self) -> SirctrlR {
        SirctrlR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - MIR"]
    #[inline(always)]
    pub fn mir(&self) -> MirR {
        MirR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - These bits in Slave Interrupt Request Register are reserved."]
    #[inline(always)]
    pub fn rsvd74(&self) -> Rsvd74R {
        Rsvd74R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - IBI_STS"]
    #[inline(always)]
    pub fn ibists(&self) -> IbistsR {
        IbistsR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:23 - These bits in Slave Interrupt Request Register are reserved."]
    #[inline(always)]
    pub fn rsvd2310(&self) -> Rsvd2310R {
        Rsvd2310R::new(((self.bits >> 10) & 0x3fff) as u16)
    }
    #[doc = "Bits 24:31 - These bits in Slave Interrupt Request Register are reserved."]
    #[inline(always)]
    pub fn rsvd3124(&self) -> Rsvd3124R {
        Rsvd3124R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SIR"]
    #[inline(always)]
    pub fn sir(&mut self) -> SirW<I3cd08cSpec> {
        SirW::new(self, 0)
    }
    #[doc = "Bits 1:2 - SIR_CTRL"]
    #[inline(always)]
    pub fn sirctrl(&mut self) -> SirctrlW<I3cd08cSpec> {
        SirctrlW::new(self, 1)
    }
    #[doc = "Bit 3 - MIR"]
    #[inline(always)]
    pub fn mir(&mut self) -> MirW<I3cd08cSpec> {
        MirW::new(self, 3)
    }
}
#[doc = "Slave Interrupt Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd08c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd08c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd08cSpec;
impl crate::RegisterSpec for I3cd08cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd08c::R`](R) reader structure"]
impl crate::Readable for I3cd08cSpec {}
#[doc = "`write(|w| ..)` method takes [`i3cd08c::W`](W) writer structure"]
impl crate::Writable for I3cd08cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD08C to value 0"]
impl crate::Resettable for I3cd08cSpec {}
