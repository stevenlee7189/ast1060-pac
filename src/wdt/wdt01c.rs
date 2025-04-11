#[doc = "Register `WDT01C` reader"]
pub type R = crate::R<Wdt01cSpec>;
#[doc = "Register `WDT01C` writer"]
pub type W = crate::W<Wdt01cSpec>;
#[doc = "Field `EnblRstARMAndRelatedCtrls` reader - Enable reset ARM and related controllers"]
pub type EnblRstArmandRelatedCtrlsR = crate::BitReader;
#[doc = "Field `EnblRstARMAndRelatedCtrls` writer - Enable reset ARM and related controllers"]
pub type EnblRstArmandRelatedCtrlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstSRAMCtrl` reader - Enable reset SRAM controller"]
pub type EnblRstSramctrlR = crate::BitReader;
#[doc = "Field `EnblRstSRAMCtrl` writer - Enable reset SRAM controller"]
pub type EnblRstSramctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstAHBBridges` reader - Enable reset AHB bridges"]
pub type EnblRstAhbbridgesR = crate::BitReader;
#[doc = "Field `EnblRstAHBBridges` writer - Enable reset AHB bridges"]
pub type EnblRstAhbbridgesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstSOCCtrl` reader - Enable reset SOC controller"]
pub type EnblRstSocctrlR = crate::BitReader;
#[doc = "Field `EnblRstSOCCtrl` writer - Enable reset SOC controller"]
pub type EnblRstSocctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::FieldReader<u16>;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `EnblRstUSBDevCtrl` reader - Enable reset USB Dev controller"]
pub type EnblRstUsbdevCtrlR = crate::BitReader;
#[doc = "Field `EnblRstUSBDevCtrl` writer - Enable reset USB Dev controller"]
pub type EnblRstUsbdevCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EnblRstHACEng` reader - Enable reset HAC engine"]
pub type EnblRstHacengR = crate::BitReader;
#[doc = "Field `EnblRstHACEng` writer - Enable reset HAC engine"]
pub type EnblRstHacengW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable reset ARM and related controllers"]
    #[inline(always)]
    pub fn enbl_rst_armand_related_ctrls(&self) -> EnblRstArmandRelatedCtrlsR {
        EnblRstArmandRelatedCtrlsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable reset SRAM controller"]
    #[inline(always)]
    pub fn enbl_rst_sramctrl(&self) -> EnblRstSramctrlR {
        EnblRstSramctrlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable reset AHB bridges"]
    #[inline(always)]
    pub fn enbl_rst_ahbbridges(&self) -> EnblRstAhbbridgesR {
        EnblRstAhbbridgesR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable reset SOC controller"]
    #[inline(always)]
    pub fn enbl_rst_socctrl(&self) -> EnblRstSocctrlR {
        EnblRstSocctrlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 5:14 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 5) & 0x03ff) as u16)
    }
    #[doc = "Bit 7 - Enable reset USB Dev controller"]
    #[inline(always)]
    pub fn enbl_rst_usbdev_ctrl(&self) -> EnblRstUsbdevCtrlR {
        EnblRstUsbdevCtrlR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Enable reset HAC engine"]
    #[inline(always)]
    pub fn enbl_rst_haceng(&self) -> EnblRstHacengR {
        EnblRstHacengR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable reset ARM and related controllers"]
    #[inline(always)]
    pub fn enbl_rst_armand_related_ctrls(&mut self) -> EnblRstArmandRelatedCtrlsW<Wdt01cSpec> {
        EnblRstArmandRelatedCtrlsW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable reset SRAM controller"]
    #[inline(always)]
    pub fn enbl_rst_sramctrl(&mut self) -> EnblRstSramctrlW<Wdt01cSpec> {
        EnblRstSramctrlW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable reset AHB bridges"]
    #[inline(always)]
    pub fn enbl_rst_ahbbridges(&mut self) -> EnblRstAhbbridgesW<Wdt01cSpec> {
        EnblRstAhbbridgesW::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Wdt01cSpec> {
        Reserved4W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable reset SOC controller"]
    #[inline(always)]
    pub fn enbl_rst_socctrl(&mut self) -> EnblRstSocctrlW<Wdt01cSpec> {
        EnblRstSocctrlW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Wdt01cSpec> {
        Reserved2W::new(self, 5)
    }
    #[doc = "Bits 5:14 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Wdt01cSpec> {
        Reserved3W::new(self, 5)
    }
    #[doc = "Bit 7 - Enable reset USB Dev controller"]
    #[inline(always)]
    pub fn enbl_rst_usbdev_ctrl(&mut self) -> EnblRstUsbdevCtrlW<Wdt01cSpec> {
        EnblRstUsbdevCtrlW::new(self, 7)
    }
    #[doc = "Bits 8:14 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Wdt01cSpec> {
        Reserved1W::new(self, 8)
    }
    #[doc = "Bit 15 - Enable reset HAC engine"]
    #[inline(always)]
    pub fn enbl_rst_haceng(&mut self) -> EnblRstHacengW<Wdt01cSpec> {
        EnblRstHacengW::new(self, 15)
    }
}
#[doc = "WDTn Reset Mask Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt01c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt01c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wdt01cSpec;
impl crate::RegisterSpec for Wdt01cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt01c::R`](R) reader structure"]
impl crate::Readable for Wdt01cSpec {}
#[doc = "`write(|w| ..)` method takes [`wdt01c::W`](W) writer structure"]
impl crate::Writable for Wdt01cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDT01C to value 0x030f_1ff1"]
impl crate::Resettable for Wdt01cSpec {
    const RESET_VALUE: u32 = 0x030f_1ff1;
}
