#[doc = "Register `I3CD294` reader"]
pub type R = crate::R<I3cd294Spec>;
#[doc = "Register `I3CD294` writer"]
pub type W = crate::W<I3cd294Spec>;
#[doc = "Field `DEVSTATICADDR` reader - DEV_STATIC_ADDR"]
pub type DevstaticaddrR = crate::FieldReader;
#[doc = "Field `DEVSTATICADDR` writer - DEV_STATIC_ADDR"]
pub type DevstaticaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RSVD157` reader - These bits in Device Address Table Register are reserved."]
pub type Rsvd157R = crate::FieldReader;
#[doc = "Field `IBIPECEN` reader - Packet Error Check enabled for accepted IBI from the device. PEC byte is appended at the end of IBI data from the"]
pub type IbipecenR = crate::BitReader;
#[doc = "Field `IBIPECEN` writer - Packet Error Check enabled for accepted IBI from the device. PEC byte is appended at the end of IBI data from the"]
pub type IbipecenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBIWITHDATA` reader - Mandatory one or more data bytes follow the accepted IBI from the device. Data byte continuation is indicated by"]
pub type IbiwithdataR = crate::BitReader;
#[doc = "Field `IBIWITHDATA` writer - Mandatory one or more data bytes follow the accepted IBI from the device. Data byte continuation is indicated by"]
pub type IbiwithdataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIRREJECT` reader - In-Band Slave Interrupt Request Reject field is used to control, per device, whether to accept Master request from"]
pub type SirrejectR = crate::BitReader;
#[doc = "Field `SIRREJECT` writer - In-Band Slave Interrupt Request Reject field is used to control, per device, whether to accept Master request from"]
pub type SirrejectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MRREJECT` reader - In-Band Master Request Reject field is used to control, per device, whether to accept Master request from Devices."]
pub type MrrejectR = crate::BitReader;
#[doc = "Field `MRREJECT` writer - In-Band Master Request Reject field is used to control, per device, whether to accept Master request from Devices."]
pub type MrrejectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD15` reader - These bits in Device Address Table Register are reserved."]
pub type Rsvd15R = crate::BitReader;
#[doc = "Field `DEVDYNAMICADDR` reader - DEV_DYNAMIC_ADDR"]
pub type DevdynamicaddrR = crate::FieldReader;
#[doc = "Field `DEVDYNAMICADDR` writer - DEV_DYNAMIC_ADDR"]
pub type DevdynamicaddrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RSVD2824` reader - These bits in Device Address Table Register are reserved."]
pub type Rsvd2824R = crate::FieldReader;
#[doc = "Field `DEVNACKRETRYCNT` reader - DEV_NACK_RETRY_CNT"]
pub type DevnackretrycntR = crate::FieldReader;
#[doc = "Field `DEVNACKRETRYCNT` writer - DEV_NACK_RETRY_CNT"]
pub type DevnackretrycntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LEGACYI2CDEVICE` reader - LEGACY_I2C_DEVICE"]
pub type Legacyi2cdeviceR = crate::BitReader;
#[doc = "Field `LEGACYI2CDEVICE` writer - LEGACY_I2C_DEVICE"]
pub type Legacyi2cdeviceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - DEV_STATIC_ADDR"]
    #[inline(always)]
    pub fn devstaticaddr(&self) -> DevstaticaddrR {
        DevstaticaddrR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:10 - These bits in Device Address Table Register are reserved."]
    #[inline(always)]
    pub fn rsvd157(&self) -> Rsvd157R {
        Rsvd157R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 11 - Packet Error Check enabled for accepted IBI from the device. PEC byte is appended at the end of IBI data from the"]
    #[inline(always)]
    pub fn ibipecen(&self) -> IbipecenR {
        IbipecenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Mandatory one or more data bytes follow the accepted IBI from the device. Data byte continuation is indicated by"]
    #[inline(always)]
    pub fn ibiwithdata(&self) -> IbiwithdataR {
        IbiwithdataR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - In-Band Slave Interrupt Request Reject field is used to control, per device, whether to accept Master request from"]
    #[inline(always)]
    pub fn sirreject(&self) -> SirrejectR {
        SirrejectR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - In-Band Master Request Reject field is used to control, per device, whether to accept Master request from Devices."]
    #[inline(always)]
    pub fn mrreject(&self) -> MrrejectR {
        MrrejectR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - These bits in Device Address Table Register are reserved."]
    #[inline(always)]
    pub fn rsvd15(&self) -> Rsvd15R {
        Rsvd15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - DEV_DYNAMIC_ADDR"]
    #[inline(always)]
    pub fn devdynamicaddr(&self) -> DevdynamicaddrR {
        DevdynamicaddrR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:28 - These bits in Device Address Table Register are reserved."]
    #[inline(always)]
    pub fn rsvd2824(&self) -> Rsvd2824R {
        Rsvd2824R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:30 - DEV_NACK_RETRY_CNT"]
    #[inline(always)]
    pub fn devnackretrycnt(&self) -> DevnackretrycntR {
        DevnackretrycntR::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - LEGACY_I2C_DEVICE"]
    #[inline(always)]
    pub fn legacyi2cdevice(&self) -> Legacyi2cdeviceR {
        Legacyi2cdeviceR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - DEV_STATIC_ADDR"]
    #[inline(always)]
    pub fn devstaticaddr(&mut self) -> DevstaticaddrW<I3cd294Spec> {
        DevstaticaddrW::new(self, 0)
    }
    #[doc = "Bit 11 - Packet Error Check enabled for accepted IBI from the device. PEC byte is appended at the end of IBI data from the"]
    #[inline(always)]
    pub fn ibipecen(&mut self) -> IbipecenW<I3cd294Spec> {
        IbipecenW::new(self, 11)
    }
    #[doc = "Bit 12 - Mandatory one or more data bytes follow the accepted IBI from the device. Data byte continuation is indicated by"]
    #[inline(always)]
    pub fn ibiwithdata(&mut self) -> IbiwithdataW<I3cd294Spec> {
        IbiwithdataW::new(self, 12)
    }
    #[doc = "Bit 13 - In-Band Slave Interrupt Request Reject field is used to control, per device, whether to accept Master request from"]
    #[inline(always)]
    pub fn sirreject(&mut self) -> SirrejectW<I3cd294Spec> {
        SirrejectW::new(self, 13)
    }
    #[doc = "Bit 14 - In-Band Master Request Reject field is used to control, per device, whether to accept Master request from Devices."]
    #[inline(always)]
    pub fn mrreject(&mut self) -> MrrejectW<I3cd294Spec> {
        MrrejectW::new(self, 14)
    }
    #[doc = "Bits 16:23 - DEV_DYNAMIC_ADDR"]
    #[inline(always)]
    pub fn devdynamicaddr(&mut self) -> DevdynamicaddrW<I3cd294Spec> {
        DevdynamicaddrW::new(self, 16)
    }
    #[doc = "Bits 29:30 - DEV_NACK_RETRY_CNT"]
    #[inline(always)]
    pub fn devnackretrycnt(&mut self) -> DevnackretrycntW<I3cd294Spec> {
        DevnackretrycntW::new(self, 29)
    }
    #[doc = "Bit 31 - LEGACY_I2C_DEVICE"]
    #[inline(always)]
    pub fn legacyi2cdevice(&mut self) -> Legacyi2cdeviceW<I3cd294Spec> {
        Legacyi2cdeviceW::new(self, 31)
    }
}
#[doc = "Device Address Table of Device6\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd294::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd294::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd294Spec;
impl crate::RegisterSpec for I3cd294Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd294::R`](R) reader structure"]
impl crate::Readable for I3cd294Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd294::W`](W) writer structure"]
impl crate::Writable for I3cd294Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD294 to value 0"]
impl crate::Resettable for I3cd294Spec {}
