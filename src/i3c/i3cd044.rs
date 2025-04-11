#[doc = "Register `I3CD044` reader"]
pub type R = crate::R<I3cd044Spec>;
#[doc = "Register `I3CD044` writer"]
pub type W = crate::W<I3cd044Spec>;
#[doc = "Field `TXTHLDSIGNALEN` reader - TX_THLD_SIGNAL_EN"]
pub type TxthldsignalenR = crate::BitReader;
#[doc = "Field `TXTHLDSIGNALEN` writer - TX_THLD_SIGNAL_EN"]
pub type TxthldsignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTHLDSIGNALEN` reader - RX_THLD_SIGNAL_EN"]
pub type RxthldsignalenR = crate::BitReader;
#[doc = "Field `RXTHLDSIGNALEN` writer - RX_THLD_SIGNAL_EN"]
pub type RxthldsignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBITHLDSIGNALEN` reader - IBI_THLD_SIGNAL_EN"]
pub type IbithldsignalenR = crate::BitReader;
#[doc = "Field `IBITHLDSIGNALEN` writer - IBI_THLD_SIGNAL_EN"]
pub type IbithldsignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDQUEUEREADYSIGNALEN` reader - CMD_QUEUE_READY_SIGNAL_EN"]
pub type CmdqueuereadysignalenR = crate::BitReader;
#[doc = "Field `CMDQUEUEREADYSIGNALEN` writer - CMD_QUEUE_READY_SIGNAL_EN"]
pub type CmdqueuereadysignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESPREADYSIGNALINTREN` reader - RESP_READY_SIGNAL_INTR_EN"]
pub type RespreadysignalintrenR = crate::BitReader;
#[doc = "Field `RESPREADYSIGNALINTREN` writer - RESP_READY_SIGNAL_INTR_EN"]
pub type RespreadysignalintrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSFERABORTSIGNALEN` reader - TRANSFER_ABORT_SIGNAL_EN"]
pub type TransferabortsignalenR = crate::BitReader;
#[doc = "Field `TRANSFERABORTSIGNALEN` writer - TRANSFER_ABORT_SIGNAL_EN"]
pub type TransferabortsignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCCUPDATEDSIGNALEN` reader - CCC_UPDATED_SIGNAL_EN"]
pub type CccupdatedsignalenR = crate::BitReader;
#[doc = "Field `CCCUPDATEDSIGNALEN` writer - CCC_UPDATED_SIGNAL_EN"]
pub type CccupdatedsignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD7` reader - This bit in Interrupt Signal Enable register is reserved."]
pub type Rsvd7R = crate::BitReader;
#[doc = "Field `DYNADDRASSGNSIGNALEN` reader - DYN_ADDR_ASSGN_SIGNAL_EN"]
pub type DynaddrassgnsignalenR = crate::BitReader;
#[doc = "Field `DYNADDRASSGNSIGNALEN` writer - DYN_ADDR_ASSGN_SIGNAL_EN"]
pub type DynaddrassgnsignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSFERERRSIGNALEN` reader - TRANSFER_ERR_SIGNAL_EN"]
pub type TransfererrsignalenR = crate::BitReader;
#[doc = "Field `TRANSFERERRSIGNALEN` writer - TRANSFER_ERR_SIGNAL_EN"]
pub type TransfererrsignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEFSLVSIGNALEN` reader - DEFSLV_SIGNAL_EN"]
pub type DefslvsignalenR = crate::BitReader;
#[doc = "Field `DEFSLVSIGNALEN` writer - DEFSLV_SIGNAL_EN"]
pub type DefslvsignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READREQRECVSIGNALEN` reader - READ_REQ_RECV_SIGNAL_EN"]
pub type ReadreqrecvsignalenR = crate::BitReader;
#[doc = "Field `READREQRECVSIGNALEN` writer - READ_REQ_RECV_SIGNAL_EN"]
pub type ReadreqrecvsignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBIUPDATEDSIGNALEN` reader - IBI_UPDATED_SIGNAL_EN"]
pub type IbiupdatedsignalenR = crate::BitReader;
#[doc = "Field `IBIUPDATEDSIGNALEN` writer - IBI_UPDATED_SIGNAL_EN"]
pub type IbiupdatedsignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSOWNERUPDATEDSIGNALEN` reader - BUSOWNER_UPDATED_SIGNAL_EN"]
pub type BusownerupdatedsignalenR = crate::BitReader;
#[doc = "Field `BUSOWNERUPDATEDSIGNALEN` writer - BUSOWNER_UPDATED_SIGNAL_EN"]
pub type BusownerupdatedsignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD1` reader - reserved"]
pub type Rsvd1R = crate::BitReader;
#[doc = "Field `BUSRESETDONESIGNALEN` reader - BUS_RESET_DONE_SIGNAL_EN"]
pub type BusresetdonesignalenR = crate::BitReader;
#[doc = "Field `BUSRESETDONESIGNALEN` writer - BUS_RESET_DONE_SIGNAL_EN"]
pub type BusresetdonesignalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - reserved"]
pub type RsvdR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - TX_THLD_SIGNAL_EN"]
    #[inline(always)]
    pub fn txthldsignalen(&self) -> TxthldsignalenR {
        TxthldsignalenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX_THLD_SIGNAL_EN"]
    #[inline(always)]
    pub fn rxthldsignalen(&self) -> RxthldsignalenR {
        RxthldsignalenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IBI_THLD_SIGNAL_EN"]
    #[inline(always)]
    pub fn ibithldsignalen(&self) -> IbithldsignalenR {
        IbithldsignalenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CMD_QUEUE_READY_SIGNAL_EN"]
    #[inline(always)]
    pub fn cmdqueuereadysignalen(&self) -> CmdqueuereadysignalenR {
        CmdqueuereadysignalenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RESP_READY_SIGNAL_INTR_EN"]
    #[inline(always)]
    pub fn respreadysignalintren(&self) -> RespreadysignalintrenR {
        RespreadysignalintrenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TRANSFER_ABORT_SIGNAL_EN"]
    #[inline(always)]
    pub fn transferabortsignalen(&self) -> TransferabortsignalenR {
        TransferabortsignalenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CCC_UPDATED_SIGNAL_EN"]
    #[inline(always)]
    pub fn cccupdatedsignalen(&self) -> CccupdatedsignalenR {
        CccupdatedsignalenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit in Interrupt Signal Enable register is reserved."]
    #[inline(always)]
    pub fn rsvd7(&self) -> Rsvd7R {
        Rsvd7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DYN_ADDR_ASSGN_SIGNAL_EN"]
    #[inline(always)]
    pub fn dynaddrassgnsignalen(&self) -> DynaddrassgnsignalenR {
        DynaddrassgnsignalenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TRANSFER_ERR_SIGNAL_EN"]
    #[inline(always)]
    pub fn transfererrsignalen(&self) -> TransfererrsignalenR {
        TransfererrsignalenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DEFSLV_SIGNAL_EN"]
    #[inline(always)]
    pub fn defslvsignalen(&self) -> DefslvsignalenR {
        DefslvsignalenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - READ_REQ_RECV_SIGNAL_EN"]
    #[inline(always)]
    pub fn readreqrecvsignalen(&self) -> ReadreqrecvsignalenR {
        ReadreqrecvsignalenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - IBI_UPDATED_SIGNAL_EN"]
    #[inline(always)]
    pub fn ibiupdatedsignalen(&self) -> IbiupdatedsignalenR {
        IbiupdatedsignalenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - BUSOWNER_UPDATED_SIGNAL_EN"]
    #[inline(always)]
    pub fn busownerupdatedsignalen(&self) -> BusownerupdatedsignalenR {
        BusownerupdatedsignalenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - reserved"]
    #[inline(always)]
    pub fn rsvd1(&self) -> Rsvd1R {
        Rsvd1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - BUS_RESET_DONE_SIGNAL_EN"]
    #[inline(always)]
    pub fn busresetdonesignalen(&self) -> BusresetdonesignalenR {
        BusresetdonesignalenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - TX_THLD_SIGNAL_EN"]
    #[inline(always)]
    pub fn txthldsignalen(&mut self) -> TxthldsignalenW<I3cd044Spec> {
        TxthldsignalenW::new(self, 0)
    }
    #[doc = "Bit 1 - RX_THLD_SIGNAL_EN"]
    #[inline(always)]
    pub fn rxthldsignalen(&mut self) -> RxthldsignalenW<I3cd044Spec> {
        RxthldsignalenW::new(self, 1)
    }
    #[doc = "Bit 2 - IBI_THLD_SIGNAL_EN"]
    #[inline(always)]
    pub fn ibithldsignalen(&mut self) -> IbithldsignalenW<I3cd044Spec> {
        IbithldsignalenW::new(self, 2)
    }
    #[doc = "Bit 3 - CMD_QUEUE_READY_SIGNAL_EN"]
    #[inline(always)]
    pub fn cmdqueuereadysignalen(&mut self) -> CmdqueuereadysignalenW<I3cd044Spec> {
        CmdqueuereadysignalenW::new(self, 3)
    }
    #[doc = "Bit 4 - RESP_READY_SIGNAL_INTR_EN"]
    #[inline(always)]
    pub fn respreadysignalintren(&mut self) -> RespreadysignalintrenW<I3cd044Spec> {
        RespreadysignalintrenW::new(self, 4)
    }
    #[doc = "Bit 5 - TRANSFER_ABORT_SIGNAL_EN"]
    #[inline(always)]
    pub fn transferabortsignalen(&mut self) -> TransferabortsignalenW<I3cd044Spec> {
        TransferabortsignalenW::new(self, 5)
    }
    #[doc = "Bit 6 - CCC_UPDATED_SIGNAL_EN"]
    #[inline(always)]
    pub fn cccupdatedsignalen(&mut self) -> CccupdatedsignalenW<I3cd044Spec> {
        CccupdatedsignalenW::new(self, 6)
    }
    #[doc = "Bit 8 - DYN_ADDR_ASSGN_SIGNAL_EN"]
    #[inline(always)]
    pub fn dynaddrassgnsignalen(&mut self) -> DynaddrassgnsignalenW<I3cd044Spec> {
        DynaddrassgnsignalenW::new(self, 8)
    }
    #[doc = "Bit 9 - TRANSFER_ERR_SIGNAL_EN"]
    #[inline(always)]
    pub fn transfererrsignalen(&mut self) -> TransfererrsignalenW<I3cd044Spec> {
        TransfererrsignalenW::new(self, 9)
    }
    #[doc = "Bit 10 - DEFSLV_SIGNAL_EN"]
    #[inline(always)]
    pub fn defslvsignalen(&mut self) -> DefslvsignalenW<I3cd044Spec> {
        DefslvsignalenW::new(self, 10)
    }
    #[doc = "Bit 11 - READ_REQ_RECV_SIGNAL_EN"]
    #[inline(always)]
    pub fn readreqrecvsignalen(&mut self) -> ReadreqrecvsignalenW<I3cd044Spec> {
        ReadreqrecvsignalenW::new(self, 11)
    }
    #[doc = "Bit 12 - IBI_UPDATED_SIGNAL_EN"]
    #[inline(always)]
    pub fn ibiupdatedsignalen(&mut self) -> IbiupdatedsignalenW<I3cd044Spec> {
        IbiupdatedsignalenW::new(self, 12)
    }
    #[doc = "Bit 13 - BUSOWNER_UPDATED_SIGNAL_EN"]
    #[inline(always)]
    pub fn busownerupdatedsignalen(&mut self) -> BusownerupdatedsignalenW<I3cd044Spec> {
        BusownerupdatedsignalenW::new(self, 13)
    }
    #[doc = "Bit 15 - BUS_RESET_DONE_SIGNAL_EN"]
    #[inline(always)]
    pub fn busresetdonesignalen(&mut self) -> BusresetdonesignalenW<I3cd044Spec> {
        BusresetdonesignalenW::new(self, 15)
    }
}
#[doc = "Interrupt Signal Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd044::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd044::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd044Spec;
impl crate::RegisterSpec for I3cd044Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd044::R`](R) reader structure"]
impl crate::Readable for I3cd044Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd044::W`](W) writer structure"]
impl crate::Writable for I3cd044Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD044 to value 0"]
impl crate::Resettable for I3cd044Spec {}
