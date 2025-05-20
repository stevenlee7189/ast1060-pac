#[doc = "Register `FMC008` reader"]
pub type R = crate::R<Fmc008Spec>;
#[doc = "Register `FMC008` writer"]
pub type W = crate::W<Fmc008Spec>;
#[doc = "Field `Reserved02` reader - Reserved (0)"]
pub type Reserved02R = crate::BitReader;
#[doc = "SPI Write Address Protected Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpiwrAddrProtectedIntenbl {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<SpiwrAddrProtectedIntenbl> for bool {
    #[inline(always)]
    fn from(variant: SpiwrAddrProtectedIntenbl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIWrAddrProtectedINTEnbl` reader - SPI Write Address Protected Interrupt Enable"]
pub type SpiwrAddrProtectedIntenblR = crate::BitReader<SpiwrAddrProtectedIntenbl>;
impl SpiwrAddrProtectedIntenblR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SpiwrAddrProtectedIntenbl {
        match self.bits {
            false => SpiwrAddrProtectedIntenbl::Disable,
            true => SpiwrAddrProtectedIntenbl::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SpiwrAddrProtectedIntenbl::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SpiwrAddrProtectedIntenbl::Enable
    }
}
#[doc = "Field `SPIWrAddrProtectedINTEnbl` writer - SPI Write Address Protected Interrupt Enable"]
pub type SpiwrAddrProtectedIntenblW<'a, REG> = crate::BitWriter<'a, REG, SpiwrAddrProtectedIntenbl>;
impl<'a, REG> SpiwrAddrProtectedIntenblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SpiwrAddrProtectedIntenbl::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(SpiwrAddrProtectedIntenbl::Enable)
    }
}
#[doc = "Field `SPICmdAbortINTEnbl` reader - SPI Command Abort Interrupt Enable"]
pub type SpicmdAbortIntenblR = crate::BitReader;
#[doc = "Field `SPICmdAbortINTEnbl` writer - SPI Command Abort Interrupt Enable"]
pub type SpicmdAbortIntenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAINTEnbl` reader - DMA Interrupt Enable"]
pub type DmaintenblR = crate::BitReader;
#[doc = "Field `DMAINTEnbl` writer - DMA Interrupt Enable"]
pub type DmaintenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMABufferModeFIFOFullEmptyINTEnbl` reader - DMA Buffer Mode FIFO Full/Empty Interrupt Enable"]
pub type DmabufferModeFifofullEmptyIntenblR = crate::BitReader;
#[doc = "Field `DMABufferModeFIFOFullEmptyINTEnbl` writer - DMA Buffer Mode FIFO Full/Empty Interrupt Enable"]
pub type DmabufferModeFifofullEmptyIntenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `SPIWrAddrProtectedStatus` reader - SPI Write Address Protected Status"]
pub type SpiwrAddrProtectedStatusR = crate::BitReader;
#[doc = "Field `SPIWrAddrProtectedStatus` writer - SPI Write Address Protected Status"]
pub type SpiwrAddrProtectedStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPICmdAbortStatus` reader - SPI Command Abort Status"]
pub type SpicmdAbortStatusR = crate::BitReader;
#[doc = "Field `SPICmdAbortStatus` writer - SPI Command Abort Status"]
pub type SpicmdAbortStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "DMA Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmastatus {
    #[doc = "0: Busy when DMA was enabled, or Idle when DMA was disabled."]
    BusyWhenDmaWasEnabledOrIdleWhenDmaWasDisabled = 0,
    #[doc = "1: DMA Finish"]
    DmaFinish = 1,
}
impl From<Dmastatus> for bool {
    #[inline(always)]
    fn from(variant: Dmastatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAStatus` reader - DMA Status"]
pub type DmastatusR = crate::BitReader<Dmastatus>;
impl DmastatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmastatus {
        match self.bits {
            false => Dmastatus::BusyWhenDmaWasEnabledOrIdleWhenDmaWasDisabled,
            true => Dmastatus::DmaFinish,
        }
    }
    #[doc = "Busy when DMA was enabled, or Idle when DMA was disabled."]
    #[inline(always)]
    pub fn is_busy_when_dma_was_enabled_or_idle_when_dma_was_disabled(&self) -> bool {
        *self == Dmastatus::BusyWhenDmaWasEnabledOrIdleWhenDmaWasDisabled
    }
    #[doc = "DMA Finish"]
    #[inline(always)]
    pub fn is_dma_finish(&self) -> bool {
        *self == Dmastatus::DmaFinish
    }
}
#[doc = "DMA Buffer Mode FIFO Empty Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmabufferModeFifoemptyStatus {
    #[doc = "0: FIFO is non-empty"]
    FifoIsNonempty = 0,
    #[doc = "1: FIFO is empty"]
    FifoIsEmpty = 1,
}
impl From<DmabufferModeFifoemptyStatus> for bool {
    #[inline(always)]
    fn from(variant: DmabufferModeFifoemptyStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMABufferModeFIFOEmptyStatus` reader - DMA Buffer Mode FIFO Empty Status"]
pub type DmabufferModeFifoemptyStatusR = crate::BitReader<DmabufferModeFifoemptyStatus>;
impl DmabufferModeFifoemptyStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmabufferModeFifoemptyStatus {
        match self.bits {
            false => DmabufferModeFifoemptyStatus::FifoIsNonempty,
            true => DmabufferModeFifoemptyStatus::FifoIsEmpty,
        }
    }
    #[doc = "FIFO is non-empty"]
    #[inline(always)]
    pub fn is_fifo_is_nonempty(&self) -> bool {
        *self == DmabufferModeFifoemptyStatus::FifoIsNonempty
    }
    #[doc = "FIFO is empty"]
    #[inline(always)]
    pub fn is_fifo_is_empty(&self) -> bool {
        *self == DmabufferModeFifoemptyStatus::FifoIsEmpty
    }
}
#[doc = "DMA Buffer Mode FIFO Full Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmabufferModeFifofullStatus {
    #[doc = "0: FIFO is non-full"]
    FifoIsNonfull = 0,
    #[doc = "1: FIFO is full"]
    FifoIsFull = 1,
}
impl From<DmabufferModeFifofullStatus> for bool {
    #[inline(always)]
    fn from(variant: DmabufferModeFifofullStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMABufferModeFIFOFullStatus` reader - DMA Buffer Mode FIFO Full Status"]
pub type DmabufferModeFifofullStatusR = crate::BitReader<DmabufferModeFifofullStatus>;
impl DmabufferModeFifofullStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmabufferModeFifofullStatus {
        match self.bits {
            false => DmabufferModeFifofullStatus::FifoIsNonfull,
            true => DmabufferModeFifofullStatus::FifoIsFull,
        }
    }
    #[doc = "FIFO is non-full"]
    #[inline(always)]
    pub fn is_fifo_is_nonfull(&self) -> bool {
        *self == DmabufferModeFifofullStatus::FifoIsNonfull
    }
    #[doc = "FIFO is full"]
    #[inline(always)]
    pub fn is_fifo_is_full(&self) -> bool {
        *self == DmabufferModeFifofullStatus::FifoIsFull
    }
}
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved02(&self) -> Reserved02R {
        Reserved02R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI Write Address Protected Interrupt Enable"]
    #[inline(always)]
    pub fn spiwr_addr_protected_intenbl(&self) -> SpiwrAddrProtectedIntenblR {
        SpiwrAddrProtectedIntenblR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPI Command Abort Interrupt Enable"]
    #[inline(always)]
    pub fn spicmd_abort_intenbl(&self) -> SpicmdAbortIntenblR {
        SpicmdAbortIntenblR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt Enable"]
    #[inline(always)]
    pub fn dmaintenbl(&self) -> DmaintenblR {
        DmaintenblR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA Buffer Mode FIFO Full/Empty Interrupt Enable"]
    #[inline(always)]
    pub fn dmabuffer_mode_fifofull_empty_intenbl(&self) -> DmabufferModeFifofullEmptyIntenblR {
        DmabufferModeFifofullEmptyIntenblR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:8 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - SPI Write Address Protected Status"]
    #[inline(always)]
    pub fn spiwr_addr_protected_status(&self) -> SpiwrAddrProtectedStatusR {
        SpiwrAddrProtectedStatusR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SPI Command Abort Status"]
    #[inline(always)]
    pub fn spicmd_abort_status(&self) -> SpicmdAbortStatusR {
        SpicmdAbortStatusR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA Status"]
    #[inline(always)]
    pub fn dmastatus(&self) -> DmastatusR {
        DmastatusR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DMA Buffer Mode FIFO Empty Status"]
    #[inline(always)]
    pub fn dmabuffer_mode_fifoempty_status(&self) -> DmabufferModeFifoemptyStatusR {
        DmabufferModeFifoemptyStatusR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DMA Buffer Mode FIFO Full Status"]
    #[inline(always)]
    pub fn dmabuffer_mode_fifofull_status(&self) -> DmabufferModeFifofullStatusR {
        DmabufferModeFifofullStatusR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bit 1 - SPI Write Address Protected Interrupt Enable"]
    #[inline(always)]
    pub fn spiwr_addr_protected_intenbl(&mut self) -> SpiwrAddrProtectedIntenblW<Fmc008Spec> {
        SpiwrAddrProtectedIntenblW::new(self, 1)
    }
    #[doc = "Bit 2 - SPI Command Abort Interrupt Enable"]
    #[inline(always)]
    pub fn spicmd_abort_intenbl(&mut self) -> SpicmdAbortIntenblW<Fmc008Spec> {
        SpicmdAbortIntenblW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Interrupt Enable"]
    #[inline(always)]
    pub fn dmaintenbl(&mut self) -> DmaintenblW<Fmc008Spec> {
        DmaintenblW::new(self, 3)
    }
    #[doc = "Bit 4 - DMA Buffer Mode FIFO Full/Empty Interrupt Enable"]
    #[inline(always)]
    pub fn dmabuffer_mode_fifofull_empty_intenbl(
        &mut self,
    ) -> DmabufferModeFifofullEmptyIntenblW<Fmc008Spec> {
        DmabufferModeFifofullEmptyIntenblW::new(self, 4)
    }
    #[doc = "Bit 9 - SPI Write Address Protected Status"]
    #[inline(always)]
    pub fn spiwr_addr_protected_status(&mut self) -> SpiwrAddrProtectedStatusW<Fmc008Spec> {
        SpiwrAddrProtectedStatusW::new(self, 9)
    }
    #[doc = "Bit 10 - SPI Command Abort Status"]
    #[inline(always)]
    pub fn spicmd_abort_status(&mut self) -> SpicmdAbortStatusW<Fmc008Spec> {
        SpicmdAbortStatusW::new(self, 10)
    }
}
#[doc = "Interrupt Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc008::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc008::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc008Spec;
impl crate::RegisterSpec for Fmc008Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc008::R`](R) reader structure"]
impl crate::Readable for Fmc008Spec {}
#[doc = "`write(|w| ..)` method takes [`fmc008::W`](W) writer structure"]
impl crate::Writable for Fmc008Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC008 to value 0"]
impl crate::Resettable for Fmc008Spec {}
