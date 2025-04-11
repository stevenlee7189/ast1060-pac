#[doc = "Register `I3CD048` reader"]
pub type R = crate::R<I3cd048Spec>;
#[doc = "Register `I3CD048` writer"]
pub type W = crate::W<I3cd048Spec>;
#[doc = "Field `TXTHLDFORCEEN` reader - TX_THLD_FORCE_EN"]
pub type TxthldforceenR = crate::BitReader;
#[doc = "Field `TXTHLDFORCEEN` writer - TX_THLD_FORCE_EN"]
pub type TxthldforceenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTHLDFORCEEN` reader - RX_THLD_FORCE_EN"]
pub type RxthldforceenR = crate::BitReader;
#[doc = "Field `RXTHLDFORCEEN` writer - RX_THLD_FORCE_EN"]
pub type RxthldforceenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBITHLDFORCEEN` reader - IBI_THLD_FORCE_EN"]
pub type IbithldforceenR = crate::BitReader;
#[doc = "Field `IBITHLDFORCEEN` writer - IBI_THLD_FORCE_EN"]
pub type IbithldforceenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDQUEUEREADYFORCEEN` reader - CMD_QUEUE_READY_FORCE_EN"]
pub type CmdqueuereadyforceenR = crate::BitReader;
#[doc = "Field `CMDQUEUEREADYFORCEEN` writer - CMD_QUEUE_READY_FORCE_EN"]
pub type CmdqueuereadyforceenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESPREADYFORCEINTREN` reader - RESP_READY_FORCE_INTR_EN"]
pub type RespreadyforceintrenR = crate::BitReader;
#[doc = "Field `RESPREADYFORCEINTREN` writer - RESP_READY_FORCE_INTR_EN"]
pub type RespreadyforceintrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSFERABORTFORCEEN` reader - TRANSFER_ABORT_FORCE_EN"]
pub type TransferabortforceenR = crate::BitReader;
#[doc = "Field `TRANSFERABORTFORCEEN` writer - TRANSFER_ABORT_FORCE_EN"]
pub type TransferabortforceenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCCUPDATEDFORCEEN` reader - CCC_UPDATED_FORCE_EN"]
pub type CccupdatedforceenR = crate::BitReader;
#[doc = "Field `CCCUPDATEDFORCEEN` writer - CCC_UPDATED_FORCE_EN"]
pub type CccupdatedforceenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD7` reader - This bit in Interrupt Force Enable register is reserved."]
pub type Rsvd7R = crate::BitReader;
#[doc = "Field `RSVD7` writer - This bit in Interrupt Force Enable register is reserved."]
pub type Rsvd7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DYNADDRASSGNFORCEEN` reader - DYN_ADDR_ASSGN_FORCE_EN"]
pub type DynaddrassgnforceenR = crate::BitReader;
#[doc = "Field `DYNADDRASSGNFORCEEN` writer - DYN_ADDR_ASSGN_FORCE_EN"]
pub type DynaddrassgnforceenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANSFERERRFORCEEN` reader - TRANSFER_ERR_FORCE_EN"]
pub type TransfererrforceenR = crate::BitReader;
#[doc = "Field `TRANSFERERRFORCEEN` writer - TRANSFER_ERR_FORCE_EN"]
pub type TransfererrforceenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEFSLVFORCEEN` reader - DEFSLV_FORCE_EN"]
pub type DefslvforceenR = crate::BitReader;
#[doc = "Field `DEFSLVFORCEEN` writer - DEFSLV_FORCE_EN"]
pub type DefslvforceenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READREQRECVFORCEEN` reader - READ_REQ_RECV_FORCE_EN"]
pub type ReadreqrecvforceenR = crate::BitReader;
#[doc = "Field `READREQRECVFORCEEN` writer - READ_REQ_RECV_FORCE_EN"]
pub type ReadreqrecvforceenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBIUPDATEDFORCEEN` reader - IBI_UPDATED_FORCE_EN"]
pub type IbiupdatedforceenR = crate::BitReader;
#[doc = "Field `IBIUPDATEDFORCEEN` writer - IBI_UPDATED_FORCE_EN"]
pub type IbiupdatedforceenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSOWNERUPDATEDFORCEEN` reader - BUSOWNER_UPDATED_FORCE_EN"]
pub type BusownerupdatedforceenR = crate::BitReader;
#[doc = "Field `BUSOWNERUPDATEDFORCEEN` writer - BUSOWNER_UPDATED_FORCE_EN"]
pub type BusownerupdatedforceenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD1` reader - reserved"]
pub type Rsvd1R = crate::BitReader;
#[doc = "Field `RSVD1` writer - reserved"]
pub type Rsvd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSRESETDONEFORCEEN` reader - BUS_RESET_DONE_FORCE_EN"]
pub type BusresetdoneforceenR = crate::BitReader;
#[doc = "Field `BUSRESETDONEFORCEEN` writer - BUS_RESET_DONE_FORCE_EN"]
pub type BusresetdoneforceenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - reserved"]
pub type RsvdR = crate::FieldReader<u16>;
#[doc = "Field `RSVD` writer - reserved"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - TX_THLD_FORCE_EN"]
    #[inline(always)]
    pub fn txthldforceen(&self) -> TxthldforceenR {
        TxthldforceenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX_THLD_FORCE_EN"]
    #[inline(always)]
    pub fn rxthldforceen(&self) -> RxthldforceenR {
        RxthldforceenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IBI_THLD_FORCE_EN"]
    #[inline(always)]
    pub fn ibithldforceen(&self) -> IbithldforceenR {
        IbithldforceenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CMD_QUEUE_READY_FORCE_EN"]
    #[inline(always)]
    pub fn cmdqueuereadyforceen(&self) -> CmdqueuereadyforceenR {
        CmdqueuereadyforceenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RESP_READY_FORCE_INTR_EN"]
    #[inline(always)]
    pub fn respreadyforceintren(&self) -> RespreadyforceintrenR {
        RespreadyforceintrenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TRANSFER_ABORT_FORCE_EN"]
    #[inline(always)]
    pub fn transferabortforceen(&self) -> TransferabortforceenR {
        TransferabortforceenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CCC_UPDATED_FORCE_EN"]
    #[inline(always)]
    pub fn cccupdatedforceen(&self) -> CccupdatedforceenR {
        CccupdatedforceenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit in Interrupt Force Enable register is reserved."]
    #[inline(always)]
    pub fn rsvd7(&self) -> Rsvd7R {
        Rsvd7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DYN_ADDR_ASSGN_FORCE_EN"]
    #[inline(always)]
    pub fn dynaddrassgnforceen(&self) -> DynaddrassgnforceenR {
        DynaddrassgnforceenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TRANSFER_ERR_FORCE_EN"]
    #[inline(always)]
    pub fn transfererrforceen(&self) -> TransfererrforceenR {
        TransfererrforceenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DEFSLV_FORCE_EN"]
    #[inline(always)]
    pub fn defslvforceen(&self) -> DefslvforceenR {
        DefslvforceenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - READ_REQ_RECV_FORCE_EN"]
    #[inline(always)]
    pub fn readreqrecvforceen(&self) -> ReadreqrecvforceenR {
        ReadreqrecvforceenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - IBI_UPDATED_FORCE_EN"]
    #[inline(always)]
    pub fn ibiupdatedforceen(&self) -> IbiupdatedforceenR {
        IbiupdatedforceenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - BUSOWNER_UPDATED_FORCE_EN"]
    #[inline(always)]
    pub fn busownerupdatedforceen(&self) -> BusownerupdatedforceenR {
        BusownerupdatedforceenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - reserved"]
    #[inline(always)]
    pub fn rsvd1(&self) -> Rsvd1R {
        Rsvd1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - BUS_RESET_DONE_FORCE_EN"]
    #[inline(always)]
    pub fn busresetdoneforceen(&self) -> BusresetdoneforceenR {
        BusresetdoneforceenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - TX_THLD_FORCE_EN"]
    #[inline(always)]
    pub fn txthldforceen(&mut self) -> TxthldforceenW<I3cd048Spec> {
        TxthldforceenW::new(self, 0)
    }
    #[doc = "Bit 1 - RX_THLD_FORCE_EN"]
    #[inline(always)]
    pub fn rxthldforceen(&mut self) -> RxthldforceenW<I3cd048Spec> {
        RxthldforceenW::new(self, 1)
    }
    #[doc = "Bit 2 - IBI_THLD_FORCE_EN"]
    #[inline(always)]
    pub fn ibithldforceen(&mut self) -> IbithldforceenW<I3cd048Spec> {
        IbithldforceenW::new(self, 2)
    }
    #[doc = "Bit 3 - CMD_QUEUE_READY_FORCE_EN"]
    #[inline(always)]
    pub fn cmdqueuereadyforceen(&mut self) -> CmdqueuereadyforceenW<I3cd048Spec> {
        CmdqueuereadyforceenW::new(self, 3)
    }
    #[doc = "Bit 4 - RESP_READY_FORCE_INTR_EN"]
    #[inline(always)]
    pub fn respreadyforceintren(&mut self) -> RespreadyforceintrenW<I3cd048Spec> {
        RespreadyforceintrenW::new(self, 4)
    }
    #[doc = "Bit 5 - TRANSFER_ABORT_FORCE_EN"]
    #[inline(always)]
    pub fn transferabortforceen(&mut self) -> TransferabortforceenW<I3cd048Spec> {
        TransferabortforceenW::new(self, 5)
    }
    #[doc = "Bit 6 - CCC_UPDATED_FORCE_EN"]
    #[inline(always)]
    pub fn cccupdatedforceen(&mut self) -> CccupdatedforceenW<I3cd048Spec> {
        CccupdatedforceenW::new(self, 6)
    }
    #[doc = "Bit 7 - This bit in Interrupt Force Enable register is reserved."]
    #[inline(always)]
    pub fn rsvd7(&mut self) -> Rsvd7W<I3cd048Spec> {
        Rsvd7W::new(self, 7)
    }
    #[doc = "Bit 8 - DYN_ADDR_ASSGN_FORCE_EN"]
    #[inline(always)]
    pub fn dynaddrassgnforceen(&mut self) -> DynaddrassgnforceenW<I3cd048Spec> {
        DynaddrassgnforceenW::new(self, 8)
    }
    #[doc = "Bit 9 - TRANSFER_ERR_FORCE_EN"]
    #[inline(always)]
    pub fn transfererrforceen(&mut self) -> TransfererrforceenW<I3cd048Spec> {
        TransfererrforceenW::new(self, 9)
    }
    #[doc = "Bit 10 - DEFSLV_FORCE_EN"]
    #[inline(always)]
    pub fn defslvforceen(&mut self) -> DefslvforceenW<I3cd048Spec> {
        DefslvforceenW::new(self, 10)
    }
    #[doc = "Bit 11 - READ_REQ_RECV_FORCE_EN"]
    #[inline(always)]
    pub fn readreqrecvforceen(&mut self) -> ReadreqrecvforceenW<I3cd048Spec> {
        ReadreqrecvforceenW::new(self, 11)
    }
    #[doc = "Bit 12 - IBI_UPDATED_FORCE_EN"]
    #[inline(always)]
    pub fn ibiupdatedforceen(&mut self) -> IbiupdatedforceenW<I3cd048Spec> {
        IbiupdatedforceenW::new(self, 12)
    }
    #[doc = "Bit 13 - BUSOWNER_UPDATED_FORCE_EN"]
    #[inline(always)]
    pub fn busownerupdatedforceen(&mut self) -> BusownerupdatedforceenW<I3cd048Spec> {
        BusownerupdatedforceenW::new(self, 13)
    }
    #[doc = "Bit 14 - reserved"]
    #[inline(always)]
    pub fn rsvd1(&mut self) -> Rsvd1W<I3cd048Spec> {
        Rsvd1W::new(self, 14)
    }
    #[doc = "Bit 15 - BUS_RESET_DONE_FORCE_EN"]
    #[inline(always)]
    pub fn busresetdoneforceen(&mut self) -> BusresetdoneforceenW<I3cd048Spec> {
        BusresetdoneforceenW::new(self, 15)
    }
    #[doc = "Bits 16:31 - reserved"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<I3cd048Spec> {
        RsvdW::new(self, 16)
    }
}
#[doc = "Interrupt Force Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd048::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd048::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd048Spec;
impl crate::RegisterSpec for I3cd048Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd048::R`](R) reader structure"]
impl crate::Readable for I3cd048Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd048::W`](W) writer structure"]
impl crate::Writable for I3cd048Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD048 to value 0"]
impl crate::Resettable for I3cd048Spec {}
