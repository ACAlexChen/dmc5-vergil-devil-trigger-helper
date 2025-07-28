use std::{env::current_dir, fs::File, io::{Read, Write}, panic, str::FromStr, sync::{Arc, Mutex}, thread};
use enigo::{Direction, Enigo, Keyboard as _, Settings};
use rdev::{listen, EventType};
use serde::{Deserialize, Serialize};
use exit_for_enter::exit_for_enter;

use keycode::Keyboard;

#[derive(Deserialize, Serialize)]
struct Config {
  listen_key: Keyboard,
  simulate_key: Keyboard
}

impl Default for Config {
  fn default() -> Self {
    Self {
      listen_key: Keyboard::G,
      simulate_key: Keyboard::F
    }
  }
}

impl FromStr for Config {
  type Err = serde_json::Error;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    serde_json::from_str(s)
  }
}

impl Config {
  pub fn to_string(&self) -> serde_json::Result<String> {
    serde_json::to_string(self)
  }
}

fn main() {
  let config_path = current_dir().expect("无法获取程序所在目录").join("config.json");
  let mut config_file = File::options().read(true).write(true).create(true).open(config_path).expect("无法打开配置文件");
  let mut config_content = String::new();
  config_file.read_to_string(&mut config_content).expect("无法读取配置文件内容");
  let config = match Config::from_str(&config_content) {
    Ok(cfg) => cfg,
    Err(_) => {
      config_file.write(Config::default().to_string().expect("创建默认配置文件失败：无法将默认Config转换为String").as_bytes()).expect("创建默认配置文件失败：无法写入配置文件");
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

  // 不知道这几行有没有用
  panic::set_hook(Box::new(move |_| {
    println!("wc怎么panic了");
    let simulating = is_simulating.lock().unwrap();
    let mut enigo = enigo.lock().unwrap();
    if *simulating {
      enigo.key(config.simulate_key.into(), Direction::Release).expect("无法释放按键");
    }
  }));
  ctrlc::set_handler(move || {
    let simulating = state_clone_second.lock().unwrap();
    let mut enigo = enigo_clone_second.lock().unwrap();
    if *simulating {
      enigo.key(config.simulate_key.into(), Direction::Release).expect("无法释放按键");
    }
  }).expect("无法设置 Ctrl+C 处理程序");

  println!("当前监听的按键：{}，当前模拟的按键：{}", config.listen_key, config.simulate_key);

  thread::spawn(move || {
    listen(
      move |event| {
        if event.event_type == EventType::KeyPress(config.listen_key.into()) {
          println!("{} 触发", config.listen_key);
          let mut simulating = state_clone.lock().expect("无法获取is_simulating变量的互斥锁");
          let mut enigo = enigo_clone.lock().expect("无法获取Enigo实例的互斥锁");
          if !*simulating {
            enigo.key(config.simulate_key.into(), Direction::Press).expect("无法按下按键");
            println!("{} 按下", config.simulate_key);
            *simulating = true;
          } else {
            enigo.key(config.simulate_key.into(), Direction::Release).expect("无法释放按键");
            println!("{} 释放", config.simulate_key);
            *simulating = false;
          };
        }
        if event.event_type == EventType::KeyPress(config.simulate_key.into()) {
          let mut simulating = state_clone.lock().expect("无法获取is_simulating变量的互斥锁");
          if *simulating {
            let mut enigo = enigo_clone.lock().expect("无法获取Enigo实例的互斥锁");
            enigo.key(config.simulate_key.into(), Direction::Release).expect("无法释放按键");
            println!("{} 释放", config.simulate_key);
            *simulating = false;
          }
        }
      }
    ).expect("监听按键失败")
  });

  loop {
    thread::park();
  }

}

