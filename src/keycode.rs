use rdev::Key as RdevKey;
use rdev::Button as RdevButton;
use enigo::Key as EnigoKey;

use strum_macros::{Display, EnumString};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Display, EnumString)]
pub enum Keyboard {
  A,
  B,
  C,
  D,
  E,
  F,
  G,
  H,
  I,
  J,
  K,
  L,
  M,
  N,
  O,
  P,
  Q,
  R,
  S,
  T,
  U,
  V,
  W,
  X,
  Y,
  Z,
  Tab,
  ShiftLeft,
  ShiftRight,
  ControlLeft,
  ControlRight,
  AltLeft,
  AltRight,
  Space,
  Backspace,
  Enter,
  CapsLock,
  Esc
}

impl Keyboard {
  pub fn to_ascii(&self) -> u8 {
    match self {
      Keyboard::A => 65,
      Keyboard::B => 66,
      Keyboard::C => 67,
      Keyboard::D => 68,
      Keyboard::E => 69,
      Keyboard::F => 70,
      Keyboard::G => 71,
      Keyboard::H => 72,
      Keyboard::I => 73,
      Keyboard::J => 74,
      Keyboard::K => 75,
      Keyboard::L => 76,
      Keyboard::M => 77,
      Keyboard::N => 78,
      Keyboard::O => 79,
      Keyboard::P => 80,
      Keyboard::Q => 81,
      Keyboard::R => 82,
      Keyboard::S => 83,
      Keyboard::T => 84,
      Keyboard::U => 85,
      Keyboard::V => 86,
      Keyboard::W => 87,
      Keyboard::X => 88,
      Keyboard::Y => 89,
      Keyboard::Z => 90,
      Keyboard::Backspace => 8,
      Keyboard::CapsLock => 20,
      Keyboard::Enter => 13,
      Keyboard::Esc => 27,
      Keyboard::AltLeft => 18,
      Keyboard::AltRight => 18,
      Keyboard::ControlLeft => 17,
      Keyboard::ControlRight => 17,
      Keyboard::ShiftLeft => 16,
      Keyboard::ShiftRight => 16,
      Keyboard::Space => 32,
      Keyboard::Tab => 9
    }
  }

  pub fn to_rdev_key(&self) -> RdevKey {
    match self {
      Keyboard::A => RdevKey::KeyA,
      Keyboard::B => RdevKey::KeyB,
      Keyboard::C => RdevKey::KeyC,
      Keyboard::D => RdevKey::KeyD,
      Keyboard::E => RdevKey::KeyE,
      Keyboard::F => RdevKey::KeyF,
      Keyboard::G => RdevKey::KeyG,
      Keyboard::H => RdevKey::KeyH,
      Keyboard::I => RdevKey::KeyI,
      Keyboard::J => RdevKey::KeyJ,
      Keyboard::K => RdevKey::KeyK,
      Keyboard::L => RdevKey::KeyL,
      Keyboard::M => RdevKey::KeyM,
      Keyboard::N => RdevKey::KeyN,
      Keyboard::O => RdevKey::KeyO,
      Keyboard::P => RdevKey::KeyP,
      Keyboard::Q => RdevKey::KeyQ,
      Keyboard::R => RdevKey::KeyR,
      Keyboard::S => RdevKey::KeyS,
      Keyboard::T => RdevKey::KeyT,
      Keyboard::U => RdevKey::KeyU,
      Keyboard::V => RdevKey::KeyV,
      Keyboard::W => RdevKey::KeyW,
      Keyboard::X => RdevKey::KeyX,
      Keyboard::Y => RdevKey::KeyY,
      Keyboard::Z => RdevKey::KeyZ,
      Keyboard::Backspace => RdevKey::Backspace,
      Keyboard::CapsLock => RdevKey::CapsLock,
      Keyboard::Enter => todo!("没找到XD"),
      Keyboard::Esc => todo!("这个也没找到XD"),
      Keyboard::AltLeft => RdevKey::Alt,
      Keyboard::AltRight => RdevKey::Alt,
      Keyboard::ControlLeft => RdevKey::ControlLeft,
      Keyboard::ControlRight => RdevKey::ControlRight,
      Keyboard::ShiftLeft => RdevKey::ShiftLeft,
      Keyboard::ShiftRight => RdevKey::ShiftRight,
      Keyboard::Space => RdevKey::Space,
      Keyboard::Tab => RdevKey::Tab
    }
  }

  pub fn to_enigo_key(&self) -> EnigoKey {
    match self {
      Keyboard::A => EnigoKey::A,
      Keyboard::B => EnigoKey::B,
      Keyboard::C => EnigoKey::C,
      Keyboard::D => EnigoKey::D,
      Keyboard::E => EnigoKey::E,
      Keyboard::F => EnigoKey::F,
      Keyboard::G => EnigoKey::G,
      Keyboard::H => EnigoKey::H,
      Keyboard::I => EnigoKey::I,
      Keyboard::J => EnigoKey::J,
      Keyboard::K => EnigoKey::K,
      Keyboard::L => EnigoKey::L,
      Keyboard::M => EnigoKey::M,
      Keyboard::N => EnigoKey::N,
      Keyboard::O => EnigoKey::O,
      Keyboard::P => EnigoKey::P,
      Keyboard::Q => EnigoKey::Q,
      Keyboard::R => EnigoKey::R,
      Keyboard::S => EnigoKey::S,
      Keyboard::T => EnigoKey::T,
      Keyboard::U => EnigoKey::U,
      Keyboard::V => EnigoKey::V,
      Keyboard::W => EnigoKey::W,
      Keyboard::X => EnigoKey::X,
      Keyboard::Y => EnigoKey::Y,
      Keyboard::Z => EnigoKey::Z,
      Keyboard::Backspace => EnigoKey::Backspace,
      Keyboard::CapsLock => EnigoKey::CapsLock,
      Keyboard::Enter => todo!("没找到QwQ"),
      Keyboard::Esc => todo!("没找到QwQ"),
      Keyboard::AltLeft => EnigoKey::Alt,
      Keyboard::AltRight => EnigoKey::Alt,
      Keyboard::ControlLeft => EnigoKey::Control,
      Keyboard::ControlRight => EnigoKey::Control,
      Keyboard::ShiftLeft => EnigoKey::Shift,
      Keyboard::ShiftRight => EnigoKey::Shift,
      Keyboard::Space => EnigoKey::Space,
      Keyboard::Tab => EnigoKey::Tab
    }
  }
}

impl Into<u8> for Keyboard {
  fn into(self) -> u8 {
    self.to_ascii()
  }
}

impl Into<RdevKey> for Keyboard {
  fn into(self) -> RdevKey {
    self.to_rdev_key()
  }
}

impl Into<EnigoKey> for Keyboard {
  fn into(self) -> EnigoKey {
    self.to_enigo_key()
  }
}

impl Into<String> for Keyboard {
  fn into(self) -> String {
    self.to_string()
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Display, EnumString)]
pub enum Mouse {
  Left,
  Right,
  Middle,
  Forward,
  Back
}

impl Mouse {
  fn to_rdev_button(&self) -> RdevButton {
    match self {
      Mouse::Left => RdevButton::Left,
      Mouse::Right => RdevButton::Right,
      Mouse::Middle => RdevButton::Middle,
      Mouse::Back => todo!("没找到XD"),
      Mouse::Forward => todo!("这个也是XD")
    }
  }
}

impl Into<RdevButton> for Mouse {
  fn into(self) -> RdevButton {
    self.to_rdev_button()
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Display)]
pub enum Key {
  Keyboard(Keyboard),
  Mouse(Mouse)
}

