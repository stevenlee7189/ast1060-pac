#[doc = "Register `I3CD004` reader"]
pub type R = crate::R<I3cd004Spec>;
#[doc = "Register `I3CD004` writer"]
pub type W = crate::W<I3cd004Spec>;
#[doc = "Field `DevStaticAddr` reader - Device Static Address."]
pub type DevStaticAddrR = crate::FieldReader;
#[doc = "Field `DevStaticAddr` writer - Device Static Address."]
pub type DevStaticAddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RSVD147` reader - These bits in Device Address Register are reserved. It will always return 0."]
pub type Rsvd147R = crate::FieldReader;
#[doc = "Field `StaticAddrValid` reader - Static Address Valid"]
pub type StaticAddrValidR = crate::BitReader;
#[doc = "Field `StaticAddrValid` writer - Static Address Valid"]
pub type StaticAddrValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DevDynamicAddr` reader - Device Dynamic Address"]
pub type DevDynamicAddrR = crate::FieldReader;
#[doc = "Field `DevDynamicAddr` writer - Device Dynamic Address"]
pub type DevDynamicAddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RSVD3023` reader - These bits in Device Address Register are reserved. It will always return 0."]
pub type Rsvd3023R = crate::FieldReader;
#[doc = "Field `DynamicAddrValid` reader - Dynamic Address Valid"]
pub type DynamicAddrValidR = crate::BitReader;
#[doc = "Field `DynamicAddrValid` writer - Dynamic Address Valid"]
pub type DynamicAddrValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Device Static Address."]
    #[inline(always)]
    pub fn dev_static_addr(&self) -> DevStaticAddrR {
        DevStaticAddrR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:14 - These bits in Device Address Register are reserved. It will always return 0."]
    #[inline(always)]
    pub fn rsvd147(&self) -> Rsvd147R {
        Rsvd147R::new(((self.bits >> 7) & 0xff) as u8)
    }
    #[doc = "Bit 15 - Static Address Valid"]
    #[inline(always)]
    pub fn static_addr_valid(&self) -> StaticAddrValidR {
        StaticAddrValidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Device Dynamic Address"]
    #[inline(always)]
    pub fn dev_dynamic_addr(&self) -> DevDynamicAddrR {
        DevDynamicAddrR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 23:30 - These bits in Device Address Register are reserved. It will always return 0."]
    #[inline(always)]
    pub fn rsvd3023(&self) -> Rsvd3023R {
        Rsvd3023R::new(((self.bits >> 23) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Dynamic Address Valid"]
    #[inline(always)]
    pub fn dynamic_addr_valid(&self) -> DynamicAddrValidR {
        DynamicAddrValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Device Static Address."]
    #[inline(always)]
    pub fn dev_static_addr(&mut self) -> DevStaticAddrW<I3cd004Spec> {
        DevStaticAddrW::new(self, 0)
    }
    #[doc = "Bit 15 - Static Address Valid"]
    #[inline(always)]
    pub fn static_addr_valid(&mut self) -> StaticAddrValidW<I3cd004Spec> {
        StaticAddrValidW::new(self, 15)
    }
    #[doc = "Bits 16:22 - Device Dynamic Address"]
    #[inline(always)]
    pub fn dev_dynamic_addr(&mut self) -> DevDynamicAddrW<I3cd004Spec> {
        DevDynamicAddrW::new(self, 16)
    }
    #[doc = "Bit 31 - Dynamic Address Valid"]
    #[inline(always)]
    pub fn dynamic_addr_valid(&mut self) -> DynamicAddrValidW<I3cd004Spec> {
        DynamicAddrValidW::new(self, 31)
    }
}
#[doc = "Device Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd004::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd004::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd004Spec;
impl crate::RegisterSpec for I3cd004Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd004::R`](R) reader structure"]
impl crate::Readable for I3cd004Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd004::W`](W) writer structure"]
impl crate::Writable for I3cd004Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD004 to value 0"]
impl crate::Resettable for I3cd004Spec {}
