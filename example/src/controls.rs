use crate::{textures, web};

pub fn on_click(item_id: &str) {
    match item_id {
        val if val == textures::NOKIA_ART_JAM_3_HOUSE.id => {
            log::info!("House is thinking it's not Lupus!");
        }
        val if val == textures::NOKIA_ART_JAM_3_KEYBOARD_CAT.id => {
            log::info!("Look at that cat GO!");
        }
        val if val == textures::NOKIA_ART_JAM_3_WORMS.id => {
            log::info!("Damn these worms are ANGRY!");
        }
        val if val == textures::VERMINTIDE_TAPESTRY.id => {
            log::info!("Clicked on the tapestry!");
        }
        val if val == textures::UBERSREIK_FIVE.id => {
            let popup_page = web::access::popup_page();
            if popup_page.hidden() {
                web::access::document().exit_pointer_lock();
            }
            popup_page.set_hidden(!popup_page.hidden());
        }
        &_ => {}
    }
}

pub fn on_mouse_capture() {
    web::access::popup_page().set_hidden(true);
}
