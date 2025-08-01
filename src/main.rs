use std::{env::current_dir, fs::File, io::{Read, Write}, panic, str::FromStr, sync::{Arc, Mutex}, thread, time::Duration};
use enigo::{Direction, Enigo, Keyboard as _, Mouse, Settings};
use rdev::{listen, EventType};
use serde::{Deserialize, Serialize};
use exit_for_enter::exit_for_enter;

use keycode::{Key, Keyboard};

#[derive(Deserialize, Serialize)]
struct Config {
  listen_key: Key,
  simulate_key: Key
}

impl Default for Config {
  fn default() -> Self {
    Self {
      listen_key: Key::Keyboard(Keyboard::G),
      simulate_key: Key::Keyboard(Keyboard::F)
    }
  }
}

impl FromStr for Config {
  type Err = anyhow::Error;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(toml::from_str(s)?)
  }
}

impl Config {
  pub fn to_string(&self) -> anyhow::Result<String> {
    Ok(toml::to_string(self)?)
  }
}

fn main() {
  let config_path = current_dir().expect("无法获取程序所在目录").join("config.toml");
  let mut config_file = File::options().read(true).write(true).create(true).open(config_path).expect("无法打开配置文件");
  let mut config_content = String::new();
  config_file.read_to_string(&mut config_content).expect("无法读取配置文件内容");
  let config = match Config::from_str(&config_content) {
    Ok(cfg) => cfg,
    Err(_) => {
      config_file.write_all(Config::default().to_string().expect("创建默认配置文件失败：无法将默认Config转换为String").as_bytes()).expect("创建默认配置文件失败：无法写入配置文件");
      println!("请填写配置文件");
      exit_for_enter();
    }
  };

  let is_simulating = Arc::new(Mutex::new(false));
  let state_clone = Arc::clone(&is_simulating);
  let state_clone_second = Arc::clone(&is_simulating);

  let enigo = Arc::new(Mutex::new(Enigo::new(&Settings::default()).expect("无法创建Enigo实例")));
  let enigo_clone = Arc::clone(&enigo);
  let enigo_clone_second = Arc::clone(&enigo);

  let listen_key_released = Arc::new(Mutex::new(true));
  let release_state_clone = Arc::clone(&listen_key_released);
  // 不知道这几行有没有用
  panic::set_hook(Box::new(move |_| {
    println!("wc怎么panic了");
    let simulating = is_simulating.lock().unwrap();
    let mut enigo = enigo.lock().unwrap();
    if *simulating {
      if let Key::Keyboard(value) = config.simulate_key {
        enigo.key(value.try_into().unwrap(), Direction::Release).expect("无法释放按键");
      } else if let Key::Mouse(value) = config.simulate_key {
        enigo.button(value.try_into().unwrap(), Direction::Release).expect("无法释放按键");
      }
    }
  }));
  ctrlc::set_handler(move || {
    let simulating = state_clone_second.lock().unwrap();
    let mut enigo = enigo_clone_second.lock().unwrap();
    if *simulating {
      if let Key::Keyboard(value) = config.simulate_key {
        enigo.key(value.try_into().unwrap(), Direction::Release).expect("无法释放按键");
      } else if let Key::Mouse(value) = config.simulate_key {
        enigo.button(value.try_into().unwrap(), Direction::Release).expect("无法释放按键");
      }
    }
  }).expect("无法设置 Ctrl+C 处理程序");

  println!("当前监听的按键：{:?}，当前模拟的按键：{:?}", config.listen_key, config.simulate_key);

  thread::spawn(move || {
    listen(
      move |event| {
        if let Key::Keyboard(value) = config.listen_key {
          if event.event_type == EventType::KeyPress(value.try_into().unwrap()) {
            println!("{:?} 触发", config.listen_key);
            let mut released = release_state_clone.lock().expect("无法获取listen_key_released变量的互斥锁");
            if *released {
              *released = false
            } else {
              return
            }
            drop(released); // 为了防止死锁我也是尽心尽力了
            let mut simulating = state_clone.lock().expect("无法获取is_simulating变量的互斥锁");
            let mut enigo = enigo_clone.lock().expect("无法获取Enigo实例的互斥锁");
            if !*simulating {
              if let Key::Keyboard(value) = config.simulate_key {
                enigo.key(value.try_into().unwrap(), Direction::Release).expect("无法释放按键");
              } else if let Key::Mouse(value) = config.simulate_key {
                enigo.button(value.try_into().unwrap(), Direction::Release).expect("无法释放按键");
              }
              println!("{:?} 按下", config.simulate_key);
              *simulating = true;
            } else {
              if let Key::Keyboard(value) = config.simulate_key {
                enigo.key(value.try_into().unwrap(), Direction::Release).expect("无法释放按键");
              } else if let Key::Mouse(value) = config.simulate_key {
                enigo.button(value.try_into().unwrap(), Direction::Release).expect("无法释放按键");
              }
              println!("{:?} 释放", config.simulate_key);
              *simulating = false;
            };
          } else if event.event_type == EventType::KeyRelease(value.try_into().unwrap()) { // 记录一下，这个bug劳资排了两小时才排出来解决方案，sb程序一跑起来电脑直接卡死
            let mut released = release_state_clone.lock().expect("无法获取listen_key_released变量的互斥锁");
            if *released {
              return
            } else {
              *released = true
            }
          } else {
            if let Key::Keyboard(v) = config.simulate_key {
              if event.event_type == EventType::KeyPress(v.try_into().unwrap()) {
                let released = release_state_clone.lock().expect("无法获取listen_key_released变量的互斥锁");
                if !*released {
                  return
                }
                drop(released);
                let mut simulating = state_clone.lock().expect("无法获取is_simulating变量的互斥锁");
                if *simulating {
                  let mut enigo = enigo_clone.lock().expect("无法获取Enigo实例的互斥锁");
                  if let Key::Keyboard(value) = config.simulate_key {
                    enigo.key(value.try_into().unwrap(), Direction::Release).expect("无法释放按键");
                  } else if let Key::Mouse(value) = config.simulate_key {
                    enigo.button(value.try_into().unwrap(), Direction::Release).expect("无法释放按键");
                  }
                  println!("{:?} 释放", config.simulate_key);
                  *simulating = false;
                }
              }
            } else if let Key::Mouse(v) = config.simulate_key {
              if event.event_type == EventType::ButtonPress(v.try_into().unwrap()) {
                let released = release_state_clone.lock().expect("无法获取listen_key_released变量的互斥锁");
                if !*released {
                  return
                }
                drop(released);
                let mut simulating = state_clone.lock().expect("无法获取is_simulating变量的互斥锁");
                if *simulating {
                  let mut enigo = enigo_clone.lock().expect("无法获取Enigo实例的互斥锁");
                  if let Key::Keyboard(value) = config.simulate_key {
                    enigo.key(value.try_into().unwrap(), Direction::Release).expect("无法释放按键");
                  } else if let Key::Mouse(value) = config.simulate_key {
                    enigo.button(value.try_into().unwrap(), Direction::Release).expect("无法释放按键");
                  }
                  println!("{:?} 释放", config.simulate_key);
                  *simulating = false;
                }
              }
            }
          }
        } else if let Key::Mouse(value) = config.listen_key {
          if event.event_type == EventType::ButtonPress(value.try_into().unwrap()) {
            println!("{:?} 触发", config.listen_key);
            let mut released = release_state_clone.lock().expect("无法获取listen_key_released变量的互斥锁");
            if *released {
              *released = false
            } else {
              return
            }
            drop(released); // 为了防止死锁我也是尽心尽力了
            let mut simulating = state_clone.lock().expect("无法获取is_simulating变量的互斥锁");
            let mut enigo = enigo_clone.lock().expect("无法获取Enigo实例的互斥锁");
            if !*simulating {
              if let Key::Keyboard(value) = config.simulate_key {
                enigo.key(value.try_into().unwrap(), Direction::Release).expect("无法释放按键");
              } else if let Key::Mouse(value) = config.simulate_key {
                enigo.button(value.try_into().unwrap(), Direction::Release).expect("无法释放按键");
              }
              println!("{:?} 按下", config.simulate_key);
              *simulating = true;
            } else {
              if let Key::Keyboard(value) = config.simulate_key {
                enigo.key(value.try_into().unwrap(), Direction::Release).expect("无法释放按键");
              } else if let Key::Mouse(value) = config.simulate_key {
                enigo.button(value.try_into().unwrap(), Direction::Release).expect("无法释放按键");
              }
              println!("{:?} 释放", config.simulate_key);
              *simulating = false;
            };
          } else if event.event_type == EventType::ButtonRelease(value.try_into().unwrap()) {
            let mut released = release_state_clone.lock().expect("无法获取listen_key_released变量的互斥锁");
            if *released {
              return
            } else {
              *released = true
            }
          }else {
            if let Key::Keyboard(v) = config.simulate_key {
              if event.event_type == EventType::KeyPress(v.try_into().unwrap()) {
                let released = release_state_clone.lock().expect("无法获取listen_key_released变量的互斥锁");
                if !*released {
                  return
                }
                drop(released);
                let mut simulating = state_clone.lock().expect("无法获取is_simulating变量的互斥锁");
                if *simulating {
                  let mut enigo = enigo_clone.lock().expect("无法获取Enigo实例的互斥锁");
                  if let Key::Keyboard(value) = config.simulate_key {
                    enigo.key(value.try_into().unwrap(), Direction::Release).expect("无法释放按键");
                  } else if let Key::Mouse(value) = config.simulate_key {
                    enigo.button(value.try_into().unwrap(), Direction::Release).expect("无法释放按键");
                  }
                  println!("{:?} 释放", config.simulate_key);
                  *simulating = false;
                }
              }
            } else if let Key::Mouse(v) = config.simulate_key {
              if event.event_type == EventType::ButtonPress(v.try_into().unwrap()) {
                let released = release_state_clone.lock().expect("无法获取listen_key_released变量的互斥锁");
                if !*released {
                  return
                }
                drop(released);
                let mut simulating = state_clone.lock().expect("无法获取is_simulating变量的互斥锁");
                if *simulating {
                  let mut enigo = enigo_clone.lock().expect("无法获取Enigo实例的互斥锁");
                  if let Key::Keyboard(value) = config.simulate_key {
                    enigo.key(value.try_into().unwrap(), Direction::Release).expect("无法释放按键");
                  } else if let Key::Mouse(value) = config.simulate_key {
                    enigo.button(value.try_into().unwrap(), Direction::Release).expect("无法释放按键");
                  }
                  println!("{:?} 释放", config.simulate_key);
                  *simulating = false;
                }
              }
            }
          }
        }
      }
    ).expect("监听按键失败")
  });

  loop {
    thread::park();
  }

}

