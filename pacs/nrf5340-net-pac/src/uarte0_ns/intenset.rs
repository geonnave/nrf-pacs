#[doc = "Register `INTENSET` reader"]
pub struct R(crate::R<INTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENSET` writer"]
pub struct W(crate::W<INTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTS` reader - Write '1' to enable interrupt for event CTS"]
pub type CTS_R = crate::BitReader<CTS_A>;
#[doc = "Write '1' to enable interrupt for event CTS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTS_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CTS_A> for bool {
    #[inline(always)]
    fn from(variant: CTS_A) -> Self {
        variant as u8 != 0
    }
}
impl CTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTS_A {
        match self.bits {
            false => CTS_A::DISABLED,
            true => CTS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTS_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event CTS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTS_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CTS_AW> for bool {
    #[inline(always)]
    fn from(variant: CTS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTS` writer - Write '1' to enable interrupt for event CTS"]
pub type CTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, CTS_AW, O>;
impl<'a, const O: u8> CTS_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CTS_AW::SET)
    }
}
#[doc = "Field `NCTS` reader - Write '1' to enable interrupt for event NCTS"]
pub type NCTS_R = crate::BitReader<NCTS_A>;
#[doc = "Write '1' to enable interrupt for event NCTS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NCTS_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<NCTS_A> for bool {
    #[inline(always)]
    fn from(variant: NCTS_A) -> Self {
        variant as u8 != 0
    }
}
impl NCTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCTS_A {
        match self.bits {
            false => NCTS_A::DISABLED,
            true => NCTS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NCTS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NCTS_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event NCTS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NCTS_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<NCTS_AW> for bool {
    #[inline(always)]
    fn from(variant: NCTS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCTS` writer - Write '1' to enable interrupt for event NCTS"]
pub type NCTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, NCTS_AW, O>;
impl<'a, const O: u8> NCTS_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(NCTS_AW::SET)
    }
}
#[doc = "Field `RXDRDY` reader - Write '1' to enable interrupt for event RXDRDY"]
pub type RXDRDY_R = crate::BitReader<RXDRDY_A>;
#[doc = "Write '1' to enable interrupt for event RXDRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDRDY_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RXDRDY_A> for bool {
    #[inline(always)]
    fn from(variant: RXDRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl RXDRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDRDY_A {
        match self.bits {
            false => RXDRDY_A::DISABLED,
            true => RXDRDY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXDRDY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXDRDY_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event RXDRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDRDY_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<RXDRDY_AW> for bool {
    #[inline(always)]
    fn from(variant: RXDRDY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDRDY` writer - Write '1' to enable interrupt for event RXDRDY"]
pub type RXDRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, RXDRDY_AW, O>;
impl<'a, const O: u8> RXDRDY_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(RXDRDY_AW::SET)
    }
}
#[doc = "Field `ENDRX` reader - Write '1' to enable interrupt for event ENDRX"]
pub type ENDRX_R = crate::BitReader<ENDRX_A>;
#[doc = "Write '1' to enable interrupt for event ENDRX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDRX_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ENDRX_A> for bool {
    #[inline(always)]
    fn from(variant: ENDRX_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDRX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDRX_A {
        match self.bits {
            false => ENDRX_A::DISABLED,
            true => ENDRX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDRX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDRX_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event ENDRX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDRX_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<ENDRX_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDRX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDRX` writer - Write '1' to enable interrupt for event ENDRX"]
pub type ENDRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, ENDRX_AW, O>;
impl<'a, const O: u8> ENDRX_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ENDRX_AW::SET)
    }
}
#[doc = "Field `TXDRDY` reader - Write '1' to enable interrupt for event TXDRDY"]
pub type TXDRDY_R = crate::BitReader<TXDRDY_A>;
#[doc = "Write '1' to enable interrupt for event TXDRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDRDY_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TXDRDY_A> for bool {
    #[inline(always)]
    fn from(variant: TXDRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl TXDRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDRDY_A {
        match self.bits {
            false => TXDRDY_A::DISABLED,
            true => TXDRDY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXDRDY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXDRDY_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event TXDRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDRDY_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<TXDRDY_AW> for bool {
    #[inline(always)]
    fn from(variant: TXDRDY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDRDY` writer - Write '1' to enable interrupt for event TXDRDY"]
pub type TXDRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, TXDRDY_AW, O>;
impl<'a, const O: u8> TXDRDY_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TXDRDY_AW::SET)
    }
}
#[doc = "Field `ENDTX` reader - Write '1' to enable interrupt for event ENDTX"]
pub type ENDTX_R = crate::BitReader<ENDTX_A>;
#[doc = "Write '1' to enable interrupt for event ENDTX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDTX_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ENDTX_A> for bool {
    #[inline(always)]
    fn from(variant: ENDTX_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDTX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDTX_A {
        match self.bits {
            false => ENDTX_A::DISABLED,
            true => ENDTX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDTX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDTX_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event ENDTX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDTX_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<ENDTX_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDTX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDTX` writer - Write '1' to enable interrupt for event ENDTX"]
pub type ENDTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, ENDTX_AW, O>;
impl<'a, const O: u8> ENDTX_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ENDTX_AW::SET)
    }
}
#[doc = "Field `ERROR` reader - Write '1' to enable interrupt for event ERROR"]
pub type ERROR_R = crate::BitReader<ERROR_A>;
#[doc = "Write '1' to enable interrupt for event ERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: ERROR_A) -> Self {
        variant as u8 != 0
    }
}
impl ERROR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERROR_A {
        match self.bits {
            false => ERROR_A::DISABLED,
            true => ERROR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERROR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERROR_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event ERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<ERROR_AW> for bool {
    #[inline(always)]
    fn from(variant: ERROR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` writer - Write '1' to enable interrupt for event ERROR"]
pub type ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, ERROR_AW, O>;
impl<'a, const O: u8> ERROR_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ERROR_AW::SET)
    }
}
#[doc = "Field `RXTO` reader - Write '1' to enable interrupt for event RXTO"]
pub type RXTO_R = crate::BitReader<RXTO_A>;
#[doc = "Write '1' to enable interrupt for event RXTO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTO_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RXTO_A> for bool {
    #[inline(always)]
    fn from(variant: RXTO_A) -> Self {
        variant as u8 != 0
    }
}
impl RXTO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXTO_A {
        match self.bits {
            false => RXTO_A::DISABLED,
            true => RXTO_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXTO_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXTO_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event RXTO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTO_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<RXTO_AW> for bool {
    #[inline(always)]
    fn from(variant: RXTO_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXTO` writer - Write '1' to enable interrupt for event RXTO"]
pub type RXTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, RXTO_AW, O>;
impl<'a, const O: u8> RXTO_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(RXTO_AW::SET)
    }
}
#[doc = "Field `RXSTARTED` reader - Write '1' to enable interrupt for event RXSTARTED"]
pub type RXSTARTED_R = crate::BitReader<RXSTARTED_A>;
#[doc = "Write '1' to enable interrupt for event RXSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXSTARTED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RXSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: RXSTARTED_A) -> Self {
        variant as u8 != 0
    }
}
impl RXSTARTED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXSTARTED_A {
        match self.bits {
            false => RXSTARTED_A::DISABLED,
            true => RXSTARTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXSTARTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXSTARTED_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event RXSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXSTARTED_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<RXSTARTED_AW> for bool {
    #[inline(always)]
    fn from(variant: RXSTARTED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXSTARTED` writer - Write '1' to enable interrupt for event RXSTARTED"]
pub type RXSTARTED_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, RXSTARTED_AW, O>;
impl<'a, const O: u8> RXSTARTED_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(RXSTARTED_AW::SET)
    }
}
#[doc = "Field `TXSTARTED` reader - Write '1' to enable interrupt for event TXSTARTED"]
pub type TXSTARTED_R = crate::BitReader<TXSTARTED_A>;
#[doc = "Write '1' to enable interrupt for event TXSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSTARTED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TXSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: TXSTARTED_A) -> Self {
        variant as u8 != 0
    }
}
impl TXSTARTED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXSTARTED_A {
        match self.bits {
            false => TXSTARTED_A::DISABLED,
            true => TXSTARTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXSTARTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXSTARTED_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event TXSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSTARTED_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<TXSTARTED_AW> for bool {
    #[inline(always)]
    fn from(variant: TXSTARTED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXSTARTED` writer - Write '1' to enable interrupt for event TXSTARTED"]
pub type TXSTARTED_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, TXSTARTED_AW, O>;
impl<'a, const O: u8> TXSTARTED_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TXSTARTED_AW::SET)
    }
}
#[doc = "Field `TXSTOPPED` reader - Write '1' to enable interrupt for event TXSTOPPED"]
pub type TXSTOPPED_R = crate::BitReader<TXSTOPPED_A>;
#[doc = "Write '1' to enable interrupt for event TXSTOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSTOPPED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TXSTOPPED_A> for bool {
    #[inline(always)]
    fn from(variant: TXSTOPPED_A) -> Self {
        variant as u8 != 0
    }
}
impl TXSTOPPED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXSTOPPED_A {
        match self.bits {
            false => TXSTOPPED_A::DISABLED,
            true => TXSTOPPED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXSTOPPED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXSTOPPED_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event TXSTOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSTOPPED_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<TXSTOPPED_AW> for bool {
    #[inline(always)]
    fn from(variant: TXSTOPPED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXSTOPPED` writer - Write '1' to enable interrupt for event TXSTOPPED"]
pub type TXSTOPPED_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, TXSTOPPED_AW, O>;
impl<'a, const O: u8> TXSTOPPED_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TXSTOPPED_AW::SET)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event CTS"]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event NCTS"]
    #[inline(always)]
    pub fn ncts(&self) -> NCTS_R {
        NCTS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event RXDRDY"]
    #[inline(always)]
    pub fn rxdrdy(&self) -> RXDRDY_R {
        RXDRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for event ENDRX"]
    #[inline(always)]
    pub fn endrx(&self) -> ENDRX_R {
        ENDRX_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Write '1' to enable interrupt for event TXDRDY"]
    #[inline(always)]
    pub fn txdrdy(&self) -> TXDRDY_R {
        TXDRDY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write '1' to enable interrupt for event ENDTX"]
    #[inline(always)]
    pub fn endtx(&self) -> ENDTX_R {
        ENDTX_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write '1' to enable interrupt for event ERROR"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 17 - Write '1' to enable interrupt for event RXTO"]
    #[inline(always)]
    pub fn rxto(&self) -> RXTO_R {
        RXTO_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Write '1' to enable interrupt for event RXSTARTED"]
    #[inline(always)]
    pub fn rxstarted(&self) -> RXSTARTED_R {
        RXSTARTED_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Write '1' to enable interrupt for event TXSTARTED"]
    #[inline(always)]
    pub fn txstarted(&self) -> TXSTARTED_R {
        TXSTARTED_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Write '1' to enable interrupt for event TXSTOPPED"]
    #[inline(always)]
    pub fn txstopped(&self) -> TXSTOPPED_R {
        TXSTOPPED_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event CTS"]
    #[inline(always)]
    pub fn cts(&mut self) -> CTS_W<0> {
        CTS_W::new(self)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event NCTS"]
    #[inline(always)]
    pub fn ncts(&mut self) -> NCTS_W<1> {
        NCTS_W::new(self)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event RXDRDY"]
    #[inline(always)]
    pub fn rxdrdy(&mut self) -> RXDRDY_W<2> {
        RXDRDY_W::new(self)
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for event ENDRX"]
    #[inline(always)]
    pub fn endrx(&mut self) -> ENDRX_W<4> {
        ENDRX_W::new(self)
    }
    #[doc = "Bit 7 - Write '1' to enable interrupt for event TXDRDY"]
    #[inline(always)]
    pub fn txdrdy(&mut self) -> TXDRDY_W<7> {
        TXDRDY_W::new(self)
    }
    #[doc = "Bit 8 - Write '1' to enable interrupt for event ENDTX"]
    #[inline(always)]
    pub fn endtx(&mut self) -> ENDTX_W<8> {
        ENDTX_W::new(self)
    }
    #[doc = "Bit 9 - Write '1' to enable interrupt for event ERROR"]
    #[inline(always)]
    pub fn error(&mut self) -> ERROR_W<9> {
        ERROR_W::new(self)
    }
    #[doc = "Bit 17 - Write '1' to enable interrupt for event RXTO"]
    #[inline(always)]
    pub fn rxto(&mut self) -> RXTO_W<17> {
        RXTO_W::new(self)
    }
    #[doc = "Bit 19 - Write '1' to enable interrupt for event RXSTARTED"]
    #[inline(always)]
    pub fn rxstarted(&mut self) -> RXSTARTED_W<19> {
        RXSTARTED_W::new(self)
    }
    #[doc = "Bit 20 - Write '1' to enable interrupt for event TXSTARTED"]
    #[inline(always)]
    pub fn txstarted(&mut self) -> TXSTARTED_W<20> {
        TXSTARTED_W::new(self)
    }
    #[doc = "Bit 22 - Write '1' to enable interrupt for event TXSTOPPED"]
    #[inline(always)]
    pub fn txstopped(&mut self) -> TXSTOPPED_W<22> {
        TXSTOPPED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](index.html) module"]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenset::R](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenset::W](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
