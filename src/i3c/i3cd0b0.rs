#[doc = "Register `I3CD0B0` reader"]
pub type R = crate::R<I3cd0b0Spec>;
#[doc = "Register `I3CD0B0` writer"]
pub type W = crate::W<I3cd0b0Spec>;
#[doc = "Field `DevOpMode` reader - Device Operation Mode"]
pub type DevOpModeR = crate::FieldReader;
#[doc = "Field `DevOpMode` writer - Device Operation Mode"]
pub type DevOpModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RSVD2` reader - These bits in Device Control Extended Register is reserved."]
pub type Rsvd2R = crate::BitReader;
#[doc = "Field `REQMSTACKCTRL` reader - REQMST_ACK_CTRL"]
pub type ReqmstackctrlR = crate::BitReader;
#[doc = "Field `REQMSTACKCTRL` writer - REQMST_ACK_CTRL"]
pub type ReqmstackctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - reserved"]
pub type RsvdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:1 - Device Operation Mode"]
    #[inline(always)]
    pub fn dev_op_mode(&self) -> DevOpModeR {
        DevOpModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - These bits in Device Control Extended Register is reserved."]
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - REQMST_ACK_CTRL"]
    #[inline(always)]
    pub fn reqmstackctrl(&self) -> ReqmstackctrlR {
        ReqmstackctrlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - Device Operation Mode"]
    #[inline(always)]
    pub fn dev_op_mode(&mut self) -> DevOpModeW<I3cd0b0Spec> {
        DevOpModeW::new(self, 0)
    }
    #[doc = "Bit 3 - REQMST_ACK_CTRL"]
    #[inline(always)]
    pub fn reqmstackctrl(&mut self) -> ReqmstackctrlW<I3cd0b0Spec> {
        ReqmstackctrlW::new(self, 3)
    }
}
#[doc = "Device Control Extended Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd0b0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd0b0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd0b0Spec;
impl crate::RegisterSpec for I3cd0b0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd0b0::R`](R) reader structure"]
impl crate::Readable for I3cd0b0Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd0b0::W`](W) writer structure"]
impl crate::Writable for I3cd0b0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD0B0 to value 0"]
impl crate::Resettable for I3cd0b0Spec {}
