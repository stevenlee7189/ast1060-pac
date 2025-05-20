#[doc = "Register `FMC00C` reader"]
pub type R = crate::R<Fmc00cSpec>;
#[doc = "Register `FMC00C` writer"]
pub type W = crate::W<Fmc00cSpec>;
#[doc = "Data Byte lane disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DataByteLaneDisable {
    #[doc = "1: disable data byte 0"]
    DisableDataByte0 = 1,
    #[doc = "2: disable data byte 1"]
    DisableDataByte1 = 2,
    #[doc = "4: disable data byte 2"]
    DisableDataByte2 = 4,
    #[doc = "8: disable data byte 3"]
    DisableDataByte3 = 8,
}
impl From<DataByteLaneDisable> for u8 {
    #[inline(always)]
    fn from(variant: DataByteLaneDisable) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DataByteLaneDisable {
    type Ux = u8;
}
impl crate::IsEnum for DataByteLaneDisable {}
#[doc = "Field `DataByteLaneDisable` reader - Data Byte lane disable"]
pub type DataByteLaneDisableR = crate::FieldReader<DataByteLaneDisable>;
impl DataByteLaneDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DataByteLaneDisable> {
        match self.bits {
            1 => Some(DataByteLaneDisable::DisableDataByte0),
            2 => Some(DataByteLaneDisable::DisableDataByte1),
            4 => Some(DataByteLaneDisable::DisableDataByte2),
            8 => Some(DataByteLaneDisable::DisableDataByte3),
            _ => None,
        }
    }
    #[doc = "disable data byte 0"]
    #[inline(always)]
    pub fn is_disable_data_byte_0(&self) -> bool {
        *self == DataByteLaneDisable::DisableDataByte0
    }
    #[doc = "disable data byte 1"]
    #[inline(always)]
    pub fn is_disable_data_byte_1(&self) -> bool {
        *self == DataByteLaneDisable::DisableDataByte1
    }
    #[doc = "disable data byte 2"]
    #[inline(always)]
    pub fn is_disable_data_byte_2(&self) -> bool {
        *self == DataByteLaneDisable::DisableDataByte2
    }
    #[doc = "disable data byte 3"]
    #[inline(always)]
    pub fn is_disable_data_byte_3(&self) -> bool {
        *self == DataByteLaneDisable::DisableDataByte3
    }
}
#[doc = "Field `DataByteLaneDisable` writer - Data Byte lane disable"]
pub type DataByteLaneDisableW<'a, REG> = crate::FieldWriter<'a, REG, 4, DataByteLaneDisable>;
impl<'a, REG> DataByteLaneDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disable data byte 0"]
    #[inline(always)]
    pub fn disable_data_byte_0(self) -> &'a mut crate::W<REG> {
        self.variant(DataByteLaneDisable::DisableDataByte0)
    }
    #[doc = "disable data byte 1"]
    #[inline(always)]
    pub fn disable_data_byte_1(self) -> &'a mut crate::W<REG> {
        self.variant(DataByteLaneDisable::DisableDataByte1)
    }
    #[doc = "disable data byte 2"]
    #[inline(always)]
    pub fn disable_data_byte_2(self) -> &'a mut crate::W<REG> {
        self.variant(DataByteLaneDisable::DisableDataByte2)
    }
    #[doc = "disable data byte 3"]
    #[inline(always)]
    pub fn disable_data_byte_3(self) -> &'a mut crate::W<REG> {
        self.variant(DataByteLaneDisable::DisableDataByte3)
    }
}
#[doc = "SPI Address Byte lane disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SpiaddrByteLaneDisable {
    #[doc = "1: disable address byte 0"]
    DisableAddressByte0 = 1,
    #[doc = "2: disable address byte 1"]
    DisableAddressByte1 = 2,
    #[doc = "4: disable address byte 2"]
    DisableAddressByte2 = 4,
    #[doc = "8: disable address byte 3 (only valid for 4-byte address mode)"]
    DisableAddressByte3OnlyValidFor4byteAddressMode = 8,
}
impl From<SpiaddrByteLaneDisable> for u8 {
    #[inline(always)]
    fn from(variant: SpiaddrByteLaneDisable) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SpiaddrByteLaneDisable {
    type Ux = u8;
}
impl crate::IsEnum for SpiaddrByteLaneDisable {}
#[doc = "Field `SPIAddrByteLaneDisable` reader - SPI Address Byte lane disable"]
pub type SpiaddrByteLaneDisableR = crate::FieldReader<SpiaddrByteLaneDisable>;
impl SpiaddrByteLaneDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SpiaddrByteLaneDisable> {
        match self.bits {
            1 => Some(SpiaddrByteLaneDisable::DisableAddressByte0),
            2 => Some(SpiaddrByteLaneDisable::DisableAddressByte1),
            4 => Some(SpiaddrByteLaneDisable::DisableAddressByte2),
            8 => Some(SpiaddrByteLaneDisable::DisableAddressByte3OnlyValidFor4byteAddressMode),
            _ => None,
        }
    }
    #[doc = "disable address byte 0"]
    #[inline(always)]
    pub fn is_disable_address_byte_0(&self) -> bool {
        *self == SpiaddrByteLaneDisable::DisableAddressByte0
    }
    #[doc = "disable address byte 1"]
    #[inline(always)]
    pub fn is_disable_address_byte_1(&self) -> bool {
        *self == SpiaddrByteLaneDisable::DisableAddressByte1
    }
    #[doc = "disable address byte 2"]
    #[inline(always)]
    pub fn is_disable_address_byte_2(&self) -> bool {
        *self == SpiaddrByteLaneDisable::DisableAddressByte2
    }
    #[doc = "disable address byte 3 (only valid for 4-byte address mode)"]
    #[inline(always)]
    pub fn is_disable_address_byte_3_only_valid_for_4byte_address_mode(&self) -> bool {
        *self == SpiaddrByteLaneDisable::DisableAddressByte3OnlyValidFor4byteAddressMode
    }
}
#[doc = "Field `SPIAddrByteLaneDisable` writer - SPI Address Byte lane disable"]
pub type SpiaddrByteLaneDisableW<'a, REG> = crate::FieldWriter<'a, REG, 4, SpiaddrByteLaneDisable>;
impl<'a, REG> SpiaddrByteLaneDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "disable address byte 0"]
    #[inline(always)]
    pub fn disable_address_byte_0(self) -> &'a mut crate::W<REG> {
        self.variant(SpiaddrByteLaneDisable::DisableAddressByte0)
    }
    #[doc = "disable address byte 1"]
    #[inline(always)]
    pub fn disable_address_byte_1(self) -> &'a mut crate::W<REG> {
        self.variant(SpiaddrByteLaneDisable::DisableAddressByte1)
    }
    #[doc = "disable address byte 2"]
    #[inline(always)]
    pub fn disable_address_byte_2(self) -> &'a mut crate::W<REG> {
        self.variant(SpiaddrByteLaneDisable::DisableAddressByte2)
    }
    #[doc = "disable address byte 3 (only valid for 4-byte address mode)"]
    #[inline(always)]
    pub fn disable_address_byte_3_only_valid_for_4byte_address_mode(self) -> &'a mut crate::W<REG> {
        self.variant(SpiaddrByteLaneDisable::DisableAddressByte3OnlyValidFor4byteAddressMode)
    }
}
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:3 - Data Byte lane disable"]
    #[inline(always)]
    pub fn data_byte_lane_disable(&self) -> DataByteLaneDisableR {
        DataByteLaneDisableR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - SPI Address Byte lane disable"]
    #[inline(always)]
    pub fn spiaddr_byte_lane_disable(&self) -> SpiaddrByteLaneDisableR {
        SpiaddrByteLaneDisableR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data Byte lane disable"]
    #[inline(always)]
    pub fn data_byte_lane_disable(&mut self) -> DataByteLaneDisableW<Fmc00cSpec> {
        DataByteLaneDisableW::new(self, 0)
    }
    #[doc = "Bits 4:7 - SPI Address Byte lane disable"]
    #[inline(always)]
    pub fn spiaddr_byte_lane_disable(&mut self) -> SpiaddrByteLaneDisableW<Fmc00cSpec> {
        SpiaddrByteLaneDisableW::new(self, 4)
    }
}
#[doc = "Command Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc00c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc00c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc00cSpec;
impl crate::RegisterSpec for Fmc00cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc00c::R`](R) reader structure"]
impl crate::Readable for Fmc00cSpec {}
#[doc = "`write(|w| ..)` method takes [`fmc00c::W`](W) writer structure"]
impl crate::Writable for Fmc00cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC00C to value 0"]
impl crate::Resettable for Fmc00cSpec {}
