#[doc = "Register `SPI06C` reader"]
pub type R = crate::R<Spi06cSpec>;
#[doc = "Register `SPI06C` writer"]
pub type W = crate::W<Spi06cSpec>;
#[doc = "Field `CmdForRead3BMode` reader - Command for Read 3B mode"]
pub type CmdForRead3bmodeR = crate::FieldReader;
#[doc = "Field `CmdForRead3BMode` writer - Command for Read 3B mode"]
pub type CmdForRead3bmodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CmdForRead4BMode` reader - Command for Read 4B mode"]
pub type CmdForRead4bmodeR = crate::FieldReader;
#[doc = "Field `CmdForRead4BMode` writer - Command for Read 4B mode"]
pub type CmdForRead4bmodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader;
#[doc = "IO Mode for Page Program\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IomodeForPageProgram {
    #[doc = "0: single bit."]
    SingleBit = 0,
    #[doc = "2: dual bit read/write, data cycle only."]
    DualBitReadwriteDataCycleOnly = 2,
    #[doc = "3: dual bit read/write, including address and dummy byte cycle."]
    DualBitReadwriteIncludingAddressAndDummyByteCycle = 3,
    #[doc = "4: quad bit read/write, data cycle only."]
    QuadBitReadwriteDataCycleOnly = 4,
    #[doc = "5: quad bit read/write, including address and dummy byte cycle."]
    QuadBitReadwriteIncludingAddressAndDummyByteCycle = 5,
    #[doc = "8: QPI mode, quad bit on command/address/data cycles."]
    QpiModeQuadBitOnCommandaddressdataCycles = 8,
}
impl From<IomodeForPageProgram> for u8 {
    #[inline(always)]
    fn from(variant: IomodeForPageProgram) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IomodeForPageProgram {
    type Ux = u8;
}
impl crate::IsEnum for IomodeForPageProgram {}
#[doc = "Field `IOModeForPageProgram` reader - IO Mode for Page Program"]
pub type IomodeForPageProgramR = crate::FieldReader<IomodeForPageProgram>;
impl IomodeForPageProgramR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IomodeForPageProgram> {
        match self.bits {
            0 => Some(IomodeForPageProgram::SingleBit),
            2 => Some(IomodeForPageProgram::DualBitReadwriteDataCycleOnly),
            3 => Some(IomodeForPageProgram::DualBitReadwriteIncludingAddressAndDummyByteCycle),
            4 => Some(IomodeForPageProgram::QuadBitReadwriteDataCycleOnly),
            5 => Some(IomodeForPageProgram::QuadBitReadwriteIncludingAddressAndDummyByteCycle),
            8 => Some(IomodeForPageProgram::QpiModeQuadBitOnCommandaddressdataCycles),
            _ => None,
        }
    }
    #[doc = "single bit."]
    #[inline(always)]
    pub fn is_single_bit(&self) -> bool {
        *self == IomodeForPageProgram::SingleBit
    }
    #[doc = "dual bit read/write, data cycle only."]
    #[inline(always)]
    pub fn is_dual_bit_readwrite_data_cycle_only(&self) -> bool {
        *self == IomodeForPageProgram::DualBitReadwriteDataCycleOnly
    }
    #[doc = "dual bit read/write, including address and dummy byte cycle."]
    #[inline(always)]
    pub fn is_dual_bit_readwrite_including_address_and_dummy_byte_cycle(&self) -> bool {
        *self == IomodeForPageProgram::DualBitReadwriteIncludingAddressAndDummyByteCycle
    }
    #[doc = "quad bit read/write, data cycle only."]
    #[inline(always)]
    pub fn is_quad_bit_readwrite_data_cycle_only(&self) -> bool {
        *self == IomodeForPageProgram::QuadBitReadwriteDataCycleOnly
    }
    #[doc = "quad bit read/write, including address and dummy byte cycle."]
    #[inline(always)]
    pub fn is_quad_bit_readwrite_including_address_and_dummy_byte_cycle(&self) -> bool {
        *self == IomodeForPageProgram::QuadBitReadwriteIncludingAddressAndDummyByteCycle
    }
    #[doc = "QPI mode, quad bit on command/address/data cycles."]
    #[inline(always)]
    pub fn is_qpi_mode_quad_bit_on_commandaddressdata_cycles(&self) -> bool {
        *self == IomodeForPageProgram::QpiModeQuadBitOnCommandaddressdataCycles
    }
}
#[doc = "Field `IOModeForPageProgram` writer - IO Mode for Page Program"]
pub type IomodeForPageProgramW<'a, REG> = crate::FieldWriter<'a, REG, 4, IomodeForPageProgram>;
impl<'a, REG> IomodeForPageProgramW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "single bit."]
    #[inline(always)]
    pub fn single_bit(self) -> &'a mut crate::W<REG> {
        self.variant(IomodeForPageProgram::SingleBit)
    }
    #[doc = "dual bit read/write, data cycle only."]
    #[inline(always)]
    pub fn dual_bit_readwrite_data_cycle_only(self) -> &'a mut crate::W<REG> {
        self.variant(IomodeForPageProgram::DualBitReadwriteDataCycleOnly)
    }
    #[doc = "dual bit read/write, including address and dummy byte cycle."]
    #[inline(always)]
    pub fn dual_bit_readwrite_including_address_and_dummy_byte_cycle(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(IomodeForPageProgram::DualBitReadwriteIncludingAddressAndDummyByteCycle)
    }
    #[doc = "quad bit read/write, data cycle only."]
    #[inline(always)]
    pub fn quad_bit_readwrite_data_cycle_only(self) -> &'a mut crate::W<REG> {
        self.variant(IomodeForPageProgram::QuadBitReadwriteDataCycleOnly)
    }
    #[doc = "quad bit read/write, including address and dummy byte cycle."]
    #[inline(always)]
    pub fn quad_bit_readwrite_including_address_and_dummy_byte_cycle(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(IomodeForPageProgram::QuadBitReadwriteIncludingAddressAndDummyByteCycle)
    }
    #[doc = "QPI mode, quad bit on command/address/data cycles."]
    #[inline(always)]
    pub fn qpi_mode_quad_bit_on_commandaddressdata_cycles(self) -> &'a mut crate::W<REG> {
        self.variant(IomodeForPageProgram::QpiModeQuadBitOnCommandaddressdataCycles)
    }
}
#[doc = "IO Mode for Read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IomodeForRead {
    #[doc = "0: single bit."]
    SingleBit = 0,
    #[doc = "2: dual bit read/write, data cycle only."]
    DualBitReadwriteDataCycleOnly = 2,
    #[doc = "3: dual bit read/write, including address and dummy byte cycle."]
    DualBitReadwriteIncludingAddressAndDummyByteCycle = 3,
    #[doc = "4: quad bit read/write, data cycle only."]
    QuadBitReadwriteDataCycleOnly = 4,
    #[doc = "5: quad bit read/write, including address and dummy byte cycle."]
    QuadBitReadwriteIncludingAddressAndDummyByteCycle = 5,
    #[doc = "8: QPI mode, quad bit on command/address/data cycles."]
    QpiModeQuadBitOnCommandaddressdataCycles = 8,
}
impl From<IomodeForRead> for u8 {
    #[inline(always)]
    fn from(variant: IomodeForRead) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IomodeForRead {
    type Ux = u8;
}
impl crate::IsEnum for IomodeForRead {}
#[doc = "Field `IOModeForRead` reader - IO Mode for Read"]
pub type IomodeForReadR = crate::FieldReader<IomodeForRead>;
impl IomodeForReadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IomodeForRead> {
        match self.bits {
            0 => Some(IomodeForRead::SingleBit),
            2 => Some(IomodeForRead::DualBitReadwriteDataCycleOnly),
            3 => Some(IomodeForRead::DualBitReadwriteIncludingAddressAndDummyByteCycle),
            4 => Some(IomodeForRead::QuadBitReadwriteDataCycleOnly),
            5 => Some(IomodeForRead::QuadBitReadwriteIncludingAddressAndDummyByteCycle),
            8 => Some(IomodeForRead::QpiModeQuadBitOnCommandaddressdataCycles),
            _ => None,
        }
    }
    #[doc = "single bit."]
    #[inline(always)]
    pub fn is_single_bit(&self) -> bool {
        *self == IomodeForRead::SingleBit
    }
    #[doc = "dual bit read/write, data cycle only."]
    #[inline(always)]
    pub fn is_dual_bit_readwrite_data_cycle_only(&self) -> bool {
        *self == IomodeForRead::DualBitReadwriteDataCycleOnly
    }
    #[doc = "dual bit read/write, including address and dummy byte cycle."]
    #[inline(always)]
    pub fn is_dual_bit_readwrite_including_address_and_dummy_byte_cycle(&self) -> bool {
        *self == IomodeForRead::DualBitReadwriteIncludingAddressAndDummyByteCycle
    }
    #[doc = "quad bit read/write, data cycle only."]
    #[inline(always)]
    pub fn is_quad_bit_readwrite_data_cycle_only(&self) -> bool {
        *self == IomodeForRead::QuadBitReadwriteDataCycleOnly
    }
    #[doc = "quad bit read/write, including address and dummy byte cycle."]
    #[inline(always)]
    pub fn is_quad_bit_readwrite_including_address_and_dummy_byte_cycle(&self) -> bool {
        *self == IomodeForRead::QuadBitReadwriteIncludingAddressAndDummyByteCycle
    }
    #[doc = "QPI mode, quad bit on command/address/data cycles."]
    #[inline(always)]
    pub fn is_qpi_mode_quad_bit_on_commandaddressdata_cycles(&self) -> bool {
        *self == IomodeForRead::QpiModeQuadBitOnCommandaddressdataCycles
    }
}
#[doc = "Field `IOModeForRead` writer - IO Mode for Read"]
pub type IomodeForReadW<'a, REG> = crate::FieldWriter<'a, REG, 4, IomodeForRead>;
impl<'a, REG> IomodeForReadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "single bit."]
    #[inline(always)]
    pub fn single_bit(self) -> &'a mut crate::W<REG> {
        self.variant(IomodeForRead::SingleBit)
    }
    #[doc = "dual bit read/write, data cycle only."]
    #[inline(always)]
    pub fn dual_bit_readwrite_data_cycle_only(self) -> &'a mut crate::W<REG> {
        self.variant(IomodeForRead::DualBitReadwriteDataCycleOnly)
    }
    #[doc = "dual bit read/write, including address and dummy byte cycle."]
    #[inline(always)]
    pub fn dual_bit_readwrite_including_address_and_dummy_byte_cycle(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(IomodeForRead::DualBitReadwriteIncludingAddressAndDummyByteCycle)
    }
    #[doc = "quad bit read/write, data cycle only."]
    #[inline(always)]
    pub fn quad_bit_readwrite_data_cycle_only(self) -> &'a mut crate::W<REG> {
        self.variant(IomodeForRead::QuadBitReadwriteDataCycleOnly)
    }
    #[doc = "quad bit read/write, including address and dummy byte cycle."]
    #[inline(always)]
    pub fn quad_bit_readwrite_including_address_and_dummy_byte_cycle(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(IomodeForRead::QuadBitReadwriteIncludingAddressAndDummyByteCycle)
    }
    #[doc = "QPI mode, quad bit on command/address/data cycles."]
    #[inline(always)]
    pub fn qpi_mode_quad_bit_on_commandaddressdata_cycles(self) -> &'a mut crate::W<REG> {
        self.variant(IomodeForRead::QpiModeQuadBitOnCommandaddressdataCycles)
    }
}
impl R {
    #[doc = "Bits 0:7 - Command for Read 3B mode"]
    #[inline(always)]
    pub fn cmd_for_read3bmode(&self) -> CmdForRead3bmodeR {
        CmdForRead3bmodeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Command for Read 4B mode"]
    #[inline(always)]
    pub fn cmd_for_read4bmode(&self) -> CmdForRead4bmodeR {
        CmdForRead4bmodeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - IO Mode for Page Program"]
    #[inline(always)]
    pub fn iomode_for_page_program(&self) -> IomodeForPageProgramR {
        IomodeForPageProgramR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - IO Mode for Read"]
    #[inline(always)]
    pub fn iomode_for_read(&self) -> IomodeForReadR {
        IomodeForReadR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command for Read 3B mode"]
    #[inline(always)]
    pub fn cmd_for_read3bmode(&mut self) -> CmdForRead3bmodeW<Spi06cSpec> {
        CmdForRead3bmodeW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Command for Read 4B mode"]
    #[inline(always)]
    pub fn cmd_for_read4bmode(&mut self) -> CmdForRead4bmodeW<Spi06cSpec> {
        CmdForRead4bmodeW::new(self, 8)
    }
    #[doc = "Bits 24:27 - IO Mode for Page Program"]
    #[inline(always)]
    pub fn iomode_for_page_program(&mut self) -> IomodeForPageProgramW<Spi06cSpec> {
        IomodeForPageProgramW::new(self, 24)
    }
    #[doc = "Bits 28:31 - IO Mode for Read"]
    #[inline(always)]
    pub fn iomode_for_read(&mut self) -> IomodeForReadW<Spi06cSpec> {
        IomodeForReadW::new(self, 28)
    }
}
#[doc = "Host Direct Access Commands \\#4 (SPI1 only)\n\nYou can [`read`](crate::Reg::read) this register and get [`spi06c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi06c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi06cSpec;
impl crate::RegisterSpec for Spi06cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi06c::R`](R) reader structure"]
impl crate::Readable for Spi06cSpec {}
#[doc = "`write(|w| ..)` method takes [`spi06c::W`](W) writer structure"]
impl crate::Writable for Spi06cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI06C to value 0x1303"]
impl crate::Resettable for Spi06cSpec {
    const RESET_VALUE: u32 = 0x1303;
}
