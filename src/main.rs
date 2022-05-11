mod hotkeymanager;
use hotkeymanager::Hotkey;
use inputbot;

fn main() {
    let mut my_hotkeys: Vec<Hotkey> = vec![];
    let hotkey1 = Hotkey::new(vec![inputbot::KeybdKey::CapsLockKey], "first_hotkey");
    let hotkey2 = Hotkey::new(vec![inputbot::KeybdKey::LControlKey, inputbot::KeybdKey::DKey], "second_hotkey");
    my_hotkeys.push(hotkey1);
    my_hotkeys.push(hotkey2);
    loop {
        for hotkey in my_hotkeys.iter_mut(){
            if hotkey.identifier == "first_hotkey"{
                if hotkey.check(){
                    println!("CapsLock Released!");
                }
            } else if hotkey.identifier == "second_hotkey"{
                if hotkey.check(){
                    println!("Left Control + D Pressed!")
                }
            }

        }
    }

}
