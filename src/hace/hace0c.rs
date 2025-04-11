#[doc = "Register `HACE0C` reader"]
pub type R = crate::R<Hace0cSpec>;
#[doc = "Register `HACE0C` writer"]
pub type W = crate::W<Hace0cSpec>;
#[doc = "Field `CryptoDataLenBytes` reader - Crypto data length (bytes)"]
pub type CryptoDataLenBytesR = crate::FieldReader<u32>;
#[doc = "Field `CryptoDataLenBytes` writer - Crypto data length (bytes)"]
pub type CryptoDataLenBytesW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:27 - Crypto data length (bytes)"]
    #[inline(always)]
    pub fn crypto_data_len_bytes(&self) -> CryptoDataLenBytesR {
        CryptoDataLenBytesR::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bits 28:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:27 - Crypto data length (bytes)"]
    #[inline(always)]
    pub fn crypto_data_len_bytes(&mut self) -> CryptoDataLenBytesW<Hace0cSpec> {
        CryptoDataLenBytesW::new(self, 0)
    }
}
#[doc = "Crypto Data Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hace0c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace0c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hace0cSpec;
impl crate::RegisterSpec for Hace0cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hace0c::R`](R) reader structure"]
impl crate::Readable for Hace0cSpec {}
#[doc = "`write(|w| ..)` method takes [`hace0c::W`](W) writer structure"]
impl crate::Writable for Hace0cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HACE0C to value 0"]
impl crate::Resettable for Hace0cSpec {}
