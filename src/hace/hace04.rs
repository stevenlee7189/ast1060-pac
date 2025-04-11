#[doc = "Register `HACE04` reader"]
pub type R = crate::R<Hace04Spec>;
#[doc = "Register `HACE04` writer"]
pub type W = crate::W<Hace04Spec>;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `BaseAddrOfCryptoDestData300` reader - Base address of crypto destination data\\[30:0\\] (byte aligned)"]
pub type BaseAddrOfCryptoDestData300R = crate::FieldReader<u32>;
#[doc = "Field `BaseAddrOfCryptoDestData300` writer - Base address of crypto destination data\\[30:0\\] (byte aligned)"]
pub type BaseAddrOfCryptoDestData300W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `BaseAddrOfSgListForCryptoDestData303` reader - Base address of scatter-gather list for crypto destination data\\[30:3\\] (8-byte aligned)"]
pub type BaseAddrOfSgListForCryptoDestData303R = crate::FieldReader<u32>;
#[doc = "Field `BaseAddrOfSgListForCryptoDestData303` writer - Base address of scatter-gather list for crypto destination data\\[30:3\\] (8-byte aligned)"]
pub type BaseAddrOfSgListForCryptoDestData303W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::BitReader;
#[doc = "Field `Reserved02` reader - Reserved (0)"]
pub type Reserved02R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 0:30 - Base address of crypto destination data\\[30:0\\] (byte aligned)"]
    #[inline(always)]
    pub fn base_addr_of_crypto_dest_data300(&self) -> BaseAddrOfCryptoDestData300R {
        BaseAddrOfCryptoDestData300R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bits 3:30 - Base address of scatter-gather list for crypto destination data\\[30:3\\] (8-byte aligned)"]
    #[inline(always)]
    pub fn base_addr_of_sg_list_for_crypto_dest_data303(
        &self,
    ) -> BaseAddrOfSgListForCryptoDestData303R {
        BaseAddrOfSgListForCryptoDestData303R::new((self.bits >> 3) & 0x0fff_ffff)
    }
    #[doc = "Bit 31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved02(&self) -> Reserved02R {
        Reserved02R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Base address of crypto destination data\\[30:0\\] (byte aligned)"]
    #[inline(always)]
    pub fn base_addr_of_crypto_dest_data300(&mut self) -> BaseAddrOfCryptoDestData300W<Hace04Spec> {
        BaseAddrOfCryptoDestData300W::new(self, 0)
    }
    #[doc = "Bits 3:30 - Base address of scatter-gather list for crypto destination data\\[30:3\\] (8-byte aligned)"]
    #[inline(always)]
    pub fn base_addr_of_sg_list_for_crypto_dest_data303(
        &mut self,
    ) -> BaseAddrOfSgListForCryptoDestData303W<Hace04Spec> {
        BaseAddrOfSgListForCryptoDestData303W::new(self, 3)
    }
}
#[doc = "Crypto Data Destination Base Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hace04::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace04::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hace04Spec;
impl crate::RegisterSpec for Hace04Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hace04::R`](R) reader structure"]
impl crate::Readable for Hace04Spec {}
#[doc = "`write(|w| ..)` method takes [`hace04::W`](W) writer structure"]
impl crate::Writable for Hace04Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HACE04 to value 0"]
impl crate::Resettable for Hace04Spec {}
