#[doc = "Register `HASH_CLK` writer"]
pub type W = crate::W<HashClkSpec>;
#[doc = "Enables clock for the HASH engine.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Disable clock for the HASH engine."]
    Disable = 0,
    #[doc = "1: Enable clock for the HASH engine."]
    Enable = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` writer - Enables clock for the HASH engine."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable clock for the HASH engine."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Disable)
    }
    #[doc = "Enable clock for the HASH engine."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enable)
    }
}
impl W {
    #[doc = "Bit 0 - Enables clock for the HASH engine."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<HashClkSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "Clock control for the HASH engine.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_clk::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashClkSpec;
impl crate::RegisterSpec for HashClkSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hash_clk::W`](W) writer structure"]
impl crate::Writable for HashClkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_CLK to value 0"]
impl crate::Resettable for HashClkSpec {
    const RESET_VALUE: u32 = 0;
}
